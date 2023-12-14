use rand::Rng;

pub fn create_random_number() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}
