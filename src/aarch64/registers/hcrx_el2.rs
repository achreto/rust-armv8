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
 * Generated on: 2022-08-22T16:35:53.058907
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
 * Register:    Extended Hypervisor Configuration Register (hcrx_el2)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Provides configuration controls for virtualization, including defining whether various operations are trapped to EL2.
 * File:        AArch64-hcrx_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Extended Hypervisor Configuration Register value in memory
pub struct HcrxEl2(u64);

/// struct implementation for accessing the fields of register hcrx_el2
impl HcrxEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> HcrxEl2 {
        Self::default()
    }

    /// collects the modifications of HcrxEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> HcrxEl2 {
        HcrxEl2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> HcrxEl2 {
        let curval = Self::reg_rawrd() & 0x1f;
        HcrxEl2(curval)
    }

    /// reading the Extended Hypervisor Configuration Register (hcrx_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, HCRX_EL2
            asm!("mrs {}, S3_4_C1_C2_2", out(reg) regval);
        }
        return regval;
    }

    /// writing the Extended Hypervisor Configuration Register (hcrx_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR HCRX_EL2, <Xt>
            asm!("msr S3_4_C1_C2_2, {}", in(reg) val);
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x1f;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 31;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: fgtnxs_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fgtnxs_1_extract(&self) -> u64 {
        // bits 4..4
        self.0.get_bits(4..=4)
    }

    /// reads the current register value and extract field `fgtnxs_1` from it
    pub fn fgtnxs_1_read() -> u64 {
        Self::with_reg_val().fgtnxs_1_extract()
    }

    /// inserts the given value `val` into the field `fgtnxs_1`
    pub fn fgtnxs_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 4..4
        self.0.set_bits(4..=4, val);
        self
    }

    /// reads the register, updates the `fgtnxs_1` field, and writes the updated value
    pub fn fgtnxs_1_write(val: u64) {
        Self::with_reg_val().fgtnxs_1_insert(val).write();
    }

    /*
     * Field: fnxs_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fnxs_1_extract(&self) -> u64 {
        // bits 3..3
        self.0.get_bits(3..=3)
    }

    /// reads the current register value and extract field `fnxs_1` from it
    pub fn fnxs_1_read() -> u64 {
        Self::with_reg_val().fnxs_1_extract()
    }

    /// inserts the given value `val` into the field `fnxs_1`
    pub fn fnxs_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 3..3
        self.0.set_bits(3..=3, val);
        self
    }

    /// reads the register, updates the `fnxs_1` field, and writes the updated value
    pub fn fnxs_1_write(val: u64) {
        Self::with_reg_val().fnxs_1_insert(val).write();
    }

    /*
     * Field: enasr_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn enasr_1_extract(&self) -> u64 {
        // bits 2..2
        self.0.get_bits(2..=2)
    }

    /// reads the current register value and extract field `enasr_1` from it
    pub fn enasr_1_read() -> u64 {
        Self::with_reg_val().enasr_1_extract()
    }

    /// inserts the given value `val` into the field `enasr_1`
    pub fn enasr_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 2..2
        self.0.set_bits(2..=2, val);
        self
    }

    /// reads the register, updates the `enasr_1` field, and writes the updated value
    pub fn enasr_1_write(val: u64) {
        Self::with_reg_val().enasr_1_insert(val).write();
    }

    /*
     * Field: enals_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn enals_1_extract(&self) -> u64 {
        // bits 1..1
        self.0.get_bits(1..=1)
    }

    /// reads the current register value and extract field `enals_1` from it
    pub fn enals_1_read() -> u64 {
        Self::with_reg_val().enals_1_extract()
    }

    /// inserts the given value `val` into the field `enals_1`
    pub fn enals_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 1..1
        self.0.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `enals_1` field, and writes the updated value
    pub fn enals_1_write(val: u64) {
        Self::with_reg_val().enals_1_insert(val).write();
    }

    /*
     * Field: enas0_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn enas0_1_extract(&self) -> u64 {
        // bits 0..0
        self.0.get_bits(0..=0)
    }

    /// reads the current register value and extract field `enas0_1` from it
    pub fn enas0_1_read() -> u64 {
        Self::with_reg_val().enas0_1_extract()
    }

    /// inserts the given value `val` into the field `enas0_1`
    pub fn enas0_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..0
        self.0.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `enas0_1` field, and writes the updated value
    pub fn enas0_1_write(val: u64) {
        Self::with_reg_val().enas0_1_insert(val).write();
    }
}

impl Default for HcrxEl2 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> HcrxEl2 {
        HcrxEl2(0)
    }
}
