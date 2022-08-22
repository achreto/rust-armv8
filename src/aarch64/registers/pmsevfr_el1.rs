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

use core::arch::asm;
use bit_field::BitField;


/**************************************************************************************************
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 * Generated on: 2022-08-22T16:25:59.093205
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
 * Register:    Sampling Event Filter Register (pmsevfr_el1)
 * Group:       Statistical Profiling Extension registers
 * Type:        64-bit Register
 * Description: Controls sample filtering by events. The overall filter is the logical AND of these filters. For example, if E[3] and E[5] are both set to 1, only samples that have both event 3 (Level 1 unified or data cache refill) and event 5 set (TLB walk) are recorded.
 * File:        AArch64-pmsevfr_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Sampling Event Filter Register value in memory
pub struct PmsevfrEl1(u64);

/// struct implementation for accessing the fields of register pmsevfr_el1
impl PmsevfrEl1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> PmsevfrEl1 {
        Self::default()
    }

    /// collects the modifications of PmsevfrEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> PmsevfrEl1 {
        PmsevfrEl1(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  PmsevfrEl1 {
        let curval = Self::reg_rawrd() & 0xffff0000ff06f8ea;
        PmsevfrEl1(curval)
    }


    
    /// reading the Sampling Event Filter Register (pmsevfr_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, PMSEVFR_EL1
            asm!("mrs {}, S3_0_C9_C9_5", out(reg) regval);
        }
        return regval;
    }


    /// writing the Sampling Event Filter Register (pmsevfr_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR PMSEVFR_EL1, <Xt>
            asm!("msr S3_0_C9_C9_5, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0xffff0000ff06f8ea;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 18446462603011487978;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: ex
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ex_extract(&self) -> u64 {
        // bits 48..63
        self.0.get_bits(48..=63)
    }

    /// reads the current register value and extract field `ex` from it
    pub fn ex_read(&mut self) -> u64 {
        Self::with_reg_val().ex_extract()
    }

    /// inserts the given value `val` into the field `ex`
    pub fn ex_insert(&mut self, val: u64) -> &mut Self {
        // bits 48..63
        self.0.set_bits(48..=63, val);
        self
    }

    /// reads the register, updates the `ex` field, and writes the updated value
    pub fn ex_write(&mut self, val: u64) {
        Self::with_reg_val().ex_insert(val).write();
    }

    /*
     * Field: e63_63_63
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e63_63_63_extract(&self) -> u64 {
        // bits 63..63
        self.0.get_bits(63..=63)
    }

    /// reads the current register value and extract field `e63_63_63` from it
    pub fn e63_63_63_read(&mut self) -> u64 {
        Self::with_reg_val().e63_63_63_extract()
    }

    /// inserts the given value `val` into the field `e63_63_63`
    pub fn e63_63_63_insert(&mut self, val: u64) -> &mut Self {
        // bits 63..63
        self.0.set_bits(63..=63, val);
        self
    }

    /// reads the register, updates the `e63_63_63` field, and writes the updated value
    pub fn e63_63_63_write(&mut self, val: u64) {
        Self::with_reg_val().e63_63_63_insert(val).write();
    }

    /*
     * Field: e62_62_62
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e62_62_62_extract(&self) -> u64 {
        // bits 62..62
        self.0.get_bits(62..=62)
    }

    /// reads the current register value and extract field `e62_62_62` from it
    pub fn e62_62_62_read(&mut self) -> u64 {
        Self::with_reg_val().e62_62_62_extract()
    }

    /// inserts the given value `val` into the field `e62_62_62`
    pub fn e62_62_62_insert(&mut self, val: u64) -> &mut Self {
        // bits 62..62
        self.0.set_bits(62..=62, val);
        self
    }

    /// reads the register, updates the `e62_62_62` field, and writes the updated value
    pub fn e62_62_62_write(&mut self, val: u64) {
        Self::with_reg_val().e62_62_62_insert(val).write();
    }

    /*
     * Field: e61_61_61
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e61_61_61_extract(&self) -> u64 {
        // bits 61..61
        self.0.get_bits(61..=61)
    }

    /// reads the current register value and extract field `e61_61_61` from it
    pub fn e61_61_61_read(&mut self) -> u64 {
        Self::with_reg_val().e61_61_61_extract()
    }

    /// inserts the given value `val` into the field `e61_61_61`
    pub fn e61_61_61_insert(&mut self, val: u64) -> &mut Self {
        // bits 61..61
        self.0.set_bits(61..=61, val);
        self
    }

    /// reads the register, updates the `e61_61_61` field, and writes the updated value
    pub fn e61_61_61_write(&mut self, val: u64) {
        Self::with_reg_val().e61_61_61_insert(val).write();
    }

    /*
     * Field: e60_60_60
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e60_60_60_extract(&self) -> u64 {
        // bits 60..60
        self.0.get_bits(60..=60)
    }

    /// reads the current register value and extract field `e60_60_60` from it
    pub fn e60_60_60_read(&mut self) -> u64 {
        Self::with_reg_val().e60_60_60_extract()
    }

    /// inserts the given value `val` into the field `e60_60_60`
    pub fn e60_60_60_insert(&mut self, val: u64) -> &mut Self {
        // bits 60..60
        self.0.set_bits(60..=60, val);
        self
    }

    /// reads the register, updates the `e60_60_60` field, and writes the updated value
    pub fn e60_60_60_write(&mut self, val: u64) {
        Self::with_reg_val().e60_60_60_insert(val).write();
    }

    /*
     * Field: e59_59_59
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e59_59_59_extract(&self) -> u64 {
        // bits 59..59
        self.0.get_bits(59..=59)
    }

    /// reads the current register value and extract field `e59_59_59` from it
    pub fn e59_59_59_read(&mut self) -> u64 {
        Self::with_reg_val().e59_59_59_extract()
    }

    /// inserts the given value `val` into the field `e59_59_59`
    pub fn e59_59_59_insert(&mut self, val: u64) -> &mut Self {
        // bits 59..59
        self.0.set_bits(59..=59, val);
        self
    }

    /// reads the register, updates the `e59_59_59` field, and writes the updated value
    pub fn e59_59_59_write(&mut self, val: u64) {
        Self::with_reg_val().e59_59_59_insert(val).write();
    }

    /*
     * Field: e58_58_58
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e58_58_58_extract(&self) -> u64 {
        // bits 58..58
        self.0.get_bits(58..=58)
    }

    /// reads the current register value and extract field `e58_58_58` from it
    pub fn e58_58_58_read(&mut self) -> u64 {
        Self::with_reg_val().e58_58_58_extract()
    }

    /// inserts the given value `val` into the field `e58_58_58`
    pub fn e58_58_58_insert(&mut self, val: u64) -> &mut Self {
        // bits 58..58
        self.0.set_bits(58..=58, val);
        self
    }

    /// reads the register, updates the `e58_58_58` field, and writes the updated value
    pub fn e58_58_58_write(&mut self, val: u64) {
        Self::with_reg_val().e58_58_58_insert(val).write();
    }

    /*
     * Field: e57_57_57
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e57_57_57_extract(&self) -> u64 {
        // bits 57..57
        self.0.get_bits(57..=57)
    }

    /// reads the current register value and extract field `e57_57_57` from it
    pub fn e57_57_57_read(&mut self) -> u64 {
        Self::with_reg_val().e57_57_57_extract()
    }

    /// inserts the given value `val` into the field `e57_57_57`
    pub fn e57_57_57_insert(&mut self, val: u64) -> &mut Self {
        // bits 57..57
        self.0.set_bits(57..=57, val);
        self
    }

    /// reads the register, updates the `e57_57_57` field, and writes the updated value
    pub fn e57_57_57_write(&mut self, val: u64) {
        Self::with_reg_val().e57_57_57_insert(val).write();
    }

    /*
     * Field: e56_56_56
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e56_56_56_extract(&self) -> u64 {
        // bits 56..56
        self.0.get_bits(56..=56)
    }

    /// reads the current register value and extract field `e56_56_56` from it
    pub fn e56_56_56_read(&mut self) -> u64 {
        Self::with_reg_val().e56_56_56_extract()
    }

    /// inserts the given value `val` into the field `e56_56_56`
    pub fn e56_56_56_insert(&mut self, val: u64) -> &mut Self {
        // bits 56..56
        self.0.set_bits(56..=56, val);
        self
    }

    /// reads the register, updates the `e56_56_56` field, and writes the updated value
    pub fn e56_56_56_write(&mut self, val: u64) {
        Self::with_reg_val().e56_56_56_insert(val).write();
    }

    /*
     * Field: e55_55_55
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e55_55_55_extract(&self) -> u64 {
        // bits 55..55
        self.0.get_bits(55..=55)
    }

    /// reads the current register value and extract field `e55_55_55` from it
    pub fn e55_55_55_read(&mut self) -> u64 {
        Self::with_reg_val().e55_55_55_extract()
    }

    /// inserts the given value `val` into the field `e55_55_55`
    pub fn e55_55_55_insert(&mut self, val: u64) -> &mut Self {
        // bits 55..55
        self.0.set_bits(55..=55, val);
        self
    }

    /// reads the register, updates the `e55_55_55` field, and writes the updated value
    pub fn e55_55_55_write(&mut self, val: u64) {
        Self::with_reg_val().e55_55_55_insert(val).write();
    }

    /*
     * Field: e54_54_54
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e54_54_54_extract(&self) -> u64 {
        // bits 54..54
        self.0.get_bits(54..=54)
    }

    /// reads the current register value and extract field `e54_54_54` from it
    pub fn e54_54_54_read(&mut self) -> u64 {
        Self::with_reg_val().e54_54_54_extract()
    }

    /// inserts the given value `val` into the field `e54_54_54`
    pub fn e54_54_54_insert(&mut self, val: u64) -> &mut Self {
        // bits 54..54
        self.0.set_bits(54..=54, val);
        self
    }

    /// reads the register, updates the `e54_54_54` field, and writes the updated value
    pub fn e54_54_54_write(&mut self, val: u64) {
        Self::with_reg_val().e54_54_54_insert(val).write();
    }

    /*
     * Field: e53_53_53
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e53_53_53_extract(&self) -> u64 {
        // bits 53..53
        self.0.get_bits(53..=53)
    }

    /// reads the current register value and extract field `e53_53_53` from it
    pub fn e53_53_53_read(&mut self) -> u64 {
        Self::with_reg_val().e53_53_53_extract()
    }

    /// inserts the given value `val` into the field `e53_53_53`
    pub fn e53_53_53_insert(&mut self, val: u64) -> &mut Self {
        // bits 53..53
        self.0.set_bits(53..=53, val);
        self
    }

    /// reads the register, updates the `e53_53_53` field, and writes the updated value
    pub fn e53_53_53_write(&mut self, val: u64) {
        Self::with_reg_val().e53_53_53_insert(val).write();
    }

    /*
     * Field: e52_52_52
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e52_52_52_extract(&self) -> u64 {
        // bits 52..52
        self.0.get_bits(52..=52)
    }

    /// reads the current register value and extract field `e52_52_52` from it
    pub fn e52_52_52_read(&mut self) -> u64 {
        Self::with_reg_val().e52_52_52_extract()
    }

    /// inserts the given value `val` into the field `e52_52_52`
    pub fn e52_52_52_insert(&mut self, val: u64) -> &mut Self {
        // bits 52..52
        self.0.set_bits(52..=52, val);
        self
    }

    /// reads the register, updates the `e52_52_52` field, and writes the updated value
    pub fn e52_52_52_write(&mut self, val: u64) {
        Self::with_reg_val().e52_52_52_insert(val).write();
    }

    /*
     * Field: e51_51_51
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e51_51_51_extract(&self) -> u64 {
        // bits 51..51
        self.0.get_bits(51..=51)
    }

    /// reads the current register value and extract field `e51_51_51` from it
    pub fn e51_51_51_read(&mut self) -> u64 {
        Self::with_reg_val().e51_51_51_extract()
    }

    /// inserts the given value `val` into the field `e51_51_51`
    pub fn e51_51_51_insert(&mut self, val: u64) -> &mut Self {
        // bits 51..51
        self.0.set_bits(51..=51, val);
        self
    }

    /// reads the register, updates the `e51_51_51` field, and writes the updated value
    pub fn e51_51_51_write(&mut self, val: u64) {
        Self::with_reg_val().e51_51_51_insert(val).write();
    }

    /*
     * Field: e50_50_50
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e50_50_50_extract(&self) -> u64 {
        // bits 50..50
        self.0.get_bits(50..=50)
    }

    /// reads the current register value and extract field `e50_50_50` from it
    pub fn e50_50_50_read(&mut self) -> u64 {
        Self::with_reg_val().e50_50_50_extract()
    }

    /// inserts the given value `val` into the field `e50_50_50`
    pub fn e50_50_50_insert(&mut self, val: u64) -> &mut Self {
        // bits 50..50
        self.0.set_bits(50..=50, val);
        self
    }

    /// reads the register, updates the `e50_50_50` field, and writes the updated value
    pub fn e50_50_50_write(&mut self, val: u64) {
        Self::with_reg_val().e50_50_50_insert(val).write();
    }

    /*
     * Field: e49_49_49
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e49_49_49_extract(&self) -> u64 {
        // bits 49..49
        self.0.get_bits(49..=49)
    }

    /// reads the current register value and extract field `e49_49_49` from it
    pub fn e49_49_49_read(&mut self) -> u64 {
        Self::with_reg_val().e49_49_49_extract()
    }

    /// inserts the given value `val` into the field `e49_49_49`
    pub fn e49_49_49_insert(&mut self, val: u64) -> &mut Self {
        // bits 49..49
        self.0.set_bits(49..=49, val);
        self
    }

    /// reads the register, updates the `e49_49_49` field, and writes the updated value
    pub fn e49_49_49_write(&mut self, val: u64) {
        Self::with_reg_val().e49_49_49_insert(val).write();
    }

    /*
     * Field: e48_48_48
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e48_48_48_extract(&self) -> u64 {
        // bits 48..48
        self.0.get_bits(48..=48)
    }

    /// reads the current register value and extract field `e48_48_48` from it
    pub fn e48_48_48_read(&mut self) -> u64 {
        Self::with_reg_val().e48_48_48_extract()
    }

    /// inserts the given value `val` into the field `e48_48_48`
    pub fn e48_48_48_insert(&mut self, val: u64) -> &mut Self {
        // bits 48..48
        self.0.set_bits(48..=48, val);
        self
    }

    /// reads the register, updates the `e48_48_48` field, and writes the updated value
    pub fn e48_48_48_write(&mut self, val: u64) {
        Self::with_reg_val().e48_48_48_insert(val).write();
    }

    /*
     * Field: e31_31_31
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e31_31_31_extract(&self) -> u64 {
        // bits 31..31
        self.0.get_bits(31..=31)
    }

    /// reads the current register value and extract field `e31_31_31` from it
    pub fn e31_31_31_read(&mut self) -> u64 {
        Self::with_reg_val().e31_31_31_extract()
    }

    /// inserts the given value `val` into the field `e31_31_31`
    pub fn e31_31_31_insert(&mut self, val: u64) -> &mut Self {
        // bits 31..31
        self.0.set_bits(31..=31, val);
        self
    }

    /// reads the register, updates the `e31_31_31` field, and writes the updated value
    pub fn e31_31_31_write(&mut self, val: u64) {
        Self::with_reg_val().e31_31_31_insert(val).write();
    }

    /*
     * Field: e30_30_30
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e30_30_30_extract(&self) -> u64 {
        // bits 30..30
        self.0.get_bits(30..=30)
    }

    /// reads the current register value and extract field `e30_30_30` from it
    pub fn e30_30_30_read(&mut self) -> u64 {
        Self::with_reg_val().e30_30_30_extract()
    }

    /// inserts the given value `val` into the field `e30_30_30`
    pub fn e30_30_30_insert(&mut self, val: u64) -> &mut Self {
        // bits 30..30
        self.0.set_bits(30..=30, val);
        self
    }

    /// reads the register, updates the `e30_30_30` field, and writes the updated value
    pub fn e30_30_30_write(&mut self, val: u64) {
        Self::with_reg_val().e30_30_30_insert(val).write();
    }

    /*
     * Field: e29_29_29
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e29_29_29_extract(&self) -> u64 {
        // bits 29..29
        self.0.get_bits(29..=29)
    }

    /// reads the current register value and extract field `e29_29_29` from it
    pub fn e29_29_29_read(&mut self) -> u64 {
        Self::with_reg_val().e29_29_29_extract()
    }

    /// inserts the given value `val` into the field `e29_29_29`
    pub fn e29_29_29_insert(&mut self, val: u64) -> &mut Self {
        // bits 29..29
        self.0.set_bits(29..=29, val);
        self
    }

    /// reads the register, updates the `e29_29_29` field, and writes the updated value
    pub fn e29_29_29_write(&mut self, val: u64) {
        Self::with_reg_val().e29_29_29_insert(val).write();
    }

    /*
     * Field: e28_28_28
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e28_28_28_extract(&self) -> u64 {
        // bits 28..28
        self.0.get_bits(28..=28)
    }

    /// reads the current register value and extract field `e28_28_28` from it
    pub fn e28_28_28_read(&mut self) -> u64 {
        Self::with_reg_val().e28_28_28_extract()
    }

    /// inserts the given value `val` into the field `e28_28_28`
    pub fn e28_28_28_insert(&mut self, val: u64) -> &mut Self {
        // bits 28..28
        self.0.set_bits(28..=28, val);
        self
    }

    /// reads the register, updates the `e28_28_28` field, and writes the updated value
    pub fn e28_28_28_write(&mut self, val: u64) {
        Self::with_reg_val().e28_28_28_insert(val).write();
    }

    /*
     * Field: e27_27_27
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e27_27_27_extract(&self) -> u64 {
        // bits 27..27
        self.0.get_bits(27..=27)
    }

    /// reads the current register value and extract field `e27_27_27` from it
    pub fn e27_27_27_read(&mut self) -> u64 {
        Self::with_reg_val().e27_27_27_extract()
    }

    /// inserts the given value `val` into the field `e27_27_27`
    pub fn e27_27_27_insert(&mut self, val: u64) -> &mut Self {
        // bits 27..27
        self.0.set_bits(27..=27, val);
        self
    }

    /// reads the register, updates the `e27_27_27` field, and writes the updated value
    pub fn e27_27_27_write(&mut self, val: u64) {
        Self::with_reg_val().e27_27_27_insert(val).write();
    }

    /*
     * Field: e26_26_26
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e26_26_26_extract(&self) -> u64 {
        // bits 26..26
        self.0.get_bits(26..=26)
    }

    /// reads the current register value and extract field `e26_26_26` from it
    pub fn e26_26_26_read(&mut self) -> u64 {
        Self::with_reg_val().e26_26_26_extract()
    }

    /// inserts the given value `val` into the field `e26_26_26`
    pub fn e26_26_26_insert(&mut self, val: u64) -> &mut Self {
        // bits 26..26
        self.0.set_bits(26..=26, val);
        self
    }

    /// reads the register, updates the `e26_26_26` field, and writes the updated value
    pub fn e26_26_26_write(&mut self, val: u64) {
        Self::with_reg_val().e26_26_26_insert(val).write();
    }

    /*
     * Field: e25_25_25
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e25_25_25_extract(&self) -> u64 {
        // bits 25..25
        self.0.get_bits(25..=25)
    }

    /// reads the current register value and extract field `e25_25_25` from it
    pub fn e25_25_25_read(&mut self) -> u64 {
        Self::with_reg_val().e25_25_25_extract()
    }

    /// inserts the given value `val` into the field `e25_25_25`
    pub fn e25_25_25_insert(&mut self, val: u64) -> &mut Self {
        // bits 25..25
        self.0.set_bits(25..=25, val);
        self
    }

    /// reads the register, updates the `e25_25_25` field, and writes the updated value
    pub fn e25_25_25_write(&mut self, val: u64) {
        Self::with_reg_val().e25_25_25_insert(val).write();
    }

    /*
     * Field: e24_24_24
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e24_24_24_extract(&self) -> u64 {
        // bits 24..24
        self.0.get_bits(24..=24)
    }

    /// reads the current register value and extract field `e24_24_24` from it
    pub fn e24_24_24_read(&mut self) -> u64 {
        Self::with_reg_val().e24_24_24_extract()
    }

    /// inserts the given value `val` into the field `e24_24_24`
    pub fn e24_24_24_insert(&mut self, val: u64) -> &mut Self {
        // bits 24..24
        self.0.set_bits(24..=24, val);
        self
    }

    /// reads the register, updates the `e24_24_24` field, and writes the updated value
    pub fn e24_24_24_write(&mut self, val: u64) {
        Self::with_reg_val().e24_24_24_insert(val).write();
    }

    /*
     * Field: e18_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e18_1_extract(&self) -> u64 {
        // bits 18..18
        self.0.get_bits(18..=18)
    }

    /// reads the current register value and extract field `e18_1` from it
    pub fn e18_1_read(&mut self) -> u64 {
        Self::with_reg_val().e18_1_extract()
    }

    /// inserts the given value `val` into the field `e18_1`
    pub fn e18_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 18..18
        self.0.set_bits(18..=18, val);
        self
    }

    /// reads the register, updates the `e18_1` field, and writes the updated value
    pub fn e18_1_write(&mut self, val: u64) {
        Self::with_reg_val().e18_1_insert(val).write();
    }

    /*
     * Field: e17_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e17_1_extract(&self) -> u64 {
        // bits 17..17
        self.0.get_bits(17..=17)
    }

    /// reads the current register value and extract field `e17_1` from it
    pub fn e17_1_read(&mut self) -> u64 {
        Self::with_reg_val().e17_1_extract()
    }

    /// inserts the given value `val` into the field `e17_1`
    pub fn e17_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 17..17
        self.0.set_bits(17..=17, val);
        self
    }

    /// reads the register, updates the `e17_1` field, and writes the updated value
    pub fn e17_1_write(&mut self, val: u64) {
        Self::with_reg_val().e17_1_insert(val).write();
    }

    /*
     * Field: e15_15_15
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e15_15_15_extract(&self) -> u64 {
        // bits 15..15
        self.0.get_bits(15..=15)
    }

    /// reads the current register value and extract field `e15_15_15` from it
    pub fn e15_15_15_read(&mut self) -> u64 {
        Self::with_reg_val().e15_15_15_extract()
    }

    /// inserts the given value `val` into the field `e15_15_15`
    pub fn e15_15_15_insert(&mut self, val: u64) -> &mut Self {
        // bits 15..15
        self.0.set_bits(15..=15, val);
        self
    }

    /// reads the register, updates the `e15_15_15` field, and writes the updated value
    pub fn e15_15_15_write(&mut self, val: u64) {
        Self::with_reg_val().e15_15_15_insert(val).write();
    }

    /*
     * Field: e14_14_14
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e14_14_14_extract(&self) -> u64 {
        // bits 14..14
        self.0.get_bits(14..=14)
    }

    /// reads the current register value and extract field `e14_14_14` from it
    pub fn e14_14_14_read(&mut self) -> u64 {
        Self::with_reg_val().e14_14_14_extract()
    }

    /// inserts the given value `val` into the field `e14_14_14`
    pub fn e14_14_14_insert(&mut self, val: u64) -> &mut Self {
        // bits 14..14
        self.0.set_bits(14..=14, val);
        self
    }

    /// reads the register, updates the `e14_14_14` field, and writes the updated value
    pub fn e14_14_14_write(&mut self, val: u64) {
        Self::with_reg_val().e14_14_14_insert(val).write();
    }

    /*
     * Field: e13_13_13
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e13_13_13_extract(&self) -> u64 {
        // bits 13..13
        self.0.get_bits(13..=13)
    }

    /// reads the current register value and extract field `e13_13_13` from it
    pub fn e13_13_13_read(&mut self) -> u64 {
        Self::with_reg_val().e13_13_13_extract()
    }

    /// inserts the given value `val` into the field `e13_13_13`
    pub fn e13_13_13_insert(&mut self, val: u64) -> &mut Self {
        // bits 13..13
        self.0.set_bits(13..=13, val);
        self
    }

    /// reads the register, updates the `e13_13_13` field, and writes the updated value
    pub fn e13_13_13_write(&mut self, val: u64) {
        Self::with_reg_val().e13_13_13_insert(val).write();
    }

    /*
     * Field: e12_12_12
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e12_12_12_extract(&self) -> u64 {
        // bits 12..12
        self.0.get_bits(12..=12)
    }

    /// reads the current register value and extract field `e12_12_12` from it
    pub fn e12_12_12_read(&mut self) -> u64 {
        Self::with_reg_val().e12_12_12_extract()
    }

    /// inserts the given value `val` into the field `e12_12_12`
    pub fn e12_12_12_insert(&mut self, val: u64) -> &mut Self {
        // bits 12..12
        self.0.set_bits(12..=12, val);
        self
    }

    /// reads the register, updates the `e12_12_12` field, and writes the updated value
    pub fn e12_12_12_write(&mut self, val: u64) {
        Self::with_reg_val().e12_12_12_insert(val).write();
    }

    /*
     * Field: e11_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e11_1_extract(&self) -> u64 {
        // bits 11..11
        self.0.get_bits(11..=11)
    }

    /// reads the current register value and extract field `e11_1` from it
    pub fn e11_1_read(&mut self) -> u64 {
        Self::with_reg_val().e11_1_extract()
    }

    /// inserts the given value `val` into the field `e11_1`
    pub fn e11_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 11..11
        self.0.set_bits(11..=11, val);
        self
    }

    /// reads the register, updates the `e11_1` field, and writes the updated value
    pub fn e11_1_write(&mut self, val: u64) {
        Self::with_reg_val().e11_1_insert(val).write();
    }

    /*
     * Field: e7
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e7_extract(&self) -> u64 {
        // bits 7..7
        self.0.get_bits(7..=7)
    }

    /// reads the current register value and extract field `e7` from it
    pub fn e7_read(&mut self) -> u64 {
        Self::with_reg_val().e7_extract()
    }

    /// inserts the given value `val` into the field `e7`
    pub fn e7_insert(&mut self, val: u64) -> &mut Self {
        // bits 7..7
        self.0.set_bits(7..=7, val);
        self
    }

    /// reads the register, updates the `e7` field, and writes the updated value
    pub fn e7_write(&mut self, val: u64) {
        Self::with_reg_val().e7_insert(val).write();
    }

    /*
     * Field: e6_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e6_1_extract(&self) -> u64 {
        // bits 6..6
        self.0.get_bits(6..=6)
    }

    /// reads the current register value and extract field `e6_1` from it
    pub fn e6_1_read(&mut self) -> u64 {
        Self::with_reg_val().e6_1_extract()
    }

    /// inserts the given value `val` into the field `e6_1`
    pub fn e6_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 6..6
        self.0.set_bits(6..=6, val);
        self
    }

    /// reads the register, updates the `e6_1` field, and writes the updated value
    pub fn e6_1_write(&mut self, val: u64) {
        Self::with_reg_val().e6_1_insert(val).write();
    }

    /*
     * Field: e5
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e5_extract(&self) -> u64 {
        // bits 5..5
        self.0.get_bits(5..=5)
    }

    /// reads the current register value and extract field `e5` from it
    pub fn e5_read(&mut self) -> u64 {
        Self::with_reg_val().e5_extract()
    }

    /// inserts the given value `val` into the field `e5`
    pub fn e5_insert(&mut self, val: u64) -> &mut Self {
        // bits 5..5
        self.0.set_bits(5..=5, val);
        self
    }

    /// reads the register, updates the `e5` field, and writes the updated value
    pub fn e5_write(&mut self, val: u64) {
        Self::with_reg_val().e5_insert(val).write();
    }

    /*
     * Field: e3
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e3_extract(&self) -> u64 {
        // bits 3..3
        self.0.get_bits(3..=3)
    }

    /// reads the current register value and extract field `e3` from it
    pub fn e3_read(&mut self) -> u64 {
        Self::with_reg_val().e3_extract()
    }

    /// inserts the given value `val` into the field `e3`
    pub fn e3_insert(&mut self, val: u64) -> &mut Self {
        // bits 3..3
        self.0.set_bits(3..=3, val);
        self
    }

    /// reads the register, updates the `e3` field, and writes the updated value
    pub fn e3_write(&mut self, val: u64) {
        Self::with_reg_val().e3_insert(val).write();
    }

    /*
     * Field: e1_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e1_1_extract(&self) -> u64 {
        // bits 1..1
        self.0.get_bits(1..=1)
    }

    /// reads the current register value and extract field `e1_1` from it
    pub fn e1_1_read(&mut self) -> u64 {
        Self::with_reg_val().e1_1_extract()
    }

    /// inserts the given value `val` into the field `e1_1`
    pub fn e1_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 1..1
        self.0.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `e1_1` field, and writes the updated value
    pub fn e1_1_write(&mut self, val: u64) {
        Self::with_reg_val().e1_1_insert(val).write();
    }

}

impl Default for PmsevfrEl1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> PmsevfrEl1 {
        PmsevfrEl1(0)
    }
}
