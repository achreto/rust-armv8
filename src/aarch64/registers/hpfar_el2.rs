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
 * Generated on: 2022-08-22T16:35:53.060835
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
 * Register:    Hypervisor IPA Fault Address Register (hpfar_el2)
 * Group:       Exception and fault handling registers
 * Type:        64-bit Register
 * Description: Holds the faulting IPA for some aborts on a stage 2 translation taken to EL2.
 * File:        AArch64-hpfar_el2.xml
 */


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Hypervisor IPA Fault Address Register value in memory
pub struct HpfarEl2(u64);

/// struct implementation for accessing the fields of register hpfar_el2
impl HpfarEl2 {

    /// creates a new default value
    #[inline(always)]
    pub fn new() -> HpfarEl2 {
        Self::default()
    }

    /// collects the modifications of HpfarEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> HpfarEl2 {
        HpfarEl2(self.0)
    }

    
    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() ->  HpfarEl2 {
        let curval = Self::reg_rawrd() & 0x80000ffffffffff0;
        HpfarEl2(curval)
    }


    
    /// reading the Hypervisor IPA Fault Address Register (hpfar_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, HPFAR_EL2
            asm!("mrs {}, hpfar_el2", out(reg) regval);
        }
        return regval;
    }


    /// writing the Hypervisor IPA Fault Address Register (hpfar_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR HPFAR_EL2, <Xt>
            asm!("msr hpfar_el2, {}", in(reg) val);
        }
    }



    
    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut Self {
        self.0 = Self::reg_rawrd() & 0x80000ffffffffff0;
        self
    }

    
    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.0)
    }


    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.0 = newval & 9223389629040820208;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }


    
    /*
     * Field: ns_1
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn ns_1_extract(&self) -> u64 {
        // bits 63..63
        self.0.get_bits(63..=63)
    }

    /// reads the current register value and extract field `ns_1` from it
    pub fn ns_1_read() -> u64 {
        Self::with_reg_val().ns_1_extract()
    }

    /// inserts the given value `val` into the field `ns_1`
    pub fn ns_1_insert(&mut self, val: u64) -> &mut Self {
        // bits 63..63
        self.0.set_bits(63..=63, val);
        self
    }

    /// reads the register, updates the `ns_1` field, and writes the updated value
    pub fn ns_1_write(val: u64) {
        Self::with_reg_val().ns_1_insert(val).write();
    }

    /*
     * Field: fipa
     * --------------------------------------------------------------------------------------------
     */


    /// extracts field val from current value
    pub fn fipa_extract(&self) -> u64 {
        // bits 4..43
        self.0.get_bits(4..=43)
    }

    /// reads the current register value and extract field `fipa` from it
    pub fn fipa_read() -> u64 {
        Self::with_reg_val().fipa_extract()
    }

    /// inserts the given value `val` into the field `fipa`
    pub fn fipa_insert(&mut self, val: u64) -> &mut Self {
        // bits 4..43
        self.0.set_bits(4..=43, val);
        self
    }

    /// reads the register, updates the `fipa` field, and writes the updated value
    pub fn fipa_write(val: u64) {
        Self::with_reg_val().fipa_insert(val).write();
    }

}

impl Default for HpfarEl2 {
    /// creates a new default value
    #[inline(always)]
    fn default() -> HpfarEl2 {
        HpfarEl2(0)
    }
}
