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
 * Generated on: 2022-08-22T16:35:53.082091
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
 * Register:    Trace Filter Control Register (EL1) (trfcr_el1)
 * Group:       Debug registers
 * Type:        64-bit Register
 * Description: Provides EL1 controls for Trace.
 * File:        AArch64-trfcr_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Trace Filter Control Register (EL1) value in memory
pub struct TrfcrEl1(u64);

/// struct implementation for accessing the fields of register trfcr_el1
impl TrfcrEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> TrfcrEl1 {
        Self::default()
    }

    /// collects the modifications of TrfcrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> TrfcrEl1 {
        TrfcrEl1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> TrfcrEl1 {
        let curval = Self::reg_rawrd() & 0x63;
        TrfcrEl1(curval)
    }

    /// reading the Trace Filter Control Register (EL1) (trfcr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, TRFCR_EL1
            asm!("mrs {}, S3_0_C1_C2_1", out(reg) regval);
        }
        return regval;
    }

    /// writing the Trace Filter Control Register (EL1) (trfcr_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR TRFCR_EL1, <Xt>
            asm!("msr S3_0_C1_C2_1, {}", in(reg) val);
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x63;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 99;
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
     * Field: e1tre
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn e1tre_extract(&self) -> u64 {
        // bits 1..1
        self.0.get_bits(1..=1)
    }

    /// reads the current register value and extract field `e1tre` from it
    pub fn e1tre_read() -> u64 {
        Self::with_reg_val().e1tre_extract()
    }

    /// inserts the given value `val` into the field `e1tre`
    pub fn e1tre_insert(&mut self, val: u64) -> &mut Self {
        // bits 1..1
        self.0.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `e1tre` field, and writes the updated value
    pub fn e1tre_write(val: u64) {
        Self::with_reg_val().e1tre_insert(val).write();
    }

    /*
     * Field: e0tre
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn e0tre_extract(&self) -> u64 {
        // bits 0..0
        self.0.get_bits(0..=0)
    }

    /// reads the current register value and extract field `e0tre` from it
    pub fn e0tre_read() -> u64 {
        Self::with_reg_val().e0tre_extract()
    }

    /// inserts the given value `val` into the field `e0tre`
    pub fn e0tre_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..0
        self.0.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `e0tre` field, and writes the updated value
    pub fn e0tre_write(val: u64) {
        Self::with_reg_val().e0tre_insert(val).write();
    }
}

impl Default for TrfcrEl1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> TrfcrEl1 {
        TrfcrEl1(0)
    }
}
