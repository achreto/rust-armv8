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
use core::arch::asm;

/**************************************************************************************************
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 * Generated on: 2022-08-22T16:35:53.067871
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
 * Register:    AArch32 Processor Feature Register 1 (id_pfr1_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Gives information about the AArch32 programmers' model.
 * File:        AArch64-id_pfr1_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the AArch32 Processor Feature Register 1 value in memory
pub struct IdPfr1El1(u64);

/// struct implementation for accessing the fields of register id_pfr1_el1
impl IdPfr1El1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IdPfr1El1 {
        Self::default()
    }

    /// collects the modifications of IdPfr1El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IdPfr1El1 {
        IdPfr1El1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> IdPfr1El1 {
        let curval = Self::reg_rawrd() & 0xffffffff;
        IdPfr1El1(curval)
    }

    /// reading the AArch32 Processor Feature Register 1 (id_pfr1_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ID_PFR1_EL1
            asm!("mrs {}, id_pfr1_el1", out(reg) regval);
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
     * Field: gic
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn gic_extract(&self) -> u64 {
        // bits 28..31
        self.0.get_bits(28..=31)
    }

    /// reads the current register value and extract field `gic` from it
    pub fn gic_read() -> u64 {
        Self::with_reg_val().gic_extract()
    }
    // no insert() method for field gic
    /*
     * Field: virt_frac
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn virt_frac_extract(&self) -> u64 {
        // bits 24..27
        self.0.get_bits(24..=27)
    }

    /// reads the current register value and extract field `virt_frac` from it
    pub fn virt_frac_read() -> u64 {
        Self::with_reg_val().virt_frac_extract()
    }
    // no insert() method for field virt_frac
    /*
     * Field: sec_frac
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn sec_frac_extract(&self) -> u64 {
        // bits 20..23
        self.0.get_bits(20..=23)
    }

    /// reads the current register value and extract field `sec_frac` from it
    pub fn sec_frac_read() -> u64 {
        Self::with_reg_val().sec_frac_extract()
    }
    // no insert() method for field sec_frac
    /*
     * Field: gentimer
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn gentimer_extract(&self) -> u64 {
        // bits 16..19
        self.0.get_bits(16..=19)
    }

    /// reads the current register value and extract field `gentimer` from it
    pub fn gentimer_read() -> u64 {
        Self::with_reg_val().gentimer_extract()
    }
    // no insert() method for field gentimer
    /*
     * Field: virtualization
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn virtualization_extract(&self) -> u64 {
        // bits 12..15
        self.0.get_bits(12..=15)
    }

    /// reads the current register value and extract field `virtualization` from it
    pub fn virtualization_read() -> u64 {
        Self::with_reg_val().virtualization_extract()
    }
    // no insert() method for field virtualization
    /*
     * Field: mprogmod
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn mprogmod_extract(&self) -> u64 {
        // bits 8..11
        self.0.get_bits(8..=11)
    }

    /// reads the current register value and extract field `mprogmod` from it
    pub fn mprogmod_read() -> u64 {
        Self::with_reg_val().mprogmod_extract()
    }
    // no insert() method for field mprogmod
    /*
     * Field: security
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn security_extract(&self) -> u64 {
        // bits 4..7
        self.0.get_bits(4..=7)
    }

    /// reads the current register value and extract field `security` from it
    pub fn security_read() -> u64 {
        Self::with_reg_val().security_extract()
    }
    // no insert() method for field security
    /*
     * Field: progmod
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn progmod_extract(&self) -> u64 {
        // bits 0..3
        self.0.get_bits(0..=3)
    }

    /// reads the current register value and extract field `progmod` from it
    pub fn progmod_read() -> u64 {
        Self::with_reg_val().progmod_extract()
    }
    // no insert() method for field progmod
}

impl Default for IdPfr1El1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> IdPfr1El1 {
        IdPfr1El1(0)
    }
}
