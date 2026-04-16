use crate::utils::{
    phonemes::{CODAS, NUCLEI, ONSETS, TAILS},
    pick,
};
use nanorand::ChaCha;

#[derive(Clone, Copy)]
pub enum Pattern {
    OnT,
    OnNuT,
    Cv,
    Cvc,
    Cvt,
    Cvcv,
    Cvct,
    Cvcvc,
    Vcvcv,
    Cvcvt,
    Cvctv,
    Cvcvct,
    Vcvcvc,
    Cvcvcv,
}

pub const PATTERNS: &[(Pattern, u8)] = &[
    (Pattern::OnT, 4),
    (Pattern::OnNuT, 3),
    (Pattern::Cv, 1),
    (Pattern::Cvc, 1),
    (Pattern::Cvt, 2),
    (Pattern::Cvcv, 3),
    (Pattern::Cvct, 3),
    (Pattern::Cvcvc, 2),
    (Pattern::Vcvcv, 2),
    (Pattern::Cvcvt, 2),
    (Pattern::Cvctv, 2),
    (Pattern::Cvcvct, 2),
    (Pattern::Vcvcvc, 2),
    (Pattern::Cvcvcv, 2),
];

pub fn build_pattern(pattern: &Pattern, rng: &mut ChaCha<8>) -> String {
    let parts: &[&[&str]] = match pattern {
        Pattern::OnT => &[ONSETS, TAILS],
        Pattern::OnNuT => &[ONSETS, NUCLEI, TAILS],
        Pattern::Cv => &[ONSETS, NUCLEI],
        Pattern::Cvc => &[ONSETS, NUCLEI, CODAS],
        Pattern::Cvt => &[ONSETS, NUCLEI, TAILS],
        Pattern::Cvcv => &[ONSETS, NUCLEI, ONSETS, NUCLEI],
        Pattern::Cvct => &[ONSETS, NUCLEI, CODAS, TAILS],
        Pattern::Cvcvc => &[ONSETS, NUCLEI, ONSETS, NUCLEI, CODAS],
        Pattern::Vcvcv => &[NUCLEI, ONSETS, NUCLEI, ONSETS, NUCLEI],
        Pattern::Cvcvt => &[ONSETS, NUCLEI, ONSETS, NUCLEI, TAILS],
        Pattern::Cvctv => &[ONSETS, NUCLEI, CODAS, TAILS, NUCLEI],
        Pattern::Cvcvct => &[ONSETS, NUCLEI, ONSETS, NUCLEI, CODAS, TAILS],
        Pattern::Vcvcvc => &[NUCLEI, ONSETS, NUCLEI, ONSETS, NUCLEI, CODAS],
        Pattern::Cvcvcv => &[ONSETS, NUCLEI, ONSETS, NUCLEI, ONSETS, NUCLEI],
    };
    parts.iter().map(|arr| pick(arr, rng)).collect()
}
