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
 * Generated on: 2022-08-22T15:51:28.506229
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
 * Register:    Activity Monitors Count Enable Set Register 0 (amcntenset0_el0)
 * Group:       Activity Monitors registers
 * Type:        64-bit Register
 * Description: Enable control bits for the architected activity monitors event counters,
 * File:        AArch64-amcntenset0_el0.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Activity Monitors Count Enable Set Register 0 value in memory
pub struct Amcntenset0El0(u64);

/// struct implementation for accessing the fields of register amcntenset0_el0
impl Amcntenset0El0 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> Amcntenset0El0 {
        Self::default()
    }

    /// collects the modifications of Amcntenset0El0 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> Amcntenset0El0 {
        Amcntenset0El0(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> Amcntenset0El0 {
        let curval = Self::reg_rawrd() & 0xffff;
        Amcntenset0El0(curval)
    }

    /// reading the Activity Monitors Count Enable Set Register 0 (amcntenset0_el0) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, AMCNTENSET0_EL0
            llvm_asm!("mrs $0, S3_3_C13_C2_5" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Activity Monitors Count Enable Set Register 0 (amcntenset0_el0) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR AMCNTENSET0_EL0, <Xt>
            llvm_asm!("msr S3_3_C13_C2_5, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xffff;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 65535;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: pn
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pn_extract(&self) -> u64 {
        // bits 0..15
        self.val.get_bits(0..=15)
    }

    /// reads the current register value and extract field `pn` from it
    pub fn pn_read(&mut self) -> u64 {
        Self::with_reg_val().pn_extract()
    }

    /// inserts the given value `val` into the field `pn`
    pub fn pn_insert(&mut self, val: u64) -> &mut self {
        // bits 0..15
        self.val.set_bits(0..=15, val);
        self
    }

    /// reads the register, updates the `pn` field, and writes the updated value
    pub fn pn_write(&mut self, val: u64) {
        Self::with_reg_val().pn_insert(val).write();
    }
}

impl Default for Amcntenset0El0 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> Amcntenset0El0 {
        Amcntenset0El0(0)
    }
}
