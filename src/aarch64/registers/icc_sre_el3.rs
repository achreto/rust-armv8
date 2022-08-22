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
 * Generated on: 2022-08-22T15:51:28.519713
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
 * Register:    Interrupt Controller System Register Enable register (EL3) (icc_sre_el3)
 * Group:       GIC control registers
 * Type:        64-bit Register
 * Description: Controls whether the System register interface or the memory-mapped interface to the GIC CPU interface is used for EL3.
 * File:        AArch64-icc_sre_el3.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Interrupt Controller System Register Enable register (EL3) value in memory
pub struct IccSreEl3(u64);

/// struct implementation for accessing the fields of register icc_sre_el3
impl IccSreEl3 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IccSreEl3 {
        Self::default()
    }

    /// collects the modifications of IccSreEl3 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IccSreEl3 {
        IccSreEl3(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> IccSreEl3 {
        let curval = Self::reg_rawrd() & 0xf;
        IccSreEl3(curval)
    }

    /// reading the Interrupt Controller System Register Enable register (EL3) (icc_sre_el3) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ICC_SRE_EL3
            llvm_asm!("mrs $0, icc_sre_el3" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Interrupt Controller System Register Enable register (EL3) (icc_sre_el3) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR ICC_SRE_EL3, <Xt>
            llvm_asm!("msr icc_sre_el3, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xf;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 15;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: enable
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn enable_extract(&self) -> u64 {
        // bits 3..3
        self.val.get_bits(3..=3)
    }

    /// reads the current register value and extract field `enable` from it
    pub fn enable_read(&mut self) -> u64 {
        Self::with_reg_val().enable_extract()
    }

    /// inserts the given value `val` into the field `enable`
    pub fn enable_insert(&mut self, val: u64) -> &mut self {
        // bits 3..3
        self.val.set_bits(3..=3, val);
        self
    }

    /// reads the register, updates the `enable` field, and writes the updated value
    pub fn enable_write(&mut self, val: u64) {
        Self::with_reg_val().enable_insert(val).write();
    }

    /*
     * Field: dib
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dib_extract(&self) -> u64 {
        // bits 2..2
        self.val.get_bits(2..=2)
    }

    /// reads the current register value and extract field `dib` from it
    pub fn dib_read(&mut self) -> u64 {
        Self::with_reg_val().dib_extract()
    }

    /// inserts the given value `val` into the field `dib`
    pub fn dib_insert(&mut self, val: u64) -> &mut self {
        // bits 2..2
        self.val.set_bits(2..=2, val);
        self
    }

    /// reads the register, updates the `dib` field, and writes the updated value
    pub fn dib_write(&mut self, val: u64) {
        Self::with_reg_val().dib_insert(val).write();
    }

    /*
     * Field: dfb
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dfb_extract(&self) -> u64 {
        // bits 1..1
        self.val.get_bits(1..=1)
    }

    /// reads the current register value and extract field `dfb` from it
    pub fn dfb_read(&mut self) -> u64 {
        Self::with_reg_val().dfb_extract()
    }

    /// inserts the given value `val` into the field `dfb`
    pub fn dfb_insert(&mut self, val: u64) -> &mut self {
        // bits 1..1
        self.val.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `dfb` field, and writes the updated value
    pub fn dfb_write(&mut self, val: u64) {
        Self::with_reg_val().dfb_insert(val).write();
    }

    /*
     * Field: sre
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn sre_extract(&self) -> u64 {
        // bits 0..0
        self.val.get_bits(0..=0)
    }

    /// reads the current register value and extract field `sre` from it
    pub fn sre_read(&mut self) -> u64 {
        Self::with_reg_val().sre_extract()
    }

    /// inserts the given value `val` into the field `sre`
    pub fn sre_insert(&mut self, val: u64) -> &mut self {
        // bits 0..0
        self.val.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `sre` field, and writes the updated value
    pub fn sre_write(&mut self, val: u64) {
        Self::with_reg_val().sre_insert(val).write();
    }
}

impl Default for IccSreEl3 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> IccSreEl3 {
        IccSreEl3(0)
    }
}
