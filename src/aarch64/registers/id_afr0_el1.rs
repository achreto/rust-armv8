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
 * Generated on: 2022-08-22T16:35:53.066182
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
 * Register:    AArch32 Auxiliary Feature Register 0 (id_afr0_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides information about the
 * File:        AArch64-id_afr0_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the AArch32 Auxiliary Feature Register 0 value in memory
pub struct IdAfr0El1(u64);

/// struct implementation for accessing the fields of register id_afr0_el1
impl IdAfr0El1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IdAfr0El1 {
        Self::default()
    }

    /// collects the modifications of IdAfr0El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IdAfr0El1 {
        IdAfr0El1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> IdAfr0El1 {
        let curval = Self::reg_rawrd() & 0xffff;
        IdAfr0El1(curval)
    }

    /// reading the AArch32 Auxiliary Feature Register 0 (id_afr0_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ID_AFR0_EL1
            asm!("mrs {}, id_afr0_el1", out(reg) regval);
        }
        return regval;
    }

    // register is not writable. not emitting write accessor

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xffff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 65535;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: implementation_defined_15_12
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn implementation_defined_15_12_extract(&self) -> u64 {
        // bits 12..15
        self.0.get_bits(12..=15)
    }

    /// reads the current register value and extract field `implementation_defined_15_12` from it
    pub fn implementation_defined_15_12_read() -> u64 {
        Self::with_reg_val().implementation_defined_15_12_extract()
    }
    // no insert() method for field implementation_defined_15_12
    /*
     * Field: implementation_defined_11_8
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn implementation_defined_11_8_extract(&self) -> u64 {
        // bits 8..11
        self.0.get_bits(8..=11)
    }

    /// reads the current register value and extract field `implementation_defined_11_8` from it
    pub fn implementation_defined_11_8_read() -> u64 {
        Self::with_reg_val().implementation_defined_11_8_extract()
    }
    // no insert() method for field implementation_defined_11_8
    /*
     * Field: implementation_defined_7_4
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn implementation_defined_7_4_extract(&self) -> u64 {
        // bits 4..7
        self.0.get_bits(4..=7)
    }

    /// reads the current register value and extract field `implementation_defined_7_4` from it
    pub fn implementation_defined_7_4_read() -> u64 {
        Self::with_reg_val().implementation_defined_7_4_extract()
    }
    // no insert() method for field implementation_defined_7_4
    /*
     * Field: implementation_defined_3_0
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn implementation_defined_3_0_extract(&self) -> u64 {
        // bits 0..3
        self.0.get_bits(0..=3)
    }

    /// reads the current register value and extract field `implementation_defined_3_0` from it
    pub fn implementation_defined_3_0_read() -> u64 {
        Self::with_reg_val().implementation_defined_3_0_extract()
    }
    // no insert() method for field implementation_defined_3_0
}

impl Default for IdAfr0El1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> IdAfr0El1 {
        IdAfr0El1(0)
    }
}
