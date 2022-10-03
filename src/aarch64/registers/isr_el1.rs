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
 * Generated on: 2022-08-22T16:35:53.068152
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
 * Register:    Interrupt Status Register (isr_el1)
 * Group:       Exception and fault handling registers
 * Type:        64-bit Register
 * Description: Shows the pending status of the IRQ, FIQ, or SError interrupt.
 * File:        AArch64-isr_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Interrupt Status Register value in memory
pub struct IsrEl1(u64);

/// struct implementation for accessing the fields of register isr_el1
impl IsrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IsrEl1 {
        Self::default()
    }

    /// collects the modifications of IsrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IsrEl1 {
        IsrEl1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> IsrEl1 {
        let curval = Self::reg_rawrd() & 0x1c0;
        IsrEl1(curval)
    }

    /// reading the Interrupt Status Register (isr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ISR_EL1
            asm!("mrs {}, isr_el1", out(reg) regval);
        }
        return regval;
    }

    // register is not writable. not emitting write accessor

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x1c0;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 448;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
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
    // no insert() method for field a
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
    // no insert() method for field i
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
    // no insert() method for field f
}

impl Default for IsrEl1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> IsrEl1 {
        IsrEl1(0)
    }
}
