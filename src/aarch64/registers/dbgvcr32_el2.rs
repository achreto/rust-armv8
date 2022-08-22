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
 * Generated on: 2022-08-22T15:51:28.511710
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
 * Register:    Debug Vector Catch Register (dbgvcr32_el2)
 * Group:       Debug registers
 * Type:        64-bit Register
 * Description: Allows access to the AArch32 register
 * File:        AArch64-dbgvcr32_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Debug Vector Catch Register value in memory
pub struct Dbgvcr32El2(u64);

/// struct implementation for accessing the fields of register dbgvcr32_el2
impl Dbgvcr32El2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> Dbgvcr32El2 {
        Self::default()
    }

    /// collects the modifications of Dbgvcr32El2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> Dbgvcr32El2 {
        Dbgvcr32El2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> Dbgvcr32El2 {
        let curval = Self::reg_rawrd() & 0xde0000de;
        Dbgvcr32El2(curval)
    }

    /// reading the Debug Vector Catch Register (dbgvcr32_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, DBGVCR32_EL2
            llvm_asm!("mrs $0, dbgvcr32_el2" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Debug Vector Catch Register (dbgvcr32_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR DBGVCR32_EL2, <Xt>
            llvm_asm!("msr dbgvcr32_el2, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xde0000de;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 3724542174;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: nsf
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn nsf_extract(&self) -> u64 {
        // bits 31..31
        self.val.get_bits(31..=31)
    }

    /// reads the current register value and extract field `nsf` from it
    pub fn nsf_read(&mut self) -> u64 {
        Self::with_reg_val().nsf_extract()
    }

    /// inserts the given value `val` into the field `nsf`
    pub fn nsf_insert(&mut self, val: u64) -> &mut self {
        // bits 31..31
        self.val.set_bits(31..=31, val);
        self
    }

    /// reads the register, updates the `nsf` field, and writes the updated value
    pub fn nsf_write(&mut self, val: u64) {
        Self::with_reg_val().nsf_insert(val).write();
    }

    /*
     * Field: nsi
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn nsi_extract(&self) -> u64 {
        // bits 30..30
        self.val.get_bits(30..=30)
    }

    /// reads the current register value and extract field `nsi` from it
    pub fn nsi_read(&mut self) -> u64 {
        Self::with_reg_val().nsi_extract()
    }

    /// inserts the given value `val` into the field `nsi`
    pub fn nsi_insert(&mut self, val: u64) -> &mut self {
        // bits 30..30
        self.val.set_bits(30..=30, val);
        self
    }

    /// reads the register, updates the `nsi` field, and writes the updated value
    pub fn nsi_write(&mut self, val: u64) {
        Self::with_reg_val().nsi_insert(val).write();
    }

    /*
     * Field: nsd
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn nsd_extract(&self) -> u64 {
        // bits 28..28
        self.val.get_bits(28..=28)
    }

    /// reads the current register value and extract field `nsd` from it
    pub fn nsd_read(&mut self) -> u64 {
        Self::with_reg_val().nsd_extract()
    }

    /// inserts the given value `val` into the field `nsd`
    pub fn nsd_insert(&mut self, val: u64) -> &mut self {
        // bits 28..28
        self.val.set_bits(28..=28, val);
        self
    }

    /// reads the register, updates the `nsd` field, and writes the updated value
    pub fn nsd_write(&mut self, val: u64) {
        Self::with_reg_val().nsd_insert(val).write();
    }

    /*
     * Field: nsp
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn nsp_extract(&self) -> u64 {
        // bits 27..27
        self.val.get_bits(27..=27)
    }

    /// reads the current register value and extract field `nsp` from it
    pub fn nsp_read(&mut self) -> u64 {
        Self::with_reg_val().nsp_extract()
    }

    /// inserts the given value `val` into the field `nsp`
    pub fn nsp_insert(&mut self, val: u64) -> &mut self {
        // bits 27..27
        self.val.set_bits(27..=27, val);
        self
    }

    /// reads the register, updates the `nsp` field, and writes the updated value
    pub fn nsp_write(&mut self, val: u64) {
        Self::with_reg_val().nsp_insert(val).write();
    }

    /*
     * Field: nss
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn nss_extract(&self) -> u64 {
        // bits 26..26
        self.val.get_bits(26..=26)
    }

    /// reads the current register value and extract field `nss` from it
    pub fn nss_read(&mut self) -> u64 {
        Self::with_reg_val().nss_extract()
    }

    /// inserts the given value `val` into the field `nss`
    pub fn nss_insert(&mut self, val: u64) -> &mut self {
        // bits 26..26
        self.val.set_bits(26..=26, val);
        self
    }

    /// reads the register, updates the `nss` field, and writes the updated value
    pub fn nss_write(&mut self, val: u64) {
        Self::with_reg_val().nss_insert(val).write();
    }

    /*
     * Field: nsu
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn nsu_extract(&self) -> u64 {
        // bits 25..25
        self.val.get_bits(25..=25)
    }

    /// reads the current register value and extract field `nsu` from it
    pub fn nsu_read(&mut self) -> u64 {
        Self::with_reg_val().nsu_extract()
    }

    /// inserts the given value `val` into the field `nsu`
    pub fn nsu_insert(&mut self, val: u64) -> &mut self {
        // bits 25..25
        self.val.set_bits(25..=25, val);
        self
    }

    /// reads the register, updates the `nsu` field, and writes the updated value
    pub fn nsu_write(&mut self, val: u64) {
        Self::with_reg_val().nsu_insert(val).write();
    }

    /*
     * Field: sf
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn sf_extract(&self) -> u64 {
        // bits 7..7
        self.val.get_bits(7..=7)
    }

    /// reads the current register value and extract field `sf` from it
    pub fn sf_read(&mut self) -> u64 {
        Self::with_reg_val().sf_extract()
    }

    /// inserts the given value `val` into the field `sf`
    pub fn sf_insert(&mut self, val: u64) -> &mut self {
        // bits 7..7
        self.val.set_bits(7..=7, val);
        self
    }

    /// reads the register, updates the `sf` field, and writes the updated value
    pub fn sf_write(&mut self, val: u64) {
        Self::with_reg_val().sf_insert(val).write();
    }

    /*
     * Field: si
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn si_extract(&self) -> u64 {
        // bits 6..6
        self.val.get_bits(6..=6)
    }

    /// reads the current register value and extract field `si` from it
    pub fn si_read(&mut self) -> u64 {
        Self::with_reg_val().si_extract()
    }

    /// inserts the given value `val` into the field `si`
    pub fn si_insert(&mut self, val: u64) -> &mut self {
        // bits 6..6
        self.val.set_bits(6..=6, val);
        self
    }

    /// reads the register, updates the `si` field, and writes the updated value
    pub fn si_write(&mut self, val: u64) {
        Self::with_reg_val().si_insert(val).write();
    }

    /*
     * Field: sd
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn sd_extract(&self) -> u64 {
        // bits 4..4
        self.val.get_bits(4..=4)
    }

    /// reads the current register value and extract field `sd` from it
    pub fn sd_read(&mut self) -> u64 {
        Self::with_reg_val().sd_extract()
    }

    /// inserts the given value `val` into the field `sd`
    pub fn sd_insert(&mut self, val: u64) -> &mut self {
        // bits 4..4
        self.val.set_bits(4..=4, val);
        self
    }

    /// reads the register, updates the `sd` field, and writes the updated value
    pub fn sd_write(&mut self, val: u64) {
        Self::with_reg_val().sd_insert(val).write();
    }

    /*
     * Field: sp
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn sp_extract(&self) -> u64 {
        // bits 3..3
        self.val.get_bits(3..=3)
    }

    /// reads the current register value and extract field `sp` from it
    pub fn sp_read(&mut self) -> u64 {
        Self::with_reg_val().sp_extract()
    }

    /// inserts the given value `val` into the field `sp`
    pub fn sp_insert(&mut self, val: u64) -> &mut self {
        // bits 3..3
        self.val.set_bits(3..=3, val);
        self
    }

    /// reads the register, updates the `sp` field, and writes the updated value
    pub fn sp_write(&mut self, val: u64) {
        Self::with_reg_val().sp_insert(val).write();
    }

    /*
     * Field: ss
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ss_extract(&self) -> u64 {
        // bits 2..2
        self.val.get_bits(2..=2)
    }

    /// reads the current register value and extract field `ss` from it
    pub fn ss_read(&mut self) -> u64 {
        Self::with_reg_val().ss_extract()
    }

    /// inserts the given value `val` into the field `ss`
    pub fn ss_insert(&mut self, val: u64) -> &mut self {
        // bits 2..2
        self.val.set_bits(2..=2, val);
        self
    }

    /// reads the register, updates the `ss` field, and writes the updated value
    pub fn ss_write(&mut self, val: u64) {
        Self::with_reg_val().ss_insert(val).write();
    }

    /*
     * Field: su
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn su_extract(&self) -> u64 {
        // bits 1..1
        self.val.get_bits(1..=1)
    }

    /// reads the current register value and extract field `su` from it
    pub fn su_read(&mut self) -> u64 {
        Self::with_reg_val().su_extract()
    }

    /// inserts the given value `val` into the field `su`
    pub fn su_insert(&mut self, val: u64) -> &mut self {
        // bits 1..1
        self.val.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `su` field, and writes the updated value
    pub fn su_write(&mut self, val: u64) {
        Self::with_reg_val().su_insert(val).write();
    }
}

impl Default for Dbgvcr32El2 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> Dbgvcr32El2 {
        Dbgvcr32El2(0)
    }
}
