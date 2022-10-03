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
 * Generated on: 2022-08-22T16:35:53.054876
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
 * Register:    Data Cache Zero ID register (dczid_el0)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Indicates the block size that is written with byte values of 0 by the
 * File:        AArch64-dczid_el0.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Data Cache Zero ID register value in memory
pub struct DczidEl0(u64);

/// struct implementation for accessing the fields of register dczid_el0
impl DczidEl0 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> DczidEl0 {
        Self::default()
    }

    /// collects the modifications of DczidEl0 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> DczidEl0 {
        DczidEl0(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> DczidEl0 {
        let curval = Self::reg_rawrd() & 0x1f;
        DczidEl0(curval)
    }

    /// reading the Data Cache Zero ID register (dczid_el0) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, DCZID_EL0
            asm!("mrs {}, dczid_el0", out(reg) regval);
        }
        return regval;
    }

    // register is not writable. not emitting write accessor

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x1f;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 31;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: dzp
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dzp_extract(&self) -> u64 {
        // bits 4..4
        self.0.get_bits(4..=4)
    }

    /// reads the current register value and extract field `dzp` from it
    pub fn dzp_read() -> u64 {
        Self::with_reg_val().dzp_extract()
    }
    // no insert() method for field dzp
    /*
     * Field: bs
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn bs_extract(&self) -> u64 {
        // bits 0..3
        self.0.get_bits(0..=3)
    }

    /// reads the current register value and extract field `bs` from it
    pub fn bs_read() -> u64 {
        Self::with_reg_val().bs_extract()
    }
    // no insert() method for field bs
}

impl Default for DczidEl0 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> DczidEl0 {
        DczidEl0(0)
    }
}
