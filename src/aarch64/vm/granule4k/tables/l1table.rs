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
use crate::aarch64::vm::granule4k::L2Table;
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

/// A L1 Table Descriptor
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct L1DescriptorTable(u64);

impl L1DescriptorTable {
    /// creates a new L1DescriptorTable
    pub fn new() -> Self {
        let mut t = L1DescriptorTable(0);
        // has bit 1 always set to 1
        t.0.set_bit(1, true);
        t
    }

    /// creates a new L1 table descriptor with the given L2 table descriptor
    pub fn with_table(table: &L2Table) -> Self {
        let mut entry = L1DescriptorTable::new();
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
    pub fn table(&mut self, table: &L2Table) -> &mut Self {
        let pt = PAddr::from(table);
        self.0.set_bits(12..48, pt.as_u64() >> BASE_PAGE_SHIFT);
        self
    }
}

table_next_level_attributes_impl!(L1DescriptorTable);

impl Default for L1DescriptorTable {
    fn default() -> Self {
        L1DescriptorTable::new()
    }
}

/// A L1 Block Descriptor
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct L1DescriptorBlock(u64);

impl L1DescriptorBlock {
    /// creates a new L1DescriptorBlock
    pub fn new() -> Self {
        L1DescriptorBlock(0)
    }

    /// creates a new L1DescriptorBlock with the given frame
    pub fn with_frame(frame: PAddr) -> Self {
        let mut entry = L1DescriptorBlock::new();
        entry.frame(frame);
        entry
    }

    /// clears the descritpor by zeroing it
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
            Some(PAddr::from(self.0.get_bits(21..48) << LARGE_PAGE_SHIFT))
        } else {
            None
        }
    }

    // sets the frame address of the entry
    pub fn frame(&mut self, frame: PAddr) -> &mut Self {
        assert!(frame % LARGE_PAGE_SIZE == 0);
        self.0.set_bits(21..48, frame.as_u64() >> LARGE_PAGE_SHIFT);
        self
    }
}

page_block_lower_attributes_impl!(L1DescriptorBlock);
page_block_upper_attributes_impl!(L1DescriptorBlock);

impl Default for L1DescriptorBlock {
    fn default() -> Self {
        L1DescriptorBlock::new()
    }
}

/// A L1 Descriptor consists of an address and a attributes.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct L1Descriptor(u64);

impl L1Descriptor {
    /// creates a new L1Descriptor
    pub fn new() -> Self {
        L1Descriptor(0)
    }

    /// obtains the block descriptor from the L1 descriptor, if applicable
    pub fn to_block_descriptor(&self) -> Option<L1DescriptorBlock> {
        if !self.is_valid() {
            Some(L1DescriptorBlock(0))
        } else if self.is_block() {
            let mut val = self.0;
            // make sure the res0 fields are cleared
            val.set_bits(48..49, 0).set_bits(17..21, 0);
            Some(L1DescriptorBlock(self.0))
        } else {
            None
        }
    }

    /// obtains the table descriptor from the L1 descriptor, if applicable
    pub fn to_table_descriptor(&self) -> Option<L1DescriptorTable> {
        if !self.is_valid() {
            Some(L1DescriptorTable(0))
        } else if self.is_table() {
            let mut val = self.0;
            // make sure the res0 fields are cleared
            val.set_bits(48..51, 0);
            Some(L1DescriptorTable(val))
        } else {
            None
        }
    }

    /// clears the descriptory
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

    /// the entry is a block descriptor (super page mapping)
    pub fn is_block(&self) -> bool {
        self.is_valid() && !self.0.get_bit(1)
    }

    /// sets the entry to be the block descriptor
    pub fn block(&mut self, block: L1DescriptorBlock) -> &mut Self {
        self.0 = block.0;
        self
    }

    /// the entry is a table descriptor (points to a page table)
    pub fn is_table(&self) -> bool {
        self.is_valid() && self.0.get_bit(1)
    }

    /// sets the entry to be the table descriptor
    pub fn table(&mut self, table: L1DescriptorTable) -> &mut Self {
        self.0 = table.0;
        self
    }

