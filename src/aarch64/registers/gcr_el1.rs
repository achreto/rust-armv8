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
 * Generated on: 2022-08-22T16:35:53.057827
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
 * Register:    Tag Control Register. (gcr_el1)
 * Group:       Generic System Control
 * Type:        64-bit Register
 * Description: Tag Control Register.
 * File:        AArch64-gcr_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Tag Control Register. value in memory
pub struct GcrEl1(u64);

/// struct implementation for accessing the fields of register gcr_el1
impl GcrEl1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> GcrEl1 {
        Self::default()
    }

    /// collects the modifications of GcrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> GcrEl1 {
        GcrEl1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  GcrEl1 {
        let curval = Self::reg_rawrd() & 0x1ffff;
        GcrEl1(curval)
    }


    
    /// reading the Tag Control Register. (gcr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, GCR_EL1
            asm!("mrs {}, S3_0_C1_C0_6", out(reg) regval);
        }
        return regval;
    }


    /// writing the Tag Control Register. (gcr_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR GCR_EL1, <Xt>
            asm!("msr S3_0_C1_C0_6, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x1ffff;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 131071;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: rrnd
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn rrnd_extract(&self) -> u64 {
        // bits 16..16
        self.0.get_bits(16..=16)
    }

    /// reads the current register value and extract field `rrnd` from it
    pub fn rrnd_read() -> u64 {
        Self::with_reg_val().rrnd_extract()
    }

    /// inserts the given value `val` into the field `rrnd`
    pub fn rrnd_insert(&mut self, val: u64) -> &mut Self {
        // bits 16..16
        self.0.set_bits(16..=16, val);
        self
    }

    /// reads the register, updates the `rrnd` field, and writes the updated value
    pub fn rrnd_write(val: u64) {
        Self::with_reg_val().rrnd_insert(val).write();
    }

    /*
     * Field: exclude
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn exclude_extract(&self) -> u64 {
        // bits 0..15
        self.0.get_bits(0..=15)
    }

    /// reads the current register value and extract field `exclude` from it
    pub fn exclude_read() -> u64 {
        Self::with_reg_val().exclude_extract()
    }

    /// inserts the given value `val` into the field `exclude`
    pub fn exclude_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..15
        self.0.set_bits(0..=15, val);
        self
    }

    /// reads the register, updates the `exclude` field, and writes the updated value
    pub fn exclude_write(val: u64) {
        Self::with_reg_val().exclude_insert(val).write();
    }

}

impl Default for GcrEl1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> GcrEl1 {
        GcrEl1(0)
    }
}
