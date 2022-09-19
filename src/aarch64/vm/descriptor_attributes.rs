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

// Data access permissions for stage 1 translations that applies to EL0 and a
// higher Exception level
//
// AP[2:1]  Access from higher Exception level  Access from EL0
// 00       Read/write                          None
// 01       Read/write                          Read/write
// 10       Read-only                           None
// 11       Read-only                           Read-only

pub const ACCESS_PERMISSIONS_NONE: u64 = 0b00;
pub const ACCESS_PERMISSIONS_RO: u64 = 0b11;
pub const ACCESS_PERMISSIONS_RW: u64 = 0b01;

// Stage 1 Shareability attribute, for Normal memory
// If the Effective value of TCR_ELx.DS is 0, when using the VMSAv8-64 translation table format,
// the SH[1:0] field in a block or page Translation Table descriptor specifies the Shareability
// attributes of the corresponding memory region. Table D5-36 on page D5-2777 shows the encoding
// of this field.
//
// SH[1:0]   Normal memory
//   00      Non-shareable
//   01      Reserved, CONSTRAINED UNPREDICTABLEa
//   10      Outer Shareable
//   11      Inner Shareable

pub const SHAREABILITY_NONE: u64 = 0b00;
pub const SHAREABILITY_OUTER: u64 = 0b10;
pub const SHAREABILITY_INNER: u64 = 0b11;

/// Lower Attribute fields in stage 1 VMSAv8-64 Block and Page descriptors
pub trait BlockPageDescriptorLowerAttributes {
    /// Obtains the stage 1 memory attributes index field, for the MAIR_ELx
    ///
    /// # Bits
    ///
    /// AttrIndx[2:0], bits[4:2]
    ///
    fn attr_index(&self) -> u64;

    /// Sets the stage 1 memory attributes index field, for the MAIR_ELx
    ///
    /// # Bits
    ///
    /// AttrIndx[2:0], bits[4:2]
    ///
    fn set_attr_index(&mut self, index: u64) -> &mut Self;

    /// Obtains the non-secure bit.
    ///
    /// For memory accesses from Secure state, specifies whether the output address is
    /// in the Secure or Non-secure address map
    ///
    /// # Bits
    ///
    /// NS, bit[5]
    ///
    fn is_non_secure(&self) -> bool;

    /// Sets the non-secure bit.
    ///
    /// Memory accesses from the Secure state will be using the non-secure address map.
    ///
    /// # Bits
    ///
    /// NS, bit[5]
    ///
    fn non_secure(&mut self) -> &mut Self;

    /// Clears the non-secure bit.
    ///
    /// Memory accesses from the Secure state will be using the secure address map.
    ///
    /// # Bits
    ///
    /// NS, bit[5]
    ///
    fn secure(&mut self) -> &mut Self;

    ///  Is accessible either read/write or read/only
    ///
    /// # Bits
    ///
    /// AP[2:1], bits[7:6]
    ///
    fn is_accessible(&self) -> bool;

    /// Sets data access to none
    ///
    /// # Bits
    ///
    /// AP[2:1], bits[7:6]
    ///
    fn no_access(&mut self) -> &mut Self;

    /// Checks whether data access is read-only
    ///
    /// # Bits
    ///
    /// AP[2:1], bits[7:6]
    ///
    fn is_read_only(&mut self) -> bool;

    /// Set data access to be read only
    ///
    /// # Bits
    ///
    /// AP[2:1], bits[7:6]
    ///
    fn read_only(&mut self) -> &mut Self;

    /// Checks whether data access is read-only
    ///
    /// # Bits
    ///
    /// AP[2:1], bits[7:6]
    ///
    fn is_read_write(&mut self) -> bool;

    /// Set data access to be read/write
    ///
    /// # Bits
    ///
    /// AP[2:1], bits[7:6]
    ///
    fn read_write(&mut self) -> &mut Self;

    /// Checks whether the memory is non-shareable
    ///
    /// # Bits
    ///
    /// SH, bits[9:8]
    ///
    fn is_non_shareable(&self) -> bool;

    /// Sets shareability to non-shareable
    ///
    /// # Bits
    ///
    /// SH, bits[9:8]
    ///
    fn non_shareable(&mut self) -> &mut Self;

    /// Checks whether the memory is outer shareable
    ///
    /// # Bits
    ///
    /// SH, bits[9:8]
    ///
    fn is_outer_shareable(&self) -> bool;

