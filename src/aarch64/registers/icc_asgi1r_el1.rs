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
 * Generated on: 2022-08-22T16:25:59.079337
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
 * Register:    Interrupt Controller Alias Software Generated Interrupt Group 1 Register (icc_asgi1r_el1)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Generates Group 1 SGIs for the Security state that is not the current Security state.
 * File:        AArch64-icc_asgi1r_el1.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Interrupt Controller Alias Software Generated Interrupt Group 1 Register value in memory
pub struct IccAsgi1rEl1(u64);

/// struct implementation for accessing the fields of register icc_asgi1r_el1
impl IccAsgi1rEl1 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IccAsgi1rEl1 {
        Self::default()
    }

    /// collects the modifications of IccAsgi1rEl1 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IccAsgi1rEl1 {
        IccAsgi1rEl1(self.0)
    }

    // no current() method as it is write only

    
    /// writing the Interrupt Controller Alias Software Generated Interrupt Group 1 Register (icc_asgi1r_el1) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR ICC_ASGI1R_EL1, <Xt>
            asm!("msr icc_asgi1r_el1, {}", in(reg) val);
        }
    }

// register is not readable. not emitting read accessor


    // no read() method as it is write only
    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 72042196848607231;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: aff3
     * --------------------------------------------------------------------------------------------
     */

// no extract() method for field aff3
    /// inserts the given value `val` into the field `aff3`
    pub fn aff3_insert(&mut self, val: u64) -> &mut Self {
        // bits 48..55
        self.0.set_bits(48..=55, val);
        self
    }

    /// sets the field `aff3` to the given value `val`
    pub fn aff3_write(&mut self, val: u64) {
        Self::default().aff3_insert(val).write();
    }

    /*
     * Field: rs
     * --------------------------------------------------------------------------------------------
     */

// no extract() method for field rs
    /// inserts the given value `val` into the field `rs`
    pub fn rs_insert(&mut self, val: u64) -> &mut Self {
        // bits 44..47
        self.0.set_bits(44..=47, val);
        self
    }

    /// sets the field `rs` to the given value `val`
    pub fn rs_write(&mut self, val: u64) {
        Self::default().rs_insert(val).write();
    }

    /*
     * Field: irm
     * --------------------------------------------------------------------------------------------
     */

// no extract() method for field irm
    /// inserts the given value `val` into the field `irm`
    pub fn irm_insert(&mut self, val: u64) -> &mut Self {
        // bits 40..40
        self.0.set_bits(40..=40, val);
        self
    }

    /// sets the field `irm` to the given value `val`
    pub fn irm_write(&mut self, val: u64) {
        Self::default().irm_insert(val).write();
    }

    /*
     * Field: aff2
     * --------------------------------------------------------------------------------------------
     */

// no extract() method for field aff2
    /// inserts the given value `val` into the field `aff2`
    pub fn aff2_insert(&mut self, val: u64) -> &mut Self {
        // bits 32..39
        self.0.set_bits(32..=39, val);
        self
    }

    /// sets the field `aff2` to the given value `val`
    pub fn aff2_write(&mut self, val: u64) {
        Self::default().aff2_insert(val).write();
    }

    /*
     * Field: intid
     * --------------------------------------------------------------------------------------------
     */

// no extract() method for field intid
    /// inserts the given value `val` into the field `intid`
    pub fn intid_insert(&mut self, val: u64) -> &mut Self {
        // bits 24..27
        self.0.set_bits(24..=27, val);
        self
    }

    /// sets the field `intid` to the given value `val`
    pub fn intid_write(&mut self, val: u64) {
        Self::default().intid_insert(val).write();
    }

    /*
     * Field: aff1
     * --------------------------------------------------------------------------------------------
     */

// no extract() method for field aff1
    /// inserts the given value `val` into the field `aff1`
    pub fn aff1_insert(&mut self, val: u64) -> &mut Self {
        // bits 16..23
        self.0.set_bits(16..=23, val);
        self
    }

    /// sets the field `aff1` to the given value `val`
    pub fn aff1_write(&mut self, val: u64) {
        Self::default().aff1_insert(val).write();
    }

    /*
     * Field: targetlist
     * --------------------------------------------------------------------------------------------
     */

// no extract() method for field targetlist
    /// inserts the given value `val` into the field `targetlist`
    pub fn targetlist_insert(&mut self, val: u64) -> &mut Self {
        // bits 0..15
        self.0.set_bits(0..=15, val);
        self
    }

    /// sets the field `targetlist` to the given value `val`
    pub fn targetlist_write(&mut self, val: u64) {
        Self::default().targetlist_insert(val).write();
    }

}

impl Default for IccAsgi1rEl1 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> IccAsgi1rEl1 {
        IccAsgi1rEl1(0)
    }
}
