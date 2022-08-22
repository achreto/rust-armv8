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
 * Generated on: 2022-08-22T16:35:53.054391
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
 * Register:    Debug Data Transfer Register, half-duplex (dbgdtr_el0)
 * Group:       Debug registers
 * Type:        64-bit Register
 * Description: Transfers 64 bits of data between the PE and an external debugger. Can transfer both ways using only a single register.
 * File:        AArch64-dbgdtr_el0.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Debug Data Transfer Register, half-duplex value in memory
pub struct DbgdtrEl0(u64);

/// struct implementation for accessing the fields of register dbgdtr_el0
impl DbgdtrEl0 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> DbgdtrEl0 {
        Self::default()
    }

    /// collects the modifications of DbgdtrEl0 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> DbgdtrEl0 {
        DbgdtrEl0(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  DbgdtrEl0 {
        let curval = Self::reg_rawrd() & 0xffffffffffffffff;
        DbgdtrEl0(curval)
    }


    
    /// reading the Debug Data Transfer Register, half-duplex (dbgdtr_el0) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, DBGDTR_EL0
            asm!("mrs {}, dbgdtr_el0", out(reg) regval);
        }
        return regval;
    }


    /// writing the Debug Data Transfer Register, half-duplex (dbgdtr_el0) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR DBGDTR_EL0, <Xt>
            asm!("msr dbgdtr_el0, {}", in(reg) val);
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
     * Field: highword
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn highword_extract(&self) -> u64 {
        // bits 32..63
        self.0.get_bits(32..=63)
    }

    /// reads the current register value and extract field `highword` from it
    pub fn highword_read() -> u64 {
        Self::with_reg_val().highword_extract()
    }

    /// inserts the given value `val` into the field `highword`
    pub fn highword_insert(&mut self, val: u64) -> &mut Self {
        // bits 32..63
        self.0.set_bits(32..=63, val);
        self
    }

    /// reads the register, updates the `highword` field, and writes the updated value
    pub fn highword_write(val: u64) {
        Self::with_reg_val().highword_insert(val).write();
    }

    /*
     * Field: lowword
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn lowword_extract(&self) -> u64 {
        // bits 0..31
        self.0.get_bits(0..=31)
    }

    /// reads the current register value and extract field `lowword` from it
    pub fn lowword_read() -> u64 {
        Self::with_reg_val().lowword_extract()
    }

    /// inserts the given value `val` into the field `lowword`
    pub fn lowword_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..31
        self.0.set_bits(0..=31, val);
        self
    }

    /// reads the register, updates the `lowword` field, and writes the updated value
    pub fn lowword_write(val: u64) {
        Self::with_reg_val().lowword_insert(val).write();
    }

}

impl Default for DbgdtrEl0 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> DbgdtrEl0 {
        DbgdtrEl0(0)
    }
}
