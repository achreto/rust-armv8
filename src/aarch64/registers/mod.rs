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

/**************************************************************************************************
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 * Generated on: 2022-08-22T16:35:53.044844
 * Version: Armv8.7-A-2020-09
 * Source: https://developer.arm.com/-/media/developer/products/architecture/armv8-a-architecture/2020-09/SysReg_xml_v87A-2020-09.tar.gz
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 *************************************************************************************************/

/*
 * ================================================================================================
 * A group mapping that does not have a known primary
 * ================================================================================================
 */

// Accelerator Data
mod accdata_el1;
pub use accdata_el1::AccdataEl1;

// Hypervisor Activity Monitors Fine-Grained Read Trap Register
mod hafgrtr_el2;
pub use hafgrtr_el2::HafgrtrEl2;

// Hypervisor Debug Fine-Grained Read Trap Register
mod hdfgrtr_el2;
pub use hdfgrtr_el2::HdfgrtrEl2;

// Hypervisor Debug Fine-Grained Write Trap Register
mod hdfgwtr_el2;
pub use hdfgwtr_el2::HdfgwtrEl2;

// Hypervisor Fine-Grained Instruction Trap Register
mod hfgitr_el2;
pub use hfgitr_el2::HfgitrEl2;

// Hypervisor Fine-Grained Read Trap Register
mod hfgrtr_el2;
pub use hfgrtr_el2::HfgrtrEl2;

// Hypervisor Fine-Grained Write Trap Register
mod hfgwtr_el2;
pub use hfgwtr_el2::HfgwtrEl2;

// Sampling Inverted Event Filter Register
mod pmsnevfr_el1;
pub use pmsnevfr_el1::PmsnevfrEl1;

/*
 * ================================================================================================
 * Activity Monitors registers
 * ================================================================================================
 */

// Activity Monitors Configuration Register
mod amcfgr_el0;
pub use amcfgr_el0::AmcfgrEl0;

// Activity Monitors Counter Group 1 Identification Register
mod amcg1idr_el0;
pub use amcg1idr_el0::Amcg1idrEl0;

// Activity Monitors Counter Group Configuration Register
mod amcgcr_el0;
pub use amcgcr_el0::AmcgcrEl0;

// Activity Monitors Count Enable Clear Register 0
mod amcntenclr0_el0;
pub use amcntenclr0_el0::Amcntenclr0El0;

// Activity Monitors Count Enable Clear Register 1
mod amcntenclr1_el0;
pub use amcntenclr1_el0::Amcntenclr1El0;

// Activity Monitors Count Enable Set Register 0
mod amcntenset0_el0;
pub use amcntenset0_el0::Amcntenset0El0;

// Activity Monitors Count Enable Set Register 1
mod amcntenset1_el0;
pub use amcntenset1_el0::Amcntenset1El0;

// Activity Monitors Control Register
mod amcr_el0;
pub use amcr_el0::AmcrEl0;

// Activity Monitors User Enable Register
mod amuserenr_el0;
pub use amuserenr_el0::AmuserenrEl0;

/*
 * ================================================================================================
 * Address translation instructions
 * ================================================================================================
 */

// Physical Address Register
mod par_el1;
pub use par_el1::ParEl1;

/*
 * ================================================================================================
 * Debug registers
 * ================================================================================================
 */

// Debug Authentication Status register
mod dbgauthstatus_el1;
pub use dbgauthstatus_el1::DbgauthstatusEl1;

// Debug CLAIM Tag Clear register
mod dbgclaimclr_el1;
pub use dbgclaimclr_el1::DbgclaimclrEl1;

// Debug CLAIM Tag Set register
mod dbgclaimset_el1;
pub use dbgclaimset_el1::DbgclaimsetEl1;

// Debug Data Transfer Register, half-duplex
mod dbgdtr_el0;
pub use dbgdtr_el0::DbgdtrEl0;

// Debug Data Transfer Register, Receive
mod dbgdtrrx_el0;
pub use dbgdtrrx_el0::DbgdtrrxEl0;

// Debug Data Transfer Register, Transmit
mod dbgdtrtx_el0;
pub use dbgdtrtx_el0::DbgdtrtxEl0;

// Debug Power Control Register
mod dbgprcr_el1;
pub use dbgprcr_el1::DbgprcrEl1;

// Debug Vector Catch Register
mod dbgvcr32_el2;
pub use dbgvcr32_el2::Dbgvcr32El2;

// Debug Link Register
mod dlr_el0;
pub use dlr_el0::DlrEl0;

// Debug Saved Program Status Register
mod dspsr_el0;
pub use dspsr_el0::DspsrEl0;

// Monitor DCC Interrupt Enable Register
mod mdccint_el1;
pub use mdccint_el1::MdccintEl1;

// Monitor DCC Status Register
mod mdccsr_el0;
pub use mdccsr_el0::MdccsrEl0;

// Monitor Debug ROM Address Register
mod mdrar_el1;
pub use mdrar_el1::MdrarEl1;

// Monitor Debug System Control Register
mod mdscr_el1;
pub use mdscr_el1::MdscrEl1;

// OS Double Lock Register
mod osdlr_el1;
pub use osdlr_el1::OsdlrEl1;

// OS Lock Data Transfer Register, Receive
mod osdtrrx_el1;
pub use osdtrrx_el1::OsdtrrxEl1;

// OS Lock Data Transfer Register, Transmit
mod osdtrtx_el1;
pub use osdtrtx_el1::OsdtrtxEl1;

// OS Lock Exception Catch Control Register
mod oseccr_el1;
pub use oseccr_el1::OseccrEl1;

// OS Lock Access Register
mod oslar_el1;
pub use oslar_el1::OslarEl1;

// OS Lock Status Register
mod oslsr_el1;
pub use oslsr_el1::OslsrEl1;

// Trace Filter Control Register (EL1)
mod trfcr_el1;
pub use trfcr_el1::TrfcrEl1;

