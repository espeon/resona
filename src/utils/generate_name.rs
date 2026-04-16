use nanorand::ChaCha;

use crate::utils::{
    err::SonaError,
    pad::pad_to_len,
    pattern::{build_pattern, PATTERNS},
    weighted_pick::weighted_pick,
};

pub fn generate_name(rng: &mut ChaCha<8>, config: &crate::Config) -> Result<String, SonaError> {
    if config.min < 1 {
        return Err(SonaError::MinLengthTooSmall);
    }
    if config.min > config.max {
        return Err(SonaError::InvalidLengthRange(config.min, config.max));
    }

    let pattern = *weighted_pick(rng, PATTERNS).ok_or(SonaError::NoPatterns)?;

    let base = build_pattern(&pattern, rng);
    Ok(pad_to_len(&base, config.min, config.max, rng))
}
