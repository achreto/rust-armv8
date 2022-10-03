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
 * Generated on: 2022-08-22T16:35:53.065642
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
 * Register:    AArch64 Memory Model Feature Register 1 (id_aa64mmfr1_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides information about the implemented memory model and memory management support in AArch64 state.
 * File:        AArch64-id_aa64mmfr1_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the AArch64 Memory Model Feature Register 1 value in memory
pub struct IdAa64mmfr1El1(u64);

/// struct implementation for accessing the fields of register id_aa64mmfr1_el1
impl IdAa64mmfr1El1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IdAa64mmfr1El1 {
        Self::default()
    }

    /// collects the modifications of IdAa64mmfr1El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IdAa64mmfr1El1 {
        IdAa64mmfr1El1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> IdAa64mmfr1El1 {
        let curval = Self::reg_rawrd() & 0xffffffffffff;
        IdAa64mmfr1El1(curval)
    }

    /// reading the AArch64 Memory Model Feature Register 1 (id_aa64mmfr1_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ID_AA64MMFR1_EL1
            asm!("mrs {}, id_aa64mmfr1_el1", out(reg) regval);
        }
        return regval;
    }

    // register is not writable. not emitting write accessor

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xffffffffffff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 281474976710655;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: afp
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn afp_extract(&self) -> u64 {
        // bits 44..47
        self.0.get_bits(44..=47)
    }

    /// reads the current register value and extract field `afp` from it
    pub fn afp_read() -> u64 {
        Self::with_reg_val().afp_extract()
    }
    // no insert() method for field afp
    /*
     * Field: hcx
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn hcx_extract(&self) -> u64 {
        // bits 40..43
        self.0.get_bits(40..=43)
    }

    /// reads the current register value and extract field `hcx` from it
    pub fn hcx_read() -> u64 {
        Self::with_reg_val().hcx_extract()
    }
    // no insert() method for field hcx
    /*
     * Field: ets
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ets_extract(&self) -> u64 {
        // bits 36..39
        self.0.get_bits(36..=39)
    }

    /// reads the current register value and extract field `ets` from it
    pub fn ets_read() -> u64 {
        Self::with_reg_val().ets_extract()
    }
    // no insert() method for field ets
    /*
     * Field: twed
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn twed_extract(&self) -> u64 {
        // bits 32..35
        self.0.get_bits(32..=35)
    }

    /// reads the current register value and extract field `twed` from it
    pub fn twed_read() -> u64 {
        Self::with_reg_val().twed_extract()
    }
    // no insert() method for field twed
    /*
     * Field: xnx
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn xnx_extract(&self) -> u64 {
        // bits 28..31
        self.0.get_bits(28..=31)
    }

    /// reads the current register value and extract field `xnx` from it
    pub fn xnx_read() -> u64 {
        Self::with_reg_val().xnx_extract()
    }
    // no insert() method for field xnx
    /*
     * Field: specsei
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn specsei_extract(&self) -> u64 {
        // bits 24..27
        self.0.get_bits(24..=27)
    }

    /// reads the current register value and extract field `specsei` from it
    pub fn specsei_read() -> u64 {
        Self::with_reg_val().specsei_extract()
    }
    // no insert() method for field specsei
    /*
     * Field: pan
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pan_extract(&self) -> u64 {
        // bits 20..23
        self.0.get_bits(20..=23)
    }

    /// reads the current register value and extract field `pan` from it
    pub fn pan_read() -> u64 {
        Self::with_reg_val().pan_extract()
    }
    // no insert() method for field pan
    /*
     * Field: lo
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn lo_extract(&self) -> u64 {
        // bits 16..19
        self.0.get_bits(16..=19)
    }

    /// reads the current register value and extract field `lo` from it
    pub fn lo_read() -> u64 {
        Self::with_reg_val().lo_extract()
    }
    // no insert() method for field lo
    /*
     * Field: hpds
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn hpds_extract(&self) -> u64 {
        // bits 12..15
        self.0.get_bits(12..=15)
    }

    /// reads the current register value and extract field `hpds` from it
    pub fn hpds_read() -> u64 {
        Self::with_reg_val().hpds_extract()
    }
    // no insert() method for field hpds
    /*
     * Field: vh
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn vh_extract(&self) -> u64 {
        // bits 8..11
        self.0.get_bits(8..=11)
    }

    /// reads the current register value and extract field `vh` from it
    pub fn vh_read() -> u64 {
        Self::with_reg_val().vh_extract()
    }
    // no insert() method for field vh
    /*
     * Field: vmidbits
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn vmidbits_extract(&self) -> u64 {
        // bits 4..7
        self.0.get_bits(4..=7)
    }

    /// reads the current register value and extract field `vmidbits` from it
    pub fn vmidbits_read() -> u64 {
        Self::with_reg_val().vmidbits_extract()
    }
    // no insert() method for field vmidbits
    /*
     * Field: hafdbs
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn hafdbs_extract(&self) -> u64 {
        // bits 0..3
        self.0.get_bits(0..=3)
    }

    /// reads the current register value and extract field `hafdbs` from it
    pub fn hafdbs_read() -> u64 {
        Self::with_reg_val().hafdbs_extract()
    }
    // no insert() method for field hafdbs
}

impl Default for IdAa64mmfr1El1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> IdAa64mmfr1El1 {
        IdAa64mmfr1El1(0)
    }
}