// Trace Filter Control Register (EL2)
mod trfcr_el2;
pub use trfcr_el2::TrfcrEl2;

/*
 * ================================================================================================
 * Exception and fault handling registers
 * ================================================================================================
 */

// Auxiliary Fault Status Register 0 (EL1)
mod afsr0_el1;
pub use afsr0_el1::Afsr0El1;

// Auxiliary Fault Status Register 0 (EL2)
mod afsr0_el2;
pub use afsr0_el2::Afsr0El2;

// Auxiliary Fault Status Register 1 (EL1)
mod afsr1_el1;
pub use afsr1_el1::Afsr1El1;

// Auxiliary Fault Status Register 1 (EL2)
mod afsr1_el2;
pub use afsr1_el2::Afsr1El2;

// Exception Syndrome Register (EL1)
mod esr_el1;
pub use esr_el1::EsrEl1;

// Exception Syndrome Register (EL2)
mod esr_el2;
pub use esr_el2::EsrEl2;

// Exception Syndrome Register (EL3)
mod esr_el3;
pub use esr_el3::EsrEl3;

// Fault Address Register (EL1)
mod far_el1;
pub use far_el1::FarEl1;

// Fault Address Register (EL2)
mod far_el2;
pub use far_el2::FarEl2;

// Fault Address Register (EL3)
mod far_el3;
pub use far_el3::FarEl3;

// Hypervisor IPA Fault Address Register
mod hpfar_el2;
pub use hpfar_el2::HpfarEl2;

// Instruction Fault Status Register (EL2)
mod ifsr32_el2;
pub use ifsr32_el2::Ifsr32El2;

// Interrupt Status Register
mod isr_el1;
pub use isr_el1::IsrEl1;

// Vector Base Address Register (EL1)
mod vbar_el1;
pub use vbar_el1::VbarEl1;

// Vector Base Address Register (EL2)
mod vbar_el2;
pub use vbar_el2::VbarEl2;

/*
 * ================================================================================================
 * Floating-point registers
 * ================================================================================================
 */

// Floating-point Control Register
mod fpcr;
pub use fpcr::Fpcr;

// Floating-Point Exception Control register
mod fpexc32_el2;
pub use fpexc32_el2::Fpexc32El2;

// Floating-point Status Register
mod fpsr;
pub use fpsr::Fpsr;

// AArch32 Media and VFP Feature Register 0
mod mvfr0_el1;
pub use mvfr0_el1::Mvfr0El1;

// AArch32 Media and VFP Feature Register 1
mod mvfr1_el1;
pub use mvfr1_el1::Mvfr1El1;

// AArch32 Media and VFP Feature Register 2
mod mvfr2_el1;
pub use mvfr2_el1::Mvfr2El1;

/*
 * ================================================================================================
 * GIC control registers
 * ================================================================================================
 */

// Interrupt Controller Control Register (EL3)
mod icc_ctlr_el3;
pub use icc_ctlr_el3::IccCtlrEl3;

// Interrupt Controller System Register Enable register (EL3)
mod icc_sre_el3;
pub use icc_sre_el3::IccSreEl3;

/*
 * ================================================================================================
 * Generic System Control
 * ================================================================================================
 */

// Tag Control Register.
mod gcr_el1;
pub use gcr_el1::GcrEl1;

// Random Allocation Tag Seed Register.
mod rgsr_el1;
pub use rgsr_el1::RgsrEl1;

// Random Number
mod rndr;
pub use rndr::Rndr;

// Reseeded Random Number
mod rndrrs;
pub use rndrrs::Rndrrs;

// AArch32 Secure Debug Enable Register
mod sder32_el2;
pub use sder32_el2::Sder32El2;

// Tag Fault Status Register (EL1)
mod tfsr_el1;
pub use tfsr_el1::TfsrEl1;

// Tag Fault Status Register (EL2)
mod tfsr_el2;
pub use tfsr_el2::TfsrEl2;

// Tag Fault Status Register (EL3)
mod tfsr_el3;
pub use tfsr_el3::TfsrEl3;

// Tag Fault Status Register (EL0).
mod tfsre0_el1;
pub use tfsre0_el1::Tfsre0El1;

// Virtual Nested Control Register
mod vncr_el2;
pub use vncr_el2::VncrEl2;

// Virtualization Secure Translation Control Register
mod vstcr_el2;
pub use vstcr_el2::VstcrEl2;

// Virtualization Secure Translation Table Base Register
mod vsttbr_el2;
pub use vsttbr_el2::VsttbrEl2;

/*
 * ================================================================================================
 * Generic Timer registers
 * ================================================================================================
 */

// Counter-timer Frequency register
mod cntfrq_el0;
pub use cntfrq_el0::CntfrqEl0;

// Counter-timer Virtual Timer Control register (EL2)
mod cnthv_ctl_el2;
pub use cnthv_ctl_el2::CnthvCtlEl2;

// Counter-timer Virtual Timer CompareValue register (EL2)
mod cnthv_cval_el2;
pub use cnthv_cval_el2::CnthvCvalEl2;

// Counter-timer Virtual Timer TimerValue Register (EL2)
mod cnthv_tval_el2;
pub use cnthv_tval_el2::CnthvTvalEl2;

// Counter-timer Secure Virtual Timer Control register (EL2)
mod cnthvs_ctl_el2;
pub use cnthvs_ctl_el2::CnthvsCtlEl2;

// Counter-timer Secure Virtual Timer CompareValue register (EL2)
mod cnthvs_cval_el2;
pub use cnthvs_cval_el2::CnthvsCvalEl2;

// Counter-timer Secure Virtual Timer TimerValue register (EL2)
mod cnthvs_tval_el2;
pub use cnthvs_tval_el2::CnthvsTvalEl2;

// Counter-timer Kernel Control register
mod cntkctl_el1;
pub use cntkctl_el1::CntkctlEl1;

// Counter-timer Physical Timer Control register
mod cntp_ctl_el0;
pub use cntp_ctl_el0::CntpCtlEl0;

