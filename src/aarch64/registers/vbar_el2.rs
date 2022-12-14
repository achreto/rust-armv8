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
 * Generated on: 2022-08-22T16:35:53.082981
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
 * Register:    Vector Base Address Register (EL2) (vbar_el2)
 * Group:       Exception and fault handling registers
 * Type:        64-bit Register
 * Description: Holds the vector base address for any exception that is taken to EL2.
 * File:        AArch64-vbar_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Vector Base Address Register (EL2) value in memory
pub struct VbarEl2(u64);

/// struct implementation for accessing the fields of register vbar_el2
impl VbarEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> VbarEl2 {
        Self::default()
    }

    /// collects the modifications of VbarEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> VbarEl2 {
        VbarEl2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> VbarEl2 {
        let curval = Self::reg_rawrd() & 0xfffffffffffff800;
        VbarEl2(curval)
    }

    /// reading the Vector Base Address Register (EL2) (vbar_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, VBAR_EL2
            asm!("mrs {}, vbar_el2", out(reg) regval);
        }
        return regval;
    }

    /// writing the Vector Base Address Register (EL2) (vbar_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR VBAR_EL2, <Xt>
            asm!("msr vbar_el2, {}", in(reg) val);
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xfffffffffffff800;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 18446744073709549568;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: none_63_11
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn none_63_11_extract(&self) -> u64 {
        // bits 11..63
        self.0.get_bits(11..=63)
    }

    /// reads the current register value and extract field `none_63_11` from it
    pub fn none_63_11_read() -> u64 {
        Self::with_reg_val().none_63_11_extract()
    }

    /// inserts the given value `val` into the field `none_63_11`
    pub fn none_63_11_insert(&mut self, val: u64) -> &mut Self {
        // bits 11..63
        self.0.set_bits(11..=63, val);
        self
    }

    /// reads the register, updates the `none_63_11` field, and writes the updated value
    pub fn none_63_11_write(val: u64) {
        Self::with_reg_val().none_63_11_insert(val).write();
    }
}

impl Default for VbarEl2 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> VbarEl2 {
        VbarEl2(0)
    }
}
