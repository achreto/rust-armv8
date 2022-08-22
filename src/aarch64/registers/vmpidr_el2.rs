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
 * Generated on: 2022-08-22T15:51:28.540201
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
 * Register:    Virtualization Multiprocessor ID Register (vmpidr_el2)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Holds the value of the Virtualization Multiprocessor ID. This is the value returned by EL1 reads of
 * File:        AArch64-vmpidr_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Virtualization Multiprocessor ID Register value in memory
pub struct VmpidrEl2(u64);

/// struct implementation for accessing the fields of register vmpidr_el2
impl VmpidrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> VmpidrEl2 {
        Self::default()
    }

    /// collects the modifications of VmpidrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> VmpidrEl2 {
        VmpidrEl2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> VmpidrEl2 {
        let curval = Self::reg_rawrd() & 0xff41ffffff;
        VmpidrEl2(curval)
    }

    /// reading the Virtualization Multiprocessor ID Register (vmpidr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, VMPIDR_EL2
            llvm_asm!("mrs $0, vmpidr_el2" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Virtualization Multiprocessor ID Register (vmpidr_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR VMPIDR_EL2, <Xt>
            llvm_asm!("msr vmpidr_el2, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xff41ffffff;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 1096323956735;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: aff3
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn aff3_extract(&self) -> u64 {
        // bits 32..39
        self.val.get_bits(32..=39)
    }

    /// reads the current register value and extract field `aff3` from it
    pub fn aff3_read(&mut self) -> u64 {
        Self::with_reg_val().aff3_extract()
    }

    /// inserts the given value `val` into the field `aff3`
    pub fn aff3_insert(&mut self, val: u64) -> &mut self {
        // bits 32..39
        self.val.set_bits(32..=39, val);
        self
    }

    /// reads the register, updates the `aff3` field, and writes the updated value
    pub fn aff3_write(&mut self, val: u64) {
        Self::with_reg_val().aff3_insert(val).write();
    }

    /*
     * Field: u
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn u_extract(&self) -> u64 {
        // bits 30..30
        self.val.get_bits(30..=30)
    }

    /// reads the current register value and extract field `u` from it
    pub fn u_read(&mut self) -> u64 {
        Self::with_reg_val().u_extract()
    }

    /// inserts the given value `val` into the field `u`
    pub fn u_insert(&mut self, val: u64) -> &mut self {
        // bits 30..30
        self.val.set_bits(30..=30, val);
        self
    }

    /// reads the register, updates the `u` field, and writes the updated value
    pub fn u_write(&mut self, val: u64) {
        Self::with_reg_val().u_insert(val).write();
    }

    /*
     * Field: mt
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn mt_extract(&self) -> u64 {
        // bits 24..24
        self.val.get_bits(24..=24)
    }

    /// reads the current register value and extract field `mt` from it
    pub fn mt_read(&mut self) -> u64 {
        Self::with_reg_val().mt_extract()
    }

    /// inserts the given value `val` into the field `mt`
    pub fn mt_insert(&mut self, val: u64) -> &mut self {
        // bits 24..24
        self.val.set_bits(24..=24, val);
        self
    }

    /// reads the register, updates the `mt` field, and writes the updated value
    pub fn mt_write(&mut self, val: u64) {
        Self::with_reg_val().mt_insert(val).write();
    }

    /*
     * Field: aff2
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn aff2_extract(&self) -> u64 {
        // bits 16..23
        self.val.get_bits(16..=23)
    }

    /// reads the current register value and extract field `aff2` from it
    pub fn aff2_read(&mut self) -> u64 {
        Self::with_reg_val().aff2_extract()
    }

    /// inserts the given value `val` into the field `aff2`
    pub fn aff2_insert(&mut self, val: u64) -> &mut self {
        // bits 16..23
        self.val.set_bits(16..=23, val);
        self
    }

    /// reads the register, updates the `aff2` field, and writes the updated value
    pub fn aff2_write(&mut self, val: u64) {
        Self::with_reg_val().aff2_insert(val).write();
    }

    /*
     * Field: aff1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn aff1_extract(&self) -> u64 {
        // bits 8..15
        self.val.get_bits(8..=15)
    }

    /// reads the current register value and extract field `aff1` from it
    pub fn aff1_read(&mut self) -> u64 {
        Self::with_reg_val().aff1_extract()
    }

    /// inserts the given value `val` into the field `aff1`
    pub fn aff1_insert(&mut self, val: u64) -> &mut self {
        // bits 8..15
        self.val.set_bits(8..=15, val);
        self
    }

    /// reads the register, updates the `aff1` field, and writes the updated value
    pub fn aff1_write(&mut self, val: u64) {
        Self::with_reg_val().aff1_insert(val).write();
    }

    /*
     * Field: aff0
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn aff0_extract(&self) -> u64 {
        // bits 0..7
        self.val.get_bits(0..=7)
    }

    /// reads the current register value and extract field `aff0` from it
    pub fn aff0_read(&mut self) -> u64 {
        Self::with_reg_val().aff0_extract()
    }

    /// inserts the given value `val` into the field `aff0`
    pub fn aff0_insert(&mut self, val: u64) -> &mut self {
        // bits 0..7
        self.val.set_bits(0..=7, val);
        self
    }

    /// reads the register, updates the `aff0` field, and writes the updated value
    pub fn aff0_write(&mut self, val: u64) {
        Self::with_reg_val().aff0_insert(val).write();
    }
}

impl Default for VmpidrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> VmpidrEl2 {
        VmpidrEl2(0)
    }
}
