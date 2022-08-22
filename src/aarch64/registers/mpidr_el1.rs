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
 * Generated on: 2022-08-22T16:35:53.071320
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
 * Register:    Multiprocessor Affinity Register (mpidr_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: In a multiprocessor system, provides an additional PE identification mechanism for scheduling purposes.
 * File:        AArch64-mpidr_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Multiprocessor Affinity Register value in memory
pub struct MpidrEl1(u64);

/// struct implementation for accessing the fields of register mpidr_el1
impl MpidrEl1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> MpidrEl1 {
        Self::default()
    }

    /// collects the modifications of MpidrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> MpidrEl1 {
        MpidrEl1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  MpidrEl1 {
        let curval = Self::reg_rawrd() & 0xff41ffffff;
        MpidrEl1(curval)
    }


    
    /// reading the Multiprocessor Affinity Register (mpidr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, MPIDR_EL1
            asm!("mrs {}, mpidr_el1", out(reg) regval);
        }
        return regval;
    }

// register is not writable. not emitting write accessor


    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xff41ffffff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 1096323956735;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: aff3
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn aff3_extract(&self) -> u64 {
        // bits 32..39
        self.0.get_bits(32..=39)
    }

    /// reads the current register value and extract field `aff3` from it
    pub fn aff3_read() -> u64 {
        Self::with_reg_val().aff3_extract()
    }
// no insert() method for field aff3
    /*
     * Field: u
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn u_extract(&self) -> u64 {
        // bits 30..30
        self.0.get_bits(30..=30)
    }

    /// reads the current register value and extract field `u` from it
    pub fn u_read() -> u64 {
        Self::with_reg_val().u_extract()
    }
// no insert() method for field u
    /*
     * Field: mt
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn mt_extract(&self) -> u64 {
        // bits 24..24
        self.0.get_bits(24..=24)
    }

    /// reads the current register value and extract field `mt` from it
    pub fn mt_read() -> u64 {
        Self::with_reg_val().mt_extract()
    }
// no insert() method for field mt
    /*
     * Field: aff2
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn aff2_extract(&self) -> u64 {
        // bits 16..23
        self.0.get_bits(16..=23)
    }

    /// reads the current register value and extract field `aff2` from it
    pub fn aff2_read() -> u64 {
        Self::with_reg_val().aff2_extract()
    }
// no insert() method for field aff2
    /*
     * Field: aff1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn aff1_extract(&self) -> u64 {
        // bits 8..15
        self.0.get_bits(8..=15)
    }

    /// reads the current register value and extract field `aff1` from it
    pub fn aff1_read() -> u64 {
        Self::with_reg_val().aff1_extract()
    }
// no insert() method for field aff1
    /*
     * Field: aff0
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn aff0_extract(&self) -> u64 {
        // bits 0..7
        self.0.get_bits(0..=7)
    }

    /// reads the current register value and extract field `aff0` from it
    pub fn aff0_read() -> u64 {
        Self::with_reg_val().aff0_extract()
    }
// no insert() method for field aff0
}

impl Default for MpidrEl1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> MpidrEl1 {
        MpidrEl1(0)
    }
}