// Counter-timer Physical Timer CompareValue register
mod cntp_cval_el0;
pub use cntp_cval_el0::CntpCvalEl0;

// Counter-timer Physical Timer TimerValue register
mod cntp_tval_el0;
pub use cntp_tval_el0::CntpTvalEl0;

// Counter-timer Physical Count register
mod cntpct_el0;
pub use cntpct_el0::CntpctEl0;

// Counter-timer Self-Synchronized Physical Count register
mod cntpctss_el0;
pub use cntpctss_el0::CntpctssEl0;

// Counter-timer Physical Offset register
mod cntpoff_el2;
pub use cntpoff_el2::CntpoffEl2;

// Counter-timer Physical Secure Timer Control register
mod cntps_ctl_el1;
pub use cntps_ctl_el1::CntpsCtlEl1;

// Counter-timer Physical Secure Timer CompareValue register
mod cntps_cval_el1;
pub use cntps_cval_el1::CntpsCvalEl1;

// Counter-timer Physical Secure Timer TimerValue register
mod cntps_tval_el1;
pub use cntps_tval_el1::CntpsTvalEl1;

// Counter-timer Virtual Timer Control register
mod cntv_ctl_el0;
pub use cntv_ctl_el0::CntvCtlEl0;

// Counter-timer Virtual Timer CompareValue register
mod cntv_cval_el0;
pub use cntv_cval_el0::CntvCvalEl0;

// Counter-timer Virtual Timer TimerValue register
mod cntv_tval_el0;
pub use cntv_tval_el0::CntvTvalEl0;

// Counter-timer Virtual Count register
mod cntvct_el0;
pub use cntvct_el0::CntvctEl0;

// Counter-timer Self-Synchronized Virtual Count register
mod cntvctss_el0;
pub use cntvctss_el0::CntvctssEl0;

/*
 * ================================================================================================
 * IMPLEMENTATION DEFINED
 * ================================================================================================
 */

// Auxiliary Control Register (EL1)
mod actlr_el1;
pub use actlr_el1::ActlrEl1;

// Auxiliary ID Register
mod aidr_el1;
pub use aidr_el1::AidrEl1;

/*
 * ================================================================================================
 * Identification registers
 * ================================================================================================
 */

// Current Cache Size ID Register 2
mod ccsidr2_el1;
pub use ccsidr2_el1::Ccsidr2El1;

// Current Cache Size ID Register
mod ccsidr_el1;
pub use ccsidr_el1::CcsidrEl1;

// Cache Level ID Register
mod clidr_el1;
pub use clidr_el1::ClidrEl1;

// Cache Size Selection Register
mod csselr_el1;
pub use csselr_el1::CsselrEl1;

// Cache Type Register
mod ctr_el0;
pub use ctr_el0::CtrEl0;

// Data Cache Zero ID register
mod dczid_el0;
pub use dczid_el0::DczidEl0;

//  Multiple tag transfer ID register
mod gmid_el1;
pub use gmid_el1::GmidEl1;

// AArch64 Auxiliary Feature Register 0
mod id_aa64afr0_el1;
pub use id_aa64afr0_el1::IdAa64afr0El1;

// AArch64 Auxiliary Feature Register 1
mod id_aa64afr1_el1;
pub use id_aa64afr1_el1::IdAa64afr1El1;

// AArch64 Debug Feature Register 0
mod id_aa64dfr0_el1;
pub use id_aa64dfr0_el1::IdAa64dfr0El1;

// AArch64 Debug Feature Register 1
mod id_aa64dfr1_el1;
pub use id_aa64dfr1_el1::IdAa64dfr1El1;

// AArch64 Instruction Set Attribute Register 0
mod id_aa64isar0_el1;
pub use id_aa64isar0_el1::IdAa64isar0El1;

// AArch64 Instruction Set Attribute Register 1
mod id_aa64isar1_el1;
pub use id_aa64isar1_el1::IdAa64isar1El1;

// AArch64 Instruction Set Attribute Register 2
mod id_aa64isar2_el1;
pub use id_aa64isar2_el1::IdAa64isar2El1;

// AArch64 Memory Model Feature Register 0
mod id_aa64mmfr0_el1;
pub use id_aa64mmfr0_el1::IdAa64mmfr0El1;

// AArch64 Memory Model Feature Register 1
mod id_aa64mmfr1_el1;
pub use id_aa64mmfr1_el1::IdAa64mmfr1El1;

// AArch64 Memory Model Feature Register 2
mod id_aa64mmfr2_el1;
pub use id_aa64mmfr2_el1::IdAa64mmfr2El1;

// AArch64 Processor Feature Register 0
mod id_aa64pfr0_el1;
pub use id_aa64pfr0_el1::IdAa64pfr0El1;

// AArch64 Processor Feature Register 1
mod id_aa64pfr1_el1;
pub use id_aa64pfr1_el1::IdAa64pfr1El1;

// SVE Feature ID register 0
mod id_aa64zfr0_el1;
pub use id_aa64zfr0_el1::IdAa64zfr0El1;

// AArch32 Auxiliary Feature Register 0
mod id_afr0_el1;
pub use id_afr0_el1::IdAfr0El1;

// AArch32 Debug Feature Register 0
mod id_dfr0_el1;
pub use id_dfr0_el1::IdDfr0El1;

// Debug Feature Register 1
mod id_dfr1_el1;
pub use id_dfr1_el1::IdDfr1El1;

// AArch32 Instruction Set Attribute Register 0
mod id_isar0_el1;
pub use id_isar0_el1::IdIsar0El1;

// AArch32 Instruction Set Attribute Register 1
mod id_isar1_el1;
pub use id_isar1_el1::IdIsar1El1;

// AArch32 Instruction Set Attribute Register 2
mod id_isar2_el1;
pub use id_isar2_el1::IdIsar2El1;

// AArch32 Instruction Set Attribute Register 3
mod id_isar3_el1;
pub use id_isar3_el1::IdIsar3El1;

