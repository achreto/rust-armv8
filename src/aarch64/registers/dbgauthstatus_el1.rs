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
 * Generated on: 2022-08-22T16:25:59.072764
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
 * Register:    Debug Authentication Status register (dbgauthstatus_el1)
 * Group:       Debug registers
 * Type:        64-bit Register
 * Description: Provides information about the state of the 
 * File:        AArch64-dbgauthstatus_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Debug Authentication Status register value in memory
pub struct DbgauthstatusEl1(u64);

/// struct implementation for accessing the fields of register dbgauthstatus_el1
impl DbgauthstatusEl1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> DbgauthstatusEl1 {
        Self::default()
    }

    /// collects the modifications of DbgauthstatusEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> DbgauthstatusEl1 {
        DbgauthstatusEl1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  DbgauthstatusEl1 {
        let curval = Self::reg_rawrd() & 0xff;
        DbgauthstatusEl1(curval)
    }


    
    /// reading the Debug Authentication Status register (dbgauthstatus_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, DBGAUTHSTATUS_EL1
            asm!("mrs {}, dbgauthstatus_el1", out(reg) regval);
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
     * Field: snid_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn snid_1_extract(&self) -> u64 {
        // bits 6..7
        self.0.get_bits(6..=7)
    }

    /// reads the current register value and extract field `snid_1` from it
    pub fn snid_1_read(&mut self) -> u64 {
        Self::with_reg_val().snid_1_extract()
    }
// no insert() method for field snid_1
    /*
     * Field: snid_2
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn snid_2_extract(&self) -> u64 {
        // bits 6..7
        self.0.get_bits(6..=7)
    }

    /// reads the current register value and extract field `snid_2` from it
    pub fn snid_2_read(&mut self) -> u64 {
        Self::with_reg_val().snid_2_extract()
    }
// no insert() method for field snid_2
    /*
     * Field: sid
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn sid_extract(&self) -> u64 {
        // bits 4..5
        self.0.get_bits(4..=5)
    }

    /// reads the current register value and extract field `sid` from it
    pub fn sid_read(&mut self) -> u64 {
        Self::with_reg_val().sid_extract()
    }
// no insert() method for field sid
    /*
     * Field: nsnid_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn nsnid_1_extract(&self) -> u64 {
        // bits 2..3
        self.0.get_bits(2..=3)
    }

    /// reads the current register value and extract field `nsnid_1` from it
    pub fn nsnid_1_read(&mut self) -> u64 {
        Self::with_reg_val().nsnid_1_extract()
    }
// no insert() method for field nsnid_1
    /*
     * Field: nsnid_2
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn nsnid_2_extract(&self) -> u64 {
        // bits 2..3
        self.0.get_bits(2..=3)
    }

    /// reads the current register value and extract field `nsnid_2` from it
    pub fn nsnid_2_read(&mut self) -> u64 {
        Self::with_reg_val().nsnid_2_extract()
    }
// no insert() method for field nsnid_2
    /*
     * Field: nsid
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn nsid_extract(&self) -> u64 {
        // bits 0..1
        self.0.get_bits(0..=1)
    }

    /// reads the current register value and extract field `nsid` from it
    pub fn nsid_read(&mut self) -> u64 {
        Self::with_reg_val().nsid_extract()
    }
// no insert() method for field nsid
}

impl Default for DbgauthstatusEl1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> DbgauthstatusEl1 {
        DbgauthstatusEl1(0)
    }
}
