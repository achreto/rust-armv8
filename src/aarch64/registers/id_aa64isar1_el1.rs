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
 * Generated on: 2022-08-22T16:25:59.084062
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
 * Register:    AArch64 Instruction Set Attribute Register 1 (id_aa64isar1_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides information about the features and instructions implemented in AArch64 state.
 * File:        AArch64-id_aa64isar1_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the AArch64 Instruction Set Attribute Register 1 value in memory
pub struct IdAa64isar1El1(u64);

/// struct implementation for accessing the fields of register id_aa64isar1_el1
impl IdAa64isar1El1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IdAa64isar1El1 {
        Self::default()
    }

    /// collects the modifications of IdAa64isar1El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IdAa64isar1El1 {
        IdAa64isar1El1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  IdAa64isar1El1 {
        let curval = Self::reg_rawrd() & 0xffffffffffffffff;
        IdAa64isar1El1(curval)
    }


    
    /// reading the AArch64 Instruction Set Attribute Register 1 (id_aa64isar1_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ID_AA64ISAR1_EL1
            asm!("mrs {}, id_aa64isar1_el1", out(reg) regval);
        }
        return regval;
    }

// register is not writable. not emitting write accessor


    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xffffffffffffffff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 18446744073709551615;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: ls64
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ls64_extract(&self) -> u64 {
        // bits 60..63
        self.0.get_bits(60..=63)
    }

    /// reads the current register value and extract field `ls64` from it
    pub fn ls64_read(&mut self) -> u64 {
        Self::with_reg_val().ls64_extract()
    }
// no insert() method for field ls64
    /*
     * Field: xs
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn xs_extract(&self) -> u64 {
        // bits 56..59
        self.0.get_bits(56..=59)
    }

    /// reads the current register value and extract field `xs` from it
    pub fn xs_read(&mut self) -> u64 {
        Self::with_reg_val().xs_extract()
    }
// no insert() method for field xs
    /*
     * Field: i8mm
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn i8mm_extract(&self) -> u64 {
        // bits 52..55
        self.0.get_bits(52..=55)
    }

    /// reads the current register value and extract field `i8mm` from it
    pub fn i8mm_read(&mut self) -> u64 {
        Self::with_reg_val().i8mm_extract()
    }
// no insert() method for field i8mm
    /*
     * Field: dgh
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn dgh_extract(&self) -> u64 {
        // bits 48..51
        self.0.get_bits(48..=51)
    }

    /// reads the current register value and extract field `dgh` from it
    pub fn dgh_read(&mut self) -> u64 {
        Self::with_reg_val().dgh_extract()
    }
// no insert() method for field dgh
    /*
     * Field: bf16
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn bf16_extract(&self) -> u64 {
        // bits 44..47
        self.0.get_bits(44..=47)
    }

    /// reads the current register value and extract field `bf16` from it
    pub fn bf16_read(&mut self) -> u64 {
        Self::with_reg_val().bf16_extract()
    }
// no insert() method for field bf16
    /*
     * Field: specres
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn specres_extract(&self) -> u64 {
        // bits 40..43
        self.0.get_bits(40..=43)
    }

    /// reads the current register value and extract field `specres` from it
    pub fn specres_read(&mut self) -> u64 {
        Self::with_reg_val().specres_extract()
    }
// no insert() method for field specres
    /*
     * Field: sb
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn sb_extract(&self) -> u64 {
        // bits 36..39
        self.0.get_bits(36..=39)
    }

    /// reads the current register value and extract field `sb` from it
    pub fn sb_read(&mut self) -> u64 {
        Self::with_reg_val().sb_extract()
    }
// no insert() method for field sb
    /*
     * Field: frintts
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn frintts_extract(&self) -> u64 {
        // bits 32..35
        self.0.get_bits(32..=35)
    }

    /// reads the current register value and extract field `frintts` from it
    pub fn frintts_read(&mut self) -> u64 {
        Self::with_reg_val().frintts_extract()
    }
// no insert() method for field frintts
    /*
     * Field: gpi
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn gpi_extract(&self) -> u64 {
        // bits 28..31
        self.0.get_bits(28..=31)
    }

    /// reads the current register value and extract field `gpi` from it
    pub fn gpi_read(&mut self) -> u64 {
        Self::with_reg_val().gpi_extract()
    }
// no insert() method for field gpi
    /*
     * Field: gpa
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn gpa_extract(&self) -> u64 {
        // bits 24..27
        self.0.get_bits(24..=27)
    }

    /// reads the current register value and extract field `gpa` from it
    pub fn gpa_read(&mut self) -> u64 {
        Self::with_reg_val().gpa_extract()
    }
// no insert() method for field gpa
    /*
     * Field: lrcpc
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn lrcpc_extract(&self) -> u64 {
        // bits 20..23
        self.0.get_bits(20..=23)
    }

    /// reads the current register value and extract field `lrcpc` from it
    pub fn lrcpc_read(&mut self) -> u64 {
        Self::with_reg_val().lrcpc_extract()
    }
// no insert() method for field lrcpc
    /*
     * Field: fcma
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn fcma_extract(&self) -> u64 {
        // bits 16..19
        self.0.get_bits(16..=19)
    }

    /// reads the current register value and extract field `fcma` from it
    pub fn fcma_read(&mut self) -> u64 {
        Self::with_reg_val().fcma_extract()
    }
// no insert() method for field fcma
    /*
     * Field: jscvt
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn jscvt_extract(&self) -> u64 {
        // bits 12..15
        self.0.get_bits(12..=15)
    }

    /// reads the current register value and extract field `jscvt` from it
    pub fn jscvt_read(&mut self) -> u64 {
        Self::with_reg_val().jscvt_extract()
    }
// no insert() method for field jscvt
    /*
     * Field: api
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn api_extract(&self) -> u64 {
        // bits 8..11
        self.0.get_bits(8..=11)
    }

    /// reads the current register value and extract field `api` from it
    pub fn api_read(&mut self) -> u64 {
        Self::with_reg_val().api_extract()
    }
// no insert() method for field api
    /*
     * Field: apa
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn apa_extract(&self) -> u64 {
        // bits 4..7
        self.0.get_bits(4..=7)
    }

    /// reads the current register value and extract field `apa` from it
    pub fn apa_read(&mut self) -> u64 {
        Self::with_reg_val().apa_extract()
    }
// no insert() method for field apa
    /*
     * Field: dpb
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn dpb_extract(&self) -> u64 {
        // bits 0..3
        self.0.get_bits(0..=3)
    }

    /// reads the current register value and extract field `dpb` from it
    pub fn dpb_read(&mut self) -> u64 {
        Self::with_reg_val().dpb_extract()
    }
// no insert() method for field dpb
}

impl Default for IdAa64isar1El1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> IdAa64isar1El1 {
        IdAa64isar1El1(0)
    }
}
