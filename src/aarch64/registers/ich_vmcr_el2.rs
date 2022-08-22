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
 * Generated on: 2022-08-22T15:51:28.520234
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
 * Register:    Interrupt Controller Virtual Machine Control Register (ich_vmcr_el2)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Enables the hypervisor to save and restore the virtual machine view of the GIC state.
 * File:        AArch64-ich_vmcr_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Interrupt Controller Virtual Machine Control Register value in memory
pub struct IchVmcrEl2(u64);

/// struct implementation for accessing the fields of register ich_vmcr_el2
impl IchVmcrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IchVmcrEl2 {
        Self::default()
    }

    /// collects the modifications of IchVmcrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IchVmcrEl2 {
        IchVmcrEl2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> IchVmcrEl2 {
        let curval = Self::reg_rawrd() & 0xfffc021f;
        IchVmcrEl2(curval)
    }

    /// reading the Interrupt Controller Virtual Machine Control Register (ich_vmcr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ICH_VMCR_EL2
            llvm_asm!("mrs $0, ich_vmcr_el2" : "=r"(regval));
        }
        return regval;
    }

    /// writing the Interrupt Controller Virtual Machine Control Register (ich_vmcr_el2) register
    #[inline(always)]
    fn reg_rawwr(val: u64) {
        unsafe {
            // MSR ICH_VMCR_EL2, <Xt>
            llvm_asm!("msr ich_vmcr_el2, $0" : : "r"(val));
        }
    }

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xfffc021f;
        self
    }

    /// writes the current value to the register
    #[inline(always)]
    pub fn write(&self) {
        Self::reg_rawwr(self.val)
    }

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 4294705695;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: vpmr
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn vpmr_extract(&self) -> u64 {
        // bits 24..31
        self.val.get_bits(24..=31)
    }

    /// reads the current register value and extract field `vpmr` from it
    pub fn vpmr_read(&mut self) -> u64 {
        Self::with_reg_val().vpmr_extract()
    }

    /// inserts the given value `val` into the field `vpmr`
    pub fn vpmr_insert(&mut self, val: u64) -> &mut self {
        // bits 24..31
        self.val.set_bits(24..=31, val);
        self
    }

    /// reads the register, updates the `vpmr` field, and writes the updated value
    pub fn vpmr_write(&mut self, val: u64) {
        Self::with_reg_val().vpmr_insert(val).write();
    }

    /*
     * Field: vbpr0
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn vbpr0_extract(&self) -> u64 {
        // bits 21..23
        self.val.get_bits(21..=23)
    }

    /// reads the current register value and extract field `vbpr0` from it
    pub fn vbpr0_read(&mut self) -> u64 {
        Self::with_reg_val().vbpr0_extract()
    }

    /// inserts the given value `val` into the field `vbpr0`
    pub fn vbpr0_insert(&mut self, val: u64) -> &mut self {
        // bits 21..23
        self.val.set_bits(21..=23, val);
        self
    }

    /// reads the register, updates the `vbpr0` field, and writes the updated value
    pub fn vbpr0_write(&mut self, val: u64) {
        Self::with_reg_val().vbpr0_insert(val).write();
    }

    /*
     * Field: vbpr1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn vbpr1_extract(&self) -> u64 {
        // bits 18..20
        self.val.get_bits(18..=20)
    }

    /// reads the current register value and extract field `vbpr1` from it
    pub fn vbpr1_read(&mut self) -> u64 {
        Self::with_reg_val().vbpr1_extract()
    }

    /// inserts the given value `val` into the field `vbpr1`
    pub fn vbpr1_insert(&mut self, val: u64) -> &mut self {
        // bits 18..20
        self.val.set_bits(18..=20, val);
        self
    }

    /// reads the register, updates the `vbpr1` field, and writes the updated value
    pub fn vbpr1_write(&mut self, val: u64) {
        Self::with_reg_val().vbpr1_insert(val).write();
    }

    /*
     * Field: veoim
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn veoim_extract(&self) -> u64 {
        // bits 9..9
        self.val.get_bits(9..=9)
    }

    /// reads the current register value and extract field `veoim` from it
    pub fn veoim_read(&mut self) -> u64 {
        Self::with_reg_val().veoim_extract()
    }

    /// inserts the given value `val` into the field `veoim`
    pub fn veoim_insert(&mut self, val: u64) -> &mut self {
        // bits 9..9
        self.val.set_bits(9..=9, val);
        self
    }

    /// reads the register, updates the `veoim` field, and writes the updated value
    pub fn veoim_write(&mut self, val: u64) {
        Self::with_reg_val().veoim_insert(val).write();
    }

    /*
     * Field: vcbpr
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn vcbpr_extract(&self) -> u64 {
        // bits 4..4
        self.val.get_bits(4..=4)
    }

    /// reads the current register value and extract field `vcbpr` from it
    pub fn vcbpr_read(&mut self) -> u64 {
        Self::with_reg_val().vcbpr_extract()
    }

    /// inserts the given value `val` into the field `vcbpr`
    pub fn vcbpr_insert(&mut self, val: u64) -> &mut self {
        // bits 4..4
        self.val.set_bits(4..=4, val);
        self
    }

    /// reads the register, updates the `vcbpr` field, and writes the updated value
    pub fn vcbpr_write(&mut self, val: u64) {
        Self::with_reg_val().vcbpr_insert(val).write();
    }

    /*
     * Field: vfiqen
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn vfiqen_extract(&self) -> u64 {
        // bits 3..3
        self.val.get_bits(3..=3)
    }

    /// reads the current register value and extract field `vfiqen` from it
    pub fn vfiqen_read(&mut self) -> u64 {
        Self::with_reg_val().vfiqen_extract()
    }

    /// inserts the given value `val` into the field `vfiqen`
    pub fn vfiqen_insert(&mut self, val: u64) -> &mut self {
        // bits 3..3
        self.val.set_bits(3..=3, val);
        self
    }

    /// reads the register, updates the `vfiqen` field, and writes the updated value
    pub fn vfiqen_write(&mut self, val: u64) {
        Self::with_reg_val().vfiqen_insert(val).write();
    }

    /*
     * Field: vackctl
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn vackctl_extract(&self) -> u64 {
        // bits 2..2
        self.val.get_bits(2..=2)
    }

    /// reads the current register value and extract field `vackctl` from it
    pub fn vackctl_read(&mut self) -> u64 {
        Self::with_reg_val().vackctl_extract()
    }

    /// inserts the given value `val` into the field `vackctl`
    pub fn vackctl_insert(&mut self, val: u64) -> &mut self {
        // bits 2..2
        self.val.set_bits(2..=2, val);
        self
    }

    /// reads the register, updates the `vackctl` field, and writes the updated value
    pub fn vackctl_write(&mut self, val: u64) {
        Self::with_reg_val().vackctl_insert(val).write();
    }

    /*
     * Field: veng1
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn veng1_extract(&self) -> u64 {
        // bits 1..1
        self.val.get_bits(1..=1)
    }

    /// reads the current register value and extract field `veng1` from it
    pub fn veng1_read(&mut self) -> u64 {
        Self::with_reg_val().veng1_extract()
    }

    /// inserts the given value `val` into the field `veng1`
    pub fn veng1_insert(&mut self, val: u64) -> &mut self {
        // bits 1..1
        self.val.set_bits(1..=1, val);
        self
    }

    /// reads the register, updates the `veng1` field, and writes the updated value
    pub fn veng1_write(&mut self, val: u64) {
        Self::with_reg_val().veng1_insert(val).write();
    }

    /*
     * Field: veng0
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn veng0_extract(&self) -> u64 {
        // bits 0..0
        self.val.get_bits(0..=0)
    }

    /// reads the current register value and extract field `veng0` from it
    pub fn veng0_read(&mut self) -> u64 {
        Self::with_reg_val().veng0_extract()
    }

    /// inserts the given value `val` into the field `veng0`
    pub fn veng0_insert(&mut self, val: u64) -> &mut self {
        // bits 0..0
        self.val.set_bits(0..=0, val);
        self
    }

    /// reads the register, updates the `veng0` field, and writes the updated value
    pub fn veng0_write(&mut self, val: u64) {
        Self::with_reg_val().veng0_insert(val).write();
    }
}

impl Default for IchVmcrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> IchVmcrEl2 {
        IchVmcrEl2(0)
    }
}
