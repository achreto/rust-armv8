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
 * Generated on: 2022-08-22T16:35:53.075901
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
 * Register:    Performance Monitors Software Increment register (pmswinc_el0)
 * Group:       Performance Monitors registers
 * Type:        64-bit Register
 * Description: Increments a counter that is configured to count the Software increment event, event
 * File:        AArch64-pmswinc_el0.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Performance Monitors Software Increment register value in memory
pub struct PmswincEl0(u64);

/// struct implementation for accessing the fields of register pmswinc_el0
impl PmswincEl0 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> PmswincEl0 {
        Self::default()
    }

    /// collects the modifications of PmswincEl0 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> PmswincEl0 {
        PmswincEl0(self.0)
    }

    // no current() method as it is write only

    /// writing the Performance Monitors Software Increment register (pmswinc_el0) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR PMSWINC_EL0, <Xt>
            asm!("msr pmswinc_el0, {}", in(reg) val);
        }
    }

    // register is not readable. not emitting read accessor

    // no read() method as it is write only

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 2147483647;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: pn
     * --------------------------------------------------------------------------------------------
     */

    // no extract() method for field pn
    /// inserts the given value `val` into the field `pn`
    pub fn pn_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..30
        self.0.set_bits(0..=30, val);
        self
    }

    /// sets the field `pn` to the given value `val`
    pub fn pn_write(&mut self, val: u64) {
        Self::default().pn_insert(val).write();
    }
}

impl Default for PmswincEl0 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> PmswincEl0 {
        PmswincEl0(0)
    }
}
