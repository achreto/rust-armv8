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
 * Generated on: 2022-08-22T15:51:28.514928
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
 * Register:    Hypervisor Activity Monitors Fine-Grained Read Trap Register (hafgrtr_el2)
 * Group:       A group mapping that does not have a known primary
 * Type:        64-bit Register
 * Description: Provides controls for traps of
 * File:        AArch64-hafgrtr_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Hypervisor Activity Monitors Fine-Grained Read Trap Register value in memory
pub struct HafgrtrEl2(u64);

/// struct implementation for accessing the fields of register hafgrtr_el2
impl HafgrtrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> HafgrtrEl2 {
        Self::default()
    }

    /// collects the modifications of HafgrtrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> HafgrtrEl2 {
        HafgrtrEl2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> HafgrtrEl2 {
        let curval = Self::reg_rawrd() & 0x3fffffffe001f;
        HafgrtrEl2(curval)
    }

    /// reading the Hypervisor Activity Monitors Fine-Grained Read Trap Register (hafgrtr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, HAFGRTR_EL2
            llvm_asm!("mrs $0, S3_4_C3_C1_6" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Hypervisor Activity Monitors Fine-Grained Read Trap Register (hafgrtr_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR HAFGRTR_EL2, <Xt>
            llvm_asm!("msr S3_4_C3_C1_6, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0x3fffffffe001f;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 1125899906711583;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: amevtyper1x_el0
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevtyper1x_el0_extract(&self) -> u64 {
        // bits 49..49
        self.val.get_bits(49..=49)
    }

    /// reads the current register value and extract field `amevtyper1x_el0` from it
    pub fn amevtyper1x_el0_read(&mut self) -> u64 {
        Self::with_reg_val().amevtyper1x_el0_extract()
    }

    /// inserts the given value `val` into the field `amevtyper1x_el0`
    pub fn amevtyper1x_el0_insert(&mut self, val: u64) -> &mut self {
        // bits 49..49
        self.val.set_bits(49..=49, val);
        self
    }

    /// reads the register, updates the `amevtyper1x_el0` field, and writes the updated value
    pub fn amevtyper1x_el0_write(&mut self, val: u64) {
        Self::with_reg_val().amevtyper1x_el0_insert(val).write();
    }

    /*
     * Field: amevtyper115_el0_49_49
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevtyper115_el0_49_49_extract(&self) -> u64 {
        // bits 49..49
        self.val.get_bits(49..=49)
    }

    /// reads the current register value and extract field `amevtyper115_el0_49_49` from it
    pub fn amevtyper115_el0_49_49_read(&mut self) -> u64 {
        Self::with_reg_val().amevtyper115_el0_49_49_extract()
    }

    /// inserts the given value `val` into the field `amevtyper115_el0_49_49`
    pub fn amevtyper115_el0_49_49_insert(&mut self, val: u64) -> &mut self {
        // bits 49..49
        self.val.set_bits(49..=49, val);
        self
    }

    /// reads the register, updates the `amevtyper115_el0_49_49` field, and writes the updated value
    pub fn amevtyper115_el0_49_49_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevtyper115_el0_49_49_insert(val)
            .write();
    }

    /*
     * Field: amevcntr1x_el0
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevcntr1x_el0_extract(&self) -> u64 {
        // bits 48..48
        self.val.get_bits(48..=48)
    }

    /// reads the current register value and extract field `amevcntr1x_el0` from it
    pub fn amevcntr1x_el0_read(&mut self) -> u64 {
        Self::with_reg_val().amevcntr1x_el0_extract()
    }

    /// inserts the given value `val` into the field `amevcntr1x_el0`
    pub fn amevcntr1x_el0_insert(&mut self, val: u64) -> &mut self {
        // bits 48..48
        self.val.set_bits(48..=48, val);
        self
    }

    /// reads the register, updates the `amevcntr1x_el0` field, and writes the updated value
    pub fn amevcntr1x_el0_write(&mut self, val: u64) {
        Self::with_reg_val().amevcntr1x_el0_insert(val).write();
    }

    /*
     * Field: amevcntr115_el0_48_48
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevcntr115_el0_48_48_extract(&self) -> u64 {
        // bits 48..48
        self.val.get_bits(48..=48)
    }

    /// reads the current register value and extract field `amevcntr115_el0_48_48` from it
    pub fn amevcntr115_el0_48_48_read(&mut self) -> u64 {
        Self::with_reg_val().amevcntr115_el0_48_48_extract()
    }

    /// inserts the given value `val` into the field `amevcntr115_el0_48_48`
    pub fn amevcntr115_el0_48_48_insert(&mut self, val: u64) -> &mut self {
        // bits 48..48
        self.val.set_bits(48..=48, val);
        self
    }

    /// reads the register, updates the `amevcntr115_el0_48_48` field, and writes the updated value
    pub fn amevcntr115_el0_48_48_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevcntr115_el0_48_48_insert(val)
            .write();
    }

    /*
     * Field: amevtyper114_el0_47_47
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevtyper114_el0_47_47_extract(&self) -> u64 {
        // bits 47..47
        self.val.get_bits(47..=47)
    }

    /// reads the current register value and extract field `amevtyper114_el0_47_47` from it
    pub fn amevtyper114_el0_47_47_read(&mut self) -> u64 {
        Self::with_reg_val().amevtyper114_el0_47_47_extract()
    }

    /// inserts the given value `val` into the field `amevtyper114_el0_47_47`
    pub fn amevtyper114_el0_47_47_insert(&mut self, val: u64) -> &mut self {
        // bits 47..47
        self.val.set_bits(47..=47, val);
        self
    }

    /// reads the register, updates the `amevtyper114_el0_47_47` field, and writes the updated value
    pub fn amevtyper114_el0_47_47_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevtyper114_el0_47_47_insert(val)
            .write();
    }

    /*
     * Field: amevcntr114_el0_46_46
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevcntr114_el0_46_46_extract(&self) -> u64 {
        // bits 46..46
        self.val.get_bits(46..=46)
    }

    /// reads the current register value and extract field `amevcntr114_el0_46_46` from it
    pub fn amevcntr114_el0_46_46_read(&mut self) -> u64 {
        Self::with_reg_val().amevcntr114_el0_46_46_extract()
    }

    /// inserts the given value `val` into the field `amevcntr114_el0_46_46`
    pub fn amevcntr114_el0_46_46_insert(&mut self, val: u64) -> &mut self {
        // bits 46..46
        self.val.set_bits(46..=46, val);
        self
    }

    /// reads the register, updates the `amevcntr114_el0_46_46` field, and writes the updated value
    pub fn amevcntr114_el0_46_46_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevcntr114_el0_46_46_insert(val)
            .write();
    }

    /*
     * Field: amevtyper113_el0_45_45
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevtyper113_el0_45_45_extract(&self) -> u64 {
        // bits 45..45
        self.val.get_bits(45..=45)
    }

    /// reads the current register value and extract field `amevtyper113_el0_45_45` from it
    pub fn amevtyper113_el0_45_45_read(&mut self) -> u64 {
        Self::with_reg_val().amevtyper113_el0_45_45_extract()
    }

    /// inserts the given value `val` into the field `amevtyper113_el0_45_45`
    pub fn amevtyper113_el0_45_45_insert(&mut self, val: u64) -> &mut self {
        // bits 45..45
        self.val.set_bits(45..=45, val);
        self
    }

    /// reads the register, updates the `amevtyper113_el0_45_45` field, and writes the updated value
    pub fn amevtyper113_el0_45_45_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevtyper113_el0_45_45_insert(val)
            .write();
    }

    /*
     * Field: amevcntr113_el0_44_44
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevcntr113_el0_44_44_extract(&self) -> u64 {
        // bits 44..44
        self.val.get_bits(44..=44)
    }

    /// reads the current register value and extract field `amevcntr113_el0_44_44` from it
    pub fn amevcntr113_el0_44_44_read(&mut self) -> u64 {
        Self::with_reg_val().amevcntr113_el0_44_44_extract()
    }

    /// inserts the given value `val` into the field `amevcntr113_el0_44_44`
    pub fn amevcntr113_el0_44_44_insert(&mut self, val: u64) -> &mut self {
        // bits 44..44
        self.val.set_bits(44..=44, val);
        self
    }

    /// reads the register, updates the `amevcntr113_el0_44_44` field, and writes the updated value
    pub fn amevcntr113_el0_44_44_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevcntr113_el0_44_44_insert(val)
            .write();
    }

    /*
     * Field: amevtyper112_el0_43_43
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevtyper112_el0_43_43_extract(&self) -> u64 {
        // bits 43..43
        self.val.get_bits(43..=43)
    }

    /// reads the current register value and extract field `amevtyper112_el0_43_43` from it
    pub fn amevtyper112_el0_43_43_read(&mut self) -> u64 {
        Self::with_reg_val().amevtyper112_el0_43_43_extract()
    }

    /// inserts the given value `val` into the field `amevtyper112_el0_43_43`
    pub fn amevtyper112_el0_43_43_insert(&mut self, val: u64) -> &mut self {
        // bits 43..43
        self.val.set_bits(43..=43, val);
        self
    }

    /// reads the register, updates the `amevtyper112_el0_43_43` field, and writes the updated value
    pub fn amevtyper112_el0_43_43_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevtyper112_el0_43_43_insert(val)
            .write();
    }

    /*
     * Field: amevcntr112_el0_42_42
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevcntr112_el0_42_42_extract(&self) -> u64 {
        // bits 42..42
        self.val.get_bits(42..=42)
    }

    /// reads the current register value and extract field `amevcntr112_el0_42_42` from it
    pub fn amevcntr112_el0_42_42_read(&mut self) -> u64 {
        Self::with_reg_val().amevcntr112_el0_42_42_extract()
    }

    /// inserts the given value `val` into the field `amevcntr112_el0_42_42`
    pub fn amevcntr112_el0_42_42_insert(&mut self, val: u64) -> &mut self {
        // bits 42..42
        self.val.set_bits(42..=42, val);
        self
    }

    /// reads the register, updates the `amevcntr112_el0_42_42` field, and writes the updated value
    pub fn amevcntr112_el0_42_42_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevcntr112_el0_42_42_insert(val)
            .write();
    }

    /*
     * Field: amevtyper111_el0_41_41
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevtyper111_el0_41_41_extract(&self) -> u64 {
        // bits 41..41
        self.val.get_bits(41..=41)
    }

    /// reads the current register value and extract field `amevtyper111_el0_41_41` from it
    pub fn amevtyper111_el0_41_41_read(&mut self) -> u64 {
        Self::with_reg_val().amevtyper111_el0_41_41_extract()
    }

    /// inserts the given value `val` into the field `amevtyper111_el0_41_41`
    pub fn amevtyper111_el0_41_41_insert(&mut self, val: u64) -> &mut self {
        // bits 41..41
        self.val.set_bits(41..=41, val);
        self
    }

    /// reads the register, updates the `amevtyper111_el0_41_41` field, and writes the updated value
    pub fn amevtyper111_el0_41_41_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevtyper111_el0_41_41_insert(val)
            .write();
    }

    /*
     * Field: amevcntr111_el0_40_40
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevcntr111_el0_40_40_extract(&self) -> u64 {
        // bits 40..40
        self.val.get_bits(40..=40)
    }

    /// reads the current register value and extract field `amevcntr111_el0_40_40` from it
    pub fn amevcntr111_el0_40_40_read(&mut self) -> u64 {
        Self::with_reg_val().amevcntr111_el0_40_40_extract()
    }

    /// inserts the given value `val` into the field `amevcntr111_el0_40_40`
    pub fn amevcntr111_el0_40_40_insert(&mut self, val: u64) -> &mut self {
        // bits 40..40
        self.val.set_bits(40..=40, val);
        self
    }

    /// reads the register, updates the `amevcntr111_el0_40_40` field, and writes the updated value
    pub fn amevcntr111_el0_40_40_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevcntr111_el0_40_40_insert(val)
            .write();
    }

    /*
     * Field: amevtyper110_el0_39_39
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevtyper110_el0_39_39_extract(&self) -> u64 {
        // bits 39..39
        self.val.get_bits(39..=39)
    }

    /// reads the current register value and extract field `amevtyper110_el0_39_39` from it
    pub fn amevtyper110_el0_39_39_read(&mut self) -> u64 {
        Self::with_reg_val().amevtyper110_el0_39_39_extract()
    }

    /// inserts the given value `val` into the field `amevtyper110_el0_39_39`
    pub fn amevtyper110_el0_39_39_insert(&mut self, val: u64) -> &mut self {
        // bits 39..39
        self.val.set_bits(39..=39, val);
        self
    }

    /// reads the register, updates the `amevtyper110_el0_39_39` field, and writes the updated value
    pub fn amevtyper110_el0_39_39_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevtyper110_el0_39_39_insert(val)
            .write();
    }

    /*
     * Field: amevcntr110_el0_38_38
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevcntr110_el0_38_38_extract(&self) -> u64 {
        // bits 38..38
        self.val.get_bits(38..=38)
    }

    /// reads the current register value and extract field `amevcntr110_el0_38_38` from it
    pub fn amevcntr110_el0_38_38_read(&mut self) -> u64 {
        Self::with_reg_val().amevcntr110_el0_38_38_extract()
    }

    /// inserts the given value `val` into the field `amevcntr110_el0_38_38`
    pub fn amevcntr110_el0_38_38_insert(&mut self, val: u64) -> &mut self {
        // bits 38..38
        self.val.set_bits(38..=38, val);
        self
    }

    /// reads the register, updates the `amevcntr110_el0_38_38` field, and writes the updated value
    pub fn amevcntr110_el0_38_38_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevcntr110_el0_38_38_insert(val)
            .write();
    }

    /*
     * Field: amevtyper19_el0_37_37
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevtyper19_el0_37_37_extract(&self) -> u64 {
        // bits 37..37
        self.val.get_bits(37..=37)
    }

    /// reads the current register value and extract field `amevtyper19_el0_37_37` from it
    pub fn amevtyper19_el0_37_37_read(&mut self) -> u64 {
        Self::with_reg_val().amevtyper19_el0_37_37_extract()
    }

    /// inserts the given value `val` into the field `amevtyper19_el0_37_37`
    pub fn amevtyper19_el0_37_37_insert(&mut self, val: u64) -> &mut self {
        // bits 37..37
        self.val.set_bits(37..=37, val);
        self
    }

    /// reads the register, updates the `amevtyper19_el0_37_37` field, and writes the updated value
    pub fn amevtyper19_el0_37_37_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevtyper19_el0_37_37_insert(val)
            .write();
    }

    /*
     * Field: amevcntr19_el0_36_36
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevcntr19_el0_36_36_extract(&self) -> u64 {
        // bits 36..36
        self.val.get_bits(36..=36)
    }

    /// reads the current register value and extract field `amevcntr19_el0_36_36` from it
    pub fn amevcntr19_el0_36_36_read(&mut self) -> u64 {
        Self::with_reg_val().amevcntr19_el0_36_36_extract()
    }

    /// inserts the given value `val` into the field `amevcntr19_el0_36_36`
    pub fn amevcntr19_el0_36_36_insert(&mut self, val: u64) -> &mut self {
        // bits 36..36
        self.val.set_bits(36..=36, val);
        self
    }

    /// reads the register, updates the `amevcntr19_el0_36_36` field, and writes the updated value
    pub fn amevcntr19_el0_36_36_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevcntr19_el0_36_36_insert(val)
            .write();
    }

    /*
     * Field: amevtyper18_el0_35_35
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevtyper18_el0_35_35_extract(&self) -> u64 {
        // bits 35..35
        self.val.get_bits(35..=35)
    }

    /// reads the current register value and extract field `amevtyper18_el0_35_35` from it
    pub fn amevtyper18_el0_35_35_read(&mut self) -> u64 {
        Self::with_reg_val().amevtyper18_el0_35_35_extract()
    }

    /// inserts the given value `val` into the field `amevtyper18_el0_35_35`
    pub fn amevtyper18_el0_35_35_insert(&mut self, val: u64) -> &mut self {
        // bits 35..35
        self.val.set_bits(35..=35, val);
        self
    }

    /// reads the register, updates the `amevtyper18_el0_35_35` field, and writes the updated value
    pub fn amevtyper18_el0_35_35_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevtyper18_el0_35_35_insert(val)
            .write();
    }

    /*
     * Field: amevcntr18_el0_34_34
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevcntr18_el0_34_34_extract(&self) -> u64 {
        // bits 34..34
        self.val.get_bits(34..=34)
    }

    /// reads the current register value and extract field `amevcntr18_el0_34_34` from it
    pub fn amevcntr18_el0_34_34_read(&mut self) -> u64 {
        Self::with_reg_val().amevcntr18_el0_34_34_extract()
    }

    /// inserts the given value `val` into the field `amevcntr18_el0_34_34`
    pub fn amevcntr18_el0_34_34_insert(&mut self, val: u64) -> &mut self {
        // bits 34..34
        self.val.set_bits(34..=34, val);
        self
    }

    /// reads the register, updates the `amevcntr18_el0_34_34` field, and writes the updated value
    pub fn amevcntr18_el0_34_34_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevcntr18_el0_34_34_insert(val)
            .write();
    }

    /*
     * Field: amevtyper17_el0_33_33
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevtyper17_el0_33_33_extract(&self) -> u64 {
        // bits 33..33
        self.val.get_bits(33..=33)
    }

    /// reads the current register value and extract field `amevtyper17_el0_33_33` from it
    pub fn amevtyper17_el0_33_33_read(&mut self) -> u64 {
        Self::with_reg_val().amevtyper17_el0_33_33_extract()
    }

    /// inserts the given value `val` into the field `amevtyper17_el0_33_33`
    pub fn amevtyper17_el0_33_33_insert(&mut self, val: u64) -> &mut self {
        // bits 33..33
        self.val.set_bits(33..=33, val);
        self
    }

    /// reads the register, updates the `amevtyper17_el0_33_33` field, and writes the updated value
    pub fn amevtyper17_el0_33_33_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevtyper17_el0_33_33_insert(val)
            .write();
    }

    /*
     * Field: amevcntr17_el0_32_32
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevcntr17_el0_32_32_extract(&self) -> u64 {
        // bits 32..32
        self.val.get_bits(32..=32)
    }

    /// reads the current register value and extract field `amevcntr17_el0_32_32` from it
    pub fn amevcntr17_el0_32_32_read(&mut self) -> u64 {
        Self::with_reg_val().amevcntr17_el0_32_32_extract()
    }

    /// inserts the given value `val` into the field `amevcntr17_el0_32_32`
    pub fn amevcntr17_el0_32_32_insert(&mut self, val: u64) -> &mut self {
        // bits 32..32
        self.val.set_bits(32..=32, val);
        self
    }

    /// reads the register, updates the `amevcntr17_el0_32_32` field, and writes the updated value
    pub fn amevcntr17_el0_32_32_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevcntr17_el0_32_32_insert(val)
            .write();
    }

    /*
     * Field: amevtyper16_el0_31_31
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevtyper16_el0_31_31_extract(&self) -> u64 {
        // bits 31..31
        self.val.get_bits(31..=31)
    }

    /// reads the current register value and extract field `amevtyper16_el0_31_31` from it
    pub fn amevtyper16_el0_31_31_read(&mut self) -> u64 {
        Self::with_reg_val().amevtyper16_el0_31_31_extract()
    }

    /// inserts the given value `val` into the field `amevtyper16_el0_31_31`
    pub fn amevtyper16_el0_31_31_insert(&mut self, val: u64) -> &mut self {
        // bits 31..31
        self.val.set_bits(31..=31, val);
        self
    }

    /// reads the register, updates the `amevtyper16_el0_31_31` field, and writes the updated value
    pub fn amevtyper16_el0_31_31_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevtyper16_el0_31_31_insert(val)
            .write();
    }

    /*
     * Field: amevcntr16_el0_30_30
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevcntr16_el0_30_30_extract(&self) -> u64 {
        // bits 30..30
        self.val.get_bits(30..=30)
    }

    /// reads the current register value and extract field `amevcntr16_el0_30_30` from it
    pub fn amevcntr16_el0_30_30_read(&mut self) -> u64 {
        Self::with_reg_val().amevcntr16_el0_30_30_extract()
    }

    /// inserts the given value `val` into the field `amevcntr16_el0_30_30`
    pub fn amevcntr16_el0_30_30_insert(&mut self, val: u64) -> &mut self {
        // bits 30..30
        self.val.set_bits(30..=30, val);
        self
    }

    /// reads the register, updates the `amevcntr16_el0_30_30` field, and writes the updated value
    pub fn amevcntr16_el0_30_30_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevcntr16_el0_30_30_insert(val)
            .write();
    }

    /*
     * Field: amevtyper15_el0_29_29
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevtyper15_el0_29_29_extract(&self) -> u64 {
        // bits 29..29
        self.val.get_bits(29..=29)
    }

    /// reads the current register value and extract field `amevtyper15_el0_29_29` from it
    pub fn amevtyper15_el0_29_29_read(&mut self) -> u64 {
        Self::with_reg_val().amevtyper15_el0_29_29_extract()
    }

    /// inserts the given value `val` into the field `amevtyper15_el0_29_29`
    pub fn amevtyper15_el0_29_29_insert(&mut self, val: u64) -> &mut self {
        // bits 29..29
        self.val.set_bits(29..=29, val);
        self
    }

    /// reads the register, updates the `amevtyper15_el0_29_29` field, and writes the updated value
    pub fn amevtyper15_el0_29_29_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevtyper15_el0_29_29_insert(val)
            .write();
    }

    /*
     * Field: amevcntr15_el0_28_28
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevcntr15_el0_28_28_extract(&self) -> u64 {
        // bits 28..28
        self.val.get_bits(28..=28)
    }

    /// reads the current register value and extract field `amevcntr15_el0_28_28` from it
    pub fn amevcntr15_el0_28_28_read(&mut self) -> u64 {
        Self::with_reg_val().amevcntr15_el0_28_28_extract()
    }

    /// inserts the given value `val` into the field `amevcntr15_el0_28_28`
    pub fn amevcntr15_el0_28_28_insert(&mut self, val: u64) -> &mut self {
        // bits 28..28
        self.val.set_bits(28..=28, val);
        self
    }

    /// reads the register, updates the `amevcntr15_el0_28_28` field, and writes the updated value
    pub fn amevcntr15_el0_28_28_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevcntr15_el0_28_28_insert(val)
            .write();
    }

    /*
     * Field: amevtyper14_el0_27_27
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevtyper14_el0_27_27_extract(&self) -> u64 {
        // bits 27..27
        self.val.get_bits(27..=27)
    }

    /// reads the current register value and extract field `amevtyper14_el0_27_27` from it
    pub fn amevtyper14_el0_27_27_read(&mut self) -> u64 {
        Self::with_reg_val().amevtyper14_el0_27_27_extract()
    }

    /// inserts the given value `val` into the field `amevtyper14_el0_27_27`
    pub fn amevtyper14_el0_27_27_insert(&mut self, val: u64) -> &mut self {
        // bits 27..27
        self.val.set_bits(27..=27, val);
        self
    }

    /// reads the register, updates the `amevtyper14_el0_27_27` field, and writes the updated value
    pub fn amevtyper14_el0_27_27_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevtyper14_el0_27_27_insert(val)
            .write();
    }

    /*
     * Field: amevcntr14_el0_26_26
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevcntr14_el0_26_26_extract(&self) -> u64 {
        // bits 26..26
        self.val.get_bits(26..=26)
    }

    /// reads the current register value and extract field `amevcntr14_el0_26_26` from it
    pub fn amevcntr14_el0_26_26_read(&mut self) -> u64 {
        Self::with_reg_val().amevcntr14_el0_26_26_extract()
    }

    /// inserts the given value `val` into the field `amevcntr14_el0_26_26`
    pub fn amevcntr14_el0_26_26_insert(&mut self, val: u64) -> &mut self {
        // bits 26..26
        self.val.set_bits(26..=26, val);
        self
    }

    /// reads the register, updates the `amevcntr14_el0_26_26` field, and writes the updated value
    pub fn amevcntr14_el0_26_26_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevcntr14_el0_26_26_insert(val)
            .write();
    }

    /*
     * Field: amevtyper13_el0_25_25
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevtyper13_el0_25_25_extract(&self) -> u64 {
        // bits 25..25
        self.val.get_bits(25..=25)
    }

    /// reads the current register value and extract field `amevtyper13_el0_25_25` from it
    pub fn amevtyper13_el0_25_25_read(&mut self) -> u64 {
        Self::with_reg_val().amevtyper13_el0_25_25_extract()
    }

    /// inserts the given value `val` into the field `amevtyper13_el0_25_25`
    pub fn amevtyper13_el0_25_25_insert(&mut self, val: u64) -> &mut self {
        // bits 25..25
        self.val.set_bits(25..=25, val);
        self
    }

    /// reads the register, updates the `amevtyper13_el0_25_25` field, and writes the updated value
    pub fn amevtyper13_el0_25_25_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevtyper13_el0_25_25_insert(val)
            .write();
    }

    /*
     * Field: amevcntr13_el0_24_24
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevcntr13_el0_24_24_extract(&self) -> u64 {
        // bits 24..24
        self.val.get_bits(24..=24)
    }

    /// reads the current register value and extract field `amevcntr13_el0_24_24` from it
    pub fn amevcntr13_el0_24_24_read(&mut self) -> u64 {
        Self::with_reg_val().amevcntr13_el0_24_24_extract()
    }

    /// inserts the given value `val` into the field `amevcntr13_el0_24_24`
    pub fn amevcntr13_el0_24_24_insert(&mut self, val: u64) -> &mut self {
        // bits 24..24
        self.val.set_bits(24..=24, val);
        self
    }

    /// reads the register, updates the `amevcntr13_el0_24_24` field, and writes the updated value
    pub fn amevcntr13_el0_24_24_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevcntr13_el0_24_24_insert(val)
            .write();
    }

    /*
     * Field: amevtyper12_el0_23_23
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevtyper12_el0_23_23_extract(&self) -> u64 {
        // bits 23..23
        self.val.get_bits(23..=23)
    }

    /// reads the current register value and extract field `amevtyper12_el0_23_23` from it
    pub fn amevtyper12_el0_23_23_read(&mut self) -> u64 {
        Self::with_reg_val().amevtyper12_el0_23_23_extract()
    }

    /// inserts the given value `val` into the field `amevtyper12_el0_23_23`
    pub fn amevtyper12_el0_23_23_insert(&mut self, val: u64) -> &mut self {
        // bits 23..23
        self.val.set_bits(23..=23, val);
        self
    }

    /// reads the register, updates the `amevtyper12_el0_23_23` field, and writes the updated value
    pub fn amevtyper12_el0_23_23_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevtyper12_el0_23_23_insert(val)
            .write();
    }

    /*
     * Field: amevcntr12_el0_22_22
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevcntr12_el0_22_22_extract(&self) -> u64 {
        // bits 22..22
        self.val.get_bits(22..=22)
    }

    /// reads the current register value and extract field `amevcntr12_el0_22_22` from it
    pub fn amevcntr12_el0_22_22_read(&mut self) -> u64 {
        Self::with_reg_val().amevcntr12_el0_22_22_extract()
    }

    /// inserts the given value `val` into the field `amevcntr12_el0_22_22`
    pub fn amevcntr12_el0_22_22_insert(&mut self, val: u64) -> &mut self {
        // bits 22..22
        self.val.set_bits(22..=22, val);
        self
    }

    /// reads the register, updates the `amevcntr12_el0_22_22` field, and writes the updated value
    pub fn amevcntr12_el0_22_22_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevcntr12_el0_22_22_insert(val)
            .write();
    }

    /*
     * Field: amevtyper11_el0_21_21
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevtyper11_el0_21_21_extract(&self) -> u64 {
        // bits 21..21
        self.val.get_bits(21..=21)
    }

    /// reads the current register value and extract field `amevtyper11_el0_21_21` from it
    pub fn amevtyper11_el0_21_21_read(&mut self) -> u64 {
        Self::with_reg_val().amevtyper11_el0_21_21_extract()
    }

    /// inserts the given value `val` into the field `amevtyper11_el0_21_21`
    pub fn amevtyper11_el0_21_21_insert(&mut self, val: u64) -> &mut self {
        // bits 21..21
        self.val.set_bits(21..=21, val);
        self
    }

    /// reads the register, updates the `amevtyper11_el0_21_21` field, and writes the updated value
    pub fn amevtyper11_el0_21_21_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevtyper11_el0_21_21_insert(val)
            .write();
    }

    /*
     * Field: amevcntr11_el0_20_20
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevcntr11_el0_20_20_extract(&self) -> u64 {
        // bits 20..20
        self.val.get_bits(20..=20)
    }

    /// reads the current register value and extract field `amevcntr11_el0_20_20` from it
    pub fn amevcntr11_el0_20_20_read(&mut self) -> u64 {
        Self::with_reg_val().amevcntr11_el0_20_20_extract()
    }

    /// inserts the given value `val` into the field `amevcntr11_el0_20_20`
    pub fn amevcntr11_el0_20_20_insert(&mut self, val: u64) -> &mut self {
        // bits 20..20
        self.val.set_bits(20..=20, val);
        self
    }

    /// reads the register, updates the `amevcntr11_el0_20_20` field, and writes the updated value
    pub fn amevcntr11_el0_20_20_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevcntr11_el0_20_20_insert(val)
            .write();
    }

    /*
     * Field: amevtyper10_el0_19_19
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevtyper10_el0_19_19_extract(&self) -> u64 {
        // bits 19..19
        self.val.get_bits(19..=19)
    }

    /// reads the current register value and extract field `amevtyper10_el0_19_19` from it
    pub fn amevtyper10_el0_19_19_read(&mut self) -> u64 {
        Self::with_reg_val().amevtyper10_el0_19_19_extract()
    }

    /// inserts the given value `val` into the field `amevtyper10_el0_19_19`
    pub fn amevtyper10_el0_19_19_insert(&mut self, val: u64) -> &mut self {
        // bits 19..19
        self.val.set_bits(19..=19, val);
        self
    }

    /// reads the register, updates the `amevtyper10_el0_19_19` field, and writes the updated value
    pub fn amevtyper10_el0_19_19_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevtyper10_el0_19_19_insert(val)
            .write();
    }

    /*
     * Field: amevcntr10_el0_18_18
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevcntr10_el0_18_18_extract(&self) -> u64 {
        // bits 18..18
        self.val.get_bits(18..=18)
    }

    /// reads the current register value and extract field `amevcntr10_el0_18_18` from it
    pub fn amevcntr10_el0_18_18_read(&mut self) -> u64 {
        Self::with_reg_val().amevcntr10_el0_18_18_extract()
    }

    /// inserts the given value `val` into the field `amevcntr10_el0_18_18`
    pub fn amevcntr10_el0_18_18_insert(&mut self, val: u64) -> &mut self {
        // bits 18..18
        self.val.set_bits(18..=18, val);
        self
    }

    /// reads the register, updates the `amevcntr10_el0_18_18` field, and writes the updated value
    pub fn amevcntr10_el0_18_18_write(&mut self, val: u64) {
        Self::with_reg_val()
            .amevcntr10_el0_18_18_insert(val)
            .write();
    }

    /*
     * Field: amcntenx
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amcntenx_extract(&self) -> u64 {
        // bits 17..17
        self.val.get_bits(17..=17)
    }

    /// reads the current register value and extract field `amcntenx` from it
    pub fn amcntenx_read(&mut self) -> u64 {
        Self::with_reg_val().amcntenx_extract()
    }

    /// inserts the given value `val` into the field `amcntenx`
    pub fn amcntenx_insert(&mut self, val: u64) -> &mut self {
        // bits 17..17
        self.val.set_bits(17..=17, val);
        self
    }

    /// reads the register, updates the `amcntenx` field, and writes the updated value
    pub fn amcntenx_write(&mut self, val: u64) {
        Self::with_reg_val().amcntenx_insert(val).write();
    }

    /*
     * Field: amcnten1_17_17
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amcnten1_17_17_extract(&self) -> u64 {
        // bits 17..17
        self.val.get_bits(17..=17)
    }

    /// reads the current register value and extract field `amcnten1_17_17` from it
    pub fn amcnten1_17_17_read(&mut self) -> u64 {
        Self::with_reg_val().amcnten1_17_17_extract()
    }

    /// inserts the given value `val` into the field `amcnten1_17_17`
    pub fn amcnten1_17_17_insert(&mut self, val: u64) -> &mut self {
        // bits 17..17
        self.val.set_bits(17..=17, val);
        self
    }

    /// reads the register, updates the `amcnten1_17_17` field, and writes the updated value
    pub fn amcnten1_17_17_write(&mut self, val: u64) {
        Self::with_reg_val().amcnten1_17_17_insert(val).write();
    }

    /*
     * Field: amevcntr0x_el0
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amevcntr0x_el0_extract(&self) -> u64 {
        // bits 1..4
        self.val.get_bits(1..=4)
    }

    /// reads the current register value and extract field `amevcntr0x_el0` from it
    pub fn amevcntr0x_el0_read(&mut self) -> u64 {
        Self::with_reg_val().amevcntr0x_el0_extract()
    }

    /// inserts the given value `val` into the field `amevcntr0x_el0`
    pub fn amevcntr0x_el0_insert(&mut self, val: u64) -> &mut self {
        // bits 1..4
        self.val.set_bits(1..=4, val);
        self
    }

    /// reads the register, updates the `amevcntr0x_el0` field, and writes the updated value
    pub fn amevcntr0x_el0_write(&mut self, val: u64) {
        Self::with_reg_val().amevcntr0x_el0_insert(val).write();
    }

    /*
     * Field: amcnten0_0_0
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amcnten0_0_0_extract(&self) -> u64 {
        // bits 0..0
        self.val.get_bits(0..=0)
    }

    /// reads the current register value and extract field `amcnten0_0_0` from it
    pub fn amcnten0_0_0_read(&mut self) -> u64 {
        Self::with_reg_val().amcnten0_0_0_extract()
    }

    /// inserts the given value `val` into the field `amcnten0_0_0`
    pub fn amcnten0_0_0_insert(&mut self, val: u64) -> &mut self {
        // bits 0..0
        self.val.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `amcnten0_0_0` field, and writes the updated value
    pub fn amcnten0_0_0_write(&mut self, val: u64) {
        Self::with_reg_val().amcnten0_0_0_insert(val).write();
    }
}

impl Default for HafgrtrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> HafgrtrEl2 {
        HafgrtrEl2(0)
    }
}
