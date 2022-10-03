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
 * Generated on: 2022-08-22T16:35:53.066557
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
 * Register:    AArch32 Instruction Set Attribute Register 1 (id_isar1_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides information about the instruction sets implemented by the PE in AArch32 state.
 * File:        AArch64-id_isar1_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the AArch32 Instruction Set Attribute Register 1 value in memory
pub struct IdIsar1El1(u64);

/// struct implementation for accessing the fields of register id_isar1_el1
impl IdIsar1El1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IdIsar1El1 {
        Self::default()
    }

    /// collects the modifications of IdIsar1El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IdIsar1El1 {
        IdIsar1El1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> IdIsar1El1 {
        let curval = Self::reg_rawrd() & 0xffffffff;
        IdIsar1El1(curval)
    }

    /// reading the AArch32 Instruction Set Attribute Register 1 (id_isar1_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ID_ISAR1_EL1
            asm!("mrs {}, id_isar1_el1", out(reg) regval);
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
     * Field: jazelle
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn jazelle_extract(&self) -> u64 {
        // bits 28..31
        self.0.get_bits(28..=31)
    }

    /// reads the current register value and extract field `jazelle` from it
    pub fn jazelle_read() -> u64 {
        Self::with_reg_val().jazelle_extract()
    }
    // no insert() method for field jazelle
    /*
     * Field: interwork
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn interwork_extract(&self) -> u64 {
        // bits 24..27
        self.0.get_bits(24..=27)
    }

    /// reads the current register value and extract field `interwork` from it
    pub fn interwork_read() -> u64 {
        Self::with_reg_val().interwork_extract()
    }
    // no insert() method for field interwork
    /*
     * Field: immediate
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn immediate_extract(&self) -> u64 {
        // bits 20..23
        self.0.get_bits(20..=23)
    }

    /// reads the current register value and extract field `immediate` from it
    pub fn immediate_read() -> u64 {
        Self::with_reg_val().immediate_extract()
    }
    // no insert() method for field immediate
    /*
     * Field: ifthen
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ifthen_extract(&self) -> u64 {
        // bits 16..19
        self.0.get_bits(16..=19)
    }

    /// reads the current register value and extract field `ifthen` from it
    pub fn ifthen_read() -> u64 {
        Self::with_reg_val().ifthen_extract()
    }
    // no insert() method for field ifthen
    /*
     * Field: extend
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn extend_extract(&self) -> u64 {
        // bits 12..15
        self.0.get_bits(12..=15)
    }

    /// reads the current register value and extract field `extend` from it
    pub fn extend_read() -> u64 {
        Self::with_reg_val().extend_extract()
    }
    // no insert() method for field extend
    /*
     * Field: except_ar
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn except_ar_extract(&self) -> u64 {
        // bits 8..11
        self.0.get_bits(8..=11)
    }

    /// reads the current register value and extract field `except_ar` from it
    pub fn except_ar_read() -> u64 {
        Self::with_reg_val().except_ar_extract()
    }
    // no insert() method for field except_ar
    /*
     * Field: except
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn except_extract(&self) -> u64 {
        // bits 4..7
        self.0.get_bits(4..=7)
    }

    /// reads the current register value and extract field `except` from it
    pub fn except_read() -> u64 {
        Self::with_reg_val().except_extract()
    }
    // no insert() method for field except
    /*
     * Field: endian
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn endian_extract(&self) -> u64 {
        // bits 0..3
        self.0.get_bits(0..=3)
    }

    /// reads the current register value and extract field `endian` from it
    pub fn endian_read() -> u64 {
        Self::with_reg_val().endian_extract()
    }
    // no insert() method for field endian
}

impl Default for IdIsar1El1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> IdIsar1El1 {
        IdIsar1El1(0)
    }
}
