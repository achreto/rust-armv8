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
 * Generated on: 2022-08-22T15:51:28.508540
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
 * Register:    Counter-timer Secure Virtual Timer Control register (EL2) (cnthvs_ctl_el2)
 * Group:       Generic Timer registers
 * Type:        64-bit Register
 * Description: Control register for the Secure EL2 virtual timer.
 * File:        AArch64-cnthvs_ctl_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Counter-timer Secure Virtual Timer Control register (EL2) value in memory
pub struct CnthvsCtlEl2(u64);

/// struct implementation for accessing the fields of register cnthvs_ctl_el2
impl CnthvsCtlEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> CnthvsCtlEl2 {
        Self::default()
    }

    /// collects the modifications of CnthvsCtlEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> CnthvsCtlEl2 {
        CnthvsCtlEl2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> CnthvsCtlEl2 {
        let curval = Self::reg_rawrd() & 0x7;
        CnthvsCtlEl2(curval)
    }

    /// reading the Counter-timer Secure Virtual Timer Control register (EL2) (cnthvs_ctl_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, CNTHVS_CTL_EL2
            llvm_asm!("mrs $0, S3_4_C14_C4_1" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Counter-timer Secure Virtual Timer Control register (EL2) (cnthvs_ctl_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR CNTHVS_CTL_EL2, <Xt>
            llvm_asm!("msr S3_4_C14_C4_1, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0x7;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 7;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: istatus
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn istatus_extract(&self) -> u64 {
        // bits 2..2
        self.val.get_bits(2..=2)
    }

    /// reads the current register value and extract field `istatus` from it
    pub fn istatus_read(&mut self) -> u64 {
        Self::with_reg_val().istatus_extract()
    }

    /// inserts the given value `val` into the field `istatus`
    pub fn istatus_insert(&mut self, val: u64) -> &mut self {
        // bits 2..2
        self.val.set_bits(2..=2, val);
        self
    }

    /// reads the register, updates the `istatus` field, and writes the updated value
    pub fn istatus_write(&mut self, val: u64) {
        Self::with_reg_val().istatus_insert(val).write();
    }

    /*
     * Field: imask
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn imask_extract(&self) -> u64 {
        // bits 1..1
        self.val.get_bits(1..=1)
    }

    /// reads the current register value and extract field `imask` from it
    pub fn imask_read(&mut self) -> u64 {
        Self::with_reg_val().imask_extract()
    }

    /// inserts the given value `val` into the field `imask`
    pub fn imask_insert(&mut self, val: u64) -> &mut self {
        // bits 1..1
        self.val.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `imask` field, and writes the updated value
    pub fn imask_write(&mut self, val: u64) {
        Self::with_reg_val().imask_insert(val).write();
    }

    /*
     * Field: enable
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn enable_extract(&self) -> u64 {
        // bits 0..0
        self.val.get_bits(0..=0)
    }

    /// reads the current register value and extract field `enable` from it
    pub fn enable_read(&mut self) -> u64 {
        Self::with_reg_val().enable_extract()
    }

    /// inserts the given value `val` into the field `enable`
    pub fn enable_insert(&mut self, val: u64) -> &mut self {
        // bits 0..0
        self.val.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `enable` field, and writes the updated value
    pub fn enable_write(&mut self, val: u64) {
        Self::with_reg_val().enable_insert(val).write();
    }
}

impl Default for CnthvsCtlEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> CnthvsCtlEl2 {
        CnthvsCtlEl2(0)
    }
}
