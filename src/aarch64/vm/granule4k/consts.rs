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

/// Log2 of base page size (12 bits)
pub const BASE_PAGE_SHIFT: usize = 12;

/// size of the base page size in bits (12 bits)
pub const BASE_PAGE_BITS: usize = BASE_PAGE_SHIFT;

/// size of the base page size in bytes (4 KiB)
pub const BASE_PAGE_SIZE: usize = 1 << BASE_PAGE_BITS;

/// Log2 of large page size (21 bits)
pub const LARGE_PAGE_SHIFT: usize = 21;

/// size of the large page size in bits (21 bits)
pub const LARGE_PAGE_BITS: usize = LARGE_PAGE_SHIFT;

/// size of the large page size in bytes (2 MiB)
pub const LARGE_PAGE_SIZE: usize = 1 << LARGE_PAGE_BITS;

/// Log2 of huge page size (30 bits)
pub const HUGE_PAGE_SHIFT: usize = 30;

/// size of the huge page size in bits (30 bits)
pub const HUGE_PAGE_BITS: usize = HUGE_PAGE_SHIFT;

/// size of the huge page size in bytes (1 GiB)
pub const HUGE_PAGE_SIZE: usize = 1 << HUGE_PAGE_BITS;

/// Number of entries in the L0 translation table
pub const L0_TABLE_ENTRIES: usize = 512;

/// Size of an entry in the L0 translation table
pub const L0_TABLE_ENTRY_SIZE: usize = 8;

/// Size of the L0 translation table in bytes
pub const L0_TABLE_SIZE: usize = L0_TABLE_ENTRIES * L0_TABLE_ENTRY_SIZE;

/// Number of entries in the L1 translation table
pub const L1_TABLE_ENTRIES: usize = 512;

/// Size of an entry in the L1 translation table
pub const L1_TABLE_ENTRY_SIZE: usize = 8;

/// Size of the L1 translation table in bytes
pub const L1_TABLE_SIZE: usize = L1_TABLE_ENTRIES * L1_TABLE_ENTRY_SIZE;

/// Number of entries in the L2 translation table
pub const L2_TABLE_ENTRIES: usize = 512;

/// Size of an entry in the L2 translation table
pub const L2_TABLE_ENTRY_SIZE: usize = 8;

/// Size of the L2 translation table in bytes
pub const L2_TABLE_SIZE: usize = L2_TABLE_ENTRIES * L2_TABLE_ENTRY_SIZE;

/// Number of entries in the L3 translation table
pub const L3_TABLE_ENTRIES: usize = 512;

/// Size of an entry in the L3 translation table
pub const L3_TABLE_ENTRY_SIZE: usize = 8;

/// Size of the L3 translation table in bytes
pub const L3_TABLE_SIZE: usize = L3_TABLE_ENTRIES * L3_TABLE_ENTRY_SIZE;
