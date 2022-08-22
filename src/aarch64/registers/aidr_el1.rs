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
 * Generated on: 2022-08-22T15:51:28.505507
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
 * Register:    Auxiliary ID Register (aidr_el1)
 * Group:       IMPLEMENTATION DEFINED
 * Type:        64-bit Register
 * Description: Provides
 * File:        AArch64-aidr_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Auxiliary ID Register value in memory
pub struct AidrEl1(u64);

/// struct implementation for accessing the fields of register aidr_el1
impl AidrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> AidrEl1 {
        Self::default()
    }

    /// collects the modifications of AidrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> AidrEl1 {
        AidrEl1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> AidrEl1 {
        let curval = Self::reg_rawrd() & 0xffffffffffffffff;
        AidrEl1(curval)
    }

    /// reading the Auxiliary ID Register (aidr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, AIDR_EL1
            llvm_asm!("mrs $0, aidr_el1" : "=r"(regval));
        }
        return regval;
    }

    // register is not writable. not emitting write accessor

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xffffffffffffffff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 18446744073709551615;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: implementation_defined_63_0
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn implementation_defined_63_0_extract(&self) -> u64 {
        // bits 0..63
        self.val.get_bits(0..=63)
    }

    /// reads the current register value and extract field `implementation_defined_63_0` from it
    pub fn implementation_defined_63_0_read(&mut self) -> u64 {
        Self::with_reg_val().implementation_defined_63_0_extract()
    }
    // no insert() method for field implementation_defined_63_0
}

impl Default for AidrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> AidrEl1 {
        AidrEl1(0)
    }
}
