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
 * Generated on: 2022-08-22T16:25:59.086541
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
 * Register:    AArch32 Processor Feature Register 0 (id_pfr0_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Gives top-level information about the instruction sets supported by the PE in AArch32 state.
 * File:        AArch64-id_pfr0_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the AArch32 Processor Feature Register 0 value in memory
pub struct IdPfr0El1(u64);

/// struct implementation for accessing the fields of register id_pfr0_el1
impl IdPfr0El1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IdPfr0El1 {
        Self::default()
    }

    /// collects the modifications of IdPfr0El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IdPfr0El1 {
        IdPfr0El1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  IdPfr0El1 {
        let curval = Self::reg_rawrd() & 0xffffffff;
        IdPfr0El1(curval)
    }


    
    /// reading the AArch32 Processor Feature Register 0 (id_pfr0_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ID_PFR0_EL1
            asm!("mrs {}, id_pfr0_el1", out(reg) regval);
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
     * Field: ras
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ras_extract(&self) -> u64 {
        // bits 28..31
        self.0.get_bits(28..=31)
    }

    /// reads the current register value and extract field `ras` from it
    pub fn ras_read(&mut self) -> u64 {
        Self::with_reg_val().ras_extract()
    }
// no insert() method for field ras
    /*
     * Field: dit
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn dit_extract(&self) -> u64 {
        // bits 24..27
        self.0.get_bits(24..=27)
    }

    /// reads the current register value and extract field `dit` from it
    pub fn dit_read(&mut self) -> u64 {
        Self::with_reg_val().dit_extract()
    }
// no insert() method for field dit
    /*
     * Field: amu
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn amu_extract(&self) -> u64 {
        // bits 20..23
        self.0.get_bits(20..=23)
    }

    /// reads the current register value and extract field `amu` from it
    pub fn amu_read(&mut self) -> u64 {
        Self::with_reg_val().amu_extract()
    }
// no insert() method for field amu
    /*
     * Field: csv2
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn csv2_extract(&self) -> u64 {
        // bits 16..19
        self.0.get_bits(16..=19)
    }

    /// reads the current register value and extract field `csv2` from it
    pub fn csv2_read(&mut self) -> u64 {
        Self::with_reg_val().csv2_extract()
    }
// no insert() method for field csv2
    /*
     * Field: state3
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn state3_extract(&self) -> u64 {
        // bits 12..15
        self.0.get_bits(12..=15)
    }

    /// reads the current register value and extract field `state3` from it
    pub fn state3_read(&mut self) -> u64 {
        Self::with_reg_val().state3_extract()
    }
// no insert() method for field state3
    /*
     * Field: state2
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn state2_extract(&self) -> u64 {
        // bits 8..11
        self.0.get_bits(8..=11)
    }

    /// reads the current register value and extract field `state2` from it
    pub fn state2_read(&mut self) -> u64 {
        Self::with_reg_val().state2_extract()
    }
// no insert() method for field state2
    /*
     * Field: state1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn state1_extract(&self) -> u64 {
        // bits 4..7
        self.0.get_bits(4..=7)
    }

    /// reads the current register value and extract field `state1` from it
    pub fn state1_read(&mut self) -> u64 {
        Self::with_reg_val().state1_extract()
    }
// no insert() method for field state1
    /*
     * Field: state0
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn state0_extract(&self) -> u64 {
        // bits 0..3
        self.0.get_bits(0..=3)
    }

    /// reads the current register value and extract field `state0` from it
    pub fn state0_read(&mut self) -> u64 {
        Self::with_reg_val().state0_extract()
    }
// no insert() method for field state0
}

impl Default for IdPfr0El1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> IdPfr0El1 {
        IdPfr0El1(0)
    }
}
