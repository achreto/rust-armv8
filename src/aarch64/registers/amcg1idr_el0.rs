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
 * Generated on: 2022-08-22T15:51:28.505909
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
 * Register:    Activity Monitors Counter Group 1 Identification Register (amcg1idr_el0)
 * Group:       Activity Monitors registers
 * Type:        64-bit Register
 * Description: Defines which auxiliary counters are implemented, and which of them have a corresponding virtual offset register,
 * File:        AArch64-amcg1idr_el0.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Activity Monitors Counter Group 1 Identification Register value in memory
pub struct Amcg1idrEl0(u64);

/// struct implementation for accessing the fields of register amcg1idr_el0
impl Amcg1idrEl0 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> Amcg1idrEl0 {
        Self::default()
    }

    /// collects the modifications of Amcg1idrEl0 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> Amcg1idrEl0 {
        Amcg1idrEl0(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> Amcg1idrEl0 {
        let curval = Self::reg_rawrd() & 0xffffffff;
        Amcg1idrEl0(curval)
    }

    /// reading the Activity Monitors Counter Group 1 Identification Register (amcg1idr_el0) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, AMCG1IDR_EL0
            llvm_asm!("mrs $0, S3_3_C13_C2_6" : "=r"(regval));
        }
        return regval;
    }

    // register is not writable. not emitting write accessor

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xffffffff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 4294967295;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: amevcntoff1n_el2
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevcntoff1n_el2_extract(&self) -> u64 {
        // bits 16..31
        self.val.get_bits(16..=31)
    }

    /// reads the current register value and extract field `amevcntoff1n_el2` from it
    pub fn amevcntoff1n_el2_read(&mut self) -> u64 {
        Self::with_reg_val().amevcntoff1n_el2_extract()
    }
    // no insert() method for field amevcntoff1n_el2
    /*
     * Field: amevcntr1n_el0
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevcntr1n_el0_extract(&self) -> u64 {
        // bits 0..15
        self.val.get_bits(0..=15)
    }

    /// reads the current register value and extract field `amevcntr1n_el0` from it
    pub fn amevcntr1n_el0_read(&mut self) -> u64 {
        Self::with_reg_val().amevcntr1n_el0_extract()
    }
    // no insert() method for field amevcntr1n_el0
}

impl Default for Amcg1idrEl0 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> Amcg1idrEl0 {
        Amcg1idrEl0(0)
    }
}
