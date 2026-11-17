//! Server-side equipment enhancement state and invariant checks.
//!
//! This module intentionally does not invent item IDs, material costs, success
//! rates, or client packet layouts. Those are data/protocol inputs to a later
//! integration step. The helpers here define safe state transitions for the
//! progression requested in the GDD.

use serde::{Deserialize, Serialize};

pub const MAX_PLUS: u8 = 12;
pub const MAX_PERFECTION: u8 = 54;
pub const MAX_SOUL_LEVEL: u8 = 7;
pub const SOCKET_COUNT: usize = 2;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct EquipmentProgress {
    pub plus: u8,
    pub sockets: [Option<u32>; SOCKET_COUNT],
    pub soul_level: u8,
    pub perfection: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpgradeError {
    MaxLevel,
    InvalidSocket,
    SocketOccupied,
    SocketEmpty,
    InvalidLevel,
}

impl EquipmentProgress {
    /// Apply one successful + upgrade. Call only after the server has
    /// validated and consumed the configured material/fee atomically.
    pub fn increase_plus(&mut self) -> Result<u8, UpgradeError> {
        if self.plus >= MAX_PLUS {
            return Err(UpgradeError::MaxLevel);
        }
        self.plus += 1;
        Ok(self.plus)
    }

    /// Open a socket (0 or 1). The caller must validate and consume its
    /// configured material before committing the returned state.
    pub fn open_socket(&mut self, socket: usize) -> Result<(), UpgradeError> {
        let slot = self.sockets.get_mut(socket).ok_or(UpgradeError::InvalidSocket)?;
        if slot.is_some() {
            return Err(UpgradeError::SocketOccupied);
        }
        // Some(None) represents a valid but empty socket; socket openness is
        // tracked separately by `open_sockets` in persisted state.
        self.open_sockets[socket] = true;
        Ok(())
    }

    /// Number of unlocked gem slots. Kept separately from gem IDs so opening
    /// an empty socket is distinguishable from a closed one.
    #[serde(default)]
    pub open_sockets: [bool; SOCKET_COUNT],

    pub fn insert_gem(&mut self, socket: usize, gem_type: u32) -> Result<(), UpgradeError> {
        let Some(is_open) = self.open_sockets.get(socket) else {
            return Err(UpgradeError::InvalidSocket);
        };
        if !is_open {
            return Err(UpgradeError::InvalidSocket);
        }
        let slot = self.sockets.get_mut(socket).ok_or(UpgradeError::InvalidSocket)?;
        if slot.is_some() {
            return Err(UpgradeError::SocketOccupied);
        }
        if gem_type == 0 {
            return Err(UpgradeError::InvalidLevel);
        }
        *slot = Some(gem_type);
        Ok(())
    }

    pub fn remove_gem(&mut self, socket: usize) -> Result<u32, UpgradeError> {
        let Some(is_open) = self.open_sockets.get(socket) else {
            return Err(UpgradeError::InvalidSocket);
        };
        if !is_open {
            return Err(UpgradeError::InvalidSocket);
        }
        self.sockets.get_mut(socket).ok_or(UpgradeError::InvalidSocket)?
            .take().ok_or(UpgradeError::SocketEmpty)
    }

    pub fn set_soul_level(&mut self, level: u8) -> Result<(), UpgradeError> {
        if level > MAX_SOUL_LEVEL {
            return Err(UpgradeError::InvalidLevel);
        }
        self.soul_level = level;
        Ok(())
    }

    pub fn set_perfection(&mut self, level: u8) -> Result<(), UpgradeError> {
        if level > MAX_PERFECTION {
            return Err(UpgradeError::InvalidLevel);
        }
        self.perfection = level;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plus_and_perfection_are_bounded() {
        let mut p = EquipmentProgress::default();
        for expected in 1..=MAX_PLUS {
            assert_eq!(p.increase_plus(), Ok(expected));
        }
        assert_eq!(p.increase_plus(), Err(UpgradeError::MaxLevel));
        assert_eq!(p.set_perfection(MAX_PERFECTION), Ok(()));
        assert_eq!(p.set_perfection(MAX_PERFECTION + 1), Err(UpgradeError::InvalidLevel));
        assert_eq!(p.set_soul_level(MAX_SOUL_LEVEL), Ok(()));
        assert_eq!(p.set_soul_level(MAX_SOUL_LEVEL + 1), Err(UpgradeError::InvalidLevel));
    }

    #[test]
    fn sockets_require_open_empty_slots_and_nonzero_gems() {
        let mut p = EquipmentProgress::default();
        assert_eq!(p.insert_gem(0, 700001), Err(UpgradeError::InvalidSocket));
        assert_eq!(p.open_socket(2), Err(UpgradeError::InvalidSocket));
        assert_eq!(p.open_socket(0), Ok(()));
        assert_eq!(p.insert_gem(0, 0), Err(UpgradeError::InvalidLevel));
        assert_eq!(p.insert_gem(0, 700001), Ok(()));
        assert_eq!(p.insert_gem(0, 700002), Err(UpgradeError::SocketOccupied));
        assert_eq!(p.remove_gem(0), Ok(700001));
        assert_eq!(p.remove_gem(0), Err(UpgradeError::SocketEmpty));
    }

    #[test]
    fn progress_round_trips_as_json_with_legacy_defaults() {
        let mut p = EquipmentProgress::default();
        p.open_socket(1).unwrap();
        p.insert_gem(1, 700001).unwrap();
        p.increase_plus().unwrap();
        let json = serde_json::to_string(&p).unwrap();
        assert_eq!(serde_json::from_str::<EquipmentProgress>(&json).unwrap(), p);
        assert_eq!(serde_json::from_str::<EquipmentProgress>("{}").unwrap(), EquipmentProgress::default());
    }
}
