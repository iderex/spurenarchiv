//! The generator behind `examples/synthetic`.
//!
//! It writes a whole deposit: the manifest, the metadata document, and the two
//! arrays. Nothing about the deposit is hand-written, so the tracked copy can
//! be regenerated and compared rather than trusted.
//!
//! Two rules shape it. The array is computed from numbers this run has already
//! written into the metadata document, read back out of that document rather
//! than out of a second copy of them, so a parameter cannot be one thing in the
//! deposit and another in the arithmetic. And every quantity the model does not
//! represent carries `not_applicable` rather than a plausible number, because a
//! computed dataset that fills in a dark count rate is a computed dataset a
//! reanalysis will treat as a measurement.
//!
//! Some of the parameters below are not in the deposit, and each of those says
//! at its own constant why the model has no field to put it in. That is a
//! finding about the model rather than a shortcut taken here, and
//! `examples/synthetic/README.md` is where it is written down.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use serde_json::{json, Value};
use sha2::{Digest, Sha256};

/// The full width at half maximum of the unstreaked photoline, in electron
/// volts. It is here rather than in the deposit because it stands for the
/// bandwidth of the ionising pulse, and the model carries no field about that
/// pulse at all.
const LINE_WIDTH: f64 = 1.6;

/// The seed of the pseudo-random sequence the counts are drawn against. Fixed,
/// because a deposit that is different on every run cannot be regenerated and
/// compared. Not in the deposit, for the same reason as the constant below it.
const SEED: u64 = 20_260_210;

/// Counts at the peak of the line, per shot. It sets the height of the array
/// and nothing else, and it is held low enough that the draw below stays inside
/// the range where its arithmetic is exact. Not in the deposit: how many counts
/// a computation decides to produce is not a property of a measurement.
const PEAK_COUNTS_PER_SHOT: f64 = 0.15;

/// The date the tracked deposit was produced, written into the deposit from
/// here. A constant rather than the clock, because a date read from the clock
/// changes the bytes on every run and the deposit could then never be compared
/// against the tracked one.
const PRODUCED_ON: &str = "2026-08-10";

/// The intensity full width at half maximum of the dressing pulse, in seconds.
const DURATION: f64 = 5e-15;

/// The centre wavelength of the dressing pulse, in metres.
const WAVELENGTH: f64 = 8e-7;

/// The peak intensity of the dressing pulse, in watts per square centimetre.
const INTENSITY: f64 = 1e12;

/// Shots behind each delay point. It scales the counts and the Poisson width
/// with them.
const SHOTS_PER_POINT: i64 = 2000;

// Each of the four above is written into the metadata document and read back
// out of it before the array is computed, so the document is where they live
// and these are the values it is written with rather than a second copy the
// arithmetic uses.

fn main() -> ExitCode {
    let mut arguments = std::env::args().skip(1);
    let root = match arguments.next() {
        Some(path) if path != "-h" && path != "--help" => PathBuf::from(path),
        _ => {
            println!(
                "usage: synthetic-example <deposit directory>\n\n\
                 Writes the manifest, the metadata document and the arrays of the\n\
                 synthetic deposit into that directory, replacing what is there.\n\
                 The tracked copy is examples/synthetic/neon-2p-streaking."
            );
            return ExitCode::from(2);
        }
    };

    let metadata = metadata_document();
    let metadata_bytes = document(&metadata);

    let (spectrogram, uncertainty) = arrays(&metadata);

    let dataset = root.join("scan-001");
    let files = [
        ("scan-001/metadata.json", "dataset_metadata", metadata_bytes),
        ("scan-001/spectrogram.bin", "array", spectrogram),
        ("scan-001/uncertainty.bin", "array", uncertainty),
    ];

    let manifest = document(&manifest_document(&files));

    if let Err(error) = std::fs::create_dir_all(&dataset) {
        eprintln!("cannot make {}: {error}", dataset.display());
        return ExitCode::from(2);
    }
    for (path, _, bytes) in &files {
        if let Err(error) = write(&root.join(path), bytes) {
            eprintln!("{error}");
            return ExitCode::from(2);
        }
    }
    if let Err(error) = write(&root.join("manifest.json"), &manifest) {
        eprintln!("{error}");
        return ExitCode::from(2);
    }

    println!("wrote {}", root.display());
    ExitCode::SUCCESS
}

fn write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = std::fs::File::create(path)
        .map_err(|error| format!("cannot write {}: {error}", path.display()))?;
    file.write_all(bytes)
        .map_err(|error| format!("cannot write {}: {error}", path.display()))
}

