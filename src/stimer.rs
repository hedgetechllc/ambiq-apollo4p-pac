#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    stcfg: Stcfg,
    sttmr: Sttmr,
    _reserved2: [u8; 0x08],
    scapctrl0: Scapctrl0,
    scapctrl1: Scapctrl1,
    scapctrl2: Scapctrl2,
    scapctrl3: Scapctrl3,
    scmpr0: Scmpr0,
    scmpr1: Scmpr1,
    scmpr2: Scmpr2,
    scmpr3: Scmpr3,
    scmpr4: Scmpr4,
    scmpr5: Scmpr5,
    scmpr6: Scmpr6,
    scmpr7: Scmpr7,
    scapt0: Scapt0,
    scapt1: Scapt1,
    scapt2: Scapt2,
    scapt3: Scapt3,
    snvr0: Snvr0,
    snvr1: Snvr1,
    snvr2: Snvr2,
    _reserved21: [u8; 0xa4],
    stminten: Stminten,
    stmintstat: Stmintstat,
    stmintclr: Stmintclr,
    stmintset: Stmintset,
}
impl RegisterBlock {
    #[doc = "0x00 - The STIMER Configuration Register contains the software control for selecting the clock divider and source feeding the system timer."]
    #[inline(always)]
    pub const fn stcfg(&self) -> &Stcfg {
        &self.stcfg
    }
    #[doc = "0x04 - The COUNTER Register contains the running count of time as maintained by incrementing for every rising clock edge of the clock source selected in the configuration register. It is this counter value that captured in the capture registers and it is this counter value that is compared against the various compare registers. This register cannot be written, but can be cleared to 0 for a deterministic value. Use the FREEZE bit will stop this counter from incrementing."]
    #[inline(always)]
    pub const fn sttmr(&self) -> &Sttmr {
        &self.sttmr
    }
    #[doc = "0x10 - The STIMER Capture Control Register controls each of the 4 capture registers. It selects their GPIO pin number for a trigger source, enables a capture operation and sets the input polarity for the capture. NOTE: 8-bit writes can control individual capture registers atomically."]
    #[inline(always)]
    pub const fn scapctrl0(&self) -> &Scapctrl0 {
        &self.scapctrl0
    }
    #[doc = "0x14 - The STIMER Capture Control Register controls each of the 4 capture registers. It selects their GPIO pin number for a trigger source, enables a capture operation and sets the input polarity for the capture. NOTE: 8-bit writes can control individual capture registers atomically."]
    #[inline(always)]
    pub const fn scapctrl1(&self) -> &Scapctrl1 {
        &self.scapctrl1
    }
    #[doc = "0x18 - The STIMER Capture Control Register controls each of the 4 capture registers. It selects their GPIO pin number for a trigger source, enables a capture operation and sets the input polarity for the capture. NOTE: 8-bit writes can control individual capture registers atomically."]
    #[inline(always)]
    pub const fn scapctrl2(&self) -> &Scapctrl2 {
        &self.scapctrl2
    }
    #[doc = "0x1c - The STIMER Capture Control Register controls each of the 4 capture registers. It selects their GPIO pin number for a trigger source, enables a capture operation and sets the input polarity for the capture. NOTE: 8-bit writes can control individual capture registers atomically."]
    #[inline(always)]
    pub const fn scapctrl3(&self) -> &Scapctrl3 {
        &self.scapctrl3
    }
    #[doc = "0x20 - The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur."]
    #[inline(always)]
    pub const fn scmpr0(&self) -> &Scmpr0 {
        &self.scmpr0
    }
    #[doc = "0x24 - The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur."]
    #[inline(always)]
    pub const fn scmpr1(&self) -> &Scmpr1 {
        &self.scmpr1
    }
    #[doc = "0x28 - The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur."]
    #[inline(always)]
    pub const fn scmpr2(&self) -> &Scmpr2 {
        &self.scmpr2
    }
    #[doc = "0x2c - The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur."]
    #[inline(always)]
    pub const fn scmpr3(&self) -> &Scmpr3 {
        &self.scmpr3
    }
    #[doc = "0x30 - The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur."]
    #[inline(always)]
    pub const fn scmpr4(&self) -> &Scmpr4 {
        &self.scmpr4
    }
    #[doc = "0x34 - The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur."]
    #[inline(always)]
    pub const fn scmpr5(&self) -> &Scmpr5 {
        &self.scmpr5
    }
    #[doc = "0x38 - The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur."]
    #[inline(always)]
    pub const fn scmpr6(&self) -> &Scmpr6 {
        &self.scmpr6
    }
    #[doc = "0x3c - The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur."]
    #[inline(always)]
    pub const fn scmpr7(&self) -> &Scmpr7 {
        &self.scmpr7
    }
    #[doc = "0x40 - The STIMER capture Register 0 captures the VALUE in the COUNTER register whenever capture condition (event) occurs. This register holds a time stamp for the event."]
    #[inline(always)]
    pub const fn scapt0(&self) -> &Scapt0 {
        &self.scapt0
    }
    #[doc = "0x44 - The STIMER capture Register 1 captures the VALUE in the COUNTER register whenever capture condition (event) occurs. This register holds a time stamp for the event."]
    #[inline(always)]
    pub const fn scapt1(&self) -> &Scapt1 {
        &self.scapt1
    }
    #[doc = "0x48 - The STIMER capture Register 2 captures the VALUE in the COUNTER register whenever capture condition (event) occurs. This register holds a time stamp for the event."]
    #[inline(always)]
    pub const fn scapt2(&self) -> &Scapt2 {
        &self.scapt2
    }
    #[doc = "0x4c - The STIMER capture Register 3 captures the VALUE in the COUNTER register whenever capture condition (event) occurs. This register holds a time stamp for the event."]
    #[inline(always)]
    pub const fn scapt3(&self) -> &Scapt3 {
        &self.scapt3
    }
    #[doc = "0x50 - The SNVR0 Register contains a portion of the stored epoch offset associated with the time in the COUNTER register. This register is only reset by POI not by HRESETn. Its contents are intended to survive all reset level except POI and full power cycles."]
    #[inline(always)]
    pub const fn snvr0(&self) -> &Snvr0 {
        &self.snvr0
    }
    #[doc = "0x54 - The SNVR1 Register contains a portion of the stored epoch offset associated with the time in the COUNTER register. This register is only reset by POI not by HRESETn. Its contents are intended to survive all reset level except POI and full power cycles."]
    #[inline(always)]
    pub const fn snvr1(&self) -> &Snvr1 {
        &self.snvr1
    }
    #[doc = "0x58 - The SNVR2 Register contains a portion of the stored epoch offset associated with the time in the COUNTER register. This register is only reset by POI not by HRESETn. Its contents are intended to survive all reset level except POI and full power cycles."]
    #[inline(always)]
    pub const fn snvr2(&self) -> &Snvr2 {
        &self.snvr2
    }
    #[doc = "0x100 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn stminten(&self) -> &Stminten {
        &self.stminten
    }
    #[doc = "0x104 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn stmintstat(&self) -> &Stmintstat {
        &self.stmintstat
    }
    #[doc = "0x108 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn stmintclr(&self) -> &Stmintclr {
        &self.stmintclr
    }
    #[doc = "0x10c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn stmintset(&self) -> &Stmintset {
        &self.stmintset
    }
}
#[doc = "STCFG (rw) register accessor: The STIMER Configuration Register contains the software control for selecting the clock divider and source feeding the system timer.\n\nYou can [`read`](crate::Reg::read) this register and get [`stcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcfg`]
module"]
#[doc(alias = "STCFG")]
pub type Stcfg = crate::Reg<stcfg::StcfgSpec>;
#[doc = "The STIMER Configuration Register contains the software control for selecting the clock divider and source feeding the system timer."]
pub mod stcfg;
#[doc = "STTMR (rw) register accessor: The COUNTER Register contains the running count of time as maintained by incrementing for every rising clock edge of the clock source selected in the configuration register. It is this counter value that captured in the capture registers and it is this counter value that is compared against the various compare registers. This register cannot be written, but can be cleared to 0 for a deterministic value. Use the FREEZE bit will stop this counter from incrementing.\n\nYou can [`read`](crate::Reg::read) this register and get [`sttmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sttmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sttmr`]
module"]
#[doc(alias = "STTMR")]
pub type Sttmr = crate::Reg<sttmr::SttmrSpec>;
#[doc = "The COUNTER Register contains the running count of time as maintained by incrementing for every rising clock edge of the clock source selected in the configuration register. It is this counter value that captured in the capture registers and it is this counter value that is compared against the various compare registers. This register cannot be written, but can be cleared to 0 for a deterministic value. Use the FREEZE bit will stop this counter from incrementing."]
pub mod sttmr;
#[doc = "SCAPCTRL0 (rw) register accessor: The STIMER Capture Control Register controls each of the 4 capture registers. It selects their GPIO pin number for a trigger source, enables a capture operation and sets the input polarity for the capture. NOTE: 8-bit writes can control individual capture registers atomically.\n\nYou can [`read`](crate::Reg::read) this register and get [`scapctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scapctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scapctrl0`]
module"]
#[doc(alias = "SCAPCTRL0")]
pub type Scapctrl0 = crate::Reg<scapctrl0::Scapctrl0Spec>;
#[doc = "The STIMER Capture Control Register controls each of the 4 capture registers. It selects their GPIO pin number for a trigger source, enables a capture operation and sets the input polarity for the capture. NOTE: 8-bit writes can control individual capture registers atomically."]
pub mod scapctrl0;
#[doc = "SCAPCTRL1 (rw) register accessor: The STIMER Capture Control Register controls each of the 4 capture registers. It selects their GPIO pin number for a trigger source, enables a capture operation and sets the input polarity for the capture. NOTE: 8-bit writes can control individual capture registers atomically.\n\nYou can [`read`](crate::Reg::read) this register and get [`scapctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scapctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scapctrl1`]
module"]
#[doc(alias = "SCAPCTRL1")]
pub type Scapctrl1 = crate::Reg<scapctrl1::Scapctrl1Spec>;
#[doc = "The STIMER Capture Control Register controls each of the 4 capture registers. It selects their GPIO pin number for a trigger source, enables a capture operation and sets the input polarity for the capture. NOTE: 8-bit writes can control individual capture registers atomically."]
pub mod scapctrl1;
#[doc = "SCAPCTRL2 (rw) register accessor: The STIMER Capture Control Register controls each of the 4 capture registers. It selects their GPIO pin number for a trigger source, enables a capture operation and sets the input polarity for the capture. NOTE: 8-bit writes can control individual capture registers atomically.\n\nYou can [`read`](crate::Reg::read) this register and get [`scapctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scapctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scapctrl2`]
module"]
#[doc(alias = "SCAPCTRL2")]
pub type Scapctrl2 = crate::Reg<scapctrl2::Scapctrl2Spec>;
#[doc = "The STIMER Capture Control Register controls each of the 4 capture registers. It selects their GPIO pin number for a trigger source, enables a capture operation and sets the input polarity for the capture. NOTE: 8-bit writes can control individual capture registers atomically."]
pub mod scapctrl2;
#[doc = "SCAPCTRL3 (rw) register accessor: The STIMER Capture Control Register controls each of the 4 capture registers. It selects their GPIO pin number for a trigger source, enables a capture operation and sets the input polarity for the capture. NOTE: 8-bit writes can control individual capture registers atomically.\n\nYou can [`read`](crate::Reg::read) this register and get [`scapctrl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scapctrl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scapctrl3`]
module"]
#[doc(alias = "SCAPCTRL3")]
pub type Scapctrl3 = crate::Reg<scapctrl3::Scapctrl3Spec>;
#[doc = "The STIMER Capture Control Register controls each of the 4 capture registers. It selects their GPIO pin number for a trigger source, enables a capture operation and sets the input polarity for the capture. NOTE: 8-bit writes can control individual capture registers atomically."]
pub mod scapctrl3;
#[doc = "SCMPR0 (rw) register accessor: The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur.\n\nYou can [`read`](crate::Reg::read) this register and get [`scmpr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmpr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmpr0`]
module"]
#[doc(alias = "SCMPR0")]
pub type Scmpr0 = crate::Reg<scmpr0::Scmpr0Spec>;
#[doc = "The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur."]
pub mod scmpr0;
#[doc = "SCMPR1 (rw) register accessor: The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur.\n\nYou can [`read`](crate::Reg::read) this register and get [`scmpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmpr1`]
module"]
#[doc(alias = "SCMPR1")]
pub type Scmpr1 = crate::Reg<scmpr1::Scmpr1Spec>;
#[doc = "The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur."]
pub mod scmpr1;
#[doc = "SCMPR2 (rw) register accessor: The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur.\n\nYou can [`read`](crate::Reg::read) this register and get [`scmpr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmpr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmpr2`]
module"]
#[doc(alias = "SCMPR2")]
pub type Scmpr2 = crate::Reg<scmpr2::Scmpr2Spec>;
#[doc = "The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur."]
pub mod scmpr2;
#[doc = "SCMPR3 (rw) register accessor: The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur.\n\nYou can [`read`](crate::Reg::read) this register and get [`scmpr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmpr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmpr3`]
module"]
#[doc(alias = "SCMPR3")]
pub type Scmpr3 = crate::Reg<scmpr3::Scmpr3Spec>;
#[doc = "The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur."]
pub mod scmpr3;
#[doc = "SCMPR4 (rw) register accessor: The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur.\n\nYou can [`read`](crate::Reg::read) this register and get [`scmpr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmpr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmpr4`]
module"]
#[doc(alias = "SCMPR4")]
pub type Scmpr4 = crate::Reg<scmpr4::Scmpr4Spec>;
#[doc = "The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur."]
pub mod scmpr4;
#[doc = "SCMPR5 (rw) register accessor: The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur.\n\nYou can [`read`](crate::Reg::read) this register and get [`scmpr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmpr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmpr5`]
module"]
#[doc(alias = "SCMPR5")]
pub type Scmpr5 = crate::Reg<scmpr5::Scmpr5Spec>;
#[doc = "The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur."]
pub mod scmpr5;
#[doc = "SCMPR6 (rw) register accessor: The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur.\n\nYou can [`read`](crate::Reg::read) this register and get [`scmpr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmpr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmpr6`]
module"]
#[doc(alias = "SCMPR6")]
pub type Scmpr6 = crate::Reg<scmpr6::Scmpr6Spec>;
#[doc = "The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur."]
pub mod scmpr6;
#[doc = "SCMPR7 (rw) register accessor: The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur.\n\nYou can [`read`](crate::Reg::read) this register and get [`scmpr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmpr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmpr7`]
module"]
#[doc(alias = "SCMPR7")]
pub type Scmpr7 = crate::Reg<scmpr7::Scmpr7Spec>;
#[doc = "The VALUE in this bit field is used to compare against the VALUE in the COUNTER register. If the match criterion in the configuration register is met then a corresponding interrupt status bit is set. The match criterion is defined as COUNTER equal to COMPARE. To establish a desired value in this COMPARE register, write the number of ticks in the future to this register to indicate when to interrupt. The hardware does the addition to the COUNTER value in the STIMER clock domain so that the math is precise. Reading this register shows the COUNTER value at which this interrupt will occur."]
pub mod scmpr7;
#[doc = "SCAPT0 (rw) register accessor: The STIMER capture Register 0 captures the VALUE in the COUNTER register whenever capture condition (event) occurs. This register holds a time stamp for the event.\n\nYou can [`read`](crate::Reg::read) this register and get [`scapt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scapt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scapt0`]
module"]
#[doc(alias = "SCAPT0")]
pub type Scapt0 = crate::Reg<scapt0::Scapt0Spec>;
#[doc = "The STIMER capture Register 0 captures the VALUE in the COUNTER register whenever capture condition (event) occurs. This register holds a time stamp for the event."]
pub mod scapt0;
#[doc = "SCAPT1 (rw) register accessor: The STIMER capture Register 1 captures the VALUE in the COUNTER register whenever capture condition (event) occurs. This register holds a time stamp for the event.\n\nYou can [`read`](crate::Reg::read) this register and get [`scapt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scapt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scapt1`]
module"]
#[doc(alias = "SCAPT1")]
pub type Scapt1 = crate::Reg<scapt1::Scapt1Spec>;
#[doc = "The STIMER capture Register 1 captures the VALUE in the COUNTER register whenever capture condition (event) occurs. This register holds a time stamp for the event."]
pub mod scapt1;
#[doc = "SCAPT2 (rw) register accessor: The STIMER capture Register 2 captures the VALUE in the COUNTER register whenever capture condition (event) occurs. This register holds a time stamp for the event.\n\nYou can [`read`](crate::Reg::read) this register and get [`scapt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scapt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scapt2`]
module"]
#[doc(alias = "SCAPT2")]
pub type Scapt2 = crate::Reg<scapt2::Scapt2Spec>;
#[doc = "The STIMER capture Register 2 captures the VALUE in the COUNTER register whenever capture condition (event) occurs. This register holds a time stamp for the event."]
pub mod scapt2;
#[doc = "SCAPT3 (rw) register accessor: The STIMER capture Register 3 captures the VALUE in the COUNTER register whenever capture condition (event) occurs. This register holds a time stamp for the event.\n\nYou can [`read`](crate::Reg::read) this register and get [`scapt3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scapt3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scapt3`]
module"]
#[doc(alias = "SCAPT3")]
pub type Scapt3 = crate::Reg<scapt3::Scapt3Spec>;
#[doc = "The STIMER capture Register 3 captures the VALUE in the COUNTER register whenever capture condition (event) occurs. This register holds a time stamp for the event."]
pub mod scapt3;
#[doc = "SNVR0 (rw) register accessor: The SNVR0 Register contains a portion of the stored epoch offset associated with the time in the COUNTER register. This register is only reset by POI not by HRESETn. Its contents are intended to survive all reset level except POI and full power cycles.\n\nYou can [`read`](crate::Reg::read) this register and get [`snvr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snvr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snvr0`]
module"]
#[doc(alias = "SNVR0")]
pub type Snvr0 = crate::Reg<snvr0::Snvr0Spec>;
#[doc = "The SNVR0 Register contains a portion of the stored epoch offset associated with the time in the COUNTER register. This register is only reset by POI not by HRESETn. Its contents are intended to survive all reset level except POI and full power cycles."]
pub mod snvr0;
#[doc = "SNVR1 (rw) register accessor: The SNVR1 Register contains a portion of the stored epoch offset associated with the time in the COUNTER register. This register is only reset by POI not by HRESETn. Its contents are intended to survive all reset level except POI and full power cycles.\n\nYou can [`read`](crate::Reg::read) this register and get [`snvr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snvr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snvr1`]
module"]
#[doc(alias = "SNVR1")]
pub type Snvr1 = crate::Reg<snvr1::Snvr1Spec>;
#[doc = "The SNVR1 Register contains a portion of the stored epoch offset associated with the time in the COUNTER register. This register is only reset by POI not by HRESETn. Its contents are intended to survive all reset level except POI and full power cycles."]
pub mod snvr1;
#[doc = "SNVR2 (rw) register accessor: The SNVR2 Register contains a portion of the stored epoch offset associated with the time in the COUNTER register. This register is only reset by POI not by HRESETn. Its contents are intended to survive all reset level except POI and full power cycles.\n\nYou can [`read`](crate::Reg::read) this register and get [`snvr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snvr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snvr2`]
module"]
#[doc(alias = "SNVR2")]
pub type Snvr2 = crate::Reg<snvr2::Snvr2Spec>;
#[doc = "The SNVR2 Register contains a portion of the stored epoch offset associated with the time in the COUNTER register. This register is only reset by POI not by HRESETn. Its contents are intended to survive all reset level except POI and full power cycles."]
pub mod snvr2;
#[doc = "STMINTEN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`stminten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stminten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stminten`]
module"]
#[doc(alias = "STMINTEN")]
pub type Stminten = crate::Reg<stminten::StmintenSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod stminten;
#[doc = "STMINTSTAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`stmintstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stmintstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stmintstat`]
module"]
#[doc(alias = "STMINTSTAT")]
pub type Stmintstat = crate::Reg<stmintstat::StmintstatSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod stmintstat;
#[doc = "STMINTCLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`stmintclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stmintclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stmintclr`]
module"]
#[doc(alias = "STMINTCLR")]
pub type Stmintclr = crate::Reg<stmintclr::StmintclrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod stmintclr;
#[doc = "STMINTSET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`stmintset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stmintset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stmintset`]
module"]
#[doc(alias = "STMINTSET")]
pub type Stmintset = crate::Reg<stmintset::StmintsetSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod stmintset;
