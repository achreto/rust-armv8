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
 * Generated on: 2022-08-22T16:35:53.053429
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
 * Register:    Architectural Feature Trap Register (EL3) (cptr_el3)
 * Group:       Security registers
 * Type:        64-bit Register
 * Description: Controls trapping to EL3 of accesses to
 * File:        AArch64-cptr_el3.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Architectural Feature Trap Register (EL3) value in memory
pub struct CptrEl3(u64);

/// struct implementation for accessing the fields of register cptr_el3
impl CptrEl3 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> CptrEl3 {
        Self::default()
    }

    /// collects the modifications of CptrEl3 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> CptrEl3 {
        CptrEl3(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> CptrEl3 {
        let curval = Self::reg_rawrd() & 0xc0100500;
        CptrEl3(curval)
    }

    /// reading the Architectural Feature Trap Register (EL3) (cptr_el3) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, CPTR_EL3
            asm!("mrs {}, cptr_el3", out(reg) regval);
        }
        return regval;
    }

    /// writing the Architectural Feature Trap Register (EL3) (cptr_el3) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR CPTR_EL3, <Xt>
            asm!("msr cptr_el3, {}", in(reg) val);
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xc0100500;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 3222275328;
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
        // bits 20..20
        self.0.get_bits(20..=20)
    }

    /// reads the current register value and extract field `tta` from it
    pub fn tta_read() -> u64 {
        Self::with_reg_val().tta_extract()
    }

    /// inserts the given value `val` into the field `tta`
    pub fn tta_insert(&mut self, val: u64) -> &mut Self {
        // bits 20..20
        self.0.set_bits(20..=20, val);
        self
    }

    /// reads the register, updates the `tta` field, and writes the updated value
    pub fn tta_write(val: u64) {
        Self::with_reg_val().tta_insert(val).write();
    }

    /*
     * Field: tfp
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tfp_extract(&self) -> u64 {
        // bits 10..10
        self.0.get_bits(10..=10)
    }

    /// reads the current register value and extract field `tfp` from it
    pub fn tfp_read() -> u64 {
        Self::with_reg_val().tfp_extract()
    }

    /// inserts the given value `val` into the field `tfp`
    pub fn tfp_insert(&mut self, val: u64) -> &mut Self {
        // bits 10..10
        self.0.set_bits(10..=10, val);
        self
    }

    /// reads the register, updates the `tfp` field, and writes the updated value
    pub fn tfp_write(val: u64) {
        Self::with_reg_val().tfp_insert(val).write();
    }

    /*
     * Field: ez_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ez_1_extract(&self) -> u64 {
        // bits 8..8
        self.0.get_bits(8..=8)
    }

    /// reads the current register value and extract field `ez_1` from it
    pub fn ez_1_read() -> u64 {
        Self::with_reg_val().ez_1_extract()
    }

    /// inserts the given value `val` into the field `ez_1`
    pub fn ez_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 8..8
        self.0.set_bits(8..=8, val);
        self
    }

    /// reads the register, updates the `ez_1` field, and writes the updated value
    pub fn ez_1_write(val: u64) {
        Self::with_reg_val().ez_1_insert(val).write();
    }
}

impl Default for CptrEl3 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> CptrEl3 {
        CptrEl3(0)
    }
}
