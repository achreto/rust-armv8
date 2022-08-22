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
 * Generated on: 2022-08-22T16:35:53.053536
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
 * Register:    Cache Size Selection Register (csselr_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Selects the current Cache Size ID Register, 
 * File:        AArch64-csselr_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Cache Size Selection Register value in memory
pub struct CsselrEl1(u64);

/// struct implementation for accessing the fields of register csselr_el1
impl CsselrEl1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> CsselrEl1 {
        Self::default()
    }

    /// collects the modifications of CsselrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> CsselrEl1 {
        CsselrEl1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  CsselrEl1 {
        let curval = Self::reg_rawrd() & 0x1f;
        CsselrEl1(curval)
    }


    
    /// reading the Cache Size Selection Register (csselr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, CSSELR_EL1
            asm!("mrs {}, csselr_el1", out(reg) regval);
        }
        return regval;
    }


    /// writing the Cache Size Selection Register (csselr_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR CSSELR_EL1, <Xt>
            asm!("msr csselr_el1, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x1f;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 31;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: tnd_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tnd_1_extract(&self) -> u64 {
        // bits 4..4
        self.0.get_bits(4..=4)
    }

    /// reads the current register value and extract field `tnd_1` from it
    pub fn tnd_1_read() -> u64 {
        Self::with_reg_val().tnd_1_extract()
    }

    /// inserts the given value `val` into the field `tnd_1`
    pub fn tnd_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 4..4
        self.0.set_bits(4..=4, val);
        self
    }

    /// reads the register, updates the `tnd_1` field, and writes the updated value
    pub fn tnd_1_write(val: u64) {
        Self::with_reg_val().tnd_1_insert(val).write();
    }

    /*
     * Field: level
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn level_extract(&self) -> u64 {
        // bits 1..3
        self.0.get_bits(1..=3)
    }

    /// reads the current register value and extract field `level` from it
    pub fn level_read() -> u64 {
        Self::with_reg_val().level_extract()
    }

    /// inserts the given value `val` into the field `level`
    pub fn level_insert(&mut self, val: u64) -> &mut Self {
        // bits 1..3
        self.0.set_bits(1..=3, val);
        self
    }

    /// reads the register, updates the `level` field, and writes the updated value
    pub fn level_write(val: u64) {
        Self::with_reg_val().level_insert(val).write();
    }

    /*
     * Field: ind
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ind_extract(&self) -> u64 {
        // bits 0..0
        self.0.get_bits(0..=0)
    }

    /// reads the current register value and extract field `ind` from it
    pub fn ind_read() -> u64 {
        Self::with_reg_val().ind_extract()
    }

    /// inserts the given value `val` into the field `ind`
    pub fn ind_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..0
        self.0.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `ind` field, and writes the updated value
    pub fn ind_write(val: u64) {
        Self::with_reg_val().ind_insert(val).write();
    }

}

impl Default for CsselrEl1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> CsselrEl1 {
        CsselrEl1(0)
    }
}
