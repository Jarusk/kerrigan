use std::env;


const DEFAULT_SIZE: u8 = 8;
const DEFAULT_THREADS: u8 = 1;


pub struct ConfigOptions {
    help: bool,
    num_queens: u8,
    num_threads: u8,
}

impl ConfigOptions {
    pub fn new() -> ConfigOptions {
        ConfigOptions{help: false, num_queens: DEFAULT_SIZE, num_threads: DEFAULT_THREADS}
    }

    pub fn parse_cmd_args(&mut self) {
        self.help = true;
    }
}