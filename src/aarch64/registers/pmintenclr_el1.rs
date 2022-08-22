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
 * Generated on: 2022-08-22T16:35:53.074041
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
 * Register:    Performance Monitors Interrupt Enable Clear register (pmintenclr_el1)
 * Group:       Performance Monitors registers
 * Type:        64-bit Register
 * Description: Disables the generation of interrupt requests on overflows from the Cycle Count Register, 
 * File:        AArch64-pmintenclr_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Performance Monitors Interrupt Enable Clear register value in memory
pub struct PmintenclrEl1(u64);

/// struct implementation for accessing the fields of register pmintenclr_el1
impl PmintenclrEl1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> PmintenclrEl1 {
        Self::default()
    }

    /// collects the modifications of PmintenclrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> PmintenclrEl1 {
        PmintenclrEl1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  PmintenclrEl1 {
        let curval = Self::reg_rawrd() & 0xffffffff;
        PmintenclrEl1(curval)
    }


    
    /// reading the Performance Monitors Interrupt Enable Clear register (pmintenclr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, PMINTENCLR_EL1
            asm!("mrs {}, pmintenclr_el1", out(reg) regval);
        }
        return regval;
    }


    /// writing the Performance Monitors Interrupt Enable Clear register (pmintenclr_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR PMINTENCLR_EL1, <Xt>
            asm!("msr pmintenclr_el1, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xffffffff;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 4294967295;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: c
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn c_extract(&self) -> u64 {
        // bits 31..31
        self.0.get_bits(31..=31)
    }

    /// reads the current register value and extract field `c` from it
    pub fn c_read() -> u64 {
        Self::with_reg_val().c_extract()
    }

    /// inserts the given value `val` into the field `c`
    pub fn c_insert(&mut self, val: u64) -> &mut Self {
        // bits 31..31
        self.0.set_bits(31..=31, val);
        self
    }

    /// reads the register, updates the `c` field, and writes the updated value
    pub fn c_write(val: u64) {
        Self::with_reg_val().c_insert(val).write();
    }

    /*
     * Field: pn
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn pn_extract(&self) -> u64 {
        // bits 0..30
        self.0.get_bits(0..=30)
    }

    /// reads the current register value and extract field `pn` from it
    pub fn pn_read() -> u64 {
        Self::with_reg_val().pn_extract()
    }

    /// inserts the given value `val` into the field `pn`
    pub fn pn_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..30
        self.0.set_bits(0..=30, val);
        self
    }

    /// reads the register, updates the `pn` field, and writes the updated value
    pub fn pn_write(val: u64) {
        Self::with_reg_val().pn_insert(val).write();
    }

}

impl Default for PmintenclrEl1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> PmintenclrEl1 {
        PmintenclrEl1(0)
    }
}
