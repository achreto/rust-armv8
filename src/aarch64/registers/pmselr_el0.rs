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
 * Generated on: 2022-08-22T16:35:53.074720
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
 * Register:    Performance Monitors Event Counter Selection Register (pmselr_el0)
 * Group:       Performance Monitors registers
 * Type:        64-bit Register
 * Description: Selects the current event counter
 * File:        AArch64-pmselr_el0.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Performance Monitors Event Counter Selection Register value in memory
pub struct PmselrEl0(u64);

/// struct implementation for accessing the fields of register pmselr_el0
impl PmselrEl0 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> PmselrEl0 {
        Self::default()
    }

    /// collects the modifications of PmselrEl0 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> PmselrEl0 {
        PmselrEl0(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> PmselrEl0 {
        let curval = Self::reg_rawrd() & 0x1f;
        PmselrEl0(curval)
    }

    /// reading the Performance Monitors Event Counter Selection Register (pmselr_el0) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, PMSELR_EL0
            asm!("mrs {}, pmselr_el0", out(reg) regval);
        }
        return regval;
    }

    /// writing the Performance Monitors Event Counter Selection Register (pmselr_el0) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR PMSELR_EL0, <Xt>
            asm!("msr pmselr_el0, {}", in(reg) val);
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
     * Field: sel
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn sel_extract(&self) -> u64 {
        // bits 0..4
        self.0.get_bits(0..=4)
    }

    /// reads the current register value and extract field `sel` from it
    pub fn sel_read() -> u64 {
        Self::with_reg_val().sel_extract()
    }

    /// inserts the given value `val` into the field `sel`
    pub fn sel_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..4
        self.0.set_bits(0..=4, val);
        self
    }

    /// reads the register, updates the `sel` field, and writes the updated value
    pub fn sel_write(val: u64) {
        Self::with_reg_val().sel_insert(val).write();
    }
}

impl Default for PmselrEl0 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> PmselrEl0 {
        PmselrEl0(0)
    }
}
