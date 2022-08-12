// MIT License
//
// Copyright (c) 2022 Reto Achermann
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use bit_field::BitField;

use crate::aarch64::vm::granule4k::consts::*;
use crate::aarch64::vm::granule4k::PAddr;
use crate::aarch64::vm::granule4k::VAddr;

use crate::aarch64::vm::descriptor_attributes::*;
use crate::{lower_attributes_impl, upper_attributes_impl};

/// A L3 Table Entry consists of an address and attributes in a 64-bit word.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct L3TableEntry(pub u64);

impl L3TableEntry {
    /// creates a new L3TableEntry
    pub fn new() -> Self {
        L3TableEntry(0)
    }

    /// creates a new L3Table Entry with the given address and flags
    pub fn with_frame(frame: PAddr, flags: u64) -> L3TableEntry {
        // assert!(pt_val == pt.into());
        // assert!(pt % BASE_PAGE_SIZE == 0);
        let mut entry = L3TableEntry::new();
        entry.frame_address(frame);
        entry
    }

    /// clears the entry
    pub fn clear(&mut self) -> &mut Self {
        self.0 = 0;
        self
    }

    /// checks whether the entry is valid
    pub fn is_valid(&self) -> bool {
        self.0.get_bit(0)
    }

    /// sets the entry to be valid
    pub fn valid(&mut self) -> &mut Self {
        self.0.set_bit(0, true);
        // bit 1 must always be set
        self.0.set_bit(1, true);
        self
    }

    /// marks the entry as invalid
    pub fn invalid(&mut self) -> &mut Self {
        self.0.set_bit(0, false);
        // also clear bit 1 otherwise we're in the reserved state
        self.0.set_bit(1, false);
        self
    }

    /// obtains the physical address of the entry
    pub fn get_paddr(&self) -> PAddr {
        PAddr::from(self.0.get_bits(12..48) << BASE_PAGE_SHIFT)
    }

    // sets the address
    pub fn frame_address(&mut self, frame: PAddr) -> &mut Self {
        assert!(frame % BASE_PAGE_SIZE == 0);
        self.0.set_bits(12..48, frame.as_u64() >> BASE_PAGE_SHIFT);
        self
    }
}

// attribute implementation
lower_attributes_impl!(L3TableEntry);
upper_attributes_impl!(L3TableEntry);

impl Default for L3TableEntry {
    fn default() -> Self {
        L3TableEntry::new()
    }
}

// #[repr(transparent)]
#[repr(align(4096))]
#[derive(Clone)]
pub struct L3Table([L3TableEntry; L3_TABLE_ENTRIES]);

impl L3Table {
    /// obtains the table as a slice of entries
    pub fn as_slice(&self) -> &[L3TableEntry] {
        &self.0
    }

    /// obtains the table as a slice of entries
    pub fn as_slice_mut(&mut self) -> &mut [L3TableEntry] {
        &mut self.0
    }

    /// obtains a reference to the entry
    pub fn entry(&self, idx: usize) -> &L3TableEntry {
        &self.0[idx]
    }

    /// obtains a reference to the entry
    pub fn entry_mut(&mut self, idx: usize) -> &mut L3TableEntry {
        &mut self.0[idx]
    }
}

impl From<&L3Table> for VAddr {
    fn from(num: &L3Table) -> Self {
        VAddr(num as *const _ as u64)
    }
}

impl From<&L3Table> for PAddr {
    fn from(num: &L3Table) -> Self {
        PAddr::from(VAddr::from(num))
    }
}
