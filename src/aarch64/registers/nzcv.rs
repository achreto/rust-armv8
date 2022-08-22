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
 * Generated on: 2022-08-22T16:25:59.090540
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
 * Register:    Condition Flags (nzcv)
 * Group:       Process state registers
 * Type:        64-bit Register
 * Description: Allows access to the condition flags.
 * File:        AArch64-nzcv.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Condition Flags value in memory
pub struct Nzcv(u64);

/// struct implementation for accessing the fields of register nzcv
impl Nzcv {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> Nzcv {
        Self::default()
    }

    /// collects the modifications of Nzcv and creates new object
    #[inline(always)]
    pub fn collect(&self) -> Nzcv {
        Nzcv(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  Nzcv {
        let curval = Self::reg_rawrd() & 0xf0000000;
        Nzcv(curval)
    }


    
    /// reading the Condition Flags (nzcv) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, NZCV
            asm!("mrs {}, nzcv", out(reg) regval);
        }
        return regval;
    }


    /// writing the Condition Flags (nzcv) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR NZCV, <Xt>
            asm!("msr nzcv, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xf0000000;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 4026531840;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: n
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn n_extract(&self) -> u64 {
        // bits 31..31
        self.0.get_bits(31..=31)
    }

    /// reads the current register value and extract field `n` from it
    pub fn n_read(&mut self) -> u64 {
        Self::with_reg_val().n_extract()
    }

    /// inserts the given value `val` into the field `n`
    pub fn n_insert(&mut self, val: u64) -> &mut Self {
        // bits 31..31
        self.0.set_bits(31..=31, val);
        self
    }

    /// reads the register, updates the `n` field, and writes the updated value
    pub fn n_write(&mut self, val: u64) {
        Self::with_reg_val().n_insert(val).write();
    }

    /*
     * Field: z
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn z_extract(&self) -> u64 {
        // bits 30..30
        self.0.get_bits(30..=30)
    }

    /// reads the current register value and extract field `z` from it
    pub fn z_read(&mut self) -> u64 {
        Self::with_reg_val().z_extract()
    }

    /// inserts the given value `val` into the field `z`
    pub fn z_insert(&mut self, val: u64) -> &mut Self {
        // bits 30..30
        self.0.set_bits(30..=30, val);
        self
    }

    /// reads the register, updates the `z` field, and writes the updated value
    pub fn z_write(&mut self, val: u64) {
        Self::with_reg_val().z_insert(val).write();
    }

    /*
     * Field: c
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn c_extract(&self) -> u64 {
        // bits 29..29
        self.0.get_bits(29..=29)
    }

    /// reads the current register value and extract field `c` from it
    pub fn c_read(&mut self) -> u64 {
        Self::with_reg_val().c_extract()
    }

    /// inserts the given value `val` into the field `c`
    pub fn c_insert(&mut self, val: u64) -> &mut Self {
        // bits 29..29
        self.0.set_bits(29..=29, val);
        self
    }

    /// reads the register, updates the `c` field, and writes the updated value
    pub fn c_write(&mut self, val: u64) {
        Self::with_reg_val().c_insert(val).write();
    }

    /*
     * Field: v
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn v_extract(&self) -> u64 {
        // bits 28..28
        self.0.get_bits(28..=28)
    }

    /// reads the current register value and extract field `v` from it
    pub fn v_read(&mut self) -> u64 {
        Self::with_reg_val().v_extract()
    }

    /// inserts the given value `val` into the field `v`
    pub fn v_insert(&mut self, val: u64) -> &mut Self {
        // bits 28..28
        self.0.set_bits(28..=28, val);
        self
    }

    /// reads the register, updates the `v` field, and writes the updated value
    pub fn v_write(&mut self, val: u64) {
        Self::with_reg_val().v_insert(val).write();
    }

}

impl Default for Nzcv {
    /// creates a new default value
    #[inline(always)]
    fn default() -> Nzcv {
        Nzcv(0)
    }
}
