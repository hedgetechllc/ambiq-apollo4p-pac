#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg0: Cfg0,
    cfg1: Cfg1,
    cfg2: Cfg2,
    cfg3: Cfg3,
    idx0: Idx0,
    idx1: Idx1,
    idx2: Idx2,
    fifoadd: Fifoadd,
    fifo0: Fifo0,
    fifo1: Fifo1,
    fifo2: Fifo2,
    fifo3: Fifo3,
    fifo4: Fifo4,
    fifo5: Fifo5,
    _reserved14: [u8; 0x34],
    hwvers: Hwvers,
    _reserved15: [u8; 0x08],
    info: Info,
    _reserved16: [u8; 0x04],
    timeout1: Timeout1,
    timeout2: Timeout2,
    _reserved18: [u8; 0x1f78],
    clkctrl: Clkctrl,
    sramctrl: Sramctrl,
    _reserved20: [u8; 0x0c],
    utmistickystatus: Utmistickystatus,
    obsclrstat: Obsclrstat,
    dpdmpulldown: Dpdmpulldown,
    bcdetstatus: Bcdetstatus,
    bcdetcrtl1: Bcdetcrtl1,
    bcdetcrtl2: Bcdetcrtl2,
}
impl RegisterBlock {
    #[doc = "0x00 - Function address, power management, interrupt status register for EP0 and IN Endpoints 1 to 5"]
    #[inline(always)]
    pub const fn cfg0(&self) -> &Cfg0 {
        &self.cfg0
    }
    #[doc = "0x04 - Indicates which of the IN Endpoint 1 - 5 interrupts and the single Endpoint 0 interrupt are currently active. Also indicates which of the interrupts for OUT Endpoint 1 - 5 are currently active. All active interrupts are cleared when this register is read."]
    #[inline(always)]
    pub const fn cfg1(&self) -> &Cfg1 {
        &self.cfg1
    }
    #[doc = "0x08 - Provides interrupt enable and (currently active) status bits for each of the state interrupts, as well as the IN Endpoint and OUT Endpoint nterrupts. All active interrupts are cleared when this register is read. On reset, all IN and OUT Endpoint interrupts, in addition to Endpoint 0, are set to 1 while the remaining bits are set to 0."]
    #[inline(always)]
    pub const fn cfg2(&self) -> &Cfg2 {
        &self.cfg2
    }
    #[doc = "0x0c - Provides Test fields to put the USB Controller into one of four test modes described in the USB 2.0 specification. Only one of the Test fields should be set at any time. (Not used in normal operation.) Also includes an index field that determines which endpoint control,status registers are accessed via the IDXn register fields, and a Frame field that holds the last received frame number."]
    #[inline(always)]
    pub const fn cfg3(&self) -> &Cfg3 {
        &self.cfg3
    }
    #[doc = "0x10 - Provides additional control and status for IN transactions through the currently-selected endpoint. (To avoid CMSIS conflicts, the address here includes an additional offset of 0x1000. Access to this register must take this into account.) The value returned when this register is read reflects the status of an endpoint specified by setting the endpoint index in the CFG3_ENDPOINT field. When the endpoint index (CFG3_ENDPOINT) = 0, this field provides status and control of Endpoint 0. Also, the MAXPAYLOAD field defines the maximum amount of data that can be transferred through the selected IN endpoint in a single operation. There is a MAXPAYLOAD for each IN endpoint (except Endpoint 0). Note that the action initiated by setting a field in one of the IDXn registers when the selected endpoint has not been configured is undefined."]
    #[inline(always)]
    pub const fn idx0(&self) -> &Idx0 {
        &self.idx0
    }
    #[doc = "0x14 - Provides control and status bits for OUT transactions through the currently-selected endpoint. It is reset to 0. The value returned when this register is read reflects the status of an endpoint specified by setting the endpoint index in the CFG3_ENDPOINT field. Also, the MAXPAYLOAD field defines the maximum amount of data that can be transferred through the selected OUT endpoint in a single operation. There is a MAXPAYLOAD for each OUT endpoint (except Endpoint 0). Note that the action initiated by setting a field in one of the IDXn registers when the selected endpoint has not been configured is undefined."]
    #[inline(always)]
    pub const fn idx1(&self) -> &Idx1 {
        &self.idx1
    }
    #[doc = "0x18 - Contains the outcount value for number of received bytes in the packet in the OUT FIFO, and the configurable IN and OUT Endpoint FIFO size."]
    #[inline(always)]
    pub const fn idx2(&self) -> &Idx2 {
        &self.idx2
    }
    #[doc = "0x1c - Sets the start address of the selected IN and OUT endpoint FIFOs."]
    #[inline(always)]
    pub const fn fifoadd(&self) -> &Fifoadd {
        &self.fifoadd
    }
    #[doc = "0x20 - Endpoint 0 FIFO register"]
    #[inline(always)]
    pub const fn fifo0(&self) -> &Fifo0 {
        &self.fifo0
    }
    #[doc = "0x24 - Endpoint 1 FIFO register"]
    #[inline(always)]
    pub const fn fifo1(&self) -> &Fifo1 {
        &self.fifo1
    }
    #[doc = "0x28 - Endpoint 2 FIFO register"]
    #[inline(always)]
    pub const fn fifo2(&self) -> &Fifo2 {
        &self.fifo2
    }
    #[doc = "0x2c - Endpoint 3 FIFO register"]
    #[inline(always)]
    pub const fn fifo3(&self) -> &Fifo3 {
        &self.fifo3
    }
    #[doc = "0x30 - Endpoint 4 FIFO register"]
    #[inline(always)]
    pub const fn fifo4(&self) -> &Fifo4 {
        &self.fifo4
    }
    #[doc = "0x34 - Endpoint 5 FIFO register"]
    #[inline(always)]
    pub const fn fifo5(&self) -> &Fifo5 {
        &self.fifo5
    }
    #[doc = "0x6c - Read-only register that returns version number (xx.yyy) of the core hardware."]
    #[inline(always)]
    pub const fn hwvers(&self) -> &Hwvers {
        &self.hwvers
    }
    #[doc = "0x78 - Contains read-only info of the number of IN and OUT endpoints included in the design, width of the RAM, the ability to reset the USB Controller via software, a soft reset bit for the CLK clock domain and a soft reset bit for the XCLK clock domain."]
    #[inline(always)]
    pub const fn info(&self) -> &Info {
        &self.info
    }
    #[doc = "0x80 - Holds the configurable chirp timeout value."]
    #[inline(always)]
    pub const fn timeout1(&self) -> &Timeout1 {
        &self.timeout1
    }
    #[doc = "0x84 - Holds the configurable delay from the end of High Speed resume signal to enable UTM normal operating mode."]
    #[inline(always)]
    pub const fn timeout2(&self) -> &Timeout2 {
        &self.timeout2
    }
    #[doc = "0x2000 - Provides optional control for turning off the interface clocks to USB Controller and PHY as well as the reference clock to the USB PHY."]
    #[inline(always)]
    pub const fn clkctrl(&self) -> &Clkctrl {
        &self.clkctrl
    }
    #[doc = "0x2004 - Provides optional SRAM tuning control."]
    #[inline(always)]
    pub const fn sramctrl(&self) -> &Sramctrl {
        &self.sramctrl
    }
    #[doc = "0x2014 - This read only register provides the results from the PHY OBS port controlled by reg 0x20\\[5:4\\]. IF any bits are set, the bits are sticky. Clear this register using the OBSCLRSTAT register."]
    #[inline(always)]
    pub const fn utmistickystatus(&self) -> &Utmistickystatus {
        &self.utmistickystatus
    }
    #[doc = "0x2018 - Clears all bits in the sticky obs status register."]
    #[inline(always)]
    pub const fn obsclrstat(&self) -> &Obsclrstat {
        &self.obsclrstat
    }
    #[doc = "0x201c - Enables a pulldown resistor(15K) on D+ or D-"]
    #[inline(always)]
    pub const fn dpdmpulldown(&self) -> &Dpdmpulldown {
        &self.dpdmpulldown
    }
    #[doc = "0x2020 - USB Battery Charge Detenction Registers"]
    #[inline(always)]
    pub const fn bcdetstatus(&self) -> &Bcdetstatus {
        &self.bcdetstatus
    }
    #[doc = "0x2024 - Battery Charging detection main control register"]
    #[inline(always)]
    pub const fn bcdetcrtl1(&self) -> &Bcdetcrtl1 {
        &self.bcdetcrtl1
    }
    #[doc = "0x2028 - Battery Charging auxillary detection control register"]
    #[inline(always)]
    pub const fn bcdetcrtl2(&self) -> &Bcdetcrtl2 {
        &self.bcdetcrtl2
    }
}
#[doc = "CFG0 (rw) register accessor: Function address, power management, interrupt status register for EP0 and IN Endpoints 1 to 5\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0`]
module"]
#[doc(alias = "CFG0")]
pub type Cfg0 = crate::Reg<cfg0::Cfg0Spec>;
#[doc = "Function address, power management, interrupt status register for EP0 and IN Endpoints 1 to 5"]
pub mod cfg0;
#[doc = "CFG1 (rw) register accessor: Indicates which of the IN Endpoint 1 - 5 interrupts and the single Endpoint 0 interrupt are currently active. Also indicates which of the interrupts for OUT Endpoint 1 - 5 are currently active. All active interrupts are cleared when this register is read.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`]
module"]
#[doc(alias = "CFG1")]
pub type Cfg1 = crate::Reg<cfg1::Cfg1Spec>;
#[doc = "Indicates which of the IN Endpoint 1 - 5 interrupts and the single Endpoint 0 interrupt are currently active. Also indicates which of the interrupts for OUT Endpoint 1 - 5 are currently active. All active interrupts are cleared when this register is read."]
pub mod cfg1;
#[doc = "CFG2 (rw) register accessor: Provides interrupt enable and (currently active) status bits for each of the state interrupts, as well as the IN Endpoint and OUT Endpoint nterrupts. All active interrupts are cleared when this register is read. On reset, all IN and OUT Endpoint interrupts, in addition to Endpoint 0, are set to 1 while the remaining bits are set to 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg2`]
module"]
#[doc(alias = "CFG2")]
pub type Cfg2 = crate::Reg<cfg2::Cfg2Spec>;
#[doc = "Provides interrupt enable and (currently active) status bits for each of the state interrupts, as well as the IN Endpoint and OUT Endpoint nterrupts. All active interrupts are cleared when this register is read. On reset, all IN and OUT Endpoint interrupts, in addition to Endpoint 0, are set to 1 while the remaining bits are set to 0."]
pub mod cfg2;
#[doc = "CFG3 (rw) register accessor: Provides Test fields to put the USB Controller into one of four test modes described in the USB 2.0 specification. Only one of the Test fields should be set at any time. (Not used in normal operation.) Also includes an index field that determines which endpoint control,status registers are accessed via the IDXn register fields, and a Frame field that holds the last received frame number.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg3`]
module"]
#[doc(alias = "CFG3")]
pub type Cfg3 = crate::Reg<cfg3::Cfg3Spec>;
#[doc = "Provides Test fields to put the USB Controller into one of four test modes described in the USB 2.0 specification. Only one of the Test fields should be set at any time. (Not used in normal operation.) Also includes an index field that determines which endpoint control,status registers are accessed via the IDXn register fields, and a Frame field that holds the last received frame number."]
pub mod cfg3;
#[doc = "IDX0 (rw) register accessor: Provides additional control and status for IN transactions through the currently-selected endpoint. (To avoid CMSIS conflicts, the address here includes an additional offset of 0x1000. Access to this register must take this into account.) The value returned when this register is read reflects the status of an endpoint specified by setting the endpoint index in the CFG3_ENDPOINT field. When the endpoint index (CFG3_ENDPOINT) = 0, this field provides status and control of Endpoint 0. Also, the MAXPAYLOAD field defines the maximum amount of data that can be transferred through the selected IN endpoint in a single operation. There is a MAXPAYLOAD for each IN endpoint (except Endpoint 0). Note that the action initiated by setting a field in one of the IDXn registers when the selected endpoint has not been configured is undefined.\n\nYou can [`read`](crate::Reg::read) this register and get [`idx0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idx0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idx0`]
module"]
#[doc(alias = "IDX0")]
pub type Idx0 = crate::Reg<idx0::Idx0Spec>;
#[doc = "Provides additional control and status for IN transactions through the currently-selected endpoint. (To avoid CMSIS conflicts, the address here includes an additional offset of 0x1000. Access to this register must take this into account.) The value returned when this register is read reflects the status of an endpoint specified by setting the endpoint index in the CFG3_ENDPOINT field. When the endpoint index (CFG3_ENDPOINT) = 0, this field provides status and control of Endpoint 0. Also, the MAXPAYLOAD field defines the maximum amount of data that can be transferred through the selected IN endpoint in a single operation. There is a MAXPAYLOAD for each IN endpoint (except Endpoint 0). Note that the action initiated by setting a field in one of the IDXn registers when the selected endpoint has not been configured is undefined."]
pub mod idx0;
#[doc = "IDX1 (rw) register accessor: Provides control and status bits for OUT transactions through the currently-selected endpoint. It is reset to 0. The value returned when this register is read reflects the status of an endpoint specified by setting the endpoint index in the CFG3_ENDPOINT field. Also, the MAXPAYLOAD field defines the maximum amount of data that can be transferred through the selected OUT endpoint in a single operation. There is a MAXPAYLOAD for each OUT endpoint (except Endpoint 0). Note that the action initiated by setting a field in one of the IDXn registers when the selected endpoint has not been configured is undefined.\n\nYou can [`read`](crate::Reg::read) this register and get [`idx1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idx1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idx1`]
module"]
#[doc(alias = "IDX1")]
pub type Idx1 = crate::Reg<idx1::Idx1Spec>;
#[doc = "Provides control and status bits for OUT transactions through the currently-selected endpoint. It is reset to 0. The value returned when this register is read reflects the status of an endpoint specified by setting the endpoint index in the CFG3_ENDPOINT field. Also, the MAXPAYLOAD field defines the maximum amount of data that can be transferred through the selected OUT endpoint in a single operation. There is a MAXPAYLOAD for each OUT endpoint (except Endpoint 0). Note that the action initiated by setting a field in one of the IDXn registers when the selected endpoint has not been configured is undefined."]
pub mod idx1;
#[doc = "IDX2 (rw) register accessor: Contains the outcount value for number of received bytes in the packet in the OUT FIFO, and the configurable IN and OUT Endpoint FIFO size.\n\nYou can [`read`](crate::Reg::read) this register and get [`idx2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idx2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idx2`]
module"]
#[doc(alias = "IDX2")]
pub type Idx2 = crate::Reg<idx2::Idx2Spec>;
#[doc = "Contains the outcount value for number of received bytes in the packet in the OUT FIFO, and the configurable IN and OUT Endpoint FIFO size."]
pub mod idx2;
#[doc = "FIFOADD (rw) register accessor: Sets the start address of the selected IN and OUT endpoint FIFOs.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoadd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoadd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifoadd`]
module"]
#[doc(alias = "FIFOADD")]
pub type Fifoadd = crate::Reg<fifoadd::FifoaddSpec>;
#[doc = "Sets the start address of the selected IN and OUT endpoint FIFOs."]
pub mod fifoadd;
#[doc = "FIFO0 (rw) register accessor: Endpoint 0 FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo0`]
module"]
#[doc(alias = "FIFO0")]
pub type Fifo0 = crate::Reg<fifo0::Fifo0Spec>;
#[doc = "Endpoint 0 FIFO register"]
pub mod fifo0;
#[doc = "FIFO1 (rw) register accessor: Endpoint 1 FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo1`]
module"]
#[doc(alias = "FIFO1")]
pub type Fifo1 = crate::Reg<fifo1::Fifo1Spec>;
#[doc = "Endpoint 1 FIFO register"]
pub mod fifo1;
#[doc = "FIFO2 (rw) register accessor: Endpoint 2 FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo2`]
module"]
#[doc(alias = "FIFO2")]
pub type Fifo2 = crate::Reg<fifo2::Fifo2Spec>;
#[doc = "Endpoint 2 FIFO register"]
pub mod fifo2;
#[doc = "FIFO3 (rw) register accessor: Endpoint 3 FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo3`]
module"]
#[doc(alias = "FIFO3")]
pub type Fifo3 = crate::Reg<fifo3::Fifo3Spec>;
#[doc = "Endpoint 3 FIFO register"]
pub mod fifo3;
#[doc = "FIFO4 (rw) register accessor: Endpoint 4 FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo4`]
module"]
#[doc(alias = "FIFO4")]
pub type Fifo4 = crate::Reg<fifo4::Fifo4Spec>;
#[doc = "Endpoint 4 FIFO register"]
pub mod fifo4;
#[doc = "FIFO5 (rw) register accessor: Endpoint 5 FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo5`]
module"]
#[doc(alias = "FIFO5")]
pub type Fifo5 = crate::Reg<fifo5::Fifo5Spec>;
#[doc = "Endpoint 5 FIFO register"]
pub mod fifo5;
#[doc = "HWVERS (rw) register accessor: Read-only register that returns version number (xx.yyy) of the core hardware.\n\nYou can [`read`](crate::Reg::read) this register and get [`hwvers::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwvers::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwvers`]
module"]
#[doc(alias = "HWVERS")]
pub type Hwvers = crate::Reg<hwvers::HwversSpec>;
#[doc = "Read-only register that returns version number (xx.yyy) of the core hardware."]
pub mod hwvers;
#[doc = "INFO (rw) register accessor: Contains read-only info of the number of IN and OUT endpoints included in the design, width of the RAM, the ability to reset the USB Controller via software, a soft reset bit for the CLK clock domain and a soft reset bit for the XCLK clock domain.\n\nYou can [`read`](crate::Reg::read) this register and get [`info::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`info::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@info`]
module"]
#[doc(alias = "INFO")]
pub type Info = crate::Reg<info::InfoSpec>;
#[doc = "Contains read-only info of the number of IN and OUT endpoints included in the design, width of the RAM, the ability to reset the USB Controller via software, a soft reset bit for the CLK clock domain and a soft reset bit for the XCLK clock domain."]
pub mod info;
#[doc = "TIMEOUT1 (rw) register accessor: Holds the configurable chirp timeout value.\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout1`]
module"]
#[doc(alias = "TIMEOUT1")]
pub type Timeout1 = crate::Reg<timeout1::Timeout1Spec>;
#[doc = "Holds the configurable chirp timeout value."]
pub mod timeout1;
#[doc = "TIMEOUT2 (rw) register accessor: Holds the configurable delay from the end of High Speed resume signal to enable UTM normal operating mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout2`]
module"]
#[doc(alias = "TIMEOUT2")]
pub type Timeout2 = crate::Reg<timeout2::Timeout2Spec>;
#[doc = "Holds the configurable delay from the end of High Speed resume signal to enable UTM normal operating mode."]
pub mod timeout2;
#[doc = "CLKCTRL (rw) register accessor: Provides optional control for turning off the interface clocks to USB Controller and PHY as well as the reference clock to the USB PHY.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkctrl`]
module"]
#[doc(alias = "CLKCTRL")]
pub type Clkctrl = crate::Reg<clkctrl::ClkctrlSpec>;
#[doc = "Provides optional control for turning off the interface clocks to USB Controller and PHY as well as the reference clock to the USB PHY."]
pub mod clkctrl;
#[doc = "SRAMCTRL (rw) register accessor: Provides optional SRAM tuning control.\n\nYou can [`read`](crate::Reg::read) this register and get [`sramctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sramctrl`]
module"]
#[doc(alias = "SRAMCTRL")]
pub type Sramctrl = crate::Reg<sramctrl::SramctrlSpec>;
#[doc = "Provides optional SRAM tuning control."]
pub mod sramctrl;
#[doc = "UTMISTICKYSTATUS (rw) register accessor: This read only register provides the results from the PHY OBS port controlled by reg 0x20\\[5:4\\]. IF any bits are set, the bits are sticky. Clear this register using the OBSCLRSTAT register.\n\nYou can [`read`](crate::Reg::read) this register and get [`utmistickystatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utmistickystatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@utmistickystatus`]
module"]
#[doc(alias = "UTMISTICKYSTATUS")]
pub type Utmistickystatus = crate::Reg<utmistickystatus::UtmistickystatusSpec>;
#[doc = "This read only register provides the results from the PHY OBS port controlled by reg 0x20\\[5:4\\]. IF any bits are set, the bits are sticky. Clear this register using the OBSCLRSTAT register."]
pub mod utmistickystatus;
#[doc = "OBSCLRSTAT (rw) register accessor: Clears all bits in the sticky obs status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`obsclrstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obsclrstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obsclrstat`]
module"]
#[doc(alias = "OBSCLRSTAT")]
pub type Obsclrstat = crate::Reg<obsclrstat::ObsclrstatSpec>;
#[doc = "Clears all bits in the sticky obs status register."]
pub mod obsclrstat;
#[doc = "DPDMPULLDOWN (rw) register accessor: Enables a pulldown resistor(15K) on D+ or D-\n\nYou can [`read`](crate::Reg::read) this register and get [`dpdmpulldown::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpdmpulldown::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpdmpulldown`]
module"]
#[doc(alias = "DPDMPULLDOWN")]
pub type Dpdmpulldown = crate::Reg<dpdmpulldown::DpdmpulldownSpec>;
#[doc = "Enables a pulldown resistor(15K) on D+ or D-"]
pub mod dpdmpulldown;
#[doc = "BCDETSTATUS (rw) register accessor: USB Battery Charge Detenction Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`bcdetstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcdetstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdetstatus`]
module"]
#[doc(alias = "BCDETSTATUS")]
pub type Bcdetstatus = crate::Reg<bcdetstatus::BcdetstatusSpec>;
#[doc = "USB Battery Charge Detenction Registers"]
pub mod bcdetstatus;
#[doc = "BCDETCRTL1 (rw) register accessor: Battery Charging detection main control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcdetcrtl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcdetcrtl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdetcrtl1`]
module"]
#[doc(alias = "BCDETCRTL1")]
pub type Bcdetcrtl1 = crate::Reg<bcdetcrtl1::Bcdetcrtl1Spec>;
#[doc = "Battery Charging detection main control register"]
pub mod bcdetcrtl1;
#[doc = "BCDETCRTL2 (rw) register accessor: Battery Charging auxillary detection control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcdetcrtl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcdetcrtl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdetcrtl2`]
module"]
#[doc(alias = "BCDETCRTL2")]
pub type Bcdetcrtl2 = crate::Reg<bcdetcrtl2::Bcdetcrtl2Spec>;
#[doc = "Battery Charging auxillary detection control register"]
pub mod bcdetcrtl2;