/// A JSON document as bytes: two-space indentation, a line feed at the end and
/// no carriage return anywhere, which is what `docs/spec/deposit-layout.md`
/// requires of everything inside a deposit.
fn document(value: &Value) -> Vec<u8> {
    let mut text = serde_json::to_string_pretty(value).expect("a document built here serialises");
    text.push('\n');
    text.into_bytes()
}

fn manifest_document(files: &[(&str, &str, Vec<u8>)]) -> Value {
    let entries: Vec<Value> = files
        .iter()
        .map(|(path, role, bytes)| {
            json!({
                "path": path,
                "size": bytes.len(),
                "digest": format!("sha256:{:x}", Sha256::digest(bytes)),
                "role": role,
            })
        })
        .collect();
    json!({
        "schema_version": "1.0",
        "file_count": entries.len(),
        "files": entries,
    })
}

/// The delay axis, the energy axis and the two arrays, all read back out of the
/// document that was just built.
fn arrays(metadata: &Value) -> (Vec<u8>, Vec<u8>) {
    let delays = numbers(metadata, "/delay_values/value");
    let energies = numbers(
        metadata,
        "/energy_calibration/value/parameters/bin_energies",
    );
    let shots = numbers(metadata, "/shots_per_point/value");
    let duration = number(metadata, "/dressing_pulse_duration/value/value");
    let wavelength = number(metadata, "/dressing_centre_wavelength/value");
    let intensity = number(metadata, "/dressing_peak_intensity/value/value");

    // The unstreaked line sits at the centre of the stated axis. That is a
    // choice of the model rather than a physical fact, and it is the one that
    // removes the need for a photon energy the model cannot carry.
    let centre = (energies[0] + energies[energies.len() - 1]) / 2.0;

    // Ponderomotive energy in electron volts from the intensity in watts per
    // square centimetre and the wavelength in metres, and the streaking
    // amplitude from it. Both are the textbook relations and both are stated in
    // examples/synthetic/README.md, where they can be checked.
    let micrometres = wavelength * 1e6;
    let ponderomotive = 9.33e-14 * intensity * micrometres * micrometres;
    let amplitude = (8.0 * centre * ponderomotive).sqrt();

    // The carrier, and the envelope the streak follows. The envelope of the
    // vector potential is wider than the intensity envelope by root two, which
    // is the Gaussian relation between the two and not an adjustment.
    let angular_frequency = std::f64::consts::TAU * 299_792_458.0 / wavelength;
    let envelope_width = duration * std::f64::consts::SQRT_2;

    let mut noise = Noise::new(SEED);
    let mut counts: Vec<i32> = Vec::with_capacity(delays.len() * energies.len());
    let mut sigma: Vec<f64> = Vec::with_capacity(delays.len() * energies.len());

    for (point, delay) in delays.iter().enumerate() {
        let streak =
            amplitude * gaussian(*delay, envelope_width) * (angular_frequency * delay).cos();
        let peak = PEAK_COUNTS_PER_SHOT * shots[point];
        for energy in &energies {
            let expected = peak * gaussian(energy - centre - streak, LINE_WIDTH);
            let drawn = noise.poisson(expected);
            counts.push(drawn);
            sigma.push(f64::from(drawn).sqrt());
        }
    }

    let mut spectrogram = Vec::with_capacity(counts.len() * 4);
    for value in counts {
        spectrogram.extend_from_slice(&value.to_le_bytes());
    }
    let mut uncertainty = Vec::with_capacity(sigma.len() * 8);
    for value in sigma {
        uncertainty.extend_from_slice(&value.to_le_bytes());
    }
    (spectrogram, uncertainty)
}

/// A Gaussian of unit height, written by its full width at half maximum so that
/// every width in this generator is the width the metadata document states.
fn gaussian(offset: f64, full_width_at_half_maximum: f64) -> f64 {
    let ratio = offset / full_width_at_half_maximum;
    (-4.0 * std::f64::consts::LN_2 * ratio * ratio).exp()
}

fn number(document: &Value, pointer: &str) -> f64 {
    document
        .pointer(pointer)
        .and_then(Value::as_f64)
        .unwrap_or_else(|| panic!("{pointer} is a number in the document this run built"))
}

fn numbers(document: &Value, pointer: &str) -> Vec<f64> {
    document
        .pointer(pointer)
        .and_then(Value::as_array)
        .unwrap_or_else(|| panic!("{pointer} is an array in the document this run built"))
        .iter()
        .map(|value| {
            value
                .as_f64()
                .unwrap_or_else(|| panic!("{pointer} holds numbers"))
        })
        .collect()
}

