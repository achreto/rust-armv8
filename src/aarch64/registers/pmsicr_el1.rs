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
 * Generated on: 2022-08-22T15:51:28.531771
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
 * Register:    Sampling Interval Counter Register (pmsicr_el1)
 * Group:       Statistical Profiling Extension registers
 * Type:        64-bit Register
 * Description: Software must write zero to PMSICR_EL1 before enabling sample profiling for a sampling session. Software must then treat PMSICR_EL1 as an opaque, 64-bit, read/write register used for context switches only.
 * File:        AArch64-pmsicr_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Sampling Interval Counter Register value in memory
pub struct PmsicrEl1(u64);

/// struct implementation for accessing the fields of register pmsicr_el1
impl PmsicrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> PmsicrEl1 {
        Self::default()
    }

    /// collects the modifications of PmsicrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> PmsicrEl1 {
        PmsicrEl1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> PmsicrEl1 {
        let curval = Self::reg_rawrd() & 0xff000000ffffffff;
        PmsicrEl1(curval)
    }

    /// reading the Sampling Interval Counter Register (pmsicr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, PMSICR_EL1
            llvm_asm!("mrs $0, S3_0_C9_C9_2" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Sampling Interval Counter Register (pmsicr_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR PMSICR_EL1, <Xt>
            llvm_asm!("msr S3_0_C9_C9_2, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xff000000ffffffff;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 18374686483966590975;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: ecount_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ecount_1_extract(&self) -> u64 {
        // bits 56..63
        self.val.get_bits(56..=63)
    }

    /// reads the current register value and extract field `ecount_1` from it
    pub fn ecount_1_read(&mut self) -> u64 {
        Self::with_reg_val().ecount_1_extract()
    }

    /// inserts the given value `val` into the field `ecount_1`
    pub fn ecount_1_insert(&mut self, val: u64) -> &mut self {
        // bits 56..63
        self.val.set_bits(56..=63, val);
        self
    }

    /// reads the register, updates the `ecount_1` field, and writes the updated value
    pub fn ecount_1_write(&mut self, val: u64) {
        Self::with_reg_val().ecount_1_insert(val).write();
    }

    /*
     * Field: count
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn count_extract(&self) -> u64 {
        // bits 0..31
        self.val.get_bits(0..=31)
    }

    /// reads the current register value and extract field `count` from it
    pub fn count_read(&mut self) -> u64 {
        Self::with_reg_val().count_extract()
    }

    /// inserts the given value `val` into the field `count`
    pub fn count_insert(&mut self, val: u64) -> &mut self {
        // bits 0..31
        self.val.set_bits(0..=31, val);
        self
    }

    /// reads the register, updates the `count` field, and writes the updated value
    pub fn count_write(&mut self, val: u64) {
        Self::with_reg_val().count_insert(val).write();
    }
}

impl Default for PmsicrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> PmsicrEl1 {
        PmsicrEl1(0)
    }
}
