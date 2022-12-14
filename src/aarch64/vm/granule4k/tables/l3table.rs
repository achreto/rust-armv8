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
use crate::{page_block_lower_attributes_impl, page_block_upper_attributes_impl};

/// A L3 Table Entry consists of an address and attributes in a 64-bit word.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct L3Descriptor(u64);

impl L3Descriptor {
    /// creates a new L3Descriptor
    pub fn new() -> Self {
        L3Descriptor(0)
    }

    /// creates a new L3Table Entry with the given address and flags
    pub fn with_frame(frame: PAddr) -> L3Descriptor {
        // assert!(pt_val == pt.into());
        // assert!(pt % BASE_PAGE_SIZE == 0);
        let mut entry = L3Descriptor::new();
        entry.frame(frame).valid();
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
        PAddr::from(self.0.get_bits(12..=47) << BASE_PAGE_SHIFT)
    }

    /// obtains the physical address of the entry
    pub fn get_frame(&self) -> Option<PAddr> {
        if self.is_valid() {
            Some(PAddr::from(self.0.get_bits(12..=47) << BASE_PAGE_SHIFT))
        } else {
            None
        }
    }

    // sets the frame address of the entry
    pub fn frame(&mut self, frame: PAddr) -> &mut Self {
        assert!(frame % BASE_PAGE_SIZE == 0);
        self.0.set_bits(12..=47, frame.as_u64() >> BASE_PAGE_SHIFT);
        self
    }

    /// obtains the descriptor as an u64
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

// attribute implementation
page_block_lower_attributes_impl!(L3Descriptor);
page_block_upper_attributes_impl!(L3Descriptor);

impl Default for L3Descriptor {
    fn default() -> Self {
        L3Descriptor::new()
    }
}

// #[repr(transparent)]
#[repr(align(4096))]
#[derive(Clone)]
pub struct L3Table([L3Descriptor; L3_TABLE_ENTRIES]);

impl L3Table {
    /// obtains the table as a slice of entries
    pub fn as_slice(&self) -> &[L3Descriptor] {
        &self.0
    }

    /// obtains the table as a slice of entries
    pub fn as_slice_mut(&mut self) -> &mut [L3Descriptor] {
        &mut self.0
    }

    /// sets the reference to a L1 table
    ///
    /// # Panics
    ///
    /// The function panics if the entry is out of bounds or there has already been
    /// a valid entry set in the descriptor
    pub fn set_entry(&mut self, idx: usize, entry: L3Descriptor) {
        if idx < L3_TABLE_ENTRIES {
            if self.0[idx].is_valid() {
                panic!("L0Table::set_entry: entry is already valid, invalidate first");
            } else {
                self.0[idx] = entry;
            }
        } else {
            panic!(
                "table index {} out of supported range {}..{}",
                idx, 0, L3_TABLE_ENTRIES
            );
        }
    }

    /// sets the reference to a L2 table
    ///
    /// # Panics
    ///
    /// The function panics if there has already been a valid entry set in the descriptor
    pub fn set_entry_at_vaddr(&mut self, vaddr: VAddr, entry: L3Descriptor) {
        let idx = Self::index(vaddr);
        self.set_entry(idx, entry);
    }

    /// obtains a reference to the entry with the given index
    ///
    /// # Panics
    ///
    /// The function panics if the entry is out of bounds
    pub fn entry(&self, idx: usize) -> L3Descriptor {
        if idx < L3_TABLE_ENTRIES {
            self.0[idx]
        } else {
            panic!(
                "table index {} out of supported range {}..{}",
                idx, 0, L3_TABLE_ENTRIES
            );
        }
    }

    /// obtains the entry based on the vaddr
    pub fn entry_at_vaddr(&self, vaddr: VAddr) -> L3Descriptor {
        let idx = Self::index(vaddr);
        self.entry(idx)
    }

    /// obtains a reference to the entry with the given index
    ///
    /// # Panics
    ///
    /// The function panics if the entry is out of bounds
    pub fn entry_as_ref(&self, idx: usize) -> &L3Descriptor {
        if idx < L3_TABLE_ENTRIES {
            &self.0[idx]
        } else {
            panic!(
                "table index {} out of supported range {}..{}",
                idx, 0, L3_TABLE_ENTRIES
            );
        }
    }

    /// obtains the entry based on the vaddr
    pub fn entry_at_vaddr_as_ref(&self, vaddr: VAddr) -> &L3Descriptor {
        let idx = Self::index(vaddr);
        self.entry_as_ref(idx)
    }

    /// obtains a mutable reference to the entry with the given idnex
    pub fn entry_as_mut_ref(&mut self, idx: usize) -> &mut L3Descriptor {
        if idx < L3_TABLE_ENTRIES {
            &mut self.0[idx]
        } else {
            panic!(
                "table index {} out of supported range {}..{}",
                idx, 0, L3_TABLE_ENTRIES
            );
        }
    }

    /// obtains a mutable reference to the entry based on the vaddr
    pub fn entry_at_vaddr_as_mut_ref(&mut self, vaddr: VAddr) -> &mut L3Descriptor {
        let idx = Self::index(vaddr);
        self.entry_as_mut_ref(idx)
    }

    /// calculates the index of the entry based on the vaddr
    pub fn index(va: VAddr) -> usize {
        va.as_u64().get_bits(12..21) as usize
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
