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
 * Generated on: 2022-08-22T16:35:53.066986
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
 * Register:    AArch32 Instruction Set Attribute Register 5 (id_isar5_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides information about the instruction sets implemented by the PE in AArch32 state.
 * File:        AArch64-id_isar5_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the AArch32 Instruction Set Attribute Register 5 value in memory
pub struct IdIsar5El1(u64);

/// struct implementation for accessing the fields of register id_isar5_el1
impl IdIsar5El1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IdIsar5El1 {
        Self::default()
    }

    /// collects the modifications of IdIsar5El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IdIsar5El1 {
        IdIsar5El1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  IdIsar5El1 {
        let curval = Self::reg_rawrd() & 0xff0fffff;
        IdIsar5El1(curval)
    }


    
    /// reading the AArch32 Instruction Set Attribute Register 5 (id_isar5_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ID_ISAR5_EL1
            asm!("mrs {}, id_isar5_el1", out(reg) regval);
        }
        return regval;
    }

// register is not writable. not emitting write accessor


    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xff0fffff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 4279238655;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: vcma
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn vcma_extract(&self) -> u64 {
        // bits 28..31
        self.0.get_bits(28..=31)
    }

    /// reads the current register value and extract field `vcma` from it
    pub fn vcma_read() -> u64 {
        Self::with_reg_val().vcma_extract()
    }
// no insert() method for field vcma
    /*
     * Field: rdm
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn rdm_extract(&self) -> u64 {
        // bits 24..27
        self.0.get_bits(24..=27)
    }

    /// reads the current register value and extract field `rdm` from it
    pub fn rdm_read() -> u64 {
        Self::with_reg_val().rdm_extract()
    }
// no insert() method for field rdm
    /*
     * Field: crc32
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn crc32_extract(&self) -> u64 {
        // bits 16..19
        self.0.get_bits(16..=19)
    }

    /// reads the current register value and extract field `crc32` from it
    pub fn crc32_read() -> u64 {
        Self::with_reg_val().crc32_extract()
    }
// no insert() method for field crc32
    /*
     * Field: sha2
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn sha2_extract(&self) -> u64 {
        // bits 12..15
        self.0.get_bits(12..=15)
    }

    /// reads the current register value and extract field `sha2` from it
    pub fn sha2_read() -> u64 {
        Self::with_reg_val().sha2_extract()
    }
// no insert() method for field sha2
    /*
     * Field: sha1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn sha1_extract(&self) -> u64 {
        // bits 8..11
        self.0.get_bits(8..=11)
    }

    /// reads the current register value and extract field `sha1` from it
    pub fn sha1_read() -> u64 {
        Self::with_reg_val().sha1_extract()
    }
// no insert() method for field sha1
    /*
     * Field: aes
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn aes_extract(&self) -> u64 {
        // bits 4..7
        self.0.get_bits(4..=7)
    }

    /// reads the current register value and extract field `aes` from it
    pub fn aes_read() -> u64 {
        Self::with_reg_val().aes_extract()
    }
// no insert() method for field aes
    /*
     * Field: sevl
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn sevl_extract(&self) -> u64 {
        // bits 0..3
        self.0.get_bits(0..=3)
    }

    /// reads the current register value and extract field `sevl` from it
    pub fn sevl_read() -> u64 {
        Self::with_reg_val().sevl_extract()
    }
// no insert() method for field sevl
}

impl Default for IdIsar5El1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> IdIsar5El1 {
        IdIsar5El1(0)
    }
}
