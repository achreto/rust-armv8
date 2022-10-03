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
use core::arch::asm;

/**************************************************************************************************
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 * Generated on: 2022-08-22T16:35:53.084231
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
 * Register:    SVE Control Register for EL2 (zcr_el2)
 * Group:       Other system control registers
 * Type:        64-bit Register
 * Description: The SVE Control Register for EL2 is used to control aspects of SVE visible at Exception levels EL2, EL1, and EL0, when EL2 is enabled in the current Security state.
 * File:        AArch64-zcr_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the SVE Control Register for EL2 value in memory
pub struct ZcrEl2(u64);

/// struct implementation for accessing the fields of register zcr_el2
impl ZcrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> ZcrEl2 {
        Self::default()
    }

    /// collects the modifications of ZcrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> ZcrEl2 {
        ZcrEl2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> ZcrEl2 {
        let curval = Self::reg_rawrd() & 0xf;
        ZcrEl2(curval)
    }

    /// reading the SVE Control Register for EL2 (zcr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ZCR_EL2
            asm!("mrs {}, S3_4_C1_C2_0", out(reg) regval);
        }
        return regval;
    }

    /// writing the SVE Control Register for EL2 (zcr_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR ZCR_EL2, <Xt>
            asm!("msr S3_4_C1_C2_0, {}", in(reg) val);
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xf;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 15;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: len
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn len_extract(&self) -> u64 {
        // bits 0..3
        self.0.get_bits(0..=3)
    }

    /// reads the current register value and extract field `len` from it
    pub fn len_read() -> u64 {
        Self::with_reg_val().len_extract()
    }

    /// inserts the given value `val` into the field `len`
    pub fn len_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..3
        self.0.set_bits(0..=3, val);
        self
    }

    /// reads the register, updates the `len` field, and writes the updated value
    pub fn len_write(val: u64) {
        Self::with_reg_val().len_insert(val).write();
    }
}

impl Default for ZcrEl2 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> ZcrEl2 {
        ZcrEl2(0)
    }
}
