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
 * Generated on: 2022-08-22T16:35:53.065193
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
 * Register:    AArch64 Instruction Set Attribute Register 0 (id_aa64isar0_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides information about the instructions implemented in AArch64 state.
 * File:        AArch64-id_aa64isar0_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the AArch64 Instruction Set Attribute Register 0 value in memory
pub struct IdAa64isar0El1(u64);

/// struct implementation for accessing the fields of register id_aa64isar0_el1
impl IdAa64isar0El1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IdAa64isar0El1 {
        Self::default()
    }

    /// collects the modifications of IdAa64isar0El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IdAa64isar0El1 {
        IdAa64isar0El1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  IdAa64isar0El1 {
        let curval = Self::reg_rawrd() & 0xfffffffff0fffff0;
        IdAa64isar0El1(curval)
    }


    
    /// reading the AArch64 Instruction Set Attribute Register 0 (id_aa64isar0_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ID_AA64ISAR0_EL1
            asm!("mrs {}, id_aa64isar0_el1", out(reg) regval);
        }
        return regval;
    }

// register is not writable. not emitting write accessor


    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xfffffffff0fffff0;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 18446744073457893360;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: rndr
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn rndr_extract(&self) -> u64 {
        // bits 60..63
        self.0.get_bits(60..=63)
    }

    /// reads the current register value and extract field `rndr` from it
    pub fn rndr_read() -> u64 {
        Self::with_reg_val().rndr_extract()
    }
// no insert() method for field rndr
    /*
     * Field: tlb
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tlb_extract(&self) -> u64 {
        // bits 56..59
        self.0.get_bits(56..=59)
    }

    /// reads the current register value and extract field `tlb` from it
    pub fn tlb_read() -> u64 {
        Self::with_reg_val().tlb_extract()
    }
// no insert() method for field tlb
    /*
     * Field: ts
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ts_extract(&self) -> u64 {
        // bits 52..55
        self.0.get_bits(52..=55)
    }

    /// reads the current register value and extract field `ts` from it
    pub fn ts_read() -> u64 {
        Self::with_reg_val().ts_extract()
    }
// no insert() method for field ts
    /*
     * Field: fhm
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn fhm_extract(&self) -> u64 {
        // bits 48..51
        self.0.get_bits(48..=51)
    }

    /// reads the current register value and extract field `fhm` from it
    pub fn fhm_read() -> u64 {
        Self::with_reg_val().fhm_extract()
    }
// no insert() method for field fhm
    /*
     * Field: dp
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn dp_extract(&self) -> u64 {
        // bits 44..47
        self.0.get_bits(44..=47)
    }

    /// reads the current register value and extract field `dp` from it
    pub fn dp_read() -> u64 {
        Self::with_reg_val().dp_extract()
    }
// no insert() method for field dp
    /*
     * Field: sm4
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn sm4_extract(&self) -> u64 {
        // bits 40..43
        self.0.get_bits(40..=43)
    }

    /// reads the current register value and extract field `sm4` from it
    pub fn sm4_read() -> u64 {
        Self::with_reg_val().sm4_extract()
    }
// no insert() method for field sm4
    /*
     * Field: sm3
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn sm3_extract(&self) -> u64 {
        // bits 36..39
        self.0.get_bits(36..=39)
    }

    /// reads the current register value and extract field `sm3` from it
    pub fn sm3_read() -> u64 {
        Self::with_reg_val().sm3_extract()
    }
// no insert() method for field sm3
    /*
     * Field: sha3
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn sha3_extract(&self) -> u64 {
        // bits 32..35
        self.0.get_bits(32..=35)
    }

    /// reads the current register value and extract field `sha3` from it
    pub fn sha3_read() -> u64 {
        Self::with_reg_val().sha3_extract()
    }
// no insert() method for field sha3
    /*
     * Field: rdm
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn rdm_extract(&self) -> u64 {
        // bits 28..31
        self.0.get_bits(28..=31)
    }

    /// reads the current register value and extract field `rdm` from it
    pub fn rdm_read() -> u64 {
        Self::with_reg_val().rdm_extract()
    }
// no insert() method for field rdm
    /*
     * Field: atomic
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn atomic_extract(&self) -> u64 {
        // bits 20..23
        self.0.get_bits(20..=23)
    }

    /// reads the current register value and extract field `atomic` from it
    pub fn atomic_read() -> u64 {
        Self::with_reg_val().atomic_extract()
    }
// no insert() method for field atomic
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
}

impl Default for IdAa64isar0El1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> IdAa64isar0El1 {
        IdAa64isar0El1(0)
    }
}
