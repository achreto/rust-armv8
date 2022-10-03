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
 * Generated on: 2022-08-22T16:35:53.058435
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
 * Register:    Hypervisor Configuration Register (hcr_el2)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Provides configuration controls for virtualization, including defining whether various operations are trapped to EL2.
 * File:        AArch64-hcr_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Hypervisor Configuration Register value in memory
pub struct HcrEl2(u64);

/// struct implementation for accessing the fields of register hcr_el2
impl HcrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> HcrEl2 {
        Self::default()
    }

    /// collects the modifications of HcrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> HcrEl2 {
        HcrEl2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> HcrEl2 {
        let curval = Self::reg_rawrd() & 0xfffeff7fffffffff;
        HcrEl2(curval)
    }

    /// reading the Hypervisor Configuration Register (hcr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, HCR_EL2
            asm!("mrs {}, hcr_el2", out(reg) regval);
        }
        return regval;
    }

    /// writing the Hypervisor Configuration Register (hcr_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR HCR_EL2, <Xt>
            asm!("msr hcr_el2, {}", in(reg) val);
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xfffeff7fffffffff;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 18446462048977027071;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: twedel_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn twedel_1_extract(&self) -> u64 {
        // bits 60..63
        self.0.get_bits(60..=63)
    }

    /// reads the current register value and extract field `twedel_1` from it
    pub fn twedel_1_read() -> u64 {
        Self::with_reg_val().twedel_1_extract()
    }

    /// inserts the given value `val` into the field `twedel_1`
    pub fn twedel_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 60..63
        self.0.set_bits(60..=63, val);
        self
    }

    /// reads the register, updates the `twedel_1` field, and writes the updated value
    pub fn twedel_1_write(val: u64) {
        Self::with_reg_val().twedel_1_insert(val).write();
    }

    /*
     * Field: tweden_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tweden_1_extract(&self) -> u64 {
        // bits 59..59
        self.0.get_bits(59..=59)
    }

    /// reads the current register value and extract field `tweden_1` from it
    pub fn tweden_1_read() -> u64 {
        Self::with_reg_val().tweden_1_extract()
    }

    /// inserts the given value `val` into the field `tweden_1`
    pub fn tweden_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 59..59
        self.0.set_bits(59..=59, val);
        self
    }

    /// reads the register, updates the `tweden_1` field, and writes the updated value
    pub fn tweden_1_write(val: u64) {
        Self::with_reg_val().tweden_1_insert(val).write();
    }

    /*
     * Field: tid5_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tid5_1_extract(&self) -> u64 {
        // bits 58..58
        self.0.get_bits(58..=58)
    }

    /// reads the current register value and extract field `tid5_1` from it
    pub fn tid5_1_read() -> u64 {
        Self::with_reg_val().tid5_1_extract()
    }

    /// inserts the given value `val` into the field `tid5_1`
    pub fn tid5_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 58..58
        self.0.set_bits(58..=58, val);
        self
    }

    /// reads the register, updates the `tid5_1` field, and writes the updated value
    pub fn tid5_1_write(val: u64) {
        Self::with_reg_val().tid5_1_insert(val).write();
    }

    /*
     * Field: dct_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dct_1_extract(&self) -> u64 {
        // bits 57..57
        self.0.get_bits(57..=57)
    }

    /// reads the current register value and extract field `dct_1` from it
    pub fn dct_1_read() -> u64 {
        Self::with_reg_val().dct_1_extract()
    }

    /// inserts the given value `val` into the field `dct_1`
    pub fn dct_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 57..57
        self.0.set_bits(57..=57, val);
        self
    }

    /// reads the register, updates the `dct_1` field, and writes the updated value
    pub fn dct_1_write(val: u64) {
        Self::with_reg_val().dct_1_insert(val).write();
    }

    /*
     * Field: ata_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ata_1_extract(&self) -> u64 {
        // bits 56..56
        self.0.get_bits(56..=56)
    }

    /// reads the current register value and extract field `ata_1` from it
    pub fn ata_1_read() -> u64 {
        Self::with_reg_val().ata_1_extract()
    }

    /// inserts the given value `val` into the field `ata_1`
    pub fn ata_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 56..56
        self.0.set_bits(56..=56, val);
        self
    }

    /// reads the register, updates the `ata_1` field, and writes the updated value
    pub fn ata_1_write(val: u64) {
        Self::with_reg_val().ata_1_insert(val).write();
    }

    /*
     * Field: ttlbos_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ttlbos_1_extract(&self) -> u64 {
        // bits 55..55
        self.0.get_bits(55..=55)
    }

    /// reads the current register value and extract field `ttlbos_1` from it
    pub fn ttlbos_1_read() -> u64 {
        Self::with_reg_val().ttlbos_1_extract()
    }

    /// inserts the given value `val` into the field `ttlbos_1`
    pub fn ttlbos_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 55..55
        self.0.set_bits(55..=55, val);
        self
    }

    /// reads the register, updates the `ttlbos_1` field, and writes the updated value
    pub fn ttlbos_1_write(val: u64) {
        Self::with_reg_val().ttlbos_1_insert(val).write();
    }

    /*
     * Field: ttlbis_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ttlbis_1_extract(&self) -> u64 {
        // bits 54..54
        self.0.get_bits(54..=54)
    }

    /// reads the current register value and extract field `ttlbis_1` from it
    pub fn ttlbis_1_read() -> u64 {
        Self::with_reg_val().ttlbis_1_extract()
    }

    /// inserts the given value `val` into the field `ttlbis_1`
    pub fn ttlbis_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 54..54
        self.0.set_bits(54..=54, val);
        self
    }

    /// reads the register, updates the `ttlbis_1` field, and writes the updated value
    pub fn ttlbis_1_write(val: u64) {
        Self::with_reg_val().ttlbis_1_insert(val).write();
    }

    /*
     * Field: enscxt_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn enscxt_1_extract(&self) -> u64 {
        // bits 53..53
        self.0.get_bits(53..=53)
    }

    /// reads the current register value and extract field `enscxt_1` from it
    pub fn enscxt_1_read() -> u64 {
        Self::with_reg_val().enscxt_1_extract()
    }

    /// inserts the given value `val` into the field `enscxt_1`
    pub fn enscxt_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 53..53
        self.0.set_bits(53..=53, val);
        self
    }

    /// reads the register, updates the `enscxt_1` field, and writes the updated value
    pub fn enscxt_1_write(val: u64) {
        Self::with_reg_val().enscxt_1_insert(val).write();
    }

    /*
     * Field: tocu_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tocu_1_extract(&self) -> u64 {
        // bits 52..52
        self.0.get_bits(52..=52)
    }

    /// reads the current register value and extract field `tocu_1` from it
    pub fn tocu_1_read() -> u64 {
        Self::with_reg_val().tocu_1_extract()
    }

    /// inserts the given value `val` into the field `tocu_1`
    pub fn tocu_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 52..52
        self.0.set_bits(52..=52, val);
        self
    }

    /// reads the register, updates the `tocu_1` field, and writes the updated value
    pub fn tocu_1_write(val: u64) {
        Self::with_reg_val().tocu_1_insert(val).write();
    }

    /*
     * Field: amvoffen_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amvoffen_1_extract(&self) -> u64 {
        // bits 51..51
        self.0.get_bits(51..=51)
    }

    /// reads the current register value and extract field `amvoffen_1` from it
    pub fn amvoffen_1_read() -> u64 {
        Self::with_reg_val().amvoffen_1_extract()
    }

    /// inserts the given value `val` into the field `amvoffen_1`
    pub fn amvoffen_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 51..51
        self.0.set_bits(51..=51, val);
        self
    }

    /// reads the register, updates the `amvoffen_1` field, and writes the updated value
    pub fn amvoffen_1_write(val: u64) {
        Self::with_reg_val().amvoffen_1_insert(val).write();
    }

    /*
     * Field: ticab_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ticab_1_extract(&self) -> u64 {
        // bits 50..50
        self.0.get_bits(50..=50)
    }

    /// reads the current register value and extract field `ticab_1` from it
    pub fn ticab_1_read() -> u64 {
        Self::with_reg_val().ticab_1_extract()
    }

    /// inserts the given value `val` into the field `ticab_1`
    pub fn ticab_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 50..50
        self.0.set_bits(50..=50, val);
        self
    }

    /// reads the register, updates the `ticab_1` field, and writes the updated value
    pub fn ticab_1_write(val: u64) {
        Self::with_reg_val().ticab_1_insert(val).write();
    }

    /*
     * Field: tid4_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tid4_1_extract(&self) -> u64 {
        // bits 49..49
        self.0.get_bits(49..=49)
    }

    /// reads the current register value and extract field `tid4_1` from it
    pub fn tid4_1_read() -> u64 {
        Self::with_reg_val().tid4_1_extract()
    }

    /// inserts the given value `val` into the field `tid4_1`
    pub fn tid4_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 49..49
        self.0.set_bits(49..=49, val);
        self
    }

    /// reads the register, updates the `tid4_1` field, and writes the updated value
    pub fn tid4_1_write(val: u64) {
        Self::with_reg_val().tid4_1_insert(val).write();
    }

    /*
     * Field: fien_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fien_1_extract(&self) -> u64 {
        // bits 47..47
        self.0.get_bits(47..=47)
    }

    /// reads the current register value and extract field `fien_1` from it
    pub fn fien_1_read() -> u64 {
        Self::with_reg_val().fien_1_extract()
    }

    /// inserts the given value `val` into the field `fien_1`
    pub fn fien_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 47..47
        self.0.set_bits(47..=47, val);
        self
    }

    /// reads the register, updates the `fien_1` field, and writes the updated value
    pub fn fien_1_write(val: u64) {
        Self::with_reg_val().fien_1_insert(val).write();
    }

    /*
     * Field: fwb_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fwb_1_extract(&self) -> u64 {
        // bits 46..46
        self.0.get_bits(46..=46)
    }

    /// reads the current register value and extract field `fwb_1` from it
    pub fn fwb_1_read() -> u64 {
        Self::with_reg_val().fwb_1_extract()
    }

    /// inserts the given value `val` into the field `fwb_1`
    pub fn fwb_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 46..46
        self.0.set_bits(46..=46, val);
        self
    }

    /// reads the register, updates the `fwb_1` field, and writes the updated value
    pub fn fwb_1_write(val: u64) {
        Self::with_reg_val().fwb_1_insert(val).write();
    }

    /*
     * Field: nv2_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn nv2_1_extract(&self) -> u64 {
        // bits 45..45
        self.0.get_bits(45..=45)
    }

    /// reads the current register value and extract field `nv2_1` from it
    pub fn nv2_1_read() -> u64 {
        Self::with_reg_val().nv2_1_extract()
    }

    /// inserts the given value `val` into the field `nv2_1`
    pub fn nv2_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 45..45
        self.0.set_bits(45..=45, val);
        self
    }

    /// reads the register, updates the `nv2_1` field, and writes the updated value
    pub fn nv2_1_write(val: u64) {
        Self::with_reg_val().nv2_1_insert(val).write();
    }

    /*
     * Field: at_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn at_1_extract(&self) -> u64 {
        // bits 44..44
        self.0.get_bits(44..=44)
    }

    /// reads the current register value and extract field `at_1` from it
    pub fn at_1_read() -> u64 {
        Self::with_reg_val().at_1_extract()
    }

    /// inserts the given value `val` into the field `at_1`
    pub fn at_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 44..44
        self.0.set_bits(44..=44, val);
        self
    }

    /// reads the register, updates the `at_1` field, and writes the updated value
    pub fn at_1_write(val: u64) {
        Self::with_reg_val().at_1_insert(val).write();
    }

    /*
     * Field: nv1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn nv1_1_extract(&self) -> u64 {
        // bits 43..43
        self.0.get_bits(43..=43)
    }

    /// reads the current register value and extract field `nv1_1` from it
    pub fn nv1_1_read() -> u64 {
        Self::with_reg_val().nv1_1_extract()
    }

    /// inserts the given value `val` into the field `nv1_1`
    pub fn nv1_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 43..43
        self.0.set_bits(43..=43, val);
        self
    }

    /// reads the register, updates the `nv1_1` field, and writes the updated value
    pub fn nv1_1_write(val: u64) {
        Self::with_reg_val().nv1_1_insert(val).write();
    }

    /*
     * Field: nv1_2
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn nv1_2_extract(&self) -> u64 {
        // bits 43..43
        self.0.get_bits(43..=43)
    }

    /// reads the current register value and extract field `nv1_2` from it
    pub fn nv1_2_read() -> u64 {
        Self::with_reg_val().nv1_2_extract()
    }

    /// inserts the given value `val` into the field `nv1_2`
    pub fn nv1_2_insert(&mut self, val: u64) -> &mut Self {
        // bits 43..43
        self.0.set_bits(43..=43, val);
        self
    }

    /// reads the register, updates the `nv1_2` field, and writes the updated value
    pub fn nv1_2_write(val: u64) {
        Self::with_reg_val().nv1_2_insert(val).write();
    }

    /*
     * Field: nv_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn nv_1_extract(&self) -> u64 {
        // bits 42..42
        self.0.get_bits(42..=42)
    }

    /// reads the current register value and extract field `nv_1` from it
    pub fn nv_1_read() -> u64 {
        Self::with_reg_val().nv_1_extract()
    }

    /// inserts the given value `val` into the field `nv_1`
    pub fn nv_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 42..42
        self.0.set_bits(42..=42, val);
        self
    }

    /// reads the register, updates the `nv_1` field, and writes the updated value
    pub fn nv_1_write(val: u64) {
        Self::with_reg_val().nv_1_insert(val).write();
    }

    /*
     * Field: nv_2
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn nv_2_extract(&self) -> u64 {
        // bits 42..42
        self.0.get_bits(42..=42)
    }

    /// reads the current register value and extract field `nv_2` from it
    pub fn nv_2_read() -> u64 {
        Self::with_reg_val().nv_2_extract()
    }

    /// inserts the given value `val` into the field `nv_2`
    pub fn nv_2_insert(&mut self, val: u64) -> &mut Self {
        // bits 42..42
        self.0.set_bits(42..=42, val);
        self
    }

    /// reads the register, updates the `nv_2` field, and writes the updated value
    pub fn nv_2_write(val: u64) {
        Self::with_reg_val().nv_2_insert(val).write();
    }

    /*
     * Field: api_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn api_1_extract(&self) -> u64 {
        // bits 41..41
        self.0.get_bits(41..=41)
    }

    /// reads the current register value and extract field `api_1` from it
    pub fn api_1_read() -> u64 {
        Self::with_reg_val().api_1_extract()
    }

    /// inserts the given value `val` into the field `api_1`
    pub fn api_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 41..41
        self.0.set_bits(41..=41, val);
        self
    }

    /// reads the register, updates the `api_1` field, and writes the updated value
    pub fn api_1_write(val: u64) {
        Self::with_reg_val().api_1_insert(val).write();
    }

    /*
     * Field: apk_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn apk_1_extract(&self) -> u64 {
        // bits 40..40
        self.0.get_bits(40..=40)
    }

    /// reads the current register value and extract field `apk_1` from it
    pub fn apk_1_read() -> u64 {
        Self::with_reg_val().apk_1_extract()
    }

    /// inserts the given value `val` into the field `apk_1`
    pub fn apk_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 40..40
        self.0.set_bits(40..=40, val);
        self
    }

    /// reads the register, updates the `apk_1` field, and writes the updated value
    pub fn apk_1_write(val: u64) {
        Self::with_reg_val().apk_1_insert(val).write();
    }

    /*
     * Field: miocnce
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn miocnce_extract(&self) -> u64 {
        // bits 38..38
        self.0.get_bits(38..=38)
    }

    /// reads the current register value and extract field `miocnce` from it
    pub fn miocnce_read() -> u64 {
        Self::with_reg_val().miocnce_extract()
    }

    /// inserts the given value `val` into the field `miocnce`
    pub fn miocnce_insert(&mut self, val: u64) -> &mut Self {
        // bits 38..38
        self.0.set_bits(38..=38, val);
        self
    }

    /// reads the register, updates the `miocnce` field, and writes the updated value
    pub fn miocnce_write(val: u64) {
        Self::with_reg_val().miocnce_insert(val).write();
    }

    /*
     * Field: tea_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tea_1_extract(&self) -> u64 {
        // bits 37..37
        self.0.get_bits(37..=37)
    }

    /// reads the current register value and extract field `tea_1` from it
    pub fn tea_1_read() -> u64 {
        Self::with_reg_val().tea_1_extract()
    }

    /// inserts the given value `val` into the field `tea_1`
    pub fn tea_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 37..37
        self.0.set_bits(37..=37, val);
        self
    }

    /// reads the register, updates the `tea_1` field, and writes the updated value
    pub fn tea_1_write(val: u64) {
        Self::with_reg_val().tea_1_insert(val).write();
    }

    /*
     * Field: terr_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn terr_1_extract(&self) -> u64 {
        // bits 36..36
        self.0.get_bits(36..=36)
    }

    /// reads the current register value and extract field `terr_1` from it
    pub fn terr_1_read() -> u64 {
        Self::with_reg_val().terr_1_extract()
    }

    /// inserts the given value `val` into the field `terr_1`
    pub fn terr_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 36..36
        self.0.set_bits(36..=36, val);
        self
    }

    /// reads the register, updates the `terr_1` field, and writes the updated value
    pub fn terr_1_write(val: u64) {
        Self::with_reg_val().terr_1_insert(val).write();
    }

    /*
     * Field: tlor_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlor_1_extract(&self) -> u64 {
        // bits 35..35
        self.0.get_bits(35..=35)
    }

    /// reads the current register value and extract field `tlor_1` from it
    pub fn tlor_1_read() -> u64 {
        Self::with_reg_val().tlor_1_extract()
    }

    /// inserts the given value `val` into the field `tlor_1`
    pub fn tlor_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 35..35
        self.0.set_bits(35..=35, val);
        self
    }

    /// reads the register, updates the `tlor_1` field, and writes the updated value
    pub fn tlor_1_write(val: u64) {
        Self::with_reg_val().tlor_1_insert(val).write();
    }

    /*
     * Field: e2h_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn e2h_1_extract(&self) -> u64 {
        // bits 34..34
        self.0.get_bits(34..=34)
    }

    /// reads the current register value and extract field `e2h_1` from it
    pub fn e2h_1_read() -> u64 {
        Self::with_reg_val().e2h_1_extract()
    }

    /// inserts the given value `val` into the field `e2h_1`
    pub fn e2h_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 34..34
        self.0.set_bits(34..=34, val);
        self
    }

    /// reads the register, updates the `e2h_1` field, and writes the updated value
    pub fn e2h_1_write(val: u64) {
        Self::with_reg_val().e2h_1_insert(val).write();
    }

    /*
     * Field: id
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn id_extract(&self) -> u64 {
        // bits 33..33
        self.0.get_bits(33..=33)
    }

    /// reads the current register value and extract field `id` from it
    pub fn id_read() -> u64 {
        Self::with_reg_val().id_extract()
    }

    /// inserts the given value `val` into the field `id`
    pub fn id_insert(&mut self, val: u64) -> &mut Self {
        // bits 33..33
        self.0.set_bits(33..=33, val);
        self
    }

    /// reads the register, updates the `id` field, and writes the updated value
    pub fn id_write(val: u64) {
        Self::with_reg_val().id_insert(val).write();
    }

    /*
     * Field: cd
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn cd_extract(&self) -> u64 {
        // bits 32..32
        self.0.get_bits(32..=32)
    }

    /// reads the current register value and extract field `cd` from it
    pub fn cd_read() -> u64 {
        Self::with_reg_val().cd_extract()
    }

    /// inserts the given value `val` into the field `cd`
    pub fn cd_insert(&mut self, val: u64) -> &mut Self {
        // bits 32..32
        self.0.set_bits(32..=32, val);
        self
    }

    /// reads the register, updates the `cd` field, and writes the updated value
    pub fn cd_write(val: u64) {
        Self::with_reg_val().cd_insert(val).write();
    }

    /*
     * Field: rw_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn rw_1_extract(&self) -> u64 {
        // bits 31..31
        self.0.get_bits(31..=31)
    }

    /// reads the current register value and extract field `rw_1` from it
    pub fn rw_1_read() -> u64 {
        Self::with_reg_val().rw_1_extract()
    }

    /// inserts the given value `val` into the field `rw_1`
    pub fn rw_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 31..31
        self.0.set_bits(31..=31, val);
        self
    }

    /// reads the register, updates the `rw_1` field, and writes the updated value
    pub fn rw_1_write(val: u64) {
        Self::with_reg_val().rw_1_insert(val).write();
    }

    /*
     * Field: trvm
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn trvm_extract(&self) -> u64 {
        // bits 30..30
        self.0.get_bits(30..=30)
    }

    /// reads the current register value and extract field `trvm` from it
    pub fn trvm_read() -> u64 {
        Self::with_reg_val().trvm_extract()
    }

    /// inserts the given value `val` into the field `trvm`
    pub fn trvm_insert(&mut self, val: u64) -> &mut Self {
        // bits 30..30
        self.0.set_bits(30..=30, val);
        self
    }

    /// reads the register, updates the `trvm` field, and writes the updated value
    pub fn trvm_write(val: u64) {
        Self::with_reg_val().trvm_insert(val).write();
    }

    /*
     * Field: hcd_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn hcd_1_extract(&self) -> u64 {
        // bits 29..29
        self.0.get_bits(29..=29)
    }

    /// reads the current register value and extract field `hcd_1` from it
    pub fn hcd_1_read() -> u64 {
        Self::with_reg_val().hcd_1_extract()
    }

    /// inserts the given value `val` into the field `hcd_1`
    pub fn hcd_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 29..29
        self.0.set_bits(29..=29, val);
        self
    }

    /// reads the register, updates the `hcd_1` field, and writes the updated value
    pub fn hcd_1_write(val: u64) {
        Self::with_reg_val().hcd_1_insert(val).write();
    }

    /*
     * Field: tdz
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tdz_extract(&self) -> u64 {
        // bits 28..28
        self.0.get_bits(28..=28)
    }

    /// reads the current register value and extract field `tdz` from it
    pub fn tdz_read() -> u64 {
        Self::with_reg_val().tdz_extract()
    }

    /// inserts the given value `val` into the field `tdz`
    pub fn tdz_insert(&mut self, val: u64) -> &mut Self {
        // bits 28..28
        self.0.set_bits(28..=28, val);
        self
    }

    /// reads the register, updates the `tdz` field, and writes the updated value
    pub fn tdz_write(val: u64) {
        Self::with_reg_val().tdz_insert(val).write();
    }

    /*
     * Field: tge
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tge_extract(&self) -> u64 {
        // bits 27..27
        self.0.get_bits(27..=27)
    }

    /// reads the current register value and extract field `tge` from it
    pub fn tge_read() -> u64 {
        Self::with_reg_val().tge_extract()
    }

    /// inserts the given value `val` into the field `tge`
    pub fn tge_insert(&mut self, val: u64) -> &mut Self {
        // bits 27..27
        self.0.set_bits(27..=27, val);
        self
    }

    /// reads the register, updates the `tge` field, and writes the updated value
    pub fn tge_write(val: u64) {
        Self::with_reg_val().tge_insert(val).write();
    }

    /*
     * Field: tvm
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tvm_extract(&self) -> u64 {
        // bits 26..26
        self.0.get_bits(26..=26)
    }

    /// reads the current register value and extract field `tvm` from it
    pub fn tvm_read() -> u64 {
        Self::with_reg_val().tvm_extract()
    }

    /// inserts the given value `val` into the field `tvm`
    pub fn tvm_insert(&mut self, val: u64) -> &mut Self {
        // bits 26..26
        self.0.set_bits(26..=26, val);
        self
    }

    /// reads the register, updates the `tvm` field, and writes the updated value
    pub fn tvm_write(val: u64) {
        Self::with_reg_val().tvm_insert(val).write();
    }

    /*
     * Field: ttlb
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ttlb_extract(&self) -> u64 {
        // bits 25..25
        self.0.get_bits(25..=25)
    }

    /// reads the current register value and extract field `ttlb` from it
    pub fn ttlb_read() -> u64 {
        Self::with_reg_val().ttlb_extract()
    }

    /// inserts the given value `val` into the field `ttlb`
    pub fn ttlb_insert(&mut self, val: u64) -> &mut Self {
        // bits 25..25
        self.0.set_bits(25..=25, val);
        self
    }

    /// reads the register, updates the `ttlb` field, and writes the updated value
    pub fn ttlb_write(val: u64) {
        Self::with_reg_val().ttlb_insert(val).write();
    }

    /*
     * Field: tpu
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tpu_extract(&self) -> u64 {
        // bits 24..24
        self.0.get_bits(24..=24)
    }

    /// reads the current register value and extract field `tpu` from it
    pub fn tpu_read() -> u64 {
        Self::with_reg_val().tpu_extract()
    }

    /// inserts the given value `val` into the field `tpu`
    pub fn tpu_insert(&mut self, val: u64) -> &mut Self {
        // bits 24..24
        self.0.set_bits(24..=24, val);
        self
    }

    /// reads the register, updates the `tpu` field, and writes the updated value
    pub fn tpu_write(val: u64) {
        Self::with_reg_val().tpu_insert(val).write();
    }

    /*
     * Field: tpcp_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tpcp_1_extract(&self) -> u64 {
        // bits 23..23
        self.0.get_bits(23..=23)
    }

    /// reads the current register value and extract field `tpcp_1` from it
    pub fn tpcp_1_read() -> u64 {
        Self::with_reg_val().tpcp_1_extract()
    }

    /// inserts the given value `val` into the field `tpcp_1`
    pub fn tpcp_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 23..23
        self.0.set_bits(23..=23, val);
        self
    }

    /// reads the register, updates the `tpcp_1` field, and writes the updated value
    pub fn tpcp_1_write(val: u64) {
        Self::with_reg_val().tpcp_1_insert(val).write();
    }

    /*
     * Field: tpc_2
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tpc_2_extract(&self) -> u64 {
        // bits 23..23
        self.0.get_bits(23..=23)
    }

    /// reads the current register value and extract field `tpc_2` from it
    pub fn tpc_2_read() -> u64 {
        Self::with_reg_val().tpc_2_extract()
    }

    /// inserts the given value `val` into the field `tpc_2`
    pub fn tpc_2_insert(&mut self, val: u64) -> &mut Self {
        // bits 23..23
        self.0.set_bits(23..=23, val);
        self
    }

    /// reads the register, updates the `tpc_2` field, and writes the updated value
    pub fn tpc_2_write(val: u64) {
        Self::with_reg_val().tpc_2_insert(val).write();
    }

    /*
     * Field: tsw
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tsw_extract(&self) -> u64 {
        // bits 22..22
        self.0.get_bits(22..=22)
    }

    /// reads the current register value and extract field `tsw` from it
    pub fn tsw_read() -> u64 {
        Self::with_reg_val().tsw_extract()
    }

    /// inserts the given value `val` into the field `tsw`
    pub fn tsw_insert(&mut self, val: u64) -> &mut Self {
        // bits 22..22
        self.0.set_bits(22..=22, val);
        self
    }

    /// reads the register, updates the `tsw` field, and writes the updated value
    pub fn tsw_write(val: u64) {
        Self::with_reg_val().tsw_insert(val).write();
    }

    /*
     * Field: tacr
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tacr_extract(&self) -> u64 {
        // bits 21..21
        self.0.get_bits(21..=21)
    }

    /// reads the current register value and extract field `tacr` from it
    pub fn tacr_read() -> u64 {
        Self::with_reg_val().tacr_extract()
    }

    /// inserts the given value `val` into the field `tacr`
    pub fn tacr_insert(&mut self, val: u64) -> &mut Self {
        // bits 21..21
        self.0.set_bits(21..=21, val);
        self
    }

    /// reads the register, updates the `tacr` field, and writes the updated value
    pub fn tacr_write(val: u64) {
        Self::with_reg_val().tacr_insert(val).write();
    }

    /*
     * Field: tidcp
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tidcp_extract(&self) -> u64 {
        // bits 20..20
        self.0.get_bits(20..=20)
    }

    /// reads the current register value and extract field `tidcp` from it
    pub fn tidcp_read() -> u64 {
        Self::with_reg_val().tidcp_extract()
    }

    /// inserts the given value `val` into the field `tidcp`
    pub fn tidcp_insert(&mut self, val: u64) -> &mut Self {
        // bits 20..20
        self.0.set_bits(20..=20, val);
        self
    }

    /// reads the register, updates the `tidcp` field, and writes the updated value
    pub fn tidcp_write(val: u64) {
        Self::with_reg_val().tidcp_insert(val).write();
    }

    /*
     * Field: tsc
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tsc_extract(&self) -> u64 {
        // bits 19..19
        self.0.get_bits(19..=19)
    }

    /// reads the current register value and extract field `tsc` from it
    pub fn tsc_read() -> u64 {
        Self::with_reg_val().tsc_extract()
    }

    /// inserts the given value `val` into the field `tsc`
    pub fn tsc_insert(&mut self, val: u64) -> &mut Self {
        // bits 19..19
        self.0.set_bits(19..=19, val);
        self
    }

    /// reads the register, updates the `tsc` field, and writes the updated value
    pub fn tsc_write(val: u64) {
        Self::with_reg_val().tsc_insert(val).write();
    }

    /*
     * Field: tid3
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tid3_extract(&self) -> u64 {
        // bits 18..18
        self.0.get_bits(18..=18)
    }

    /// reads the current register value and extract field `tid3` from it
    pub fn tid3_read() -> u64 {
        Self::with_reg_val().tid3_extract()
    }

    /// inserts the given value `val` into the field `tid3`
    pub fn tid3_insert(&mut self, val: u64) -> &mut Self {
        // bits 18..18
        self.0.set_bits(18..=18, val);
        self
    }

    /// reads the register, updates the `tid3` field, and writes the updated value
    pub fn tid3_write(val: u64) {
        Self::with_reg_val().tid3_insert(val).write();
    }

    /*
     * Field: tid2
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tid2_extract(&self) -> u64 {
        // bits 17..17
        self.0.get_bits(17..=17)
    }

    /// reads the current register value and extract field `tid2` from it
    pub fn tid2_read() -> u64 {
        Self::with_reg_val().tid2_extract()
    }

    /// inserts the given value `val` into the field `tid2`
    pub fn tid2_insert(&mut self, val: u64) -> &mut Self {
        // bits 17..17
        self.0.set_bits(17..=17, val);
        self
    }

    /// reads the register, updates the `tid2` field, and writes the updated value
    pub fn tid2_write(val: u64) {
        Self::with_reg_val().tid2_insert(val).write();
    }

    /*
     * Field: tid1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tid1_extract(&self) -> u64 {
        // bits 16..16
        self.0.get_bits(16..=16)
    }

    /// reads the current register value and extract field `tid1` from it
    pub fn tid1_read() -> u64 {
        Self::with_reg_val().tid1_extract()
    }

    /// inserts the given value `val` into the field `tid1`
    pub fn tid1_insert(&mut self, val: u64) -> &mut Self {
        // bits 16..16
        self.0.set_bits(16..=16, val);
        self
    }

    /// reads the register, updates the `tid1` field, and writes the updated value
    pub fn tid1_write(val: u64) {
        Self::with_reg_val().tid1_insert(val).write();
    }

    /*
     * Field: tid0_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tid0_1_extract(&self) -> u64 {
        // bits 15..15
        self.0.get_bits(15..=15)
    }

    /// reads the current register value and extract field `tid0_1` from it
    pub fn tid0_1_read() -> u64 {
        Self::with_reg_val().tid0_1_extract()
    }

    /// inserts the given value `val` into the field `tid0_1`
    pub fn tid0_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 15..15
        self.0.set_bits(15..=15, val);
        self
    }

    /// reads the register, updates the `tid0_1` field, and writes the updated value
    pub fn tid0_1_write(val: u64) {
        Self::with_reg_val().tid0_1_insert(val).write();
    }

    /*
     * Field: twe
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn twe_extract(&self) -> u64 {
        // bits 14..14
        self.0.get_bits(14..=14)
    }

    /// reads the current register value and extract field `twe` from it
    pub fn twe_read() -> u64 {
        Self::with_reg_val().twe_extract()
    }

    /// inserts the given value `val` into the field `twe`
    pub fn twe_insert(&mut self, val: u64) -> &mut Self {
        // bits 14..14
        self.0.set_bits(14..=14, val);
        self
    }

    /// reads the register, updates the `twe` field, and writes the updated value
    pub fn twe_write(val: u64) {
        Self::with_reg_val().twe_insert(val).write();
    }

    /*
     * Field: twi
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn twi_extract(&self) -> u64 {
        // bits 13..13
        self.0.get_bits(13..=13)
    }

    /// reads the current register value and extract field `twi` from it
    pub fn twi_read() -> u64 {
        Self::with_reg_val().twi_extract()
    }

    /// inserts the given value `val` into the field `twi`
    pub fn twi_insert(&mut self, val: u64) -> &mut Self {
        // bits 13..13
        self.0.set_bits(13..=13, val);
        self
    }

    /// reads the register, updates the `twi` field, and writes the updated value
    pub fn twi_write(val: u64) {
        Self::with_reg_val().twi_insert(val).write();
    }

    /*
     * Field: dc
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dc_extract(&self) -> u64 {
        // bits 12..12
        self.0.get_bits(12..=12)
    }

    /// reads the current register value and extract field `dc` from it
    pub fn dc_read() -> u64 {
        Self::with_reg_val().dc_extract()
    }

    /// inserts the given value `val` into the field `dc`
    pub fn dc_insert(&mut self, val: u64) -> &mut Self {
        // bits 12..12
        self.0.set_bits(12..=12, val);
        self
    }

    /// reads the register, updates the `dc` field, and writes the updated value
    pub fn dc_write(val: u64) {
        Self::with_reg_val().dc_insert(val).write();
    }

    /*
     * Field: bsu
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn bsu_extract(&self) -> u64 {
        // bits 10..11
        self.0.get_bits(10..=11)
    }

    /// reads the current register value and extract field `bsu` from it
    pub fn bsu_read() -> u64 {
        Self::with_reg_val().bsu_extract()
    }

    /// inserts the given value `val` into the field `bsu`
    pub fn bsu_insert(&mut self, val: u64) -> &mut Self {
        // bits 10..11
        self.0.set_bits(10..=11, val);
        self
    }

    /// reads the register, updates the `bsu` field, and writes the updated value
    pub fn bsu_write(val: u64) {
        Self::with_reg_val().bsu_insert(val).write();
    }

    /*
     * Field: fb
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fb_extract(&self) -> u64 {
        // bits 9..9
        self.0.get_bits(9..=9)
    }

    /// reads the current register value and extract field `fb` from it
    pub fn fb_read() -> u64 {
        Self::with_reg_val().fb_extract()
    }

    /// inserts the given value `val` into the field `fb`
    pub fn fb_insert(&mut self, val: u64) -> &mut Self {
        // bits 9..9
        self.0.set_bits(9..=9, val);
        self
    }

    /// reads the register, updates the `fb` field, and writes the updated value
    pub fn fb_write(val: u64) {
        Self::with_reg_val().fb_insert(val).write();
    }

    /*
     * Field: vse
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn vse_extract(&self) -> u64 {
        // bits 8..8
        self.0.get_bits(8..=8)
    }

    /// reads the current register value and extract field `vse` from it
    pub fn vse_read() -> u64 {
        Self::with_reg_val().vse_extract()
    }

    /// inserts the given value `val` into the field `vse`
    pub fn vse_insert(&mut self, val: u64) -> &mut Self {
        // bits 8..8
        self.0.set_bits(8..=8, val);
        self
    }

    /// reads the register, updates the `vse` field, and writes the updated value
    pub fn vse_write(val: u64) {
        Self::with_reg_val().vse_insert(val).write();
    }

    /*
     * Field: vi
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn vi_extract(&self) -> u64 {
        // bits 7..7
        self.0.get_bits(7..=7)
    }

    /// reads the current register value and extract field `vi` from it
    pub fn vi_read() -> u64 {
        Self::with_reg_val().vi_extract()
    }

    /// inserts the given value `val` into the field `vi`
    pub fn vi_insert(&mut self, val: u64) -> &mut Self {
        // bits 7..7
        self.0.set_bits(7..=7, val);
        self
    }

    /// reads the register, updates the `vi` field, and writes the updated value
    pub fn vi_write(val: u64) {
        Self::with_reg_val().vi_insert(val).write();
    }

    /*
     * Field: vf
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn vf_extract(&self) -> u64 {
        // bits 6..6
        self.0.get_bits(6..=6)
    }

    /// reads the current register value and extract field `vf` from it
    pub fn vf_read() -> u64 {
        Self::with_reg_val().vf_extract()
    }

    /// inserts the given value `val` into the field `vf`
    pub fn vf_insert(&mut self, val: u64) -> &mut Self {
        // bits 6..6
        self.0.set_bits(6..=6, val);
        self
    }

    /// reads the register, updates the `vf` field, and writes the updated value
    pub fn vf_write(val: u64) {
        Self::with_reg_val().vf_insert(val).write();
    }

    /*
     * Field: amo
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amo_extract(&self) -> u64 {
        // bits 5..5
        self.0.get_bits(5..=5)
    }

    /// reads the current register value and extract field `amo` from it
    pub fn amo_read() -> u64 {
        Self::with_reg_val().amo_extract()
    }

    /// inserts the given value `val` into the field `amo`
    pub fn amo_insert(&mut self, val: u64) -> &mut Self {
        // bits 5..5
        self.0.set_bits(5..=5, val);
        self
    }

    /// reads the register, updates the `amo` field, and writes the updated value
    pub fn amo_write(val: u64) {
        Self::with_reg_val().amo_insert(val).write();
    }

    /*
     * Field: imo
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn imo_extract(&self) -> u64 {
        // bits 4..4
        self.0.get_bits(4..=4)
    }

    /// reads the current register value and extract field `imo` from it
    pub fn imo_read() -> u64 {
        Self::with_reg_val().imo_extract()
    }

    /// inserts the given value `val` into the field `imo`
    pub fn imo_insert(&mut self, val: u64) -> &mut Self {
        // bits 4..4
        self.0.set_bits(4..=4, val);
        self
    }

    /// reads the register, updates the `imo` field, and writes the updated value
    pub fn imo_write(val: u64) {
        Self::with_reg_val().imo_insert(val).write();
    }

    /*
     * Field: fmo
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fmo_extract(&self) -> u64 {
        // bits 3..3
        self.0.get_bits(3..=3)
    }

    /// reads the current register value and extract field `fmo` from it
    pub fn fmo_read() -> u64 {
        Self::with_reg_val().fmo_extract()
    }

    /// inserts the given value `val` into the field `fmo`
    pub fn fmo_insert(&mut self, val: u64) -> &mut Self {
        // bits 3..3
        self.0.set_bits(3..=3, val);
        self
    }

    /// reads the register, updates the `fmo` field, and writes the updated value
    pub fn fmo_write(val: u64) {
        Self::with_reg_val().fmo_insert(val).write();
    }

    /*
     * Field: ptw
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ptw_extract(&self) -> u64 {
        // bits 2..2
        self.0.get_bits(2..=2)
    }

    /// reads the current register value and extract field `ptw` from it
    pub fn ptw_read() -> u64 {
        Self::with_reg_val().ptw_extract()
    }

    /// inserts the given value `val` into the field `ptw`
    pub fn ptw_insert(&mut self, val: u64) -> &mut Self {
        // bits 2..2
        self.0.set_bits(2..=2, val);
        self
    }

    /// reads the register, updates the `ptw` field, and writes the updated value
    pub fn ptw_write(val: u64) {
        Self::with_reg_val().ptw_insert(val).write();
    }

    /*
     * Field: swio
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn swio_extract(&self) -> u64 {
        // bits 1..1
        self.0.get_bits(1..=1)
    }

    /// reads the current register value and extract field `swio` from it
    pub fn swio_read() -> u64 {
        Self::with_reg_val().swio_extract()
    }

    /// inserts the given value `val` into the field `swio`
    pub fn swio_insert(&mut self, val: u64) -> &mut Self {
        // bits 1..1
        self.0.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `swio` field, and writes the updated value
    pub fn swio_write(val: u64) {
        Self::with_reg_val().swio_insert(val).write();
    }

    /*
     * Field: vm
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn vm_extract(&self) -> u64 {
        // bits 0..0
        self.0.get_bits(0..=0)
    }

    /// reads the current register value and extract field `vm` from it
    pub fn vm_read() -> u64 {
        Self::with_reg_val().vm_extract()
    }

    /// inserts the given value `val` into the field `vm`
    pub fn vm_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..0
        self.0.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `vm` field, and writes the updated value
    pub fn vm_write(val: u64) {
        Self::with_reg_val().vm_insert(val).write();
    }
}

impl Default for HcrEl2 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> HcrEl2 {
        HcrEl2(0)
    }
}
