use nanorand::{ChaCha, Rng};

fn get_in_range(rng: &mut ChaCha<8>, min: u32, max: u32) -> u32 {
    min + rng.generate::<u32>() % (max - min)
}

pub fn weighted_pick<'a, T>(rng: &mut ChaCha<8>, items: &'a [(T, u8)]) -> Option<&'a T> {
    let total_weight: u32 = items.iter().map(|(_, weight)| *weight as u32).sum();
    if total_weight == 0 {
        return None;
    }

    let mut pick = get_in_range(rng, 0, total_weight);

    for (item, weight) in items {
        let weight = *weight as u32;
        if pick < weight {
            return Some(item);
        }
        pick -= weight;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weighted_pick() {
        let items = vec![("apple", 5u8), ("banana", 3), ("cherry", 2)];
        let mut counts = [0; 3];

        let mut rng = ChaCha::new();

        for _ in 0..10000 {
            if let Some(item) = weighted_pick(&mut rng, &items) {
                match *item {
                    "apple" => counts[0] += 1,
                    "banana" => counts[1] += 1,
                    "cherry" => counts[2] += 1,
                    _ => (),
                }
            }
        }

        let total = counts.iter().sum::<i32>() as f64;
        let apple_ratio = counts[0] as f64 / total;
        let banana_ratio = counts[1] as f64 / total;
        let cherry_ratio = counts[2] as f64 / total;

        assert!((apple_ratio - 0.5).abs() < 0.05);
        assert!((banana_ratio - 0.3).abs() < 0.05);
        assert!((cherry_ratio - 0.2).abs() < 0.05);
    }

    #[test]
    fn test_zero_weights() {
        let items = vec![("apple", 0u8), ("banana", 0), ("cherry", 0)];
        let mut rng = ChaCha::new();
        assert!(weighted_pick(&mut rng, &items).is_none());
    }

    #[test]
    fn test_will_never_pick_zero_weight() {
        let items = vec![("apple", 5u8), ("banana", 3), ("cherry", 0)];
        let mut counts = [0; 2];

        let mut rng = ChaCha::new();

        for _ in 0..10000 {
            if let Some(item) = weighted_pick(&mut rng, &items) {
                match *item {
                    "apple" => counts[0] += 1,
                    "banana" => counts[1] += 1,
                    _ => (),
                }
            }
        }

        assert_eq!(counts[0] + counts[1], 10000);
        assert!(counts[2..].iter().all(|&c| c == 0));
    }
}
