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
 * Generated on: 2022-08-22T16:25:59.084187
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
 * Register:    AArch64 Instruction Set Attribute Register 2 (id_aa64isar2_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides information about the features and instructions implemented in AArch64 state.
 * File:        AArch64-id_aa64isar2_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the AArch64 Instruction Set Attribute Register 2 value in memory
pub struct IdAa64isar2El1(u64);

/// struct implementation for accessing the fields of register id_aa64isar2_el1
impl IdAa64isar2El1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IdAa64isar2El1 {
        Self::default()
    }

    /// collects the modifications of IdAa64isar2El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IdAa64isar2El1 {
        IdAa64isar2El1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  IdAa64isar2El1 {
        let curval = Self::reg_rawrd() & 0xff;
        IdAa64isar2El1(curval)
    }


    
    /// reading the AArch64 Instruction Set Attribute Register 2 (id_aa64isar2_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ID_AA64ISAR2_EL1
            asm!("mrs {}, S3_0_C0_C6_2", out(reg) regval);
        }
        return regval;
    }

// register is not writable. not emitting write accessor


    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 255;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: rpres
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn rpres_extract(&self) -> u64 {
        // bits 4..7
        self.0.get_bits(4..=7)
    }

    /// reads the current register value and extract field `rpres` from it
    pub fn rpres_read(&mut self) -> u64 {
        Self::with_reg_val().rpres_extract()
    }
// no insert() method for field rpres
    /*
     * Field: wfxt
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn wfxt_extract(&self) -> u64 {
        // bits 0..3
        self.0.get_bits(0..=3)
    }

    /// reads the current register value and extract field `wfxt` from it
    pub fn wfxt_read(&mut self) -> u64 {
        Self::with_reg_val().wfxt_extract()
    }
// no insert() method for field wfxt
}

impl Default for IdAa64isar2El1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> IdAa64isar2El1 {
        IdAa64isar2El1(0)
    }
}
