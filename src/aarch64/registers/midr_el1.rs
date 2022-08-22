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
 * Generated on: 2022-08-22T15:51:28.526599
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
 * Register:    Main ID Register (midr_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides identification information for the PE, including an implementer code for the device and a device ID number.
 * File:        AArch64-midr_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Main ID Register value in memory
pub struct MidrEl1(u64);

/// struct implementation for accessing the fields of register midr_el1
impl MidrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> MidrEl1 {
        Self::default()
    }

    /// collects the modifications of MidrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> MidrEl1 {
        MidrEl1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> MidrEl1 {
        let curval = Self::reg_rawrd() & 0xffffffff;
        MidrEl1(curval)
    }

    /// reading the Main ID Register (midr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, MIDR_EL1
            llvm_asm!("mrs $0, midr_el1" : "=r"(regval));
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
     * Field: implementer
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn implementer_extract(&self) -> u64 {
        // bits 24..31
        self.val.get_bits(24..=31)
    }

    /// reads the current register value and extract field `implementer` from it
    pub fn implementer_read(&mut self) -> u64 {
        Self::with_reg_val().implementer_extract()
    }
    // no insert() method for field implementer
    /*
     * Field: variant
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn variant_extract(&self) -> u64 {
        // bits 20..23
        self.val.get_bits(20..=23)
    }

    /// reads the current register value and extract field `variant` from it
    pub fn variant_read(&mut self) -> u64 {
        Self::with_reg_val().variant_extract()
    }
    // no insert() method for field variant
    /*
     * Field: architecture
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn architecture_extract(&self) -> u64 {
        // bits 16..19
        self.val.get_bits(16..=19)
    }

    /// reads the current register value and extract field `architecture` from it
    pub fn architecture_read(&mut self) -> u64 {
        Self::with_reg_val().architecture_extract()
    }
    // no insert() method for field architecture
    /*
     * Field: partnum
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn partnum_extract(&self) -> u64 {
        // bits 4..15
        self.val.get_bits(4..=15)
    }

    /// reads the current register value and extract field `partnum` from it
    pub fn partnum_read(&mut self) -> u64 {
        Self::with_reg_val().partnum_extract()
    }
    // no insert() method for field partnum
    /*
     * Field: revision
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn revision_extract(&self) -> u64 {
        // bits 0..3
        self.val.get_bits(0..=3)
    }

    /// reads the current register value and extract field `revision` from it
    pub fn revision_read(&mut self) -> u64 {
        Self::with_reg_val().revision_extract()
    }
    // no insert() method for field revision
}

impl Default for MidrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> MidrEl1 {
        MidrEl1(0)
    }
}
