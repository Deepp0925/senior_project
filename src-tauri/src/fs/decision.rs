use walkdir::DirEntry as WalkDirEntry;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Decision {
    /// Needs user input to continue with this entry
    /// this is the default state
    NeedInput,
    /// Ignore this entry, do not copy it
    Skip,
    /// Override this entry, replacing existing files
    Replace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UserDecision {
    /// Ignore this entry, do not copy it
    Skip,
    /// Ignore all entries, do not copy them
    SkipAll,
    /// Override this entry, replacing existing files
    Replace,
    /// Override all entries, replacing existing files
    ReplaceAll,
}

#[derive(Debug)]
pub enum DecisionEntry {
    /// Duplicate entry
    Duplicate(WalkDirEntry, Decision),
    /// Modified entry
    Modified(WalkDirEntry, Decision),
}

impl DecisionEntry {
    pub fn update_decision(self, decision: UserDecision) -> Self {
        let decision = match decision {
            UserDecision::Skip => Decision::Skip,
            UserDecision::SkipAll => Decision::Skip,
            UserDecision::Replace => Decision::Replace,
            UserDecision::ReplaceAll => Decision::Replace,
        };

        match self {
            Self::Duplicate(entry, ..) => Self::Duplicate(entry, decision),
            Self::Modified(entry, ..) => Self::Modified(entry, decision),
        }
    }
}
