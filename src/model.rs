pub struct Params {
    verbose: bool,
    path: String,
}

impl Params {
    pub fn new(verbose: bool, path: String) -> Self {
        Params { verbose, path }
    }

    pub fn set_verbose(&mut self, v: bool) {
        self.verbose = v;
    }

    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }

    pub fn get_verbose(&self) -> bool {
        self.verbose
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
}
