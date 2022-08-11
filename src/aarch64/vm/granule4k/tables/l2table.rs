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
use crate::aarch64::vm::granule4k::L3Table;
use crate::aarch64::vm::granule4k::PAddr;

// VMSAv8-64 translation table level -1, level 0, level 1, and level 2 descriptor formats
// The difference in the level -1, level 0, level 1 and level 2 VMSAv8-64 Translation Table
// descriptor formats depends on the following:
//  * The translation granule size.
//  * Whether a Block descriptor is permitted.
//  * If a Block descriptor is permitted, the size of the memory region described by that entry.
//  * The maximum supported OA size.

/// A L2 Table Entry consists of an address and a flags.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct L2TableEntry(pub u64);

pub type L2Table = [L2TableEntry; L2_TABLE_ENTRIES];

impl L2TableEntry {
    pub fn new() -> Self {
        L2TableEntry(0)
    }

    /// the descriptor is valid
    pub fn valid(&self) -> bool {
        self.0.get_bit(0)
    }

    pub fn set_valid(&mut self) -> &mut Self {
        self.0.set_bit(0, true);
        self
    }

    /// the entry is a block descriptor (super page mapping)
    pub fn block(&self) -> bool {
        self.0.get_bit(0) && !self.0.get_bit(1)
    }

    /// the entry is a table descriptor (points to a page table)
    pub fn table(&self) -> bool {
        self.0.get_bit(0) && self.0.get_bit(1)
    }

    /// returns the physical address of the entry
    pub fn paddr(&self) -> Option<PAddr> {
        if self.block() {
            Some(PAddr::from(self.0.get_bits(21..48) << LARGE_PAGE_SHIFT))
        } else if self.table() {
            Some(PAddr::from(self.0.get_bits(12..48) << BASE_PAGE_SHIFT))
        } else {
            None
        }
    }

    pub fn set_block_addr(&mut self, paddr: PAddr) -> &mut Self {
        assert!(paddr % LARGE_PAGE_SIZE == 0);
        self.0.set_bits(21..48, paddr.as_u64() >> LARGE_PAGE_SHIFT);
        self.0.set_bit(1, false);
        self
    }

    pub fn set_table_addr(&mut self, table: &L3Table) -> &mut Self {
        let pt = PAddr::from(table);
        self.0.set_bits(12..48, pt.as_u64() >> BASE_PAGE_SHIFT);
        self.0.set_bit(1, true);
        self
    }

    pub fn get_address(&self) -> u64 {
        self.0 & 0xFFFF_FFFF_FFFF_F000
    }

    pub fn get_flags(&self) -> u64 {
        self.0 & 0xF
    }
}

pub enum L2 {
    Block(u64),
    Table(u64),
}
