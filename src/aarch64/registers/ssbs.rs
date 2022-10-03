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
 * Generated on: 2022-08-22T16:35:53.080253
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
 * Register:    Speculative Store Bypass Safe (ssbs)
 * Group:       Process state registers
 * Type:        64-bit Register
 * Description: Allows access to the Speculative Store Bypass Safe bit.
 * File:        AArch64-ssbs.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Speculative Store Bypass Safe value in memory
pub struct Ssbs(u64);

/// struct implementation for accessing the fields of register ssbs
impl Ssbs {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> Ssbs {
        Self::default()
    }

    /// collects the modifications of Ssbs and creates new object
    #[inline(always)]
    pub fn collect(&self) -> Ssbs {
        Ssbs(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> Ssbs {
        let curval = Self::reg_rawrd() & 0x1000;
        Ssbs(curval)
    }

    /// reading the Speculative Store Bypass Safe (ssbs) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, SSBS
            asm!("mrs {}, S3_3_C4_C2_6", out(reg) regval);
        }
        return regval;
    }

    /// writing the Speculative Store Bypass Safe (ssbs) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR SSBS, <Xt>
            asm!("msr S3_3_C4_C2_6, {}", in(reg) val);
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x1000;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 4096;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: ssbs
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ssbs_extract(&self) -> u64 {
        // bits 12..12
        self.0.get_bits(12..=12)
    }

    /// reads the current register value and extract field `ssbs` from it
    pub fn ssbs_read() -> u64 {
        Self::with_reg_val().ssbs_extract()
    }

    /// inserts the given value `val` into the field `ssbs`
    pub fn ssbs_insert(&mut self, val: u64) -> &mut Self {
        // bits 12..12
        self.0.set_bits(12..=12, val);
        self
    }

    /// reads the register, updates the `ssbs` field, and writes the updated value
    pub fn ssbs_write(val: u64) {
        Self::with_reg_val().ssbs_insert(val).write();
    }
}

impl Default for Ssbs {
    /// creates a new default value
    #[inline(always)]
    fn default() -> Ssbs {
        Ssbs(0)
    }
}
