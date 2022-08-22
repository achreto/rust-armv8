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

/**************************************************************************************************
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 * Generated on: 2022-08-22T15:51:28.529515
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
 * Register:    Profiling Buffer ID Register (pmbidr_el1)
 * Group:       Statistical Profiling Extension registers
 * Type:        64-bit Register
 * Description: Provides information to software as to whether the buffer can be programmed at the current Exception level.
 * File:        AArch64-pmbidr_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Profiling Buffer ID Register value in memory
pub struct PmbidrEl1(u64);

/// struct implementation for accessing the fields of register pmbidr_el1
impl PmbidrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> PmbidrEl1 {
        Self::default()
    }

    /// collects the modifications of PmbidrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> PmbidrEl1 {
        PmbidrEl1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> PmbidrEl1 {
        let curval = Self::reg_rawrd() & 0x3f;
        PmbidrEl1(curval)
    }

    /// reading the Profiling Buffer ID Register (pmbidr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, PMBIDR_EL1
            llvm_asm!("mrs $0, S3_0_C9_C10_7" : "=r"(regval));
        }
        return regval;
    }

    // register is not writable. not emitting write accessor

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0x3f;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 63;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: f
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn f_extract(&self) -> u64 {
        // bits 5..5
        self.val.get_bits(5..=5)
    }

    /// reads the current register value and extract field `f` from it
    pub fn f_read(&mut self) -> u64 {
        Self::with_reg_val().f_extract()
    }
    // no insert() method for field f
    /*
     * Field: p
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn p_extract(&self) -> u64 {
        // bits 4..4
        self.val.get_bits(4..=4)
    }

    /// reads the current register value and extract field `p` from it
    pub fn p_read(&mut self) -> u64 {
        Self::with_reg_val().p_extract()
    }
    // no insert() method for field p
    /*
     * Field: align
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn align_extract(&self) -> u64 {
        // bits 0..3
        self.val.get_bits(0..=3)
    }

    /// reads the current register value and extract field `align` from it
    pub fn align_read(&mut self) -> u64 {
        Self::with_reg_val().align_extract()
    }
    // no insert() method for field align
}

impl Default for PmbidrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> PmbidrEl1 {
        PmbidrEl1(0)
    }
}