/// The counts are drawn rather than rounded, so that `completeness_level` of
/// `counts` and a Poisson noise model are true of the array rather than only
/// claimed about it.
///
/// The sequence is a fixed one. Nothing here is a source of randomness in the
/// sense anything should rely on; it exists to make the array look like counts
/// and to make the same array come back on the next run.
struct Noise(u64);

impl Noise {
    fn new(seed: u64) -> Self {
        Noise(seed | 1)
    }

    /// Xorshift, in the multiplied form, taken to a fraction in [0, 1).
    fn uniform(&mut self) -> f64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        let drawn = self.0.wrapping_mul(0x2545_F491_4F6C_DD1D);
        // The top 53 bits, which is every bit a double can hold without
        // rounding, over two to the fifty-three.
        (drawn >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Knuth's method: multiply uniforms until the product falls under
    /// `exp(-mean)`. It is exact for the means this generator produces and it
    /// stops being usable somewhere above a mean of seven hundred, where that
    /// exponential is no longer a number. `PEAK_COUNTS_PER_SHOT` is what holds
    /// the means below that, and this refuses rather than drifting past it.
    fn poisson(&mut self, mean: f64) -> i32 {
        assert!(
            mean.is_finite() && (0.0..=700.0).contains(&mean),
            "a mean of {mean} is outside the range this draw is exact over"
        );
        let limit = (-mean).exp();
        let mut drawn = 0i32;
        let mut product = 1.0;
        loop {
            product *= self.uniform();
            if product <= limit {
                return drawn;
            }
            drawn += 1;
        }
    }
}

/// The delay axis: forty-one points three tenths of a femtosecond apart,
/// centred on zero.
///
/// The step is what makes the trace readable rather than a taste. The dressing
/// carrier has a period of about 2.7 femtoseconds, so a step of one
/// femtosecond samples it two and a half times and the oscillation the whole
/// example exists to show comes out as a jitter nobody could fit. This step
/// samples it about nine times.
fn delays() -> Vec<f64> {
    (0..41)
        .map(|index| (f64::from(index) - 20.0) * 0.3e-15)
        .collect()
}

/// The energy axis: twenty-four bin centres one electron volt apart. Every one
/// of them is exactly representable, so the axis carries no rounding of its
/// own into the arithmetic above.
fn energies() -> Vec<f64> {
    (0..24).map(|index| 56.5 + f64::from(index)).collect()
}

/// The spectrum of the dressing pulse, as the transform limit of the stated
/// envelope. It is `estimated` in the document because the model uses a single
/// carrier frequency and never this curve.
fn dressing_spectrum(centre: f64, duration: f64) -> (Vec<f64>, Vec<f64>) {
    // The time-bandwidth product of a Gaussian, from the intensity duration to
    // the intensity bandwidth in frequency.
    let bandwidth = 0.441 / duration;
    let speed = 299_792_458.0;
    let wavelength_width = centre * centre * bandwidth / speed;
    let mut wavelengths = Vec::new();
    let mut intensities = Vec::new();
    for step in -2..=2 {
        let wavelength = centre + f64::from(step) * wavelength_width / 2.0;
        wavelengths.push(wavelength);
        intensities.push(gaussian(wavelength - centre, wavelength_width));
    }
    (wavelengths, intensities)
}

/// A field that carries a value.
fn present(value: Value) -> Value {
    json!({ "state": "present", "value": value })
}

/// A field whose value was chosen rather than measured, and what it was chosen
/// from.
fn estimated(value: Value, basis: &str) -> Value {
    json!({ "state": "estimated", "value": value, "basis": basis })
}

/// A quantity that has no meaning for this dataset. Every use of this below is
/// a quantity of an apparatus that does not exist, and none of them is a
/// quantity somebody failed to record.
fn not_applicable() -> Value {
    json!({ "state": "not_applicable" })
}

/// The whole document, in the groups the model is argued in. It is assembled
/// from several pieces rather than written as one, because the key order a
/// deposit is written in is not the order the fields are reasoned about and
/// the document sorts its keys either way.
fn metadata_document() -> Value {
    let mut fields = serde_json::Map::new();
    fields.insert("schema_version".into(), json!("1.0"));
    for group in [
        array_fields(),
        delay_fields(),
        energy_fields(),
        detector_fields(),
        dressing_fields(),
        target_fields(),
        acquisition_fields(),
        provenance_fields(),
    ] {
        for (key, value) in group {
            let previous = fields.insert(key.to_string(), value);
            assert!(previous.is_none(), "{key} is written once");
        }
    }
    Value::Object(fields)
}

/// What the array is and how to read it.
fn array_fields() -> Vec<(&'static str, Value)> {
    let delays = delays();
    let energies = energies();
    vec![
        (
            "spectrogram",
            present(json!({
                "file": "spectrogram.bin",
                "shape": [delays.len(), energies.len()],
                "element_type": "int32",
                "byte_order": "little_endian"
            })),
        ),
        (
            "spectrogram_axis_order",
            present(json!(["delay", "energy"])),
        ),
        (
            "spectrogram_value_semantics",
            present(json!({ "quantity_per_pixel": "counts", "per": "bin" })),
        ),
        (
            "axis_bin_convention",
            present(json!({ "delay": "bin_centre", "energy": "bin_centre" })),
        ),
        (
            "unmeasured_pixel_marker",
            present(json!({ "how_marked": "every_pixel_measured" })),
        ),
        ("saturated_pixel_marker", not_applicable()),
    ]
}

/// The delay axis and what its numbers mean.
fn delay_fields() -> Vec<(&'static str, Value)> {
    let delays = delays();
    vec![
        ("delay_axis_quantity", present(json!("optical_delay"))),
        ("delay_values", present(json!(delays))),
        (
            "delay_value_uncertainty",
            present(json!(vec![0.0; delays.len()])),
        ),
        (
            "delay_jitter",
            present(json!({
                "rms": 0.0,
                "how_determined": "The delays are the numbers the model was evaluated on, so each point sits exactly where the axis says it does."
            })),
        ),
        ("delay_reference_point", present(json!("envelope_peak"))),
        (
            "delay_sign_convention",
            present(json!("xuv_minus_dressing")),
        ),
        (
            "delay_zero_definition",
            present(json!({
                "condition": "The peak of the ionising envelope coincides with the peak of the dressing envelope.",
                "how_located": "Placed there by construction; both envelopes are evaluated on one delay axis whose zero is that coincidence."
            })),
        ),
        ("stage_to_delay_relation", not_applicable()),
    ]
}

/// The energy axis, and everything that would otherwise have to be undone to
/// recover it.
fn energy_fields() -> Vec<(&'static str, Value)> {
    let energies = energies();
    let first = energies[0];
    let last = energies[energies.len() - 1];
    vec![
        ("energy_axis_quantity", present(json!("kinetic_energy"))),
        (
            "energy_calibration",
            present(json!({
                "form": "tabulated",
                "parameters": {
                    "bin_energies": energies,
                    "unit": "eV",
                    "note": "One entry per energy bin, in the order the array holds them."
                },
                "uncertainty": {
                    "value": 0.0,
                    "how_obtained": "The bin energies are the numbers the line profile was evaluated on, so the axis carries no calibration error."
                },
                "description": "The energy of each bin, stated outright. Nothing was fitted, so no analytic form is reported as though it had been."
            })),
        ),
        ("calibration_reference", not_applicable()),
        ("spectrometer_resolution", not_applicable()),
        ("transmission_function", not_applicable()),
        (
            "detection_efficiency",
            present(json!({
                "energies": [first, last],
                "values": [1.0, 1.0],
                "how_determined": "Unity across the axis by construction; no efficiency is applied."
            })),
        ),
    ]
}

/// The apparatus. Almost all of it is absent, and absent for one reason: there
/// is no apparatus, so these are quantities of a thing that does not exist
/// rather than quantities nobody wrote down.
fn detector_fields() -> Vec<(&'static str, Value)> {
    vec![
        (
            "detector_type",
            present(json!({
                "kind": "other",
                "configuration": "None. This is a computed electron spectrum and no detector stands behind it. The field is required and its list has no entry for a dataset with no detector, so the nearest true answer is written here and examples/synthetic/README.md says so."
            })),
        ),
        (
            "detection_direction",
            present(json!({
                "frame": "model frame, z along the beams, y along the dressing polarisation",
                "components": [0.0, 1.0, 0.0]
            })),
        ),
        (
            "detector_background",
            present(json!({ "subtracted": "none" })),
        ),
        ("detector_angular_acceptance", not_applicable()),
        ("detector_dead_time", not_applicable()),
        ("detector_saturation", not_applicable()),
        ("detector_noise_parameters", not_applicable()),
        ("subtracted_background_variance", not_applicable()),
    ]
}

/// The dressing field. These are the numbers the streak is computed from, so
/// they are the ones a reanalysis has to get back out of the array.
fn dressing_fields() -> Vec<(&'static str, Value)> {
    let (wavelengths, intensities) = dressing_spectrum(WAVELENGTH, DURATION);
    vec![
        ("dressing_centre_wavelength", present(json!(WAVELENGTH))),
        (
            "dressing_pulse_duration",
            present(json!({
                "value": DURATION,
                "measure": "fwhm_intensity",
                "how_determined": "The width the envelope was evaluated at."
            })),
        ),
        (
            "dressing_peak_intensity",
            present(json!({
                "value": INTENSITY,
                "determination_method": "other",
                "how_determined": "The intensity the ponderomotive energy was computed from. It was chosen rather than measured, and choosing it is what fixes the streaking amplitude."
            })),
        ),
        (
            "dressing_polarisation",
            present(json!({
                "state": "linear",
                "direction": {
                    "frame": "model frame, z along the beams, y along the dressing polarisation",
                    "components": [0.0, 1.0, 0.0]
                }
            })),
        ),
        (
            "dressing_carrier_envelope_phase",
            present(json!({ "stabilised": true, "tagging": "single_value", "value": 0.0 })),
        ),
        (
            "dressing_spectrum",
            estimated(
                json!({ "wavelengths": wavelengths, "spectral_intensities": intensities }),
                "The transform limit of the stated Gaussian envelope. A single carrier frequency is what the array was computed from and this curve never enters it, so it describes the pulse the model implies rather than a spectrum anything measured.",
            ),
        ),
    ]
}

/// The target.
fn target_fields() -> Vec<(&'static str, Value)> {
    vec![
        ("target_species", present(json!("Ne"))),
        (
            "target_composition",
            present(json!({
                "components": [{ "species": "Ne", "fraction": 1.0 }],
                "how_determined": "One species by construction."
            })),
        ),
        (
            "target_shells",
            present(json!([{
                "shell": "Ne 2p",
                "threshold": 21.56,
                "source": "A label for the single line the model emits. That line is placed at the centre of the stated energy axis and this number never enters the array, so it is not a threshold anything was computed from."
            }])),
        ),
        ("target_resonances", present(json!([]))),
        ("target_number_density", not_applicable()),
        ("target_jet_geometry", not_applicable()),
    ]
}

/// What the numbers in the array are, and how the scan that produced them ran.
fn acquisition_fields() -> Vec<(&'static str, Value)> {
    vec![
        ("completeness_level", present(json!("counts"))),
        (
            "noise_model",
            present(json!("poisson_from_counts_and_shots")),
        ),
        (
            "shots_per_point",
            present(json!(vec![SHOTS_PER_POINT; delays().len()])),
        ),
        (
            "uncertainty_array",
            present(json!({
                "method": "poisson_from_counts",
                "file": "uncertainty.bin",
                "how_computed": "The square root of the counts in each pixel, as 64-bit little-endian floats in the order the spectrogram holds them. No field says how to read this file, which examples/synthetic/README.md records."
            })),
        ),
        ("inter_scan_scatter", not_applicable()),
        (
            "scan_combination",
            present(json!({ "scan_count": 1, "how_combined": "single_scan" })),
        ),
        (
            "acquisition_order",
            present(json!({
                "kind": "monotonic_increasing",
                "description": "The delay points are evaluated in the order the axis lists them."
            })),
        ),
        ("acquisition_timestamps", not_applicable()),
        ("monitoring_channels", not_applicable()),
    ]
}

/// Where the dataset came from.
fn provenance_fields() -> Vec<(&'static str, Value)> {
    vec![
        (
            "converter",
            present(json!({
                "conversion": {
                    "name": "synthetic-example",
                    "version": env!("CARGO_PKG_VERSION"),
                    "code_reference": "tool/src/bin/synthetic_example.rs",
                    "invocation": "cargo run --locked --offline --manifest-path tool/Cargo.toml --bin synthetic-example -- examples/synthetic/neon-2p-streaking",
                    "source_export_checksum": "none: nothing was converted, the array was computed from this document",
                    "conversion_date": PRODUCED_ON
                },
                "description": "This dataset was computed rather than converted. The block is required and has no way to say that, so the checksum slot carries the sentence instead of a digest, and examples/synthetic/README.md records it."
            })),
        ),
        ("processing_history", present(json!([]))),
        ("instrument", not_applicable()),
        ("facility", not_applicable()),
        ("measurement_date", not_applicable()),
        ("publication_reference", not_applicable()),
    ]
}
