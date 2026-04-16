use nanorand::{ChaCha, Rng};

pub mod err;
pub mod generate_name;
pub mod pad;
pub mod pattern;
pub mod phonemes;
pub mod weighted_pick;

pub fn pick<'a>(arr: &'a [&str], rng: &mut ChaCha<8>) -> &'a str {
    if arr.is_empty() {
        return "";
    }
    let idx = rng.generate::<usize>() % arr.len();
    &arr[idx]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pick_empty() {
        let mut rng = ChaCha::new();
        let result = pick(&[], &mut rng);
        assert_eq!(result, "");
    }

    #[test]
    fn test_pick_single() {
        let mut rng = ChaCha::new();
        let result = pick(&["only"], &mut rng);
        assert_eq!(result, "only");
    }

    #[test]
    fn test_pick_distributes() {
        let arr = &["a", "b", "c", "d", "e"];
        let mut counts = [0; 5];
        let mut rng = ChaCha::new();

        for _ in 0..10000 {
            let picked = pick(arr, &mut rng);
            if let Some(idx) = arr.iter().position(|&s| s == picked) {
                counts[idx] += 1;
            }
        }

        for count in counts {
            assert!((count as f64 / 10000.0 - 0.2).abs() < 0.05);
        }
    }
}
