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
 * Generated on: 2022-08-22T16:35:53.068323
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
 * Register:    LORegion End Address (EL1) (lorea_el1)
 * Group:       Virtual memory control registers
 * Type:        64-bit Register
 * Description: Holds the physical address of the end of the LORegion described in the current LORegion descriptor selected by 
 * File:        AArch64-lorea_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the LORegion End Address (EL1) value in memory
pub struct LoreaEl1(u64);

/// struct implementation for accessing the fields of register lorea_el1
impl LoreaEl1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> LoreaEl1 {
        Self::default()
    }

    /// collects the modifications of LoreaEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> LoreaEl1 {
        LoreaEl1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  LoreaEl1 {
        let curval = Self::reg_rawrd() & 0xfffffffff0000;
        LoreaEl1(curval)
    }


    
    /// reading the LORegion End Address (EL1) (lorea_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, LOREA_EL1
            asm!("mrs {}, S3_0_C10_C4_1", out(reg) regval);
        }
        return regval;
    }


    /// writing the LORegion End Address (EL1) (lorea_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR LOREA_EL1, <Xt>
            asm!("msr S3_0_C10_C4_1, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xfffffffff0000;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 4503599627304960;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: ea5148_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ea5148_1_extract(&self) -> u64 {
        // bits 48..51
        self.0.get_bits(48..=51)
    }

    /// reads the current register value and extract field `ea5148_1` from it
    pub fn ea5148_1_read() -> u64 {
        Self::with_reg_val().ea5148_1_extract()
    }

    /// inserts the given value `val` into the field `ea5148_1`
    pub fn ea5148_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 48..51
        self.0.set_bits(48..=51, val);
        self
    }

    /// reads the register, updates the `ea5148_1` field, and writes the updated value
    pub fn ea5148_1_write(val: u64) {
        Self::with_reg_val().ea5148_1_insert(val).write();
    }

    /*
     * Field: ea4716
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ea4716_extract(&self) -> u64 {
        // bits 16..47
        self.0.get_bits(16..=47)
    }

    /// reads the current register value and extract field `ea4716` from it
    pub fn ea4716_read() -> u64 {
        Self::with_reg_val().ea4716_extract()
    }

    /// inserts the given value `val` into the field `ea4716`
    pub fn ea4716_insert(&mut self, val: u64) -> &mut Self {
        // bits 16..47
        self.0.set_bits(16..=47, val);
        self
    }

    /// reads the register, updates the `ea4716` field, and writes the updated value
    pub fn ea4716_write(val: u64) {
        Self::with_reg_val().ea4716_insert(val).write();
    }

}

impl Default for LoreaEl1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> LoreaEl1 {
        LoreaEl1(0)
    }
}
