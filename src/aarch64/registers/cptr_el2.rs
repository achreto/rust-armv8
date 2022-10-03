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
 * Generated on: 2022-08-22T16:35:53.053321
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
 * Register:    Architectural Feature Trap Register (EL2) (cptr_el2)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Controls trapping to EL2 of accesses to
 * File:        AArch64-cptr_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Architectural Feature Trap Register (EL2) value in memory
pub struct CptrEl2(u64);

/// struct implementation for accessing the fields of register cptr_el2
impl CptrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> CptrEl2 {
        Self::default()
    }

    /// collects the modifications of CptrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> CptrEl2 {
        CptrEl2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> CptrEl2 {
        let curval = Self::reg_rawrd() & 0xd0330000;
        CptrEl2(curval)
    }

    /// reading the Architectural Feature Trap Register (EL2) (cptr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, CPTR_EL2
            asm!("mrs {}, cptr_el2", out(reg) regval);
        }
        return regval;
    }

    /// writing the Architectural Feature Trap Register (EL2) (cptr_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR CPTR_EL2, <Xt>
            asm!("msr cptr_el2, {}", in(reg) val);
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xd0330000;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 3493003264;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: tcpac
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tcpac_extract(&self) -> u64 {
        // bits 31..31
        self.0.get_bits(31..=31)
    }

    /// reads the current register value and extract field `tcpac` from it
    pub fn tcpac_read() -> u64 {
        Self::with_reg_val().tcpac_extract()
    }

    /// inserts the given value `val` into the field `tcpac`
    pub fn tcpac_insert(&mut self, val: u64) -> &mut Self {
        // bits 31..31
        self.0.set_bits(31..=31, val);
        self
    }

    /// reads the register, updates the `tcpac` field, and writes the updated value
    pub fn tcpac_write(val: u64) {
        Self::with_reg_val().tcpac_insert(val).write();
    }

    /*
     * Field: tam_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tam_1_extract(&self) -> u64 {
        // bits 30..30
        self.0.get_bits(30..=30)
    }

    /// reads the current register value and extract field `tam_1` from it
    pub fn tam_1_read() -> u64 {
        Self::with_reg_val().tam_1_extract()
    }

    /// inserts the given value `val` into the field `tam_1`
    pub fn tam_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 30..30
        self.0.set_bits(30..=30, val);
        self
    }

    /// reads the register, updates the `tam_1` field, and writes the updated value
    pub fn tam_1_write(val: u64) {
        Self::with_reg_val().tam_1_insert(val).write();
    }

    /*
     * Field: tta
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tta_extract(&self) -> u64 {
        // bits 28..28
        self.0.get_bits(28..=28)
    }

    /// reads the current register value and extract field `tta` from it
    pub fn tta_read() -> u64 {
        Self::with_reg_val().tta_extract()
    }

    /// inserts the given value `val` into the field `tta`
    pub fn tta_insert(&mut self, val: u64) -> &mut Self {
        // bits 28..28
        self.0.set_bits(28..=28, val);
        self
    }

    /// reads the register, updates the `tta` field, and writes the updated value
    pub fn tta_write(val: u64) {
        Self::with_reg_val().tta_insert(val).write();
    }

    /*
     * Field: fpen
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fpen_extract(&self) -> u64 {
        // bits 20..21
        self.0.get_bits(20..=21)
    }

    /// reads the current register value and extract field `fpen` from it
    pub fn fpen_read() -> u64 {
        Self::with_reg_val().fpen_extract()
    }

    /// inserts the given value `val` into the field `fpen`
    pub fn fpen_insert(&mut self, val: u64) -> &mut Self {
        // bits 20..21
        self.0.set_bits(20..=21, val);
        self
    }

    /// reads the register, updates the `fpen` field, and writes the updated value
    pub fn fpen_write(val: u64) {
        Self::with_reg_val().fpen_insert(val).write();
    }

    /*
     * Field: zen_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn zen_1_extract(&self) -> u64 {
        // bits 16..17
        self.0.get_bits(16..=17)
    }

    /// reads the current register value and extract field `zen_1` from it
    pub fn zen_1_read() -> u64 {
        Self::with_reg_val().zen_1_extract()
    }

    /// inserts the given value `val` into the field `zen_1`
    pub fn zen_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 16..17
        self.0.set_bits(16..=17, val);
        self
    }

    /// reads the register, updates the `zen_1` field, and writes the updated value
    pub fn zen_1_write(val: u64) {
        Self::with_reg_val().zen_1_insert(val).write();
    }
}

impl Default for CptrEl2 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> CptrEl2 {
        CptrEl2(0)
    }
}
