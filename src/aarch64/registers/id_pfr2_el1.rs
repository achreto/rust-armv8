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
 * Generated on: 2022-08-22T16:25:59.086738
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
 * Register:    AArch32 Processor Feature Register 2 (id_pfr2_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Gives information about the AArch32 programmers' model.
 * File:        AArch64-id_pfr2_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the AArch32 Processor Feature Register 2 value in memory
pub struct IdPfr2El1(u64);

/// struct implementation for accessing the fields of register id_pfr2_el1
impl IdPfr2El1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IdPfr2El1 {
        Self::default()
    }

    /// collects the modifications of IdPfr2El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IdPfr2El1 {
        IdPfr2El1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  IdPfr2El1 {
        let curval = Self::reg_rawrd() & 0xfff;
        IdPfr2El1(curval)
    }


    
    /// reading the AArch32 Processor Feature Register 2 (id_pfr2_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ID_PFR2_EL1
            asm!("mrs {}, S3_0_C0_C3_4", out(reg) regval);
        }
        return regval;
    }

// register is not writable. not emitting write accessor


    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xfff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 4095;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: ras_frac
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ras_frac_extract(&self) -> u64 {
        // bits 8..11
        self.0.get_bits(8..=11)
    }

    /// reads the current register value and extract field `ras_frac` from it
    pub fn ras_frac_read(&mut self) -> u64 {
        Self::with_reg_val().ras_frac_extract()
    }
// no insert() method for field ras_frac
    /*
     * Field: ssbs
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ssbs_extract(&self) -> u64 {
        // bits 4..7
        self.0.get_bits(4..=7)
    }

    /// reads the current register value and extract field `ssbs` from it
    pub fn ssbs_read(&mut self) -> u64 {
        Self::with_reg_val().ssbs_extract()
    }
// no insert() method for field ssbs
    /*
     * Field: csv3
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn csv3_extract(&self) -> u64 {
        // bits 0..3
        self.0.get_bits(0..=3)
    }

    /// reads the current register value and extract field `csv3` from it
    pub fn csv3_read(&mut self) -> u64 {
        Self::with_reg_val().csv3_extract()
    }
// no insert() method for field csv3
}

impl Default for IdPfr2El1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> IdPfr2El1 {
        IdPfr2El1(0)
    }
}
