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
 * Generated on: 2022-08-22T16:35:53.059032
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
 * Register:    Hypervisor Debug Fine-Grained Read Trap Register (hdfgrtr_el2)
 * Group:       A group mapping that does not have a known primary
 * Type:        64-bit Register
 * Description: Provides controls for traps of
 * File:        AArch64-hdfgrtr_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Hypervisor Debug Fine-Grained Read Trap Register value in memory
pub struct HdfgrtrEl2(u64);

/// struct implementation for accessing the fields of register hdfgrtr_el2
impl HdfgrtrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> HdfgrtrEl2 {
        Self::default()
    }

    /// collects the modifications of HdfgrtrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> HdfgrtrEl2 {
        HdfgrtrEl2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> HdfgrtrEl2 {
        let curval = Self::reg_rawrd() & 0x4601fb3fffcffeff;
        HdfgrtrEl2(curval)
    }

    /// reading the Hypervisor Debug Fine-Grained Read Trap Register (hdfgrtr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, HDFGRTR_EL2
            asm!("mrs {}, S3_4_C3_C1_4", out(reg) regval);
        }
        return regval;
    }

    /// writing the Hypervisor Debug Fine-Grained Read Trap Register (hdfgrtr_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR HDFGRTR_EL2, <Xt>
            asm!("msr S3_4_C3_C1_4, {}", in(reg) val);
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x4601fb3fffcffeff;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 5044589309924998911;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: npmsnevfr_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn npmsnevfr_el1_1_extract(&self) -> u64 {
        // bits 62..62
        self.0.get_bits(62..=62)
    }

    /// reads the current register value and extract field `npmsnevfr_el1_1` from it
    pub fn npmsnevfr_el1_1_read() -> u64 {
        Self::with_reg_val().npmsnevfr_el1_1_extract()
    }

    /// inserts the given value `val` into the field `npmsnevfr_el1_1`
    pub fn npmsnevfr_el1_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 62..62
        self.0.set_bits(62..=62, val);
        self
    }

    /// reads the register, updates the `npmsnevfr_el1_1` field, and writes the updated value
    pub fn npmsnevfr_el1_1_write(val: u64) {
        Self::with_reg_val().npmsnevfr_el1_1_insert(val).write();
    }

    /*
     * Field: pmceidn_el0_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pmceidn_el0_1_extract(&self) -> u64 {
        // bits 58..58
        self.0.get_bits(58..=58)
    }

    /// reads the current register value and extract field `pmceidn_el0_1` from it
    pub fn pmceidn_el0_1_read() -> u64 {
        Self::with_reg_val().pmceidn_el0_1_extract()
    }

    /// inserts the given value `val` into the field `pmceidn_el0_1`
    pub fn pmceidn_el0_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 58..58
        self.0.set_bits(58..=58, val);
        self
    }

    /// reads the register, updates the `pmceidn_el0_1` field, and writes the updated value
    pub fn pmceidn_el0_1_write(val: u64) {
        Self::with_reg_val().pmceidn_el0_1_insert(val).write();
    }

    /*
     * Field: pmuserenr_el0_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pmuserenr_el0_1_extract(&self) -> u64 {
        // bits 57..57
        self.0.get_bits(57..=57)
    }

    /// reads the current register value and extract field `pmuserenr_el0_1` from it
    pub fn pmuserenr_el0_1_read() -> u64 {
        Self::with_reg_val().pmuserenr_el0_1_extract()
    }

    /// inserts the given value `val` into the field `pmuserenr_el0_1`
    pub fn pmuserenr_el0_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 57..57
        self.0.set_bits(57..=57, val);
        self
    }

    /// reads the register, updates the `pmuserenr_el0_1` field, and writes the updated value
    pub fn pmuserenr_el0_1_write(val: u64) {
        Self::with_reg_val().pmuserenr_el0_1_insert(val).write();
    }

    /*
     * Field: trcvictlr_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn trcvictlr_1_extract(&self) -> u64 {
        // bits 48..48
        self.0.get_bits(48..=48)
    }

    /// reads the current register value and extract field `trcvictlr_1` from it
    pub fn trcvictlr_1_read() -> u64 {
        Self::with_reg_val().trcvictlr_1_extract()
    }

    /// inserts the given value `val` into the field `trcvictlr_1`
    pub fn trcvictlr_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 48..48
        self.0.set_bits(48..=48, val);
        self
    }

    /// reads the register, updates the `trcvictlr_1` field, and writes the updated value
    pub fn trcvictlr_1_write(val: u64) {
        Self::with_reg_val().trcvictlr_1_insert(val).write();
    }

    /*
     * Field: trcstatr_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn trcstatr_1_extract(&self) -> u64 {
        // bits 47..47
        self.0.get_bits(47..=47)
    }

    /// reads the current register value and extract field `trcstatr_1` from it
    pub fn trcstatr_1_read() -> u64 {
        Self::with_reg_val().trcstatr_1_extract()
    }

    /// inserts the given value `val` into the field `trcstatr_1`
    pub fn trcstatr_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 47..47
        self.0.set_bits(47..=47, val);
        self
    }

    /// reads the register, updates the `trcstatr_1` field, and writes the updated value
    pub fn trcstatr_1_write(val: u64) {
        Self::with_reg_val().trcstatr_1_insert(val).write();
    }

    /*
     * Field: trcsscsrn_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn trcsscsrn_1_extract(&self) -> u64 {
        // bits 46..46
        self.0.get_bits(46..=46)
    }

    /// reads the current register value and extract field `trcsscsrn_1` from it
    pub fn trcsscsrn_1_read() -> u64 {
        Self::with_reg_val().trcsscsrn_1_extract()
    }

    /// inserts the given value `val` into the field `trcsscsrn_1`
    pub fn trcsscsrn_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 46..46
        self.0.set_bits(46..=46, val);
        self
    }

    /// reads the register, updates the `trcsscsrn_1` field, and writes the updated value
    pub fn trcsscsrn_1_write(val: u64) {
        Self::with_reg_val().trcsscsrn_1_insert(val).write();
    }

    /*
     * Field: trcseqstr_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn trcseqstr_1_extract(&self) -> u64 {
        // bits 45..45
        self.0.get_bits(45..=45)
    }

    /// reads the current register value and extract field `trcseqstr_1` from it
    pub fn trcseqstr_1_read() -> u64 {
        Self::with_reg_val().trcseqstr_1_extract()
    }

    /// inserts the given value `val` into the field `trcseqstr_1`
    pub fn trcseqstr_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 45..45
        self.0.set_bits(45..=45, val);
        self
    }

    /// reads the register, updates the `trcseqstr_1` field, and writes the updated value
    pub fn trcseqstr_1_write(val: u64) {
        Self::with_reg_val().trcseqstr_1_insert(val).write();
    }

    /*
     * Field: trcprgctlr_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn trcprgctlr_1_extract(&self) -> u64 {
        // bits 44..44
        self.0.get_bits(44..=44)
    }

    /// reads the current register value and extract field `trcprgctlr_1` from it
    pub fn trcprgctlr_1_read() -> u64 {
        Self::with_reg_val().trcprgctlr_1_extract()
    }

    /// inserts the given value `val` into the field `trcprgctlr_1`
    pub fn trcprgctlr_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 44..44
        self.0.set_bits(44..=44, val);
        self
    }

    /// reads the register, updates the `trcprgctlr_1` field, and writes the updated value
    pub fn trcprgctlr_1_write(val: u64) {
        Self::with_reg_val().trcprgctlr_1_insert(val).write();
    }

    /*
     * Field: trcoslsr_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn trcoslsr_1_extract(&self) -> u64 {
        // bits 43..43
        self.0.get_bits(43..=43)
    }

    /// reads the current register value and extract field `trcoslsr_1` from it
    pub fn trcoslsr_1_read() -> u64 {
        Self::with_reg_val().trcoslsr_1_extract()
    }

    /// inserts the given value `val` into the field `trcoslsr_1`
    pub fn trcoslsr_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 43..43
        self.0.set_bits(43..=43, val);
        self
    }

    /// reads the register, updates the `trcoslsr_1` field, and writes the updated value
    pub fn trcoslsr_1_write(val: u64) {
        Self::with_reg_val().trcoslsr_1_insert(val).write();
    }

    /*
     * Field: trcimspecn_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn trcimspecn_1_extract(&self) -> u64 {
        // bits 41..41
        self.0.get_bits(41..=41)
    }

    /// reads the current register value and extract field `trcimspecn_1` from it
    pub fn trcimspecn_1_read() -> u64 {
        Self::with_reg_val().trcimspecn_1_extract()
    }

    /// inserts the given value `val` into the field `trcimspecn_1`
    pub fn trcimspecn_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 41..41
        self.0.set_bits(41..=41, val);
        self
    }

    /// reads the register, updates the `trcimspecn_1` field, and writes the updated value
    pub fn trcimspecn_1_write(val: u64) {
        Self::with_reg_val().trcimspecn_1_insert(val).write();
    }

    /*
     * Field: trcid_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn trcid_1_extract(&self) -> u64 {
        // bits 40..40
        self.0.get_bits(40..=40)
    }

    /// reads the current register value and extract field `trcid_1` from it
    pub fn trcid_1_read() -> u64 {
        Self::with_reg_val().trcid_1_extract()
    }

    /// inserts the given value `val` into the field `trcid_1`
    pub fn trcid_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 40..40
        self.0.set_bits(40..=40, val);
        self
    }

    /// reads the register, updates the `trcid_1` field, and writes the updated value
    pub fn trcid_1_write(val: u64) {
        Self::with_reg_val().trcid_1_insert(val).write();
    }

    /*
     * Field: trccntvrn_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn trccntvrn_1_extract(&self) -> u64 {
        // bits 37..37
        self.0.get_bits(37..=37)
    }

    /// reads the current register value and extract field `trccntvrn_1` from it
    pub fn trccntvrn_1_read() -> u64 {
        Self::with_reg_val().trccntvrn_1_extract()
    }

    /// inserts the given value `val` into the field `trccntvrn_1`
    pub fn trccntvrn_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 37..37
        self.0.set_bits(37..=37, val);
        self
    }

    /// reads the register, updates the `trccntvrn_1` field, and writes the updated value
    pub fn trccntvrn_1_write(val: u64) {
        Self::with_reg_val().trccntvrn_1_insert(val).write();
    }

    /*
     * Field: trcclaim_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn trcclaim_1_extract(&self) -> u64 {
        // bits 36..36
        self.0.get_bits(36..=36)
    }

    /// reads the current register value and extract field `trcclaim_1` from it
    pub fn trcclaim_1_read() -> u64 {
        Self::with_reg_val().trcclaim_1_extract()
    }

    /// inserts the given value `val` into the field `trcclaim_1`
    pub fn trcclaim_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 36..36
        self.0.set_bits(36..=36, val);
        self
    }

    /// reads the register, updates the `trcclaim_1` field, and writes the updated value
    pub fn trcclaim_1_write(val: u64) {
        Self::with_reg_val().trcclaim_1_insert(val).write();
    }

    /*
     * Field: trcauxctlr_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn trcauxctlr_1_extract(&self) -> u64 {
        // bits 35..35
        self.0.get_bits(35..=35)
    }

    /// reads the current register value and extract field `trcauxctlr_1` from it
    pub fn trcauxctlr_1_read() -> u64 {
        Self::with_reg_val().trcauxctlr_1_extract()
    }

    /// inserts the given value `val` into the field `trcauxctlr_1`
    pub fn trcauxctlr_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 35..35
        self.0.set_bits(35..=35, val);
        self
    }

    /// reads the register, updates the `trcauxctlr_1` field, and writes the updated value
    pub fn trcauxctlr_1_write(val: u64) {
        Self::with_reg_val().trcauxctlr_1_insert(val).write();
    }

    /*
     * Field: trcauthstatus_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn trcauthstatus_1_extract(&self) -> u64 {
        // bits 34..34
        self.0.get_bits(34..=34)
    }

    /// reads the current register value and extract field `trcauthstatus_1` from it
    pub fn trcauthstatus_1_read() -> u64 {
        Self::with_reg_val().trcauthstatus_1_extract()
    }

    /// inserts the given value `val` into the field `trcauthstatus_1`
    pub fn trcauthstatus_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 34..34
        self.0.set_bits(34..=34, val);
        self
    }

    /// reads the register, updates the `trcauthstatus_1` field, and writes the updated value
    pub fn trcauthstatus_1_write(val: u64) {
        Self::with_reg_val().trcauthstatus_1_insert(val).write();
    }

    /*
     * Field: trc_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn trc_1_extract(&self) -> u64 {
        // bits 33..33
        self.0.get_bits(33..=33)
    }

    /// reads the current register value and extract field `trc_1` from it
    pub fn trc_1_read() -> u64 {
        Self::with_reg_val().trc_1_extract()
    }

    /// inserts the given value `val` into the field `trc_1`
    pub fn trc_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 33..33
        self.0.set_bits(33..=33, val);
        self
    }

    /// reads the register, updates the `trc_1` field, and writes the updated value
    pub fn trc_1_write(val: u64) {
        Self::with_reg_val().trc_1_insert(val).write();
    }

    /*
     * Field: pmslatfr_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pmslatfr_el1_1_extract(&self) -> u64 {
        // bits 32..32
        self.0.get_bits(32..=32)
    }

    /// reads the current register value and extract field `pmslatfr_el1_1` from it
    pub fn pmslatfr_el1_1_read() -> u64 {
        Self::with_reg_val().pmslatfr_el1_1_extract()
    }

    /// inserts the given value `val` into the field `pmslatfr_el1_1`
    pub fn pmslatfr_el1_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 32..32
        self.0.set_bits(32..=32, val);
        self
    }

    /// reads the register, updates the `pmslatfr_el1_1` field, and writes the updated value
    pub fn pmslatfr_el1_1_write(val: u64) {
        Self::with_reg_val().pmslatfr_el1_1_insert(val).write();
    }

    /*
     * Field: pmsirr_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pmsirr_el1_1_extract(&self) -> u64 {
        // bits 31..31
        self.0.get_bits(31..=31)
    }

    /// reads the current register value and extract field `pmsirr_el1_1` from it
    pub fn pmsirr_el1_1_read() -> u64 {
        Self::with_reg_val().pmsirr_el1_1_extract()
    }

    /// inserts the given value `val` into the field `pmsirr_el1_1`
    pub fn pmsirr_el1_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 31..31
        self.0.set_bits(31..=31, val);
        self
    }

    /// reads the register, updates the `pmsirr_el1_1` field, and writes the updated value
    pub fn pmsirr_el1_1_write(val: u64) {
        Self::with_reg_val().pmsirr_el1_1_insert(val).write();
    }

    /*
     * Field: pmsidr_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pmsidr_el1_1_extract(&self) -> u64 {
        // bits 30..30
        self.0.get_bits(30..=30)
    }

    /// reads the current register value and extract field `pmsidr_el1_1` from it
    pub fn pmsidr_el1_1_read() -> u64 {
        Self::with_reg_val().pmsidr_el1_1_extract()
    }

    /// inserts the given value `val` into the field `pmsidr_el1_1`
    pub fn pmsidr_el1_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 30..30
        self.0.set_bits(30..=30, val);
        self
    }

    /// reads the register, updates the `pmsidr_el1_1` field, and writes the updated value
    pub fn pmsidr_el1_1_write(val: u64) {
        Self::with_reg_val().pmsidr_el1_1_insert(val).write();
    }

    /*
     * Field: pmsicr_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pmsicr_el1_1_extract(&self) -> u64 {
        // bits 29..29
        self.0.get_bits(29..=29)
    }

    /// reads the current register value and extract field `pmsicr_el1_1` from it
    pub fn pmsicr_el1_1_read() -> u64 {
        Self::with_reg_val().pmsicr_el1_1_extract()
    }

    /// inserts the given value `val` into the field `pmsicr_el1_1`
    pub fn pmsicr_el1_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 29..29
        self.0.set_bits(29..=29, val);
        self
    }

    /// reads the register, updates the `pmsicr_el1_1` field, and writes the updated value
    pub fn pmsicr_el1_1_write(val: u64) {
        Self::with_reg_val().pmsicr_el1_1_insert(val).write();
    }

    /*
     * Field: pmsfcr_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pmsfcr_el1_1_extract(&self) -> u64 {
        // bits 28..28
        self.0.get_bits(28..=28)
    }

    /// reads the current register value and extract field `pmsfcr_el1_1` from it
    pub fn pmsfcr_el1_1_read() -> u64 {
        Self::with_reg_val().pmsfcr_el1_1_extract()
    }

    /// inserts the given value `val` into the field `pmsfcr_el1_1`
    pub fn pmsfcr_el1_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 28..28
        self.0.set_bits(28..=28, val);
        self
    }

    /// reads the register, updates the `pmsfcr_el1_1` field, and writes the updated value
    pub fn pmsfcr_el1_1_write(val: u64) {
        Self::with_reg_val().pmsfcr_el1_1_insert(val).write();
    }

    /*
     * Field: pmsevfr_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pmsevfr_el1_1_extract(&self) -> u64 {
        // bits 27..27
        self.0.get_bits(27..=27)
    }

    /// reads the current register value and extract field `pmsevfr_el1_1` from it
    pub fn pmsevfr_el1_1_read() -> u64 {
        Self::with_reg_val().pmsevfr_el1_1_extract()
    }

    /// inserts the given value `val` into the field `pmsevfr_el1_1`
    pub fn pmsevfr_el1_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 27..27
        self.0.set_bits(27..=27, val);
        self
    }

    /// reads the register, updates the `pmsevfr_el1_1` field, and writes the updated value
    pub fn pmsevfr_el1_1_write(val: u64) {
        Self::with_reg_val().pmsevfr_el1_1_insert(val).write();
    }

    /*
     * Field: pmscr_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pmscr_el1_1_extract(&self) -> u64 {
        // bits 26..26
        self.0.get_bits(26..=26)
    }

    /// reads the current register value and extract field `pmscr_el1_1` from it
    pub fn pmscr_el1_1_read() -> u64 {
        Self::with_reg_val().pmscr_el1_1_extract()
    }

    /// inserts the given value `val` into the field `pmscr_el1_1`
    pub fn pmscr_el1_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 26..26
        self.0.set_bits(26..=26, val);
        self
    }

    /// reads the register, updates the `pmscr_el1_1` field, and writes the updated value
    pub fn pmscr_el1_1_write(val: u64) {
        Self::with_reg_val().pmscr_el1_1_insert(val).write();
    }

    /*
     * Field: pmbsr_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pmbsr_el1_1_extract(&self) -> u64 {
        // bits 25..25
        self.0.get_bits(25..=25)
    }

    /// reads the current register value and extract field `pmbsr_el1_1` from it
    pub fn pmbsr_el1_1_read() -> u64 {
        Self::with_reg_val().pmbsr_el1_1_extract()
    }

    /// inserts the given value `val` into the field `pmbsr_el1_1`
    pub fn pmbsr_el1_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 25..25
        self.0.set_bits(25..=25, val);
        self
    }

    /// reads the register, updates the `pmbsr_el1_1` field, and writes the updated value
    pub fn pmbsr_el1_1_write(val: u64) {
        Self::with_reg_val().pmbsr_el1_1_insert(val).write();
    }

    /*
     * Field: pmbptr_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pmbptr_el1_1_extract(&self) -> u64 {
        // bits 24..24
        self.0.get_bits(24..=24)
    }

    /// reads the current register value and extract field `pmbptr_el1_1` from it
    pub fn pmbptr_el1_1_read() -> u64 {
        Self::with_reg_val().pmbptr_el1_1_extract()
    }

    /// inserts the given value `val` into the field `pmbptr_el1_1`
    pub fn pmbptr_el1_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 24..24
        self.0.set_bits(24..=24, val);
        self
    }

    /// reads the register, updates the `pmbptr_el1_1` field, and writes the updated value
    pub fn pmbptr_el1_1_write(val: u64) {
        Self::with_reg_val().pmbptr_el1_1_insert(val).write();
    }

    /*
     * Field: pmblimitr_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pmblimitr_el1_1_extract(&self) -> u64 {
        // bits 23..23
        self.0.get_bits(23..=23)
    }

    /// reads the current register value and extract field `pmblimitr_el1_1` from it
    pub fn pmblimitr_el1_1_read() -> u64 {
        Self::with_reg_val().pmblimitr_el1_1_extract()
    }

    /// inserts the given value `val` into the field `pmblimitr_el1_1`
    pub fn pmblimitr_el1_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 23..23
        self.0.set_bits(23..=23, val);
        self
    }

    /// reads the register, updates the `pmblimitr_el1_1` field, and writes the updated value
    pub fn pmblimitr_el1_1_write(val: u64) {
        Self::with_reg_val().pmblimitr_el1_1_insert(val).write();
    }

    /*
     * Field: pmmir_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pmmir_el1_1_extract(&self) -> u64 {
        // bits 22..22
        self.0.get_bits(22..=22)
    }

    /// reads the current register value and extract field `pmmir_el1_1` from it
    pub fn pmmir_el1_1_read() -> u64 {
        Self::with_reg_val().pmmir_el1_1_extract()
    }

    /// inserts the given value `val` into the field `pmmir_el1_1`
    pub fn pmmir_el1_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 22..22
        self.0.set_bits(22..=22, val);
        self
    }

    /// reads the register, updates the `pmmir_el1_1` field, and writes the updated value
    pub fn pmmir_el1_1_write(val: u64) {
        Self::with_reg_val().pmmir_el1_1_insert(val).write();
    }

    /*
     * Field: pmselr_el0_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pmselr_el0_1_extract(&self) -> u64 {
        // bits 19..19
        self.0.get_bits(19..=19)
    }

    /// reads the current register value and extract field `pmselr_el0_1` from it
    pub fn pmselr_el0_1_read() -> u64 {
        Self::with_reg_val().pmselr_el0_1_extract()
    }

    /// inserts the given value `val` into the field `pmselr_el0_1`
    pub fn pmselr_el0_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 19..19
        self.0.set_bits(19..=19, val);
        self
    }

    /// reads the register, updates the `pmselr_el0_1` field, and writes the updated value
    pub fn pmselr_el0_1_write(val: u64) {
        Self::with_reg_val().pmselr_el0_1_insert(val).write();
    }

    /*
     * Field: pmovs_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pmovs_1_extract(&self) -> u64 {
        // bits 18..18
        self.0.get_bits(18..=18)
    }

    /// reads the current register value and extract field `pmovs_1` from it
    pub fn pmovs_1_read() -> u64 {
        Self::with_reg_val().pmovs_1_extract()
    }

    /// inserts the given value `val` into the field `pmovs_1`
    pub fn pmovs_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 18..18
        self.0.set_bits(18..=18, val);
        self
    }

    /// reads the register, updates the `pmovs_1` field, and writes the updated value
    pub fn pmovs_1_write(val: u64) {
        Self::with_reg_val().pmovs_1_insert(val).write();
    }

    /*
     * Field: pminten_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pminten_1_extract(&self) -> u64 {
        // bits 17..17
        self.0.get_bits(17..=17)
    }

    /// reads the current register value and extract field `pminten_1` from it
    pub fn pminten_1_read() -> u64 {
        Self::with_reg_val().pminten_1_extract()
    }

    /// inserts the given value `val` into the field `pminten_1`
    pub fn pminten_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 17..17
        self.0.set_bits(17..=17, val);
        self
    }

    /// reads the register, updates the `pminten_1` field, and writes the updated value
    pub fn pminten_1_write(val: u64) {
        Self::with_reg_val().pminten_1_insert(val).write();
    }

    /*
     * Field: pmcnten_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pmcnten_1_extract(&self) -> u64 {
        // bits 16..16
        self.0.get_bits(16..=16)
    }

    /// reads the current register value and extract field `pmcnten_1` from it
    pub fn pmcnten_1_read() -> u64 {
        Self::with_reg_val().pmcnten_1_extract()
    }

    /// inserts the given value `val` into the field `pmcnten_1`
    pub fn pmcnten_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 16..16
        self.0.set_bits(16..=16, val);
        self
    }

    /// reads the register, updates the `pmcnten_1` field, and writes the updated value
    pub fn pmcnten_1_write(val: u64) {
        Self::with_reg_val().pmcnten_1_insert(val).write();
    }

    /*
     * Field: pmccntr_el0_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pmccntr_el0_1_extract(&self) -> u64 {
        // bits 15..15
        self.0.get_bits(15..=15)
    }

    /// reads the current register value and extract field `pmccntr_el0_1` from it
    pub fn pmccntr_el0_1_read() -> u64 {
        Self::with_reg_val().pmccntr_el0_1_extract()
    }

    /// inserts the given value `val` into the field `pmccntr_el0_1`
    pub fn pmccntr_el0_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 15..15
        self.0.set_bits(15..=15, val);
        self
    }

    /// reads the register, updates the `pmccntr_el0_1` field, and writes the updated value
    pub fn pmccntr_el0_1_write(val: u64) {
        Self::with_reg_val().pmccntr_el0_1_insert(val).write();
    }

    /*
     * Field: pmccfiltr_el0_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pmccfiltr_el0_1_extract(&self) -> u64 {
        // bits 14..14
        self.0.get_bits(14..=14)
    }

    /// reads the current register value and extract field `pmccfiltr_el0_1` from it
    pub fn pmccfiltr_el0_1_read() -> u64 {
        Self::with_reg_val().pmccfiltr_el0_1_extract()
    }

    /// inserts the given value `val` into the field `pmccfiltr_el0_1`
    pub fn pmccfiltr_el0_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 14..14
        self.0.set_bits(14..=14, val);
        self
    }

    /// reads the register, updates the `pmccfiltr_el0_1` field, and writes the updated value
    pub fn pmccfiltr_el0_1_write(val: u64) {
        Self::with_reg_val().pmccfiltr_el0_1_insert(val).write();
    }

    /*
     * Field: pmevtypern_el0_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pmevtypern_el0_1_extract(&self) -> u64 {
        // bits 13..13
        self.0.get_bits(13..=13)
    }

    /// reads the current register value and extract field `pmevtypern_el0_1` from it
    pub fn pmevtypern_el0_1_read() -> u64 {
        Self::with_reg_val().pmevtypern_el0_1_extract()
    }

    /// inserts the given value `val` into the field `pmevtypern_el0_1`
    pub fn pmevtypern_el0_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 13..13
        self.0.set_bits(13..=13, val);
        self
    }

    /// reads the register, updates the `pmevtypern_el0_1` field, and writes the updated value
    pub fn pmevtypern_el0_1_write(val: u64) {
        Self::with_reg_val().pmevtypern_el0_1_insert(val).write();
    }

    /*
     * Field: pmevcntrn_el0_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pmevcntrn_el0_1_extract(&self) -> u64 {
        // bits 12..12
        self.0.get_bits(12..=12)
    }

    /// reads the current register value and extract field `pmevcntrn_el0_1` from it
    pub fn pmevcntrn_el0_1_read() -> u64 {
        Self::with_reg_val().pmevcntrn_el0_1_extract()
    }

    /// inserts the given value `val` into the field `pmevcntrn_el0_1`
    pub fn pmevcntrn_el0_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 12..12
        self.0.set_bits(12..=12, val);
        self
    }

    /// reads the register, updates the `pmevcntrn_el0_1` field, and writes the updated value
    pub fn pmevcntrn_el0_1_write(val: u64) {
        Self::with_reg_val().pmevcntrn_el0_1_insert(val).write();
    }

    /*
     * Field: osdlr_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn osdlr_el1_1_extract(&self) -> u64 {
        // bits 11..11
        self.0.get_bits(11..=11)
    }

    /// reads the current register value and extract field `osdlr_el1_1` from it
    pub fn osdlr_el1_1_read() -> u64 {
        Self::with_reg_val().osdlr_el1_1_extract()
    }

    /// inserts the given value `val` into the field `osdlr_el1_1`
    pub fn osdlr_el1_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 11..11
        self.0.set_bits(11..=11, val);
        self
    }

    /// reads the register, updates the `osdlr_el1_1` field, and writes the updated value
    pub fn osdlr_el1_1_write(val: u64) {
        Self::with_reg_val().osdlr_el1_1_insert(val).write();
    }

    /*
     * Field: oseccr_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn oseccr_el1_extract(&self) -> u64 {
        // bits 10..10
        self.0.get_bits(10..=10)
    }

    /// reads the current register value and extract field `oseccr_el1` from it
    pub fn oseccr_el1_read() -> u64 {
        Self::with_reg_val().oseccr_el1_extract()
    }

    /// inserts the given value `val` into the field `oseccr_el1`
    pub fn oseccr_el1_insert(&mut self, val: u64) -> &mut Self {
        // bits 10..10
        self.0.set_bits(10..=10, val);
        self
    }

    /// reads the register, updates the `oseccr_el1` field, and writes the updated value
    pub fn oseccr_el1_write(val: u64) {
        Self::with_reg_val().oseccr_el1_insert(val).write();
    }

    /*
     * Field: oslsr_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn oslsr_el1_extract(&self) -> u64 {
        // bits 9..9
        self.0.get_bits(9..=9)
    }

    /// reads the current register value and extract field `oslsr_el1` from it
    pub fn oslsr_el1_read() -> u64 {
        Self::with_reg_val().oslsr_el1_extract()
    }

    /// inserts the given value `val` into the field `oslsr_el1`
    pub fn oslsr_el1_insert(&mut self, val: u64) -> &mut Self {
        // bits 9..9
        self.0.set_bits(9..=9, val);
        self
    }

    /// reads the register, updates the `oslsr_el1` field, and writes the updated value
    pub fn oslsr_el1_write(val: u64) {
        Self::with_reg_val().oslsr_el1_insert(val).write();
    }

    /*
     * Field: dbgprcr_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dbgprcr_el1_extract(&self) -> u64 {
        // bits 7..7
        self.0.get_bits(7..=7)
    }

    /// reads the current register value and extract field `dbgprcr_el1` from it
    pub fn dbgprcr_el1_read() -> u64 {
        Self::with_reg_val().dbgprcr_el1_extract()
    }

    /// inserts the given value `val` into the field `dbgprcr_el1`
    pub fn dbgprcr_el1_insert(&mut self, val: u64) -> &mut Self {
        // bits 7..7
        self.0.set_bits(7..=7, val);
        self
    }

    /// reads the register, updates the `dbgprcr_el1` field, and writes the updated value
    pub fn dbgprcr_el1_write(val: u64) {
        Self::with_reg_val().dbgprcr_el1_insert(val).write();
    }

    /*
     * Field: dbgauthstatus_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dbgauthstatus_el1_extract(&self) -> u64 {
        // bits 6..6
        self.0.get_bits(6..=6)
    }

    /// reads the current register value and extract field `dbgauthstatus_el1` from it
    pub fn dbgauthstatus_el1_read() -> u64 {
        Self::with_reg_val().dbgauthstatus_el1_extract()
    }

    /// inserts the given value `val` into the field `dbgauthstatus_el1`
    pub fn dbgauthstatus_el1_insert(&mut self, val: u64) -> &mut Self {
        // bits 6..6
        self.0.set_bits(6..=6, val);
        self
    }

    /// reads the register, updates the `dbgauthstatus_el1` field, and writes the updated value
    pub fn dbgauthstatus_el1_write(val: u64) {
        Self::with_reg_val().dbgauthstatus_el1_insert(val).write();
    }

    /*
     * Field: dbgclaim
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dbgclaim_extract(&self) -> u64 {
        // bits 5..5
        self.0.get_bits(5..=5)
    }

    /// reads the current register value and extract field `dbgclaim` from it
    pub fn dbgclaim_read() -> u64 {
        Self::with_reg_val().dbgclaim_extract()
    }

    /// inserts the given value `val` into the field `dbgclaim`
    pub fn dbgclaim_insert(&mut self, val: u64) -> &mut Self {
        // bits 5..5
        self.0.set_bits(5..=5, val);
        self
    }

    /// reads the register, updates the `dbgclaim` field, and writes the updated value
    pub fn dbgclaim_write(val: u64) {
        Self::with_reg_val().dbgclaim_insert(val).write();
    }

    /*
     * Field: mdscr_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn mdscr_el1_extract(&self) -> u64 {
        // bits 4..4
        self.0.get_bits(4..=4)
    }

    /// reads the current register value and extract field `mdscr_el1` from it
    pub fn mdscr_el1_read() -> u64 {
        Self::with_reg_val().mdscr_el1_extract()
    }

    /// inserts the given value `val` into the field `mdscr_el1`
    pub fn mdscr_el1_insert(&mut self, val: u64) -> &mut Self {
        // bits 4..4
        self.0.set_bits(4..=4, val);
        self
    }

    /// reads the register, updates the `mdscr_el1` field, and writes the updated value
    pub fn mdscr_el1_write(val: u64) {
        Self::with_reg_val().mdscr_el1_insert(val).write();
    }

    /*
     * Field: dbgwvrn_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dbgwvrn_el1_extract(&self) -> u64 {
        // bits 3..3
        self.0.get_bits(3..=3)
    }

    /// reads the current register value and extract field `dbgwvrn_el1` from it
    pub fn dbgwvrn_el1_read() -> u64 {
        Self::with_reg_val().dbgwvrn_el1_extract()
    }

    /// inserts the given value `val` into the field `dbgwvrn_el1`
    pub fn dbgwvrn_el1_insert(&mut self, val: u64) -> &mut Self {
        // bits 3..3
        self.0.set_bits(3..=3, val);
        self
    }

    /// reads the register, updates the `dbgwvrn_el1` field, and writes the updated value
    pub fn dbgwvrn_el1_write(val: u64) {
        Self::with_reg_val().dbgwvrn_el1_insert(val).write();
    }

    /*
     * Field: dbgwcrn_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dbgwcrn_el1_extract(&self) -> u64 {
        // bits 2..2
        self.0.get_bits(2..=2)
    }

    /// reads the current register value and extract field `dbgwcrn_el1` from it
    pub fn dbgwcrn_el1_read() -> u64 {
        Self::with_reg_val().dbgwcrn_el1_extract()
    }

    /// inserts the given value `val` into the field `dbgwcrn_el1`
    pub fn dbgwcrn_el1_insert(&mut self, val: u64) -> &mut Self {
        // bits 2..2
        self.0.set_bits(2..=2, val);
        self
    }

    /// reads the register, updates the `dbgwcrn_el1` field, and writes the updated value
    pub fn dbgwcrn_el1_write(val: u64) {
        Self::with_reg_val().dbgwcrn_el1_insert(val).write();
    }

    /*
     * Field: dbgbvrn_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dbgbvrn_el1_extract(&self) -> u64 {
        // bits 1..1
        self.0.get_bits(1..=1)
    }

    /// reads the current register value and extract field `dbgbvrn_el1` from it
    pub fn dbgbvrn_el1_read() -> u64 {
        Self::with_reg_val().dbgbvrn_el1_extract()
    }

    /// inserts the given value `val` into the field `dbgbvrn_el1`
    pub fn dbgbvrn_el1_insert(&mut self, val: u64) -> &mut Self {
        // bits 1..1
        self.0.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `dbgbvrn_el1` field, and writes the updated value
    pub fn dbgbvrn_el1_write(val: u64) {
        Self::with_reg_val().dbgbvrn_el1_insert(val).write();
    }

    /*
     * Field: dbgbcrn_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dbgbcrn_el1_extract(&self) -> u64 {
        // bits 0..0
        self.0.get_bits(0..=0)
    }

    /// reads the current register value and extract field `dbgbcrn_el1` from it
    pub fn dbgbcrn_el1_read() -> u64 {
        Self::with_reg_val().dbgbcrn_el1_extract()
    }

    /// inserts the given value `val` into the field `dbgbcrn_el1`
    pub fn dbgbcrn_el1_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..0
        self.0.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `dbgbcrn_el1` field, and writes the updated value
    pub fn dbgbcrn_el1_write(val: u64) {
        Self::with_reg_val().dbgbcrn_el1_insert(val).write();
    }
}

impl Default for HdfgrtrEl2 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> HdfgrtrEl2 {
        HdfgrtrEl2(0)
    }
}
