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
 * Generated on: 2022-08-22T16:35:53.069055
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
 * Register:    Monitor Debug Configuration Register (EL2) (mdcr_el2)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Provides EL2 configuration options for self-hosted debug and the Performance Monitors Extension.
 * File:        AArch64-mdcr_el2.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Monitor Debug Configuration Register (EL2) value in memory
pub struct MdcrEl2(u64);

/// struct implementation for accessing the fields of register mdcr_el2
impl MdcrEl2 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> MdcrEl2 {
        Self::default()
    }

    /// collects the modifications of MdcrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> MdcrEl2 {
        MdcrEl2(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  MdcrEl2 {
        let curval = Self::reg_rawrd() & 0x103c8a7fff;
        MdcrEl2(curval)
    }


    
    /// reading the Monitor Debug Configuration Register (EL2) (mdcr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, MDCR_EL2
            asm!("mrs {}, mdcr_el2", out(reg) regval);
        }
        return regval;
    }


    /// writing the Monitor Debug Configuration Register (EL2) (mdcr_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR MDCR_EL2, <Xt>
            asm!("msr mdcr_el2, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x103c8a7fff;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 69735186431;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: hpmfzs_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn hpmfzs_1_extract(&self) -> u64 {
        // bits 36..36
        self.0.get_bits(36..=36)
    }

    /// reads the current register value and extract field `hpmfzs_1` from it
    pub fn hpmfzs_1_read() -> u64 {
        Self::with_reg_val().hpmfzs_1_extract()
    }

    /// inserts the given value `val` into the field `hpmfzs_1`
    pub fn hpmfzs_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 36..36
        self.0.set_bits(36..=36, val);
        self
    }

    /// reads the register, updates the `hpmfzs_1` field, and writes the updated value
    pub fn hpmfzs_1_write(val: u64) {
        Self::with_reg_val().hpmfzs_1_insert(val).write();
    }

    /*
     * Field: hpmfzo_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn hpmfzo_1_extract(&self) -> u64 {
        // bits 29..29
        self.0.get_bits(29..=29)
    }

    /// reads the current register value and extract field `hpmfzo_1` from it
    pub fn hpmfzo_1_read() -> u64 {
        Self::with_reg_val().hpmfzo_1_extract()
    }

    /// inserts the given value `val` into the field `hpmfzo_1`
    pub fn hpmfzo_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 29..29
        self.0.set_bits(29..=29, val);
        self
    }

    /// reads the register, updates the `hpmfzo_1` field, and writes the updated value
    pub fn hpmfzo_1_write(val: u64) {
        Self::with_reg_val().hpmfzo_1_insert(val).write();
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
     * Field: hlp_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn hlp_1_extract(&self) -> u64 {
        // bits 26..26
        self.0.get_bits(26..=26)
    }

    /// reads the current register value and extract field `hlp_1` from it
    pub fn hlp_1_read() -> u64 {
        Self::with_reg_val().hlp_1_extract()
    }

    /// inserts the given value `val` into the field `hlp_1`
    pub fn hlp_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 26..26
        self.0.set_bits(26..=26, val);
        self
    }

    /// reads the register, updates the `hlp_1` field, and writes the updated value
    pub fn hlp_1_write(val: u64) {
        Self::with_reg_val().hlp_1_insert(val).write();
    }

    /*
     * Field: hccd_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn hccd_1_extract(&self) -> u64 {
        // bits 23..23
        self.0.get_bits(23..=23)
    }

    /// reads the current register value and extract field `hccd_1` from it
    pub fn hccd_1_read() -> u64 {
        Self::with_reg_val().hccd_1_extract()
    }

    /// inserts the given value `val` into the field `hccd_1`
    pub fn hccd_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 23..23
        self.0.set_bits(23..=23, val);
        self
    }

    /// reads the register, updates the `hccd_1` field, and writes the updated value
    pub fn hccd_1_write(val: u64) {
        Self::with_reg_val().hccd_1_insert(val).write();
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
     * Field: hpmd_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn hpmd_1_extract(&self) -> u64 {
        // bits 17..17
        self.0.get_bits(17..=17)
    }

    /// reads the current register value and extract field `hpmd_1` from it
    pub fn hpmd_1_read() -> u64 {
        Self::with_reg_val().hpmd_1_extract()
    }

    /// inserts the given value `val` into the field `hpmd_1`
    pub fn hpmd_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 17..17
        self.0.set_bits(17..=17, val);
        self
    }

    /// reads the register, updates the `hpmd_1` field, and writes the updated value
    pub fn hpmd_1_write(val: u64) {
        Self::with_reg_val().hpmd_1_insert(val).write();
    }

    /*
     * Field: tpms_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tpms_1_extract(&self) -> u64 {
        // bits 14..14
        self.0.get_bits(14..=14)
    }

    /// reads the current register value and extract field `tpms_1` from it
    pub fn tpms_1_read() -> u64 {
        Self::with_reg_val().tpms_1_extract()
    }

    /// inserts the given value `val` into the field `tpms_1`
    pub fn tpms_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 14..14
        self.0.set_bits(14..=14, val);
        self
    }

    /// reads the register, updates the `tpms_1` field, and writes the updated value
    pub fn tpms_1_write(val: u64) {
        Self::with_reg_val().tpms_1_insert(val).write();
    }

    /*
     * Field: e2pb_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn e2pb_1_extract(&self) -> u64 {
        // bits 12..13
        self.0.get_bits(12..=13)
    }

    /// reads the current register value and extract field `e2pb_1` from it
    pub fn e2pb_1_read() -> u64 {
        Self::with_reg_val().e2pb_1_extract()
    }

    /// inserts the given value `val` into the field `e2pb_1`
    pub fn e2pb_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 12..13
        self.0.set_bits(12..=13, val);
        self
    }

    /// reads the register, updates the `e2pb_1` field, and writes the updated value
    pub fn e2pb_1_write(val: u64) {
        Self::with_reg_val().e2pb_1_insert(val).write();
    }

    /*
     * Field: tdra
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tdra_extract(&self) -> u64 {
        // bits 11..11
        self.0.get_bits(11..=11)
    }

    /// reads the current register value and extract field `tdra` from it
    pub fn tdra_read() -> u64 {
        Self::with_reg_val().tdra_extract()
    }

    /// inserts the given value `val` into the field `tdra`
    pub fn tdra_insert(&mut self, val: u64) -> &mut Self {
        // bits 11..11
        self.0.set_bits(11..=11, val);
        self
    }

    /// reads the register, updates the `tdra` field, and writes the updated value
    pub fn tdra_write(val: u64) {
        Self::with_reg_val().tdra_insert(val).write();
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
     * Field: tde
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tde_extract(&self) -> u64 {
        // bits 8..8
        self.0.get_bits(8..=8)
    }

    /// reads the current register value and extract field `tde` from it
    pub fn tde_read() -> u64 {
        Self::with_reg_val().tde_extract()
    }

    /// inserts the given value `val` into the field `tde`
    pub fn tde_insert(&mut self, val: u64) -> &mut Self {
        // bits 8..8
        self.0.set_bits(8..=8, val);
        self
    }

    /// reads the register, updates the `tde` field, and writes the updated value
    pub fn tde_write(val: u64) {
        Self::with_reg_val().tde_insert(val).write();
    }

    /*
     * Field: hpme_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn hpme_1_extract(&self) -> u64 {
        // bits 7..7
        self.0.get_bits(7..=7)
    }

    /// reads the current register value and extract field `hpme_1` from it
    pub fn hpme_1_read() -> u64 {
        Self::with_reg_val().hpme_1_extract()
    }

    /// inserts the given value `val` into the field `hpme_1`
    pub fn hpme_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 7..7
        self.0.set_bits(7..=7, val);
        self
    }

    /// reads the register, updates the `hpme_1` field, and writes the updated value
    pub fn hpme_1_write(val: u64) {
        Self::with_reg_val().hpme_1_insert(val).write();
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

    /*
     * Field: tpmcr_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn tpmcr_1_extract(&self) -> u64 {
        // bits 5..5
        self.0.get_bits(5..=5)
    }

    /// reads the current register value and extract field `tpmcr_1` from it
    pub fn tpmcr_1_read() -> u64 {
        Self::with_reg_val().tpmcr_1_extract()
    }

    /// inserts the given value `val` into the field `tpmcr_1`
    pub fn tpmcr_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 5..5
        self.0.set_bits(5..=5, val);
        self
    }

    /// reads the register, updates the `tpmcr_1` field, and writes the updated value
    pub fn tpmcr_1_write(val: u64) {
        Self::with_reg_val().tpmcr_1_insert(val).write();
    }

    /*
     * Field: hpmn_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn hpmn_1_extract(&self) -> u64 {
        // bits 0..4
        self.0.get_bits(0..=4)
    }

    /// reads the current register value and extract field `hpmn_1` from it
    pub fn hpmn_1_read() -> u64 {
        Self::with_reg_val().hpmn_1_extract()
    }

    /// inserts the given value `val` into the field `hpmn_1`
    pub fn hpmn_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..4
        self.0.set_bits(0..=4, val);
        self
    }

    /// reads the register, updates the `hpmn_1` field, and writes the updated value
    pub fn hpmn_1_write(val: u64) {
        Self::with_reg_val().hpmn_1_insert(val).write();
    }

}

impl Default for MdcrEl2 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> MdcrEl2 {
        MdcrEl2(0)
    }
}
