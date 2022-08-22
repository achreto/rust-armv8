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

use core::arch::asm;
use bit_field::BitField;


/**************************************************************************************************
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 * Generated on: 2022-08-22T16:25:59.085240
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
 * Register:    AArch32 Instruction Set Attribute Register 0 (id_isar0_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides information about the instruction sets implemented by the PE in AArch32 state.
 * File:        AArch64-id_isar0_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the AArch32 Instruction Set Attribute Register 0 value in memory
pub struct IdIsar0El1(u64);

/// struct implementation for accessing the fields of register id_isar0_el1
impl IdIsar0El1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IdIsar0El1 {
        Self::default()
    }

    /// collects the modifications of IdIsar0El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IdIsar0El1 {
        IdIsar0El1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  IdIsar0El1 {
        let curval = Self::reg_rawrd() & 0xfffffff;
        IdIsar0El1(curval)
    }


    
    /// reading the AArch32 Instruction Set Attribute Register 0 (id_isar0_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ID_ISAR0_EL1
            asm!("mrs {}, id_isar0_el1", out(reg) regval);
        }
        return regval;
    }

// register is not writable. not emitting write accessor


    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xfffffff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 268435455;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: divide
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn divide_extract(&self) -> u64 {
        // bits 24..27
        self.0.get_bits(24..=27)
    }

    /// reads the current register value and extract field `divide` from it
    pub fn divide_read(&mut self) -> u64 {
        Self::with_reg_val().divide_extract()
    }
// no insert() method for field divide
    /*
     * Field: debug
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn debug_extract(&self) -> u64 {
        // bits 20..23
        self.0.get_bits(20..=23)
    }

    /// reads the current register value and extract field `debug` from it
    pub fn debug_read(&mut self) -> u64 {
        Self::with_reg_val().debug_extract()
    }
// no insert() method for field debug
    /*
     * Field: coproc
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn coproc_extract(&self) -> u64 {
        // bits 16..19
        self.0.get_bits(16..=19)
    }

    /// reads the current register value and extract field `coproc` from it
    pub fn coproc_read(&mut self) -> u64 {
        Self::with_reg_val().coproc_extract()
    }
// no insert() method for field coproc
    /*
     * Field: cmpbranch
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn cmpbranch_extract(&self) -> u64 {
        // bits 12..15
        self.0.get_bits(12..=15)
    }

    /// reads the current register value and extract field `cmpbranch` from it
    pub fn cmpbranch_read(&mut self) -> u64 {
        Self::with_reg_val().cmpbranch_extract()
    }
// no insert() method for field cmpbranch
    /*
     * Field: bitfield
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn bitfield_extract(&self) -> u64 {
        // bits 8..11
        self.0.get_bits(8..=11)
    }

    /// reads the current register value and extract field `bitfield` from it
    pub fn bitfield_read(&mut self) -> u64 {
        Self::with_reg_val().bitfield_extract()
    }
// no insert() method for field bitfield
    /*
     * Field: bitcount
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn bitcount_extract(&self) -> u64 {
        // bits 4..7
        self.0.get_bits(4..=7)
    }

    /// reads the current register value and extract field `bitcount` from it
    pub fn bitcount_read(&mut self) -> u64 {
        Self::with_reg_val().bitcount_extract()
    }
// no insert() method for field bitcount
    /*
     * Field: swap
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn swap_extract(&self) -> u64 {
        // bits 0..3
        self.0.get_bits(0..=3)
    }

    /// reads the current register value and extract field `swap` from it
    pub fn swap_read(&mut self) -> u64 {
        Self::with_reg_val().swap_extract()
    }
// no insert() method for field swap
}

impl Default for IdIsar0El1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> IdIsar0El1 {
        IdIsar0El1(0)
    }
}
