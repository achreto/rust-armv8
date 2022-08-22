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
 * Generated on: 2022-08-22T15:51:28.524088
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
 * Register:    AArch32 Memory Model Feature Register 0 (id_mmfr0_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides information about the implemented memory model and memory management support in AArch32 state.
 * File:        AArch64-id_mmfr0_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the AArch32 Memory Model Feature Register 0 value in memory
pub struct IdMmfr0El1(u64);

/// struct implementation for accessing the fields of register id_mmfr0_el1
impl IdMmfr0El1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IdMmfr0El1 {
        Self::default()
    }

    /// collects the modifications of IdMmfr0El1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IdMmfr0El1 {
        IdMmfr0El1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> IdMmfr0El1 {
        let curval = Self::reg_rawrd() & 0xffffffff;
        IdMmfr0El1(curval)
    }

    /// reading the AArch32 Memory Model Feature Register 0 (id_mmfr0_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ID_MMFR0_EL1
            llvm_asm!("mrs $0, id_mmfr0_el1" : "=r"(regval));
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
     * Field: innershr
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn innershr_extract(&self) -> u64 {
        // bits 28..31
        self.val.get_bits(28..=31)
    }

    /// reads the current register value and extract field `innershr` from it
    pub fn innershr_read(&mut self) -> u64 {
        Self::with_reg_val().innershr_extract()
    }
    // no insert() method for field innershr
    /*
     * Field: fcse
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fcse_extract(&self) -> u64 {
        // bits 24..27
        self.val.get_bits(24..=27)
    }

    /// reads the current register value and extract field `fcse` from it
    pub fn fcse_read(&mut self) -> u64 {
        Self::with_reg_val().fcse_extract()
    }
    // no insert() method for field fcse
    /*
     * Field: auxreg
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn auxreg_extract(&self) -> u64 {
        // bits 20..23
        self.val.get_bits(20..=23)
    }

    /// reads the current register value and extract field `auxreg` from it
    pub fn auxreg_read(&mut self) -> u64 {
        Self::with_reg_val().auxreg_extract()
    }
    // no insert() method for field auxreg
    /*
     * Field: tcm
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tcm_extract(&self) -> u64 {
        // bits 16..19
        self.val.get_bits(16..=19)
    }

    /// reads the current register value and extract field `tcm` from it
    pub fn tcm_read(&mut self) -> u64 {
        Self::with_reg_val().tcm_extract()
    }
    // no insert() method for field tcm
    /*
     * Field: sharelvl
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn sharelvl_extract(&self) -> u64 {
        // bits 12..15
        self.val.get_bits(12..=15)
    }

    /// reads the current register value and extract field `sharelvl` from it
    pub fn sharelvl_read(&mut self) -> u64 {
        Self::with_reg_val().sharelvl_extract()
    }
    // no insert() method for field sharelvl
    /*
     * Field: outershr
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn outershr_extract(&self) -> u64 {
        // bits 8..11
        self.val.get_bits(8..=11)
    }

    /// reads the current register value and extract field `outershr` from it
    pub fn outershr_read(&mut self) -> u64 {
        Self::with_reg_val().outershr_extract()
    }
    // no insert() method for field outershr
    /*
     * Field: pmsa
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pmsa_extract(&self) -> u64 {
        // bits 4..7
        self.val.get_bits(4..=7)
    }

    /// reads the current register value and extract field `pmsa` from it
    pub fn pmsa_read(&mut self) -> u64 {
        Self::with_reg_val().pmsa_extract()
    }
    // no insert() method for field pmsa
    /*
     * Field: vmsa
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn vmsa_extract(&self) -> u64 {
        // bits 0..3
        self.val.get_bits(0..=3)
    }

    /// reads the current register value and extract field `vmsa` from it
    pub fn vmsa_read(&mut self) -> u64 {
        Self::with_reg_val().vmsa_extract()
    }
    // no insert() method for field vmsa
}

impl Default for IdMmfr0El1 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> IdMmfr0El1 {
        IdMmfr0El1(0)
    }
}
