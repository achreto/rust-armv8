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
 * Generated on: 2022-08-22T16:35:53.068895
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
 * Register:    Monitor DCC Interrupt Enable Register (mdccint_el1)
 * Group:       Debug registers
 * Type:        64-bit Register
 * Description: Enables interrupt requests to be signaled based on the DCC status flags.
 * File:        AArch64-mdccint_el1.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Monitor DCC Interrupt Enable Register value in memory
pub struct MdccintEl1(u64);

/// struct implementation for accessing the fields of register mdccint_el1
impl MdccintEl1 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> MdccintEl1 {
        Self::default()
    }

    /// collects the modifications of MdccintEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> MdccintEl1 {
        MdccintEl1(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> MdccintEl1 {
        let curval = Self::reg_rawrd() & 0x60000000;
        MdccintEl1(curval)
    }

    /// reading the Monitor DCC Interrupt Enable Register (mdccint_el1) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, MDCCINT_EL1
            asm!("mrs {}, mdccint_el1", out(reg) regval);
        }
        return regval;
    }

    /// writing the Monitor DCC Interrupt Enable Register (mdccint_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR MDCCINT_EL1, <Xt>
            asm!("msr mdccint_el1, {}", in(reg) val);
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x60000000;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 1610612736;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: rx
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn rx_extract(&self) -> u64 {
        // bits 30..30
        self.0.get_bits(30..=30)
    }

    /// reads the current register value and extract field `rx` from it
    pub fn rx_read() -> u64 {
        Self::with_reg_val().rx_extract()
    }

    /// inserts the given value `val` into the field `rx`
    pub fn rx_insert(&mut self, val: u64) -> &mut Self {
        // bits 30..30
        self.0.set_bits(30..=30, val);
        self
    }

    /// reads the register, updates the `rx` field, and writes the updated value
    pub fn rx_write(val: u64) {
        Self::with_reg_val().rx_insert(val).write();
    }

    /*
     * Field: tx
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tx_extract(&self) -> u64 {
        // bits 29..29
        self.0.get_bits(29..=29)
    }

    /// reads the current register value and extract field `tx` from it
    pub fn tx_read() -> u64 {
        Self::with_reg_val().tx_extract()
    }

    /// inserts the given value `val` into the field `tx`
    pub fn tx_insert(&mut self, val: u64) -> &mut Self {
        // bits 29..29
        self.0.set_bits(29..=29, val);
        self
    }

    /// reads the register, updates the `tx` field, and writes the updated value
    pub fn tx_write(val: u64) {
        Self::with_reg_val().tx_insert(val).write();
    }
}

impl Default for MdccintEl1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> MdccintEl1 {
        MdccintEl1(0)
    }
}
