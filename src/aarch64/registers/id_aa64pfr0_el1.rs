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
 * Generated on: 2022-08-22T16:35:53.065879
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
 * Register:    AArch64 Processor Feature Register 0 (id_aa64pfr0_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides additional information about implemented PE features in AArch64 state.
 * File:        AArch64-id_aa64pfr0_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the AArch64 Processor Feature Register 0 value in memory
pub struct IdAa64pfr0El1(u64);

/// struct implementation for accessing the fields of register id_aa64pfr0_el1
impl IdAa64pfr0El1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IdAa64pfr0El1 {
        Self::default()
    }

    /// collects the modifications of IdAa64pfr0El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IdAa64pfr0El1 {
        IdAa64pfr0El1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> IdAa64pfr0El1 {
        let curval = Self::reg_rawrd() & 0xff0fffffffffffff;
        IdAa64pfr0El1(curval)
    }

    /// reading the AArch64 Processor Feature Register 0 (id_aa64pfr0_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ID_AA64PFR0_EL1
            asm!("mrs {}, id_aa64pfr0_el1", out(reg) regval);
        }
        return regval;
    }

    // register is not writable. not emitting write accessor

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xff0fffffffffffff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 18379190079298994175;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: csv3
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn csv3_extract(&self) -> u64 {
        // bits 60..63
        self.0.get_bits(60..=63)
    }

    /// reads the current register value and extract field `csv3` from it
    pub fn csv3_read() -> u64 {
        Self::with_reg_val().csv3_extract()
    }
    // no insert() method for field csv3
    /*
     * Field: csv2
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn csv2_extract(&self) -> u64 {
        // bits 56..59
        self.0.get_bits(56..=59)
    }

    /// reads the current register value and extract field `csv2` from it
    pub fn csv2_read() -> u64 {
        Self::with_reg_val().csv2_extract()
    }
    // no insert() method for field csv2
    /*
     * Field: dit
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dit_extract(&self) -> u64 {
        // bits 48..51
        self.0.get_bits(48..=51)
    }

    /// reads the current register value and extract field `dit` from it
    pub fn dit_read() -> u64 {
        Self::with_reg_val().dit_extract()
    }
    // no insert() method for field dit
    /*
     * Field: amu
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amu_extract(&self) -> u64 {
        // bits 44..47
        self.0.get_bits(44..=47)
    }

    /// reads the current register value and extract field `amu` from it
    pub fn amu_read() -> u64 {
        Self::with_reg_val().amu_extract()
    }
    // no insert() method for field amu
    /*
     * Field: mpam
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn mpam_extract(&self) -> u64 {
        // bits 40..43
        self.0.get_bits(40..=43)
    }

    /// reads the current register value and extract field `mpam` from it
    pub fn mpam_read() -> u64 {
        Self::with_reg_val().mpam_extract()
    }
    // no insert() method for field mpam
    /*
     * Field: sel2
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn sel2_extract(&self) -> u64 {
        // bits 36..39
        self.0.get_bits(36..=39)
    }

    /// reads the current register value and extract field `sel2` from it
    pub fn sel2_read() -> u64 {
        Self::with_reg_val().sel2_extract()
    }
    // no insert() method for field sel2
    /*
     * Field: sve
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn sve_extract(&self) -> u64 {
        // bits 32..35
        self.0.get_bits(32..=35)
    }

    /// reads the current register value and extract field `sve` from it
    pub fn sve_read() -> u64 {
        Self::with_reg_val().sve_extract()
    }
    // no insert() method for field sve
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
    pub fn ras_read() -> u64 {
        Self::with_reg_val().ras_extract()
    }
    // no insert() method for field ras
    /*
     * Field: gic
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn gic_extract(&self) -> u64 {
        // bits 24..27
        self.0.get_bits(24..=27)
    }

    /// reads the current register value and extract field `gic` from it
    pub fn gic_read() -> u64 {
        Self::with_reg_val().gic_extract()
    }
    // no insert() method for field gic
    /*
     * Field: advsimd
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn advsimd_extract(&self) -> u64 {
        // bits 20..23
        self.0.get_bits(20..=23)
    }

    /// reads the current register value and extract field `advsimd` from it
    pub fn advsimd_read() -> u64 {
        Self::with_reg_val().advsimd_extract()
    }
    // no insert() method for field advsimd
    /*
     * Field: fp
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fp_extract(&self) -> u64 {
        // bits 16..19
        self.0.get_bits(16..=19)
    }

    /// reads the current register value and extract field `fp` from it
    pub fn fp_read() -> u64 {
        Self::with_reg_val().fp_extract()
    }
    // no insert() method for field fp
    /*
     * Field: el3
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn el3_extract(&self) -> u64 {
        // bits 12..15
        self.0.get_bits(12..=15)
    }

    /// reads the current register value and extract field `el3` from it
    pub fn el3_read() -> u64 {
        Self::with_reg_val().el3_extract()
    }
    // no insert() method for field el3
    /*
     * Field: el2
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn el2_extract(&self) -> u64 {
        // bits 8..11
        self.0.get_bits(8..=11)
    }

    /// reads the current register value and extract field `el2` from it
    pub fn el2_read() -> u64 {
        Self::with_reg_val().el2_extract()
    }
    // no insert() method for field el2
    /*
     * Field: el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn el1_extract(&self) -> u64 {
        // bits 4..7
        self.0.get_bits(4..=7)
    }

    /// reads the current register value and extract field `el1` from it
    pub fn el1_read() -> u64 {
        Self::with_reg_val().el1_extract()
    }
    // no insert() method for field el1
    /*
     * Field: el0
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn el0_extract(&self) -> u64 {
        // bits 0..3
        self.0.get_bits(0..=3)
    }

    /// reads the current register value and extract field `el0` from it
    pub fn el0_read() -> u64 {
        Self::with_reg_val().el0_extract()
    }
    // no insert() method for field el0
}

impl Default for IdAa64pfr0El1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> IdAa64pfr0El1 {
        IdAa64pfr0El1(0)
    }
}
