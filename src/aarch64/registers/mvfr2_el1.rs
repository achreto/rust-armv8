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
 * Generated on: 2022-08-22T16:35:53.071600
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
 * Register:    AArch32 Media and VFP Feature Register 2 (mvfr2_el1)
 * Group:       Floating-point registers
 * Type:        64-bit Register
 * Description: Describes the features provided by the AArch32 Advanced SIMD and Floating-point implementation.
 * File:        AArch64-mvfr2_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the AArch32 Media and VFP Feature Register 2 value in memory
pub struct Mvfr2El1(u64);

/// struct implementation for accessing the fields of register mvfr2_el1
impl Mvfr2El1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> Mvfr2El1 {
        Self::default()
    }

    /// collects the modifications of Mvfr2El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> Mvfr2El1 {
        Mvfr2El1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> Mvfr2El1 {
        let curval = Self::reg_rawrd() & 0xff;
        Mvfr2El1(curval)
    }

    /// reading the AArch32 Media and VFP Feature Register 2 (mvfr2_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, MVFR2_EL1
            asm!("mrs {}, mvfr2_el1", out(reg) regval);
        }
        return regval;
    }

    // register is not writable. not emitting write accessor

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 255;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: fpmisc
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fpmisc_extract(&self) -> u64 {
        // bits 4..7
        self.0.get_bits(4..=7)
    }

    /// reads the current register value and extract field `fpmisc` from it
    pub fn fpmisc_read() -> u64 {
        Self::with_reg_val().fpmisc_extract()
    }
    // no insert() method for field fpmisc
    /*
     * Field: simdmisc
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn simdmisc_extract(&self) -> u64 {
        // bits 0..3
        self.0.get_bits(0..=3)
    }

    /// reads the current register value and extract field `simdmisc` from it
    pub fn simdmisc_read() -> u64 {
        Self::with_reg_val().simdmisc_extract()
    }
    // no insert() method for field simdmisc
}

impl Default for Mvfr2El1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> Mvfr2El1 {
        Mvfr2El1(0)
    }
}