    /// Sets shareability to non-shareable
    ///
    /// # Bits
    ///
    /// SH, bits[9:8]
    ///
    fn outer_shareable(&mut self) -> &mut Self;

    /// Checks whether the memory is inner shareable
    ///
    /// # Bits
    ///
    /// SH, bits[9:8]
    ///
    fn is_inner_shareable(&self) -> bool;

    /// Sets shareability to non-shareable
    ///
    /// # Bits
    ///
    /// SH, bits[9:8]
    ///
    fn inner_shareable(&mut self) -> &mut Self;

    /// Obtains the access flag
    ///
    /// # Bits
    ///
    /// AF, bit[10]
    ///
    fn is_accessed(&self) -> bool;

    /// Sets the access flag
    ///
    /// # Bits
    ///
    /// AF, bit[10]
    ///
    fn accessed(&mut self) -> &mut Self;

    /// Clears the access flag
    ///
    /// # Bits
    ///
    /// AF, bit[10]
    ///
    fn not_accessed(&mut self) -> &mut Self;

    /// The not global bit.
    ///
    /// If a lookup using this descriptor is cached in a TLB, determines whether the TLB
    /// entry applies to all ASID values, or only to the current ASID value
    ///
    /// # Bits
    ///
    /// nG, bit[11]
    fn is_global(&self) -> bool;

    /// Makes the descriptor global by clearing the not-global bit
    ///
    /// # Bits
    ///
    /// nG, bit[11]
    fn global(&mut self) -> &mut Self;

    /// Makes the descriptor not global by setting the not-global bit
    ///
    /// # Bits
    ///
    /// nG, bit[11]
    fn not_global(&mut self) -> &mut Self;

    /// Block translation entry
    ///
    /// Generates a Translation fault when using a translation table entry that has the nT bit set.
    /// Such an entry is not permitted to be cached within the TLB.
    /// Guarantees that using a translation table entry that has the nT bit set does not break
    /// coherency, ordering guarantees or uniprocessor semantics, or fail to clear the Exclusives
    /// monitors when an entry that does not have the nT bit set is translating the same address
    /// ached within the TLB.
    ///
    /// # Bits
    ///
    /// nT, bit[16]
    ///
    fn is_block_translation(self) -> bool;

    /// clears the block translation bit
    fn block_translation(&mut self) -> &mut Self;

    /// clears the block translation bit
    fn unblock_translation(&mut self) -> &mut Self;
}

/// Upper Attribute fields in stage 1 VMSAv8-64 Block and Page descriptors
pub trait BlockPageDescriptorUpperAttributes {
    /// Is this a guarded page
    ///
    /// # Bits
    ///
    /// GP, bit[50]
    fn is_guard_page(&self) -> bool;

    /// Makes this a guarded page
    ///
    /// # Bits
    ///
    /// GP, bit[50]
    fn guard_page(&mut self) -> &mut Self;

    /// Makes this a unguarded page
    ///
    /// # Bits
    ///
    /// GP, bit[50]
    fn unguard_page(&mut self) -> &mut Self;

    /// Dirty Bit
    ///
    /// The dirty state indicates whether a page or section of memory is modified.
    ///
    /// # Bits
    ///
    /// DBM, bit[51]
    ///
    fn is_dirty(&self) -> bool;

    /// makes the entry dirty by setting the dirty bit in the page or block descriptor
    ///
    /// # Bits
    ///
    /// DBM, bit[51]
    ///
    fn dirty(&mut self) -> &mut Self;

    /// makes the entry dirty by clearing the dirty bit in the page or block descriptor
    ///
    /// # Bits
    ///
    /// DBM, bit[51]
    ///
    fn not_dirty(&mut self) -> &mut Self;

    /// Contiguous
    ///
    /// A hint bit indicating that the translation table entry is one of a contiguous set
    /// of entries, that might be cached in a single TLB entry,
    ///
    /// # Bits
    ///
    /// Contiguous, bit[52]
    ///
    fn is_contiguous(&self) -> bool;

    /// marks the page or block descriptor as contiguous
    ///
    /// # Bits
    ///
    /// Contiguous, bit[52]
    ///
    fn contiguous(&mut self) -> &mut Self;

