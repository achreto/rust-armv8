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
 * Generated on: 2022-08-22T16:35:53.071411
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
 * Register:    AArch32 Media and VFP Feature Register 0 (mvfr0_el1)
 * Group:       Floating-point registers
 * Type:        64-bit Register
 * Description: Describes the features provided by the AArch32 Advanced SIMD and Floating-point implementation.
 * File:        AArch64-mvfr0_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the AArch32 Media and VFP Feature Register 0 value in memory
pub struct Mvfr0El1(u64);

/// struct implementation for accessing the fields of register mvfr0_el1
impl Mvfr0El1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> Mvfr0El1 {
        Self::default()
    }

    /// collects the modifications of Mvfr0El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> Mvfr0El1 {
        Mvfr0El1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> Mvfr0El1 {
        let curval = Self::reg_rawrd() & 0xffffffff;
        Mvfr0El1(curval)
    }

    /// reading the AArch32 Media and VFP Feature Register 0 (mvfr0_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, MVFR0_EL1
            asm!("mrs {}, mvfr0_el1", out(reg) regval);
        }
        return regval;
    }

    // register is not writable. not emitting write accessor

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xffffffff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 4294967295;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: fpround
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fpround_extract(&self) -> u64 {
        // bits 28..31
        self.0.get_bits(28..=31)
    }

    /// reads the current register value and extract field `fpround` from it
    pub fn fpround_read() -> u64 {
        Self::with_reg_val().fpround_extract()
    }
    // no insert() method for field fpround
    /*
     * Field: fpshvec
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fpshvec_extract(&self) -> u64 {
        // bits 24..27
        self.0.get_bits(24..=27)
    }

    /// reads the current register value and extract field `fpshvec` from it
    pub fn fpshvec_read() -> u64 {
        Self::with_reg_val().fpshvec_extract()
    }
    // no insert() method for field fpshvec
    /*
     * Field: fpsqrt
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fpsqrt_extract(&self) -> u64 {
        // bits 20..23
        self.0.get_bits(20..=23)
    }

    /// reads the current register value and extract field `fpsqrt` from it
    pub fn fpsqrt_read() -> u64 {
        Self::with_reg_val().fpsqrt_extract()
    }
    // no insert() method for field fpsqrt
    /*
     * Field: fpdivide
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fpdivide_extract(&self) -> u64 {
        // bits 16..19
        self.0.get_bits(16..=19)
    }

    /// reads the current register value and extract field `fpdivide` from it
    pub fn fpdivide_read() -> u64 {
        Self::with_reg_val().fpdivide_extract()
    }
    // no insert() method for field fpdivide
    /*
     * Field: fptrap
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fptrap_extract(&self) -> u64 {
        // bits 12..15
        self.0.get_bits(12..=15)
    }

    /// reads the current register value and extract field `fptrap` from it
    pub fn fptrap_read() -> u64 {
        Self::with_reg_val().fptrap_extract()
    }
    // no insert() method for field fptrap
    /*
     * Field: fpdp
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fpdp_extract(&self) -> u64 {
        // bits 8..11
        self.0.get_bits(8..=11)
    }

    /// reads the current register value and extract field `fpdp` from it
    pub fn fpdp_read() -> u64 {
        Self::with_reg_val().fpdp_extract()
    }
    // no insert() method for field fpdp
    /*
     * Field: fpsp
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fpsp_extract(&self) -> u64 {
        // bits 4..7
        self.0.get_bits(4..=7)
    }

    /// reads the current register value and extract field `fpsp` from it
    pub fn fpsp_read() -> u64 {
        Self::with_reg_val().fpsp_extract()
    }
    // no insert() method for field fpsp
    /*
     * Field: simdreg
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn simdreg_extract(&self) -> u64 {
        // bits 0..3
        self.0.get_bits(0..=3)
    }

    /// reads the current register value and extract field `simdreg` from it
    pub fn simdreg_read() -> u64 {
        Self::with_reg_val().simdreg_extract()
    }
    // no insert() method for field simdreg
}

impl Default for Mvfr0El1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> Mvfr0El1 {
        Mvfr0El1(0)
    }
}
