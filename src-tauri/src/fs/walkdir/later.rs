use std::{
    path::Path,
    pin::Pin,
    task::{Context, Poll, Waker},
};

use super::entry_decision::EntryDecision;
use crate::path::PathExt;
use arrayvec::ArrayVec;
use futures::Stream;
use hashbrown::HashMap;
use jwalk::{ClientState, DirEntry as JWDirEntry};
pub type LaterEntryArray<const N: usize> = HashMap<String, LaterEntry>;
pub type DecidedEntryArray<const N: usize> = ArrayVec<LaterEntry, N>;

/// all the entries that need user input will stored in this struct
pub struct LaterEntries<const N: usize> {
    entries: LaterEntryArray<N>,
    decided: DecidedEntryArray<N>,
    decision: EntryDecision,
    waker: Option<Waker>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LaterEntryResult {
    /// the entry was pushed to the LaterEntries
    Ok,
    /// the LaterEntries is full, this was the last entry added to the LaterEntries
    Full,
    /// Err, the LaterEntries is full or entries were taken from the LaterEntries
    Err,
}

impl<const N: usize> LaterEntries<N> {
    pub(super) fn new() -> Self {
        Self {
            entries: HashMap::with_capacity(N),
            decided: ArrayVec::new(),
            decision: EntryDecision::NeedInput,
            waker: None,
            // index: 0,
        }
    }

    pub(super) fn decided_count(&self) -> usize {
        self.decided.len()
    }

    /// handles how duplicate/ modified entries are handled
    /// once the user has made a decision for all the entries
    /// it will set this to provided 'state'
    pub fn update_decision_all(&mut self, decision: EntryDecision) {
        self.decision = decision;
    }

    pub fn decision(&self) -> &EntryDecision {
        &self.decision
    }

    /// push the entry to the LaterEntries
    /// this will return true if the entry was pushed
    pub(super) fn push(&mut self, entry: LaterEntry) -> LaterEntryResult {
        if self.is_full() {
            return LaterEntryResult::Err;
        }

        let path = match &entry {
            LaterEntry::Modified(e, ..) => e.path().to_string(),
            LaterEntry::Duplicate(e, ..) => e.path().to_string(),
        };

        // don't care if the entry could  not be validated returned and err
        if path == Path::unknown_path() {
            // TODO: add log here for unknown path found
            return LaterEntryResult::Ok;
        }

        self.entries.insert(path, entry);

        if self.is_full() {
            return LaterEntryResult::Full;
        }

        return LaterEntryResult::Ok;
    }

    pub(super) fn decided_for(&mut self, path: String, decision: EntryDecision) {
        // this is useless case should be avoided
        if decision == EntryDecision::NeedInput {
            return;
        }

        if let Some(entry) = self.entries.remove(&path) {
            match entry {
                LaterEntry::Modified(e, ..) => {
                    self.decided.push(LaterEntry::Modified(e, decision));
                }
                LaterEntry::Duplicate(e, ..) => {
                    self.decided.push(LaterEntry::Duplicate(e, decision));
                }
            };

            if let Some(waker) = &self.waker {
                waker.wake_by_ref()
            }
        }
    }

    /// checks if the LaterEntries is empty
    /// this will return true if the LaterEntries is empty
    pub(super) fn is_empty(&self) -> bool {
        self.entries.len() == 0
    }

    /// checks if the LaterEntries is full
    /// this will return true if the LaterEntries is full
    pub(super) fn is_full(&self) -> bool {
        self.entries.len() == N
    }

    /// this resets the LaterEntries
    /// this will reset the LaterEntries to its initial state
    pub(super) fn reset(&mut self) {
        *self = Self::new();
    }
}

impl<const N: usize> Iterator for LaterEntries<N> {
    type Item = LaterEntry;

    /// this will return the entry in reverse order
    fn next(&mut self) -> Option<Self::Item> {
        if self.decided_count() == 0 {
            return None;
        }

        self.decided.pop()
    }
}

impl<const N: usize> Stream for LaterEntries<N> {
    type Item = LaterEntry;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // TODO
        todo!()
    }
}
