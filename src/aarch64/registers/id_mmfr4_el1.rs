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
 * Generated on: 2022-08-22T15:51:28.524487
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
 * Register:    AArch32 Memory Model Feature Register 4 (id_mmfr4_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides information about the implemented memory model and memory management support in AArch32 state.
 * File:        AArch64-id_mmfr4_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the AArch32 Memory Model Feature Register 4 value in memory
pub struct IdMmfr4El1(u64);

/// struct implementation for accessing the fields of register id_mmfr4_el1
impl IdMmfr4El1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IdMmfr4El1 {
        Self::default()
    }

    /// collects the modifications of IdMmfr4El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IdMmfr4El1 {
        IdMmfr4El1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> IdMmfr4El1 {
        let curval = Self::reg_rawrd() & 0xffffffff;
        IdMmfr4El1(curval)
    }

    /// reading the AArch32 Memory Model Feature Register 4 (id_mmfr4_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ID_MMFR4_EL1
            llvm_asm!("mrs $0, id_mmfr4_el1" : "=r"(regval));
        }
        return regval;
    }

    // register is not writable. not emitting write accessor

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xffffffff;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 4294967295;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: evt
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn evt_extract(&self) -> u64 {
        // bits 28..31
        self.val.get_bits(28..=31)
    }

    /// reads the current register value and extract field `evt` from it
    pub fn evt_read(&mut self) -> u64 {
        Self::with_reg_val().evt_extract()
    }
    // no insert() method for field evt
    /*
     * Field: ccidx
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ccidx_extract(&self) -> u64 {
        // bits 24..27
        self.val.get_bits(24..=27)
    }

    /// reads the current register value and extract field `ccidx` from it
    pub fn ccidx_read(&mut self) -> u64 {
        Self::with_reg_val().ccidx_extract()
    }
    // no insert() method for field ccidx
    /*
     * Field: lsm
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn lsm_extract(&self) -> u64 {
        // bits 20..23
        self.val.get_bits(20..=23)
    }

    /// reads the current register value and extract field `lsm` from it
    pub fn lsm_read(&mut self) -> u64 {
        Self::with_reg_val().lsm_extract()
    }
    // no insert() method for field lsm
    /*
     * Field: hpds
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn hpds_extract(&self) -> u64 {
        // bits 16..19
        self.val.get_bits(16..=19)
    }

    /// reads the current register value and extract field `hpds` from it
    pub fn hpds_read(&mut self) -> u64 {
        Self::with_reg_val().hpds_extract()
    }
    // no insert() method for field hpds
    /*
     * Field: cnp
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn cnp_extract(&self) -> u64 {
        // bits 12..15
        self.val.get_bits(12..=15)
    }

    /// reads the current register value and extract field `cnp` from it
    pub fn cnp_read(&mut self) -> u64 {
        Self::with_reg_val().cnp_extract()
    }
    // no insert() method for field cnp
    /*
     * Field: xnx
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn xnx_extract(&self) -> u64 {
        // bits 8..11
        self.val.get_bits(8..=11)
    }

    /// reads the current register value and extract field `xnx` from it
    pub fn xnx_read(&mut self) -> u64 {
        Self::with_reg_val().xnx_extract()
    }
    // no insert() method for field xnx
    /*
     * Field: ac2
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ac2_extract(&self) -> u64 {
        // bits 4..7
        self.val.get_bits(4..=7)
    }

    /// reads the current register value and extract field `ac2` from it
    pub fn ac2_read(&mut self) -> u64 {
        Self::with_reg_val().ac2_extract()
    }
    // no insert() method for field ac2
    /*
     * Field: specsei
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn specsei_extract(&self) -> u64 {
        // bits 0..3
        self.val.get_bits(0..=3)
    }

    /// reads the current register value and extract field `specsei` from it
    pub fn specsei_read(&mut self) -> u64 {
        Self::with_reg_val().specsei_extract()
    }
    // no insert() method for field specsei
}

impl Default for IdMmfr4El1 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> IdMmfr4El1 {
        IdMmfr4El1(0)
    }
}
