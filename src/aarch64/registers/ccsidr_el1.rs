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
 * Generated on: 2022-08-22T16:35:53.050242
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
 * Register:    Current Cache Size ID Register (ccsidr_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides information about the architecture of the currently selected cache.
 * File:        AArch64-ccsidr_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Current Cache Size ID Register value in memory
pub struct CcsidrEl1(u64);

/// struct implementation for accessing the fields of register ccsidr_el1
impl CcsidrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> CcsidrEl1 {
        Self::default()
    }

    /// collects the modifications of CcsidrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> CcsidrEl1 {
        CcsidrEl1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> CcsidrEl1 {
        let curval = Self::reg_rawrd() & 0xffffff00ffffff;
        CcsidrEl1(curval)
    }

    /// reading the Current Cache Size ID Register (ccsidr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, CCSIDR_EL1
            asm!("mrs {}, ccsidr_el1", out(reg) regval);
        }
        return regval;
    }

    // register is not writable. not emitting write accessor

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xffffff00ffffff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 72057589759737855;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: numsets
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn numsets_extract(&self) -> u64 {
        // bits 32..55
        self.0.get_bits(32..=55)
    }

    /// reads the current register value and extract field `numsets` from it
    pub fn numsets_read() -> u64 {
        Self::with_reg_val().numsets_extract()
    }
    // no insert() method for field numsets
    /*
     * Field: associativity
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn associativity_extract(&self) -> u64 {
        // bits 3..23
        self.0.get_bits(3..=23)
    }

    /// reads the current register value and extract field `associativity` from it
    pub fn associativity_read() -> u64 {
        Self::with_reg_val().associativity_extract()
    }
    // no insert() method for field associativity
    /*
     * Field: linesize
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn linesize_extract(&self) -> u64 {
        // bits 0..2
        self.0.get_bits(0..=2)
    }

    /// reads the current register value and extract field `linesize` from it
    pub fn linesize_read() -> u64 {
        Self::with_reg_val().linesize_extract()
    }
    // no insert() method for field linesize
}

impl Default for CcsidrEl1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> CcsidrEl1 {
        CcsidrEl1(0)
    }
}
