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
 * Generated on: 2022-08-22T15:51:28.520363
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
 * Register:    Interrupt Controller VGIC Type Register (ich_vtr_el2)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Reports supported GIC virtualization features.
 * File:        AArch64-ich_vtr_el2.xml
 */

/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */

/// struct holding a copy of the Interrupt Controller VGIC Type Register value in memory
pub struct IchVtrEl2(u64);

/// struct implementation for accessing the fields of register ich_vtr_el2
impl IchVtrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn new() -> IchVtrEl2 {
        Self::default()
    }

    /// collects the modifications of IchVtrEl2 and creates new object
    #[inline(always)]
    pub fn collect(&self) -> IchVtrEl2 {
        IchVtrEl2(self.0)
    }

    /// inserts field val into current value
    #[inline(always)]
    pub fn with_reg_val() -> IchVtrEl2 {
        let curval = Self::reg_rawrd() & 0xfffc001f;
        IchVtrEl2(curval)
    }

    /// reading the Interrupt Controller VGIC Type Register (ich_vtr_el2) register
    #[inline(always)]
    fn reg_rawrd() -> u64 {
        let mut regval: u64;
        unsafe {
            // MRS <Xt>, ICH_VTR_EL2
            llvm_asm!("mrs $0, ich_vtr_el2" : "=r"(regval));
        }
        return regval;
    }

    // register is not writable. not emitting write accessor

    /// updates the stored value with the current register value
    #[inline(always)]
    pub fn read(&mut self) -> &mut self {
        self.val = Self::reg_rawrd() & 0xfffc001f;
        self
    }

    // no write() method as it is read only

    // sets the value of the struct
    //pub fn set(&mut self, newval: u64) {
    //    self.val = newval & 4294705183;
    //}

    /// gets the value of the struct
    pub fn get_raw(&self) -> u64 {
        self.0
    }

    /*
     * Field: pribits
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn pribits_extract(&self) -> u64 {
        // bits 29..31
        self.val.get_bits(29..=31)
    }

    /// reads the current register value and extract field `pribits` from it
    pub fn pribits_read(&mut self) -> u64 {
        Self::with_reg_val().pribits_extract()
    }
    // no insert() method for field pribits
    /*
     * Field: prebits
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn prebits_extract(&self) -> u64 {
        // bits 26..28
        self.val.get_bits(26..=28)
    }

    /// reads the current register value and extract field `prebits` from it
    pub fn prebits_read(&mut self) -> u64 {
        Self::with_reg_val().prebits_extract()
    }
    // no insert() method for field prebits
    /*
     * Field: idbits
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn idbits_extract(&self) -> u64 {
        // bits 23..25
        self.val.get_bits(23..=25)
    }

    /// reads the current register value and extract field `idbits` from it
    pub fn idbits_read(&mut self) -> u64 {
        Self::with_reg_val().idbits_extract()
    }
    // no insert() method for field idbits
    /*
     * Field: seis
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn seis_extract(&self) -> u64 {
        // bits 22..22
        self.val.get_bits(22..=22)
    }

    /// reads the current register value and extract field `seis` from it
    pub fn seis_read(&mut self) -> u64 {
        Self::with_reg_val().seis_extract()
    }
    // no insert() method for field seis
    /*
     * Field: a3v
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn a3v_extract(&self) -> u64 {
        // bits 21..21
        self.val.get_bits(21..=21)
    }

    /// reads the current register value and extract field `a3v` from it
    pub fn a3v_read(&mut self) -> u64 {
        Self::with_reg_val().a3v_extract()
    }
    // no insert() method for field a3v
    /*
     * Field: nv4
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn nv4_extract(&self) -> u64 {
        // bits 20..20
        self.val.get_bits(20..=20)
    }

    /// reads the current register value and extract field `nv4` from it
    pub fn nv4_read(&mut self) -> u64 {
        Self::with_reg_val().nv4_extract()
    }
    // no insert() method for field nv4
    /*
     * Field: tds
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn tds_extract(&self) -> u64 {
        // bits 19..19
        self.val.get_bits(19..=19)
    }

    /// reads the current register value and extract field `tds` from it
    pub fn tds_read(&mut self) -> u64 {
        Self::with_reg_val().tds_extract()
    }
    // no insert() method for field tds
    /*
     * Field: dvim
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn dvim_extract(&self) -> u64 {
        // bits 18..18
        self.val.get_bits(18..=18)
    }

    /// reads the current register value and extract field `dvim` from it
    pub fn dvim_read(&mut self) -> u64 {
        Self::with_reg_val().dvim_extract()
    }
    // no insert() method for field dvim
    /*
     * Field: listregs
     * --------------------------------------------------------------------------------------------
     */

    /// extracts field val from current value
    pub fn listregs_extract(&self) -> u64 {
        // bits 0..4
        self.val.get_bits(0..=4)
    }

    /// reads the current register value and extract field `listregs` from it
    pub fn listregs_read(&mut self) -> u64 {
        Self::with_reg_val().listregs_extract()
    }
    // no insert() method for field listregs
}

impl Default for IchVtrEl2 {
    /// creates a new default value
    #[inline(always)]
    pub fn default() -> IchVtrEl2 {
        IchVtrEl2(0)
    }
}
