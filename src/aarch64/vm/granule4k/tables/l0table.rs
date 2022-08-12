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
use crate::aarch64::vm::granule4k::L1Table;
use crate::aarch64::vm::granule4k::PAddr;
use crate::aarch64::vm::granule4k::VAddr;

use crate::aarch64::vm::descriptor_attributes::*;
use crate::{
    page_block_lower_attributes_impl, page_block_upper_attributes_impl,
    table_next_level_attributes_impl,
};

// VMSAv8-64 translation table level -1, level 0, level 1, and level 2 descriptor formats
// The difference in the level -1, level 0, level 1 and level 2 VMSAv8-64 Translation Table
// descriptor formats depends on the following:
//  * The translation granule size.
//  * Whether a Block descriptor is permitted.
//  * If a Block descriptor is permitted, the size of the memory region described by that entry.
//  * The maximum supported OA size.

/// A L0 Table Descriptor
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct L0Descriptor(u64);

impl L0Descriptor {
    /// creates a new L0Descriptor
    pub fn new() -> Self {
        let mut t = L0Descriptor(0);
        // has bit 1 always set to 1
        t.0.set_bit(1, true);
        t
    }

    /// creates a new L0 table descriptor with the given L1 table descriptor
    pub fn with_table(table: &L1Table) -> Self {
        let mut entry = L0Descriptor::new();
        entry.table(table);
        entry
    }

    /// clears the descriptor by zeroing it
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
        self
    }

    /// marks the entry as invalid
    pub fn invalid(&mut self) -> &mut Self {
        self.0.set_bit(0, false);
        self
    }

    /// obtains the physical address of the entry
    pub fn get_paddr(&self) -> Option<PAddr> {
        if self.is_valid() {
            Some(PAddr::from(self.0.get_bits(12..48) << BASE_PAGE_SHIFT))
        } else {
            None
        }
    }

    // sets the frame address of the entry
    pub fn table(&mut self, table: &L1Table) -> &mut Self {
        let pt = PAddr::from(table);
        self.0.set_bits(12..48, pt.as_u64() >> BASE_PAGE_SHIFT);
        self
    }
}

table_next_level_attributes_impl!(L0Descriptor);

impl Default for L0Descriptor {
    fn default() -> Self {
        L0Descriptor::new()
    }
}

// #[repr(transparent)]
#[repr(align(4096))]
#[derive(Clone)]
pub struct L0Table([L0Descriptor; L0_TABLE_ENTRIES]);

impl L0Table {
    /// obtains the table as a slice of entries
    pub fn as_slice(&self) -> &[L0Descriptor] {
        &self.0
    }

    /// obtains the table as a slice of entries
    pub fn as_slice_mut(&mut self) -> &mut [L0Descriptor] {
        &mut self.0
    }

    /// obtains a reference to the entry
    pub fn entry(&self, idx: usize) -> &L0Descriptor {
        &self.0[idx]
    }

    /// obtains a reference to the entry
    pub fn entry_mut(&mut self, idx: usize) -> &mut L0Descriptor {
        &mut self.0[idx]
    }
}

impl From<&L0Table> for VAddr {
    fn from(num: &L0Table) -> Self {
        VAddr(num as *const _ as u64)
    }
}

impl From<&L0Table> for PAddr {
    fn from(num: &L0Table) -> Self {
        PAddr::from(VAddr::from(num))
    }
}
