#![no_std]

use embedded_audio_tools as tools;

mod freeverb;

pub use crate::freeverb::Freeverb;

// ========================
// Compile Time Calculation
// ========================

#[cfg(feature = "SR44k1")]
pub const SAMPLING_RATE: usize = 44_100;

#[cfg(feature = "SR48k")]
pub const SAMPLING_RATE: usize = 48_000;

#[cfg(feature = "SR88k2")]
pub const SAMPLING_RATE: usize = 88_200;

#[cfg(feature = "SR96k")]
pub const SAMPLING_RATE: usize = 96_000;

#[cfg(feature = "SR192")]
pub const SAMPLING_RATE: usize = 192_000;

const fn adjust_length(length: usize) -> usize {
    length * SAMPLING_RATE / 44100
}

const STEREO_SPREAD: usize = 23;
pub const TUNINGS: [usize; 24] = [
    adjust_length(1116),                 // COMB_TUNING_L1
    adjust_length(1116 + STEREO_SPREAD), // COMB_TUNING_R1
    adjust_length(1188),                 // COMB_TUNING_L2
    adjust_length(1188 + STEREO_SPREAD), // COMB_TUNING_R2
    adjust_length(1277),                 // COMB_TUNING_L3
    adjust_length(1277 + STEREO_SPREAD), // COMB_TUNING_R3
    adjust_length(1356),                 // COMB_TUNING_L4
    adjust_length(1356 + STEREO_SPREAD), // COMB_TUNING_R4
    adjust_length(1422),                 // COMB_TUNING_L5
    adjust_length(1422 + STEREO_SPREAD), // COMB_TUNING_R5
    adjust_length(1491),                 // COMB_TUNING_L6
    adjust_length(1491 + STEREO_SPREAD), // COMB_TUNING_R6
    adjust_length(1557),                 // COMB_TUNING_L7
    adjust_length(1557 + STEREO_SPREAD), // COMB_TUNING_R7
    adjust_length(1617),                 // COMB_TUNING_L8
    adjust_length(1617 + STEREO_SPREAD), // COMB_TUNING_R8
    adjust_length(556),                  // ALLPASS_TUNING_L1
    adjust_length(556 + STEREO_SPREAD),  // ALLPASS_TUNING_R1
    adjust_length(441),                  // ALLPASS_TUNING_L2
    adjust_length(441 + STEREO_SPREAD),  // ALLPASS_TUNING_R2
    adjust_length(341),                  // ALLPASS_TUNING_L3
    adjust_length(341 + STEREO_SPREAD),  // ALLPASS_TUNING_R3
    adjust_length(225),                  // ALLPASS_TUNING_L4
    adjust_length(225 + STEREO_SPREAD),  // ALLPASS_TUNING_R4
];

pub const MAX_BUFFER_SIZE: usize = {
    let mut sum = 0;
    let mut i = 0;

    while i != 24 {
        sum += TUNINGS[i];
        i += 1;
    }

    sum
};
