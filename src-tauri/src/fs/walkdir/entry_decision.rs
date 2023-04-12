#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EntryDecision {
    /// Needs user input to continue with this entry
    /// this is the default state
    NeedInput,
    /// Ignore this entry, do not copy it
    Skip,
    /// Override this entry, replacing existing files
    Replace,
}
