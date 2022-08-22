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
 * Generated on: 2022-08-22T15:51:28.507432
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
 * Register:    Cache Level ID Register (clidr_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Identifies the type of cache, or caches, that are implemented at each level and can be managed using the architected cache maintenance instructions that operate by set/way, up to a maximum of seven levels. Also identifies the Level of Coherence (LoC) and Level of Unification (LoU) for the cache hierarchy.
 * File:        AArch64-clidr_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Cache Level ID Register value in memory
pub struct ClidrEl1(u64);

/// struct implementation for accessing the fields of register clidr_el1
impl ClidrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> ClidrEl1 {
        Self::default()
    }

    /// collects the modifications of ClidrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> ClidrEl1 {
        ClidrEl1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> ClidrEl1 {
        let curval = Self::reg_rawrd() & 0x7fffffffffff;
        ClidrEl1(curval)
    }

    /// reading the Cache Level ID Register (clidr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, CLIDR_EL1
            llvm_asm!("mrs $0, clidr_el1" : "=r"(regval));
        }
        return regval;
    }

    // register is not writable. not emitting write accessor

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0x7fffffffffff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 140737488355327;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: ttypen_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ttypen_1_extract(&self) -> u64 {
        // bits 33..46
        self.val.get_bits(33..=46)
    }

    /// reads the current register value and extract field `ttypen_1` from it
    pub fn ttypen_1_read(&mut self) -> u64 {
        Self::with_reg_val().ttypen_1_extract()
    }
    // no insert() method for field ttypen_1
    /*
     * Field: icb
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn icb_extract(&self) -> u64 {
        // bits 30..32
        self.val.get_bits(30..=32)
    }

    /// reads the current register value and extract field `icb` from it
    pub fn icb_read(&mut self) -> u64 {
        Self::with_reg_val().icb_extract()
    }
    // no insert() method for field icb
    /*
     * Field: louu
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn louu_extract(&self) -> u64 {
        // bits 27..29
        self.val.get_bits(27..=29)
    }

    /// reads the current register value and extract field `louu` from it
    pub fn louu_read(&mut self) -> u64 {
        Self::with_reg_val().louu_extract()
    }
    // no insert() method for field louu
    /*
     * Field: loc
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn loc_extract(&self) -> u64 {
        // bits 24..26
        self.val.get_bits(24..=26)
    }

    /// reads the current register value and extract field `loc` from it
    pub fn loc_read(&mut self) -> u64 {
        Self::with_reg_val().loc_extract()
    }
    // no insert() method for field loc
    /*
     * Field: louis
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn louis_extract(&self) -> u64 {
        // bits 21..23
        self.val.get_bits(21..=23)
    }

    /// reads the current register value and extract field `louis` from it
    pub fn louis_read(&mut self) -> u64 {
        Self::with_reg_val().louis_extract()
    }
    // no insert() method for field louis
    /*
     * Field: ctypen
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ctypen_extract(&self) -> u64 {
        // bits 0..20
        self.val.get_bits(0..=20)
    }

    /// reads the current register value and extract field `ctypen` from it
    pub fn ctypen_read(&mut self) -> u64 {
        Self::with_reg_val().ctypen_extract()
    }
    // no insert() method for field ctypen
}

impl Default for ClidrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> ClidrEl1 {
        ClidrEl1(0)
    }
}
