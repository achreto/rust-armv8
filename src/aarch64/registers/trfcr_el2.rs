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
 * Generated on: 2022-08-22T16:35:53.082185
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
 * Register:    Trace Filter Control Register (EL2) (trfcr_el2)
 * Group:       Debug registers
 * Type:        64-bit Register
 * Description: Provides EL2 controls for Trace.
 * File:        AArch64-trfcr_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Trace Filter Control Register (EL2) value in memory
pub struct TrfcrEl2(u64);

/// struct implementation for accessing the fields of register trfcr_el2
impl TrfcrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> TrfcrEl2 {
        Self::default()
    }

    /// collects the modifications of TrfcrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> TrfcrEl2 {
        TrfcrEl2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> TrfcrEl2 {
        let curval = Self::reg_rawrd() & 0x6b;
        TrfcrEl2(curval)
    }

    /// reading the Trace Filter Control Register (EL2) (trfcr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, TRFCR_EL2
            asm!("mrs {}, S3_4_C1_C2_1", out(reg) regval);
        }
        return regval;
    }

    /// writing the Trace Filter Control Register (EL2) (trfcr_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR TRFCR_EL2, <Xt>
            asm!("msr S3_4_C1_C2_1, {}", in(reg) val);
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x6b;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 107;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: ts
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ts_extract(&self) -> u64 {
        // bits 5..6
        self.0.get_bits(5..=6)
    }

    /// reads the current register value and extract field `ts` from it
    pub fn ts_read() -> u64 {
        Self::with_reg_val().ts_extract()
    }

    /// inserts the given value `val` into the field `ts`
    pub fn ts_insert(&mut self, val: u64) -> &mut Self {
        // bits 5..6
        self.0.set_bits(5..=6, val);
        self
    }

    /// reads the register, updates the `ts` field, and writes the updated value
    pub fn ts_write(val: u64) {
        Self::with_reg_val().ts_insert(val).write();
    }

    /*
     * Field: cx
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn cx_extract(&self) -> u64 {
        // bits 3..3
        self.0.get_bits(3..=3)
    }

    /// reads the current register value and extract field `cx` from it
    pub fn cx_read() -> u64 {
        Self::with_reg_val().cx_extract()
    }

    /// inserts the given value `val` into the field `cx`
    pub fn cx_insert(&mut self, val: u64) -> &mut Self {
        // bits 3..3
        self.0.set_bits(3..=3, val);
        self
    }

    /// reads the register, updates the `cx` field, and writes the updated value
    pub fn cx_write(val: u64) {
        Self::with_reg_val().cx_insert(val).write();
    }

    /*
     * Field: e2tre
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn e2tre_extract(&self) -> u64 {
        // bits 1..1
        self.0.get_bits(1..=1)
    }

    /// reads the current register value and extract field `e2tre` from it
    pub fn e2tre_read() -> u64 {
        Self::with_reg_val().e2tre_extract()
    }

    /// inserts the given value `val` into the field `e2tre`
    pub fn e2tre_insert(&mut self, val: u64) -> &mut Self {
        // bits 1..1
        self.0.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `e2tre` field, and writes the updated value
    pub fn e2tre_write(val: u64) {
        Self::with_reg_val().e2tre_insert(val).write();
    }

    /*
     * Field: e0htre
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn e0htre_extract(&self) -> u64 {
        // bits 0..0
        self.0.get_bits(0..=0)
    }

    /// reads the current register value and extract field `e0htre` from it
    pub fn e0htre_read() -> u64 {
        Self::with_reg_val().e0htre_extract()
    }

    /// inserts the given value `val` into the field `e0htre`
    pub fn e0htre_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..0
        self.0.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `e0htre` field, and writes the updated value
    pub fn e0htre_write(val: u64) {
        Self::with_reg_val().e0htre_insert(val).write();
    }
}

impl Default for TrfcrEl2 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> TrfcrEl2 {
        TrfcrEl2(0)
    }
}