// AArch32 Instruction Set Attribute Register 4
mod id_isar4_el1;
pub use id_isar4_el1::IdIsar4El1;

// AArch32 Instruction Set Attribute Register 5
mod id_isar5_el1;
pub use id_isar5_el1::IdIsar5El1;

// AArch32 Instruction Set Attribute Register 6
mod id_isar6_el1;
pub use id_isar6_el1::IdIsar6El1;

// AArch32 Memory Model Feature Register 0
mod id_mmfr0_el1;
pub use id_mmfr0_el1::IdMmfr0El1;

// AArch32 Memory Model Feature Register 1
mod id_mmfr1_el1;
pub use id_mmfr1_el1::IdMmfr1El1;

// AArch32 Memory Model Feature Register 2
mod id_mmfr2_el1;
pub use id_mmfr2_el1::IdMmfr2El1;

// AArch32 Memory Model Feature Register 3
mod id_mmfr3_el1;
pub use id_mmfr3_el1::IdMmfr3El1;

// AArch32 Memory Model Feature Register 4
mod id_mmfr4_el1;
pub use id_mmfr4_el1::IdMmfr4El1;

// AArch32 Memory Model Feature Register 5
mod id_mmfr5_el1;
pub use id_mmfr5_el1::IdMmfr5El1;

// AArch32 Processor Feature Register 0
mod id_pfr0_el1;
pub use id_pfr0_el1::IdPfr0El1;

// AArch32 Processor Feature Register 1
mod id_pfr1_el1;
pub use id_pfr1_el1::IdPfr1El1;

// AArch32 Processor Feature Register 2
mod id_pfr2_el1;
pub use id_pfr2_el1::IdPfr2El1;

// Main ID Register
mod midr_el1;
pub use midr_el1::MidrEl1;

// MPAM ID Register (EL1)
mod mpamidr_el1;
pub use mpamidr_el1::MpamidrEl1;

// Multiprocessor Affinity Register
mod mpidr_el1;
pub use mpidr_el1::MpidrEl1;

// Revision ID Register
mod revidr_el1;
pub use revidr_el1::RevidrEl1;

/*
 * ================================================================================================
 * Memory Partitioning and Monitoring registers
 * ================================================================================================
 */

// MPAM0 Register (EL1)
mod mpam0_el1;
pub use mpam0_el1::Mpam0El1;

// MPAM1 Register (EL1)
mod mpam1_el1;
pub use mpam1_el1::Mpam1El1;

// MPAM2 Register (EL2)
mod mpam2_el2;
pub use mpam2_el2::Mpam2El2;

// MPAM3 Register (EL3)
mod mpam3_el3;
pub use mpam3_el3::Mpam3El3;

// MPAM Hypervisor Control Register (EL2)
mod mpamhcr_el2;
pub use mpamhcr_el2::MpamhcrEl2;

// MPAM Virtual PARTID Mapping Register 0
mod mpamvpm0_el2;
pub use mpamvpm0_el2::Mpamvpm0El2;

// MPAM Virtual PARTID Mapping Register 1
mod mpamvpm1_el2;
pub use mpamvpm1_el2::Mpamvpm1El2;

// MPAM Virtual PARTID Mapping Register 2
mod mpamvpm2_el2;
pub use mpamvpm2_el2::Mpamvpm2El2;

// MPAM Virtual PARTID Mapping Register 3
mod mpamvpm3_el2;
pub use mpamvpm3_el2::Mpamvpm3El2;

// MPAM Virtual PARTID Mapping Register 4
mod mpamvpm4_el2;
pub use mpamvpm4_el2::Mpamvpm4El2;

// MPAM Virtual PARTID Mapping Register 5
mod mpamvpm5_el2;
pub use mpamvpm5_el2::Mpamvpm5El2;

// MPAM Virtual PARTID Mapping Register 6
mod mpamvpm6_el2;
pub use mpamvpm6_el2::Mpamvpm6El2;

// MPAM Virtual PARTID Mapping Register 7
mod mpamvpm7_el2;
pub use mpamvpm7_el2::Mpamvpm7El2;

// MPAM Virtual Partition Mapping Valid Register
mod mpamvpmv_el2;
pub use mpamvpmv_el2::MpamvpmvEl2;

/*
 * ================================================================================================
 * Other system control registers
 * ================================================================================================
 */

// Architectural Feature Access Control Register
mod cpacr_el1;
pub use cpacr_el1::CpacrEl1;

// System Control Register (EL1)
mod sctlr_el1;
pub use sctlr_el1::SctlrEl1;

// System Control Register (EL3)
mod sctlr_el3;
pub use sctlr_el3::SctlrEl3;

// SVE Control Register for EL1
mod zcr_el1;
pub use zcr_el1::ZcrEl1;

// SVE Control Register for EL2
mod zcr_el2;
pub use zcr_el2::ZcrEl2;

// SVE Control Register for EL3
mod zcr_el3;
pub use zcr_el3::ZcrEl3;

/*
 * ================================================================================================
 * Performance Monitors registers
 * ================================================================================================
 */

// Performance Monitors Cycle Count Filter Register
mod pmccfiltr_el0;
pub use pmccfiltr_el0::PmccfiltrEl0;

// Performance Monitors Cycle Count Register
mod pmccntr_el0;
pub use pmccntr_el0::PmccntrEl0;

// Performance Monitors Common Event Identification register 0
mod pmceid0_el0;
pub use pmceid0_el0::Pmceid0El0;

// Performance Monitors Common Event Identification register 1
mod pmceid1_el0;
pub use pmceid1_el0::Pmceid1El0;

// Performance Monitors Count Enable Clear register
mod pmcntenclr_el0;
pub use pmcntenclr_el0::PmcntenclrEl0;

// Performance Monitors Count Enable Set register
mod pmcntenset_el0;
pub use pmcntenset_el0::PmcntensetEl0;

// Performance Monitors Control Register
mod pmcr_el0;
pub use pmcr_el0::PmcrEl0;

