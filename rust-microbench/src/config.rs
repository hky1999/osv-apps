pub struct Config {
    pub rounds: usize,
    pub warmup_rounds: usize,
    pub bytes: usize,
    pub verbose: bool,
}

impl Config {
    pub fn get() -> Self {
        Config {
            rounds: 10000,
            warmup_rounds: 10000,
            bytes: 4096,
            verbose: false,
        }
    }
    pub fn rounds(&self) -> (usize, usize) {
        (self.rounds, self.warmup_rounds)
    }

    pub fn bytes(&self) -> usize {
        self.bytes
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }
}
