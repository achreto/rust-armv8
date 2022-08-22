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
 * Generated on: 2022-08-22T15:51:28.533922
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
 * Register:    Secure Configuration Register (scr_el3)
 * Group:       Security registers
 * Type:        64-bit Register
 * Description: Defines the configuration of the current Security state. It specifies:
 * File:        AArch64-scr_el3.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Secure Configuration Register value in memory
pub struct ScrEl3(u64);

/// struct implementation for accessing the fields of register scr_el3
impl ScrEl3 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> ScrEl3 {
        Self::default()
    }

    /// collects the modifications of ScrEl3 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> ScrEl3 {
        ScrEl3(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> ScrEl3 {
        let curval = Self::reg_rawrd() & 0x7bfe3fff8f;
        ScrEl3(curval)
    }

    /// reading the Secure Configuration Register (scr_el3) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, SCR_EL3
            llvm_asm!("mrs $0, scr_el3" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Secure Configuration Register (scr_el3) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR SCR_EL3, <Xt>
            llvm_asm!("msr scr_el3, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0x7bfe3fff8f;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 532546584463;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: hxen_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn hxen_1_extract(&self) -> u64 {
        // bits 38..38
        self.val.get_bits(38..=38)
    }

    /// reads the current register value and extract field `hxen_1` from it
    pub fn hxen_1_read(&mut self) -> u64 {
        Self::with_reg_val().hxen_1_extract()
    }

    /// inserts the given value `val` into the field `hxen_1`
    pub fn hxen_1_insert(&mut self, val: u64) -> &mut self {
        // bits 38..38
        self.val.set_bits(38..=38, val);
        self
    }

    /// reads the register, updates the `hxen_1` field, and writes the updated value
    pub fn hxen_1_write(&mut self, val: u64) {
        Self::with_reg_val().hxen_1_insert(val).write();
    }

    /*
     * Field: aden_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn aden_1_extract(&self) -> u64 {
        // bits 37..37
        self.val.get_bits(37..=37)
    }

    /// reads the current register value and extract field `aden_1` from it
    pub fn aden_1_read(&mut self) -> u64 {
        Self::with_reg_val().aden_1_extract()
    }

    /// inserts the given value `val` into the field `aden_1`
    pub fn aden_1_insert(&mut self, val: u64) -> &mut self {
        // bits 37..37
        self.val.set_bits(37..=37, val);
        self
    }

    /// reads the register, updates the `aden_1` field, and writes the updated value
    pub fn aden_1_write(&mut self, val: u64) {
        Self::with_reg_val().aden_1_insert(val).write();
    }

    /*
     * Field: enas0_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn enas0_1_extract(&self) -> u64 {
        // bits 36..36
        self.val.get_bits(36..=36)
    }

    /// reads the current register value and extract field `enas0_1` from it
    pub fn enas0_1_read(&mut self) -> u64 {
        Self::with_reg_val().enas0_1_extract()
    }

    /// inserts the given value `val` into the field `enas0_1`
    pub fn enas0_1_insert(&mut self, val: u64) -> &mut self {
        // bits 36..36
        self.val.set_bits(36..=36, val);
        self
    }

    /// reads the register, updates the `enas0_1` field, and writes the updated value
    pub fn enas0_1_write(&mut self, val: u64) {
        Self::with_reg_val().enas0_1_insert(val).write();
    }

    /*
     * Field: amvoffen_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn amvoffen_1_extract(&self) -> u64 {
        // bits 35..35
        self.val.get_bits(35..=35)
    }

    /// reads the current register value and extract field `amvoffen_1` from it
    pub fn amvoffen_1_read(&mut self) -> u64 {
        Self::with_reg_val().amvoffen_1_extract()
    }

    /// inserts the given value `val` into the field `amvoffen_1`
    pub fn amvoffen_1_insert(&mut self, val: u64) -> &mut self {
        // bits 35..35
        self.val.set_bits(35..=35, val);
        self
    }

    /// reads the register, updates the `amvoffen_1` field, and writes the updated value
    pub fn amvoffen_1_write(&mut self, val: u64) {
        Self::with_reg_val().amvoffen_1_insert(val).write();
    }

    /*
     * Field: twedel_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn twedel_1_extract(&self) -> u64 {
        // bits 30..33
        self.val.get_bits(30..=33)
    }

    /// reads the current register value and extract field `twedel_1` from it
    pub fn twedel_1_read(&mut self) -> u64 {
        Self::with_reg_val().twedel_1_extract()
    }

    /// inserts the given value `val` into the field `twedel_1`
    pub fn twedel_1_insert(&mut self, val: u64) -> &mut self {
        // bits 30..33
        self.val.set_bits(30..=33, val);
        self
    }

    /// reads the register, updates the `twedel_1` field, and writes the updated value
    pub fn twedel_1_write(&mut self, val: u64) {
        Self::with_reg_val().twedel_1_insert(val).write();
    }

    /*
     * Field: tweden_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tweden_1_extract(&self) -> u64 {
        // bits 29..29
        self.val.get_bits(29..=29)
    }

    /// reads the current register value and extract field `tweden_1` from it
    pub fn tweden_1_read(&mut self) -> u64 {
        Self::with_reg_val().tweden_1_extract()
    }

    /// inserts the given value `val` into the field `tweden_1`
    pub fn tweden_1_insert(&mut self, val: u64) -> &mut self {
        // bits 29..29
        self.val.set_bits(29..=29, val);
        self
    }

    /// reads the register, updates the `tweden_1` field, and writes the updated value
    pub fn tweden_1_write(&mut self, val: u64) {
        Self::with_reg_val().tweden_1_insert(val).write();
    }

    /*
     * Field: ecven_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ecven_1_extract(&self) -> u64 {
        // bits 28..28
        self.val.get_bits(28..=28)
    }

    /// reads the current register value and extract field `ecven_1` from it
    pub fn ecven_1_read(&mut self) -> u64 {
        Self::with_reg_val().ecven_1_extract()
    }

    /// inserts the given value `val` into the field `ecven_1`
    pub fn ecven_1_insert(&mut self, val: u64) -> &mut self {
        // bits 28..28
        self.val.set_bits(28..=28, val);
        self
    }

    /// reads the register, updates the `ecven_1` field, and writes the updated value
    pub fn ecven_1_write(&mut self, val: u64) {
        Self::with_reg_val().ecven_1_insert(val).write();
    }

    /*
     * Field: fgten_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fgten_1_extract(&self) -> u64 {
        // bits 27..27
        self.val.get_bits(27..=27)
    }

    /// reads the current register value and extract field `fgten_1` from it
    pub fn fgten_1_read(&mut self) -> u64 {
        Self::with_reg_val().fgten_1_extract()
    }

    /// inserts the given value `val` into the field `fgten_1`
    pub fn fgten_1_insert(&mut self, val: u64) -> &mut self {
        // bits 27..27
        self.val.set_bits(27..=27, val);
        self
    }

    /// reads the register, updates the `fgten_1` field, and writes the updated value
    pub fn fgten_1_write(&mut self, val: u64) {
        Self::with_reg_val().fgten_1_insert(val).write();
    }

    /*
     * Field: ata_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ata_1_extract(&self) -> u64 {
        // bits 26..26
        self.val.get_bits(26..=26)
    }

    /// reads the current register value and extract field `ata_1` from it
    pub fn ata_1_read(&mut self) -> u64 {
        Self::with_reg_val().ata_1_extract()
    }

    /// inserts the given value `val` into the field `ata_1`
    pub fn ata_1_insert(&mut self, val: u64) -> &mut self {
        // bits 26..26
        self.val.set_bits(26..=26, val);
        self
    }

    /// reads the register, updates the `ata_1` field, and writes the updated value
    pub fn ata_1_write(&mut self, val: u64) {
        Self::with_reg_val().ata_1_insert(val).write();
    }

    /*
     * Field: enscxt_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn enscxt_1_extract(&self) -> u64 {
        // bits 25..25
        self.val.get_bits(25..=25)
    }

    /// reads the current register value and extract field `enscxt_1` from it
    pub fn enscxt_1_read(&mut self) -> u64 {
        Self::with_reg_val().enscxt_1_extract()
    }

    /// inserts the given value `val` into the field `enscxt_1`
    pub fn enscxt_1_insert(&mut self, val: u64) -> &mut self {
        // bits 25..25
        self.val.set_bits(25..=25, val);
        self
    }

    /// reads the register, updates the `enscxt_1` field, and writes the updated value
    pub fn enscxt_1_write(&mut self, val: u64) {
        Self::with_reg_val().enscxt_1_insert(val).write();
    }

    /*
     * Field: fien_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fien_1_extract(&self) -> u64 {
        // bits 21..21
        self.val.get_bits(21..=21)
    }

    /// reads the current register value and extract field `fien_1` from it
    pub fn fien_1_read(&mut self) -> u64 {
        Self::with_reg_val().fien_1_extract()
    }

    /// inserts the given value `val` into the field `fien_1`
    pub fn fien_1_insert(&mut self, val: u64) -> &mut self {
        // bits 21..21
        self.val.set_bits(21..=21, val);
        self
    }

    /// reads the register, updates the `fien_1` field, and writes the updated value
    pub fn fien_1_write(&mut self, val: u64) {
        Self::with_reg_val().fien_1_insert(val).write();
    }

    /*
     * Field: nmea_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn nmea_1_extract(&self) -> u64 {
        // bits 20..20
        self.val.get_bits(20..=20)
    }

    /// reads the current register value and extract field `nmea_1` from it
    pub fn nmea_1_read(&mut self) -> u64 {
        Self::with_reg_val().nmea_1_extract()
    }

    /// inserts the given value `val` into the field `nmea_1`
    pub fn nmea_1_insert(&mut self, val: u64) -> &mut self {
        // bits 20..20
        self.val.set_bits(20..=20, val);
        self
    }

    /// reads the register, updates the `nmea_1` field, and writes the updated value
    pub fn nmea_1_write(&mut self, val: u64) {
        Self::with_reg_val().nmea_1_insert(val).write();
    }

    /*
     * Field: ease_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ease_1_extract(&self) -> u64 {
        // bits 19..19
        self.val.get_bits(19..=19)
    }

    /// reads the current register value and extract field `ease_1` from it
    pub fn ease_1_read(&mut self) -> u64 {
        Self::with_reg_val().ease_1_extract()
    }

    /// inserts the given value `val` into the field `ease_1`
    pub fn ease_1_insert(&mut self, val: u64) -> &mut self {
        // bits 19..19
        self.val.set_bits(19..=19, val);
        self
    }

    /// reads the register, updates the `ease_1` field, and writes the updated value
    pub fn ease_1_write(&mut self, val: u64) {
        Self::with_reg_val().ease_1_insert(val).write();
    }

    /*
     * Field: eel2_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn eel2_1_extract(&self) -> u64 {
        // bits 18..18
        self.val.get_bits(18..=18)
    }

    /// reads the current register value and extract field `eel2_1` from it
    pub fn eel2_1_read(&mut self) -> u64 {
        Self::with_reg_val().eel2_1_extract()
    }

    /// inserts the given value `val` into the field `eel2_1`
    pub fn eel2_1_insert(&mut self, val: u64) -> &mut self {
        // bits 18..18
        self.val.set_bits(18..=18, val);
        self
    }

    /// reads the register, updates the `eel2_1` field, and writes the updated value
    pub fn eel2_1_write(&mut self, val: u64) {
        Self::with_reg_val().eel2_1_insert(val).write();
    }

    /*
     * Field: api_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn api_1_extract(&self) -> u64 {
        // bits 17..17
        self.val.get_bits(17..=17)
    }

    /// reads the current register value and extract field `api_1` from it
    pub fn api_1_read(&mut self) -> u64 {
        Self::with_reg_val().api_1_extract()
    }

    /// inserts the given value `val` into the field `api_1`
    pub fn api_1_insert(&mut self, val: u64) -> &mut self {
        // bits 17..17
        self.val.set_bits(17..=17, val);
        self
    }

    /// reads the register, updates the `api_1` field, and writes the updated value
    pub fn api_1_write(&mut self, val: u64) {
        Self::with_reg_val().api_1_insert(val).write();
    }

    /*
     * Field: api_2
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn api_2_extract(&self) -> u64 {
        // bits 17..17
        self.val.get_bits(17..=17)
    }

    /// reads the current register value and extract field `api_2` from it
    pub fn api_2_read(&mut self) -> u64 {
        Self::with_reg_val().api_2_extract()
    }

    /// inserts the given value `val` into the field `api_2`
    pub fn api_2_insert(&mut self, val: u64) -> &mut self {
        // bits 17..17
        self.val.set_bits(17..=17, val);
        self
    }

    /// reads the register, updates the `api_2` field, and writes the updated value
    pub fn api_2_write(&mut self, val: u64) {
        Self::with_reg_val().api_2_insert(val).write();
    }

    /*
     * Field: apk_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn apk_1_extract(&self) -> u64 {
        // bits 16..16
        self.val.get_bits(16..=16)
    }

    /// reads the current register value and extract field `apk_1` from it
    pub fn apk_1_read(&mut self) -> u64 {
        Self::with_reg_val().apk_1_extract()
    }

    /// inserts the given value `val` into the field `apk_1`
    pub fn apk_1_insert(&mut self, val: u64) -> &mut self {
        // bits 16..16
        self.val.set_bits(16..=16, val);
        self
    }

    /// reads the register, updates the `apk_1` field, and writes the updated value
    pub fn apk_1_write(&mut self, val: u64) {
        Self::with_reg_val().apk_1_insert(val).write();
    }

    /*
     * Field: terr_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn terr_1_extract(&self) -> u64 {
        // bits 15..15
        self.val.get_bits(15..=15)
    }

    /// reads the current register value and extract field `terr_1` from it
    pub fn terr_1_read(&mut self) -> u64 {
        Self::with_reg_val().terr_1_extract()
    }

    /// inserts the given value `val` into the field `terr_1`
    pub fn terr_1_insert(&mut self, val: u64) -> &mut self {
        // bits 15..15
        self.val.set_bits(15..=15, val);
        self
    }

    /// reads the register, updates the `terr_1` field, and writes the updated value
    pub fn terr_1_write(&mut self, val: u64) {
        Self::with_reg_val().terr_1_insert(val).write();
    }

    /*
     * Field: tlor_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tlor_1_extract(&self) -> u64 {
        // bits 14..14
        self.val.get_bits(14..=14)
    }

    /// reads the current register value and extract field `tlor_1` from it
    pub fn tlor_1_read(&mut self) -> u64 {
        Self::with_reg_val().tlor_1_extract()
    }

    /// inserts the given value `val` into the field `tlor_1`
    pub fn tlor_1_insert(&mut self, val: u64) -> &mut self {
        // bits 14..14
        self.val.set_bits(14..=14, val);
        self
    }

    /// reads the register, updates the `tlor_1` field, and writes the updated value
    pub fn tlor_1_write(&mut self, val: u64) {
        Self::with_reg_val().tlor_1_insert(val).write();
    }

    /*
     * Field: twe
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn twe_extract(&self) -> u64 {
        // bits 13..13
        self.val.get_bits(13..=13)
    }

    /// reads the current register value and extract field `twe` from it
    pub fn twe_read(&mut self) -> u64 {
        Self::with_reg_val().twe_extract()
    }

    /// inserts the given value `val` into the field `twe`
    pub fn twe_insert(&mut self, val: u64) -> &mut self {
        // bits 13..13
        self.val.set_bits(13..=13, val);
        self
    }

    /// reads the register, updates the `twe` field, and writes the updated value
    pub fn twe_write(&mut self, val: u64) {
        Self::with_reg_val().twe_insert(val).write();
    }

    /*
     * Field: twi
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn twi_extract(&self) -> u64 {
        // bits 12..12
        self.val.get_bits(12..=12)
    }

    /// reads the current register value and extract field `twi` from it
    pub fn twi_read(&mut self) -> u64 {
        Self::with_reg_val().twi_extract()
    }

    /// inserts the given value `val` into the field `twi`
    pub fn twi_insert(&mut self, val: u64) -> &mut self {
        // bits 12..12
        self.val.set_bits(12..=12, val);
        self
    }

    /// reads the register, updates the `twi` field, and writes the updated value
    pub fn twi_write(&mut self, val: u64) {
        Self::with_reg_val().twi_insert(val).write();
    }

    /*
     * Field: st
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn st_extract(&self) -> u64 {
        // bits 11..11
        self.val.get_bits(11..=11)
    }

    /// reads the current register value and extract field `st` from it
    pub fn st_read(&mut self) -> u64 {
        Self::with_reg_val().st_extract()
    }

    /// inserts the given value `val` into the field `st`
    pub fn st_insert(&mut self, val: u64) -> &mut self {
        // bits 11..11
        self.val.set_bits(11..=11, val);
        self
    }

    /// reads the register, updates the `st` field, and writes the updated value
    pub fn st_write(&mut self, val: u64) {
        Self::with_reg_val().st_insert(val).write();
    }

    /*
     * Field: rw_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn rw_1_extract(&self) -> u64 {
        // bits 10..10
        self.val.get_bits(10..=10)
    }

    /// reads the current register value and extract field `rw_1` from it
    pub fn rw_1_read(&mut self) -> u64 {
        Self::with_reg_val().rw_1_extract()
    }

    /// inserts the given value `val` into the field `rw_1`
    pub fn rw_1_insert(&mut self, val: u64) -> &mut self {
        // bits 10..10
        self.val.set_bits(10..=10, val);
        self
    }

    /// reads the register, updates the `rw_1` field, and writes the updated value
    pub fn rw_1_write(&mut self, val: u64) {
        Self::with_reg_val().rw_1_insert(val).write();
    }

    /*
     * Field: sif_1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn sif_1_extract(&self) -> u64 {
        // bits 9..9
        self.val.get_bits(9..=9)
    }

    /// reads the current register value and extract field `sif_1` from it
    pub fn sif_1_read(&mut self) -> u64 {
        Self::with_reg_val().sif_1_extract()
    }

    /// inserts the given value `val` into the field `sif_1`
    pub fn sif_1_insert(&mut self, val: u64) -> &mut self {
        // bits 9..9
        self.val.set_bits(9..=9, val);
        self
    }

    /// reads the register, updates the `sif_1` field, and writes the updated value
    pub fn sif_1_write(&mut self, val: u64) {
        Self::with_reg_val().sif_1_insert(val).write();
    }

    /*
     * Field: sif_2
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn sif_2_extract(&self) -> u64 {
        // bits 9..9
        self.val.get_bits(9..=9)
    }

    /// reads the current register value and extract field `sif_2` from it
    pub fn sif_2_read(&mut self) -> u64 {
        Self::with_reg_val().sif_2_extract()
    }

    /// inserts the given value `val` into the field `sif_2`
    pub fn sif_2_insert(&mut self, val: u64) -> &mut self {
        // bits 9..9
        self.val.set_bits(9..=9, val);
        self
    }

    /// reads the register, updates the `sif_2` field, and writes the updated value
    pub fn sif_2_write(&mut self, val: u64) {
        Self::with_reg_val().sif_2_insert(val).write();
    }

    /*
     * Field: hce
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn hce_extract(&self) -> u64 {
        // bits 8..8
        self.val.get_bits(8..=8)
    }

    /// reads the current register value and extract field `hce` from it
    pub fn hce_read(&mut self) -> u64 {
        Self::with_reg_val().hce_extract()
    }

    /// inserts the given value `val` into the field `hce`
    pub fn hce_insert(&mut self, val: u64) -> &mut self {
        // bits 8..8
        self.val.set_bits(8..=8, val);
        self
    }

    /// reads the register, updates the `hce` field, and writes the updated value
    pub fn hce_write(&mut self, val: u64) {
        Self::with_reg_val().hce_insert(val).write();
    }

    /*
     * Field: smd
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn smd_extract(&self) -> u64 {
        // bits 7..7
        self.val.get_bits(7..=7)
    }

    /// reads the current register value and extract field `smd` from it
    pub fn smd_read(&mut self) -> u64 {
        Self::with_reg_val().smd_extract()
    }

    /// inserts the given value `val` into the field `smd`
    pub fn smd_insert(&mut self, val: u64) -> &mut self {
        // bits 7..7
        self.val.set_bits(7..=7, val);
        self
    }

    /// reads the register, updates the `smd` field, and writes the updated value
    pub fn smd_write(&mut self, val: u64) {
        Self::with_reg_val().smd_insert(val).write();
    }

    /*
     * Field: ea
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ea_extract(&self) -> u64 {
        // bits 3..3
        self.val.get_bits(3..=3)
    }

    /// reads the current register value and extract field `ea` from it
    pub fn ea_read(&mut self) -> u64 {
        Self::with_reg_val().ea_extract()
    }

    /// inserts the given value `val` into the field `ea`
    pub fn ea_insert(&mut self, val: u64) -> &mut self {
        // bits 3..3
        self.val.set_bits(3..=3, val);
        self
    }

    /// reads the register, updates the `ea` field, and writes the updated value
    pub fn ea_write(&mut self, val: u64) {
        Self::with_reg_val().ea_insert(val).write();
    }

    /*
     * Field: fiq
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn fiq_extract(&self) -> u64 {
        // bits 2..2
        self.val.get_bits(2..=2)
    }

    /// reads the current register value and extract field `fiq` from it
    pub fn fiq_read(&mut self) -> u64 {
        Self::with_reg_val().fiq_extract()
    }

    /// inserts the given value `val` into the field `fiq`
    pub fn fiq_insert(&mut self, val: u64) -> &mut self {
        // bits 2..2
        self.val.set_bits(2..=2, val);
        self
    }

    /// reads the register, updates the `fiq` field, and writes the updated value
    pub fn fiq_write(&mut self, val: u64) {
        Self::with_reg_val().fiq_insert(val).write();
    }

    /*
     * Field: irq
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn irq_extract(&self) -> u64 {
        // bits 1..1
        self.val.get_bits(1..=1)
    }

    /// reads the current register value and extract field `irq` from it
    pub fn irq_read(&mut self) -> u64 {
        Self::with_reg_val().irq_extract()
    }

    /// inserts the given value `val` into the field `irq`
    pub fn irq_insert(&mut self, val: u64) -> &mut self {
        // bits 1..1
        self.val.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `irq` field, and writes the updated value
    pub fn irq_write(&mut self, val: u64) {
        Self::with_reg_val().irq_insert(val).write();
    }

    /*
     * Field: ns
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn ns_extract(&self) -> u64 {
        // bits 0..0
        self.val.get_bits(0..=0)
    }

    /// reads the current register value and extract field `ns` from it
    pub fn ns_read(&mut self) -> u64 {
        Self::with_reg_val().ns_extract()
    }

    /// inserts the given value `val` into the field `ns`
    pub fn ns_insert(&mut self, val: u64) -> &mut self {
        // bits 0..0
        self.val.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `ns` field, and writes the updated value
    pub fn ns_write(&mut self, val: u64) {
        Self::with_reg_val().ns_insert(val).write();
    }
}

impl Default for ScrEl3 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> ScrEl3 {
        ScrEl3(0)
    }
}
