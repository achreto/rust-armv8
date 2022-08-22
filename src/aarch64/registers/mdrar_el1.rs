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
 * Generated on: 2022-08-22T15:51:28.526363
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
 * Register:    Monitor Debug ROM Address Register (mdrar_el1)
 * Group:       Debug registers
 * Type:        64-bit Register
 * Description: Defines the base physical address of a 4KB-aligned memory-mapped debug component, usually a ROM table that locates and describes the memory-mapped debug components in the system. Armv8 deprecates any use of this register.
 * File:        AArch64-mdrar_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Monitor Debug ROM Address Register value in memory
pub struct MdrarEl1(u64);

/// struct implementation for accessing the fields of register mdrar_el1
impl MdrarEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> MdrarEl1 {
        Self::default()
    }

    /// collects the modifications of MdrarEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> MdrarEl1 {
        MdrarEl1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> MdrarEl1 {
        let curval = Self::reg_rawrd() & 0xffffffffff003;
        MdrarEl1(curval)
    }

    /// reading the Monitor Debug ROM Address Register (mdrar_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, MDRAR_EL1
            llvm_asm!("mrs $0, mdrar_el1" : "=r"(regval));
        }
        return regval;
    }

    // register is not writable. not emitting write accessor

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xffffffffff003;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 4503599627366403;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: romaddr
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn romaddr_extract(&self) -> u64 {
        // bits 12..51
        self.val.get_bits(12..=51)
    }

    /// reads the current register value and extract field `romaddr` from it
    pub fn romaddr_read(&mut self) -> u64 {
        Self::with_reg_val().romaddr_extract()
    }
    // no insert() method for field romaddr
    /*
     * Field: valid
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn valid_extract(&self) -> u64 {
        // bits 0..1
        self.val.get_bits(0..=1)
    }

    /// reads the current register value and extract field `valid` from it
    pub fn valid_read(&mut self) -> u64 {
        Self::with_reg_val().valid_extract()
    }
    // no insert() method for field valid
}

impl Default for MdrarEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> MdrarEl1 {
        MdrarEl1(0)
    }
}
