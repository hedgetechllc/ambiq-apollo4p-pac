#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sdma: Sdma,
    block: Block,
    argument1: Argument1,
    transfer: Transfer,
    response0: Response0,
    response1: Response1,
    response2: Response2,
    response3: Response3,
    buffer: Buffer,
    present: Present,
    hostctrl1: Hostctrl1,
    clockctrl: Clockctrl,
    intstat: Intstat,
    intenable: Intenable,
    intsig: Intsig,
    auto: Auto,
    capabilities0: Capabilities0,
    capabilities1: Capabilities1,
    maximum0: Maximum0,
    maximum1: Maximum1,
    force: Force,
    adma: Adma,
    admalowd: Admalowd,
    admahiwd: Admahiwd,
    preset0: Preset0,
    preset1: Preset1,
    preset2: Preset2,
    preset3: Preset3,
    boottoctrl: Boottoctrl,
    _reserved29: [u8; 0x04],
    vendor: Vendor,
    _reserved30: [u8; 0x80],
    slotstat: Slotstat,
}
impl RegisterBlock {
    #[doc = "0x00 - SDMA system address"]
    #[inline(always)]
    pub const fn sdma(&self) -> &Sdma {
        &self.sdma
    }
    #[doc = "0x04 - Block size"]
    #[inline(always)]
    pub const fn block(&self) -> &Block {
        &self.block
    }
    #[doc = "0x08 - Argument1"]
    #[inline(always)]
    pub const fn argument1(&self) -> &Argument1 {
        &self.argument1
    }
    #[doc = "0x0c - Transfer mode"]
    #[inline(always)]
    pub const fn transfer(&self) -> &Transfer {
        &self.transfer
    }
    #[doc = "0x10 - Response0"]
    #[inline(always)]
    pub const fn response0(&self) -> &Response0 {
        &self.response0
    }
    #[doc = "0x14 - Response1"]
    #[inline(always)]
    pub const fn response1(&self) -> &Response1 {
        &self.response1
    }
    #[doc = "0x18 - Response2"]
    #[inline(always)]
    pub const fn response2(&self) -> &Response2 {
        &self.response2
    }
    #[doc = "0x1c - Response3"]
    #[inline(always)]
    pub const fn response3(&self) -> &Response3 {
        &self.response3
    }
    #[doc = "0x20 - Buffer data port"]
    #[inline(always)]
    pub const fn buffer(&self) -> &Buffer {
        &self.buffer
    }
    #[doc = "0x24 - Present state"]
    #[inline(always)]
    pub const fn present(&self) -> &Present {
        &self.present
    }
    #[doc = "0x28 - Host control 1"]
    #[inline(always)]
    pub const fn hostctrl1(&self) -> &Hostctrl1 {
        &self.hostctrl1
    }
    #[doc = "0x2c - Clock control"]
    #[inline(always)]
    pub const fn clockctrl(&self) -> &Clockctrl {
        &self.clockctrl
    }
    #[doc = "0x30 - Interrupt enable"]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0x34 - Normal interrupt status enable"]
    #[inline(always)]
    pub const fn intenable(&self) -> &Intenable {
        &self.intenable
    }
    #[doc = "0x38 - Normal interrupt signal enable"]
    #[inline(always)]
    pub const fn intsig(&self) -> &Intsig {
        &self.intsig
    }
    #[doc = "0x3c - Auto CMD error status"]
    #[inline(always)]
    pub const fn auto(&self) -> &Auto {
        &self.auto
    }
    #[doc = "0x40 - Capabilities"]
    #[inline(always)]
    pub const fn capabilities0(&self) -> &Capabilities0 {
        &self.capabilities0
    }
    #[doc = "0x44 - Capabilities"]
    #[inline(always)]
    pub const fn capabilities1(&self) -> &Capabilities1 {
        &self.capabilities1
    }
    #[doc = "0x48 - Maximum current capabilities"]
    #[inline(always)]
    pub const fn maximum0(&self) -> &Maximum0 {
        &self.maximum0
    }
    #[doc = "0x4c - Maximum current capabilities"]
    #[inline(always)]
    pub const fn maximum1(&self) -> &Maximum1 {
        &self.maximum1
    }
    #[doc = "0x50 - Force event register for error interrupt status"]
    #[inline(always)]
    pub const fn force(&self) -> &Force {
        &self.force
    }
    #[doc = "0x54 - ADMA error status"]
    #[inline(always)]
    pub const fn adma(&self) -> &Adma {
        &self.adma
    }
    #[doc = "0x58 - ADMA system address \\[31:0\\]"]
    #[inline(always)]
    pub const fn admalowd(&self) -> &Admalowd {
        &self.admalowd
    }
    #[doc = "0x5c - ADMA system address \\[63:0\\]"]
    #[inline(always)]
    pub const fn admahiwd(&self) -> &Admahiwd {
        &self.admahiwd
    }
    #[doc = "0x60 - Preset Value initialization and default speed"]
    #[inline(always)]
    pub const fn preset0(&self) -> &Preset0 {
        &self.preset0
    }
    #[doc = "0x64 - Preset Value for high speed and SDR12"]
    #[inline(always)]
    pub const fn preset1(&self) -> &Preset1 {
        &self.preset1
    }
    #[doc = "0x68 - Preset Value for SDR25 and SDR50"]
    #[inline(always)]
    pub const fn preset2(&self) -> &Preset2 {
        &self.preset2
    }
    #[doc = "0x6c - Preset Value for SDR104 and DDR50"]
    #[inline(always)]
    pub const fn preset3(&self) -> &Preset3 {
        &self.preset3
    }
    #[doc = "0x70 - Boot Data Timeout control"]
    #[inline(always)]
    pub const fn boottoctrl(&self) -> &Boottoctrl {
        &self.boottoctrl
    }
    #[doc = "0x78 - Vendor"]
    #[inline(always)]
    pub const fn vendor(&self) -> &Vendor {
        &self.vendor
    }
    #[doc = "0xfc - Slot interrupt status"]
    #[inline(always)]
    pub const fn slotstat(&self) -> &Slotstat {
        &self.slotstat
    }
}
#[doc = "SDMA (rw) register accessor: SDMA system address\n\nYou can [`read`](crate::Reg::read) this register and get [`sdma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdma`]
module"]
#[doc(alias = "SDMA")]
pub type Sdma = crate::Reg<sdma::SdmaSpec>;
#[doc = "SDMA system address"]
pub mod sdma;
#[doc = "BLOCK (rw) register accessor: Block size\n\nYou can [`read`](crate::Reg::read) this register and get [`block::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`block::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@block`]
module"]
#[doc(alias = "BLOCK")]
pub type Block = crate::Reg<block::BlockSpec>;
#[doc = "Block size"]
pub mod block;
#[doc = "ARGUMENT1 (rw) register accessor: Argument1\n\nYou can [`read`](crate::Reg::read) this register and get [`argument1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`argument1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@argument1`]
module"]
#[doc(alias = "ARGUMENT1")]
pub type Argument1 = crate::Reg<argument1::Argument1Spec>;
#[doc = "Argument1"]
pub mod argument1;
#[doc = "TRANSFER (rw) register accessor: Transfer mode\n\nYou can [`read`](crate::Reg::read) this register and get [`transfer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`transfer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transfer`]
module"]
#[doc(alias = "TRANSFER")]
pub type Transfer = crate::Reg<transfer::TransferSpec>;
#[doc = "Transfer mode"]
pub mod transfer;
#[doc = "RESPONSE0 (rw) register accessor: Response0\n\nYou can [`read`](crate::Reg::read) this register and get [`response0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`response0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@response0`]
module"]
#[doc(alias = "RESPONSE0")]
pub type Response0 = crate::Reg<response0::Response0Spec>;
#[doc = "Response0"]
pub mod response0;
#[doc = "RESPONSE1 (rw) register accessor: Response1\n\nYou can [`read`](crate::Reg::read) this register and get [`response1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`response1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@response1`]
module"]
#[doc(alias = "RESPONSE1")]
pub type Response1 = crate::Reg<response1::Response1Spec>;
#[doc = "Response1"]
pub mod response1;
#[doc = "RESPONSE2 (rw) register accessor: Response2\n\nYou can [`read`](crate::Reg::read) this register and get [`response2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`response2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@response2`]
module"]
#[doc(alias = "RESPONSE2")]
pub type Response2 = crate::Reg<response2::Response2Spec>;
#[doc = "Response2"]
pub mod response2;
#[doc = "RESPONSE3 (rw) register accessor: Response3\n\nYou can [`read`](crate::Reg::read) this register and get [`response3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`response3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@response3`]
module"]
#[doc(alias = "RESPONSE3")]
pub type Response3 = crate::Reg<response3::Response3Spec>;
#[doc = "Response3"]
pub mod response3;
#[doc = "BUFFER (rw) register accessor: Buffer data port\n\nYou can [`read`](crate::Reg::read) this register and get [`buffer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buffer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buffer`]
module"]
#[doc(alias = "BUFFER")]
pub type Buffer = crate::Reg<buffer::BufferSpec>;
#[doc = "Buffer data port"]
pub mod buffer;
#[doc = "PRESENT (rw) register accessor: Present state\n\nYou can [`read`](crate::Reg::read) this register and get [`present::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`present::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@present`]
module"]
#[doc(alias = "PRESENT")]
pub type Present = crate::Reg<present::PresentSpec>;
#[doc = "Present state"]
pub mod present;
#[doc = "HOSTCTRL1 (rw) register accessor: Host control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hostctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostctrl1`]
module"]
#[doc(alias = "HOSTCTRL1")]
pub type Hostctrl1 = crate::Reg<hostctrl1::Hostctrl1Spec>;
#[doc = "Host control 1"]
pub mod hostctrl1;
#[doc = "CLOCKCTRL (rw) register accessor: Clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`clockctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clockctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clockctrl`]
module"]
#[doc(alias = "CLOCKCTRL")]
pub type Clockctrl = crate::Reg<clockctrl::ClockctrlSpec>;
#[doc = "Clock control"]
pub mod clockctrl;
#[doc = "INTSTAT (rw) register accessor: Interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`]
module"]
#[doc(alias = "INTSTAT")]
pub type Intstat = crate::Reg<intstat::IntstatSpec>;
#[doc = "Interrupt enable"]
pub mod intstat;
#[doc = "INTENABLE (rw) register accessor: Normal interrupt status enable\n\nYou can [`read`](crate::Reg::read) this register and get [`intenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenable`]
module"]
#[doc(alias = "INTENABLE")]
pub type Intenable = crate::Reg<intenable::IntenableSpec>;
#[doc = "Normal interrupt status enable"]
pub mod intenable;
#[doc = "INTSIG (rw) register accessor: Normal interrupt signal enable\n\nYou can [`read`](crate::Reg::read) this register and get [`intsig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsig`]
module"]
#[doc(alias = "INTSIG")]
pub type Intsig = crate::Reg<intsig::IntsigSpec>;
#[doc = "Normal interrupt signal enable"]
pub mod intsig;
#[doc = "AUTO (rw) register accessor: Auto CMD error status\n\nYou can [`read`](crate::Reg::read) this register and get [`auto::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`auto::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auto`]
module"]
#[doc(alias = "AUTO")]
pub type Auto = crate::Reg<auto::AutoSpec>;
#[doc = "Auto CMD error status"]
pub mod auto;
#[doc = "CAPABILITIES0 (rw) register accessor: Capabilities\n\nYou can [`read`](crate::Reg::read) this register and get [`capabilities0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capabilities0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capabilities0`]
module"]
#[doc(alias = "CAPABILITIES0")]
pub type Capabilities0 = crate::Reg<capabilities0::Capabilities0Spec>;
#[doc = "Capabilities"]
pub mod capabilities0;
#[doc = "CAPABILITIES1 (rw) register accessor: Capabilities\n\nYou can [`read`](crate::Reg::read) this register and get [`capabilities1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capabilities1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capabilities1`]
module"]
#[doc(alias = "CAPABILITIES1")]
pub type Capabilities1 = crate::Reg<capabilities1::Capabilities1Spec>;
#[doc = "Capabilities"]
pub mod capabilities1;
#[doc = "MAXIMUM0 (rw) register accessor: Maximum current capabilities\n\nYou can [`read`](crate::Reg::read) this register and get [`maximum0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maximum0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maximum0`]
module"]
#[doc(alias = "MAXIMUM0")]
pub type Maximum0 = crate::Reg<maximum0::Maximum0Spec>;
#[doc = "Maximum current capabilities"]
pub mod maximum0;
#[doc = "MAXIMUM1 (rw) register accessor: Maximum current capabilities\n\nYou can [`read`](crate::Reg::read) this register and get [`maximum1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maximum1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maximum1`]
module"]
#[doc(alias = "MAXIMUM1")]
pub type Maximum1 = crate::Reg<maximum1::Maximum1Spec>;
#[doc = "Maximum current capabilities"]
pub mod maximum1;
#[doc = "FORCE (rw) register accessor: Force event register for error interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`force::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@force`]
module"]
#[doc(alias = "FORCE")]
pub type Force = crate::Reg<force::ForceSpec>;
#[doc = "Force event register for error interrupt status"]
pub mod force;
#[doc = "ADMA (rw) register accessor: ADMA error status\n\nYou can [`read`](crate::Reg::read) this register and get [`adma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adma`]
module"]
#[doc(alias = "ADMA")]
pub type Adma = crate::Reg<adma::AdmaSpec>;
#[doc = "ADMA error status"]
pub mod adma;
#[doc = "ADMALOWD (rw) register accessor: ADMA system address \\[31:0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`admalowd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`admalowd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@admalowd`]
module"]
#[doc(alias = "ADMALOWD")]
pub type Admalowd = crate::Reg<admalowd::AdmalowdSpec>;
#[doc = "ADMA system address \\[31:0\\]"]
pub mod admalowd;
#[doc = "ADMAHIWD (rw) register accessor: ADMA system address \\[63:0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`admahiwd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`admahiwd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@admahiwd`]
module"]
#[doc(alias = "ADMAHIWD")]
pub type Admahiwd = crate::Reg<admahiwd::AdmahiwdSpec>;
#[doc = "ADMA system address \\[63:0\\]"]
pub mod admahiwd;
#[doc = "PRESET0 (rw) register accessor: Preset Value initialization and default speed\n\nYou can [`read`](crate::Reg::read) this register and get [`preset0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`preset0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preset0`]
module"]
#[doc(alias = "PRESET0")]
pub type Preset0 = crate::Reg<preset0::Preset0Spec>;
#[doc = "Preset Value initialization and default speed"]
pub mod preset0;
#[doc = "PRESET1 (rw) register accessor: Preset Value for high speed and SDR12\n\nYou can [`read`](crate::Reg::read) this register and get [`preset1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`preset1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preset1`]
module"]
#[doc(alias = "PRESET1")]
pub type Preset1 = crate::Reg<preset1::Preset1Spec>;
#[doc = "Preset Value for high speed and SDR12"]
pub mod preset1;
#[doc = "PRESET2 (rw) register accessor: Preset Value for SDR25 and SDR50\n\nYou can [`read`](crate::Reg::read) this register and get [`preset2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`preset2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preset2`]
module"]
#[doc(alias = "PRESET2")]
pub type Preset2 = crate::Reg<preset2::Preset2Spec>;
#[doc = "Preset Value for SDR25 and SDR50"]
pub mod preset2;
#[doc = "PRESET3 (rw) register accessor: Preset Value for SDR104 and DDR50\n\nYou can [`read`](crate::Reg::read) this register and get [`preset3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`preset3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preset3`]
module"]
#[doc(alias = "PRESET3")]
pub type Preset3 = crate::Reg<preset3::Preset3Spec>;
#[doc = "Preset Value for SDR104 and DDR50"]
pub mod preset3;
#[doc = "BOOTTOCTRL (rw) register accessor: Boot Data Timeout control\n\nYou can [`read`](crate::Reg::read) this register and get [`boottoctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boottoctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boottoctrl`]
module"]
#[doc(alias = "BOOTTOCTRL")]
pub type Boottoctrl = crate::Reg<boottoctrl::BoottoctrlSpec>;
#[doc = "Boot Data Timeout control"]
pub mod boottoctrl;
#[doc = "VENDOR (rw) register accessor: Vendor\n\nYou can [`read`](crate::Reg::read) this register and get [`vendor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vendor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vendor`]
module"]
#[doc(alias = "VENDOR")]
pub type Vendor = crate::Reg<vendor::VendorSpec>;
#[doc = "Vendor"]
pub mod vendor;
#[doc = "SLOTSTAT (rw) register accessor: Slot interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`slotstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slotstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slotstat`]
module"]
#[doc(alias = "SLOTSTAT")]
pub type Slotstat = crate::Reg<slotstat::SlotstatSpec>;
#[doc = "Slot interrupt status"]
pub mod slotstat;