// Performance Monitors Interrupt Enable Clear register
mod pmintenclr_el1;
pub use pmintenclr_el1::PmintenclrEl1;

// Performance Monitors Interrupt Enable Set register
mod pmintenset_el1;
pub use pmintenset_el1::PmintensetEl1;

// Performance Monitors Machine Identification Register
mod pmmir_el1;
pub use pmmir_el1::PmmirEl1;

// Performance Monitors Overflow Flag Status Clear Register
mod pmovsclr_el0;
pub use pmovsclr_el0::PmovsclrEl0;

// Performance Monitors Overflow Flag Status Set register
mod pmovsset_el0;
pub use pmovsset_el0::PmovssetEl0;

// Performance Monitors Event Counter Selection Register
mod pmselr_el0;
pub use pmselr_el0::PmselrEl0;

// Performance Monitors Software Increment register
mod pmswinc_el0;
pub use pmswinc_el0::PmswincEl0;

// Performance Monitors User Enable Register
mod pmuserenr_el0;
pub use pmuserenr_el0::PmuserenrEl0;

// Performance Monitors Selected Event Count Register
mod pmxevcntr_el0;
pub use pmxevcntr_el0::PmxevcntrEl0;

// Performance Monitors Selected Event Type Register
mod pmxevtyper_el0;
pub use pmxevtyper_el0::PmxevtyperEl0;

/*
 * ================================================================================================
 * Pointer authentication
 * ================================================================================================
 */

// Pointer Authentication Key A for Data (bits[127:64])
mod apdakeyhi_el1;
pub use apdakeyhi_el1::ApdakeyhiEl1;

// Pointer Authentication Key A for Data (bits[63:0])
mod apdakeylo_el1;
pub use apdakeylo_el1::ApdakeyloEl1;

// Pointer Authentication Key B for Data (bits[127:64])
mod apdbkeyhi_el1;
pub use apdbkeyhi_el1::ApdbkeyhiEl1;

// Pointer Authentication Key B for Data (bits[63:0])
mod apdbkeylo_el1;
pub use apdbkeylo_el1::ApdbkeyloEl1;

// Pointer Authentication Key A for Code (bits[127:64])
mod apgakeyhi_el1;
pub use apgakeyhi_el1::ApgakeyhiEl1;

// Pointer Authentication Key A for Code (bits[63:0])
mod apgakeylo_el1;
pub use apgakeylo_el1::ApgakeyloEl1;

// Pointer Authentication Key A for Instruction (bits[127:64])
mod apiakeyhi_el1;
pub use apiakeyhi_el1::ApiakeyhiEl1;

// Pointer Authentication Key A for Instruction (bits[63:0])
mod apiakeylo_el1;
pub use apiakeylo_el1::ApiakeyloEl1;

// Pointer Authentication Key B for Instruction (bits[127:64])
mod apibkeyhi_el1;
pub use apibkeyhi_el1::ApibkeyhiEl1;

// Pointer Authentication Key B for Instruction (bits[63:0])
mod apibkeylo_el1;
pub use apibkeylo_el1::ApibkeyloEl1;

/*
 * ================================================================================================
 * Process state registers
 * ================================================================================================
 */

// Current Exception Level
mod currentel;
pub use currentel::Currentel;

// Interrupt Mask Bits
mod daif;
pub use daif::Daif;

// Data Independent Timing
mod dit;
pub use dit::Dit;

// Condition Flags
mod nzcv;
pub use nzcv::Nzcv;

// Privileged Access Never
mod pan;
pub use pan::Pan;

// Stack Pointer Select
mod spsel;
pub use spsel::Spsel;

// Speculative Store Bypass Safe
mod ssbs;
pub use ssbs::Ssbs;

// Tag Check Override
mod tco;
pub use tco::Tco;

// User Access Override
mod uao;
pub use uao::Uao;

/*
 * ================================================================================================
 * RAS registers
 * ================================================================================================
 */

// Deferred Interrupt Status Register
mod disr_el1;
pub use disr_el1::DisrEl1;

// Error Record ID Register
mod erridr_el1;
pub use erridr_el1::ErridrEl1;

// Error Record Select Register
mod errselr_el1;
pub use errselr_el1::ErrselrEl1;

// Selected Error Record Address Register
mod erxaddr_el1;
pub use erxaddr_el1::ErxaddrEl1;

// Selected Error Record Control Register
mod erxctlr_el1;
pub use erxctlr_el1::ErxctlrEl1;

// Selected Error Record Feature Register
mod erxfr_el1;
pub use erxfr_el1::ErxfrEl1;

// Selected Error Record Miscellaneous Register 0
mod erxmisc0_el1;
pub use erxmisc0_el1::Erxmisc0El1;

// Selected Error Record Miscellaneous Register 1
mod erxmisc1_el1;
pub use erxmisc1_el1::Erxmisc1El1;

// Selected Error Record Miscellaneous Register 2
mod erxmisc2_el1;
pub use erxmisc2_el1::Erxmisc2El1;

// Selected Error Record Miscellaneous Register 3
mod erxmisc3_el1;
pub use erxmisc3_el1::Erxmisc3El1;

// Selected Pseudo-fault Generation Countdown register
mod erxpfgcdn_el1;
pub use erxpfgcdn_el1::ErxpfgcdnEl1;

// Selected Pseudo-fault Generation Control register
mod erxpfgctl_el1;
pub use erxpfgctl_el1::ErxpfgctlEl1;

// Selected Pseudo-fault Generation Feature register
mod erxpfgf_el1;
pub use erxpfgf_el1::ErxpfgfEl1;

// Selected Error Record Primary Status Register
mod erxstatus_el1;
pub use erxstatus_el1::ErxstatusEl1;

// Virtual Deferred Interrupt Status Register
mod vdisr_el2;
pub use vdisr_el2::VdisrEl2;

// Virtual SError Exception Syndrome Register
mod vsesr_el2;
pub use vsesr_el2::VsesrEl2;

