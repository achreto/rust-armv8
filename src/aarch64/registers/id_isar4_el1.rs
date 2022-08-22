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
 * Generated on: 2022-08-22T16:35:53.066870
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
 * Register:    AArch32 Instruction Set Attribute Register 4 (id_isar4_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides information about the instruction sets implemented by the PE in AArch32 state.
 * File:        AArch64-id_isar4_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the AArch32 Instruction Set Attribute Register 4 value in memory
pub struct IdIsar4El1(u64);

/// struct implementation for accessing the fields of register id_isar4_el1
impl IdIsar4El1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IdIsar4El1 {
        Self::default()
    }

    /// collects the modifications of IdIsar4El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IdIsar4El1 {
        IdIsar4El1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  IdIsar4El1 {
        let curval = Self::reg_rawrd() & 0xffffffff;
        IdIsar4El1(curval)
    }


    
    /// reading the AArch32 Instruction Set Attribute Register 4 (id_isar4_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ID_ISAR4_EL1
            asm!("mrs {}, id_isar4_el1", out(reg) regval);
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
     * Field: swp_frac
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn swp_frac_extract(&self) -> u64 {
        // bits 28..31
        self.0.get_bits(28..=31)
    }

    /// reads the current register value and extract field `swp_frac` from it
    pub fn swp_frac_read() -> u64 {
        Self::with_reg_val().swp_frac_extract()
    }
// no insert() method for field swp_frac
    /*
     * Field: psr_m
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn psr_m_extract(&self) -> u64 {
        // bits 24..27
        self.0.get_bits(24..=27)
    }

    /// reads the current register value and extract field `psr_m` from it
    pub fn psr_m_read() -> u64 {
        Self::with_reg_val().psr_m_extract()
    }
// no insert() method for field psr_m
    /*
     * Field: synchprim_frac
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn synchprim_frac_extract(&self) -> u64 {
        // bits 20..23
        self.0.get_bits(20..=23)
    }

    /// reads the current register value and extract field `synchprim_frac` from it
    pub fn synchprim_frac_read() -> u64 {
        Self::with_reg_val().synchprim_frac_extract()
    }
// no insert() method for field synchprim_frac
    /*
     * Field: barrier
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn barrier_extract(&self) -> u64 {
        // bits 16..19
        self.0.get_bits(16..=19)
    }

    /// reads the current register value and extract field `barrier` from it
    pub fn barrier_read() -> u64 {
        Self::with_reg_val().barrier_extract()
    }
// no insert() method for field barrier
    /*
     * Field: smc
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn smc_extract(&self) -> u64 {
        // bits 12..15
        self.0.get_bits(12..=15)
    }

    /// reads the current register value and extract field `smc` from it
    pub fn smc_read() -> u64 {
        Self::with_reg_val().smc_extract()
    }
// no insert() method for field smc
    /*
     * Field: writeback
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn writeback_extract(&self) -> u64 {
        // bits 8..11
        self.0.get_bits(8..=11)
    }

    /// reads the current register value and extract field `writeback` from it
    pub fn writeback_read() -> u64 {
        Self::with_reg_val().writeback_extract()
    }
// no insert() method for field writeback
    /*
     * Field: withshifts
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn withshifts_extract(&self) -> u64 {
        // bits 4..7
        self.0.get_bits(4..=7)
    }

    /// reads the current register value and extract field `withshifts` from it
    pub fn withshifts_read() -> u64 {
        Self::with_reg_val().withshifts_extract()
    }
// no insert() method for field withshifts
    /*
     * Field: unpriv
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn unpriv_extract(&self) -> u64 {
        // bits 0..3
        self.0.get_bits(0..=3)
    }

    /// reads the current register value and extract field `unpriv` from it
    pub fn unpriv_read() -> u64 {
        Self::with_reg_val().unpriv_extract()
    }
// no insert() method for field unpriv
}

impl Default for IdIsar4El1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> IdIsar4El1 {
        IdIsar4El1(0)
    }
}
