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
 * Generated on: 2022-08-22T16:25:59.087739
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
 * Register:    Monitor DCC Status Register (mdccsr_el0)
 * Group:       Debug registers
 * Type:        64-bit Register
 * Description: Read-only register containing control status flags for the DCC.
 * File:        AArch64-mdccsr_el0.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Monitor DCC Status Register value in memory
pub struct MdccsrEl0(u64);

/// struct implementation for accessing the fields of register mdccsr_el0
impl MdccsrEl0 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> MdccsrEl0 {
        Self::default()
    }

    /// collects the modifications of MdccsrEl0 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> MdccsrEl0 {
        MdccsrEl0(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  MdccsrEl0 {
        let curval = Self::reg_rawrd() & 0x60000000;
        MdccsrEl0(curval)
    }


    
    /// reading the Monitor DCC Status Register (mdccsr_el0) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, MDCCSR_EL0
            asm!("mrs {}, mdccsr_el0", out(reg) regval);
        }
        return regval;
    }

// register is not writable. not emitting write accessor


    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x60000000;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 1610612736;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: rxfull
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn rxfull_extract(&self) -> u64 {
        // bits 30..30
        self.0.get_bits(30..=30)
    }

    /// reads the current register value and extract field `rxfull` from it
    pub fn rxfull_read(&mut self) -> u64 {
        Self::with_reg_val().rxfull_extract()
    }
// no insert() method for field rxfull
    /*
     * Field: txfull
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn txfull_extract(&self) -> u64 {
        // bits 29..29
        self.0.get_bits(29..=29)
    }

    /// reads the current register value and extract field `txfull` from it
    pub fn txfull_read(&mut self) -> u64 {
        Self::with_reg_val().txfull_extract()
    }
// no insert() method for field txfull
}

impl Default for MdccsrEl0 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> MdccsrEl0 {
        MdccsrEl0(0)
    }
}
