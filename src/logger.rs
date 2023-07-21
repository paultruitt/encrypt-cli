pub struct Logger {
    verbosity: u8
}

impl Logger {
    pub fn new(verbosity: u8) ->  Self {
        Logger { verbosity }
    }

    pub fn log(&self, msg: &str) {
        if self.verbosity > 0 {
            println!("{}", msg)
        }
    }

    pub fn debug(&self, msg: &str) {
        if self.verbosity > 1 {
            println!("{}", msg)
        }
    }
}
