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
 * Generated on: 2022-08-22T16:35:53.071771
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
 * Register:    OS Double Lock Register (osdlr_el1)
 * Group:       Debug registers
 * Type:        64-bit Register
 * Description: Used to control the OS Double Lock.
 * File:        AArch64-osdlr_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the OS Double Lock Register value in memory
pub struct OsdlrEl1(u64);

/// struct implementation for accessing the fields of register osdlr_el1
impl OsdlrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> OsdlrEl1 {
        Self::default()
    }

    /// collects the modifications of OsdlrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> OsdlrEl1 {
        OsdlrEl1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> OsdlrEl1 {
        let curval = Self::reg_rawrd() & 0x1;
        OsdlrEl1(curval)
    }

    /// reading the OS Double Lock Register (osdlr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, OSDLR_EL1
            asm!("mrs {}, osdlr_el1", out(reg) regval);
        }
        return regval;
    }

    /// writing the OS Double Lock Register (osdlr_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR OSDLR_EL1, <Xt>
            asm!("msr osdlr_el1, {}", in(reg) val);
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x1;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 1;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: dlk_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dlk_1_extract(&self) -> u64 {
        // bits 0..0
        self.0.get_bits(0..=0)
    }

    /// reads the current register value and extract field `dlk_1` from it
    pub fn dlk_1_read() -> u64 {
        Self::with_reg_val().dlk_1_extract()
    }

    /// inserts the given value `val` into the field `dlk_1`
    pub fn dlk_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..0
        self.0.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `dlk_1` field, and writes the updated value
    pub fn dlk_1_write(val: u64) {
        Self::with_reg_val().dlk_1_insert(val).write();
    }
}

impl Default for OsdlrEl1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> OsdlrEl1 {
        OsdlrEl1(0)
    }
}
