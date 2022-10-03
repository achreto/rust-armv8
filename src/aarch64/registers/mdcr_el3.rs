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
 * Generated on: 2022-08-22T16:35:53.069241
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
 * Register:    Monitor Debug Configuration Register (EL3) (mdcr_el3)
 * Group:       Security registers
 * Type:        64-bit Register
 * Description: Provides EL3 configuration options for self-hosted debug and the Performance Monitors Extension.
 * File:        AArch64-mdcr_el3.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Monitor Debug Configuration Register (EL3) value in memory
pub struct MdcrEl3(u64);

/// struct implementation for accessing the fields of register mdcr_el3
impl MdcrEl3 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> MdcrEl3 {
        Self::default()
    }

    /// collects the modifications of MdcrEl3 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> MdcrEl3 {
        MdcrEl3(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> MdcrEl3 {
        let curval = Self::reg_rawrd() & 0x1c18bff640;
        MdcrEl3(curval)
    }

    /// reading the Monitor Debug Configuration Register (EL3) (mdcr_el3) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, MDCR_EL3
            asm!("mrs {}, mdcr_el3", out(reg) regval);
        }
        return regval;
    }

    /// writing the Monitor Debug Configuration Register (EL3) (mdcr_el3) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR MDCR_EL3, <Xt>
            asm!("msr mdcr_el3, {}", in(reg) val);
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x1c18bff640;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 120674317888;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: enpmsn_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn enpmsn_1_extract(&self) -> u64 {
        // bits 36..36
        self.0.get_bits(36..=36)
    }

    /// reads the current register value and extract field `enpmsn_1` from it
    pub fn enpmsn_1_read() -> u64 {
        Self::with_reg_val().enpmsn_1_extract()
    }

    /// inserts the given value `val` into the field `enpmsn_1`
    pub fn enpmsn_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 36..36
        self.0.set_bits(36..=36, val);
        self
    }

    /// reads the register, updates the `enpmsn_1` field, and writes the updated value
    pub fn enpmsn_1_write(val: u64) {
        Self::with_reg_val().enpmsn_1_insert(val).write();
    }

    /*
     * Field: mpmx_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn mpmx_1_extract(&self) -> u64 {
        // bits 35..35
        self.0.get_bits(35..=35)
    }

    /// reads the current register value and extract field `mpmx_1` from it
    pub fn mpmx_1_read() -> u64 {
        Self::with_reg_val().mpmx_1_extract()
    }

    /// inserts the given value `val` into the field `mpmx_1`
    pub fn mpmx_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 35..35
        self.0.set_bits(35..=35, val);
        self
    }

    /// reads the register, updates the `mpmx_1` field, and writes the updated value
    pub fn mpmx_1_write(val: u64) {
        Self::with_reg_val().mpmx_1_insert(val).write();
    }

    /*
     * Field: mccd_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn mccd_1_extract(&self) -> u64 {
        // bits 34..34
        self.0.get_bits(34..=34)
    }

    /// reads the current register value and extract field `mccd_1` from it
    pub fn mccd_1_read() -> u64 {
        Self::with_reg_val().mccd_1_extract()
    }

    /// inserts the given value `val` into the field `mccd_1`
    pub fn mccd_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 34..34
        self.0.set_bits(34..=34, val);
        self
    }

    /// reads the register, updates the `mccd_1` field, and writes the updated value
    pub fn mccd_1_write(val: u64) {
        Self::with_reg_val().mccd_1_insert(val).write();
    }

    /*
     * Field: mtpme_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn mtpme_1_extract(&self) -> u64 {
        // bits 28..28
        self.0.get_bits(28..=28)
    }

    /// reads the current register value and extract field `mtpme_1` from it
    pub fn mtpme_1_read() -> u64 {
        Self::with_reg_val().mtpme_1_extract()
    }

    /// inserts the given value `val` into the field `mtpme_1`
    pub fn mtpme_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 28..28
        self.0.set_bits(28..=28, val);
        self
    }

    /// reads the register, updates the `mtpme_1` field, and writes the updated value
    pub fn mtpme_1_write(val: u64) {
        Self::with_reg_val().mtpme_1_insert(val).write();
    }

    /*
     * Field: tdcc_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tdcc_1_extract(&self) -> u64 {
        // bits 27..27
        self.0.get_bits(27..=27)
    }

    /// reads the current register value and extract field `tdcc_1` from it
    pub fn tdcc_1_read() -> u64 {
        Self::with_reg_val().tdcc_1_extract()
    }

    /// inserts the given value `val` into the field `tdcc_1`
    pub fn tdcc_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 27..27
        self.0.set_bits(27..=27, val);
        self
    }

    /// reads the register, updates the `tdcc_1` field, and writes the updated value
    pub fn tdcc_1_write(val: u64) {
        Self::with_reg_val().tdcc_1_insert(val).write();
    }

    /*
     * Field: sccd_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn sccd_1_extract(&self) -> u64 {
        // bits 23..23
        self.0.get_bits(23..=23)
    }

    /// reads the current register value and extract field `sccd_1` from it
    pub fn sccd_1_read() -> u64 {
        Self::with_reg_val().sccd_1_extract()
    }

    /// inserts the given value `val` into the field `sccd_1`
    pub fn sccd_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 23..23
        self.0.set_bits(23..=23, val);
        self
    }

    /// reads the register, updates the `sccd_1` field, and writes the updated value
    pub fn sccd_1_write(val: u64) {
        Self::with_reg_val().sccd_1_insert(val).write();
    }

    /*
     * Field: epmad_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn epmad_1_extract(&self) -> u64 {
        // bits 21..21
        self.0.get_bits(21..=21)
    }

    /// reads the current register value and extract field `epmad_1` from it
    pub fn epmad_1_read() -> u64 {
        Self::with_reg_val().epmad_1_extract()
    }

    /// inserts the given value `val` into the field `epmad_1`
    pub fn epmad_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 21..21
        self.0.set_bits(21..=21, val);
        self
    }

    /// reads the register, updates the `epmad_1` field, and writes the updated value
    pub fn epmad_1_write(val: u64) {
        Self::with_reg_val().epmad_1_insert(val).write();
    }

    /*
     * Field: epmad_2
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn epmad_2_extract(&self) -> u64 {
        // bits 21..21
        self.0.get_bits(21..=21)
    }

    /// reads the current register value and extract field `epmad_2` from it
    pub fn epmad_2_read() -> u64 {
        Self::with_reg_val().epmad_2_extract()
    }

    /// inserts the given value `val` into the field `epmad_2`
    pub fn epmad_2_insert(&mut self, val: u64) -> &mut Self {
        // bits 21..21
        self.0.set_bits(21..=21, val);
        self
    }

    /// reads the register, updates the `epmad_2` field, and writes the updated value
    pub fn epmad_2_write(val: u64) {
        Self::with_reg_val().epmad_2_insert(val).write();
    }

    /*
     * Field: edad_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn edad_1_extract(&self) -> u64 {
        // bits 20..20
        self.0.get_bits(20..=20)
    }

    /// reads the current register value and extract field `edad_1` from it
    pub fn edad_1_read() -> u64 {
        Self::with_reg_val().edad_1_extract()
    }

    /// inserts the given value `val` into the field `edad_1`
    pub fn edad_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 20..20
        self.0.set_bits(20..=20, val);
        self
    }

    /// reads the register, updates the `edad_1` field, and writes the updated value
    pub fn edad_1_write(val: u64) {
        Self::with_reg_val().edad_1_insert(val).write();
    }

    /*
     * Field: edad_2
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn edad_2_extract(&self) -> u64 {
        // bits 20..20
        self.0.get_bits(20..=20)
    }

    /// reads the current register value and extract field `edad_2` from it
    pub fn edad_2_read() -> u64 {
        Self::with_reg_val().edad_2_extract()
    }

    /// inserts the given value `val` into the field `edad_2`
    pub fn edad_2_insert(&mut self, val: u64) -> &mut Self {
        // bits 20..20
        self.0.set_bits(20..=20, val);
        self
    }

    /// reads the register, updates the `edad_2` field, and writes the updated value
    pub fn edad_2_write(val: u64) {
        Self::with_reg_val().edad_2_insert(val).write();
    }

    /*
     * Field: edad_3
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn edad_3_extract(&self) -> u64 {
        // bits 20..20
        self.0.get_bits(20..=20)
    }

    /// reads the current register value and extract field `edad_3` from it
    pub fn edad_3_read() -> u64 {
        Self::with_reg_val().edad_3_extract()
    }

    /// inserts the given value `val` into the field `edad_3`
    pub fn edad_3_insert(&mut self, val: u64) -> &mut Self {
        // bits 20..20
        self.0.set_bits(20..=20, val);
        self
    }

    /// reads the register, updates the `edad_3` field, and writes the updated value
    pub fn edad_3_write(val: u64) {
        Self::with_reg_val().edad_3_insert(val).write();
    }

    /*
     * Field: ttrf_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ttrf_1_extract(&self) -> u64 {
        // bits 19..19
        self.0.get_bits(19..=19)
    }

    /// reads the current register value and extract field `ttrf_1` from it
    pub fn ttrf_1_read() -> u64 {
        Self::with_reg_val().ttrf_1_extract()
    }

    /// inserts the given value `val` into the field `ttrf_1`
    pub fn ttrf_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 19..19
        self.0.set_bits(19..=19, val);
        self
    }

    /// reads the register, updates the `ttrf_1` field, and writes the updated value
    pub fn ttrf_1_write(val: u64) {
        Self::with_reg_val().ttrf_1_insert(val).write();
    }

    /*
     * Field: ste_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ste_1_extract(&self) -> u64 {
        // bits 18..18
        self.0.get_bits(18..=18)
    }

    /// reads the current register value and extract field `ste_1` from it
    pub fn ste_1_read() -> u64 {
        Self::with_reg_val().ste_1_extract()
    }

    /// inserts the given value `val` into the field `ste_1`
    pub fn ste_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 18..18
        self.0.set_bits(18..=18, val);
        self
    }

    /// reads the register, updates the `ste_1` field, and writes the updated value
    pub fn ste_1_write(val: u64) {
        Self::with_reg_val().ste_1_insert(val).write();
    }

    /*
     * Field: spme_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn spme_1_extract(&self) -> u64 {
        // bits 17..17
        self.0.get_bits(17..=17)
    }

    /// reads the current register value and extract field `spme_1` from it
    pub fn spme_1_read() -> u64 {
        Self::with_reg_val().spme_1_extract()
    }

    /// inserts the given value `val` into the field `spme_1`
    pub fn spme_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 17..17
        self.0.set_bits(17..=17, val);
        self
    }

    /// reads the register, updates the `spme_1` field, and writes the updated value
    pub fn spme_1_write(val: u64) {
        Self::with_reg_val().spme_1_insert(val).write();
    }

    /*
     * Field: spme_2
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn spme_2_extract(&self) -> u64 {
        // bits 17..17
        self.0.get_bits(17..=17)
    }

    /// reads the current register value and extract field `spme_2` from it
    pub fn spme_2_read() -> u64 {
        Self::with_reg_val().spme_2_extract()
    }

    /// inserts the given value `val` into the field `spme_2`
    pub fn spme_2_insert(&mut self, val: u64) -> &mut Self {
        // bits 17..17
        self.0.set_bits(17..=17, val);
        self
    }

    /// reads the register, updates the `spme_2` field, and writes the updated value
    pub fn spme_2_write(val: u64) {
        Self::with_reg_val().spme_2_insert(val).write();
    }

    /*
     * Field: spme_3
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn spme_3_extract(&self) -> u64 {
        // bits 17..17
        self.0.get_bits(17..=17)
    }

    /// reads the current register value and extract field `spme_3` from it
    pub fn spme_3_read() -> u64 {
        Self::with_reg_val().spme_3_extract()
    }

    /// inserts the given value `val` into the field `spme_3`
    pub fn spme_3_insert(&mut self, val: u64) -> &mut Self {
        // bits 17..17
        self.0.set_bits(17..=17, val);
        self
    }

    /// reads the register, updates the `spme_3` field, and writes the updated value
    pub fn spme_3_write(val: u64) {
        Self::with_reg_val().spme_3_insert(val).write();
    }

    /*
     * Field: sdd
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn sdd_extract(&self) -> u64 {
        // bits 16..16
        self.0.get_bits(16..=16)
    }

    /// reads the current register value and extract field `sdd` from it
    pub fn sdd_read() -> u64 {
        Self::with_reg_val().sdd_extract()
    }

    /// inserts the given value `val` into the field `sdd`
    pub fn sdd_insert(&mut self, val: u64) -> &mut Self {
        // bits 16..16
        self.0.set_bits(16..=16, val);
        self
    }

    /// reads the register, updates the `sdd` field, and writes the updated value
    pub fn sdd_write(val: u64) {
        Self::with_reg_val().sdd_insert(val).write();
    }

    /*
     * Field: spd32_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn spd32_1_extract(&self) -> u64 {
        // bits 14..15
        self.0.get_bits(14..=15)
    }

    /// reads the current register value and extract field `spd32_1` from it
    pub fn spd32_1_read() -> u64 {
        Self::with_reg_val().spd32_1_extract()
    }

    /// inserts the given value `val` into the field `spd32_1`
    pub fn spd32_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 14..15
        self.0.set_bits(14..=15, val);
        self
    }

    /// reads the register, updates the `spd32_1` field, and writes the updated value
    pub fn spd32_1_write(val: u64) {
        Self::with_reg_val().spd32_1_insert(val).write();
    }

    /*
     * Field: nspb_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn nspb_1_extract(&self) -> u64 {
        // bits 12..13
        self.0.get_bits(12..=13)
    }

    /// reads the current register value and extract field `nspb_1` from it
    pub fn nspb_1_read() -> u64 {
        Self::with_reg_val().nspb_1_extract()
    }

    /// inserts the given value `val` into the field `nspb_1`
    pub fn nspb_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 12..13
        self.0.set_bits(12..=13, val);
        self
    }

    /// reads the register, updates the `nspb_1` field, and writes the updated value
    pub fn nspb_1_write(val: u64) {
        Self::with_reg_val().nspb_1_insert(val).write();
    }

    /*
     * Field: tdosa_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tdosa_1_extract(&self) -> u64 {
        // bits 10..10
        self.0.get_bits(10..=10)
    }

    /// reads the current register value and extract field `tdosa_1` from it
    pub fn tdosa_1_read() -> u64 {
        Self::with_reg_val().tdosa_1_extract()
    }

    /// inserts the given value `val` into the field `tdosa_1`
    pub fn tdosa_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 10..10
        self.0.set_bits(10..=10, val);
        self
    }

    /// reads the register, updates the `tdosa_1` field, and writes the updated value
    pub fn tdosa_1_write(val: u64) {
        Self::with_reg_val().tdosa_1_insert(val).write();
    }

    /*
     * Field: tdosa_2
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tdosa_2_extract(&self) -> u64 {
        // bits 10..10
        self.0.get_bits(10..=10)
    }

    /// reads the current register value and extract field `tdosa_2` from it
    pub fn tdosa_2_read() -> u64 {
        Self::with_reg_val().tdosa_2_extract()
    }

    /// inserts the given value `val` into the field `tdosa_2`
    pub fn tdosa_2_insert(&mut self, val: u64) -> &mut Self {
        // bits 10..10
        self.0.set_bits(10..=10, val);
        self
    }

    /// reads the register, updates the `tdosa_2` field, and writes the updated value
    pub fn tdosa_2_write(val: u64) {
        Self::with_reg_val().tdosa_2_insert(val).write();
    }

    /*
     * Field: tda
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tda_extract(&self) -> u64 {
        // bits 9..9
        self.0.get_bits(9..=9)
    }

    /// reads the current register value and extract field `tda` from it
    pub fn tda_read() -> u64 {
        Self::with_reg_val().tda_extract()
    }

    /// inserts the given value `val` into the field `tda`
    pub fn tda_insert(&mut self, val: u64) -> &mut Self {
        // bits 9..9
        self.0.set_bits(9..=9, val);
        self
    }

    /// reads the register, updates the `tda` field, and writes the updated value
    pub fn tda_write(val: u64) {
        Self::with_reg_val().tda_insert(val).write();
    }

    /*
     * Field: tpm_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tpm_1_extract(&self) -> u64 {
        // bits 6..6
        self.0.get_bits(6..=6)
    }

    /// reads the current register value and extract field `tpm_1` from it
    pub fn tpm_1_read() -> u64 {
        Self::with_reg_val().tpm_1_extract()
    }

    /// inserts the given value `val` into the field `tpm_1`
    pub fn tpm_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 6..6
        self.0.set_bits(6..=6, val);
        self
    }

    /// reads the register, updates the `tpm_1` field, and writes the updated value
    pub fn tpm_1_write(val: u64) {
        Self::with_reg_val().tpm_1_insert(val).write();
    }
}

impl Default for MdcrEl3 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> MdcrEl3 {
        MdcrEl3(0)
    }
}
