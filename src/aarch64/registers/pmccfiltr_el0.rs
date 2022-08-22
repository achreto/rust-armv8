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
 * Generated on: 2022-08-22T15:51:28.529930
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
 * Register:    Performance Monitors Cycle Count Filter Register (pmccfiltr_el0)
 * Group:       Performance Monitors registers
 * Type:        64-bit Register
 * Description: Determines the modes in which the Cycle Counter,
 * File:        AArch64-pmccfiltr_el0.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Performance Monitors Cycle Count Filter Register value in memory
pub struct PmccfiltrEl0(u64);

/// struct implementation for accessing the fields of register pmccfiltr_el0
impl PmccfiltrEl0 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> PmccfiltrEl0 {
        Self::default()
    }

    /// collects the modifications of PmccfiltrEl0 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> PmccfiltrEl0 {
        PmccfiltrEl0(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> PmccfiltrEl0 {
        let curval = Self::reg_rawrd() & 0xfd000000;
        PmccfiltrEl0(curval)
    }

    /// reading the Performance Monitors Cycle Count Filter Register (pmccfiltr_el0) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, PMCCFILTR_EL0
            llvm_asm!("mrs $0, pmccfiltr_el0" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Performance Monitors Cycle Count Filter Register (pmccfiltr_el0) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR PMCCFILTR_EL0, <Xt>
            llvm_asm!("msr pmccfiltr_el0, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xfd000000;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 4244635648;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: p
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn p_extract(&self) -> u64 {
        // bits 31..31
        self.val.get_bits(31..=31)
    }

    /// reads the current register value and extract field `p` from it
    pub fn p_read(&mut self) -> u64 {
        Self::with_reg_val().p_extract()
    }

    /// inserts the given value `val` into the field `p`
    pub fn p_insert(&mut self, val: u64) -> &mut self {
        // bits 31..31
        self.val.set_bits(31..=31, val);
        self
    }

    /// reads the register, updates the `p` field, and writes the updated value
    pub fn p_write(&mut self, val: u64) {
        Self::with_reg_val().p_insert(val).write();
    }

    /*
     * Field: u
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn u_extract(&self) -> u64 {
        // bits 30..30
        self.val.get_bits(30..=30)
    }

    /// reads the current register value and extract field `u` from it
    pub fn u_read(&mut self) -> u64 {
        Self::with_reg_val().u_extract()
    }

    /// inserts the given value `val` into the field `u`
    pub fn u_insert(&mut self, val: u64) -> &mut self {
        // bits 30..30
        self.val.set_bits(30..=30, val);
        self
    }

    /// reads the register, updates the `u` field, and writes the updated value
    pub fn u_write(&mut self, val: u64) {
        Self::with_reg_val().u_insert(val).write();
    }

    /*
     * Field: nsk_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn nsk_1_extract(&self) -> u64 {
        // bits 29..29
        self.val.get_bits(29..=29)
    }

    /// reads the current register value and extract field `nsk_1` from it
    pub fn nsk_1_read(&mut self) -> u64 {
        Self::with_reg_val().nsk_1_extract()
    }

    /// inserts the given value `val` into the field `nsk_1`
    pub fn nsk_1_insert(&mut self, val: u64) -> &mut self {
        // bits 29..29
        self.val.set_bits(29..=29, val);
        self
    }

    /// reads the register, updates the `nsk_1` field, and writes the updated value
    pub fn nsk_1_write(&mut self, val: u64) {
        Self::with_reg_val().nsk_1_insert(val).write();
    }

    /*
     * Field: nsu_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn nsu_1_extract(&self) -> u64 {
        // bits 28..28
        self.val.get_bits(28..=28)
    }

    /// reads the current register value and extract field `nsu_1` from it
    pub fn nsu_1_read(&mut self) -> u64 {
        Self::with_reg_val().nsu_1_extract()
    }

    /// inserts the given value `val` into the field `nsu_1`
    pub fn nsu_1_insert(&mut self, val: u64) -> &mut self {
        // bits 28..28
        self.val.set_bits(28..=28, val);
        self
    }

    /// reads the register, updates the `nsu_1` field, and writes the updated value
    pub fn nsu_1_write(&mut self, val: u64) {
        Self::with_reg_val().nsu_1_insert(val).write();
    }

    /*
     * Field: nsh_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn nsh_1_extract(&self) -> u64 {
        // bits 27..27
        self.val.get_bits(27..=27)
    }

    /// reads the current register value and extract field `nsh_1` from it
    pub fn nsh_1_read(&mut self) -> u64 {
        Self::with_reg_val().nsh_1_extract()
    }

    /// inserts the given value `val` into the field `nsh_1`
    pub fn nsh_1_insert(&mut self, val: u64) -> &mut self {
        // bits 27..27
        self.val.set_bits(27..=27, val);
        self
    }

    /// reads the register, updates the `nsh_1` field, and writes the updated value
    pub fn nsh_1_write(&mut self, val: u64) {
        Self::with_reg_val().nsh_1_insert(val).write();
    }

    /*
     * Field: m_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn m_1_extract(&self) -> u64 {
        // bits 26..26
        self.val.get_bits(26..=26)
    }

    /// reads the current register value and extract field `m_1` from it
    pub fn m_1_read(&mut self) -> u64 {
        Self::with_reg_val().m_1_extract()
    }

    /// inserts the given value `val` into the field `m_1`
    pub fn m_1_insert(&mut self, val: u64) -> &mut self {
        // bits 26..26
        self.val.set_bits(26..=26, val);
        self
    }

    /// reads the register, updates the `m_1` field, and writes the updated value
    pub fn m_1_write(&mut self, val: u64) {
        Self::with_reg_val().m_1_insert(val).write();
    }

    /*
     * Field: sh_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn sh_1_extract(&self) -> u64 {
        // bits 24..24
        self.val.get_bits(24..=24)
    }

    /// reads the current register value and extract field `sh_1` from it
    pub fn sh_1_read(&mut self) -> u64 {
        Self::with_reg_val().sh_1_extract()
    }

    /// inserts the given value `val` into the field `sh_1`
    pub fn sh_1_insert(&mut self, val: u64) -> &mut self {
        // bits 24..24
        self.val.set_bits(24..=24, val);
        self
    }

    /// reads the register, updates the `sh_1` field, and writes the updated value
    pub fn sh_1_write(&mut self, val: u64) {
        Self::with_reg_val().sh_1_insert(val).write();
    }
}

impl Default for PmccfiltrEl0 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> PmccfiltrEl0 {
        PmccfiltrEl0(0)
    }
}