    /// obtains the physical address of the entry
    pub fn get_paddr(&self) -> Option<PAddr> {
        if self.is_table() {
            let table = L1DescriptorTable(self.0);
            table.get_paddr()
        } else if self.is_block() {
            let block = L1DescriptorBlock(self.0);
            block.get_paddr()
        } else {
            None
        }
    }
}

impl Default for L1Descriptor {
    fn default() -> Self {
        L1Descriptor::new()
    }
}

impl From<L1DescriptorBlock> for L1Descriptor {
    fn from(other: L1DescriptorBlock) -> Self {
        L1Descriptor(other.0)
    }
}

impl From<L1DescriptorTable> for L1Descriptor {
    fn from(other: L1DescriptorTable) -> Self {
        L1Descriptor(other.0)
    }
}

// #[repr(transparent)]
#[repr(align(4096))]
#[derive(Clone)]
pub struct L1Table([L1Descriptor; L1_TABLE_ENTRIES]);

impl L1Table {
    /// obtains the table as a slice of entries
    pub fn as_slice(&self) -> &[L1Descriptor] {
        &self.0
    }

    /// obtains the table as a slice of entries
    pub fn as_slice_mut(&mut self) -> &mut [L1Descriptor] {
        &mut self.0
    }

    /// sets the reference to a L1 table
    ///
    /// # Panics
    ///
    /// The function panics if the entry is out of bounds or there has already been
    /// a valid entry set in the descriptor
    pub fn set_entry(&mut self, idx: usize, entry: L1Descriptor) {
        if idx < L1_TABLE_ENTRIES {
            if self.0[idx].is_valid() {
                panic!("L0Table::set_entry: entry is already valid, invalidate first");
            } else {
                self.0[idx] = entry;
            }
        } else {
            panic!("table index {} out of supported range {}..{}", idx, 0, L1_TABLE_ENTRIES);
        }
    }

    /// sets the reference to a L1 table
    ///
    /// # Panics
    ///
    /// The function panics if there has already been a valid entry set in the descriptor
    pub fn set_entry_at_vaddr(&mut self, vaddr: VAddr, entry: L1Descriptor) {
        let idx = Self::index(vaddr);
        self.set_entry(idx, entry);
    }

    /// obtains a reference to the entry with the given index
    ///
    /// # Panics
    ///
    /// The function panics if the entry is out of bounds
    pub fn entry(&self, idx: usize) -> &L1Descriptor {
        if idx < L1_TABLE_ENTRIES {
            &self.0[idx]
        } else {
            panic!("table index {} out of supported range {}..{}", idx, 0, L1_TABLE_ENTRIES);
        }
    }

    /// obtains the entry based on the vaddr
    pub fn entry_at_vaddr(&self, vaddr: VAddr) -> &L1Descriptor {
        let idx = Self::index(vaddr);
        self.entry(idx)
    }

    /// obtains a mutable reference to the entry with the given idnex
    pub fn entry_mut(&mut self, idx: usize) -> &mut L1Descriptor {
        if idx < L1_TABLE_ENTRIES {
            &mut self.0[idx]
        } else {
            panic!("table index {} out of supported range {}..{}", idx, 0, L1_TABLE_ENTRIES);
        }
    }

    /// obtains a mutable reference to the entry based on the vaddr
    pub fn entry_at_vaddr_mut(&mut self, vaddr: VAddr) -> &mut L1Descriptor {
        let idx = Self::index(vaddr);
        self.entry_mut(idx)
    }

    /// calculates the index of the entry based on the vaddr
    pub fn index(va: VAddr) -> usize {
        va.as_u64().get_bits(30..39) as usize
    }
}

impl From<&L1Table> for VAddr {
    fn from(num: &L1Table) -> Self {
        VAddr(num as *const _ as u64)
    }
}

impl From<&L1Table> for PAddr {
    fn from(num: &L1Table) -> Self {
        PAddr::from(VAddr::from(num))
    }
}
