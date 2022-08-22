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
 * Generated on: 2022-08-22T16:25:59.093014
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
 * Register:    Statistical Profiling Control Register (EL2) (pmscr_el2)
 * Group:       Statistical Profiling Extension registers
 * Type:        64-bit Register
 * Description: Provides EL2 controls for Statistical Profiling.
 * File:        AArch64-pmscr_el2.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Statistical Profiling Control Register (EL2) value in memory
pub struct PmscrEl2(u64);

/// struct implementation for accessing the fields of register pmscr_el2
impl PmscrEl2 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> PmscrEl2 {
        Self::default()
    }

    /// collects the modifications of PmscrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> PmscrEl2 {
        PmscrEl2(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  PmscrEl2 {
        let curval = Self::reg_rawrd() & 0xfb;
        PmscrEl2(curval)
    }


    
    /// reading the Statistical Profiling Control Register (EL2) (pmscr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, PMSCR_EL2
            asm!("mrs {}, S3_4_C9_C9_0", out(reg) regval);
        }
        return regval;
    }


    /// writing the Statistical Profiling Control Register (EL2) (pmscr_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR PMSCR_EL2, <Xt>
            asm!("msr S3_4_C9_C9_0, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xfb;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 251;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: pct
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn pct_extract(&self) -> u64 {
        // bits 6..7
        self.0.get_bits(6..=7)
    }

    /// reads the current register value and extract field `pct` from it
    pub fn pct_read(&mut self) -> u64 {
        Self::with_reg_val().pct_extract()
    }

    /// inserts the given value `val` into the field `pct`
    pub fn pct_insert(&mut self, val: u64) -> &mut Self {
        // bits 6..7
        self.0.set_bits(6..=7, val);
        self
    }

    /// reads the register, updates the `pct` field, and writes the updated value
    pub fn pct_write(&mut self, val: u64) {
        Self::with_reg_val().pct_insert(val).write();
    }

    /*
     * Field: ts
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ts_extract(&self) -> u64 {
        // bits 5..5
        self.0.get_bits(5..=5)
    }

    /// reads the current register value and extract field `ts` from it
    pub fn ts_read(&mut self) -> u64 {
        Self::with_reg_val().ts_extract()
    }

    /// inserts the given value `val` into the field `ts`
    pub fn ts_insert(&mut self, val: u64) -> &mut Self {
        // bits 5..5
        self.0.set_bits(5..=5, val);
        self
    }

    /// reads the register, updates the `ts` field, and writes the updated value
    pub fn ts_write(&mut self, val: u64) {
        Self::with_reg_val().ts_insert(val).write();
    }

    /*
     * Field: pa
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn pa_extract(&self) -> u64 {
        // bits 4..4
        self.0.get_bits(4..=4)
    }

    /// reads the current register value and extract field `pa` from it
    pub fn pa_read(&mut self) -> u64 {
        Self::with_reg_val().pa_extract()
    }

    /// inserts the given value `val` into the field `pa`
    pub fn pa_insert(&mut self, val: u64) -> &mut Self {
        // bits 4..4
        self.0.set_bits(4..=4, val);
        self
    }

    /// reads the register, updates the `pa` field, and writes the updated value
    pub fn pa_write(&mut self, val: u64) {
        Self::with_reg_val().pa_insert(val).write();
    }

    /*
     * Field: cx
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn cx_extract(&self) -> u64 {
        // bits 3..3
        self.0.get_bits(3..=3)
    }

    /// reads the current register value and extract field `cx` from it
    pub fn cx_read(&mut self) -> u64 {
        Self::with_reg_val().cx_extract()
    }

    /// inserts the given value `val` into the field `cx`
    pub fn cx_insert(&mut self, val: u64) -> &mut Self {
        // bits 3..3
        self.0.set_bits(3..=3, val);
        self
    }

    /// reads the register, updates the `cx` field, and writes the updated value
    pub fn cx_write(&mut self, val: u64) {
        Self::with_reg_val().cx_insert(val).write();
    }

    /*
     * Field: e2spe
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e2spe_extract(&self) -> u64 {
        // bits 1..1
        self.0.get_bits(1..=1)
    }

    /// reads the current register value and extract field `e2spe` from it
    pub fn e2spe_read(&mut self) -> u64 {
        Self::with_reg_val().e2spe_extract()
    }

    /// inserts the given value `val` into the field `e2spe`
    pub fn e2spe_insert(&mut self, val: u64) -> &mut Self {
        // bits 1..1
        self.0.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `e2spe` field, and writes the updated value
    pub fn e2spe_write(&mut self, val: u64) {
        Self::with_reg_val().e2spe_insert(val).write();
    }

    /*
     * Field: e0hspe
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e0hspe_extract(&self) -> u64 {
        // bits 0..0
        self.0.get_bits(0..=0)
    }

    /// reads the current register value and extract field `e0hspe` from it
    pub fn e0hspe_read(&mut self) -> u64 {
        Self::with_reg_val().e0hspe_extract()
    }

    /// inserts the given value `val` into the field `e0hspe`
    pub fn e0hspe_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..0
        self.0.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `e0hspe` field, and writes the updated value
    pub fn e0hspe_write(&mut self, val: u64) {
        Self::with_reg_val().e0hspe_insert(val).write();
    }

}

impl Default for PmscrEl2 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> PmscrEl2 {
        PmscrEl2(0)
    }
}
