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
 * Generated on: 2022-08-22T16:35:53.078465
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
 * Register:    AArch32 Secure Debug Enable Register (sder32_el3)
 * Group:       Security registers
 * Type:        64-bit Register
 * Description: Allows access to the AArch32 register
 * File:        AArch64-sder32_el3.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the AArch32 Secure Debug Enable Register value in memory
pub struct Sder32El3(u64);

/// struct implementation for accessing the fields of register sder32_el3
impl Sder32El3 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> Sder32El3 {
        Self::default()
    }

    /// collects the modifications of Sder32El3 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> Sder32El3 {
        Sder32El3(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> Sder32El3 {
        let curval = Self::reg_rawrd() & 0x3;
        Sder32El3(curval)
    }

    /// reading the AArch32 Secure Debug Enable Register (sder32_el3) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, SDER32_EL3
            asm!("mrs {}, sder32_el3", out(reg) regval);
        }
        return regval;
    }

    /// writing the AArch32 Secure Debug Enable Register (sder32_el3) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR SDER32_EL3, <Xt>
            asm!("msr sder32_el3, {}", in(reg) val);
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x3;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 3;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: suniden
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn suniden_extract(&self) -> u64 {
        // bits 1..1
        self.0.get_bits(1..=1)
    }

    /// reads the current register value and extract field `suniden` from it
    pub fn suniden_read() -> u64 {
        Self::with_reg_val().suniden_extract()
    }

    /// inserts the given value `val` into the field `suniden`
    pub fn suniden_insert(&mut self, val: u64) -> &mut Self {
        // bits 1..1
        self.0.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `suniden` field, and writes the updated value
    pub fn suniden_write(val: u64) {
        Self::with_reg_val().suniden_insert(val).write();
    }

    /*
     * Field: suiden
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn suiden_extract(&self) -> u64 {
        // bits 0..0
        self.0.get_bits(0..=0)
    }

    /// reads the current register value and extract field `suiden` from it
    pub fn suiden_read() -> u64 {
        Self::with_reg_val().suiden_extract()
    }

    /// inserts the given value `val` into the field `suiden`
    pub fn suiden_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..0
        self.0.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `suiden` field, and writes the updated value
    pub fn suiden_write(val: u64) {
        Self::with_reg_val().suiden_insert(val).write();
    }
}

impl Default for Sder32El3 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> Sder32El3 {
        Sder32El3(0)
    }
}
