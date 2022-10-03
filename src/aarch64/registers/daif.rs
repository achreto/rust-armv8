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
 * Generated on: 2022-08-22T16:35:53.053877
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
 * Register:    Interrupt Mask Bits (daif)
 * Group:       Process state registers
 * Type:        64-bit Register
 * Description: Allows access to the interrupt mask bits.
 * File:        AArch64-daif.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Interrupt Mask Bits value in memory
pub struct Daif(u64);

/// struct implementation for accessing the fields of register daif
impl Daif {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> Daif {
        Self::default()
    }

    /// collects the modifications of Daif and creates new object
    #[inline(always)]
    pub fn collect(&self) -> Daif {
        Daif(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> Daif {
        let curval = Self::reg_rawrd() & 0x3c0;
        Daif(curval)
    }

    /// reading the Interrupt Mask Bits (daif) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, DAIF
            asm!("mrs {}, daif", out(reg) regval);
        }
        return regval;
    }

    /// writing the Interrupt Mask Bits (daif) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR DAIF, <Xt>
            asm!("msr daif, {}", in(reg) val);
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x3c0;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 960;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: d
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn d_extract(&self) -> u64 {
        // bits 9..9
        self.0.get_bits(9..=9)
    }

    /// reads the current register value and extract field `d` from it
    pub fn d_read() -> u64 {
        Self::with_reg_val().d_extract()
    }

    /// inserts the given value `val` into the field `d`
    pub fn d_insert(&mut self, val: u64) -> &mut Self {
        // bits 9..9
        self.0.set_bits(9..=9, val);
        self
    }

    /// reads the register, updates the `d` field, and writes the updated value
    pub fn d_write(val: u64) {
        Self::with_reg_val().d_insert(val).write();
    }

    /*
     * Field: a
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn a_extract(&self) -> u64 {
        // bits 8..8
        self.0.get_bits(8..=8)
    }

    /// reads the current register value and extract field `a` from it
    pub fn a_read() -> u64 {
        Self::with_reg_val().a_extract()
    }

    /// inserts the given value `val` into the field `a`
    pub fn a_insert(&mut self, val: u64) -> &mut Self {
        // bits 8..8
        self.0.set_bits(8..=8, val);
        self
    }

    /// reads the register, updates the `a` field, and writes the updated value
    pub fn a_write(val: u64) {
        Self::with_reg_val().a_insert(val).write();
    }

    /*
     * Field: i
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn i_extract(&self) -> u64 {
        // bits 7..7
        self.0.get_bits(7..=7)
    }

    /// reads the current register value and extract field `i` from it
    pub fn i_read() -> u64 {
        Self::with_reg_val().i_extract()
    }

    /// inserts the given value `val` into the field `i`
    pub fn i_insert(&mut self, val: u64) -> &mut Self {
        // bits 7..7
        self.0.set_bits(7..=7, val);
        self
    }

    /// reads the register, updates the `i` field, and writes the updated value
    pub fn i_write(val: u64) {
        Self::with_reg_val().i_insert(val).write();
    }

    /*
     * Field: f
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn f_extract(&self) -> u64 {
        // bits 6..6
        self.0.get_bits(6..=6)
    }

    /// reads the current register value and extract field `f` from it
    pub fn f_read() -> u64 {
        Self::with_reg_val().f_extract()
    }

    /// inserts the given value `val` into the field `f`
    pub fn f_insert(&mut self, val: u64) -> &mut Self {
        // bits 6..6
        self.0.set_bits(6..=6, val);
        self
    }

    /// reads the register, updates the `f` field, and writes the updated value
    pub fn f_write(val: u64) {
        Self::with_reg_val().f_insert(val).write();
    }
}

impl Default for Daif {
    /// creates a new default value
    #[inline(always)]
    fn default() -> Daif {
        Daif(0)
    }
}
