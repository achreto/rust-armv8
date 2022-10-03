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
 * Generated on: 2022-08-22T16:35:53.066662
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
 * Register:    AArch32 Instruction Set Attribute Register 2 (id_isar2_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides information about the instruction sets implemented by the PE in AArch32 state.
 * File:        AArch64-id_isar2_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the AArch32 Instruction Set Attribute Register 2 value in memory
pub struct IdIsar2El1(u64);

/// struct implementation for accessing the fields of register id_isar2_el1
impl IdIsar2El1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IdIsar2El1 {
        Self::default()
    }

    /// collects the modifications of IdIsar2El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IdIsar2El1 {
        IdIsar2El1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> IdIsar2El1 {
        let curval = Self::reg_rawrd() & 0xffffffff;
        IdIsar2El1(curval)
    }

    /// reading the AArch32 Instruction Set Attribute Register 2 (id_isar2_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ID_ISAR2_EL1
            asm!("mrs {}, id_isar2_el1", out(reg) regval);
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
     * Field: reversal
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn reversal_extract(&self) -> u64 {
        // bits 28..31
        self.0.get_bits(28..=31)
    }

    /// reads the current register value and extract field `reversal` from it
    pub fn reversal_read() -> u64 {
        Self::with_reg_val().reversal_extract()
    }
    // no insert() method for field reversal
    /*
     * Field: psr_ar
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn psr_ar_extract(&self) -> u64 {
        // bits 24..27
        self.0.get_bits(24..=27)
    }

    /// reads the current register value and extract field `psr_ar` from it
    pub fn psr_ar_read() -> u64 {
        Self::with_reg_val().psr_ar_extract()
    }
    // no insert() method for field psr_ar
    /*
     * Field: multu
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn multu_extract(&self) -> u64 {
        // bits 20..23
        self.0.get_bits(20..=23)
    }

    /// reads the current register value and extract field `multu` from it
    pub fn multu_read() -> u64 {
        Self::with_reg_val().multu_extract()
    }
    // no insert() method for field multu
    /*
     * Field: mults
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn mults_extract(&self) -> u64 {
        // bits 16..19
        self.0.get_bits(16..=19)
    }

    /// reads the current register value and extract field `mults` from it
    pub fn mults_read() -> u64 {
        Self::with_reg_val().mults_extract()
    }
    // no insert() method for field mults
    /*
     * Field: mult
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn mult_extract(&self) -> u64 {
        // bits 12..15
        self.0.get_bits(12..=15)
    }

    /// reads the current register value and extract field `mult` from it
    pub fn mult_read() -> u64 {
        Self::with_reg_val().mult_extract()
    }
    // no insert() method for field mult
    /*
     * Field: multiaccessint
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn multiaccessint_extract(&self) -> u64 {
        // bits 8..11
        self.0.get_bits(8..=11)
    }

    /// reads the current register value and extract field `multiaccessint` from it
    pub fn multiaccessint_read() -> u64 {
        Self::with_reg_val().multiaccessint_extract()
    }
    // no insert() method for field multiaccessint
    /*
     * Field: memhint
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn memhint_extract(&self) -> u64 {
        // bits 4..7
        self.0.get_bits(4..=7)
    }

    /// reads the current register value and extract field `memhint` from it
    pub fn memhint_read() -> u64 {
        Self::with_reg_val().memhint_extract()
    }
    // no insert() method for field memhint
    /*
     * Field: loadstore
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn loadstore_extract(&self) -> u64 {
        // bits 0..3
        self.0.get_bits(0..=3)
    }

    /// reads the current register value and extract field `loadstore` from it
    pub fn loadstore_read() -> u64 {
        Self::with_reg_val().loadstore_extract()
    }
    // no insert() method for field loadstore
}

impl Default for IdIsar2El1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> IdIsar2El1 {
        IdIsar2El1(0)
    }
}
