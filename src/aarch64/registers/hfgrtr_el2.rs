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
 * Generated on: 2022-08-22T15:51:28.516923
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
 * Register:    Hypervisor Fine-Grained Read Trap Register (hfgrtr_el2)
 * Group:       A group mapping that does not have a known primary
 * Type:        64-bit Register
 * Description: Provides controls for traps of
 * File:        AArch64-hfgrtr_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Hypervisor Fine-Grained Read Trap Register value in memory
pub struct HfgrtrEl2(u64);

/// struct implementation for accessing the fields of register hfgrtr_el2
impl HfgrtrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> HfgrtrEl2 {
        Self::default()
    }

    /// collects the modifications of HfgrtrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> HfgrtrEl2 {
        HfgrtrEl2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> HfgrtrEl2 {
        let curval = Self::reg_rawrd() & 0x7ffffffffffff;
        HfgrtrEl2(curval)
    }

    /// reading the Hypervisor Fine-Grained Read Trap Register (hfgrtr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, HFGRTR_EL2
            llvm_asm!("mrs $0, S3_4_C1_C1_4" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Hypervisor Fine-Grained Read Trap Register (hfgrtr_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR HFGRTR_EL2, <Xt>
            llvm_asm!("msr S3_4_C1_C1_4, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0x7ffffffffffff;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 2251799813685247;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: naccdata_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn naccdata_el1_1_extract(&self) -> u64 {
        // bits 50..50
        self.val.get_bits(50..=50)
    }

    /// reads the current register value and extract field `naccdata_el1_1` from it
    pub fn naccdata_el1_1_read(&mut self) -> u64 {
        Self::with_reg_val().naccdata_el1_1_extract()
    }

    /// inserts the given value `val` into the field `naccdata_el1_1`
    pub fn naccdata_el1_1_insert(&mut self, val: u64) -> &mut self {
        // bits 50..50
        self.val.set_bits(50..=50, val);
        self
    }

    /// reads the register, updates the `naccdata_el1_1` field, and writes the updated value
    pub fn naccdata_el1_1_write(&mut self, val: u64) {
        Self::with_reg_val().naccdata_el1_1_insert(val).write();
    }

    /*
     * Field: erxaddr_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn erxaddr_el1_1_extract(&self) -> u64 {
        // bits 49..49
        self.val.get_bits(49..=49)
    }

    /// reads the current register value and extract field `erxaddr_el1_1` from it
    pub fn erxaddr_el1_1_read(&mut self) -> u64 {
        Self::with_reg_val().erxaddr_el1_1_extract()
    }

    /// inserts the given value `val` into the field `erxaddr_el1_1`
    pub fn erxaddr_el1_1_insert(&mut self, val: u64) -> &mut self {
        // bits 49..49
        self.val.set_bits(49..=49, val);
        self
    }

    /// reads the register, updates the `erxaddr_el1_1` field, and writes the updated value
    pub fn erxaddr_el1_1_write(&mut self, val: u64) {
        Self::with_reg_val().erxaddr_el1_1_insert(val).write();
    }

    /*
     * Field: erxpfgcdn_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn erxpfgcdn_el1_1_extract(&self) -> u64 {
        // bits 48..48
        self.val.get_bits(48..=48)
    }

    /// reads the current register value and extract field `erxpfgcdn_el1_1` from it
    pub fn erxpfgcdn_el1_1_read(&mut self) -> u64 {
        Self::with_reg_val().erxpfgcdn_el1_1_extract()
    }

    /// inserts the given value `val` into the field `erxpfgcdn_el1_1`
    pub fn erxpfgcdn_el1_1_insert(&mut self, val: u64) -> &mut self {
        // bits 48..48
        self.val.set_bits(48..=48, val);
        self
    }

    /// reads the register, updates the `erxpfgcdn_el1_1` field, and writes the updated value
    pub fn erxpfgcdn_el1_1_write(&mut self, val: u64) {
        Self::with_reg_val().erxpfgcdn_el1_1_insert(val).write();
    }

    /*
     * Field: erxpfgctl_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn erxpfgctl_el1_1_extract(&self) -> u64 {
        // bits 47..47
        self.val.get_bits(47..=47)
    }

    /// reads the current register value and extract field `erxpfgctl_el1_1` from it
    pub fn erxpfgctl_el1_1_read(&mut self) -> u64 {
        Self::with_reg_val().erxpfgctl_el1_1_extract()
    }

    /// inserts the given value `val` into the field `erxpfgctl_el1_1`
    pub fn erxpfgctl_el1_1_insert(&mut self, val: u64) -> &mut self {
        // bits 47..47
        self.val.set_bits(47..=47, val);
        self
    }

    /// reads the register, updates the `erxpfgctl_el1_1` field, and writes the updated value
    pub fn erxpfgctl_el1_1_write(&mut self, val: u64) {
        Self::with_reg_val().erxpfgctl_el1_1_insert(val).write();
    }

    /*
     * Field: erxpfgf_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn erxpfgf_el1_1_extract(&self) -> u64 {
        // bits 46..46
        self.val.get_bits(46..=46)
    }

    /// reads the current register value and extract field `erxpfgf_el1_1` from it
    pub fn erxpfgf_el1_1_read(&mut self) -> u64 {
        Self::with_reg_val().erxpfgf_el1_1_extract()
    }

    /// inserts the given value `val` into the field `erxpfgf_el1_1`
    pub fn erxpfgf_el1_1_insert(&mut self, val: u64) -> &mut self {
        // bits 46..46
        self.val.set_bits(46..=46, val);
        self
    }

    /// reads the register, updates the `erxpfgf_el1_1` field, and writes the updated value
    pub fn erxpfgf_el1_1_write(&mut self, val: u64) {
        Self::with_reg_val().erxpfgf_el1_1_insert(val).write();
    }

    /*
     * Field: erxmiscn_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn erxmiscn_el1_1_extract(&self) -> u64 {
        // bits 45..45
        self.val.get_bits(45..=45)
    }

    /// reads the current register value and extract field `erxmiscn_el1_1` from it
    pub fn erxmiscn_el1_1_read(&mut self) -> u64 {
        Self::with_reg_val().erxmiscn_el1_1_extract()
    }

    /// inserts the given value `val` into the field `erxmiscn_el1_1`
    pub fn erxmiscn_el1_1_insert(&mut self, val: u64) -> &mut self {
        // bits 45..45
        self.val.set_bits(45..=45, val);
        self
    }

    /// reads the register, updates the `erxmiscn_el1_1` field, and writes the updated value
    pub fn erxmiscn_el1_1_write(&mut self, val: u64) {
        Self::with_reg_val().erxmiscn_el1_1_insert(val).write();
    }

    /*
     * Field: erxstatus_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn erxstatus_el1_1_extract(&self) -> u64 {
        // bits 44..44
        self.val.get_bits(44..=44)
    }

    /// reads the current register value and extract field `erxstatus_el1_1` from it
    pub fn erxstatus_el1_1_read(&mut self) -> u64 {
        Self::with_reg_val().erxstatus_el1_1_extract()
    }

    /// inserts the given value `val` into the field `erxstatus_el1_1`
    pub fn erxstatus_el1_1_insert(&mut self, val: u64) -> &mut self {
        // bits 44..44
        self.val.set_bits(44..=44, val);
        self
    }

    /// reads the register, updates the `erxstatus_el1_1` field, and writes the updated value
    pub fn erxstatus_el1_1_write(&mut self, val: u64) {
        Self::with_reg_val().erxstatus_el1_1_insert(val).write();
    }

    /*
     * Field: erxctlr_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn erxctlr_el1_1_extract(&self) -> u64 {
        // bits 43..43
        self.val.get_bits(43..=43)
    }

    /// reads the current register value and extract field `erxctlr_el1_1` from it
    pub fn erxctlr_el1_1_read(&mut self) -> u64 {
        Self::with_reg_val().erxctlr_el1_1_extract()
    }

    /// inserts the given value `val` into the field `erxctlr_el1_1`
    pub fn erxctlr_el1_1_insert(&mut self, val: u64) -> &mut self {
        // bits 43..43
        self.val.set_bits(43..=43, val);
        self
    }

    /// reads the register, updates the `erxctlr_el1_1` field, and writes the updated value
    pub fn erxctlr_el1_1_write(&mut self, val: u64) {
        Self::with_reg_val().erxctlr_el1_1_insert(val).write();
    }

    /*
     * Field: erxfr_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn erxfr_el1_1_extract(&self) -> u64 {
        // bits 42..42
        self.val.get_bits(42..=42)
    }

    /// reads the current register value and extract field `erxfr_el1_1` from it
    pub fn erxfr_el1_1_read(&mut self) -> u64 {
        Self::with_reg_val().erxfr_el1_1_extract()
    }

    /// inserts the given value `val` into the field `erxfr_el1_1`
    pub fn erxfr_el1_1_insert(&mut self, val: u64) -> &mut self {
        // bits 42..42
        self.val.set_bits(42..=42, val);
        self
    }

    /// reads the register, updates the `erxfr_el1_1` field, and writes the updated value
    pub fn erxfr_el1_1_write(&mut self, val: u64) {
        Self::with_reg_val().erxfr_el1_1_insert(val).write();
    }

    /*
     * Field: errselr_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn errselr_el1_1_extract(&self) -> u64 {
        // bits 41..41
        self.val.get_bits(41..=41)
    }

    /// reads the current register value and extract field `errselr_el1_1` from it
    pub fn errselr_el1_1_read(&mut self) -> u64 {
        Self::with_reg_val().errselr_el1_1_extract()
    }

    /// inserts the given value `val` into the field `errselr_el1_1`
    pub fn errselr_el1_1_insert(&mut self, val: u64) -> &mut self {
        // bits 41..41
        self.val.set_bits(41..=41, val);
        self
    }

    /// reads the register, updates the `errselr_el1_1` field, and writes the updated value
    pub fn errselr_el1_1_write(&mut self, val: u64) {
        Self::with_reg_val().errselr_el1_1_insert(val).write();
    }

    /*
     * Field: erridr_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn erridr_el1_1_extract(&self) -> u64 {
        // bits 40..40
        self.val.get_bits(40..=40)
    }

    /// reads the current register value and extract field `erridr_el1_1` from it
    pub fn erridr_el1_1_read(&mut self) -> u64 {
        Self::with_reg_val().erridr_el1_1_extract()
    }

    /// inserts the given value `val` into the field `erridr_el1_1`
    pub fn erridr_el1_1_insert(&mut self, val: u64) -> &mut self {
        // bits 40..40
        self.val.set_bits(40..=40, val);
        self
    }

    /// reads the register, updates the `erridr_el1_1` field, and writes the updated value
    pub fn erridr_el1_1_write(&mut self, val: u64) {
        Self::with_reg_val().erridr_el1_1_insert(val).write();
    }

    /*
     * Field: icc_igrpenn_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn icc_igrpenn_el1_1_extract(&self) -> u64 {
        // bits 39..39
        self.val.get_bits(39..=39)
    }

    /// reads the current register value and extract field `icc_igrpenn_el1_1` from it
    pub fn icc_igrpenn_el1_1_read(&mut self) -> u64 {
        Self::with_reg_val().icc_igrpenn_el1_1_extract()
    }

    /// inserts the given value `val` into the field `icc_igrpenn_el1_1`
    pub fn icc_igrpenn_el1_1_insert(&mut self, val: u64) -> &mut self {
        // bits 39..39
        self.val.set_bits(39..=39, val);
        self
    }

    /// reads the register, updates the `icc_igrpenn_el1_1` field, and writes the updated value
    pub fn icc_igrpenn_el1_1_write(&mut self, val: u64) {
        Self::with_reg_val().icc_igrpenn_el1_1_insert(val).write();
    }

    /*
     * Field: vbar_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn vbar_el1_extract(&self) -> u64 {
        // bits 38..38
        self.val.get_bits(38..=38)
    }

    /// reads the current register value and extract field `vbar_el1` from it
    pub fn vbar_el1_read(&mut self) -> u64 {
        Self::with_reg_val().vbar_el1_extract()
    }

    /// inserts the given value `val` into the field `vbar_el1`
    pub fn vbar_el1_insert(&mut self, val: u64) -> &mut self {
        // bits 38..38
        self.val.set_bits(38..=38, val);
        self
    }

    /// reads the register, updates the `vbar_el1` field, and writes the updated value
    pub fn vbar_el1_write(&mut self, val: u64) {
        Self::with_reg_val().vbar_el1_insert(val).write();
    }

    /*
     * Field: ttbr1_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ttbr1_el1_extract(&self) -> u64 {
        // bits 37..37
        self.val.get_bits(37..=37)
    }

    /// reads the current register value and extract field `ttbr1_el1` from it
    pub fn ttbr1_el1_read(&mut self) -> u64 {
        Self::with_reg_val().ttbr1_el1_extract()
    }

    /// inserts the given value `val` into the field `ttbr1_el1`
    pub fn ttbr1_el1_insert(&mut self, val: u64) -> &mut self {
        // bits 37..37
        self.val.set_bits(37..=37, val);
        self
    }

    /// reads the register, updates the `ttbr1_el1` field, and writes the updated value
    pub fn ttbr1_el1_write(&mut self, val: u64) {
        Self::with_reg_val().ttbr1_el1_insert(val).write();
    }

    /*
     * Field: ttbr0_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ttbr0_el1_extract(&self) -> u64 {
        // bits 36..36
        self.val.get_bits(36..=36)
    }

    /// reads the current register value and extract field `ttbr0_el1` from it
    pub fn ttbr0_el1_read(&mut self) -> u64 {
        Self::with_reg_val().ttbr0_el1_extract()
    }

    /// inserts the given value `val` into the field `ttbr0_el1`
    pub fn ttbr0_el1_insert(&mut self, val: u64) -> &mut self {
        // bits 36..36
        self.val.set_bits(36..=36, val);
        self
    }

    /// reads the register, updates the `ttbr0_el1` field, and writes the updated value
    pub fn ttbr0_el1_write(&mut self, val: u64) {
        Self::with_reg_val().ttbr0_el1_insert(val).write();
    }

    /*
     * Field: tpidr_el0
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tpidr_el0_extract(&self) -> u64 {
        // bits 35..35
        self.val.get_bits(35..=35)
    }

    /// reads the current register value and extract field `tpidr_el0` from it
    pub fn tpidr_el0_read(&mut self) -> u64 {
        Self::with_reg_val().tpidr_el0_extract()
    }

    /// inserts the given value `val` into the field `tpidr_el0`
    pub fn tpidr_el0_insert(&mut self, val: u64) -> &mut self {
        // bits 35..35
        self.val.set_bits(35..=35, val);
        self
    }

    /// reads the register, updates the `tpidr_el0` field, and writes the updated value
    pub fn tpidr_el0_write(&mut self, val: u64) {
        Self::with_reg_val().tpidr_el0_insert(val).write();
    }

    /*
     * Field: tpidrro_el0
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tpidrro_el0_extract(&self) -> u64 {
        // bits 34..34
        self.val.get_bits(34..=34)
    }

    /// reads the current register value and extract field `tpidrro_el0` from it
    pub fn tpidrro_el0_read(&mut self) -> u64 {
        Self::with_reg_val().tpidrro_el0_extract()
    }

    /// inserts the given value `val` into the field `tpidrro_el0`
    pub fn tpidrro_el0_insert(&mut self, val: u64) -> &mut self {
        // bits 34..34
        self.val.set_bits(34..=34, val);
        self
    }

    /// reads the register, updates the `tpidrro_el0` field, and writes the updated value
    pub fn tpidrro_el0_write(&mut self, val: u64) {
        Self::with_reg_val().tpidrro_el0_insert(val).write();
    }

    /*
     * Field: tpidr_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tpidr_el1_extract(&self) -> u64 {
        // bits 33..33
        self.val.get_bits(33..=33)
    }

    /// reads the current register value and extract field `tpidr_el1` from it
    pub fn tpidr_el1_read(&mut self) -> u64 {
        Self::with_reg_val().tpidr_el1_extract()
    }

    /// inserts the given value `val` into the field `tpidr_el1`
    pub fn tpidr_el1_insert(&mut self, val: u64) -> &mut self {
        // bits 33..33
        self.val.set_bits(33..=33, val);
        self
    }

    /// reads the register, updates the `tpidr_el1` field, and writes the updated value
    pub fn tpidr_el1_write(&mut self, val: u64) {
        Self::with_reg_val().tpidr_el1_insert(val).write();
    }

    /*
     * Field: tcr_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tcr_el1_extract(&self) -> u64 {
        // bits 32..32
        self.val.get_bits(32..=32)
    }

    /// reads the current register value and extract field `tcr_el1` from it
    pub fn tcr_el1_read(&mut self) -> u64 {
        Self::with_reg_val().tcr_el1_extract()
    }

    /// inserts the given value `val` into the field `tcr_el1`
    pub fn tcr_el1_insert(&mut self, val: u64) -> &mut self {
        // bits 32..32
        self.val.set_bits(32..=32, val);
        self
    }

    /// reads the register, updates the `tcr_el1` field, and writes the updated value
    pub fn tcr_el1_write(&mut self, val: u64) {
        Self::with_reg_val().tcr_el1_insert(val).write();
    }

    /*
     * Field: scxtnum_el0_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn scxtnum_el0_1_extract(&self) -> u64 {
        // bits 31..31
        self.val.get_bits(31..=31)
    }

    /// reads the current register value and extract field `scxtnum_el0_1` from it
    pub fn scxtnum_el0_1_read(&mut self) -> u64 {
        Self::with_reg_val().scxtnum_el0_1_extract()
    }

    /// inserts the given value `val` into the field `scxtnum_el0_1`
    pub fn scxtnum_el0_1_insert(&mut self, val: u64) -> &mut self {
        // bits 31..31
        self.val.set_bits(31..=31, val);
        self
    }

    /// reads the register, updates the `scxtnum_el0_1` field, and writes the updated value
    pub fn scxtnum_el0_1_write(&mut self, val: u64) {
        Self::with_reg_val().scxtnum_el0_1_insert(val).write();
    }

    /*
     * Field: scxtnum_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn scxtnum_el1_1_extract(&self) -> u64 {
        // bits 30..30
        self.val.get_bits(30..=30)
    }

    /// reads the current register value and extract field `scxtnum_el1_1` from it
    pub fn scxtnum_el1_1_read(&mut self) -> u64 {
        Self::with_reg_val().scxtnum_el1_1_extract()
    }

    /// inserts the given value `val` into the field `scxtnum_el1_1`
    pub fn scxtnum_el1_1_insert(&mut self, val: u64) -> &mut self {
        // bits 30..30
        self.val.set_bits(30..=30, val);
        self
    }

    /// reads the register, updates the `scxtnum_el1_1` field, and writes the updated value
    pub fn scxtnum_el1_1_write(&mut self, val: u64) {
        Self::with_reg_val().scxtnum_el1_1_insert(val).write();
    }

    /*
     * Field: sctlr_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn sctlr_el1_extract(&self) -> u64 {
        // bits 29..29
        self.val.get_bits(29..=29)
    }

    /// reads the current register value and extract field `sctlr_el1` from it
    pub fn sctlr_el1_read(&mut self) -> u64 {
        Self::with_reg_val().sctlr_el1_extract()
    }

    /// inserts the given value `val` into the field `sctlr_el1`
    pub fn sctlr_el1_insert(&mut self, val: u64) -> &mut self {
        // bits 29..29
        self.val.set_bits(29..=29, val);
        self
    }

    /// reads the register, updates the `sctlr_el1` field, and writes the updated value
    pub fn sctlr_el1_write(&mut self, val: u64) {
        Self::with_reg_val().sctlr_el1_insert(val).write();
    }

    /*
     * Field: revidr_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn revidr_el1_extract(&self) -> u64 {
        // bits 28..28
        self.val.get_bits(28..=28)
    }

    /// reads the current register value and extract field `revidr_el1` from it
    pub fn revidr_el1_read(&mut self) -> u64 {
        Self::with_reg_val().revidr_el1_extract()
    }

    /// inserts the given value `val` into the field `revidr_el1`
    pub fn revidr_el1_insert(&mut self, val: u64) -> &mut self {
        // bits 28..28
        self.val.set_bits(28..=28, val);
        self
    }

    /// reads the register, updates the `revidr_el1` field, and writes the updated value
    pub fn revidr_el1_write(&mut self, val: u64) {
        Self::with_reg_val().revidr_el1_insert(val).write();
    }

    /*
     * Field: par_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn par_el1_extract(&self) -> u64 {
        // bits 27..27
        self.val.get_bits(27..=27)
    }

    /// reads the current register value and extract field `par_el1` from it
    pub fn par_el1_read(&mut self) -> u64 {
        Self::with_reg_val().par_el1_extract()
    }

    /// inserts the given value `val` into the field `par_el1`
    pub fn par_el1_insert(&mut self, val: u64) -> &mut self {
        // bits 27..27
        self.val.set_bits(27..=27, val);
        self
    }

    /// reads the register, updates the `par_el1` field, and writes the updated value
    pub fn par_el1_write(&mut self, val: u64) {
        Self::with_reg_val().par_el1_insert(val).write();
    }

    /*
     * Field: mpidr_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn mpidr_el1_extract(&self) -> u64 {
        // bits 26..26
        self.val.get_bits(26..=26)
    }

    /// reads the current register value and extract field `mpidr_el1` from it
    pub fn mpidr_el1_read(&mut self) -> u64 {
        Self::with_reg_val().mpidr_el1_extract()
    }

    /// inserts the given value `val` into the field `mpidr_el1`
    pub fn mpidr_el1_insert(&mut self, val: u64) -> &mut self {
        // bits 26..26
        self.val.set_bits(26..=26, val);
        self
    }

    /// reads the register, updates the `mpidr_el1` field, and writes the updated value
    pub fn mpidr_el1_write(&mut self, val: u64) {
        Self::with_reg_val().mpidr_el1_insert(val).write();
    }

    /*
     * Field: midr_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn midr_el1_extract(&self) -> u64 {
        // bits 25..25
        self.val.get_bits(25..=25)
    }

    /// reads the current register value and extract field `midr_el1` from it
    pub fn midr_el1_read(&mut self) -> u64 {
        Self::with_reg_val().midr_el1_extract()
    }

    /// inserts the given value `val` into the field `midr_el1`
    pub fn midr_el1_insert(&mut self, val: u64) -> &mut self {
        // bits 25..25
        self.val.set_bits(25..=25, val);
        self
    }

    /// reads the register, updates the `midr_el1` field, and writes the updated value
    pub fn midr_el1_write(&mut self, val: u64) {
        Self::with_reg_val().midr_el1_insert(val).write();
    }

    /*
     * Field: mair_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn mair_el1_extract(&self) -> u64 {
        // bits 24..24
        self.val.get_bits(24..=24)
    }

    /// reads the current register value and extract field `mair_el1` from it
    pub fn mair_el1_read(&mut self) -> u64 {
        Self::with_reg_val().mair_el1_extract()
    }

    /// inserts the given value `val` into the field `mair_el1`
    pub fn mair_el1_insert(&mut self, val: u64) -> &mut self {
        // bits 24..24
        self.val.set_bits(24..=24, val);
        self
    }

    /// reads the register, updates the `mair_el1` field, and writes the updated value
    pub fn mair_el1_write(&mut self, val: u64) {
        Self::with_reg_val().mair_el1_insert(val).write();
    }

    /*
     * Field: lorsa_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn lorsa_el1_1_extract(&self) -> u64 {
        // bits 23..23
        self.val.get_bits(23..=23)
    }

    /// reads the current register value and extract field `lorsa_el1_1` from it
    pub fn lorsa_el1_1_read(&mut self) -> u64 {
        Self::with_reg_val().lorsa_el1_1_extract()
    }

    /// inserts the given value `val` into the field `lorsa_el1_1`
    pub fn lorsa_el1_1_insert(&mut self, val: u64) -> &mut self {
        // bits 23..23
        self.val.set_bits(23..=23, val);
        self
    }

    /// reads the register, updates the `lorsa_el1_1` field, and writes the updated value
    pub fn lorsa_el1_1_write(&mut self, val: u64) {
        Self::with_reg_val().lorsa_el1_1_insert(val).write();
    }

    /*
     * Field: lorn_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn lorn_el1_1_extract(&self) -> u64 {
        // bits 22..22
        self.val.get_bits(22..=22)
    }

    /// reads the current register value and extract field `lorn_el1_1` from it
    pub fn lorn_el1_1_read(&mut self) -> u64 {
        Self::with_reg_val().lorn_el1_1_extract()
    }

    /// inserts the given value `val` into the field `lorn_el1_1`
    pub fn lorn_el1_1_insert(&mut self, val: u64) -> &mut self {
        // bits 22..22
        self.val.set_bits(22..=22, val);
        self
    }

    /// reads the register, updates the `lorn_el1_1` field, and writes the updated value
    pub fn lorn_el1_1_write(&mut self, val: u64) {
        Self::with_reg_val().lorn_el1_1_insert(val).write();
    }

    /*
     * Field: lorid_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn lorid_el1_1_extract(&self) -> u64 {
        // bits 21..21
        self.val.get_bits(21..=21)
    }

    /// reads the current register value and extract field `lorid_el1_1` from it
    pub fn lorid_el1_1_read(&mut self) -> u64 {
        Self::with_reg_val().lorid_el1_1_extract()
    }

    /// inserts the given value `val` into the field `lorid_el1_1`
    pub fn lorid_el1_1_insert(&mut self, val: u64) -> &mut self {
        // bits 21..21
        self.val.set_bits(21..=21, val);
        self
    }

    /// reads the register, updates the `lorid_el1_1` field, and writes the updated value
    pub fn lorid_el1_1_write(&mut self, val: u64) {
        Self::with_reg_val().lorid_el1_1_insert(val).write();
    }

    /*
     * Field: lorea_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn lorea_el1_1_extract(&self) -> u64 {
        // bits 20..20
        self.val.get_bits(20..=20)
    }

    /// reads the current register value and extract field `lorea_el1_1` from it
    pub fn lorea_el1_1_read(&mut self) -> u64 {
        Self::with_reg_val().lorea_el1_1_extract()
    }

    /// inserts the given value `val` into the field `lorea_el1_1`
    pub fn lorea_el1_1_insert(&mut self, val: u64) -> &mut self {
        // bits 20..20
        self.val.set_bits(20..=20, val);
        self
    }

    /// reads the register, updates the `lorea_el1_1` field, and writes the updated value
    pub fn lorea_el1_1_write(&mut self, val: u64) {
        Self::with_reg_val().lorea_el1_1_insert(val).write();
    }

    /*
     * Field: lorc_el1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn lorc_el1_1_extract(&self) -> u64 {
        // bits 19..19
        self.val.get_bits(19..=19)
    }

    /// reads the current register value and extract field `lorc_el1_1` from it
    pub fn lorc_el1_1_read(&mut self) -> u64 {
        Self::with_reg_val().lorc_el1_1_extract()
    }

    /// inserts the given value `val` into the field `lorc_el1_1`
    pub fn lorc_el1_1_insert(&mut self, val: u64) -> &mut self {
        // bits 19..19
        self.val.set_bits(19..=19, val);
        self
    }

    /// reads the register, updates the `lorc_el1_1` field, and writes the updated value
    pub fn lorc_el1_1_write(&mut self, val: u64) {
        Self::with_reg_val().lorc_el1_1_insert(val).write();
    }

    /*
     * Field: isr_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn isr_el1_extract(&self) -> u64 {
        // bits 18..18
        self.val.get_bits(18..=18)
    }

    /// reads the current register value and extract field `isr_el1` from it
    pub fn isr_el1_read(&mut self) -> u64 {
        Self::with_reg_val().isr_el1_extract()
    }

    /// inserts the given value `val` into the field `isr_el1`
    pub fn isr_el1_insert(&mut self, val: u64) -> &mut self {
        // bits 18..18
        self.val.set_bits(18..=18, val);
        self
    }

    /// reads the register, updates the `isr_el1` field, and writes the updated value
    pub fn isr_el1_write(&mut self, val: u64) {
        Self::with_reg_val().isr_el1_insert(val).write();
    }

    /*
     * Field: far_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn far_el1_extract(&self) -> u64 {
        // bits 17..17
        self.val.get_bits(17..=17)
    }

    /// reads the current register value and extract field `far_el1` from it
    pub fn far_el1_read(&mut self) -> u64 {
        Self::with_reg_val().far_el1_extract()
    }

    /// inserts the given value `val` into the field `far_el1`
    pub fn far_el1_insert(&mut self, val: u64) -> &mut self {
        // bits 17..17
        self.val.set_bits(17..=17, val);
        self
    }

    /// reads the register, updates the `far_el1` field, and writes the updated value
    pub fn far_el1_write(&mut self, val: u64) {
        Self::with_reg_val().far_el1_insert(val).write();
    }

    /*
     * Field: esr_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn esr_el1_extract(&self) -> u64 {
        // bits 16..16
        self.val.get_bits(16..=16)
    }

    /// reads the current register value and extract field `esr_el1` from it
    pub fn esr_el1_read(&mut self) -> u64 {
        Self::with_reg_val().esr_el1_extract()
    }

    /// inserts the given value `val` into the field `esr_el1`
    pub fn esr_el1_insert(&mut self, val: u64) -> &mut self {
        // bits 16..16
        self.val.set_bits(16..=16, val);
        self
    }

    /// reads the register, updates the `esr_el1` field, and writes the updated value
    pub fn esr_el1_write(&mut self, val: u64) {
        Self::with_reg_val().esr_el1_insert(val).write();
    }

    /*
     * Field: dczid_el0
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dczid_el0_extract(&self) -> u64 {
        // bits 15..15
        self.val.get_bits(15..=15)
    }

    /// reads the current register value and extract field `dczid_el0` from it
    pub fn dczid_el0_read(&mut self) -> u64 {
        Self::with_reg_val().dczid_el0_extract()
    }

    /// inserts the given value `val` into the field `dczid_el0`
    pub fn dczid_el0_insert(&mut self, val: u64) -> &mut self {
        // bits 15..15
        self.val.set_bits(15..=15, val);
        self
    }

    /// reads the register, updates the `dczid_el0` field, and writes the updated value
    pub fn dczid_el0_write(&mut self, val: u64) {
        Self::with_reg_val().dczid_el0_insert(val).write();
    }

    /*
     * Field: ctr_el0
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ctr_el0_extract(&self) -> u64 {
        // bits 14..14
        self.val.get_bits(14..=14)
    }

    /// reads the current register value and extract field `ctr_el0` from it
    pub fn ctr_el0_read(&mut self) -> u64 {
        Self::with_reg_val().ctr_el0_extract()
    }

    /// inserts the given value `val` into the field `ctr_el0`
    pub fn ctr_el0_insert(&mut self, val: u64) -> &mut self {
        // bits 14..14
        self.val.set_bits(14..=14, val);
        self
    }

    /// reads the register, updates the `ctr_el0` field, and writes the updated value
    pub fn ctr_el0_write(&mut self, val: u64) {
        Self::with_reg_val().ctr_el0_insert(val).write();
    }

    /*
     * Field: csselr_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn csselr_el1_extract(&self) -> u64 {
        // bits 13..13
        self.val.get_bits(13..=13)
    }

    /// reads the current register value and extract field `csselr_el1` from it
    pub fn csselr_el1_read(&mut self) -> u64 {
        Self::with_reg_val().csselr_el1_extract()
    }

    /// inserts the given value `val` into the field `csselr_el1`
    pub fn csselr_el1_insert(&mut self, val: u64) -> &mut self {
        // bits 13..13
        self.val.set_bits(13..=13, val);
        self
    }

    /// reads the register, updates the `csselr_el1` field, and writes the updated value
    pub fn csselr_el1_write(&mut self, val: u64) {
        Self::with_reg_val().csselr_el1_insert(val).write();
    }

    /*
     * Field: cpacr_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn cpacr_el1_extract(&self) -> u64 {
        // bits 12..12
        self.val.get_bits(12..=12)
    }

    /// reads the current register value and extract field `cpacr_el1` from it
    pub fn cpacr_el1_read(&mut self) -> u64 {
        Self::with_reg_val().cpacr_el1_extract()
    }

    /// inserts the given value `val` into the field `cpacr_el1`
    pub fn cpacr_el1_insert(&mut self, val: u64) -> &mut self {
        // bits 12..12
        self.val.set_bits(12..=12, val);
        self
    }

    /// reads the register, updates the `cpacr_el1` field, and writes the updated value
    pub fn cpacr_el1_write(&mut self, val: u64) {
        Self::with_reg_val().cpacr_el1_insert(val).write();
    }

    /*
     * Field: contextidr_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn contextidr_el1_extract(&self) -> u64 {
        // bits 11..11
        self.val.get_bits(11..=11)
    }

    /// reads the current register value and extract field `contextidr_el1` from it
    pub fn contextidr_el1_read(&mut self) -> u64 {
        Self::with_reg_val().contextidr_el1_extract()
    }

    /// inserts the given value `val` into the field `contextidr_el1`
    pub fn contextidr_el1_insert(&mut self, val: u64) -> &mut self {
        // bits 11..11
        self.val.set_bits(11..=11, val);
        self
    }

    /// reads the register, updates the `contextidr_el1` field, and writes the updated value
    pub fn contextidr_el1_write(&mut self, val: u64) {
        Self::with_reg_val().contextidr_el1_insert(val).write();
    }

    /*
     * Field: clidr_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn clidr_el1_extract(&self) -> u64 {
        // bits 10..10
        self.val.get_bits(10..=10)
    }

    /// reads the current register value and extract field `clidr_el1` from it
    pub fn clidr_el1_read(&mut self) -> u64 {
        Self::with_reg_val().clidr_el1_extract()
    }

    /// inserts the given value `val` into the field `clidr_el1`
    pub fn clidr_el1_insert(&mut self, val: u64) -> &mut self {
        // bits 10..10
        self.val.set_bits(10..=10, val);
        self
    }

    /// reads the register, updates the `clidr_el1` field, and writes the updated value
    pub fn clidr_el1_write(&mut self, val: u64) {
        Self::with_reg_val().clidr_el1_insert(val).write();
    }

    /*
     * Field: ccsidr_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ccsidr_el1_extract(&self) -> u64 {
        // bits 9..9
        self.val.get_bits(9..=9)
    }

    /// reads the current register value and extract field `ccsidr_el1` from it
    pub fn ccsidr_el1_read(&mut self) -> u64 {
        Self::with_reg_val().ccsidr_el1_extract()
    }

    /// inserts the given value `val` into the field `ccsidr_el1`
    pub fn ccsidr_el1_insert(&mut self, val: u64) -> &mut self {
        // bits 9..9
        self.val.set_bits(9..=9, val);
        self
    }

    /// reads the register, updates the `ccsidr_el1` field, and writes the updated value
    pub fn ccsidr_el1_write(&mut self, val: u64) {
        Self::with_reg_val().ccsidr_el1_insert(val).write();
    }

    /*
     * Field: apibkey_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn apibkey_1_extract(&self) -> u64 {
        // bits 8..8
        self.val.get_bits(8..=8)
    }

    /// reads the current register value and extract field `apibkey_1` from it
    pub fn apibkey_1_read(&mut self) -> u64 {
        Self::with_reg_val().apibkey_1_extract()
    }

    /// inserts the given value `val` into the field `apibkey_1`
    pub fn apibkey_1_insert(&mut self, val: u64) -> &mut self {
        // bits 8..8
        self.val.set_bits(8..=8, val);
        self
    }

    /// reads the register, updates the `apibkey_1` field, and writes the updated value
    pub fn apibkey_1_write(&mut self, val: u64) {
        Self::with_reg_val().apibkey_1_insert(val).write();
    }

    /*
     * Field: apiakey_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn apiakey_1_extract(&self) -> u64 {
        // bits 7..7
        self.val.get_bits(7..=7)
    }

    /// reads the current register value and extract field `apiakey_1` from it
    pub fn apiakey_1_read(&mut self) -> u64 {
        Self::with_reg_val().apiakey_1_extract()
    }

    /// inserts the given value `val` into the field `apiakey_1`
    pub fn apiakey_1_insert(&mut self, val: u64) -> &mut self {
        // bits 7..7
        self.val.set_bits(7..=7, val);
        self
    }

    /// reads the register, updates the `apiakey_1` field, and writes the updated value
    pub fn apiakey_1_write(&mut self, val: u64) {
        Self::with_reg_val().apiakey_1_insert(val).write();
    }

    /*
     * Field: apgakey_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn apgakey_1_extract(&self) -> u64 {
        // bits 6..6
        self.val.get_bits(6..=6)
    }

    /// reads the current register value and extract field `apgakey_1` from it
    pub fn apgakey_1_read(&mut self) -> u64 {
        Self::with_reg_val().apgakey_1_extract()
    }

    /// inserts the given value `val` into the field `apgakey_1`
    pub fn apgakey_1_insert(&mut self, val: u64) -> &mut self {
        // bits 6..6
        self.val.set_bits(6..=6, val);
        self
    }

    /// reads the register, updates the `apgakey_1` field, and writes the updated value
    pub fn apgakey_1_write(&mut self, val: u64) {
        Self::with_reg_val().apgakey_1_insert(val).write();
    }

    /*
     * Field: apdbkey_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn apdbkey_1_extract(&self) -> u64 {
        // bits 5..5
        self.val.get_bits(5..=5)
    }

    /// reads the current register value and extract field `apdbkey_1` from it
    pub fn apdbkey_1_read(&mut self) -> u64 {
        Self::with_reg_val().apdbkey_1_extract()
    }

    /// inserts the given value `val` into the field `apdbkey_1`
    pub fn apdbkey_1_insert(&mut self, val: u64) -> &mut self {
        // bits 5..5
        self.val.set_bits(5..=5, val);
        self
    }

    /// reads the register, updates the `apdbkey_1` field, and writes the updated value
    pub fn apdbkey_1_write(&mut self, val: u64) {
        Self::with_reg_val().apdbkey_1_insert(val).write();
    }

    /*
     * Field: apdakey_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn apdakey_1_extract(&self) -> u64 {
        // bits 4..4
        self.val.get_bits(4..=4)
    }

    /// reads the current register value and extract field `apdakey_1` from it
    pub fn apdakey_1_read(&mut self) -> u64 {
        Self::with_reg_val().apdakey_1_extract()
    }

    /// inserts the given value `val` into the field `apdakey_1`
    pub fn apdakey_1_insert(&mut self, val: u64) -> &mut self {
        // bits 4..4
        self.val.set_bits(4..=4, val);
        self
    }

    /// reads the register, updates the `apdakey_1` field, and writes the updated value
    pub fn apdakey_1_write(&mut self, val: u64) {
        Self::with_reg_val().apdakey_1_insert(val).write();
    }

    /*
     * Field: amair_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amair_el1_extract(&self) -> u64 {
        // bits 3..3
        self.val.get_bits(3..=3)
    }

    /// reads the current register value and extract field `amair_el1` from it
    pub fn amair_el1_read(&mut self) -> u64 {
        Self::with_reg_val().amair_el1_extract()
    }

    /// inserts the given value `val` into the field `amair_el1`
    pub fn amair_el1_insert(&mut self, val: u64) -> &mut self {
        // bits 3..3
        self.val.set_bits(3..=3, val);
        self
    }

    /// reads the register, updates the `amair_el1` field, and writes the updated value
    pub fn amair_el1_write(&mut self, val: u64) {
        Self::with_reg_val().amair_el1_insert(val).write();
    }

    /*
     * Field: aidr_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn aidr_el1_extract(&self) -> u64 {
        // bits 2..2
        self.val.get_bits(2..=2)
    }

    /// reads the current register value and extract field `aidr_el1` from it
    pub fn aidr_el1_read(&mut self) -> u64 {
        Self::with_reg_val().aidr_el1_extract()
    }

    /// inserts the given value `val` into the field `aidr_el1`
    pub fn aidr_el1_insert(&mut self, val: u64) -> &mut self {
        // bits 2..2
        self.val.set_bits(2..=2, val);
        self
    }

    /// reads the register, updates the `aidr_el1` field, and writes the updated value
    pub fn aidr_el1_write(&mut self, val: u64) {
        Self::with_reg_val().aidr_el1_insert(val).write();
    }

    /*
     * Field: afsr1_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn afsr1_el1_extract(&self) -> u64 {
        // bits 1..1
        self.val.get_bits(1..=1)
    }

    /// reads the current register value and extract field `afsr1_el1` from it
    pub fn afsr1_el1_read(&mut self) -> u64 {
        Self::with_reg_val().afsr1_el1_extract()
    }

    /// inserts the given value `val` into the field `afsr1_el1`
    pub fn afsr1_el1_insert(&mut self, val: u64) -> &mut self {
        // bits 1..1
        self.val.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `afsr1_el1` field, and writes the updated value
    pub fn afsr1_el1_write(&mut self, val: u64) {
        Self::with_reg_val().afsr1_el1_insert(val).write();
    }

    /*
     * Field: afsr0_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn afsr0_el1_extract(&self) -> u64 {
        // bits 0..0
        self.val.get_bits(0..=0)
    }

    /// reads the current register value and extract field `afsr0_el1` from it
    pub fn afsr0_el1_read(&mut self) -> u64 {
        Self::with_reg_val().afsr0_el1_extract()
    }

    /// inserts the given value `val` into the field `afsr0_el1`
    pub fn afsr0_el1_insert(&mut self, val: u64) -> &mut self {
        // bits 0..0
        self.val.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `afsr0_el1` field, and writes the updated value
    pub fn afsr0_el1_write(&mut self, val: u64) {
        Self::with_reg_val().afsr0_el1_insert(val).write();
    }
}

impl Default for HfgrtrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> HfgrtrEl2 {
        HfgrtrEl2(0)
    }
}
