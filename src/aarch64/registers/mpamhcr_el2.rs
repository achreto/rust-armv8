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
 * Generated on: 2022-08-22T16:35:53.070242
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
 * Register:    MPAM Hypervisor Control Register (EL2) (mpamhcr_el2)
 * Group:       Memory Partitioning and Monitoring registers
 * Type:        64-bit Register
 * Description: Controls the PARTID virtualization features of MPAM. It controls the mapping of virtual PARTIDs into physical PARTIDs in
 * File:        AArch64-mpamhcr_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the MPAM Hypervisor Control Register (EL2) value in memory
pub struct MpamhcrEl2(u64);

/// struct implementation for accessing the fields of register mpamhcr_el2
impl MpamhcrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> MpamhcrEl2 {
        Self::default()
    }

    /// collects the modifications of MpamhcrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> MpamhcrEl2 {
        MpamhcrEl2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> MpamhcrEl2 {
        let curval = Self::reg_rawrd() & 0x80000103;
        MpamhcrEl2(curval)
    }

    /// reading the MPAM Hypervisor Control Register (EL2) (mpamhcr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, MPAMHCR_EL2
            asm!("mrs {}, S3_4_C10_C4_0", out(reg) regval);
        }
        return regval;
    }

    /// writing the MPAM Hypervisor Control Register (EL2) (mpamhcr_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR MPAMHCR_EL2, <Xt>
            asm!("msr S3_4_C10_C4_0, {}", in(reg) val);
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x80000103;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 2147483907;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: trap_mpamidr_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn trap_mpamidr_el1_extract(&self) -> u64 {
        // bits 31..31
        self.0.get_bits(31..=31)
    }

    /// reads the current register value and extract field `trap_mpamidr_el1` from it
    pub fn trap_mpamidr_el1_read() -> u64 {
        Self::with_reg_val().trap_mpamidr_el1_extract()
    }

    /// inserts the given value `val` into the field `trap_mpamidr_el1`
    pub fn trap_mpamidr_el1_insert(&mut self, val: u64) -> &mut Self {
        // bits 31..31
        self.0.set_bits(31..=31, val);
        self
    }

    /// reads the register, updates the `trap_mpamidr_el1` field, and writes the updated value
    pub fn trap_mpamidr_el1_write(val: u64) {
        Self::with_reg_val().trap_mpamidr_el1_insert(val).write();
    }

    /*
     * Field: gstapp_plk
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn gstapp_plk_extract(&self) -> u64 {
        // bits 8..8
        self.0.get_bits(8..=8)
    }

    /// reads the current register value and extract field `gstapp_plk` from it
    pub fn gstapp_plk_read() -> u64 {
        Self::with_reg_val().gstapp_plk_extract()
    }

    /// inserts the given value `val` into the field `gstapp_plk`
    pub fn gstapp_plk_insert(&mut self, val: u64) -> &mut Self {
        // bits 8..8
        self.0.set_bits(8..=8, val);
        self
    }

    /// reads the register, updates the `gstapp_plk` field, and writes the updated value
    pub fn gstapp_plk_write(val: u64) {
        Self::with_reg_val().gstapp_plk_insert(val).write();
    }

    /*
     * Field: el1_vpmen
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn el1_vpmen_extract(&self) -> u64 {
        // bits 1..1
        self.0.get_bits(1..=1)
    }

    /// reads the current register value and extract field `el1_vpmen` from it
    pub fn el1_vpmen_read() -> u64 {
        Self::with_reg_val().el1_vpmen_extract()
    }

    /// inserts the given value `val` into the field `el1_vpmen`
    pub fn el1_vpmen_insert(&mut self, val: u64) -> &mut Self {
        // bits 1..1
        self.0.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `el1_vpmen` field, and writes the updated value
    pub fn el1_vpmen_write(val: u64) {
        Self::with_reg_val().el1_vpmen_insert(val).write();
    }

    /*
     * Field: el0_vpmen
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn el0_vpmen_extract(&self) -> u64 {
        // bits 0..0
        self.0.get_bits(0..=0)
    }

    /// reads the current register value and extract field `el0_vpmen` from it
    pub fn el0_vpmen_read() -> u64 {
        Self::with_reg_val().el0_vpmen_extract()
    }

    /// inserts the given value `val` into the field `el0_vpmen`
    pub fn el0_vpmen_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..0
        self.0.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `el0_vpmen` field, and writes the updated value
    pub fn el0_vpmen_write(val: u64) {
        Self::with_reg_val().el0_vpmen_insert(val).write();
    }
}

impl Default for MpamhcrEl2 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> MpamhcrEl2 {
        MpamhcrEl2(0)
    }
}
