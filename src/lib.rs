// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-apt-mint: apt mint component model.
//!
//! Real implementation: mints, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mint {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AptMintError {
    EmptyMintName,
    DuplicateMint(String),
    EmptyEventSubject,
}

impl fmt::Display for AptMintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMintName => write!(f, "mint name must not be empty"),
            Self::DuplicateMint(id) => write!(f, "duplicate mint {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for AptMintError {}

pub struct AptMint {
    pub name: String,
    mints: BTreeMap<String, Mint>,
    events: BTreeMap<String, Event>,
}

impl AptMint {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            mints: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_mint(&mut self, item: Mint) -> Result<(), AptMintError> {
        if item.name.trim().is_empty() {
            return Err(AptMintError::EmptyMintName);
        }
        let id = item.id.clone();
        if self.mints.contains_key(&id) {
            return Err(AptMintError::DuplicateMint(id.clone()));
        }
        self.mints.insert(id, item);
        Ok(())
    }

    pub fn mint_count(&self) -> usize {
        self.mints.len()
    }

    pub fn get_mint(&self, id: &str) -> Option<&Mint> {
        self.mints.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), AptMintError> {
        if event.subject.trim().is_empty() {
            return Err(AptMintError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_mint(id: &str, name: &str) -> Mint {
        Mint {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = AptMint::new("AptMint");
        assert_eq!(m.name, "AptMint");
        assert_eq!(m.mint_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_mint() {
        let mut m = AptMint::new("AptMint");
        m.add_mint(sample_mint("a1", "Alpha")).unwrap();
        assert_eq!(m.mint_count(), 1);
        assert!(m.get_mint("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = AptMint::new("AptMint");
        assert!(m.add_mint(sample_mint("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = AptMint::new("AptMint");
        m.add_mint(sample_mint("x", "One")).unwrap();
        assert!(m.add_mint(sample_mint("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = AptMint::new("AptMint");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
