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
 * Generated on: 2022-08-22T15:51:28.528439
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
 * Register:    AArch32 Media and VFP Feature Register 1 (mvfr1_el1)
 * Group:       Floating-point registers
 * Type:        64-bit Register
 * Description: Describes the features provided by the AArch32 Advanced SIMD and Floating-point implementation.
 * File:        AArch64-mvfr1_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the AArch32 Media and VFP Feature Register 1 value in memory
pub struct Mvfr1El1(u64);

/// struct implementation for accessing the fields of register mvfr1_el1
impl Mvfr1El1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> Mvfr1El1 {
        Self::default()
    }

    /// collects the modifications of Mvfr1El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> Mvfr1El1 {
        Mvfr1El1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> Mvfr1El1 {
        let curval = Self::reg_rawrd() & 0xffffffff;
        Mvfr1El1(curval)
    }

    /// reading the AArch32 Media and VFP Feature Register 1 (mvfr1_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, MVFR1_EL1
            llvm_asm!("mrs $0, mvfr1_el1" : "=r"(regval));
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
     * Field: simdfmac
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn simdfmac_extract(&self) -> u64 {
        // bits 28..31
        self.val.get_bits(28..=31)
    }

    /// reads the current register value and extract field `simdfmac` from it
    pub fn simdfmac_read(&mut self) -> u64 {
        Self::with_reg_val().simdfmac_extract()
    }
    // no insert() method for field simdfmac
    /*
     * Field: fphp
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fphp_extract(&self) -> u64 {
        // bits 24..27
        self.val.get_bits(24..=27)
    }

    /// reads the current register value and extract field `fphp` from it
    pub fn fphp_read(&mut self) -> u64 {
        Self::with_reg_val().fphp_extract()
    }
    // no insert() method for field fphp
    /*
     * Field: simdhp
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn simdhp_extract(&self) -> u64 {
        // bits 20..23
        self.val.get_bits(20..=23)
    }

    /// reads the current register value and extract field `simdhp` from it
    pub fn simdhp_read(&mut self) -> u64 {
        Self::with_reg_val().simdhp_extract()
    }
    // no insert() method for field simdhp
    /*
     * Field: simdsp
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn simdsp_extract(&self) -> u64 {
        // bits 16..19
        self.val.get_bits(16..=19)
    }

    /// reads the current register value and extract field `simdsp` from it
    pub fn simdsp_read(&mut self) -> u64 {
        Self::with_reg_val().simdsp_extract()
    }
    // no insert() method for field simdsp
    /*
     * Field: simdint
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn simdint_extract(&self) -> u64 {
        // bits 12..15
        self.val.get_bits(12..=15)
    }

    /// reads the current register value and extract field `simdint` from it
    pub fn simdint_read(&mut self) -> u64 {
        Self::with_reg_val().simdint_extract()
    }
    // no insert() method for field simdint
    /*
     * Field: simdls
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn simdls_extract(&self) -> u64 {
        // bits 8..11
        self.val.get_bits(8..=11)
    }

    /// reads the current register value and extract field `simdls` from it
    pub fn simdls_read(&mut self) -> u64 {
        Self::with_reg_val().simdls_extract()
    }
    // no insert() method for field simdls
    /*
     * Field: fpdnan
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fpdnan_extract(&self) -> u64 {
        // bits 4..7
        self.val.get_bits(4..=7)
    }

    /// reads the current register value and extract field `fpdnan` from it
    pub fn fpdnan_read(&mut self) -> u64 {
        Self::with_reg_val().fpdnan_extract()
    }
    // no insert() method for field fpdnan
    /*
     * Field: fpftz
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fpftz_extract(&self) -> u64 {
        // bits 0..3
        self.val.get_bits(0..=3)
    }

    /// reads the current register value and extract field `fpftz` from it
    pub fn fpftz_read(&mut self) -> u64 {
        Self::with_reg_val().fpftz_extract()
    }
    // no insert() method for field fpftz
}

impl Default for Mvfr1El1 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> Mvfr1El1 {
        Mvfr1El1(0)
    }
}
