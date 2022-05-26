#[derive(Debug)]
pub struct Config {
    /// Maximum call stack depth.
    pub max_call_depth: usize,

    /// Maximum iteration count.
    pub max_iter_count: usize,

    /// Maximum amount of concretizations for function pointers.
    pub max_fn_ptr_resolutions: usize,

    /// Maximum amount of concretizations for a memory address. This does not apply for e.g.
    /// ArrayMemory, but does apply for ObjectMemory.
    pub max_memory_access_resolutions: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            max_call_depth: 10,
            max_iter_count: 10,
            max_fn_ptr_resolutions: 10,
            max_memory_access_resolutions: 10,
        }
    }
}
