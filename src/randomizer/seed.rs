use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct RandomizerSeed {
    pub value: u64,
    pub display_string: String,
}

impl RandomizerSeed {
    pub fn new(seed_string: &str) -> Self {
        let value = if seed_string.trim().is_empty() {
            // Generate random seed if empty
            rand::thread_rng().gen()
        } else if let Ok(num) = seed_string.parse::<u64>() {
            // Use numeric seed directly
            num
        } else {
            // Hash string to create seed
            let mut hasher = DefaultHasher::new();
            seed_string.hash(&mut hasher);
            hasher.finish()
        };
        
        Self {
            value,
            display_string: seed_string.to_string(),
        }
    }
    
    pub fn create_rng(&self) -> ChaCha8Rng {
        ChaCha8Rng::seed_from_u64(self.value)
    }
    
    pub fn random() -> Self {
        let value = rand::thread_rng().gen();
        Self {
            value,
            display_string: value.to_string(),
        }
    }
}

impl Default for RandomizerSeed {
    fn default() -> Self {
        Self::random()
    }
}
