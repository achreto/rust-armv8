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

use bit_field::BitField;

/**************************************************************************************************
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 * Generated on: 2022-08-22T15:51:28.529124
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
 * Register:    OS Lock Status Register (oslsr_el1)
 * Group:       Debug registers
 * Type:        64-bit Register
 * Description: Provides the status of the OS Lock.
 * File:        AArch64-oslsr_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the OS Lock Status Register value in memory
pub struct OslsrEl1(u64);

/// struct implementation for accessing the fields of register oslsr_el1
impl OslsrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> OslsrEl1 {
        Self::default()
    }

    /// collects the modifications of OslsrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> OslsrEl1 {
        OslsrEl1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> OslsrEl1 {
        let curval = Self::reg_rawrd() & 0xf;
        OslsrEl1(curval)
    }

    /// reading the OS Lock Status Register (oslsr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, OSLSR_EL1
            llvm_asm!("mrs $0, oslsr_el1" : "=r"(regval));
        }
        return regval;
    }

    // register is not writable. not emitting write accessor

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xf;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 15;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: oslm
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn oslm_extract(&self) -> u64 {
        // bits 3..3
        self.val.get_bits(3..=3)
    }

    /// reads the current register value and extract field `oslm` from it
    pub fn oslm_read(&mut self) -> u64 {
        Self::with_reg_val().oslm_extract()
    }
    // no insert() method for field oslm
    /*
     * Field: ntt
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ntt_extract(&self) -> u64 {
        // bits 2..2
        self.val.get_bits(2..=2)
    }

    /// reads the current register value and extract field `ntt` from it
    pub fn ntt_read(&mut self) -> u64 {
        Self::with_reg_val().ntt_extract()
    }
    // no insert() method for field ntt
    /*
     * Field: oslk
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn oslk_extract(&self) -> u64 {
        // bits 1..1
        self.val.get_bits(1..=1)
    }

    /// reads the current register value and extract field `oslk` from it
    pub fn oslk_read(&mut self) -> u64 {
        Self::with_reg_val().oslk_extract()
    }
    // no insert() method for field oslk
    /*
     * Field: oslm_0_0
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn oslm_0_0_extract(&self) -> u64 {
        // bits 0..0
        self.val.get_bits(0..=0)
    }

    /// reads the current register value and extract field `oslm_0_0` from it
    pub fn oslm_0_0_read(&mut self) -> u64 {
        Self::with_reg_val().oslm_0_0_extract()
    }
    // no insert() method for field oslm_0_0
}

impl Default for OslsrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> OslsrEl1 {
        OslsrEl1(0)
    }
}
