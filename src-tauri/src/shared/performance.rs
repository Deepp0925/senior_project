/// This will be used to determine how to use available resources.
/// generally the faster the performance the more resources will be used
/// leading to more memory usage and power consumption
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Performance {
    Fast,
    Average,
    Slow,
}
