/*
 * MIT License
 *
 * Copyright (c) 2022 Reto Achermann
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * SPDX-License-Identifier: MIT
 */

use bit_field::BitField;

/**************************************************************************************************
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 * Generated on: 2022-08-22T15:51:28.540402
 * Version: Armv8.7-A-2020-09
 * Source: https://developer.arm.com/-/media/developer/products/architecture/armv8-a-architecture/2020-09/SysReg_xml_v87A-2020-09.tar.gz
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 *************************************************************************************************/

/*
 * ================================================================================================
 * Register Information
 * ================================================================================================
 *
 * Register:    Virtualization Processor ID Register (vpidr_el2)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Holds the value of the Virtualization Processor ID. This is the value returned by EL1 reads of
 * File:        AArch64-vpidr_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Virtualization Processor ID Register value in memory
pub struct VpidrEl2(u64);

/// struct implementation for accessing the fields of register vpidr_el2
impl VpidrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> VpidrEl2 {
        Self::default()
    }

    /// collects the modifications of VpidrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> VpidrEl2 {
        VpidrEl2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> VpidrEl2 {
        let curval = Self::reg_rawrd() & 0xffffffff;
        VpidrEl2(curval)
    }

    /// reading the Virtualization Processor ID Register (vpidr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, VPIDR_EL2
            llvm_asm!("mrs $0, vpidr_el2" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Virtualization Processor ID Register (vpidr_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR VPIDR_EL2, <Xt>
            llvm_asm!("msr vpidr_el2, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xffffffff;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 4294967295;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: implementer
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn implementer_extract(&self) -> u64 {
        // bits 24..31
        self.val.get_bits(24..=31)
    }

    /// reads the current register value and extract field `implementer` from it
    pub fn implementer_read(&mut self) -> u64 {
        Self::with_reg_val().implementer_extract()
    }

    /// inserts the given value `val` into the field `implementer`
    pub fn implementer_insert(&mut self, val: u64) -> &mut self {
        // bits 24..31
        self.val.set_bits(24..=31, val);
        self
    }

    /// reads the register, updates the `implementer` field, and writes the updated value
    pub fn implementer_write(&mut self, val: u64) {
        Self::with_reg_val().implementer_insert(val).write();
    }

    /*
     * Field: variant
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn variant_extract(&self) -> u64 {
        // bits 20..23
        self.val.get_bits(20..=23)
    }

    /// reads the current register value and extract field `variant` from it
    pub fn variant_read(&mut self) -> u64 {
        Self::with_reg_val().variant_extract()
    }

    /// inserts the given value `val` into the field `variant`
    pub fn variant_insert(&mut self, val: u64) -> &mut self {
        // bits 20..23
        self.val.set_bits(20..=23, val);
        self
    }

    /// reads the register, updates the `variant` field, and writes the updated value
    pub fn variant_write(&mut self, val: u64) {
        Self::with_reg_val().variant_insert(val).write();
    }

    /*
     * Field: architecture
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn architecture_extract(&self) -> u64 {
        // bits 16..19
        self.val.get_bits(16..=19)
    }

    /// reads the current register value and extract field `architecture` from it
    pub fn architecture_read(&mut self) -> u64 {
        Self::with_reg_val().architecture_extract()
    }

    /// inserts the given value `val` into the field `architecture`
    pub fn architecture_insert(&mut self, val: u64) -> &mut self {
        // bits 16..19
        self.val.set_bits(16..=19, val);
        self
    }

    /// reads the register, updates the `architecture` field, and writes the updated value
    pub fn architecture_write(&mut self, val: u64) {
        Self::with_reg_val().architecture_insert(val).write();
    }

    /*
     * Field: partnum
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn partnum_extract(&self) -> u64 {
        // bits 4..15
        self.val.get_bits(4..=15)
    }

    /// reads the current register value and extract field `partnum` from it
    pub fn partnum_read(&mut self) -> u64 {
        Self::with_reg_val().partnum_extract()
    }

    /// inserts the given value `val` into the field `partnum`
    pub fn partnum_insert(&mut self, val: u64) -> &mut self {
        // bits 4..15
        self.val.set_bits(4..=15, val);
        self
    }

    /// reads the register, updates the `partnum` field, and writes the updated value
    pub fn partnum_write(&mut self, val: u64) {
        Self::with_reg_val().partnum_insert(val).write();
    }

    /*
     * Field: revision
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn revision_extract(&self) -> u64 {
        // bits 0..3
        self.val.get_bits(0..=3)
    }

    /// reads the current register value and extract field `revision` from it
    pub fn revision_read(&mut self) -> u64 {
        Self::with_reg_val().revision_extract()
    }

    /// inserts the given value `val` into the field `revision`
    pub fn revision_insert(&mut self, val: u64) -> &mut self {
        // bits 0..3
        self.val.set_bits(0..=3, val);
        self
    }

    /// reads the register, updates the `revision` field, and writes the updated value
    pub fn revision_write(&mut self, val: u64) {
        Self::with_reg_val().revision_insert(val).write();
    }
}

impl Default for VpidrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> VpidrEl2 {
        VpidrEl2(0)
    }
}