/*
 * ================================================================================================
 * Reset management registers
 * ================================================================================================
 */

// Reset Management Register (EL1)
mod rmr_el1;
pub use rmr_el1::RmrEl1;

// Reset Management Register (EL2)
mod rmr_el2;
pub use rmr_el2::RmrEl2;

// Reset Management Register (EL3)
mod rmr_el3;
pub use rmr_el3::RmrEl3;

// Reset Vector Base Address Register (if EL2 and EL3 not implemented)
mod rvbar_el1;
pub use rvbar_el1::RvbarEl1;

// Reset Vector Base Address Register (if EL3 not implemented)
mod rvbar_el2;
pub use rvbar_el2::RvbarEl2;

// Reset Vector Base Address Register (if EL3 implemented)
mod rvbar_el3;
pub use rvbar_el3::RvbarEl3;

/*
 * ================================================================================================
 * Security registers
 * ================================================================================================
 */

// Auxiliary Control Register (EL3)
mod actlr_el3;
pub use actlr_el3::ActlrEl3;

// Architectural Feature Trap Register (EL3)
mod cptr_el3;
pub use cptr_el3::CptrEl3;

// Monitor Debug Configuration Register (EL3)
mod mdcr_el3;
pub use mdcr_el3::MdcrEl3;

// Secure Configuration Register
mod scr_el3;
pub use scr_el3::ScrEl3;

// AArch32 Secure Debug Enable Register
mod sder32_el3;
pub use sder32_el3::Sder32El3;

/*
 * ================================================================================================
 * Special-purpose registers
 * ================================================================================================
 */

// Exception Link Register (EL1)
mod elr_el1;
pub use elr_el1::ElrEl1;

// Exception Link Register (EL2)
mod elr_el2;
pub use elr_el2::ElrEl2;

// Exception Link Register (EL3)
mod elr_el3;
pub use elr_el3::ElrEl3;

// Stack Pointer (EL0)
mod sp_el0;
pub use sp_el0::SpEl0;

// Stack Pointer (EL1)
mod sp_el1;
pub use sp_el1::SpEl1;

// Stack Pointer (EL2)
mod sp_el2;
pub use sp_el2::SpEl2;

// Saved Program Status Register (Abort mode)
mod spsr_abt;
pub use spsr_abt::SpsrAbt;

// Saved Program Status Register (EL1)
mod spsr_el1;
pub use spsr_el1::SpsrEl1;

// Saved Program Status Register (EL2)
mod spsr_el2;
pub use spsr_el2::SpsrEl2;

// Saved Program Status Register (EL3)
mod spsr_el3;
pub use spsr_el3::SpsrEl3;

// Saved Program Status Register (FIQ mode)
mod spsr_fiq;
pub use spsr_fiq::SpsrFiq;

// Saved Program Status Register (IRQ mode)
mod spsr_irq;
pub use spsr_irq::SpsrIrq;

// Saved Program Status Register (Undefined mode)
mod spsr_und;
pub use spsr_und::SpsrUnd;

/*
 * ================================================================================================
 * Statistical Profiling Extension registers
 * ================================================================================================
 */

// Profiling Buffer ID Register
mod pmbidr_el1;
pub use pmbidr_el1::PmbidrEl1;

// Profiling Buffer Limit Address Register
mod pmblimitr_el1;
pub use pmblimitr_el1::PmblimitrEl1;

// Profiling Buffer Write Pointer Register
mod pmbptr_el1;
pub use pmbptr_el1::PmbptrEl1;

// Profiling Buffer Status/syndrome Register
mod pmbsr_el1;
pub use pmbsr_el1::PmbsrEl1;

// Statistical Profiling Control Register (EL1)
mod pmscr_el1;
pub use pmscr_el1::PmscrEl1;

// Statistical Profiling Control Register (EL2)
mod pmscr_el2;
pub use pmscr_el2::PmscrEl2;

// Sampling Event Filter Register
mod pmsevfr_el1;
pub use pmsevfr_el1::PmsevfrEl1;

// Sampling Filter Control Register
mod pmsfcr_el1;
pub use pmsfcr_el1::PmsfcrEl1;

// Sampling Interval Counter Register
mod pmsicr_el1;
pub use pmsicr_el1::PmsicrEl1;

// Sampling Profiling ID Register
mod pmsidr_el1;
pub use pmsidr_el1::PmsidrEl1;

// Sampling Interval Reload Register
mod pmsirr_el1;
pub use pmsirr_el1::PmsirrEl1;

// Sampling Latency Filter Register
mod pmslatfr_el1;
pub use pmslatfr_el1::PmslatfrEl1;

/*
 * ================================================================================================
 * Thread and process ID registers
 * ================================================================================================
 */

// EL0 Read/Write Software Context Number
mod scxtnum_el0;
pub use scxtnum_el0::ScxtnumEl0;

// EL1 Read/Write Software Context Number
mod scxtnum_el1;
pub use scxtnum_el1::ScxtnumEl1;

// EL2 Read/Write Software Context Number
mod scxtnum_el2;
pub use scxtnum_el2::ScxtnumEl2;

// EL3 Read/Write Software Context Number
mod scxtnum_el3;
pub use scxtnum_el3::ScxtnumEl3;

// EL0 Read/Write Software Thread ID Register
mod tpidr_el0;
pub use tpidr_el0::TpidrEl0;

// EL1 Software Thread ID Register
mod tpidr_el1;
pub use tpidr_el1::TpidrEl1;

// EL2 Software Thread ID Register
mod tpidr_el2;
pub use tpidr_el2::TpidrEl2;

// EL3 Software Thread ID Register
mod tpidr_el3;
pub use tpidr_el3::TpidrEl3;

// EL0 Read-Only Software Thread ID Register
mod tpidrro_el0;
pub use tpidrro_el0::TpidrroEl0;

/*
 * ================================================================================================
 * Virtual memory control registers
 * ================================================================================================
 */

