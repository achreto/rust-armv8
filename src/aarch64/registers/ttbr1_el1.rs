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
 * Generated on: 2022-08-22T16:25:59.101014
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
 * Register:    Translation Table Base Register 1 (EL1) (ttbr1_el1)
 * Group:       Virtual memory control registers
 * Type:        64-bit Register
 * Description: Holds the base address of the translation table for the initial lookup for stage 1 of the translation of an address from the higher VA range in the EL1&0 stage 1 translation regime, and other information for this translation regime.
 * File:        AArch64-ttbr1_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Translation Table Base Register 1 (EL1) value in memory
pub struct Ttbr1El1(u64);

/// struct implementation for accessing the fields of register ttbr1_el1
impl Ttbr1El1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> Ttbr1El1 {
        Self::default()
    }

    /// collects the modifications of Ttbr1El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> Ttbr1El1 {
        Ttbr1El1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  Ttbr1El1 {
        let curval = Self::reg_rawrd() & 0xffffffffffffffff;
        Ttbr1El1(curval)
    }


    
    /// reading the Translation Table Base Register 1 (EL1) (ttbr1_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, TTBR1_EL1
            asm!("mrs {}, ttbr1_el1", out(reg) regval);
        }
        return regval;
    }


    /// writing the Translation Table Base Register 1 (EL1) (ttbr1_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR TTBR1_EL1, <Xt>
            asm!("msr ttbr1_el1, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xffffffffffffffff;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 18446744073709551615;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: asid
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn asid_extract(&self) -> u64 {
        // bits 48..63
        self.0.get_bits(48..=63)
    }

    /// reads the current register value and extract field `asid` from it
    pub fn asid_read(&mut self) -> u64 {
        Self::with_reg_val().asid_extract()
    }

    /// inserts the given value `val` into the field `asid`
    pub fn asid_insert(&mut self, val: u64) -> &mut Self {
        // bits 48..63
        self.0.set_bits(48..=63, val);
        self
    }

    /// reads the register, updates the `asid` field, and writes the updated value
    pub fn asid_write(&mut self, val: u64) {
        Self::with_reg_val().asid_insert(val).write();
    }

    /*
     * Field: baddr471
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn baddr471_extract(&self) -> u64 {
        // bits 1..47
        self.0.get_bits(1..=47)
    }

    /// reads the current register value and extract field `baddr471` from it
    pub fn baddr471_read(&mut self) -> u64 {
        Self::with_reg_val().baddr471_extract()
    }

    /// inserts the given value `val` into the field `baddr471`
    pub fn baddr471_insert(&mut self, val: u64) -> &mut Self {
        // bits 1..47
        self.0.set_bits(1..=47, val);
        self
    }

    /// reads the register, updates the `baddr471` field, and writes the updated value
    pub fn baddr471_write(&mut self, val: u64) {
        Self::with_reg_val().baddr471_insert(val).write();
    }

    /*
     * Field: cnp_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn cnp_1_extract(&self) -> u64 {
        // bits 0..0
        self.0.get_bits(0..=0)
    }

    /// reads the current register value and extract field `cnp_1` from it
    pub fn cnp_1_read(&mut self) -> u64 {
        Self::with_reg_val().cnp_1_extract()
    }

    /// inserts the given value `val` into the field `cnp_1`
    pub fn cnp_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..0
        self.0.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `cnp_1` field, and writes the updated value
    pub fn cnp_1_write(&mut self, val: u64) {
        Self::with_reg_val().cnp_1_insert(val).write();
    }

}

impl Default for Ttbr1El1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> Ttbr1El1 {
        Ttbr1El1(0)
    }
}