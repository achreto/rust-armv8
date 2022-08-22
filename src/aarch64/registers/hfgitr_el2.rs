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
 * Generated on: 2022-08-22T15:51:28.516496
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
 * Register:    Hypervisor Fine-Grained Instruction Trap Register (hfgitr_el2)
 * Group:       A group mapping that does not have a known primary
 * Type:        64-bit Register
 * Description: Provides instruction trap controls.
 * File:        AArch64-hfgitr_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Hypervisor Fine-Grained Instruction Trap Register value in memory
pub struct HfgitrEl2(u64);

/// struct implementation for accessing the fields of register hfgitr_el2
impl HfgitrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> HfgitrEl2 {
        Self::default()
    }

    /// collects the modifications of HfgitrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> HfgitrEl2 {
        HfgitrEl2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> HfgitrEl2 {
        let curval = Self::reg_rawrd() & 0x7fffffffffffff;
        HfgitrEl2(curval)
    }

    /// reading the Hypervisor Fine-Grained Instruction Trap Register (hfgitr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, HFGITR_EL2
            llvm_asm!("mrs $0, S3_4_C1_C1_6" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Hypervisor Fine-Grained Instruction Trap Register (hfgitr_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR HFGITR_EL2, <Xt>
            llvm_asm!("msr S3_4_C1_C1_6, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0x7fffffffffffff;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 36028797018963967;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: dccvac
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dccvac_extract(&self) -> u64 {
        // bits 54..54
        self.val.get_bits(54..=54)
    }

    /// reads the current register value and extract field `dccvac` from it
    pub fn dccvac_read(&mut self) -> u64 {
        Self::with_reg_val().dccvac_extract()
    }

    /// inserts the given value `val` into the field `dccvac`
    pub fn dccvac_insert(&mut self, val: u64) -> &mut self {
        // bits 54..54
        self.val.set_bits(54..=54, val);
        self
    }

    /// reads the register, updates the `dccvac` field, and writes the updated value
    pub fn dccvac_write(&mut self, val: u64) {
        Self::with_reg_val().dccvac_insert(val).write();
    }

    /*
     * Field: svc_el1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn svc_el1_extract(&self) -> u64 {
        // bits 53..53
        self.val.get_bits(53..=53)
    }

    /// reads the current register value and extract field `svc_el1` from it
    pub fn svc_el1_read(&mut self) -> u64 {
        Self::with_reg_val().svc_el1_extract()
    }

    /// inserts the given value `val` into the field `svc_el1`
    pub fn svc_el1_insert(&mut self, val: u64) -> &mut self {
        // bits 53..53
        self.val.set_bits(53..=53, val);
        self
    }

    /// reads the register, updates the `svc_el1` field, and writes the updated value
    pub fn svc_el1_write(&mut self, val: u64) {
        Self::with_reg_val().svc_el1_insert(val).write();
    }

    /*
     * Field: svc_el0
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn svc_el0_extract(&self) -> u64 {
        // bits 52..52
        self.val.get_bits(52..=52)
    }

    /// reads the current register value and extract field `svc_el0` from it
    pub fn svc_el0_read(&mut self) -> u64 {
        Self::with_reg_val().svc_el0_extract()
    }

    /// inserts the given value `val` into the field `svc_el0`
    pub fn svc_el0_insert(&mut self, val: u64) -> &mut self {
        // bits 52..52
        self.val.set_bits(52..=52, val);
        self
    }

    /// reads the register, updates the `svc_el0` field, and writes the updated value
    pub fn svc_el0_write(&mut self, val: u64) {
        Self::with_reg_val().svc_el0_insert(val).write();
    }

    /*
     * Field: eret
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn eret_extract(&self) -> u64 {
        // bits 51..51
        self.val.get_bits(51..=51)
    }

    /// reads the current register value and extract field `eret` from it
    pub fn eret_read(&mut self) -> u64 {
        Self::with_reg_val().eret_extract()
    }

    /// inserts the given value `val` into the field `eret`
    pub fn eret_insert(&mut self, val: u64) -> &mut self {
        // bits 51..51
        self.val.set_bits(51..=51, val);
        self
    }

    /// reads the register, updates the `eret` field, and writes the updated value
    pub fn eret_write(&mut self, val: u64) {
        Self::with_reg_val().eret_insert(val).write();
    }

    /*
     * Field: cpprctx_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn cpprctx_1_extract(&self) -> u64 {
        // bits 50..50
        self.val.get_bits(50..=50)
    }

    /// reads the current register value and extract field `cpprctx_1` from it
    pub fn cpprctx_1_read(&mut self) -> u64 {
        Self::with_reg_val().cpprctx_1_extract()
    }

    /// inserts the given value `val` into the field `cpprctx_1`
    pub fn cpprctx_1_insert(&mut self, val: u64) -> &mut self {
        // bits 50..50
        self.val.set_bits(50..=50, val);
        self
    }

    /// reads the register, updates the `cpprctx_1` field, and writes the updated value
    pub fn cpprctx_1_write(&mut self, val: u64) {
        Self::with_reg_val().cpprctx_1_insert(val).write();
    }

    /*
     * Field: dvprctx_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dvprctx_1_extract(&self) -> u64 {
        // bits 49..49
        self.val.get_bits(49..=49)
    }

    /// reads the current register value and extract field `dvprctx_1` from it
    pub fn dvprctx_1_read(&mut self) -> u64 {
        Self::with_reg_val().dvprctx_1_extract()
    }

    /// inserts the given value `val` into the field `dvprctx_1`
    pub fn dvprctx_1_insert(&mut self, val: u64) -> &mut self {
        // bits 49..49
        self.val.set_bits(49..=49, val);
        self
    }

    /// reads the register, updates the `dvprctx_1` field, and writes the updated value
    pub fn dvprctx_1_write(&mut self, val: u64) {
        Self::with_reg_val().dvprctx_1_insert(val).write();
    }

    /*
     * Field: cfprctx_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn cfprctx_1_extract(&self) -> u64 {
        // bits 48..48
        self.val.get_bits(48..=48)
    }

    /// reads the current register value and extract field `cfprctx_1` from it
    pub fn cfprctx_1_read(&mut self) -> u64 {
        Self::with_reg_val().cfprctx_1_extract()
    }

    /// inserts the given value `val` into the field `cfprctx_1`
    pub fn cfprctx_1_insert(&mut self, val: u64) -> &mut self {
        // bits 48..48
        self.val.set_bits(48..=48, val);
        self
    }

    /// reads the register, updates the `cfprctx_1` field, and writes the updated value
    pub fn cfprctx_1_write(&mut self, val: u64) {
        Self::with_reg_val().cfprctx_1_insert(val).write();
    }

    /*
     * Field: tlbivaale1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbivaale1_extract(&self) -> u64 {
        // bits 47..47
        self.val.get_bits(47..=47)
    }

    /// reads the current register value and extract field `tlbivaale1` from it
    pub fn tlbivaale1_read(&mut self) -> u64 {
        Self::with_reg_val().tlbivaale1_extract()
    }

    /// inserts the given value `val` into the field `tlbivaale1`
    pub fn tlbivaale1_insert(&mut self, val: u64) -> &mut self {
        // bits 47..47
        self.val.set_bits(47..=47, val);
        self
    }

    /// reads the register, updates the `tlbivaale1` field, and writes the updated value
    pub fn tlbivaale1_write(&mut self, val: u64) {
        Self::with_reg_val().tlbivaale1_insert(val).write();
    }

    /*
     * Field: tlbivale1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbivale1_extract(&self) -> u64 {
        // bits 46..46
        self.val.get_bits(46..=46)
    }

    /// reads the current register value and extract field `tlbivale1` from it
    pub fn tlbivale1_read(&mut self) -> u64 {
        Self::with_reg_val().tlbivale1_extract()
    }

    /// inserts the given value `val` into the field `tlbivale1`
    pub fn tlbivale1_insert(&mut self, val: u64) -> &mut self {
        // bits 46..46
        self.val.set_bits(46..=46, val);
        self
    }

    /// reads the register, updates the `tlbivale1` field, and writes the updated value
    pub fn tlbivale1_write(&mut self, val: u64) {
        Self::with_reg_val().tlbivale1_insert(val).write();
    }

    /*
     * Field: tlbivaae1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbivaae1_extract(&self) -> u64 {
        // bits 45..45
        self.val.get_bits(45..=45)
    }

    /// reads the current register value and extract field `tlbivaae1` from it
    pub fn tlbivaae1_read(&mut self) -> u64 {
        Self::with_reg_val().tlbivaae1_extract()
    }

    /// inserts the given value `val` into the field `tlbivaae1`
    pub fn tlbivaae1_insert(&mut self, val: u64) -> &mut self {
        // bits 45..45
        self.val.set_bits(45..=45, val);
        self
    }

    /// reads the register, updates the `tlbivaae1` field, and writes the updated value
    pub fn tlbivaae1_write(&mut self, val: u64) {
        Self::with_reg_val().tlbivaae1_insert(val).write();
    }

    /*
     * Field: tlbiaside1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbiaside1_extract(&self) -> u64 {
        // bits 44..44
        self.val.get_bits(44..=44)
    }

    /// reads the current register value and extract field `tlbiaside1` from it
    pub fn tlbiaside1_read(&mut self) -> u64 {
        Self::with_reg_val().tlbiaside1_extract()
    }

    /// inserts the given value `val` into the field `tlbiaside1`
    pub fn tlbiaside1_insert(&mut self, val: u64) -> &mut self {
        // bits 44..44
        self.val.set_bits(44..=44, val);
        self
    }

    /// reads the register, updates the `tlbiaside1` field, and writes the updated value
    pub fn tlbiaside1_write(&mut self, val: u64) {
        Self::with_reg_val().tlbiaside1_insert(val).write();
    }

    /*
     * Field: tlbivae1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbivae1_extract(&self) -> u64 {
        // bits 43..43
        self.val.get_bits(43..=43)
    }

    /// reads the current register value and extract field `tlbivae1` from it
    pub fn tlbivae1_read(&mut self) -> u64 {
        Self::with_reg_val().tlbivae1_extract()
    }

    /// inserts the given value `val` into the field `tlbivae1`
    pub fn tlbivae1_insert(&mut self, val: u64) -> &mut self {
        // bits 43..43
        self.val.set_bits(43..=43, val);
        self
    }

    /// reads the register, updates the `tlbivae1` field, and writes the updated value
    pub fn tlbivae1_write(&mut self, val: u64) {
        Self::with_reg_val().tlbivae1_insert(val).write();
    }

    /*
     * Field: tlbivmalle1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbivmalle1_extract(&self) -> u64 {
        // bits 42..42
        self.val.get_bits(42..=42)
    }

    /// reads the current register value and extract field `tlbivmalle1` from it
    pub fn tlbivmalle1_read(&mut self) -> u64 {
        Self::with_reg_val().tlbivmalle1_extract()
    }

    /// inserts the given value `val` into the field `tlbivmalle1`
    pub fn tlbivmalle1_insert(&mut self, val: u64) -> &mut self {
        // bits 42..42
        self.val.set_bits(42..=42, val);
        self
    }

    /// reads the register, updates the `tlbivmalle1` field, and writes the updated value
    pub fn tlbivmalle1_write(&mut self, val: u64) {
        Self::with_reg_val().tlbivmalle1_insert(val).write();
    }

    /*
     * Field: tlbirvaale1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbirvaale1_1_extract(&self) -> u64 {
        // bits 41..41
        self.val.get_bits(41..=41)
    }

    /// reads the current register value and extract field `tlbirvaale1_1` from it
    pub fn tlbirvaale1_1_read(&mut self) -> u64 {
        Self::with_reg_val().tlbirvaale1_1_extract()
    }

    /// inserts the given value `val` into the field `tlbirvaale1_1`
    pub fn tlbirvaale1_1_insert(&mut self, val: u64) -> &mut self {
        // bits 41..41
        self.val.set_bits(41..=41, val);
        self
    }

    /// reads the register, updates the `tlbirvaale1_1` field, and writes the updated value
    pub fn tlbirvaale1_1_write(&mut self, val: u64) {
        Self::with_reg_val().tlbirvaale1_1_insert(val).write();
    }

    /*
     * Field: tlbirvale1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbirvale1_1_extract(&self) -> u64 {
        // bits 40..40
        self.val.get_bits(40..=40)
    }

    /// reads the current register value and extract field `tlbirvale1_1` from it
    pub fn tlbirvale1_1_read(&mut self) -> u64 {
        Self::with_reg_val().tlbirvale1_1_extract()
    }

    /// inserts the given value `val` into the field `tlbirvale1_1`
    pub fn tlbirvale1_1_insert(&mut self, val: u64) -> &mut self {
        // bits 40..40
        self.val.set_bits(40..=40, val);
        self
    }

    /// reads the register, updates the `tlbirvale1_1` field, and writes the updated value
    pub fn tlbirvale1_1_write(&mut self, val: u64) {
        Self::with_reg_val().tlbirvale1_1_insert(val).write();
    }

    /*
     * Field: tlbirvaae1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbirvaae1_1_extract(&self) -> u64 {
        // bits 39..39
        self.val.get_bits(39..=39)
    }

    /// reads the current register value and extract field `tlbirvaae1_1` from it
    pub fn tlbirvaae1_1_read(&mut self) -> u64 {
        Self::with_reg_val().tlbirvaae1_1_extract()
    }

    /// inserts the given value `val` into the field `tlbirvaae1_1`
    pub fn tlbirvaae1_1_insert(&mut self, val: u64) -> &mut self {
        // bits 39..39
        self.val.set_bits(39..=39, val);
        self
    }

    /// reads the register, updates the `tlbirvaae1_1` field, and writes the updated value
    pub fn tlbirvaae1_1_write(&mut self, val: u64) {
        Self::with_reg_val().tlbirvaae1_1_insert(val).write();
    }

    /*
     * Field: tlbirvae1_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbirvae1_1_extract(&self) -> u64 {
        // bits 38..38
        self.val.get_bits(38..=38)
    }

    /// reads the current register value and extract field `tlbirvae1_1` from it
    pub fn tlbirvae1_1_read(&mut self) -> u64 {
        Self::with_reg_val().tlbirvae1_1_extract()
    }

    /// inserts the given value `val` into the field `tlbirvae1_1`
    pub fn tlbirvae1_1_insert(&mut self, val: u64) -> &mut self {
        // bits 38..38
        self.val.set_bits(38..=38, val);
        self
    }

    /// reads the register, updates the `tlbirvae1_1` field, and writes the updated value
    pub fn tlbirvae1_1_write(&mut self, val: u64) {
        Self::with_reg_val().tlbirvae1_1_insert(val).write();
    }

    /*
     * Field: tlbirvaale1is_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbirvaale1is_1_extract(&self) -> u64 {
        // bits 37..37
        self.val.get_bits(37..=37)
    }

    /// reads the current register value and extract field `tlbirvaale1is_1` from it
    pub fn tlbirvaale1is_1_read(&mut self) -> u64 {
        Self::with_reg_val().tlbirvaale1is_1_extract()
    }

    /// inserts the given value `val` into the field `tlbirvaale1is_1`
    pub fn tlbirvaale1is_1_insert(&mut self, val: u64) -> &mut self {
        // bits 37..37
        self.val.set_bits(37..=37, val);
        self
    }

    /// reads the register, updates the `tlbirvaale1is_1` field, and writes the updated value
    pub fn tlbirvaale1is_1_write(&mut self, val: u64) {
        Self::with_reg_val().tlbirvaale1is_1_insert(val).write();
    }

    /*
     * Field: tlbirvale1is_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbirvale1is_1_extract(&self) -> u64 {
        // bits 36..36
        self.val.get_bits(36..=36)
    }

    /// reads the current register value and extract field `tlbirvale1is_1` from it
    pub fn tlbirvale1is_1_read(&mut self) -> u64 {
        Self::with_reg_val().tlbirvale1is_1_extract()
    }

    /// inserts the given value `val` into the field `tlbirvale1is_1`
    pub fn tlbirvale1is_1_insert(&mut self, val: u64) -> &mut self {
        // bits 36..36
        self.val.set_bits(36..=36, val);
        self
    }

    /// reads the register, updates the `tlbirvale1is_1` field, and writes the updated value
    pub fn tlbirvale1is_1_write(&mut self, val: u64) {
        Self::with_reg_val().tlbirvale1is_1_insert(val).write();
    }

    /*
     * Field: tlbirvaae1is_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbirvaae1is_1_extract(&self) -> u64 {
        // bits 35..35
        self.val.get_bits(35..=35)
    }

    /// reads the current register value and extract field `tlbirvaae1is_1` from it
    pub fn tlbirvaae1is_1_read(&mut self) -> u64 {
        Self::with_reg_val().tlbirvaae1is_1_extract()
    }

    /// inserts the given value `val` into the field `tlbirvaae1is_1`
    pub fn tlbirvaae1is_1_insert(&mut self, val: u64) -> &mut self {
        // bits 35..35
        self.val.set_bits(35..=35, val);
        self
    }

    /// reads the register, updates the `tlbirvaae1is_1` field, and writes the updated value
    pub fn tlbirvaae1is_1_write(&mut self, val: u64) {
        Self::with_reg_val().tlbirvaae1is_1_insert(val).write();
    }

    /*
     * Field: tlbirvae1is_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbirvae1is_1_extract(&self) -> u64 {
        // bits 34..34
        self.val.get_bits(34..=34)
    }

    /// reads the current register value and extract field `tlbirvae1is_1` from it
    pub fn tlbirvae1is_1_read(&mut self) -> u64 {
        Self::with_reg_val().tlbirvae1is_1_extract()
    }

    /// inserts the given value `val` into the field `tlbirvae1is_1`
    pub fn tlbirvae1is_1_insert(&mut self, val: u64) -> &mut self {
        // bits 34..34
        self.val.set_bits(34..=34, val);
        self
    }

    /// reads the register, updates the `tlbirvae1is_1` field, and writes the updated value
    pub fn tlbirvae1is_1_write(&mut self, val: u64) {
        Self::with_reg_val().tlbirvae1is_1_insert(val).write();
    }

    /*
     * Field: tlbivaale1is
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbivaale1is_extract(&self) -> u64 {
        // bits 33..33
        self.val.get_bits(33..=33)
    }

    /// reads the current register value and extract field `tlbivaale1is` from it
    pub fn tlbivaale1is_read(&mut self) -> u64 {
        Self::with_reg_val().tlbivaale1is_extract()
    }

    /// inserts the given value `val` into the field `tlbivaale1is`
    pub fn tlbivaale1is_insert(&mut self, val: u64) -> &mut self {
        // bits 33..33
        self.val.set_bits(33..=33, val);
        self
    }

    /// reads the register, updates the `tlbivaale1is` field, and writes the updated value
    pub fn tlbivaale1is_write(&mut self, val: u64) {
        Self::with_reg_val().tlbivaale1is_insert(val).write();
    }

    /*
     * Field: tlbivale1is
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbivale1is_extract(&self) -> u64 {
        // bits 32..32
        self.val.get_bits(32..=32)
    }

    /// reads the current register value and extract field `tlbivale1is` from it
    pub fn tlbivale1is_read(&mut self) -> u64 {
        Self::with_reg_val().tlbivale1is_extract()
    }

    /// inserts the given value `val` into the field `tlbivale1is`
    pub fn tlbivale1is_insert(&mut self, val: u64) -> &mut self {
        // bits 32..32
        self.val.set_bits(32..=32, val);
        self
    }

    /// reads the register, updates the `tlbivale1is` field, and writes the updated value
    pub fn tlbivale1is_write(&mut self, val: u64) {
        Self::with_reg_val().tlbivale1is_insert(val).write();
    }

    /*
     * Field: tlbivaae1is
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbivaae1is_extract(&self) -> u64 {
        // bits 31..31
        self.val.get_bits(31..=31)
    }

    /// reads the current register value and extract field `tlbivaae1is` from it
    pub fn tlbivaae1is_read(&mut self) -> u64 {
        Self::with_reg_val().tlbivaae1is_extract()
    }

    /// inserts the given value `val` into the field `tlbivaae1is`
    pub fn tlbivaae1is_insert(&mut self, val: u64) -> &mut self {
        // bits 31..31
        self.val.set_bits(31..=31, val);
        self
    }

    /// reads the register, updates the `tlbivaae1is` field, and writes the updated value
    pub fn tlbivaae1is_write(&mut self, val: u64) {
        Self::with_reg_val().tlbivaae1is_insert(val).write();
    }

    /*
     * Field: tlbiaside1is
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbiaside1is_extract(&self) -> u64 {
        // bits 30..30
        self.val.get_bits(30..=30)
    }

    /// reads the current register value and extract field `tlbiaside1is` from it
    pub fn tlbiaside1is_read(&mut self) -> u64 {
        Self::with_reg_val().tlbiaside1is_extract()
    }

    /// inserts the given value `val` into the field `tlbiaside1is`
    pub fn tlbiaside1is_insert(&mut self, val: u64) -> &mut self {
        // bits 30..30
        self.val.set_bits(30..=30, val);
        self
    }

    /// reads the register, updates the `tlbiaside1is` field, and writes the updated value
    pub fn tlbiaside1is_write(&mut self, val: u64) {
        Self::with_reg_val().tlbiaside1is_insert(val).write();
    }

    /*
     * Field: tlbivae1is
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbivae1is_extract(&self) -> u64 {
        // bits 29..29
        self.val.get_bits(29..=29)
    }

    /// reads the current register value and extract field `tlbivae1is` from it
    pub fn tlbivae1is_read(&mut self) -> u64 {
        Self::with_reg_val().tlbivae1is_extract()
    }

    /// inserts the given value `val` into the field `tlbivae1is`
    pub fn tlbivae1is_insert(&mut self, val: u64) -> &mut self {
        // bits 29..29
        self.val.set_bits(29..=29, val);
        self
    }

    /// reads the register, updates the `tlbivae1is` field, and writes the updated value
    pub fn tlbivae1is_write(&mut self, val: u64) {
        Self::with_reg_val().tlbivae1is_insert(val).write();
    }

    /*
     * Field: tlbivmalle1is
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbivmalle1is_extract(&self) -> u64 {
        // bits 28..28
        self.val.get_bits(28..=28)
    }

    /// reads the current register value and extract field `tlbivmalle1is` from it
    pub fn tlbivmalle1is_read(&mut self) -> u64 {
        Self::with_reg_val().tlbivmalle1is_extract()
    }

    /// inserts the given value `val` into the field `tlbivmalle1is`
    pub fn tlbivmalle1is_insert(&mut self, val: u64) -> &mut self {
        // bits 28..28
        self.val.set_bits(28..=28, val);
        self
    }

    /// reads the register, updates the `tlbivmalle1is` field, and writes the updated value
    pub fn tlbivmalle1is_write(&mut self, val: u64) {
        Self::with_reg_val().tlbivmalle1is_insert(val).write();
    }

    /*
     * Field: tlbirvaale1os_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbirvaale1os_1_extract(&self) -> u64 {
        // bits 27..27
        self.val.get_bits(27..=27)
    }

    /// reads the current register value and extract field `tlbirvaale1os_1` from it
    pub fn tlbirvaale1os_1_read(&mut self) -> u64 {
        Self::with_reg_val().tlbirvaale1os_1_extract()
    }

    /// inserts the given value `val` into the field `tlbirvaale1os_1`
    pub fn tlbirvaale1os_1_insert(&mut self, val: u64) -> &mut self {
        // bits 27..27
        self.val.set_bits(27..=27, val);
        self
    }

    /// reads the register, updates the `tlbirvaale1os_1` field, and writes the updated value
    pub fn tlbirvaale1os_1_write(&mut self, val: u64) {
        Self::with_reg_val().tlbirvaale1os_1_insert(val).write();
    }

    /*
     * Field: tlbirvale1os_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbirvale1os_1_extract(&self) -> u64 {
        // bits 26..26
        self.val.get_bits(26..=26)
    }

    /// reads the current register value and extract field `tlbirvale1os_1` from it
    pub fn tlbirvale1os_1_read(&mut self) -> u64 {
        Self::with_reg_val().tlbirvale1os_1_extract()
    }

    /// inserts the given value `val` into the field `tlbirvale1os_1`
    pub fn tlbirvale1os_1_insert(&mut self, val: u64) -> &mut self {
        // bits 26..26
        self.val.set_bits(26..=26, val);
        self
    }

    /// reads the register, updates the `tlbirvale1os_1` field, and writes the updated value
    pub fn tlbirvale1os_1_write(&mut self, val: u64) {
        Self::with_reg_val().tlbirvale1os_1_insert(val).write();
    }

    /*
     * Field: tlbirvaae1os_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbirvaae1os_1_extract(&self) -> u64 {
        // bits 25..25
        self.val.get_bits(25..=25)
    }

    /// reads the current register value and extract field `tlbirvaae1os_1` from it
    pub fn tlbirvaae1os_1_read(&mut self) -> u64 {
        Self::with_reg_val().tlbirvaae1os_1_extract()
    }

    /// inserts the given value `val` into the field `tlbirvaae1os_1`
    pub fn tlbirvaae1os_1_insert(&mut self, val: u64) -> &mut self {
        // bits 25..25
        self.val.set_bits(25..=25, val);
        self
    }

    /// reads the register, updates the `tlbirvaae1os_1` field, and writes the updated value
    pub fn tlbirvaae1os_1_write(&mut self, val: u64) {
        Self::with_reg_val().tlbirvaae1os_1_insert(val).write();
    }

    /*
     * Field: tlbirvae1os_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbirvae1os_1_extract(&self) -> u64 {
        // bits 24..24
        self.val.get_bits(24..=24)
    }

    /// reads the current register value and extract field `tlbirvae1os_1` from it
    pub fn tlbirvae1os_1_read(&mut self) -> u64 {
        Self::with_reg_val().tlbirvae1os_1_extract()
    }

    /// inserts the given value `val` into the field `tlbirvae1os_1`
    pub fn tlbirvae1os_1_insert(&mut self, val: u64) -> &mut self {
        // bits 24..24
        self.val.set_bits(24..=24, val);
        self
    }

    /// reads the register, updates the `tlbirvae1os_1` field, and writes the updated value
    pub fn tlbirvae1os_1_write(&mut self, val: u64) {
        Self::with_reg_val().tlbirvae1os_1_insert(val).write();
    }

    /*
     * Field: tlbivaale1os_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbivaale1os_1_extract(&self) -> u64 {
        // bits 23..23
        self.val.get_bits(23..=23)
    }

    /// reads the current register value and extract field `tlbivaale1os_1` from it
    pub fn tlbivaale1os_1_read(&mut self) -> u64 {
        Self::with_reg_val().tlbivaale1os_1_extract()
    }

    /// inserts the given value `val` into the field `tlbivaale1os_1`
    pub fn tlbivaale1os_1_insert(&mut self, val: u64) -> &mut self {
        // bits 23..23
        self.val.set_bits(23..=23, val);
        self
    }

    /// reads the register, updates the `tlbivaale1os_1` field, and writes the updated value
    pub fn tlbivaale1os_1_write(&mut self, val: u64) {
        Self::with_reg_val().tlbivaale1os_1_insert(val).write();
    }

    /*
     * Field: tlbivale1os_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbivale1os_1_extract(&self) -> u64 {
        // bits 22..22
        self.val.get_bits(22..=22)
    }

    /// reads the current register value and extract field `tlbivale1os_1` from it
    pub fn tlbivale1os_1_read(&mut self) -> u64 {
        Self::with_reg_val().tlbivale1os_1_extract()
    }

    /// inserts the given value `val` into the field `tlbivale1os_1`
    pub fn tlbivale1os_1_insert(&mut self, val: u64) -> &mut self {
        // bits 22..22
        self.val.set_bits(22..=22, val);
        self
    }

    /// reads the register, updates the `tlbivale1os_1` field, and writes the updated value
    pub fn tlbivale1os_1_write(&mut self, val: u64) {
        Self::with_reg_val().tlbivale1os_1_insert(val).write();
    }

    /*
     * Field: tlbivaae1os_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbivaae1os_1_extract(&self) -> u64 {
        // bits 21..21
        self.val.get_bits(21..=21)
    }

    /// reads the current register value and extract field `tlbivaae1os_1` from it
    pub fn tlbivaae1os_1_read(&mut self) -> u64 {
        Self::with_reg_val().tlbivaae1os_1_extract()
    }

    /// inserts the given value `val` into the field `tlbivaae1os_1`
    pub fn tlbivaae1os_1_insert(&mut self, val: u64) -> &mut self {
        // bits 21..21
        self.val.set_bits(21..=21, val);
        self
    }

    /// reads the register, updates the `tlbivaae1os_1` field, and writes the updated value
    pub fn tlbivaae1os_1_write(&mut self, val: u64) {
        Self::with_reg_val().tlbivaae1os_1_insert(val).write();
    }

    /*
     * Field: tlbiaside1os_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbiaside1os_1_extract(&self) -> u64 {
        // bits 20..20
        self.val.get_bits(20..=20)
    }

    /// reads the current register value and extract field `tlbiaside1os_1` from it
    pub fn tlbiaside1os_1_read(&mut self) -> u64 {
        Self::with_reg_val().tlbiaside1os_1_extract()
    }

    /// inserts the given value `val` into the field `tlbiaside1os_1`
    pub fn tlbiaside1os_1_insert(&mut self, val: u64) -> &mut self {
        // bits 20..20
        self.val.set_bits(20..=20, val);
        self
    }

    /// reads the register, updates the `tlbiaside1os_1` field, and writes the updated value
    pub fn tlbiaside1os_1_write(&mut self, val: u64) {
        Self::with_reg_val().tlbiaside1os_1_insert(val).write();
    }

    /*
     * Field: tlbivae1os_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbivae1os_1_extract(&self) -> u64 {
        // bits 19..19
        self.val.get_bits(19..=19)
    }

    /// reads the current register value and extract field `tlbivae1os_1` from it
    pub fn tlbivae1os_1_read(&mut self) -> u64 {
        Self::with_reg_val().tlbivae1os_1_extract()
    }

    /// inserts the given value `val` into the field `tlbivae1os_1`
    pub fn tlbivae1os_1_insert(&mut self, val: u64) -> &mut self {
        // bits 19..19
        self.val.set_bits(19..=19, val);
        self
    }

    /// reads the register, updates the `tlbivae1os_1` field, and writes the updated value
    pub fn tlbivae1os_1_write(&mut self, val: u64) {
        Self::with_reg_val().tlbivae1os_1_insert(val).write();
    }

    /*
     * Field: tlbivmalle1os_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlbivmalle1os_1_extract(&self) -> u64 {
        // bits 18..18
        self.val.get_bits(18..=18)
    }

    /// reads the current register value and extract field `tlbivmalle1os_1` from it
    pub fn tlbivmalle1os_1_read(&mut self) -> u64 {
        Self::with_reg_val().tlbivmalle1os_1_extract()
    }

    /// inserts the given value `val` into the field `tlbivmalle1os_1`
    pub fn tlbivmalle1os_1_insert(&mut self, val: u64) -> &mut self {
        // bits 18..18
        self.val.set_bits(18..=18, val);
        self
    }

    /// reads the register, updates the `tlbivmalle1os_1` field, and writes the updated value
    pub fn tlbivmalle1os_1_write(&mut self, val: u64) {
        Self::with_reg_val().tlbivmalle1os_1_insert(val).write();
    }

    /*
     * Field: ats1e1wp_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ats1e1wp_1_extract(&self) -> u64 {
        // bits 17..17
        self.val.get_bits(17..=17)
    }

    /// reads the current register value and extract field `ats1e1wp_1` from it
    pub fn ats1e1wp_1_read(&mut self) -> u64 {
        Self::with_reg_val().ats1e1wp_1_extract()
    }

    /// inserts the given value `val` into the field `ats1e1wp_1`
    pub fn ats1e1wp_1_insert(&mut self, val: u64) -> &mut self {
        // bits 17..17
        self.val.set_bits(17..=17, val);
        self
    }

    /// reads the register, updates the `ats1e1wp_1` field, and writes the updated value
    pub fn ats1e1wp_1_write(&mut self, val: u64) {
        Self::with_reg_val().ats1e1wp_1_insert(val).write();
    }

    /*
     * Field: ats1e1rp_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ats1e1rp_1_extract(&self) -> u64 {
        // bits 16..16
        self.val.get_bits(16..=16)
    }

    /// reads the current register value and extract field `ats1e1rp_1` from it
    pub fn ats1e1rp_1_read(&mut self) -> u64 {
        Self::with_reg_val().ats1e1rp_1_extract()
    }

    /// inserts the given value `val` into the field `ats1e1rp_1`
    pub fn ats1e1rp_1_insert(&mut self, val: u64) -> &mut self {
        // bits 16..16
        self.val.set_bits(16..=16, val);
        self
    }

    /// reads the register, updates the `ats1e1rp_1` field, and writes the updated value
    pub fn ats1e1rp_1_write(&mut self, val: u64) {
        Self::with_reg_val().ats1e1rp_1_insert(val).write();
    }

    /*
     * Field: ats1e0w
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ats1e0w_extract(&self) -> u64 {
        // bits 15..15
        self.val.get_bits(15..=15)
    }

    /// reads the current register value and extract field `ats1e0w` from it
    pub fn ats1e0w_read(&mut self) -> u64 {
        Self::with_reg_val().ats1e0w_extract()
    }

    /// inserts the given value `val` into the field `ats1e0w`
    pub fn ats1e0w_insert(&mut self, val: u64) -> &mut self {
        // bits 15..15
        self.val.set_bits(15..=15, val);
        self
    }

    /// reads the register, updates the `ats1e0w` field, and writes the updated value
    pub fn ats1e0w_write(&mut self, val: u64) {
        Self::with_reg_val().ats1e0w_insert(val).write();
    }

    /*
     * Field: ats1e0r
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ats1e0r_extract(&self) -> u64 {
        // bits 14..14
        self.val.get_bits(14..=14)
    }

    /// reads the current register value and extract field `ats1e0r` from it
    pub fn ats1e0r_read(&mut self) -> u64 {
        Self::with_reg_val().ats1e0r_extract()
    }

    /// inserts the given value `val` into the field `ats1e0r`
    pub fn ats1e0r_insert(&mut self, val: u64) -> &mut self {
        // bits 14..14
        self.val.set_bits(14..=14, val);
        self
    }

    /// reads the register, updates the `ats1e0r` field, and writes the updated value
    pub fn ats1e0r_write(&mut self, val: u64) {
        Self::with_reg_val().ats1e0r_insert(val).write();
    }

    /*
     * Field: ats1e1w
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ats1e1w_extract(&self) -> u64 {
        // bits 13..13
        self.val.get_bits(13..=13)
    }

    /// reads the current register value and extract field `ats1e1w` from it
    pub fn ats1e1w_read(&mut self) -> u64 {
        Self::with_reg_val().ats1e1w_extract()
    }

    /// inserts the given value `val` into the field `ats1e1w`
    pub fn ats1e1w_insert(&mut self, val: u64) -> &mut self {
        // bits 13..13
        self.val.set_bits(13..=13, val);
        self
    }

    /// reads the register, updates the `ats1e1w` field, and writes the updated value
    pub fn ats1e1w_write(&mut self, val: u64) {
        Self::with_reg_val().ats1e1w_insert(val).write();
    }

    /*
     * Field: ats1e1r
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ats1e1r_extract(&self) -> u64 {
        // bits 12..12
        self.val.get_bits(12..=12)
    }

    /// reads the current register value and extract field `ats1e1r` from it
    pub fn ats1e1r_read(&mut self) -> u64 {
        Self::with_reg_val().ats1e1r_extract()
    }

    /// inserts the given value `val` into the field `ats1e1r`
    pub fn ats1e1r_insert(&mut self, val: u64) -> &mut self {
        // bits 12..12
        self.val.set_bits(12..=12, val);
        self
    }

    /// reads the register, updates the `ats1e1r` field, and writes the updated value
    pub fn ats1e1r_write(&mut self, val: u64) {
        Self::with_reg_val().ats1e1r_insert(val).write();
    }

    /*
     * Field: dczva
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dczva_extract(&self) -> u64 {
        // bits 11..11
        self.val.get_bits(11..=11)
    }

    /// reads the current register value and extract field `dczva` from it
    pub fn dczva_read(&mut self) -> u64 {
        Self::with_reg_val().dczva_extract()
    }

    /// inserts the given value `val` into the field `dczva`
    pub fn dczva_insert(&mut self, val: u64) -> &mut self {
        // bits 11..11
        self.val.set_bits(11..=11, val);
        self
    }

    /// reads the register, updates the `dczva` field, and writes the updated value
    pub fn dczva_write(&mut self, val: u64) {
        Self::with_reg_val().dczva_insert(val).write();
    }

    /*
     * Field: dccivac
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dccivac_extract(&self) -> u64 {
        // bits 10..10
        self.val.get_bits(10..=10)
    }

    /// reads the current register value and extract field `dccivac` from it
    pub fn dccivac_read(&mut self) -> u64 {
        Self::with_reg_val().dccivac_extract()
    }

    /// inserts the given value `val` into the field `dccivac`
    pub fn dccivac_insert(&mut self, val: u64) -> &mut self {
        // bits 10..10
        self.val.set_bits(10..=10, val);
        self
    }

    /// reads the register, updates the `dccivac` field, and writes the updated value
    pub fn dccivac_write(&mut self, val: u64) {
        Self::with_reg_val().dccivac_insert(val).write();
    }

    /*
     * Field: dccvadp_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dccvadp_1_extract(&self) -> u64 {
        // bits 9..9
        self.val.get_bits(9..=9)
    }

    /// reads the current register value and extract field `dccvadp_1` from it
    pub fn dccvadp_1_read(&mut self) -> u64 {
        Self::with_reg_val().dccvadp_1_extract()
    }

    /// inserts the given value `val` into the field `dccvadp_1`
    pub fn dccvadp_1_insert(&mut self, val: u64) -> &mut self {
        // bits 9..9
        self.val.set_bits(9..=9, val);
        self
    }

    /// reads the register, updates the `dccvadp_1` field, and writes the updated value
    pub fn dccvadp_1_write(&mut self, val: u64) {
        Self::with_reg_val().dccvadp_1_insert(val).write();
    }

    /*
     * Field: dccvap
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dccvap_extract(&self) -> u64 {
        // bits 8..8
        self.val.get_bits(8..=8)
    }

    /// reads the current register value and extract field `dccvap` from it
    pub fn dccvap_read(&mut self) -> u64 {
        Self::with_reg_val().dccvap_extract()
    }

    /// inserts the given value `val` into the field `dccvap`
    pub fn dccvap_insert(&mut self, val: u64) -> &mut self {
        // bits 8..8
        self.val.set_bits(8..=8, val);
        self
    }

    /// reads the register, updates the `dccvap` field, and writes the updated value
    pub fn dccvap_write(&mut self, val: u64) {
        Self::with_reg_val().dccvap_insert(val).write();
    }

    /*
     * Field: dccvau
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dccvau_extract(&self) -> u64 {
        // bits 7..7
        self.val.get_bits(7..=7)
    }

    /// reads the current register value and extract field `dccvau` from it
    pub fn dccvau_read(&mut self) -> u64 {
        Self::with_reg_val().dccvau_extract()
    }

    /// inserts the given value `val` into the field `dccvau`
    pub fn dccvau_insert(&mut self, val: u64) -> &mut self {
        // bits 7..7
        self.val.set_bits(7..=7, val);
        self
    }

    /// reads the register, updates the `dccvau` field, and writes the updated value
    pub fn dccvau_write(&mut self, val: u64) {
        Self::with_reg_val().dccvau_insert(val).write();
    }

    /*
     * Field: dccisw
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dccisw_extract(&self) -> u64 {
        // bits 6..6
        self.val.get_bits(6..=6)
    }

    /// reads the current register value and extract field `dccisw` from it
    pub fn dccisw_read(&mut self) -> u64 {
        Self::with_reg_val().dccisw_extract()
    }

    /// inserts the given value `val` into the field `dccisw`
    pub fn dccisw_insert(&mut self, val: u64) -> &mut self {
        // bits 6..6
        self.val.set_bits(6..=6, val);
        self
    }

    /// reads the register, updates the `dccisw` field, and writes the updated value
    pub fn dccisw_write(&mut self, val: u64) {
        Self::with_reg_val().dccisw_insert(val).write();
    }

    /*
     * Field: dccsw
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dccsw_extract(&self) -> u64 {
        // bits 5..5
        self.val.get_bits(5..=5)
    }

    /// reads the current register value and extract field `dccsw` from it
    pub fn dccsw_read(&mut self) -> u64 {
        Self::with_reg_val().dccsw_extract()
    }

    /// inserts the given value `val` into the field `dccsw`
    pub fn dccsw_insert(&mut self, val: u64) -> &mut self {
        // bits 5..5
        self.val.set_bits(5..=5, val);
        self
    }

    /// reads the register, updates the `dccsw` field, and writes the updated value
    pub fn dccsw_write(&mut self, val: u64) {
        Self::with_reg_val().dccsw_insert(val).write();
    }

    /*
     * Field: dcisw
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dcisw_extract(&self) -> u64 {
        // bits 4..4
        self.val.get_bits(4..=4)
    }

    /// reads the current register value and extract field `dcisw` from it
    pub fn dcisw_read(&mut self) -> u64 {
        Self::with_reg_val().dcisw_extract()
    }

    /// inserts the given value `val` into the field `dcisw`
    pub fn dcisw_insert(&mut self, val: u64) -> &mut self {
        // bits 4..4
        self.val.set_bits(4..=4, val);
        self
    }

    /// reads the register, updates the `dcisw` field, and writes the updated value
    pub fn dcisw_write(&mut self, val: u64) {
        Self::with_reg_val().dcisw_insert(val).write();
    }

    /*
     * Field: dcivac
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dcivac_extract(&self) -> u64 {
        // bits 3..3
        self.val.get_bits(3..=3)
    }

    /// reads the current register value and extract field `dcivac` from it
    pub fn dcivac_read(&mut self) -> u64 {
        Self::with_reg_val().dcivac_extract()
    }

    /// inserts the given value `val` into the field `dcivac`
    pub fn dcivac_insert(&mut self, val: u64) -> &mut self {
        // bits 3..3
        self.val.set_bits(3..=3, val);
        self
    }

    /// reads the register, updates the `dcivac` field, and writes the updated value
    pub fn dcivac_write(&mut self, val: u64) {
        Self::with_reg_val().dcivac_insert(val).write();
    }

    /*
     * Field: icivau
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn icivau_extract(&self) -> u64 {
        // bits 2..2
        self.val.get_bits(2..=2)
    }

    /// reads the current register value and extract field `icivau` from it
    pub fn icivau_read(&mut self) -> u64 {
        Self::with_reg_val().icivau_extract()
    }

    /// inserts the given value `val` into the field `icivau`
    pub fn icivau_insert(&mut self, val: u64) -> &mut self {
        // bits 2..2
        self.val.set_bits(2..=2, val);
        self
    }

    /// reads the register, updates the `icivau` field, and writes the updated value
    pub fn icivau_write(&mut self, val: u64) {
        Self::with_reg_val().icivau_insert(val).write();
    }

    /*
     * Field: iciallu
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn iciallu_extract(&self) -> u64 {
        // bits 1..1
        self.val.get_bits(1..=1)
    }

    /// reads the current register value and extract field `iciallu` from it
    pub fn iciallu_read(&mut self) -> u64 {
        Self::with_reg_val().iciallu_extract()
    }

    /// inserts the given value `val` into the field `iciallu`
    pub fn iciallu_insert(&mut self, val: u64) -> &mut self {
        // bits 1..1
        self.val.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `iciallu` field, and writes the updated value
    pub fn iciallu_write(&mut self, val: u64) {
        Self::with_reg_val().iciallu_insert(val).write();
    }

    /*
     * Field: icialluis
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn icialluis_extract(&self) -> u64 {
        // bits 0..0
        self.val.get_bits(0..=0)
    }

    /// reads the current register value and extract field `icialluis` from it
    pub fn icialluis_read(&mut self) -> u64 {
        Self::with_reg_val().icialluis_extract()
    }

    /// inserts the given value `val` into the field `icialluis`
    pub fn icialluis_insert(&mut self, val: u64) -> &mut self {
        // bits 0..0
        self.val.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `icialluis` field, and writes the updated value
    pub fn icialluis_write(&mut self, val: u64) {
        Self::with_reg_val().icialluis_insert(val).write();
    }
}

impl Default for HfgitrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> HfgitrEl2 {
        HfgitrEl2(0)
    }
}
