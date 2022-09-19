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

mod consts;
mod ioaddr;
mod ipaddr;
mod paddr;
mod tables;
mod vaddr;

pub use super::descriptor_attributes::{
    BlockPageDescriptorLowerAttributes, BlockPageDescriptorUpperAttributes, MemoryAttributes,
    TableDescriptorNextLevelAttributes,
};
pub use consts::*;
pub use ioaddr::IOAddr;
pub use ipaddr::IPAddr;
pub use paddr::PAddr;
pub use tables::{L0Descriptor, L0Table};
pub use tables::{L1Descriptor, L1DescriptorBlock, L1DescriptorTable, L1Table};
pub use tables::{L2Descriptor, L2DescriptorBlock, L2DescriptorTable, L2Table};
pub use tables::{L3Descriptor, L3Table};
pub use vaddr::VAddr;

use crate::aarch64::vm::KERNEL_OFFSET;
use crate::aarch64::registers::TcrEl1;

/// A type wrapping a base page with a 4 KiB buffer.
pub struct Page([u8; BASE_PAGE_SIZE]);

/// A type wrapping a large page with a 2 MiB buffer.
pub struct LargePage([u8; LARGE_PAGE_SIZE]);

/// A type wrapping a huge page with a 1 GiB buffer.
pub struct HugePage([u8; HUGE_PAGE_SIZE]);

/// configures the virtual memory subsystem for el1
pub fn configure_el1() {
    // configure memory attributes
    MemoryAttributes::configure_el1();

    // Configure the EL1 translation regime.
    TcrEl1::new()
        .hpd1_1_insert(1) // Hierarchical permissions are disabled.
        .hpd0_1_insert(1) // Hierarchical permissions are disabled.
        .hd_1_insert(1) // Stage 1 hardware management of dirty state enabled, only if the HA bit is also set to 1.
        .ha_1_insert(1) // Stage 1 Access flag update enabled.
        .tbi0_insert(0) // Top Byte used in the address calculation. (TTBRR0_EL1)
        .tbi1_insert(0) // Top Byte used in the address calculation. (TTBRR1_EL1)
        .as_insert(0) // 8 bit - the upper 8 bits of TTBR0_EL1 and TTBR1_EL1 are ignored by hardware
        .ips_insert(0b101) // Intermediate Physical Address Size. 48 bits, 256TB.
        .tg1_insert(0b00) // Granule size for the TTBR0_EL1. 00 = 4 KiB
        .sh1_insert(0b11) // Inner Shareable
        .orgn1_insert(0b01) // Normal memory, Outer Write-Back Read-Allocate Write-Allocate Cacheable.
        .irgn1_insert(0b01) // Normal memory, Inner Write-Back Read-Allocate Write-Allocate Cacheable.
        .epd1_insert(0) // don't disable translation table walks for TTBR1_EL1
        .a1_insert(0) // TTBR0_EL1.ASID defines the ASID.
        .t1sz_insert(16) // The size offset of the memory region addressed by TTBR1_EL1 (48-bits)
        .tg0_insert(0b00) // Granule size for the TTBR0_EL1. 00 = 4 KiB
        .sh0_insert(0b11) // Inner Shareable
        .orgn0_insert(0b01) // Normal memory, Outer Write-Back Read-Allocate Write-Allocate Cacheable.
        .irgn0_insert(0b01) // Normal memory, Inner Write-Back Read-Allocate Write-Allocate Cacheable.
        .epd0_insert(0) // don't disable translation table walks for TTBR0_EL1
        .t0sz_insert(16) // 48-bit address space
        .write();

    unsafe {
        KERNEL_OFFSET = 0xffff_0000_0000_0000;
    }


    // /* Configure the kernel page tables for EL1. */
    // configure_ttbr1(core_data->page_table_root);

    // /* Enable EL0/1 translation. */
    // configure_sctlr();

    // /* configure spsr */
    // configure_spsr(el);

    // /* configure EL 1 traps*/
    // configure_el1_traps();
}