// Auxiliary Memory Attribute Indirection Register (EL1)
mod amair_el1;
pub use amair_el1::AmairEl1;

// Auxiliary Memory Attribute Indirection Register (EL2)
mod amair_el2;
pub use amair_el2::AmairEl2;

// Context ID Register (EL1)
mod contextidr_el1;
pub use contextidr_el1::ContextidrEl1;

// Context ID Register (EL2)
mod contextidr_el2;
pub use contextidr_el2::ContextidrEl2;

// Domain Access Control Register
mod dacr32_el2;
pub use dacr32_el2::Dacr32El2;

// LORegion Control (EL1)
mod lorc_el1;
pub use lorc_el1::LorcEl1;

// LORegion End Address (EL1)
mod lorea_el1;
pub use lorea_el1::LoreaEl1;

// LORegionID (EL1)
mod lorid_el1;
pub use lorid_el1::LoridEl1;

// LORegion Number (EL1)
mod lorn_el1;
pub use lorn_el1::LornEl1;

// LORegion Start Address (EL1)
mod lorsa_el1;
pub use lorsa_el1::LorsaEl1;

// Memory Attribute Indirection Register (EL1)
mod mair_el1;
pub use mair_el1::MairEl1;

// Memory Attribute Indirection Register (EL2)
mod mair_el2;
pub use mair_el2::MairEl2;

// Memory Attribute Indirection Register (EL3)
mod mair_el3;
pub use mair_el3::MairEl3;

// Translation Control Register (EL1)
mod tcr_el1;
pub use tcr_el1::TcrEl1;

// Translation Control Register (EL2)
mod tcr_el2;
pub use tcr_el2::TcrEl2;

// Translation Control Register (EL3)
mod tcr_el3;
pub use tcr_el3::TcrEl3;

// Translation Table Base Register 0 (EL1)
mod ttbr0_el1;
pub use ttbr0_el1::Ttbr0El1;

// Translation Table Base Register 0 (EL2)
// mod ttbr0_el2;
// pub use ttbr0_el2::Ttbr0El2;

// Translation Table Base Register 0 (EL3)
mod ttbr0_el3;
pub use ttbr0_el3::Ttbr0El3;

// Translation Table Base Register 1 (EL1)
mod ttbr1_el1;
pub use ttbr1_el1::Ttbr1El1;

// Translation Table Base Register 1 (EL2)
mod ttbr1_el2;
pub use ttbr1_el2::Ttbr1El2;

// Virtualization Translation Control Register
mod vtcr_el2;
pub use vtcr_el2::VtcrEl2;

// Virtualization Translation Table Base Register
// mod vttbr_el2;
// pub use vttbr_el2::VttbrEl2;

/*
 * ================================================================================================
 * Virtualization registers
 * ================================================================================================
 */

// Auxiliary Control Register (EL2)
mod actlr_el2;
pub use actlr_el2::ActlrEl2;

// Auxiliary Fault Status Register 0 (EL3)
mod afsr0_el3;
pub use afsr0_el3::Afsr0El3;

// Auxiliary Fault Status Register 1 (EL3)
mod afsr1_el3;
pub use afsr1_el3::Afsr1El3;

// Auxiliary Memory Attribute Indirection Register (EL3)
mod amair_el3;
pub use amair_el3::AmairEl3;

// Counter-timer Hypervisor Control register
mod cnthctl_el2;
pub use cnthctl_el2::CnthctlEl2;

// Counter-timer Hypervisor Physical Timer Control register
mod cnthp_ctl_el2;
pub use cnthp_ctl_el2::CnthpCtlEl2;

// Counter-timer Physical Timer CompareValue register (EL2)
mod cnthp_cval_el2;
pub use cnthp_cval_el2::CnthpCvalEl2;

// Counter-timer Physical Timer TimerValue register (EL2)
mod cnthp_tval_el2;
pub use cnthp_tval_el2::CnthpTvalEl2;

// Counter-timer Secure Physical Timer Control register (EL2)
mod cnthps_ctl_el2;
pub use cnthps_ctl_el2::CnthpsCtlEl2;

// Counter-timer Secure Physical Timer CompareValue register (EL2)
mod cnthps_cval_el2;
pub use cnthps_cval_el2::CnthpsCvalEl2;

// Counter-timer Secure Physical Timer TimerValue register (EL2)
mod cnthps_tval_el2;
pub use cnthps_tval_el2::CnthpsTvalEl2;

// Counter-timer Virtual Offset register
mod cntvoff_el2;
pub use cntvoff_el2::CntvoffEl2;

// Architectural Feature Trap Register (EL2)
mod cptr_el2;
pub use cptr_el2::CptrEl2;

// Hypervisor Auxiliary Control Register
mod hacr_el2;
pub use hacr_el2::HacrEl2;

// Hypervisor Configuration Register
mod hcr_el2;
pub use hcr_el2::HcrEl2;

// Extended Hypervisor Configuration Register
mod hcrx_el2;
pub use hcrx_el2::HcrxEl2;

// Hypervisor System Trap Register
mod hstr_el2;
pub use hstr_el2::HstrEl2;

// Interrupt Controller Alias Software Generated Interrupt Group 1 Register
mod icc_asgi1r_el1;
pub use icc_asgi1r_el1::IccAsgi1rEl1;

// Interrupt Controller Binary Point Register 0
mod icc_bpr0_el1;
pub use icc_bpr0_el1::IccBpr0El1;

// Interrupt Controller Binary Point Register 1
mod icc_bpr1_el1;
pub use icc_bpr1_el1::IccBpr1El1;

// Interrupt Controller Control Register (EL1)
mod icc_ctlr_el1;
pub use icc_ctlr_el1::IccCtlrEl1;

// Interrupt Controller Deactivate Interrupt Register
mod icc_dir_el1;
pub use icc_dir_el1::IccDirEl1;

// Interrupt Controller End Of Interrupt Register 0
mod icc_eoir0_el1;
pub use icc_eoir0_el1::IccEoir0El1;

