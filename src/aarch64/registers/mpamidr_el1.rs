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
 * Generated on: 2022-08-22T15:51:28.527263
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
 * Register:    MPAM ID Register (EL1) (mpamidr_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Indicates the presence and maximum PARTID and PMG values supported in the implementation. It also indicates whether the implementation supports MPAM virtualization.
 * File:        AArch64-mpamidr_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the MPAM ID Register (EL1) value in memory
pub struct MpamidrEl1(u64);

/// struct implementation for accessing the fields of register mpamidr_el1
impl MpamidrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> MpamidrEl1 {
        Self::default()
    }

    /// collects the modifications of MpamidrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> MpamidrEl1 {
        MpamidrEl1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> MpamidrEl1 {
        let curval = Self::reg_rawrd() & 0x340000ff001effff;
        MpamidrEl1(curval)
    }

    /// reading the MPAM ID Register (EL1) (mpamidr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, MPAMIDR_EL1
            llvm_asm!("mrs $0, S3_0_C10_C4_4" : "=r"(regval));
        }
        return regval;
    }

    // register is not writable. not emitting write accessor

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0x340000ff001effff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 3746995985190944767;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: has_sdeflt
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn has_sdeflt_extract(&self) -> u64 {
        // bits 61..61
        self.val.get_bits(61..=61)
    }

    /// reads the current register value and extract field `has_sdeflt` from it
    pub fn has_sdeflt_read(&mut self) -> u64 {
        Self::with_reg_val().has_sdeflt_extract()
    }
    // no insert() method for field has_sdeflt
    /*
     * Field: has_force_ns
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn has_force_ns_extract(&self) -> u64 {
        // bits 60..60
        self.val.get_bits(60..=60)
    }

    /// reads the current register value and extract field `has_force_ns` from it
    pub fn has_force_ns_read(&mut self) -> u64 {
        Self::with_reg_val().has_force_ns_extract()
    }
    // no insert() method for field has_force_ns
    /*
     * Field: has_tidr
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn has_tidr_extract(&self) -> u64 {
        // bits 58..58
        self.val.get_bits(58..=58)
    }

    /// reads the current register value and extract field `has_tidr` from it
    pub fn has_tidr_read(&mut self) -> u64 {
        Self::with_reg_val().has_tidr_extract()
    }
    // no insert() method for field has_tidr
    /*
     * Field: pmg_max
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pmg_max_extract(&self) -> u64 {
        // bits 32..39
        self.val.get_bits(32..=39)
    }

    /// reads the current register value and extract field `pmg_max` from it
    pub fn pmg_max_read(&mut self) -> u64 {
        Self::with_reg_val().pmg_max_extract()
    }
    // no insert() method for field pmg_max
    /*
     * Field: vpmr_max_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn vpmr_max_1_extract(&self) -> u64 {
        // bits 18..20
        self.val.get_bits(18..=20)
    }

    /// reads the current register value and extract field `vpmr_max_1` from it
    pub fn vpmr_max_1_read(&mut self) -> u64 {
        Self::with_reg_val().vpmr_max_1_extract()
    }
    // no insert() method for field vpmr_max_1
    /*
     * Field: has_hcr
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn has_hcr_extract(&self) -> u64 {
        // bits 17..17
        self.val.get_bits(17..=17)
    }

    /// reads the current register value and extract field `has_hcr` from it
    pub fn has_hcr_read(&mut self) -> u64 {
        Self::with_reg_val().has_hcr_extract()
    }
    // no insert() method for field has_hcr
    /*
     * Field: partid_max
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn partid_max_extract(&self) -> u64 {
        // bits 0..15
        self.val.get_bits(0..=15)
    }

    /// reads the current register value and extract field `partid_max` from it
    pub fn partid_max_read(&mut self) -> u64 {
        Self::with_reg_val().partid_max_extract()
    }
    // no insert() method for field partid_max
}

impl Default for MpamidrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> MpamidrEl1 {
        MpamidrEl1(0)
    }
}
