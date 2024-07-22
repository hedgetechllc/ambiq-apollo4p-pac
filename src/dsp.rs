#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    mutex0: Mutex0,
    mutex1: Mutex1,
    mutex2: Mutex2,
    mutex3: Mutex3,
    mutex4: Mutex4,
    mutex5: Mutex5,
    mutex6: Mutex6,
    mutex7: Mutex7,
    _reserved8: [u8; 0x20],
    cpumbintset: Cpumbintset,
    cpumbintclr: Cpumbintclr,
    cpumbintstat: Cpumbintstat,
    cpucpumbdata: Cpucpumbdata,
    dsp0cpumbdata: Dsp0cpumbdata,
    dsp1cpumbdata: Dsp1cpumbdata,
    _reserved14: [u8; 0x08],
    dsp0mbintset: Dsp0mbintset,
    dsp0mbintclr: Dsp0mbintclr,
    dsp0mbintstat: Dsp0mbintstat,
    cpudsp0mbdata: Cpudsp0mbdata,
    dsp0dsp0mbdata: Dsp0dsp0mbdata,
    dsp1dsp0mbdata: Dsp1dsp0mbdata,
    _reserved20: [u8; 0x08],
    dsp1mbintset: Dsp1mbintset,
    dsp1mbintclr: Dsp1mbintclr,
    dsp1mbintstat: Dsp1mbintstat,
    cpudsp1mbdata: Cpudsp1mbdata,
    dsp0dsp1mbdata: Dsp0dsp1mbdata,
    dsp1dsp1mbdata: Dsp1dsp1mbdata,
    _reserved26: [u8; 0x28],
    dsp0control: Dsp0control,
    dsp0resetvec: Dsp0resetvec,
    dsp0irqmask: Dsp0irqmask,
    dsp0wakemask: Dsp0wakemask,
    dsp0rawirqstat31to0: Dsp0rawirqstat31to0,
    dsp0rawirqstat63to32: Dsp0rawirqstat63to32,
    dsp0rawirqstat95to64: Dsp0rawirqstat95to64,
    _reserved33: [u8; 0x04],
    dsp0l2lvlint: Dsp0l2lvlint,
    dsp0l3lvlint: Dsp0l3lvlint,
    dsp0l4lvlint: Dsp0l4lvlint,
    dsp0l5lvlint: Dsp0l5lvlint,
    dsp0idmatrigctl: Dsp0idmatrigctl,
    _reserved38: [u8; 0x0c],
    dsp0intormask31to0a: Dsp0intormask31to0a,
    dsp0intormask63to32a: Dsp0intormask63to32a,
    dsp0intormask95to64a: Dsp0intormask95to64a,
    _reserved41: [u8; 0x04],
    dsp0intormask31to0b: Dsp0intormask31to0b,
    dsp0intormask63to32b: Dsp0intormask63to32b,
    dsp0intormask95to64b: Dsp0intormask95to64b,
    _reserved44: [u8; 0x04],
    dsp0intenirq31to0: Dsp0intenirq31to0,
    dsp0intenirq63to32: Dsp0intenirq63to32,
    dsp0intenirq95to64: Dsp0intenirq95to64,
    _reserved47: [u8; 0x94],
    dsp1control: Dsp1control,
    dsp1resetvec: Dsp1resetvec,
    dsp1irqmask: Dsp1irqmask,
    dsp1wakemask: Dsp1wakemask,
    dsp1rawirqstat31to0: Dsp1rawirqstat31to0,
    dsp1rawirqstat63to32: Dsp1rawirqstat63to32,
    dsp1rawirqstat95to64: Dsp1rawirqstat95to64,
    _reserved54: [u8; 0x04],
    dsp1l2lvlint: Dsp1l2lvlint,
    dsp1l3lvlint: Dsp1l3lvlint,
    dsp1l4lvlint: Dsp1l4lvlint,
    dsp1l5lvlint: Dsp1l5lvlint,
    dsp1idmatrigctl: Dsp1idmatrigctl,
    _reserved59: [u8; 0x0c],
    dsp1intormask31to0a: Dsp1intormask31to0a,
    dsp1intormask63to32a: Dsp1intormask63to32a,
    dsp1intormask95to64a: Dsp1intormask95to64a,
    _reserved62: [u8; 0x04],
    dsp1intormask31to0b: Dsp1intormask31to0b,
    dsp1intormask63to32b: Dsp1intormask63to32b,
    dsp1intormask95to64b: Dsp1intormask95to64b,
    _reserved65: [u8; 0x04],
    dsp1intenirq31to0: Dsp1intenirq31to0,
    dsp1intenirq63to32: Dsp1intenirq63to32,
    dsp1intenirq95to64: Dsp1intenirq95to64,
}
impl RegisterBlock {
    #[doc = "0x40 - MUTEX 0"]
    #[inline(always)]
    pub const fn mutex0(&self) -> &Mutex0 {
        &self.mutex0
    }
    #[doc = "0x44 - MUTEX 1"]
    #[inline(always)]
    pub const fn mutex1(&self) -> &Mutex1 {
        &self.mutex1
    }
    #[doc = "0x48 - MUTEX 2"]
    #[inline(always)]
    pub const fn mutex2(&self) -> &Mutex2 {
        &self.mutex2
    }
    #[doc = "0x4c - MUTEX 3"]
    #[inline(always)]
    pub const fn mutex3(&self) -> &Mutex3 {
        &self.mutex3
    }
    #[doc = "0x50 - MUTEX 4"]
    #[inline(always)]
    pub const fn mutex4(&self) -> &Mutex4 {
        &self.mutex4
    }
    #[doc = "0x54 - MUTEX 5"]
    #[inline(always)]
    pub const fn mutex5(&self) -> &Mutex5 {
        &self.mutex5
    }
    #[doc = "0x58 - MUTEX 6"]
    #[inline(always)]
    pub const fn mutex6(&self) -> &Mutex6 {
        &self.mutex6
    }
    #[doc = "0x5c - MUTEX 7"]
    #[inline(always)]
    pub const fn mutex7(&self) -> &Mutex7 {
        &self.mutex7
    }
    #[doc = "0x80 - CPU Mailbox Interrupt Set"]
    #[inline(always)]
    pub const fn cpumbintset(&self) -> &Cpumbintset {
        &self.cpumbintset
    }
    #[doc = "0x84 - CPU Mailbox Interrupt Clear"]
    #[inline(always)]
    pub const fn cpumbintclr(&self) -> &Cpumbintclr {
        &self.cpumbintclr
    }
    #[doc = "0x88 - CPU Mailbox Interrupt Status"]
    #[inline(always)]
    pub const fn cpumbintstat(&self) -> &Cpumbintstat {
        &self.cpumbintstat
    }
    #[doc = "0x8c - CPU CPU Mailbox Data"]
    #[inline(always)]
    pub const fn cpucpumbdata(&self) -> &Cpucpumbdata {
        &self.cpucpumbdata
    }
    #[doc = "0x90 - DSP0 to CPU Mailbox Data"]
    #[inline(always)]
    pub const fn dsp0cpumbdata(&self) -> &Dsp0cpumbdata {
        &self.dsp0cpumbdata
    }
    #[doc = "0x94 - DSP1 to CPU Mailbox Data"]
    #[inline(always)]
    pub const fn dsp1cpumbdata(&self) -> &Dsp1cpumbdata {
        &self.dsp1cpumbdata
    }
    #[doc = "0xa0 - DSP0 Mailbox Interrupt Set"]
    #[inline(always)]
    pub const fn dsp0mbintset(&self) -> &Dsp0mbintset {
        &self.dsp0mbintset
    }
    #[doc = "0xa4 - DSP0 Mailbox Interrupt Clear"]
    #[inline(always)]
    pub const fn dsp0mbintclr(&self) -> &Dsp0mbintclr {
        &self.dsp0mbintclr
    }
    #[doc = "0xa8 - DSP 0 Mailbox Interrupt Status"]
    #[inline(always)]
    pub const fn dsp0mbintstat(&self) -> &Dsp0mbintstat {
        &self.dsp0mbintstat
    }
    #[doc = "0xac - CPU to DSP 0 Mailbox Data"]
    #[inline(always)]
    pub const fn cpudsp0mbdata(&self) -> &Cpudsp0mbdata {
        &self.cpudsp0mbdata
    }
    #[doc = "0xb0 - DSP0 to DSP 0 Mailbox Data"]
    #[inline(always)]
    pub const fn dsp0dsp0mbdata(&self) -> &Dsp0dsp0mbdata {
        &self.dsp0dsp0mbdata
    }
    #[doc = "0xb4 - DSP1 to DSP 0 Mailbox Data"]
    #[inline(always)]
    pub const fn dsp1dsp0mbdata(&self) -> &Dsp1dsp0mbdata {
        &self.dsp1dsp0mbdata
    }
    #[doc = "0xc0 - DSP1 Mailbox Interrupt Set"]
    #[inline(always)]
    pub const fn dsp1mbintset(&self) -> &Dsp1mbintset {
        &self.dsp1mbintset
    }
    #[doc = "0xc4 - DSP1 Mailbox Interrupt Clear"]
    #[inline(always)]
    pub const fn dsp1mbintclr(&self) -> &Dsp1mbintclr {
        &self.dsp1mbintclr
    }
    #[doc = "0xc8 - DSP 1 Mailbox Interrupt Status"]
    #[inline(always)]
    pub const fn dsp1mbintstat(&self) -> &Dsp1mbintstat {
        &self.dsp1mbintstat
    }
    #[doc = "0xcc - CPU to DSP 1 Mailbox Data"]
    #[inline(always)]
    pub const fn cpudsp1mbdata(&self) -> &Cpudsp1mbdata {
        &self.cpudsp1mbdata
    }
    #[doc = "0xd0 - DSP0 to DSP 1 Mailbox Data"]
    #[inline(always)]
    pub const fn dsp0dsp1mbdata(&self) -> &Dsp0dsp1mbdata {
        &self.dsp0dsp1mbdata
    }
    #[doc = "0xd4 - DSP1 to DSP 1 Mailbox Data"]
    #[inline(always)]
    pub const fn dsp1dsp1mbdata(&self) -> &Dsp1dsp1mbdata {
        &self.dsp1dsp1mbdata
    }
    #[doc = "0x100 - DSP 0 control settings"]
    #[inline(always)]
    pub const fn dsp0control(&self) -> &Dsp0control {
        &self.dsp0control
    }
    #[doc = "0x104 - DSP 0 Reset Vector"]
    #[inline(always)]
    pub const fn dsp0resetvec(&self) -> &Dsp0resetvec {
        &self.dsp0resetvec
    }
    #[doc = "0x108 - DSP 0 IRQ Mask"]
    #[inline(always)]
    pub const fn dsp0irqmask(&self) -> &Dsp0irqmask {
        &self.dsp0irqmask
    }
    #[doc = "0x10c - DSP 0 IRQ Wake Mask"]
    #[inline(always)]
    pub const fn dsp0wakemask(&self) -> &Dsp0wakemask {
        &self.dsp0wakemask
    }
    #[doc = "0x110 - DSP 0 Raw IRQ31-0 Status"]
    #[inline(always)]
    pub const fn dsp0rawirqstat31to0(&self) -> &Dsp0rawirqstat31to0 {
        &self.dsp0rawirqstat31to0
    }
    #[doc = "0x114 - DSP 0 Raw IRQ63-32 Status"]
    #[inline(always)]
    pub const fn dsp0rawirqstat63to32(&self) -> &Dsp0rawirqstat63to32 {
        &self.dsp0rawirqstat63to32
    }
    #[doc = "0x118 - DSP 0 Raw IRQ95-64 Status"]
    #[inline(always)]
    pub const fn dsp0rawirqstat95to64(&self) -> &Dsp0rawirqstat95to64 {
        &self.dsp0rawirqstat95to64
    }
    #[doc = "0x120 - DSP 0 L2 Level Interrupt Mux"]
    #[inline(always)]
    pub const fn dsp0l2lvlint(&self) -> &Dsp0l2lvlint {
        &self.dsp0l2lvlint
    }
    #[doc = "0x124 - DSP 0 L3 Level Interrupt Mux"]
    #[inline(always)]
    pub const fn dsp0l3lvlint(&self) -> &Dsp0l3lvlint {
        &self.dsp0l3lvlint
    }
    #[doc = "0x128 - DSP 0 L4 Level Interrupt Mux"]
    #[inline(always)]
    pub const fn dsp0l4lvlint(&self) -> &Dsp0l4lvlint {
        &self.dsp0l4lvlint
    }
    #[doc = "0x12c - DSP 0 L5 Level Interrupt Mux"]
    #[inline(always)]
    pub const fn dsp0l5lvlint(&self) -> &Dsp0l5lvlint {
        &self.dsp0l5lvlint
    }
    #[doc = "0x130 - DSP 0 IDMA Trigger Control and Status"]
    #[inline(always)]
    pub const fn dsp0idmatrigctl(&self) -> &Dsp0idmatrigctl {
        &self.dsp0idmatrigctl
    }
    #[doc = "0x140 - DSP0 Interrupt OR Mask A for IRQ31-0"]
    #[inline(always)]
    pub const fn dsp0intormask31to0a(&self) -> &Dsp0intormask31to0a {
        &self.dsp0intormask31to0a
    }
    #[doc = "0x144 - DSP0 Interrupt OR Mask A for IRQ63-32"]
    #[inline(always)]
    pub const fn dsp0intormask63to32a(&self) -> &Dsp0intormask63to32a {
        &self.dsp0intormask63to32a
    }
    #[doc = "0x148 - DSP0 Interrupt OR Mask A for IRQ95-64"]
    #[inline(always)]
    pub const fn dsp0intormask95to64a(&self) -> &Dsp0intormask95to64a {
        &self.dsp0intormask95to64a
    }
    #[doc = "0x150 - DSP0 Interrupt OR Mask B for IRQ31-0"]
    #[inline(always)]
    pub const fn dsp0intormask31to0b(&self) -> &Dsp0intormask31to0b {
        &self.dsp0intormask31to0b
    }
    #[doc = "0x154 - DSP0 Interrupt OR Mask A for IRQ63-32"]
    #[inline(always)]
    pub const fn dsp0intormask63to32b(&self) -> &Dsp0intormask63to32b {
        &self.dsp0intormask63to32b
    }
    #[doc = "0x158 - DSP0 Interrupt OR Mask B for IRQ95-64"]
    #[inline(always)]
    pub const fn dsp0intormask95to64b(&self) -> &Dsp0intormask95to64b {
        &self.dsp0intormask95to64b
    }
    #[doc = "0x160 - DSP0 INT Enable for IRQ31-0"]
    #[inline(always)]
    pub const fn dsp0intenirq31to0(&self) -> &Dsp0intenirq31to0 {
        &self.dsp0intenirq31to0
    }
    #[doc = "0x164 - DSP0 INT Enable for IRQ63-32"]
    #[inline(always)]
    pub const fn dsp0intenirq63to32(&self) -> &Dsp0intenirq63to32 {
        &self.dsp0intenirq63to32
    }
    #[doc = "0x168 - DSP0 INT Enable for IRQ95-64"]
    #[inline(always)]
    pub const fn dsp0intenirq95to64(&self) -> &Dsp0intenirq95to64 {
        &self.dsp0intenirq95to64
    }
    #[doc = "0x200 - DSP 1 control settings"]
    #[inline(always)]
    pub const fn dsp1control(&self) -> &Dsp1control {
        &self.dsp1control
    }
    #[doc = "0x204 - DSP 1 Reset Vector"]
    #[inline(always)]
    pub const fn dsp1resetvec(&self) -> &Dsp1resetvec {
        &self.dsp1resetvec
    }
    #[doc = "0x208 - DSP 1 IRQ Mask"]
    #[inline(always)]
    pub const fn dsp1irqmask(&self) -> &Dsp1irqmask {
        &self.dsp1irqmask
    }
    #[doc = "0x20c - DSP 1 IRQ Wake Mask"]
    #[inline(always)]
    pub const fn dsp1wakemask(&self) -> &Dsp1wakemask {
        &self.dsp1wakemask
    }
    #[doc = "0x210 - DSP 1 Raw IRQ31-0 Status"]
    #[inline(always)]
    pub const fn dsp1rawirqstat31to0(&self) -> &Dsp1rawirqstat31to0 {
        &self.dsp1rawirqstat31to0
    }
    #[doc = "0x214 - DSP 1 Raw IRQ63-32 Status"]
    #[inline(always)]
    pub const fn dsp1rawirqstat63to32(&self) -> &Dsp1rawirqstat63to32 {
        &self.dsp1rawirqstat63to32
    }
    #[doc = "0x218 - DSP 1 Raw IRQ95-64 Status"]
    #[inline(always)]
    pub const fn dsp1rawirqstat95to64(&self) -> &Dsp1rawirqstat95to64 {
        &self.dsp1rawirqstat95to64
    }
    #[doc = "0x220 - DSP 1 L2 Level Interrupt Mux"]
    #[inline(always)]
    pub const fn dsp1l2lvlint(&self) -> &Dsp1l2lvlint {
        &self.dsp1l2lvlint
    }
    #[doc = "0x224 - DSP 1 L3 Level Interrupt Mux"]
    #[inline(always)]
    pub const fn dsp1l3lvlint(&self) -> &Dsp1l3lvlint {
        &self.dsp1l3lvlint
    }
    #[doc = "0x228 - DSP 1 L4 Level Interrupt Mux"]
    #[inline(always)]
    pub const fn dsp1l4lvlint(&self) -> &Dsp1l4lvlint {
        &self.dsp1l4lvlint
    }
    #[doc = "0x22c - DSP 1 L5 Level Interrupt Mux"]
    #[inline(always)]
    pub const fn dsp1l5lvlint(&self) -> &Dsp1l5lvlint {
        &self.dsp1l5lvlint
    }
    #[doc = "0x230 - DSP 1 IDMA Trigger Control and Status"]
    #[inline(always)]
    pub const fn dsp1idmatrigctl(&self) -> &Dsp1idmatrigctl {
        &self.dsp1idmatrigctl
    }
    #[doc = "0x240 - DSP1 Interrupt OR Mask A for IRQ31-0"]
    #[inline(always)]
    pub const fn dsp1intormask31to0a(&self) -> &Dsp1intormask31to0a {
        &self.dsp1intormask31to0a
    }
    #[doc = "0x244 - DSP1 Interrupt OR Mask A for IRQ63-32"]
    #[inline(always)]
    pub const fn dsp1intormask63to32a(&self) -> &Dsp1intormask63to32a {
        &self.dsp1intormask63to32a
    }
    #[doc = "0x248 - DSP1 Interrupt OR Mask A for IRQ95-64"]
    #[inline(always)]
    pub const fn dsp1intormask95to64a(&self) -> &Dsp1intormask95to64a {
        &self.dsp1intormask95to64a
    }
    #[doc = "0x250 - DSP1 Interrupt OR Mask B for IRQ31-0"]
    #[inline(always)]
    pub const fn dsp1intormask31to0b(&self) -> &Dsp1intormask31to0b {
        &self.dsp1intormask31to0b
    }
    #[doc = "0x254 - DSP1 Interrupt OR Mask A for IRQ63-32"]
    #[inline(always)]
    pub const fn dsp1intormask63to32b(&self) -> &Dsp1intormask63to32b {
        &self.dsp1intormask63to32b
    }
    #[doc = "0x258 - DSP1 Interrupt OR Mask B for IRQ95-64"]
    #[inline(always)]
    pub const fn dsp1intormask95to64b(&self) -> &Dsp1intormask95to64b {
        &self.dsp1intormask95to64b
    }
    #[doc = "0x260 - DSP1 INT Enable for IRQ31-0"]
    #[inline(always)]
    pub const fn dsp1intenirq31to0(&self) -> &Dsp1intenirq31to0 {
        &self.dsp1intenirq31to0
    }
    #[doc = "0x264 - DSP1 INT Enable for IRQ63-32"]
    #[inline(always)]
    pub const fn dsp1intenirq63to32(&self) -> &Dsp1intenirq63to32 {
        &self.dsp1intenirq63to32
    }
    #[doc = "0x268 - DSP1 INT Enable for IRQ95-64"]
    #[inline(always)]
    pub const fn dsp1intenirq95to64(&self) -> &Dsp1intenirq95to64 {
        &self.dsp1intenirq95to64
    }
}
#[doc = "MUTEX0 (rw) register accessor: MUTEX 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mutex0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mutex0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mutex0`]
module"]
#[doc(alias = "MUTEX0")]
pub type Mutex0 = crate::Reg<mutex0::Mutex0Spec>;
#[doc = "MUTEX 0"]
pub mod mutex0;
#[doc = "MUTEX1 (rw) register accessor: MUTEX 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mutex1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mutex1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mutex1`]
module"]
#[doc(alias = "MUTEX1")]
pub type Mutex1 = crate::Reg<mutex1::Mutex1Spec>;
#[doc = "MUTEX 1"]
pub mod mutex1;
#[doc = "MUTEX2 (rw) register accessor: MUTEX 2\n\nYou can [`read`](crate::Reg::read) this register and get [`mutex2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mutex2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mutex2`]
module"]
#[doc(alias = "MUTEX2")]
pub type Mutex2 = crate::Reg<mutex2::Mutex2Spec>;
#[doc = "MUTEX 2"]
pub mod mutex2;
#[doc = "MUTEX3 (rw) register accessor: MUTEX 3\n\nYou can [`read`](crate::Reg::read) this register and get [`mutex3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mutex3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mutex3`]
module"]
#[doc(alias = "MUTEX3")]
pub type Mutex3 = crate::Reg<mutex3::Mutex3Spec>;
#[doc = "MUTEX 3"]
pub mod mutex3;
#[doc = "MUTEX4 (rw) register accessor: MUTEX 4\n\nYou can [`read`](crate::Reg::read) this register and get [`mutex4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mutex4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mutex4`]
module"]
#[doc(alias = "MUTEX4")]
pub type Mutex4 = crate::Reg<mutex4::Mutex4Spec>;
#[doc = "MUTEX 4"]
pub mod mutex4;
#[doc = "MUTEX5 (rw) register accessor: MUTEX 5\n\nYou can [`read`](crate::Reg::read) this register and get [`mutex5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mutex5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mutex5`]
module"]
#[doc(alias = "MUTEX5")]
pub type Mutex5 = crate::Reg<mutex5::Mutex5Spec>;
#[doc = "MUTEX 5"]
pub mod mutex5;
#[doc = "MUTEX6 (rw) register accessor: MUTEX 6\n\nYou can [`read`](crate::Reg::read) this register and get [`mutex6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mutex6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mutex6`]
module"]
#[doc(alias = "MUTEX6")]
pub type Mutex6 = crate::Reg<mutex6::Mutex6Spec>;
#[doc = "MUTEX 6"]
pub mod mutex6;
#[doc = "MUTEX7 (rw) register accessor: MUTEX 7\n\nYou can [`read`](crate::Reg::read) this register and get [`mutex7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mutex7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mutex7`]
module"]
#[doc(alias = "MUTEX7")]
pub type Mutex7 = crate::Reg<mutex7::Mutex7Spec>;
#[doc = "MUTEX 7"]
pub mod mutex7;
#[doc = "CPUMBINTSET (rw) register accessor: CPU Mailbox Interrupt Set\n\nYou can [`read`](crate::Reg::read) this register and get [`cpumbintset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpumbintset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpumbintset`]
module"]
#[doc(alias = "CPUMBINTSET")]
pub type Cpumbintset = crate::Reg<cpumbintset::CpumbintsetSpec>;
#[doc = "CPU Mailbox Interrupt Set"]
pub mod cpumbintset;
#[doc = "CPUMBINTCLR (rw) register accessor: CPU Mailbox Interrupt Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`cpumbintclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpumbintclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpumbintclr`]
module"]
#[doc(alias = "CPUMBINTCLR")]
pub type Cpumbintclr = crate::Reg<cpumbintclr::CpumbintclrSpec>;
#[doc = "CPU Mailbox Interrupt Clear"]
pub mod cpumbintclr;
#[doc = "CPUMBINTSTAT (rw) register accessor: CPU Mailbox Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpumbintstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpumbintstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpumbintstat`]
module"]
#[doc(alias = "CPUMBINTSTAT")]
pub type Cpumbintstat = crate::Reg<cpumbintstat::CpumbintstatSpec>;
#[doc = "CPU Mailbox Interrupt Status"]
pub mod cpumbintstat;
#[doc = "CPUCPUMBDATA (rw) register accessor: CPU CPU Mailbox Data\n\nYou can [`read`](crate::Reg::read) this register and get [`cpucpumbdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpucpumbdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpucpumbdata`]
module"]
#[doc(alias = "CPUCPUMBDATA")]
pub type Cpucpumbdata = crate::Reg<cpucpumbdata::CpucpumbdataSpec>;
#[doc = "CPU CPU Mailbox Data"]
pub mod cpucpumbdata;
#[doc = "DSP0CPUMBDATA (rw) register accessor: DSP0 to CPU Mailbox Data\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0cpumbdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0cpumbdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0cpumbdata`]
module"]
#[doc(alias = "DSP0CPUMBDATA")]
pub type Dsp0cpumbdata = crate::Reg<dsp0cpumbdata::Dsp0cpumbdataSpec>;
#[doc = "DSP0 to CPU Mailbox Data"]
pub mod dsp0cpumbdata;
#[doc = "DSP1CPUMBDATA (rw) register accessor: DSP1 to CPU Mailbox Data\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1cpumbdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1cpumbdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1cpumbdata`]
module"]
#[doc(alias = "DSP1CPUMBDATA")]
pub type Dsp1cpumbdata = crate::Reg<dsp1cpumbdata::Dsp1cpumbdataSpec>;
#[doc = "DSP1 to CPU Mailbox Data"]
pub mod dsp1cpumbdata;
#[doc = "DSP0MBINTSET (rw) register accessor: DSP0 Mailbox Interrupt Set\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0mbintset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0mbintset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0mbintset`]
module"]
#[doc(alias = "DSP0MBINTSET")]
pub type Dsp0mbintset = crate::Reg<dsp0mbintset::Dsp0mbintsetSpec>;
#[doc = "DSP0 Mailbox Interrupt Set"]
pub mod dsp0mbintset;
#[doc = "DSP0MBINTCLR (rw) register accessor: DSP0 Mailbox Interrupt Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0mbintclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0mbintclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0mbintclr`]
module"]
#[doc(alias = "DSP0MBINTCLR")]
pub type Dsp0mbintclr = crate::Reg<dsp0mbintclr::Dsp0mbintclrSpec>;
#[doc = "DSP0 Mailbox Interrupt Clear"]
pub mod dsp0mbintclr;
#[doc = "DSP0MBINTSTAT (rw) register accessor: DSP 0 Mailbox Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0mbintstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0mbintstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0mbintstat`]
module"]
#[doc(alias = "DSP0MBINTSTAT")]
pub type Dsp0mbintstat = crate::Reg<dsp0mbintstat::Dsp0mbintstatSpec>;
#[doc = "DSP 0 Mailbox Interrupt Status"]
pub mod dsp0mbintstat;
#[doc = "CPUDSP0MBDATA (rw) register accessor: CPU to DSP 0 Mailbox Data\n\nYou can [`read`](crate::Reg::read) this register and get [`cpudsp0mbdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpudsp0mbdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpudsp0mbdata`]
module"]
#[doc(alias = "CPUDSP0MBDATA")]
pub type Cpudsp0mbdata = crate::Reg<cpudsp0mbdata::Cpudsp0mbdataSpec>;
#[doc = "CPU to DSP 0 Mailbox Data"]
pub mod cpudsp0mbdata;
#[doc = "DSP0DSP0MBDATA (rw) register accessor: DSP0 to DSP 0 Mailbox Data\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0dsp0mbdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0dsp0mbdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0dsp0mbdata`]
module"]
#[doc(alias = "DSP0DSP0MBDATA")]
pub type Dsp0dsp0mbdata = crate::Reg<dsp0dsp0mbdata::Dsp0dsp0mbdataSpec>;
#[doc = "DSP0 to DSP 0 Mailbox Data"]
pub mod dsp0dsp0mbdata;
#[doc = "DSP1DSP0MBDATA (rw) register accessor: DSP1 to DSP 0 Mailbox Data\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1dsp0mbdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1dsp0mbdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1dsp0mbdata`]
module"]
#[doc(alias = "DSP1DSP0MBDATA")]
pub type Dsp1dsp0mbdata = crate::Reg<dsp1dsp0mbdata::Dsp1dsp0mbdataSpec>;
#[doc = "DSP1 to DSP 0 Mailbox Data"]
pub mod dsp1dsp0mbdata;
#[doc = "DSP1MBINTSET (rw) register accessor: DSP1 Mailbox Interrupt Set\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1mbintset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1mbintset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1mbintset`]
module"]
#[doc(alias = "DSP1MBINTSET")]
pub type Dsp1mbintset = crate::Reg<dsp1mbintset::Dsp1mbintsetSpec>;
#[doc = "DSP1 Mailbox Interrupt Set"]
pub mod dsp1mbintset;
#[doc = "DSP1MBINTCLR (rw) register accessor: DSP1 Mailbox Interrupt Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1mbintclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1mbintclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1mbintclr`]
module"]
#[doc(alias = "DSP1MBINTCLR")]
pub type Dsp1mbintclr = crate::Reg<dsp1mbintclr::Dsp1mbintclrSpec>;
#[doc = "DSP1 Mailbox Interrupt Clear"]
pub mod dsp1mbintclr;
#[doc = "DSP1MBINTSTAT (rw) register accessor: DSP 1 Mailbox Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1mbintstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1mbintstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1mbintstat`]
module"]
#[doc(alias = "DSP1MBINTSTAT")]
pub type Dsp1mbintstat = crate::Reg<dsp1mbintstat::Dsp1mbintstatSpec>;
#[doc = "DSP 1 Mailbox Interrupt Status"]
pub mod dsp1mbintstat;
#[doc = "CPUDSP1MBDATA (rw) register accessor: CPU to DSP 1 Mailbox Data\n\nYou can [`read`](crate::Reg::read) this register and get [`cpudsp1mbdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpudsp1mbdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpudsp1mbdata`]
module"]
#[doc(alias = "CPUDSP1MBDATA")]
pub type Cpudsp1mbdata = crate::Reg<cpudsp1mbdata::Cpudsp1mbdataSpec>;
#[doc = "CPU to DSP 1 Mailbox Data"]
pub mod cpudsp1mbdata;
#[doc = "DSP0DSP1MBDATA (rw) register accessor: DSP0 to DSP 1 Mailbox Data\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0dsp1mbdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0dsp1mbdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0dsp1mbdata`]
module"]
#[doc(alias = "DSP0DSP1MBDATA")]
pub type Dsp0dsp1mbdata = crate::Reg<dsp0dsp1mbdata::Dsp0dsp1mbdataSpec>;
#[doc = "DSP0 to DSP 1 Mailbox Data"]
pub mod dsp0dsp1mbdata;
#[doc = "DSP1DSP1MBDATA (rw) register accessor: DSP1 to DSP 1 Mailbox Data\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1dsp1mbdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1dsp1mbdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1dsp1mbdata`]
module"]
#[doc(alias = "DSP1DSP1MBDATA")]
pub type Dsp1dsp1mbdata = crate::Reg<dsp1dsp1mbdata::Dsp1dsp1mbdataSpec>;
#[doc = "DSP1 to DSP 1 Mailbox Data"]
pub mod dsp1dsp1mbdata;
#[doc = "DSP0CONTROL (rw) register accessor: DSP 0 control settings\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0control`]
module"]
#[doc(alias = "DSP0CONTROL")]
pub type Dsp0control = crate::Reg<dsp0control::Dsp0controlSpec>;
#[doc = "DSP 0 control settings"]
pub mod dsp0control;
#[doc = "DSP0RESETVEC (rw) register accessor: DSP 0 Reset Vector\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0resetvec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0resetvec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0resetvec`]
module"]
#[doc(alias = "DSP0RESETVEC")]
pub type Dsp0resetvec = crate::Reg<dsp0resetvec::Dsp0resetvecSpec>;
#[doc = "DSP 0 Reset Vector"]
pub mod dsp0resetvec;
#[doc = "DSP0IRQMASK (rw) register accessor: DSP 0 IRQ Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0irqmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0irqmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0irqmask`]
module"]
#[doc(alias = "DSP0IRQMASK")]
pub type Dsp0irqmask = crate::Reg<dsp0irqmask::Dsp0irqmaskSpec>;
#[doc = "DSP 0 IRQ Mask"]
pub mod dsp0irqmask;
#[doc = "DSP0WAKEMASK (rw) register accessor: DSP 0 IRQ Wake Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0wakemask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0wakemask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0wakemask`]
module"]
#[doc(alias = "DSP0WAKEMASK")]
pub type Dsp0wakemask = crate::Reg<dsp0wakemask::Dsp0wakemaskSpec>;
#[doc = "DSP 0 IRQ Wake Mask"]
pub mod dsp0wakemask;
#[doc = "DSP0RAWIRQSTAT31to0 (rw) register accessor: DSP 0 Raw IRQ31-0 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0rawirqstat31to0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0rawirqstat31to0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0rawirqstat31to0`]
module"]
#[doc(alias = "DSP0RAWIRQSTAT31to0")]
pub type Dsp0rawirqstat31to0 = crate::Reg<dsp0rawirqstat31to0::Dsp0rawirqstat31to0Spec>;
#[doc = "DSP 0 Raw IRQ31-0 Status"]
pub mod dsp0rawirqstat31to0;
#[doc = "DSP0RAWIRQSTAT63to32 (rw) register accessor: DSP 0 Raw IRQ63-32 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0rawirqstat63to32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0rawirqstat63to32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0rawirqstat63to32`]
module"]
#[doc(alias = "DSP0RAWIRQSTAT63to32")]
pub type Dsp0rawirqstat63to32 = crate::Reg<dsp0rawirqstat63to32::Dsp0rawirqstat63to32Spec>;
#[doc = "DSP 0 Raw IRQ63-32 Status"]
pub mod dsp0rawirqstat63to32;
#[doc = "DSP0RAWIRQSTAT95to64 (rw) register accessor: DSP 0 Raw IRQ95-64 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0rawirqstat95to64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0rawirqstat95to64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0rawirqstat95to64`]
module"]
#[doc(alias = "DSP0RAWIRQSTAT95to64")]
pub type Dsp0rawirqstat95to64 = crate::Reg<dsp0rawirqstat95to64::Dsp0rawirqstat95to64Spec>;
#[doc = "DSP 0 Raw IRQ95-64 Status"]
pub mod dsp0rawirqstat95to64;
#[doc = "DSP0L2LVLINT (rw) register accessor: DSP 0 L2 Level Interrupt Mux\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0l2lvlint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0l2lvlint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0l2lvlint`]
module"]
#[doc(alias = "DSP0L2LVLINT")]
pub type Dsp0l2lvlint = crate::Reg<dsp0l2lvlint::Dsp0l2lvlintSpec>;
#[doc = "DSP 0 L2 Level Interrupt Mux"]
pub mod dsp0l2lvlint;
#[doc = "DSP0L3LVLINT (rw) register accessor: DSP 0 L3 Level Interrupt Mux\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0l3lvlint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0l3lvlint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0l3lvlint`]
module"]
#[doc(alias = "DSP0L3LVLINT")]
pub type Dsp0l3lvlint = crate::Reg<dsp0l3lvlint::Dsp0l3lvlintSpec>;
#[doc = "DSP 0 L3 Level Interrupt Mux"]
pub mod dsp0l3lvlint;
#[doc = "DSP0L4LVLINT (rw) register accessor: DSP 0 L4 Level Interrupt Mux\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0l4lvlint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0l4lvlint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0l4lvlint`]
module"]
#[doc(alias = "DSP0L4LVLINT")]
pub type Dsp0l4lvlint = crate::Reg<dsp0l4lvlint::Dsp0l4lvlintSpec>;
#[doc = "DSP 0 L4 Level Interrupt Mux"]
pub mod dsp0l4lvlint;
#[doc = "DSP0L5LVLINT (rw) register accessor: DSP 0 L5 Level Interrupt Mux\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0l5lvlint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0l5lvlint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0l5lvlint`]
module"]
#[doc(alias = "DSP0L5LVLINT")]
pub type Dsp0l5lvlint = crate::Reg<dsp0l5lvlint::Dsp0l5lvlintSpec>;
#[doc = "DSP 0 L5 Level Interrupt Mux"]
pub mod dsp0l5lvlint;
#[doc = "DSP0IDMATRIGCTL (rw) register accessor: DSP 0 IDMA Trigger Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0idmatrigctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0idmatrigctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0idmatrigctl`]
module"]
#[doc(alias = "DSP0IDMATRIGCTL")]
pub type Dsp0idmatrigctl = crate::Reg<dsp0idmatrigctl::Dsp0idmatrigctlSpec>;
#[doc = "DSP 0 IDMA Trigger Control and Status"]
pub mod dsp0idmatrigctl;
#[doc = "DSP0INTORMASK31TO0A (rw) register accessor: DSP0 Interrupt OR Mask A for IRQ31-0\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0intormask31to0a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0intormask31to0a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0intormask31to0a`]
module"]
#[doc(alias = "DSP0INTORMASK31TO0A")]
pub type Dsp0intormask31to0a = crate::Reg<dsp0intormask31to0a::Dsp0intormask31to0aSpec>;
#[doc = "DSP0 Interrupt OR Mask A for IRQ31-0"]
pub mod dsp0intormask31to0a;
#[doc = "DSP0INTORMASK63TO32A (rw) register accessor: DSP0 Interrupt OR Mask A for IRQ63-32\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0intormask63to32a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0intormask63to32a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0intormask63to32a`]
module"]
#[doc(alias = "DSP0INTORMASK63TO32A")]
pub type Dsp0intormask63to32a = crate::Reg<dsp0intormask63to32a::Dsp0intormask63to32aSpec>;
#[doc = "DSP0 Interrupt OR Mask A for IRQ63-32"]
pub mod dsp0intormask63to32a;
#[doc = "DSP0INTORMASK95TO64A (rw) register accessor: DSP0 Interrupt OR Mask A for IRQ95-64\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0intormask95to64a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0intormask95to64a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0intormask95to64a`]
module"]
#[doc(alias = "DSP0INTORMASK95TO64A")]
pub type Dsp0intormask95to64a = crate::Reg<dsp0intormask95to64a::Dsp0intormask95to64aSpec>;
#[doc = "DSP0 Interrupt OR Mask A for IRQ95-64"]
pub mod dsp0intormask95to64a;
#[doc = "DSP0INTORMASK31to0B (rw) register accessor: DSP0 Interrupt OR Mask B for IRQ31-0\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0intormask31to0b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0intormask31to0b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0intormask31to0b`]
module"]
#[doc(alias = "DSP0INTORMASK31to0B")]
pub type Dsp0intormask31to0b = crate::Reg<dsp0intormask31to0b::Dsp0intormask31to0bSpec>;
#[doc = "DSP0 Interrupt OR Mask B for IRQ31-0"]
pub mod dsp0intormask31to0b;
#[doc = "DSP0INTORMASK63TO32B (rw) register accessor: DSP0 Interrupt OR Mask A for IRQ63-32\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0intormask63to32b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0intormask63to32b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0intormask63to32b`]
module"]
#[doc(alias = "DSP0INTORMASK63TO32B")]
pub type Dsp0intormask63to32b = crate::Reg<dsp0intormask63to32b::Dsp0intormask63to32bSpec>;
#[doc = "DSP0 Interrupt OR Mask A for IRQ63-32"]
pub mod dsp0intormask63to32b;
#[doc = "DSP0INTORMASK95TO64B (rw) register accessor: DSP0 Interrupt OR Mask B for IRQ95-64\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0intormask95to64b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0intormask95to64b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0intormask95to64b`]
module"]
#[doc(alias = "DSP0INTORMASK95TO64B")]
pub type Dsp0intormask95to64b = crate::Reg<dsp0intormask95to64b::Dsp0intormask95to64bSpec>;
#[doc = "DSP0 Interrupt OR Mask B for IRQ95-64"]
pub mod dsp0intormask95to64b;
#[doc = "DSP0INTENIRQ31TO0 (rw) register accessor: DSP0 INT Enable for IRQ31-0\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0intenirq31to0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0intenirq31to0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0intenirq31to0`]
module"]
#[doc(alias = "DSP0INTENIRQ31TO0")]
pub type Dsp0intenirq31to0 = crate::Reg<dsp0intenirq31to0::Dsp0intenirq31to0Spec>;
#[doc = "DSP0 INT Enable for IRQ31-0"]
pub mod dsp0intenirq31to0;
#[doc = "DSP0INTENIRQ63TO32 (rw) register accessor: DSP0 INT Enable for IRQ63-32\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0intenirq63to32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0intenirq63to32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0intenirq63to32`]
module"]
#[doc(alias = "DSP0INTENIRQ63TO32")]
pub type Dsp0intenirq63to32 = crate::Reg<dsp0intenirq63to32::Dsp0intenirq63to32Spec>;
#[doc = "DSP0 INT Enable for IRQ63-32"]
pub mod dsp0intenirq63to32;
#[doc = "DSP0INTENIRQ95TO64 (rw) register accessor: DSP0 INT Enable for IRQ95-64\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0intenirq95to64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0intenirq95to64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0intenirq95to64`]
module"]
#[doc(alias = "DSP0INTENIRQ95TO64")]
pub type Dsp0intenirq95to64 = crate::Reg<dsp0intenirq95to64::Dsp0intenirq95to64Spec>;
#[doc = "DSP0 INT Enable for IRQ95-64"]
pub mod dsp0intenirq95to64;
#[doc = "DSP1CONTROL (rw) register accessor: DSP 1 control settings\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1control`]
module"]
#[doc(alias = "DSP1CONTROL")]
pub type Dsp1control = crate::Reg<dsp1control::Dsp1controlSpec>;
#[doc = "DSP 1 control settings"]
pub mod dsp1control;
#[doc = "DSP1RESETVEC (rw) register accessor: DSP 1 Reset Vector\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1resetvec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1resetvec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1resetvec`]
module"]
#[doc(alias = "DSP1RESETVEC")]
pub type Dsp1resetvec = crate::Reg<dsp1resetvec::Dsp1resetvecSpec>;
#[doc = "DSP 1 Reset Vector"]
pub mod dsp1resetvec;
#[doc = "DSP1IRQMASK (rw) register accessor: DSP 1 IRQ Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1irqmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1irqmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1irqmask`]
module"]
#[doc(alias = "DSP1IRQMASK")]
pub type Dsp1irqmask = crate::Reg<dsp1irqmask::Dsp1irqmaskSpec>;
#[doc = "DSP 1 IRQ Mask"]
pub mod dsp1irqmask;
#[doc = "DSP1WAKEMASK (rw) register accessor: DSP 1 IRQ Wake Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1wakemask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1wakemask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1wakemask`]
module"]
#[doc(alias = "DSP1WAKEMASK")]
pub type Dsp1wakemask = crate::Reg<dsp1wakemask::Dsp1wakemaskSpec>;
#[doc = "DSP 1 IRQ Wake Mask"]
pub mod dsp1wakemask;
#[doc = "DSP1RAWIRQSTAT31to0 (rw) register accessor: DSP 1 Raw IRQ31-0 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1rawirqstat31to0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1rawirqstat31to0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1rawirqstat31to0`]
module"]
#[doc(alias = "DSP1RAWIRQSTAT31to0")]
pub type Dsp1rawirqstat31to0 = crate::Reg<dsp1rawirqstat31to0::Dsp1rawirqstat31to0Spec>;
#[doc = "DSP 1 Raw IRQ31-0 Status"]
pub mod dsp1rawirqstat31to0;
#[doc = "DSP1RAWIRQSTAT63to32 (rw) register accessor: DSP 1 Raw IRQ63-32 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1rawirqstat63to32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1rawirqstat63to32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1rawirqstat63to32`]
module"]
#[doc(alias = "DSP1RAWIRQSTAT63to32")]
pub type Dsp1rawirqstat63to32 = crate::Reg<dsp1rawirqstat63to32::Dsp1rawirqstat63to32Spec>;
#[doc = "DSP 1 Raw IRQ63-32 Status"]
pub mod dsp1rawirqstat63to32;
#[doc = "DSP1RAWIRQSTAT95to64 (rw) register accessor: DSP 1 Raw IRQ95-64 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1rawirqstat95to64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1rawirqstat95to64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1rawirqstat95to64`]
module"]
#[doc(alias = "DSP1RAWIRQSTAT95to64")]
pub type Dsp1rawirqstat95to64 = crate::Reg<dsp1rawirqstat95to64::Dsp1rawirqstat95to64Spec>;
#[doc = "DSP 1 Raw IRQ95-64 Status"]
pub mod dsp1rawirqstat95to64;
#[doc = "DSP1L2LVLINT (rw) register accessor: DSP 1 L2 Level Interrupt Mux\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1l2lvlint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1l2lvlint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1l2lvlint`]
module"]
#[doc(alias = "DSP1L2LVLINT")]
pub type Dsp1l2lvlint = crate::Reg<dsp1l2lvlint::Dsp1l2lvlintSpec>;
#[doc = "DSP 1 L2 Level Interrupt Mux"]
pub mod dsp1l2lvlint;
#[doc = "DSP1L3LVLINT (rw) register accessor: DSP 1 L3 Level Interrupt Mux\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1l3lvlint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1l3lvlint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1l3lvlint`]
module"]
#[doc(alias = "DSP1L3LVLINT")]
pub type Dsp1l3lvlint = crate::Reg<dsp1l3lvlint::Dsp1l3lvlintSpec>;
#[doc = "DSP 1 L3 Level Interrupt Mux"]
pub mod dsp1l3lvlint;
#[doc = "DSP1L4LVLINT (rw) register accessor: DSP 1 L4 Level Interrupt Mux\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1l4lvlint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1l4lvlint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1l4lvlint`]
module"]
#[doc(alias = "DSP1L4LVLINT")]
pub type Dsp1l4lvlint = crate::Reg<dsp1l4lvlint::Dsp1l4lvlintSpec>;
#[doc = "DSP 1 L4 Level Interrupt Mux"]
pub mod dsp1l4lvlint;
#[doc = "DSP1L5LVLINT (rw) register accessor: DSP 1 L5 Level Interrupt Mux\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1l5lvlint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1l5lvlint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1l5lvlint`]
module"]
#[doc(alias = "DSP1L5LVLINT")]
pub type Dsp1l5lvlint = crate::Reg<dsp1l5lvlint::Dsp1l5lvlintSpec>;
#[doc = "DSP 1 L5 Level Interrupt Mux"]
pub mod dsp1l5lvlint;
#[doc = "DSP1IDMATRIGCTL (rw) register accessor: DSP 1 IDMA Trigger Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1idmatrigctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1idmatrigctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1idmatrigctl`]
module"]
#[doc(alias = "DSP1IDMATRIGCTL")]
pub type Dsp1idmatrigctl = crate::Reg<dsp1idmatrigctl::Dsp1idmatrigctlSpec>;
#[doc = "DSP 1 IDMA Trigger Control and Status"]
pub mod dsp1idmatrigctl;
#[doc = "DSP1INTORMASK31TO0A (rw) register accessor: DSP1 Interrupt OR Mask A for IRQ31-0\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1intormask31to0a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1intormask31to0a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1intormask31to0a`]
module"]
#[doc(alias = "DSP1INTORMASK31TO0A")]
pub type Dsp1intormask31to0a = crate::Reg<dsp1intormask31to0a::Dsp1intormask31to0aSpec>;
#[doc = "DSP1 Interrupt OR Mask A for IRQ31-0"]
pub mod dsp1intormask31to0a;
#[doc = "DSP1INTORMASK63TO32A (rw) register accessor: DSP1 Interrupt OR Mask A for IRQ63-32\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1intormask63to32a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1intormask63to32a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1intormask63to32a`]
module"]
#[doc(alias = "DSP1INTORMASK63TO32A")]
pub type Dsp1intormask63to32a = crate::Reg<dsp1intormask63to32a::Dsp1intormask63to32aSpec>;
#[doc = "DSP1 Interrupt OR Mask A for IRQ63-32"]
pub mod dsp1intormask63to32a;
#[doc = "DSP1INTORMASK95TO64A (rw) register accessor: DSP1 Interrupt OR Mask A for IRQ95-64\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1intormask95to64a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1intormask95to64a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1intormask95to64a`]
module"]
#[doc(alias = "DSP1INTORMASK95TO64A")]
pub type Dsp1intormask95to64a = crate::Reg<dsp1intormask95to64a::Dsp1intormask95to64aSpec>;
#[doc = "DSP1 Interrupt OR Mask A for IRQ95-64"]
pub mod dsp1intormask95to64a;
#[doc = "DSP1INTORMASK31to0B (rw) register accessor: DSP1 Interrupt OR Mask B for IRQ31-0\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1intormask31to0b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1intormask31to0b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1intormask31to0b`]
module"]
#[doc(alias = "DSP1INTORMASK31to0B")]
pub type Dsp1intormask31to0b = crate::Reg<dsp1intormask31to0b::Dsp1intormask31to0bSpec>;
#[doc = "DSP1 Interrupt OR Mask B for IRQ31-0"]
pub mod dsp1intormask31to0b;
#[doc = "DSP1INTORMASK63TO32B (rw) register accessor: DSP1 Interrupt OR Mask A for IRQ63-32\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1intormask63to32b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1intormask63to32b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1intormask63to32b`]
module"]
#[doc(alias = "DSP1INTORMASK63TO32B")]
pub type Dsp1intormask63to32b = crate::Reg<dsp1intormask63to32b::Dsp1intormask63to32bSpec>;
#[doc = "DSP1 Interrupt OR Mask A for IRQ63-32"]
pub mod dsp1intormask63to32b;
#[doc = "DSP1INTORMASK95TO64B (rw) register accessor: DSP1 Interrupt OR Mask B for IRQ95-64\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1intormask95to64b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1intormask95to64b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1intormask95to64b`]
module"]
#[doc(alias = "DSP1INTORMASK95TO64B")]
pub type Dsp1intormask95to64b = crate::Reg<dsp1intormask95to64b::Dsp1intormask95to64bSpec>;
#[doc = "DSP1 Interrupt OR Mask B for IRQ95-64"]
pub mod dsp1intormask95to64b;
#[doc = "DSP1INTENIRQ31TO0 (rw) register accessor: DSP1 INT Enable for IRQ31-0\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1intenirq31to0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1intenirq31to0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1intenirq31to0`]
module"]
#[doc(alias = "DSP1INTENIRQ31TO0")]
pub type Dsp1intenirq31to0 = crate::Reg<dsp1intenirq31to0::Dsp1intenirq31to0Spec>;
#[doc = "DSP1 INT Enable for IRQ31-0"]
pub mod dsp1intenirq31to0;
#[doc = "DSP1INTENIRQ63TO32 (rw) register accessor: DSP1 INT Enable for IRQ63-32\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1intenirq63to32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1intenirq63to32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1intenirq63to32`]
module"]
#[doc(alias = "DSP1INTENIRQ63TO32")]
pub type Dsp1intenirq63to32 = crate::Reg<dsp1intenirq63to32::Dsp1intenirq63to32Spec>;
#[doc = "DSP1 INT Enable for IRQ63-32"]
pub mod dsp1intenirq63to32;
#[doc = "DSP1INTENIRQ95TO64 (rw) register accessor: DSP1 INT Enable for IRQ95-64\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1intenirq95to64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1intenirq95to64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1intenirq95to64`]
module"]
#[doc(alias = "DSP1INTENIRQ95TO64")]
pub type Dsp1intenirq95to64 = crate::Reg<dsp1intenirq95to64::Dsp1intenirq95to64Spec>;
#[doc = "DSP1 INT Enable for IRQ95-64"]
pub mod dsp1intenirq95to64;
