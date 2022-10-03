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
 * Generated on: 2022-08-22T16:35:53.073618
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
 * Register:    Performance Monitors Common Event Identification register 1 (pmceid1_el0)
 * Group:       Performance Monitors registers
 * Type:        64-bit Register
 * Description: Defines which common architectural events and common microarchitectural  events are implemented, or counted, using PMU events in the ranges
 * File:        AArch64-pmceid1_el0.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Performance Monitors Common Event Identification register 1 value in memory
pub struct Pmceid1El0(u64);

/// struct implementation for accessing the fields of register pmceid1_el0
impl Pmceid1El0 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> Pmceid1El0 {
        Self::default()
    }

    /// collects the modifications of Pmceid1El0 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> Pmceid1El0 {
        Pmceid1El0(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> Pmceid1El0 {
        let curval = Self::reg_rawrd() & 0xffffffffffffffff;
        Pmceid1El0(curval)
    }

    /// reading the Performance Monitors Common Event Identification register 1 (pmceid1_el0) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, PMCEID1_EL0
            asm!("mrs {}, pmceid1_el0", out(reg) regval);
        }
        return regval;
    }

    // register is not writable. not emitting write accessor

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xffffffffffffffff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 18446744073709551615;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: idhin_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn idhin_1_extract(&self) -> u64 {
        // bits 32..63
        self.0.get_bits(32..=63)
    }

    /// reads the current register value and extract field `idhin_1` from it
    pub fn idhin_1_read() -> u64 {
        Self::with_reg_val().idhin_1_extract()
    }
    // no insert() method for field idhin_1
    /*
     * Field: idn
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn idn_extract(&self) -> u64 {
        // bits 0..31
        self.0.get_bits(0..=31)
    }

    /// reads the current register value and extract field `idn` from it
    pub fn idn_read() -> u64 {
        Self::with_reg_val().idn_extract()
    }
    // no insert() method for field idn
}

impl Default for Pmceid1El0 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> Pmceid1El0 {
        Pmceid1El0(0)
    }
}
