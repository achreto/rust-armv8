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
 * Generated on: 2022-08-22T15:51:28.506381
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
 * Register:    Activity Monitors Control Register (amcr_el0)
 * Group:       Activity Monitors registers
 * Type:        64-bit Register
 * Description: Global control register for the activity monitors implementation. AMCR_EL0 is applicable to both the architected and the auxiliary counter groups.
 * File:        AArch64-amcr_el0.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Activity Monitors Control Register value in memory
pub struct AmcrEl0(u64);

/// struct implementation for accessing the fields of register amcr_el0
impl AmcrEl0 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> AmcrEl0 {
        Self::default()
    }

    /// collects the modifications of AmcrEl0 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> AmcrEl0 {
        AmcrEl0(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> AmcrEl0 {
        let curval = Self::reg_rawrd() & 0x20400;
        AmcrEl0(curval)
    }

    /// reading the Activity Monitors Control Register (amcr_el0) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, AMCR_EL0
            llvm_asm!("mrs $0, S3_3_C13_C2_0" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Activity Monitors Control Register (amcr_el0) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR AMCR_EL0, <Xt>
            llvm_asm!("msr S3_3_C13_C2_0, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0x20400;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 132096;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: cg1rz_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn cg1rz_1_extract(&self) -> u64 {
        // bits 17..17
        self.val.get_bits(17..=17)
    }

    /// reads the current register value and extract field `cg1rz_1` from it
    pub fn cg1rz_1_read(&mut self) -> u64 {
        Self::with_reg_val().cg1rz_1_extract()
    }

    /// inserts the given value `val` into the field `cg1rz_1`
    pub fn cg1rz_1_insert(&mut self, val: u64) -> &mut self {
        // bits 17..17
        self.val.set_bits(17..=17, val);
        self
    }

    /// reads the register, updates the `cg1rz_1` field, and writes the updated value
    pub fn cg1rz_1_write(&mut self, val: u64) {
        Self::with_reg_val().cg1rz_1_insert(val).write();
    }

    /*
     * Field: hdbg
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn hdbg_extract(&self) -> u64 {
        // bits 10..10
        self.val.get_bits(10..=10)
    }

    /// reads the current register value and extract field `hdbg` from it
    pub fn hdbg_read(&mut self) -> u64 {
        Self::with_reg_val().hdbg_extract()
    }

    /// inserts the given value `val` into the field `hdbg`
    pub fn hdbg_insert(&mut self, val: u64) -> &mut self {
        // bits 10..10
        self.val.set_bits(10..=10, val);
        self
    }

    /// reads the register, updates the `hdbg` field, and writes the updated value
    pub fn hdbg_write(&mut self, val: u64) {
        Self::with_reg_val().hdbg_insert(val).write();
    }
}

impl Default for AmcrEl0 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> AmcrEl0 {
        AmcrEl0(0)
    }
}
