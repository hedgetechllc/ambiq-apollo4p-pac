#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rxdata: Rxdata,
    rxchanid: Rxchanid,
    rxfifostatus: Rxfifostatus,
    rxfifosize: Rxfifosize,
    rxupperlimit: Rxupperlimit,
    _reserved5: [u8; 0x0c],
    txdata: Txdata,
    txchanid: Txchanid,
    txfifostatus: Txfifostatus,
    txfifosize: Txfifosize,
    txlowerlimit: Txlowerlimit,
    _reserved10: [u8; 0x0c],
    i2sdatacfg: I2sdatacfg,
    i2siocfg: I2siocfg,
    i2sctl: I2sctl,
    ipbirpt: Ipbirpt,
    ipcoreid: Ipcoreid,
    amqcfg: Amqcfg,
    _reserved16: [u8; 0xa8],
    clkcfg: Clkcfg,
    _reserved17: [u8; 0xfc],
    dmacfg: Dmacfg,
    rxdmatotcnt: Rxdmatotcnt,
    rxdmaaddr: Rxdmaaddr,
    rxdmastat: Rxdmastat,
    txdmatotcnt: Txdmatotcnt,
    txdmaaddr: Txdmaaddr,
    txdmastat: Txdmastat,
    _reserved24: [u8; 0x14],
    status: Status,
    _reserved25: [u8; 0xcc],
    inten: Inten,
    intstat: Intstat,
    intclr: Intclr,
    intset: Intset,
    _reserved29: [u8; 0xf0],
    i2sdbg: I2sdbg,
}
impl RegisterBlock {
    #[doc = "0x00 - Read only access to the i2S receive data"]
    #[inline(always)]
    pub const fn rxdata(&self) -> &Rxdata {
        &self.rxdata
    }
    #[doc = "0x04 - Read only received channel identification register"]
    #[inline(always)]
    pub const fn rxchanid(&self) -> &Rxchanid {
        &self.rxchanid
    }
    #[doc = "0x08 - Holds the number of samples currently in the receive FIFO, and the empty condition flag"]
    #[inline(always)]
    pub const fn rxfifostatus(&self) -> &Rxfifostatus {
        &self.rxfifostatus
    }
    #[doc = "0x0c - Holds the size of the receive FIFO in samples"]
    #[inline(always)]
    pub const fn rxfifosize(&self) -> &Rxfifosize {
        &self.rxfifosize
    }
    #[doc = "0x10 - The number of samples required to be in the RX FIFO before asserting the RX_FFi interrupt bit"]
    #[inline(always)]
    pub const fn rxupperlimit(&self) -> &Rxupperlimit {
        &self.rxupperlimit
    }
    #[doc = "0x20 - Write only register to hold the i2S sample to transmit via the write FIFO"]
    #[inline(always)]
    pub const fn txdata(&self) -> &Txdata {
        &self.txdata
    }
    #[doc = "0x24 - Channel ID used for the next audio sample to be written to the data transmission register"]
    #[inline(always)]
    pub const fn txchanid(&self) -> &Txchanid {
        &self.txchanid
    }
    #[doc = "0x28 - Holds the number of samples currently in the transmit FIFO, and the full condition flag"]
    #[inline(always)]
    pub const fn txfifostatus(&self) -> &Txfifostatus {
        &self.txfifostatus
    }
    #[doc = "0x2c - Holds the size of the transmit FIFO in samples"]
    #[inline(always)]
    pub const fn txfifosize(&self) -> &Txfifosize {
        &self.txfifosize
    }
    #[doc = "0x30 - Minimum number of samples have been reached in the transmit FIFO."]
    #[inline(always)]
    pub const fn txlowerlimit(&self) -> &Txlowerlimit {
        &self.txlowerlimit
    }
    #[doc = "0x40 - Specifies the data format of I2S sub frames"]
    #[inline(always)]
    pub const fn i2sdatacfg(&self) -> &I2sdatacfg {
        &self.i2sdatacfg
    }
    #[doc = "0x44 - Specified polarity and clock configuration of the I2S IPB clocks and IO signals"]
    #[inline(always)]
    pub const fn i2siocfg(&self) -> &I2siocfg {
        &self.i2siocfg
    }
    #[doc = "0x48 - Specified polarity and clock configuration of the I2S IPB clocks and IO signals"]
    #[inline(always)]
    pub const fn i2sctl(&self) -> &I2sctl {
        &self.i2sctl
    }
    #[doc = "0x4c - Additional mask and status registers for the IPB core."]
    #[inline(always)]
    pub const fn ipbirpt(&self) -> &Ipbirpt {
        &self.ipbirpt
    }
    #[doc = "0x50 - Returns the core ID of the IPB core, and used to write the I2S validity mask."]
    #[inline(always)]
    pub const fn ipcoreid(&self) -> &Ipcoreid {
        &self.ipcoreid
    }
    #[doc = "0x54 - Control the enablement of the ASRC module and the source of the MCLK used in the IPB core."]
    #[inline(always)]
    pub const fn amqcfg(&self) -> &Amqcfg {
        &self.amqcfg
    }
    #[doc = "0x100 - Provides clock selection and control for I2S clocks"]
    #[inline(always)]
    pub const fn clkcfg(&self) -> &Clkcfg {
        &self.clkcfg
    }
    #[doc = "0x200 - Configuration control of the DMA process, including the direction of DMA, and enablement of DMA"]
    #[inline(always)]
    pub const fn dmacfg(&self) -> &Dmacfg {
        &self.dmacfg
    }
    #[doc = "0x204 - Contains the total count of samples to be stored for the current RX DMA operation. This register is updated as DMA beats complete."]
    #[inline(always)]
    pub const fn rxdmatotcnt(&self) -> &Rxdmatotcnt {
        &self.rxdmatotcnt
    }
    #[doc = "0x208 - The address which the DMA operation will store the incoming audio samples. This address is updated as the samples are stored."]
    #[inline(always)]
    pub const fn rxdmaaddr(&self) -> &Rxdmaaddr {
        &self.rxdmaaddr
    }
    #[doc = "0x20c - Status of the RX DMA operation currently in progress."]
    #[inline(always)]
    pub const fn rxdmastat(&self) -> &Rxdmastat {
        &self.rxdmastat
    }
    #[doc = "0x210 - Contains the total count of samples to be read and transmitted for the current TX DMA operation. This register is updated as DMA beats complete."]
    #[inline(always)]
    pub const fn txdmatotcnt(&self) -> &Txdmatotcnt {
        &self.txdmatotcnt
    }
    #[doc = "0x214 - The address which the DMA operation will fetch the audio samples. This address is updated as the samples are stored."]
    #[inline(always)]
    pub const fn txdmaaddr(&self) -> &Txdmaaddr {
        &self.txdmaaddr
    }
    #[doc = "0x218 - Status of the TX DMA operation currently in progress."]
    #[inline(always)]
    pub const fn txdmastat(&self) -> &Txdmastat {
        &self.txdmastat
    }
    #[doc = "0x230 - I2S Module Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x300 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x304 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0x308 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn intclr(&self) -> &Intclr {
        &self.intclr
    }
    #[doc = "0x30c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn intset(&self) -> &Intset {
        &self.intset
    }
    #[doc = "0x400 - Debug control"]
    #[inline(always)]
    pub const fn i2sdbg(&self) -> &I2sdbg {
        &self.i2sdbg
    }
}
#[doc = "RXDATA (rw) register accessor: Read only access to the i2S receive data\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata`]
module"]
#[doc(alias = "RXDATA")]
pub type Rxdata = crate::Reg<rxdata::RxdataSpec>;
#[doc = "Read only access to the i2S receive data"]
pub mod rxdata;
#[doc = "RXCHANID (rw) register accessor: Read only received channel identification register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxchanid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxchanid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxchanid`]
module"]
#[doc(alias = "RXCHANID")]
pub type Rxchanid = crate::Reg<rxchanid::RxchanidSpec>;
#[doc = "Read only received channel identification register"]
pub mod rxchanid;
#[doc = "RXFIFOSTATUS (rw) register accessor: Holds the number of samples currently in the receive FIFO, and the empty condition flag\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfifostatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxfifostatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfifostatus`]
module"]
#[doc(alias = "RXFIFOSTATUS")]
pub type Rxfifostatus = crate::Reg<rxfifostatus::RxfifostatusSpec>;
#[doc = "Holds the number of samples currently in the receive FIFO, and the empty condition flag"]
pub mod rxfifostatus;
#[doc = "RXFIFOSIZE (rw) register accessor: Holds the size of the receive FIFO in samples\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfifosize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxfifosize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfifosize`]
module"]
#[doc(alias = "RXFIFOSIZE")]
pub type Rxfifosize = crate::Reg<rxfifosize::RxfifosizeSpec>;
#[doc = "Holds the size of the receive FIFO in samples"]
pub mod rxfifosize;
#[doc = "RXUPPERLIMIT (rw) register accessor: The number of samples required to be in the RX FIFO before asserting the RX_FFi interrupt bit\n\nYou can [`read`](crate::Reg::read) this register and get [`rxupperlimit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxupperlimit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxupperlimit`]
module"]
#[doc(alias = "RXUPPERLIMIT")]
pub type Rxupperlimit = crate::Reg<rxupperlimit::RxupperlimitSpec>;
#[doc = "The number of samples required to be in the RX FIFO before asserting the RX_FFi interrupt bit"]
pub mod rxupperlimit;
#[doc = "TXDATA (rw) register accessor: Write only register to hold the i2S sample to transmit via the write FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`]
module"]
#[doc(alias = "TXDATA")]
pub type Txdata = crate::Reg<txdata::TxdataSpec>;
#[doc = "Write only register to hold the i2S sample to transmit via the write FIFO"]
pub mod txdata;
#[doc = "TXCHANID (rw) register accessor: Channel ID used for the next audio sample to be written to the data transmission register\n\nYou can [`read`](crate::Reg::read) this register and get [`txchanid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txchanid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txchanid`]
module"]
#[doc(alias = "TXCHANID")]
pub type Txchanid = crate::Reg<txchanid::TxchanidSpec>;
#[doc = "Channel ID used for the next audio sample to be written to the data transmission register"]
pub mod txchanid;
#[doc = "TXFIFOSTATUS (rw) register accessor: Holds the number of samples currently in the transmit FIFO, and the full condition flag\n\nYou can [`read`](crate::Reg::read) this register and get [`txfifostatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txfifostatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfifostatus`]
module"]
#[doc(alias = "TXFIFOSTATUS")]
pub type Txfifostatus = crate::Reg<txfifostatus::TxfifostatusSpec>;
#[doc = "Holds the number of samples currently in the transmit FIFO, and the full condition flag"]
pub mod txfifostatus;
#[doc = "TXFIFOSIZE (rw) register accessor: Holds the size of the transmit FIFO in samples\n\nYou can [`read`](crate::Reg::read) this register and get [`txfifosize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txfifosize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfifosize`]
module"]
#[doc(alias = "TXFIFOSIZE")]
pub type Txfifosize = crate::Reg<txfifosize::TxfifosizeSpec>;
#[doc = "Holds the size of the transmit FIFO in samples"]
pub mod txfifosize;
#[doc = "TXLOWERLIMIT (rw) register accessor: Minimum number of samples have been reached in the transmit FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`txlowerlimit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txlowerlimit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txlowerlimit`]
module"]
#[doc(alias = "TXLOWERLIMIT")]
pub type Txlowerlimit = crate::Reg<txlowerlimit::TxlowerlimitSpec>;
#[doc = "Minimum number of samples have been reached in the transmit FIFO."]
pub mod txlowerlimit;
#[doc = "I2SDATACFG (rw) register accessor: Specifies the data format of I2S sub frames\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sdatacfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sdatacfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sdatacfg`]
module"]
#[doc(alias = "I2SDATACFG")]
pub type I2sdatacfg = crate::Reg<i2sdatacfg::I2sdatacfgSpec>;
#[doc = "Specifies the data format of I2S sub frames"]
pub mod i2sdatacfg;
#[doc = "I2SIOCFG (rw) register accessor: Specified polarity and clock configuration of the I2S IPB clocks and IO signals\n\nYou can [`read`](crate::Reg::read) this register and get [`i2siocfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2siocfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2siocfg`]
module"]
#[doc(alias = "I2SIOCFG")]
pub type I2siocfg = crate::Reg<i2siocfg::I2siocfgSpec>;
#[doc = "Specified polarity and clock configuration of the I2S IPB clocks and IO signals"]
pub mod i2siocfg;
#[doc = "I2SCTL (rw) register accessor: Specified polarity and clock configuration of the I2S IPB clocks and IO signals\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sctl`]
module"]
#[doc(alias = "I2SCTL")]
pub type I2sctl = crate::Reg<i2sctl::I2sctlSpec>;
#[doc = "Specified polarity and clock configuration of the I2S IPB clocks and IO signals"]
pub mod i2sctl;
#[doc = "IPBIRPT (rw) register accessor: Additional mask and status registers for the IPB core.\n\nYou can [`read`](crate::Reg::read) this register and get [`ipbirpt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipbirpt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipbirpt`]
module"]
#[doc(alias = "IPBIRPT")]
pub type Ipbirpt = crate::Reg<ipbirpt::IpbirptSpec>;
#[doc = "Additional mask and status registers for the IPB core."]
pub mod ipbirpt;
#[doc = "IPCOREID (rw) register accessor: Returns the core ID of the IPB core, and used to write the I2S validity mask.\n\nYou can [`read`](crate::Reg::read) this register and get [`ipcoreid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipcoreid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipcoreid`]
module"]
#[doc(alias = "IPCOREID")]
pub type Ipcoreid = crate::Reg<ipcoreid::IpcoreidSpec>;
#[doc = "Returns the core ID of the IPB core, and used to write the I2S validity mask."]
pub mod ipcoreid;
#[doc = "AMQCFG (rw) register accessor: Control the enablement of the ASRC module and the source of the MCLK used in the IPB core.\n\nYou can [`read`](crate::Reg::read) this register and get [`amqcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amqcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amqcfg`]
module"]
#[doc(alias = "AMQCFG")]
pub type Amqcfg = crate::Reg<amqcfg::AmqcfgSpec>;
#[doc = "Control the enablement of the ASRC module and the source of the MCLK used in the IPB core."]
pub mod amqcfg;
#[doc = "CLKCFG (rw) register accessor: Provides clock selection and control for I2S clocks\n\nYou can [`read`](crate::Reg::read) this register and get [`clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcfg`]
module"]
#[doc(alias = "CLKCFG")]
pub type Clkcfg = crate::Reg<clkcfg::ClkcfgSpec>;
#[doc = "Provides clock selection and control for I2S clocks"]
pub mod clkcfg;
#[doc = "DMACFG (rw) register accessor: Configuration control of the DMA process, including the direction of DMA, and enablement of DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacfg`]
module"]
#[doc(alias = "DMACFG")]
pub type Dmacfg = crate::Reg<dmacfg::DmacfgSpec>;
#[doc = "Configuration control of the DMA process, including the direction of DMA, and enablement of DMA"]
pub mod dmacfg;
#[doc = "RXDMATOTCNT (rw) register accessor: Contains the total count of samples to be stored for the current RX DMA operation. This register is updated as DMA beats complete.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdmatotcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdmatotcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdmatotcnt`]
module"]
#[doc(alias = "RXDMATOTCNT")]
pub type Rxdmatotcnt = crate::Reg<rxdmatotcnt::RxdmatotcntSpec>;
#[doc = "Contains the total count of samples to be stored for the current RX DMA operation. This register is updated as DMA beats complete."]
pub mod rxdmatotcnt;
#[doc = "RXDMAADDR (rw) register accessor: The address which the DMA operation will store the incoming audio samples. This address is updated as the samples are stored.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdmaaddr`]
module"]
#[doc(alias = "RXDMAADDR")]
pub type Rxdmaaddr = crate::Reg<rxdmaaddr::RxdmaaddrSpec>;
#[doc = "The address which the DMA operation will store the incoming audio samples. This address is updated as the samples are stored."]
pub mod rxdmaaddr;
#[doc = "RXDMASTAT (rw) register accessor: Status of the RX DMA operation currently in progress.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdmastat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdmastat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdmastat`]
module"]
#[doc(alias = "RXDMASTAT")]
pub type Rxdmastat = crate::Reg<rxdmastat::RxdmastatSpec>;
#[doc = "Status of the RX DMA operation currently in progress."]
pub mod rxdmastat;
#[doc = "TXDMATOTCNT (rw) register accessor: Contains the total count of samples to be read and transmitted for the current TX DMA operation. This register is updated as DMA beats complete.\n\nYou can [`read`](crate::Reg::read) this register and get [`txdmatotcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdmatotcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdmatotcnt`]
module"]
#[doc(alias = "TXDMATOTCNT")]
pub type Txdmatotcnt = crate::Reg<txdmatotcnt::TxdmatotcntSpec>;
#[doc = "Contains the total count of samples to be read and transmitted for the current TX DMA operation. This register is updated as DMA beats complete."]
pub mod txdmatotcnt;
#[doc = "TXDMAADDR (rw) register accessor: The address which the DMA operation will fetch the audio samples. This address is updated as the samples are stored.\n\nYou can [`read`](crate::Reg::read) this register and get [`txdmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdmaaddr`]
module"]
#[doc(alias = "TXDMAADDR")]
pub type Txdmaaddr = crate::Reg<txdmaaddr::TxdmaaddrSpec>;
#[doc = "The address which the DMA operation will fetch the audio samples. This address is updated as the samples are stored."]
pub mod txdmaaddr;
#[doc = "TXDMASTAT (rw) register accessor: Status of the TX DMA operation currently in progress.\n\nYou can [`read`](crate::Reg::read) this register and get [`txdmastat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdmastat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdmastat`]
module"]
#[doc(alias = "TXDMASTAT")]
pub type Txdmastat = crate::Reg<txdmastat::TxdmastatSpec>;
#[doc = "Status of the TX DMA operation currently in progress."]
pub mod txdmastat;
#[doc = "STATUS (rw) register accessor: I2S Module Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "I2S Module Status"]
pub mod status;
#[doc = "INTEN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod inten;
#[doc = "INTSTAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`]
module"]
#[doc(alias = "INTSTAT")]
pub type Intstat = crate::Reg<intstat::IntstatSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod intstat;
#[doc = "INTCLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`intclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclr`]
module"]
#[doc(alias = "INTCLR")]
pub type Intclr = crate::Reg<intclr::IntclrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod intclr;
#[doc = "INTSET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`intset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intset`]
module"]
#[doc(alias = "INTSET")]
pub type Intset = crate::Reg<intset::IntsetSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod intset;
#[doc = "I2SDBG (rw) register accessor: Debug control\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sdbg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sdbg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sdbg`]
module"]
#[doc(alias = "I2SDBG")]
pub type I2sdbg = crate::Reg<i2sdbg::I2sdbgSpec>;
#[doc = "Debug control"]
pub mod i2sdbg;
