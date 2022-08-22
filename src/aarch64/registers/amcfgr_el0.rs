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
 * Generated on: 2022-08-22T16:25:59.067633
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
 * Register:    Activity Monitors Configuration Register (amcfgr_el0)
 * Group:       Activity Monitors registers
 * Type:        64-bit Register
 * Description: Global configuration register for the activity monitors.
 * File:        AArch64-amcfgr_el0.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Activity Monitors Configuration Register value in memory
pub struct AmcfgrEl0(u64);

/// struct implementation for accessing the fields of register amcfgr_el0
impl AmcfgrEl0 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> AmcfgrEl0 {
        Self::default()
    }

    /// collects the modifications of AmcfgrEl0 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> AmcfgrEl0 {
        AmcfgrEl0(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  AmcfgrEl0 {
        let curval = Self::reg_rawrd() & 0xf1003fff;
        AmcfgrEl0(curval)
    }


    
    /// reading the Activity Monitors Configuration Register (amcfgr_el0) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, AMCFGR_EL0
            asm!("mrs {}, S3_3_C13_C2_1", out(reg) regval);
        }
        return regval;
    }

// register is not writable. not emitting write accessor


    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xf1003fff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 4043325439;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: ncg
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ncg_extract(&self) -> u64 {
        // bits 28..31
        self.0.get_bits(28..=31)
    }

    /// reads the current register value and extract field `ncg` from it
    pub fn ncg_read(&mut self) -> u64 {
        Self::with_reg_val().ncg_extract()
    }
// no insert() method for field ncg
    /*
     * Field: hdbg
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn hdbg_extract(&self) -> u64 {
        // bits 24..24
        self.0.get_bits(24..=24)
    }

    /// reads the current register value and extract field `hdbg` from it
    pub fn hdbg_read(&mut self) -> u64 {
        Self::with_reg_val().hdbg_extract()
    }
// no insert() method for field hdbg
    /*
     * Field: size
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn size_extract(&self) -> u64 {
        // bits 8..13
        self.0.get_bits(8..=13)
    }

    /// reads the current register value and extract field `size` from it
    pub fn size_read(&mut self) -> u64 {
        Self::with_reg_val().size_extract()
    }
// no insert() method for field size
    /*
     * Field: n
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn n_extract(&self) -> u64 {
        // bits 0..7
        self.0.get_bits(0..=7)
    }

    /// reads the current register value and extract field `n` from it
    pub fn n_read(&mut self) -> u64 {
        Self::with_reg_val().n_extract()
    }
// no insert() method for field n
}

impl Default for AmcfgrEl0 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> AmcfgrEl0 {
        AmcfgrEl0(0)
    }
}
