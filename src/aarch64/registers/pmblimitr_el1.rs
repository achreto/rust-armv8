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

use core::arch::asm;
use bit_field::BitField;


/**************************************************************************************************
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 * Generated on: 2022-08-22T16:25:59.091476
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
 * Register:    Profiling Buffer Limit Address Register (pmblimitr_el1)
 * Group:       Statistical Profiling Extension registers
 * Type:        64-bit Register
 * Description: Defines the upper limit for the profiling buffer, and enables the profiling buffer
 * File:        AArch64-pmblimitr_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Profiling Buffer Limit Address Register value in memory
pub struct PmblimitrEl1(u64);

/// struct implementation for accessing the fields of register pmblimitr_el1
impl PmblimitrEl1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> PmblimitrEl1 {
        Self::default()
    }

    /// collects the modifications of PmblimitrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> PmblimitrEl1 {
        PmblimitrEl1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  PmblimitrEl1 {
        let curval = Self::reg_rawrd() & 0xfffffffffffff027;
        PmblimitrEl1(curval)
    }


    
    /// reading the Profiling Buffer Limit Address Register (pmblimitr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, PMBLIMITR_EL1
            asm!("mrs {}, S3_0_C9_C10_0", out(reg) regval);
        }
        return regval;
    }


    /// writing the Profiling Buffer Limit Address Register (pmblimitr_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR PMBLIMITR_EL1, <Xt>
            asm!("msr S3_0_C9_C10_0, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xfffffffffffff027;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 18446744073709547559;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: limit
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn limit_extract(&self) -> u64 {
        // bits 12..63
        self.0.get_bits(12..=63)
    }

    /// reads the current register value and extract field `limit` from it
    pub fn limit_read(&mut self) -> u64 {
        Self::with_reg_val().limit_extract()
    }

    /// inserts the given value `val` into the field `limit`
    pub fn limit_insert(&mut self, val: u64) -> &mut Self {
        // bits 12..63
        self.0.set_bits(12..=63, val);
        self
    }

    /// reads the register, updates the `limit` field, and writes the updated value
    pub fn limit_write(&mut self, val: u64) {
        Self::with_reg_val().limit_insert(val).write();
    }

    /*
     * Field: pmfz_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn pmfz_1_extract(&self) -> u64 {
        // bits 5..5
        self.0.get_bits(5..=5)
    }

    /// reads the current register value and extract field `pmfz_1` from it
    pub fn pmfz_1_read(&mut self) -> u64 {
        Self::with_reg_val().pmfz_1_extract()
    }

    /// inserts the given value `val` into the field `pmfz_1`
    pub fn pmfz_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 5..5
        self.0.set_bits(5..=5, val);
        self
    }

    /// reads the register, updates the `pmfz_1` field, and writes the updated value
    pub fn pmfz_1_write(&mut self, val: u64) {
        Self::with_reg_val().pmfz_1_insert(val).write();
    }

    /*
     * Field: fm
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn fm_extract(&self) -> u64 {
        // bits 1..2
        self.0.get_bits(1..=2)
    }

    /// reads the current register value and extract field `fm` from it
    pub fn fm_read(&mut self) -> u64 {
        Self::with_reg_val().fm_extract()
    }

    /// inserts the given value `val` into the field `fm`
    pub fn fm_insert(&mut self, val: u64) -> &mut Self {
        // bits 1..2
        self.0.set_bits(1..=2, val);
        self
    }

    /// reads the register, updates the `fm` field, and writes the updated value
    pub fn fm_write(&mut self, val: u64) {
        Self::with_reg_val().fm_insert(val).write();
    }

    /*
     * Field: e
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e_extract(&self) -> u64 {
        // bits 0..0
        self.0.get_bits(0..=0)
    }

    /// reads the current register value and extract field `e` from it
    pub fn e_read(&mut self) -> u64 {
        Self::with_reg_val().e_extract()
    }

    /// inserts the given value `val` into the field `e`
    pub fn e_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..0
        self.0.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `e` field, and writes the updated value
    pub fn e_write(&mut self, val: u64) {
        Self::with_reg_val().e_insert(val).write();
    }

}

impl Default for PmblimitrEl1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> PmblimitrEl1 {
        PmblimitrEl1(0)
    }
}
