pub mod ply;

#[derive(Debug)]
pub struct LoadError {
    info: String,
}

impl LoadError {
    pub(crate) fn new(info: String) -> Self {
        Self { info }
    }
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to load mesh, inner error: '{}'", self.info)
    }
}

impl std::error::Error for LoadError {}

#[derive(Debug)]
pub struct SaveError {
    info: String,
}

impl SaveError {
    pub(crate) fn new(info: String) -> Self {
        Self { info }
    }
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to save mesh, inner error: '{}'", self.info)
    }
}

impl std::error::Error for SaveError {}