// Interrupt Controller End Of Interrupt Register 1
mod icc_eoir1_el1;
pub use icc_eoir1_el1::IccEoir1El1;

// Interrupt Controller Highest Priority Pending Interrupt Register 0
mod icc_hppir0_el1;
pub use icc_hppir0_el1::IccHppir0El1;

// Interrupt Controller Highest Priority Pending Interrupt Register 1
mod icc_hppir1_el1;
pub use icc_hppir1_el1::IccHppir1El1;

// Interrupt Controller Interrupt Acknowledge Register 0
mod icc_iar0_el1;
pub use icc_iar0_el1::IccIar0El1;

// Interrupt Controller Interrupt Acknowledge Register 1
mod icc_iar1_el1;
pub use icc_iar1_el1::IccIar1El1;

// Interrupt Controller Interrupt Group 0 Enable register
mod icc_igrpen0_el1;
pub use icc_igrpen0_el1::IccIgrpen0El1;

// Interrupt Controller Interrupt Group 1 Enable register
mod icc_igrpen1_el1;
pub use icc_igrpen1_el1::IccIgrpen1El1;

// Interrupt Controller Interrupt Group 1 Enable register (EL3)
mod icc_igrpen1_el3;
pub use icc_igrpen1_el3::IccIgrpen1El3;

// Interrupt Controller Interrupt Priority Mask Register
mod icc_pmr_el1;
pub use icc_pmr_el1::IccPmrEl1;

// Interrupt Controller Running Priority Register
mod icc_rpr_el1;
pub use icc_rpr_el1::IccRprEl1;

// Interrupt Controller Software Generated Interrupt Group 0 Register
mod icc_sgi0r_el1;
pub use icc_sgi0r_el1::IccSgi0rEl1;

// Interrupt Controller Software Generated Interrupt Group 1 Register
mod icc_sgi1r_el1;
pub use icc_sgi1r_el1::IccSgi1rEl1;

// Interrupt Controller System Register Enable register (EL1)
mod icc_sre_el1;
pub use icc_sre_el1::IccSreEl1;

// Interrupt Controller System Register Enable register (EL2)
mod icc_sre_el2;
pub use icc_sre_el2::IccSreEl2;

// Interrupt Controller End of Interrupt Status Register
mod ich_eisr_el2;
pub use ich_eisr_el2::IchEisrEl2;

// Interrupt Controller Empty List Register Status Register
mod ich_elrsr_el2;
pub use ich_elrsr_el2::IchElrsrEl2;

// Interrupt Controller Hyp Control Register
mod ich_hcr_el2;
pub use ich_hcr_el2::IchHcrEl2;

// Interrupt Controller Maintenance Interrupt State Register
mod ich_misr_el2;
pub use ich_misr_el2::IchMisrEl2;

// Interrupt Controller Virtual Machine Control Register
mod ich_vmcr_el2;
pub use ich_vmcr_el2::IchVmcrEl2;

// Interrupt Controller VGIC Type Register
mod ich_vtr_el2;
pub use ich_vtr_el2::IchVtrEl2;

// Interrupt Controller Virtual Binary Point Register 0
mod icv_bpr0_el1;
pub use icv_bpr0_el1::IcvBpr0El1;

// Interrupt Controller Virtual Binary Point Register 1
mod icv_bpr1_el1;
pub use icv_bpr1_el1::IcvBpr1El1;

// Interrupt Controller Virtual Control Register
mod icv_ctlr_el1;
pub use icv_ctlr_el1::IcvCtlrEl1;

// Interrupt Controller Deactivate Virtual Interrupt Register
mod icv_dir_el1;
pub use icv_dir_el1::IcvDirEl1;

// Interrupt Controller Virtual End Of Interrupt Register 0
mod icv_eoir0_el1;
pub use icv_eoir0_el1::IcvEoir0El1;

// Interrupt Controller Virtual End Of Interrupt Register 1
mod icv_eoir1_el1;
pub use icv_eoir1_el1::IcvEoir1El1;

// Interrupt Controller Virtual Highest Priority Pending Interrupt Register 0
mod icv_hppir0_el1;
pub use icv_hppir0_el1::IcvHppir0El1;

// Interrupt Controller Virtual Highest Priority Pending Interrupt Register 1
mod icv_hppir1_el1;
pub use icv_hppir1_el1::IcvHppir1El1;

// Interrupt Controller Virtual Interrupt Acknowledge Register 0
mod icv_iar0_el1;
pub use icv_iar0_el1::IcvIar0El1;

// Interrupt Controller Virtual Interrupt Acknowledge Register 1
mod icv_iar1_el1;
pub use icv_iar1_el1::IcvIar1El1;

// Interrupt Controller Virtual Interrupt Group 0 Enable register
mod icv_igrpen0_el1;
pub use icv_igrpen0_el1::IcvIgrpen0El1;

// Interrupt Controller Virtual Interrupt Group 1 Enable register
mod icv_igrpen1_el1;
pub use icv_igrpen1_el1::IcvIgrpen1El1;

// Interrupt Controller Virtual Interrupt Priority Mask Register
mod icv_pmr_el1;
pub use icv_pmr_el1::IcvPmrEl1;

// Interrupt Controller Virtual Running Priority Register
mod icv_rpr_el1;
pub use icv_rpr_el1::IcvRprEl1;

// Monitor Debug Configuration Register (EL2)
mod mdcr_el2;
pub use mdcr_el2::MdcrEl2;

// System Control Register (EL2)
mod sctlr_el2;
pub use sctlr_el2::SctlrEl2;

// Vector Base Address Register (EL3)
mod vbar_el3;
pub use vbar_el3::VbarEl3;

// Virtualization Multiprocessor ID Register
mod vmpidr_el2;
pub use vmpidr_el2::VmpidrEl2;

// Virtualization Processor ID Register
mod vpidr_el2;
pub use vpidr_el2::VpidrEl2;

// end of file