    /// marks the page or block descriptor as not contiguous
    ///
    /// # Bits
    ///
    /// Contiguous, bit[52]
    ///
    fn not_contiguous(&mut self) -> &mut Self;

    /// Privileged execute-never field
    ///
    /// # Bits
    ///
    /// PXN, bit[53]
    ///
    fn is_priv_exec(&self) -> bool;

    /// allows execution of the code in the page or block descriptor (privileged mode)
    ///
    /// # Bits
    ///
    /// PXN, bit[53]
    ///
    fn priv_exec(&mut self) -> &mut Self;

    /// prohibits execution of the code in the page or block descriptor (privileged mode)
    ///
    /// # Bits
    ///
    /// PXN, bit[53]
    ///
    fn priv_exec_never(&mut self) -> &mut Self;

    /// Unprivileged execute-never field
    ///
    /// # Bits
    ///
    /// XN or UXN, bit[54]
    ///
    fn is_user_exec(&self) -> bool;

    /// allows execution of the code in the page or block descriptor (user mode)
    ///
    /// # Bits
    ///
    /// XN or UXN, bit[54]
    ///
    fn user_exec(&mut self) -> &mut Self;

    /// prohibits execution of the code in the page or block descriptor (user mode)
    ///
    /// # Bits
    ///
    /// XN or UXN, bit[54]
    ///
    fn user_exec_never(&mut self) -> &mut Self;

    /// Page-based Hardware Attributes bits.
    ///
    /// # Bits
    ///
    /// PBHA, bits[62:59]
    ///
    fn page_based_hardware_attributes(&self) -> u64;
}

/// Next-level attributes in stage 1 VMSAv8-64 Table descriptors
pub trait TableDescriptorNextLevelAttributes {
    /// PXN limit for subsequent levels of lookup
    ///
    /// # Bits
    ///
    /// PXNTable, bit[59]
    fn is_priv_exec_table(&self) -> bool;

    /// allows execution of the code for subsequent levels of lookup (privileged mode)
    ///
    /// # Bits
    ///
    /// PXNTable, bit[59]
    ///
    fn priv_exec_table(&mut self) -> &mut Self;

    /// prohibits execution of the code for subsequent levels of lookup (privileged mode)
    ///
    /// # Bits
    ///
    /// PXNTable, bit[59]
    ///
    fn priv_exec_never_table(&mut self) -> &mut Self;

    /// UXN limit for subsequent levels of lookup,
    ///
    /// # Bits
    ///
    /// UXNTable or XNTable, bit[60]
    ///
    fn is_user_exec_table(&self) -> bool;

    /// allows execution of the code for subsequent levels of lookup (user mode)
    ///
    /// # Bits
    ///
    /// PXNTable, bit[59]
    ///
    fn user_exec_table(&mut self) -> &mut Self;

    /// prohibits execution of the code for subsequent levels of lookup (user mode)
    ///
    /// # Bits
    ///
    /// PXNTable, bit[59]
    ///
    fn user_exec_never_table(&mut self) -> &mut Self;

    /// Is accessible either read/write or read/only
    ///
    /// # Bits
    ///
    /// APTable, bits[62:61]
    ///
    fn is_accessible_table(&self) -> bool;

    /// Sets data access to none
    ///
    /// # Bits
    ///
    /// APTable, bits[62:61]
    ///
    fn no_access_table(&mut self) -> &mut Self;

    /// Checks whether data access is read-only
    ///
    /// # Bits
    ///
    /// APTable, bits[62:61]
    ///
    fn is_read_only_table(&mut self) -> bool;

    /// Set data access to be read only
    ///
    /// # Bits
    ///
    /// APTable, bits[62:61]
    ///
    fn read_only_table(&mut self) -> &mut Self;

    /// Checks whether data access is read-only
    ///
    /// # Bits
    ///
    /// APTable, bits[62:61]
    ///
    fn is_read_write_table(&mut self) -> bool;

    /// Set data access to be read/write
    ///
    /// # Bits
    ///
    /// APTable, bits[62:61]
    ///
    fn read_write_table(&mut self) -> &mut Self;

    /// Security state  for subsequent levels of lookup
    ///
    /// For memory accesses from Secure state, specifies the Security state for subsequent levels of lookup
    ///
    /// # Bits
    ///
    /// NSTable, bit[63]
    ///
    fn is_non_secure_table(&self) -> bool;

