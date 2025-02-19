use std::path::PathBuf;
use crate::error::AddonError;

#[derive(Debug)]
pub struct Sandbox {
    root_path: PathBuf,
    memory_limit: usize,
    cpu_limit: u32,
}

impl Sandbox {
    pub fn new(root_path: PathBuf) -> Self {
        Self {
            root_path,
            memory_limit: 100 * 1024 * 1024, // 100MB
            cpu_limit: 50, // 50% CPU
        }
    }

    pub fn execute<F, T>(&self, f: F) -> Result<T, AddonError>
    where
        F: FnOnce() -> Result<T, AddonError>,
    {
        // TODO: Implementar isolamento real de recursos
        f()
    }

    #[cfg(test)]
    pub fn with_limits(root_path: PathBuf, memory_limit: usize, cpu_limit: u32) -> Self {
        Self {
            root_path,
            memory_limit,
            cpu_limit,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_sandbox_creation() {
        let sandbox = Sandbox::new(PathBuf::from("./test"));
        assert_eq!(sandbox.memory_limit, 100 * 1024 * 1024);
        assert_eq!(sandbox.cpu_limit, 50);
    }

    #[test]
    fn test_sandbox_with_limits() {
        let sandbox = Sandbox::with_limits(
            PathBuf::from("./test"),
            50 * 1024 * 1024,
            25
        );
        assert_eq!(sandbox.memory_limit, 50 * 1024 * 1024);
        assert_eq!(sandbox.cpu_limit, 25);
    }
}
