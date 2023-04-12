use rand::{distributions::Alphanumeric, Rng};

pub trait StringUtils {
    /// this function will generate a random string of length `len`
    fn random(length: usize) -> String;
}

impl StringUtils for String {
    fn random(length: usize) -> String {
        return rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();
    }
}