    /// Non-secure accesses for subsequent levels of lookup
    ///
    /// # Bits
    ///
    /// NSTable, bit[63]
    ///
    fn non_secure_table(&mut self) -> &mut Self;

    /// Secure accesses for subsequent levels of lookup
    ///
    /// # Bits
    ///
    /// NSTable, bit[63]
    ///
    fn secure_table(&mut self) -> &mut Self;
}

#[macro_export]
macro_rules! page_block_lower_attributes_impl {
    ($t:ty) => {
        impl BlockPageDescriptorLowerAttributes for $t {
            #[inline]
            fn attr_index(&self) -> u64 {
                self.0.get_bits(2..=4)
            }

            #[inline]
            fn set_attr_index(&mut self, index: u64) -> &mut Self {
                self.0.set_bits(2..=4, index);
                self
            }

            #[inline]
            fn is_non_secure(&self) -> bool {
                self.0.get_bit(5)
            }

            #[inline]
            fn non_secure(&mut self) -> &mut Self {
                self.0.set_bit(5, true);
                self
            }

            #[inline]
            fn secure(&mut self) -> &mut Self {
                self.0.set_bit(5, false);
                self
            }

            #[inline]
            fn is_accessible(&self) -> bool {
                // matches!(self.0.get_bits(6..7), 0b00, 0b10)
                self.0.get_bit(6)
            }

            #[inline]
            fn no_access(&mut self) -> &mut Self {
                self.0.set_bits(6..=7, ACCESS_PERMISSIONS_NONE);
                self
            }

            #[inline]
            fn is_read_only(&mut self) -> bool {
                self.0.get_bits(6..=7) == ACCESS_PERMISSIONS_RO
            }

            #[inline]
            fn read_only(&mut self) -> &mut Self {
                self.0.set_bits(6..=7, ACCESS_PERMISSIONS_RO);
                self
            }

            #[inline]
            fn is_read_write(&mut self) -> bool {
                self.0.get_bits(6..=7) == ACCESS_PERMISSIONS_RW
            }

            #[inline]
            fn read_write(&mut self) -> &mut Self {
                self.0.set_bits(6..=7, ACCESS_PERMISSIONS_RW);
                self
            }

            #[inline]
            fn is_non_shareable(&self) -> bool {
                self.0.get_bits(8..=9) == SHAREABILITY_NONE
            }

            #[inline]
            fn non_shareable(&mut self) -> &mut Self {
                self.0.set_bits(8..=9, SHAREABILITY_NONE);
                self
            }

            #[inline]
            fn is_outer_shareable(&self) -> bool {
                self.0.get_bits(8..=9) == SHAREABILITY_OUTER
            }

            #[inline]
            fn outer_shareable(&mut self) -> &mut Self {
                self.0.set_bits(8..=9, SHAREABILITY_OUTER);
                self
            }

            #[inline]
            fn is_inner_shareable(&self) -> bool {
                self.0.get_bits(8..=9) == SHAREABILITY_INNER
            }

            #[inline]
            fn inner_shareable(&mut self) -> &mut Self {
                self.0.set_bits(8..=9, SHAREABILITY_INNER);
                self
            }

            #[inline]
            fn is_accessed(&self) -> bool {
                self.0.get_bit(10)
            }

            #[inline]
            fn accessed(&mut self) -> &mut Self {
                self.0.set_bit(10, true);
                self
            }

            #[inline]
            fn not_accessed(&mut self) -> &mut Self {
                self.0.set_bit(10, false);
                self
            }

            #[inline]
            fn is_global(&self) -> bool {
                !self.0.get_bit(11)
            }

            #[inline]
            fn global(&mut self) -> &mut Self {
                self.0.set_bit(11, false);
                self
            }

            #[inline]
            fn not_global(&mut self) -> &mut Self {
                self.0.set_bit(11, true);
                self
            }

            #[inline]
            fn is_block_translation(self) -> bool {
                self.0.get_bit(16)
            }

            #[inline]
            fn block_translation(&mut self) -> &mut Self {
                self.0.set_bit(16, true);
                self
            }

            #[inline]
            fn unblock_translation(&mut self) -> &mut Self {
                self.0.set_bit(16, false);
                self
            }
        }
    };
}

