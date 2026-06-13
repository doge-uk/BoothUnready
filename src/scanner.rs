use crate::config::{AudioFormat, Device};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use symphonia::core::codecs::CodecParameters;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetectedProperties {
    pub file_path: PathBuf,
    pub format: Option<String>,
    pub bit_depth: Option<u16>,
    pub sample_rate: Option<u32>,
    pub bitrate: Option<u32>,
    pub encoding: Option<String>,
    pub wav_format_tag: Option<u16>,
    pub extra_data: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResult {
    pub file_path: PathBuf,
    pub detected_properties: DetectedProperties,
    pub device_results: HashMap<String, DeviceScanResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceScanResult {
    pub compatible: bool,
    pub failures: Vec<String>,
}

pub fn scan_file(path: &Path, devices: &[Device]) -> ScanResult {
    let mut hint = Hint::new();
    let source = match std::fs::File::open(path) {
        Ok(f) => Box::new(f),
        Err(e) => {
            let mut device_results = HashMap::new();
            for device in devices {
                device_results.insert(
                    device.name.clone(),
                    DeviceScanResult {
                        compatible: false,
                        failures: vec![format!("Error opening file: {}", e)],
                    },
                );
            }
            return ScanResult {
                file_path: path.to_path_buf(),
                detected_properties: DetectedProperties {
                    file_path: path.to_path_buf(),
                    format: None,
                    bit_depth: None,
                    sample_rate: None,
                    bitrate: None,
                    encoding: None,
                    wav_format_tag: None,
                    extra_data: None,
                },
                device_results,
            };
        }
    };

    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
        hint.with_extension(ext);
    }

    let mss = MediaSourceStream::new(source, Default::default());

    let format_opts = FormatOptions {
        enable_gapless: true,
        ..Default::default()
    };
    let metadata_opts: MetadataOptions = Default::default();

    let probed = match symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts) {
        Ok(p) => p,
        Err(e) => {
            let mut device_results = HashMap::new();
            for device in devices {
                device_results.insert(
                    device.name.clone(),
                    DeviceScanResult {
                        compatible: false,
                        failures: vec![format!("Unsupported format or error probing: {}", e)],
                    },
                );
            }
            return ScanResult {
                file_path: path.to_path_buf(),
                detected_properties: DetectedProperties {
                    file_path: path.to_path_buf(),
                    format: None,
                    bit_depth: None,
                    sample_rate: None,
                    bitrate: None,
                    encoding: None,
                    wav_format_tag: None,
                    extra_data: None,
                },
                device_results,
            };
        }
    };

    let format_reader = probed.format;
    let track = format_reader.default_track().unwrap();
    let codec_params = track.codec_params.clone();

    let bit_depth = codec_params.bits_per_sample.map(|b| b as u16);
    let sample_rate = codec_params.sample_rate;
    
    let bitrate = if let (Some(n_frames), Some(time_base)) = (codec_params.n_frames, codec_params.time_base) {
        let duration = time_base.calc_time(n_frames);
        let total_seconds = duration.seconds as f64 + duration.frac;
        if total_seconds > 0.0 {
            let file_size = path.metadata().map(|m| m.len()).unwrap_or(0);
            Some(((file_size * 8) as f64 / total_seconds) as u32)
        } else {
            None
        }
    } else {
        None
    };

    let detected_format = get_audio_format(path, &codec_params);
    let codec_name = symphonia::default::get_codecs().get_codec(codec_params.codec).map(|d| d.long_name);

let wav_format_tag = if let Some(data) = &codec_params.extra_data {
    if data.len() >= 2 {
        let tag = u16::from_le_bytes([data[0], data[1]]);
        Some(tag)
    } else {
        None
    }
} else {

    None
};

    let props = DetectedProperties {
        file_path: path.to_path_buf(),
        format: detected_format.as_ref().map(|f| format!("{:?}", f)),
        bit_depth,
        sample_rate,
        bitrate,
        encoding: codec_name.map(|s| s.to_string()),
        wav_format_tag,
        extra_data: codec_params.extra_data.as_ref().map(|data| data.to_vec()),
    };

    let mut device_results = HashMap::new();

    for device in devices {
        let mut failures = Vec::new();
        let mut compatible = true;
        if let Some(format) = detected_format.clone() {
            if let Some(rules) = device.rules.get(&format) {
                // Check bit depth
                if let Some(depth) = props.bit_depth {
                    if !rules.supported_bit_depths.contains(&depth) {
                        compatible = false;
                        failures.push(format!(
                            "Unsupported bit depth: {} (supported: {:?})",
                            depth, rules.supported_bit_depths
                        ));
                    }
                }

                // Check sample rate
                if let Some(rate) = props.sample_rate {
                    if !rules.supported_sample_rates.contains(&rate) {
                        compatible = false;
                        failures.push(format!(
                            "Unsupported sample rate: {} Hz (supported: {:?})",
                            rate, rules.supported_sample_rates
                        ));
                    }
                }

                // Check bitrate
                if let Some((min, max)) = rules.supported_bitrates {
                    if let Some(br) = props.bitrate {
                        if br < min || br > max {
                            compatible = false;
                            failures.push(format!(
                                "Unsupported bitrate: {} bps (supported: {} - {} bps)",
                                br, min, max
                            ));
                        }
                    }
                }

                if rules.cbr_only {
                    failures.push(
                        "Device requires CBR, but VBR detection is not reliably implemented."
                            .to_string(),
                    );
                }
            } else {
                compatible = false;
                failures.push(format!(
                    "Format {:?} not supported by {}",
                    format, device.name
                ));
            }

            // Known failures
            // Known failures
            if format == AudioFormat::WAV {
                if props.bit_depth == Some(32) {
                    compatible = false;
                    failures.push("WAV: 32-bit float not supported".to_string());
                }
                if props.sample_rate.unwrap_or(0) > 96000 && device.name.starts_with("CDJ") {
                    compatible = false;
                    failures.push("WAV: Sample rate above 96000 Hz not supported".to_string());
                }
                
                // Read raw WAV header to detect EXTENSIBLE
                let raw_format_tag = read_wav_format_tag(path);

                
                if raw_format_tag == Some(0xFFFE) {
                    compatible = false;
                    failures.push("WAV: WAV EXTENSIBLE header format (wFormatTag = 0xFFFE) not supported".to_string());
                } else if props.wav_format_tag == Some(0xFFFE) {
                    compatible = false;
                    failures.push("WAV: WAV EXTENSIBLE header format (wFormatTag = 0xFFFE) not supported".to_string());
                }
            }
            if format == AudioFormat::AIFF {
                if let Some(encoding) = &props.encoding {
                    if encoding.contains("Compressed") {
                        compatible = false;
                        failures.push("AIFF: AIFF-C compressed variant not supported".to_string());
                    }
                }
            }
        } else {
            compatible = false;
            failures.push("Could not determine audio format.".to_string());
        }
        device_results.insert(
            device.name.clone(),
            DeviceScanResult {
                compatible,
                failures,
            },
        );
    }

    ScanResult {
        file_path: path.to_path_buf(),
        detected_properties: props,
        device_results,
    }
}

