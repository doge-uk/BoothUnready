use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum AudioFormat {
    MP3,
    AAC,
    WAV,
    AIFF,
    FLAC,
    ALAC,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormatRules {
    pub supported_bit_depths: HashSet<u16>,
    pub supported_sample_rates: HashSet<u32>,
    pub supported_bitrates: Option<(u32, u32)>, // Min-Max for VBR/CBR
    pub cbr_only: bool,
    pub supported_encodings: HashSet<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum KnownFailure {
    Wav32BitFloat,
    WavHighSampleRate,
    WavExtensibleFormat,
    AiffCompressed,
    AacDrm,
    NotPlayableFromDisc,
    UnsupportedFormat(AudioFormat),
    SampleRateAbove48k,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Device {
    pub name: String,
    pub rules: std::collections::HashMap<AudioFormat, FormatRules>,
    pub known_failures: HashSet<String>,
}

pub fn get_devices() -> Vec<Device> {
    vec![
        Device {
            name: "CDJ-3000X".to_string(),
            rules: [
                (AudioFormat::MP3, FormatRules {
                    supported_bit_depths: [16].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000].iter().cloned().collect(),
                    supported_bitrates: Some((8_000, 400_000)),
                    cbr_only: false,
                    supported_encodings: ["MPEG-1 Audio Layer-3".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::AAC, FormatRules {
                    supported_bit_depths: [16].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000].iter().cloned().collect(),
                    supported_bitrates: Some((16_000, 320_000)),
                    cbr_only: false,
                    supported_encodings: ["MPEG-4 AAC LC".to_string(), "MPEG-2 AAC LC".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::WAV, FormatRules {
                    supported_bit_depths: [16, 24].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000, 88200, 96000].iter().cloned().collect(),
                    supported_bitrates: None,
                    cbr_only: false,
                    supported_encodings: ["PCM".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::AIFF, FormatRules {
                    supported_bit_depths: [16, 24].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000, 88200, 96000].iter().cloned().collect(),
                    supported_bitrates: None,
                    cbr_only: false,
                    supported_encodings: ["PCM".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::FLAC, FormatRules {
                    supported_bit_depths: [16, 24].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000, 88200, 96000].iter().cloned().collect(),
                    supported_bitrates: None,
                    cbr_only: false,
                    supported_encodings: ["FLAC".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::ALAC, FormatRules {
                    supported_bit_depths: [16, 24].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000, 88200, 96000].iter().cloned().collect(),
                    supported_bitrates: None,
                    cbr_only: false,
                    supported_encodings: ["ALAC".to_string()].iter().cloned().collect(),
                }),
            ].iter().cloned().collect(),
            known_failures: [
                "WAV: 32-bit float not supported".to_string(),
                "WAV: Sample rate above 96000 Hz not supported".to_string(),
                "WAV: WAV EXTENSIBLE header format (wFormatTag = 0xFFFE) not supported".to_string(),
                "AIFF: AIFF-C compressed variant not supported".to_string(),
                "AAC: DRM-protected files cannot be played".to_string(),
            ].iter().cloned().collect(),
        },
        Device {
            name: "CDJ-3000".to_string(),
            rules: [
                (AudioFormat::MP3, FormatRules {
                    supported_bit_depths: [16].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000].iter().cloned().collect(),
                    supported_bitrates: Some((8_000, 400_000)),
                    cbr_only: false,
                    supported_encodings: ["MPEG-1 Audio Layer-3".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::AAC, FormatRules {
                    supported_bit_depths: [16].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000].iter().cloned().collect(),
                    supported_bitrates: Some((16_000, 320_000)),
                    cbr_only: true,
                    supported_encodings: ["MPEG-4 AAC LC".to_string(), "MPEG-2 AAC LC".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::WAV, FormatRules {
                    supported_bit_depths: [16, 24].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000, 88200, 96000].iter().cloned().collect(),
                    supported_bitrates: None,
                    cbr_only: false,
                    supported_encodings: ["PCM".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::AIFF, FormatRules {
                    supported_bit_depths: [16, 24].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000, 88200, 96000].iter().cloned().collect(),
                    supported_bitrates: None,
                    cbr_only: false,
                    supported_encodings: ["PCM".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::FLAC, FormatRules {
                    supported_bit_depths: [16, 24].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000, 88200, 96000].iter().cloned().collect(),
                    supported_bitrates: None,
                    cbr_only: false,
                    supported_encodings: ["FLAC".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::ALAC, FormatRules {
                    supported_bit_depths: [16, 24].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000, 88200, 96000].iter().cloned().collect(),
                    supported_bitrates: None,
                    cbr_only: false,
                    supported_encodings: ["ALAC".to_string()].iter().cloned().collect(),
                }),
            ].iter().cloned().collect(),
            known_failures: [
                "WAV: 32-bit float not supported".to_string(),
                "WAV: Sample rate above 96000 Hz not supported".to_string(),
                "WAV: WAV EXTENSIBLE header format (wFormatTag = 0xFFFE) not supported".to_string(),
                "AIFF: AIFF-C compressed variant not supported".to_string(),
                "AAC: DRM-protected files cannot be played".to_string(),
            ].iter().cloned().collect(),
        },
        Device {
            name: "CDJ-2000NXS2".to_string(),
            rules: [
                (AudioFormat::MP3, FormatRules {
                    supported_bit_depths: [16].iter().cloned().collect(),
                    supported_sample_rates: [32000, 44100, 48000, 16000, 22050, 24000].iter().cloned().collect(),
                    supported_bitrates: Some((8_000, 400_000)),
                    cbr_only: false,
                    supported_encodings: ["MPEG-1 Audio Layer-3".to_string(), "MPEG-2 Audio Layer-3".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::AAC, FormatRules {
                    supported_bit_depths: [16].iter().cloned().collect(),
                    supported_sample_rates: [16000, 22050, 24000, 32000, 44100, 48000].iter().cloned().collect(),
                    supported_bitrates: Some((16_000, 320_000)),
                    cbr_only: false,
                    supported_encodings: ["MPEG-4 AAC LC".to_string(), "MPEG-2 AAC LC".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::WAV, FormatRules {
                    supported_bit_depths: [16, 24].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000, 88200, 96000].iter().cloned().collect(),
                    supported_bitrates: None,
                    cbr_only: false,
                    supported_encodings: ["PCM".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::AIFF, FormatRules {
                    supported_bit_depths: [16, 24].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000, 88200, 96000].iter().cloned().collect(),
                    supported_bitrates: None,
                    cbr_only: false,
                    supported_encodings: ["PCM".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::FLAC, FormatRules {
                    supported_bit_depths: [16, 24].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000, 88200, 96000].iter().cloned().collect(),
                    supported_bitrates: None,
                    cbr_only: false,
                    supported_encodings: ["FLAC".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::ALAC, FormatRules {
                    supported_bit_depths: [16, 24].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000, 88200, 96000].iter().cloned().collect(),
                    supported_bitrates: None,
                    cbr_only: false,
                    supported_encodings: ["ALAC".to_string()].iter().cloned().collect(),
                }),
            ].iter().cloned().collect(),
            known_failures: [
                "WAV/AIFF/FLAC/ALAC: Not playable from disc (CD/DVD)".to_string(),
                "WAV: 32-bit float not supported".to_string(),
                "WAV: WAV EXTENSIBLE header format (wFormatTag = 0xFFFE) not supported".to_string(),
                "AIFF: AIFF-C compressed variant not supported".to_string(),
                "AAC: DRM-protected files cannot be played".to_string(),
            ].iter().cloned().collect(),
        },
        Device {
            name: "XDJ-1000MK2".to_string(),
            rules: [
                (AudioFormat::MP3, FormatRules {
                    supported_bit_depths: [16].iter().cloned().collect(),
                    supported_sample_rates: [32000, 44100, 48000, 16000, 22050, 24000].iter().cloned().collect(),
                    supported_bitrates: Some((8_000, 400_000)),
                    cbr_only: false,
                    supported_encodings: ["MPEG-1 Audio Layer-3".to_string(), "MPEG-2 Audio Layer-3".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::AAC, FormatRules {
                    supported_bit_depths: [16].iter().cloned().collect(),
                    supported_sample_rates: [16000, 22050, 24000, 32000, 44100, 48000].iter().cloned().collect(),
                    supported_bitrates: Some((16_000, 320_000)),
                    cbr_only: false,
                    supported_encodings: ["MPEG-4 AAC LC".to_string(), "MPEG-2 AAC LC".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::WAV, FormatRules {
                    supported_bit_depths: [16, 24].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000].iter().cloned().collect(),
                    supported_bitrates: None,
                    cbr_only: false,
                    supported_encodings: ["PCM".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::AIFF, FormatRules {
                    supported_bit_depths: [16, 24].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000].iter().cloned().collect(),
                    supported_bitrates: None,
                    cbr_only: false,
                    supported_encodings: ["PCM".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::FLAC, FormatRules {
                    supported_bit_depths: [16, 24].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000].iter().cloned().collect(),
                    supported_bitrates: None,
                    cbr_only: false,
                    supported_encodings: ["FLAC".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::ALAC, FormatRules {
                    supported_bit_depths: [16, 24].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000].iter().cloned().collect(),
                    supported_bitrates: None,
                    cbr_only: false,
                    supported_encodings: ["ALAC".to_string()].iter().cloned().collect(),
                }),
            ].iter().cloned().collect(),
            known_failures: [
                "WAV/AIFF/FLAC/ALAC: Sample rates above 48000 Hz not supported".to_string(),
                "WAV: 32-bit float not supported".to_string(),
                "WAV: WAV EXTENSIBLE header format (wFormatTag = 0xFFFE) not supported".to_string(),
                "AIFF: AIFF-C compressed variant not supported".to_string(),
                "AAC: DRM-protected files cannot be played".to_string(),
            ].iter().cloned().collect(),
        },
        Device {
            name: "XDJ-700".to_string(),
            rules: [
                (AudioFormat::MP3, FormatRules {
                    supported_bit_depths: [16].iter().cloned().collect(),
                    supported_sample_rates: [32000, 44100, 48000, 16000, 22050, 24000].iter().cloned().collect(),
                    supported_bitrates: Some((8_000, 400_000)),
                    cbr_only: false,
                    supported_encodings: ["MPEG-1 Audio Layer-3".to_string(), "MPEG-2 Audio Layer-3".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::AAC, FormatRules {
                    supported_bit_depths: [16].iter().cloned().collect(),
                    supported_sample_rates: [16000, 22050, 24000, 32000, 44100, 48000].iter().cloned().collect(),
                    supported_bitrates: Some((16_000, 320_000)),
                    cbr_only: false,
                    supported_encodings: ["MPEG-4 AAC LC".to_string(), "MPEG-2 AAC LC".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::WAV, FormatRules {
                    supported_bit_depths: [16, 24].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000].iter().cloned().collect(),
                    supported_bitrates: None,
                    cbr_only: false,
                    supported_encodings: ["PCM".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::AIFF, FormatRules {
                    supported_bit_depths: [16, 24].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000].iter().cloned().collect(),
                    supported_bitrates: None,
                    cbr_only: false,
                    supported_encodings: ["PCM".to_string()].iter().cloned().collect(),
                }),
            ].iter().cloned().collect(),
            known_failures: [
                "FLAC: Not supported on this device".to_string(),
                "ALAC/Apple Lossless: Not supported on this device".to_string(),
                "WAV/AIFF: Sample rates above 48000 Hz not supported".to_string(),
                "WAV: 32-bit float not supported".to_string(),
                "WAV: WAV EXTENSIBLE header format (wFormatTag = 0xFFFE) not supported".to_string(),
                "AIFF: AIFF-C compressed variant not supported".to_string(),
                "AAC: DRM-protected files cannot be played".to_string(),
            ].iter().cloned().collect(),
        },
        Device {
            name: "CDJ-2000NXS".to_string(),
            rules: [
                (AudioFormat::MP3, FormatRules {
                    supported_bit_depths: [16].iter().cloned().collect(),
                    supported_sample_rates: [32000, 44100, 48000, 16000, 22050, 24000].iter().cloned().collect(),
                    supported_bitrates: Some((8_000, 400_000)),
                    cbr_only: false,
                    supported_encodings: ["MPEG-1 Audio Layer-3".to_string(), "MPEG-2 Audio Layer-3".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::AAC, FormatRules {
                    supported_bit_depths: [16].iter().cloned().collect(),
                    supported_sample_rates: [16000, 22050, 24000, 32000, 44100, 48000].iter().cloned().collect(),
                    supported_bitrates: Some((16_000, 320_000)),
                    cbr_only: false,
                    supported_encodings: ["MPEG-4 AAC LC".to_string(), "MPEG-2 AAC LC".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::WAV, FormatRules {
                    supported_bit_depths: [16, 24].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000].iter().cloned().collect(),
                    supported_bitrates: None,
                    cbr_only: false,
                    supported_encodings: ["PCM".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::AIFF, FormatRules {
                    supported_bit_depths: [16, 24].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000].iter().cloned().collect(),
                    supported_bitrates: None,
                    cbr_only: false,
                    supported_encodings: ["PCM".to_string()].iter().cloned().collect(),
                }),
            ].iter().cloned().collect(),
            known_failures: [
                "FLAC: Not supported on this device".to_string(),
                "ALAC/Apple Lossless: Not supported on this device".to_string(),
                "WAV/AIFF: Sample rates above 48000 Hz not supported".to_string(),
                "WAV: 32-bit float not supported".to_string(),
                "WAV: WAV EXTENSIBLE header format (wFormatTag = 0xFFFE) not supported".to_string(),
                "AIFF: AIFF-C compressed variant not supported".to_string(),
                "AAC: DRM-protected files cannot be played".to_string(),
            ].iter().cloned().collect(),
        },
        Device {
            name: "XDJ-1000".to_string(),
            rules: [
                (AudioFormat::MP3, FormatRules {
                    supported_bit_depths: [16].iter().cloned().collect(),
                    supported_sample_rates: [32000, 44100, 48000, 16000, 22050, 24000].iter().cloned().collect(),
                    supported_bitrates: Some((8_000, 400_000)),
                    cbr_only: false,
                    supported_encodings: ["MPEG-1 Audio Layer-3".to_string(), "MPEG-2 Audio Layer-3".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::AAC, FormatRules {
                    supported_bit_depths: [16].iter().cloned().collect(),
                    supported_sample_rates: [16000, 22050, 24000, 32000, 44100, 48000].iter().cloned().collect(),
                    supported_bitrates: Some((16_000, 320_000)),
                    cbr_only: false,
                    supported_encodings: ["MPEG-4 AAC LC".to_string(), "MPEG-2 AAC LC".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::WAV, FormatRules {
                    supported_bit_depths: [16, 24].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000].iter().cloned().collect(),
                    supported_bitrates: None,
                    cbr_only: false,
                    supported_encodings: ["PCM".to_string()].iter().cloned().collect(),
                }),
                (AudioFormat::AIFF, FormatRules {
                    supported_bit_depths: [16, 24].iter().cloned().collect(),
                    supported_sample_rates: [44100, 48000].iter().cloned().collect(),
                    supported_bitrates: None,
                    cbr_only: false,
                    supported_encodings: ["PCM".to_string()].iter().cloned().collect(),
                }),
            ].iter().cloned().collect(),
            known_failures: [
                "FLAC: Not supported on this device".to_string(),
                "ALAC/Apple Lossless: Not supported on this device".to_string(),
                "WAV/AIFF: Sample rates above 48000 Hz not supported".to_string(),
                "WAV: 32-bit float not supported".to_string(),
                "WAV: WAV EXTENSIBLE header format (wFormatTag = 0xFFFE) not supported".to_string(),
                "AIFF: AIFF-C compressed variant not supported".to_string(),
                "AAC: DRM-protected files cannot be played".to_string(),
            ].iter().cloned().collect(),
        },
    ]
}