#[macro_export]
macro_rules! page_block_upper_attributes_impl {
    ($t:ty) => {
        impl BlockPageDescriptorUpperAttributes for $t {
            #[inline]
            fn is_guard_page(&self) -> bool {
                self.0.get_bit(50)
            }

            #[inline]
            fn guard_page(&mut self) -> &mut Self {
                self.0.set_bit(50, true);
                self
            }

            #[inline]
            fn unguard_page(&mut self) -> &mut Self {
                self.0.set_bit(50, false);
                self
            }

            #[inline]
            fn is_dirty(&self) -> bool {
                self.0.get_bit(51)
            }

            #[inline]
            fn dirty(&mut self) -> &mut Self {
                self.0.set_bit(51, true);
                self
            }

            #[inline]
            fn not_dirty(&mut self) -> &mut Self {
                self.0.set_bit(51, false);
                self
            }

            #[inline]
            fn is_contiguous(&self) -> bool {
                self.0.get_bit(52)
            }

            #[inline]
            fn contiguous(&mut self) -> &mut Self {
                self.0.set_bit(52, true);
                self
            }

            #[inline]
            fn not_contiguous(&mut self) -> &mut Self {
                self.0.set_bit(52, false);
                self
            }

            #[inline]
            fn is_priv_exec(&self) -> bool {
                !self.0.get_bit(53)
            }

            #[inline]
            fn priv_exec(&mut self) -> &mut Self {
                self.0.set_bit(53, false);
                self
            }

            #[inline]
            fn priv_exec_never(&mut self) -> &mut Self {
                self.0.set_bit(53, true);
                self
            }

            #[inline]
            fn is_user_exec(&self) -> bool {
                !self.0.get_bit(54)
            }

            #[inline]
            fn user_exec(&mut self) -> &mut Self {
                self.0.set_bit(54, false);
                self
            }

            #[inline]
            fn user_exec_never(&mut self) -> &mut Self {
                self.0.set_bit(54, true);
                self
            }

            #[inline]
            fn page_based_hardware_attributes(&self) -> u64 {
                self.0.get_bits(59..=62)
            }
        }
    };
}

#[macro_export]
macro_rules! table_next_level_attributes_impl {
    ($t:ty) => {
        impl TableDescriptorNextLevelAttributes for $t {
            #[inline]
            fn is_priv_exec_table(&self) -> bool {
                !self.0.get_bit(59)
            }

            #[inline]
            fn priv_exec_table(&mut self) -> &mut Self {
                self.0.set_bit(59, false);
                self
            }

            #[inline]
            fn priv_exec_never_table(&mut self) -> &mut Self {
                self.0.set_bit(59, true);
                self
            }

            #[inline]
            fn is_user_exec_table(&self) -> bool {
                !self.0.get_bit(60)
            }

            #[inline]
            fn user_exec_table(&mut self) -> &mut Self {
                self.0.set_bit(60, false);
                self
            }

            #[inline]
            fn user_exec_never_table(&mut self) -> &mut Self {
                self.0.set_bit(60, true);
                self
            }

            #[inline]
            fn is_accessible_table(&self) -> bool {
                !self.0.get_bit(61)
            }

            #[inline]
            fn no_access_table(&mut self) -> &mut Self {
                self.0.set_bits(61..=62, ACCESS_PERMISSIONS_NONE);
                self
            }

            #[inline]
            fn is_read_only_table(&mut self) -> bool {
                self.0.get_bits(61..=62) == ACCESS_PERMISSIONS_RO
            }

            #[inline]
            fn read_only_table(&mut self) -> &mut Self {
                self.0.set_bits(61..=62, ACCESS_PERMISSIONS_RO);
                self
            }

            #[inline]
            fn is_read_write_table(&mut self) -> bool {
                self.0.get_bits(61..=62) == ACCESS_PERMISSIONS_RW
            }

            #[inline]
            fn read_write_table(&mut self) -> &mut Self {
                self.0.set_bits(61..=62, ACCESS_PERMISSIONS_RW);
                self
            }

            #[inline]
            fn is_non_secure_table(&self) -> bool {
                self.0.get_bit(63)
            }

            #[inline]
            fn non_secure_table(&mut self) -> &mut Self {
                self.0.set_bit(63, true);
                self
            }

            #[inline]
            fn secure_table(&mut self) -> &mut Self {
                self.0.set_bit(63, false);
                self
            }
        }
    };
}