fn get_audio_format(path: &Path, params: &CodecParameters) -> Option<AudioFormat> {
    let extension = path.extension().and_then(|s| s.to_str()).map(|s| s.to_lowercase());
    let codec_name = symphonia::default::get_codecs()
        .get_codec(params.codec)
        .map(|d| d.long_name.to_ascii_lowercase());

    if let Some(codec_name) = codec_name {
        if codec_name.contains("mpeg audio layer 3") || codec_name.contains("layer-3") || codec_name.contains("layer 3") {
            return Some(AudioFormat::MP3);
        }

        if codec_name.contains("aac") {
            return Some(AudioFormat::AAC);
        }

        if codec_name.contains("pcm") {
            return match extension.as_deref() {
                Some("wav") => Some(AudioFormat::WAV),
                Some("aif") | Some("aiff") => Some(AudioFormat::AIFF),
                _ => None,
            };
        }

        if codec_name.contains("flac") {
            return Some(AudioFormat::FLAC);
        }

        if codec_name.contains("alac") || codec_name.contains("apple lossless") {
            return Some(AudioFormat::ALAC);
        }
    }

    match extension.as_deref() {
        Some("mp3") => Some(AudioFormat::MP3),
        Some("m4a") | Some("aac") | Some("mp4") => Some(AudioFormat::AAC),
        Some("wav") => Some(AudioFormat::WAV),
        Some("aif") | Some("aiff") => Some(AudioFormat::AIFF),
        Some("flac") | Some("fla") => Some(AudioFormat::FLAC),
        _ => None,
    }
}
// Add this function to read raw WAV header
fn read_wav_format_tag(path: &Path) -> Option<u16> {
    use std::io::Read;
    
    let mut file = std::fs::File::open(path).ok()?;
    let mut header = [0u8; 44]; // Read enough for WAV header
    
    if file.read_exact(&mut header).is_err() {
        return None;
    }
    
    // Check RIFF header
    if &header[0..4] != b"RIFF" {
        return None;
    }
    
    // Check WAVE format
    if &header[8..12] != b"WAVE" {
        return None;
    }
    
    // Find fmt chunk
    let mut pos = 12;
    while pos + 8 <= header.len() {
        let chunk_id = &header[pos..pos+4];
        let chunk_size = u32::from_le_bytes([header[pos+4], header[pos+5], header[pos+6], header[pos+7]]) as usize;
        
        if chunk_id == b"fmt " {
            if pos + 8 + 2 <= header.len() {
                let format_tag = u16::from_le_bytes([header[pos+8], header[pos+9]]);
                return Some(format_tag);
            }
        }
        
        pos += 8 + chunk_size;
        if pos >= header.len() {
            break;
        }
    }
    
    None
}