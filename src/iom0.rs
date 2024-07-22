#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fifo: Fifo,
    _reserved1: [u8; 0xfc],
    fifoptr: Fifoptr,
    fifothr: Fifothr,
    fifopop: Fifopop,
    fifopush: Fifopush,
    fifoctrl: Fifoctrl,
    fifoloc: Fifoloc,
    clkcfg: Clkcfg,
    submodctrl: Submodctrl,
    cmd: Cmd,
    dcxctrl: Dcxctrl,
    offsethi: Offsethi,
    cmdstat: Cmdstat,
    _reserved13: [u8; 0xd0],
    inten: Inten,
    intstat: Intstat,
    intclr: Intclr,
    intset: Intset,
    dmatrigen: Dmatrigen,
    dmatrigstat: Dmatrigstat,
    dmacfg: Dmacfg,
    dmatotcount: Dmatotcount,
    dmatargaddr: Dmatargaddr,
    dmastat: Dmastat,
    cqcfg: Cqcfg,
    cqaddr: Cqaddr,
    cqstat: Cqstat,
    cqflags: Cqflags,
    cqsetclear: Cqsetclear,
    cqpauseen: Cqpauseen,
    cqcuridx: Cqcuridx,
    cqendidx: Cqendidx,
    status: Status,
    _reserved32: [u8; 0x34],
    mspicfg: Mspicfg,
    _reserved33: [u8; 0x3c],
    mi2ccfg: Mi2ccfg,
    devcfg: Devcfg,
    _reserved35: [u8; 0xc0],
    iomdbg: Iomdbg,
}
impl RegisterBlock {
    #[doc = "0x00 - Provides direct random access to both input and output fifos. The state of the FIFO is not distured by reading these locations (ie no POP will be done). FIFO0 is accessible from addresses 0x0 - 0x1C, and is used for data outuput from the IOM to external devices. These FIFO locations can be read and written directly. FIFO locations 0x20 - 0x3C provide read only access to the input fifo. These FIFO locations cannot be directly written by the MCU, and are updated only by the internal hardware. Writes to the FIFO0 will take effect immediately. The currently FIFO pointers in register FIFOLOC indicate the current offset into each FIFO0 for the read and write operations. Access to the FIFOs can only be done in word increments; Byte access and writes are not supported. For push and pop style access to FIFO0 can be done using the FIFOPOP and FIFOPUSH registers below."]
    #[inline(always)]
    pub const fn fifo(&self) -> &Fifo {
        &self.fifo
    }
    #[doc = "0x100 - Provides the current valid byte count of data within the FIFO as seen from the internal state machines. FIFO0 is dedicated to outgoing transactions and FIFO1 is dedicated to incoming transactions. All counts are specified in units of bytes."]
    #[inline(always)]
    pub const fn fifoptr(&self) -> &Fifoptr {
        &self.fifoptr
    }
    #[doc = "0x104 - Sets the threshold values for incoming and outgoing transactions. The threshold values are used to assert the interrupt if enabled, and also used during DMA to set the transfer size as a result of DMATHR trigger. The WTHR is used to indicate when there are more than WTHR bytes of open fifo locations available in the outgoing FIFO (FIFO0). The intended use to invoke an interrupt or DMA transfer that will refill the FIFO with a byte count up to this value. The RTHR is used to indicate when there are more than RTHR bytes in the incoming FIFO (FIFO1) and a data transfer of this size can be supported, either through direct POP of the FIFO, or through DMA. The value of both RTHR and WTHR are also used to set the data transfer size of DMA operations if DMATHR trigger is enabled."]
    #[inline(always)]
    pub const fn fifothr(&self) -> &Fifothr {
        &self.fifothr
    }
    #[doc = "0x108 - Will advance the internal read pointer of the incoming FIFO (FIFO1) when read, if POPWR is not active. If POPWR is active, a write to this register is needed to advance the internal FIFO pointer."]
    #[inline(always)]
    pub const fn fifopop(&self) -> &Fifopop {
        &self.fifopop
    }
    #[doc = "0x10c - Will write new data into the outgoing FIFO and advance the internal write pointer."]
    #[inline(always)]
    pub const fn fifopush(&self) -> &Fifopush {
        &self.fifopush
    }
    #[doc = "0x110 - Provides controls for the operation of the internal FIFOs. Contains fields used to control the operation of the POP register, and also controls to reset the internal pointers of the FIFOs."]
    #[inline(always)]
    pub const fn fifoctrl(&self) -> &Fifoctrl {
        &self.fifoctrl
    }
    #[doc = "0x114 - Provides a read only value of the current read and write pointers. This register is read only and can be used alogn with the FIFO direct access method to determine the next data to be used for input and output functions."]
    #[inline(always)]
    pub const fn fifoloc(&self) -> &Fifoloc {
        &self.fifoloc
    }
    #[doc = "0x118 - Provides clock related controls used internal to the BLEIF module, and enablement of 32KHz clock to the BLE Core module. The internal clock sourced is selected via the FSEL and can be further divided by 3 using the DIV3 control. This register is also used to enable the clock, which must be done prior to performing any IO transactions."]
    #[inline(always)]
    pub const fn clkcfg(&self) -> &Clkcfg {
        &self.clkcfg
    }
    #[doc = "0x11c - Provides enable for each submodule. Only a sigle submodule can be enabled at one time."]
    #[inline(always)]
    pub const fn submodctrl(&self) -> &Submodctrl {
        &self.submodctrl
    }
    #[doc = "0x120 - Writes to this register will start an IO transaction, as well as set various parameters for the command itself. Reads will return the command value written to the CMD register. To read the number of bytes that have yet to be transferred, refer to the CTSIZE field within the CMDSTAT register."]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x124 - Enables transmission of DCX signal with SPI transactions and selects which CE signals will be used to transmit the DCX signal."]
    #[inline(always)]
    pub const fn dcxctrl(&self) -> &Dcxctrl {
        &self.dcxctrl
    }
    #[doc = "0x128 - High order bytes of offset for IO transaction"]
    #[inline(always)]
    pub const fn offsethi(&self) -> &Offsethi {
        &self.offsethi
    }
    #[doc = "0x12c - Provides staus on the execution of the command currently in progress. The fields in this register will reflect the real time status of the internal state machines and data transfers within the IOM. These are read only fields and writes to the registers are ignored."]
    #[inline(always)]
    pub const fn cmdstat(&self) -> &Cmdstat {
        &self.cmdstat
    }
    #[doc = "0x200 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x204 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0x208 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn intclr(&self) -> &Intclr {
        &self.intclr
    }
    #[doc = "0x20c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn intset(&self) -> &Intset {
        &self.intset
    }
    #[doc = "0x210 - Provides control on which event will trigger the DMA transfer after the DMA operation is setup and enabled. The trigger event will cause a number of bytes (depending on trigger event) to be transferred via the DMA operation, and can be used to adjust the latency of data to/from the IOM module to/from the dma target. DMA transfers are broken into smaller transfers internally of up to 16 bytes each, and multiple trigger events can be used to complete the entire programmed DMA transfer."]
    #[inline(always)]
    pub const fn dmatrigen(&self) -> &Dmatrigen {
        &self.dmatrigen
    }
    #[doc = "0x214 - Provides the status of trigger events that have occurred for the transaction. Some of the bits are read only and some can be reset via a write of 0."]
    #[inline(always)]
    pub const fn dmatrigstat(&self) -> &Dmatrigstat {
        &self.dmatrigstat
    }
    #[doc = "0x218 - Configuration control of the DMA process, including the direction of DMA, and enablement of DMA"]
    #[inline(always)]
    pub const fn dmacfg(&self) -> &Dmacfg {
        &self.dmacfg
    }
    #[doc = "0x21c - Contains the number of bytes to be transferred for this DMA transaction. This register is decremented as the data is transferred, and will be 0 at the completion of the DMA operation."]
    #[inline(always)]
    pub const fn dmatotcount(&self) -> &Dmatotcount {
        &self.dmatotcount
    }
    #[doc = "0x220 - The source or destination address internal the SRAM for the DMA data. For write operations, this can only be SRAM data (ADDR bit 28 = 1); For read operations, this can ve either SRAM or FLASH (ADDR bit 28 = 0)"]
    #[inline(always)]
    pub const fn dmatargaddr(&self) -> &Dmatargaddr {
        &self.dmatargaddr
    }
    #[doc = "0x224 - Status of the DMA operation currently in progress."]
    #[inline(always)]
    pub const fn dmastat(&self) -> &Dmastat {
        &self.dmastat
    }
    #[doc = "0x228 - Controls parameters and options for execution of the command queue operation. To enable command queue, create this in memory, set the address, and enable it with a write to CQEN"]
    #[inline(always)]
    pub const fn cqcfg(&self) -> &Cqcfg {
        &self.cqcfg
    }
    #[doc = "0x22c - The SRAM address which will be fetched next execution of the CQ operation. This register is updated as the CQ operation progresses, and is the live version of the register. The register can also be written by the Command Queue operation itself, allowing the relocation of successive CQ fetches. In this case, the new CQ address will be used for the next CQ address/data fetch"]
    #[inline(always)]
    pub const fn cqaddr(&self) -> &Cqaddr {
        &self.cqaddr
    }
    #[doc = "0x230 - Provides the status of the command queue operation. If the command queue is disabled, these bits will be cleared. The bits are read only"]
    #[inline(always)]
    pub const fn cqstat(&self) -> &Cqstat {
        &self.cqstat
    }
    #[doc = "0x234 - Command Queue Flag"]
    #[inline(always)]
    pub const fn cqflags(&self) -> &Cqflags {
        &self.cqflags
    }
    #[doc = "0x238 - Set/Clear the command queue software pause flags on a per-bit basis. Contains 3 fields, allowing for setting, clearing or toggling the value in the software flags. Priority when the same bit is enabled in each field is toggle, then set, then clear."]
    #[inline(always)]
    pub const fn cqsetclear(&self) -> &Cqsetclear {
        &self.cqsetclear
    }
    #[doc = "0x23c - Enables a flag to pause an active command queue operation. If a bit is '1' and the corresponding bit in the CQFLAG register is '1', CQ processing will halt until either value is changed to '0'."]
    #[inline(always)]
    pub const fn cqpauseen(&self) -> &Cqpauseen {
        &self.cqpauseen
    }
    #[doc = "0x240 - Current index value, targeted to be written by register write operations within the command queue. This is compared to the CQENDIDX and will stop the CQ operation if bit 15 of the CQPAUSEEN is '1' and this current index equals the CQENDIDX register value. This will only pause when the values are equal."]
    #[inline(always)]
    pub const fn cqcuridx(&self) -> &Cqcuridx {
        &self.cqcuridx
    }
    #[doc = "0x244 - End index value, targeted to be written by software to indicate the last valid register pair contained within the command queue. rgister write operations within the command queue. This is compared to the CQCURIDX and will stop the CQ operation if bit 15 of the CQPAUSEEN is '1' and this current index equals the CQCURIDX register value. This will only pause when the values are equal."]
    #[inline(always)]
    pub const fn cqendidx(&self) -> &Cqendidx {
        &self.cqendidx
    }
    #[doc = "0x248 - IOM Module Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x280 - Controls the configuration of the SPI master module, including POL/PHA, LSB, flow control, and delays for MISO and MOSI"]
    #[inline(always)]
    pub const fn mspicfg(&self) -> &Mspicfg {
        &self.mspicfg
    }
    #[doc = "0x2c0 - Controls the configuration of the I2C bus master."]
    #[inline(always)]
    pub const fn mi2ccfg(&self) -> &Mi2ccfg {
        &self.mi2ccfg
    }
    #[doc = "0x2c4 - Contains the I2C device address."]
    #[inline(always)]
    pub const fn devcfg(&self) -> &Devcfg {
        &self.devcfg
    }
    #[doc = "0x388 - Debug control"]
    #[inline(always)]
    pub const fn iomdbg(&self) -> &Iomdbg {
        &self.iomdbg
    }
}
#[doc = "FIFO (rw) register accessor: Provides direct random access to both input and output fifos. The state of the FIFO is not distured by reading these locations (ie no POP will be done). FIFO0 is accessible from addresses 0x0 - 0x1C, and is used for data outuput from the IOM to external devices. These FIFO locations can be read and written directly. FIFO locations 0x20 - 0x3C provide read only access to the input fifo. These FIFO locations cannot be directly written by the MCU, and are updated only by the internal hardware. Writes to the FIFO0 will take effect immediately. The currently FIFO pointers in register FIFOLOC indicate the current offset into each FIFO0 for the read and write operations. Access to the FIFOs can only be done in word increments; Byte access and writes are not supported. For push and pop style access to FIFO0 can be done using the FIFOPOP and FIFOPUSH registers below.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo`]
module"]
#[doc(alias = "FIFO")]
pub type Fifo = crate::Reg<fifo::FifoSpec>;
#[doc = "Provides direct random access to both input and output fifos. The state of the FIFO is not distured by reading these locations (ie no POP will be done). FIFO0 is accessible from addresses 0x0 - 0x1C, and is used for data outuput from the IOM to external devices. These FIFO locations can be read and written directly. FIFO locations 0x20 - 0x3C provide read only access to the input fifo. These FIFO locations cannot be directly written by the MCU, and are updated only by the internal hardware. Writes to the FIFO0 will take effect immediately. The currently FIFO pointers in register FIFOLOC indicate the current offset into each FIFO0 for the read and write operations. Access to the FIFOs can only be done in word increments; Byte access and writes are not supported. For push and pop style access to FIFO0 can be done using the FIFOPOP and FIFOPUSH registers below."]
pub mod fifo;
#[doc = "FIFOPTR (rw) register accessor: Provides the current valid byte count of data within the FIFO as seen from the internal state machines. FIFO0 is dedicated to outgoing transactions and FIFO1 is dedicated to incoming transactions. All counts are specified in units of bytes.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifoptr`]
module"]
#[doc(alias = "FIFOPTR")]
pub type Fifoptr = crate::Reg<fifoptr::FifoptrSpec>;
#[doc = "Provides the current valid byte count of data within the FIFO as seen from the internal state machines. FIFO0 is dedicated to outgoing transactions and FIFO1 is dedicated to incoming transactions. All counts are specified in units of bytes."]
pub mod fifoptr;
#[doc = "FIFOTHR (rw) register accessor: Sets the threshold values for incoming and outgoing transactions. The threshold values are used to assert the interrupt if enabled, and also used during DMA to set the transfer size as a result of DMATHR trigger. The WTHR is used to indicate when there are more than WTHR bytes of open fifo locations available in the outgoing FIFO (FIFO0). The intended use to invoke an interrupt or DMA transfer that will refill the FIFO with a byte count up to this value. The RTHR is used to indicate when there are more than RTHR bytes in the incoming FIFO (FIFO1) and a data transfer of this size can be supported, either through direct POP of the FIFO, or through DMA. The value of both RTHR and WTHR are also used to set the data transfer size of DMA operations if DMATHR trigger is enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifothr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifothr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifothr`]
module"]
#[doc(alias = "FIFOTHR")]
pub type Fifothr = crate::Reg<fifothr::FifothrSpec>;
#[doc = "Sets the threshold values for incoming and outgoing transactions. The threshold values are used to assert the interrupt if enabled, and also used during DMA to set the transfer size as a result of DMATHR trigger. The WTHR is used to indicate when there are more than WTHR bytes of open fifo locations available in the outgoing FIFO (FIFO0). The intended use to invoke an interrupt or DMA transfer that will refill the FIFO with a byte count up to this value. The RTHR is used to indicate when there are more than RTHR bytes in the incoming FIFO (FIFO1) and a data transfer of this size can be supported, either through direct POP of the FIFO, or through DMA. The value of both RTHR and WTHR are also used to set the data transfer size of DMA operations if DMATHR trigger is enabled."]
pub mod fifothr;
#[doc = "FIFOPOP (rw) register accessor: Will advance the internal read pointer of the incoming FIFO (FIFO1) when read, if POPWR is not active. If POPWR is active, a write to this register is needed to advance the internal FIFO pointer.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifopop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifopop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifopop`]
module"]
#[doc(alias = "FIFOPOP")]
pub type Fifopop = crate::Reg<fifopop::FifopopSpec>;
#[doc = "Will advance the internal read pointer of the incoming FIFO (FIFO1) when read, if POPWR is not active. If POPWR is active, a write to this register is needed to advance the internal FIFO pointer."]
pub mod fifopop;
#[doc = "FIFOPUSH (rw) register accessor: Will write new data into the outgoing FIFO and advance the internal write pointer.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifopush::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifopush::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifopush`]
module"]
#[doc(alias = "FIFOPUSH")]
pub type Fifopush = crate::Reg<fifopush::FifopushSpec>;
#[doc = "Will write new data into the outgoing FIFO and advance the internal write pointer."]
pub mod fifopush;
#[doc = "FIFOCTRL (rw) register accessor: Provides controls for the operation of the internal FIFOs. Contains fields used to control the operation of the POP register, and also controls to reset the internal pointers of the FIFOs.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifoctrl`]
module"]
#[doc(alias = "FIFOCTRL")]
pub type Fifoctrl = crate::Reg<fifoctrl::FifoctrlSpec>;
#[doc = "Provides controls for the operation of the internal FIFOs. Contains fields used to control the operation of the POP register, and also controls to reset the internal pointers of the FIFOs."]
pub mod fifoctrl;
#[doc = "FIFOLOC (rw) register accessor: Provides a read only value of the current read and write pointers. This register is read only and can be used alogn with the FIFO direct access method to determine the next data to be used for input and output functions.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoloc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoloc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifoloc`]
module"]
#[doc(alias = "FIFOLOC")]
pub type Fifoloc = crate::Reg<fifoloc::FifolocSpec>;
#[doc = "Provides a read only value of the current read and write pointers. This register is read only and can be used alogn with the FIFO direct access method to determine the next data to be used for input and output functions."]
pub mod fifoloc;
#[doc = "CLKCFG (rw) register accessor: Provides clock related controls used internal to the BLEIF module, and enablement of 32KHz clock to the BLE Core module. The internal clock sourced is selected via the FSEL and can be further divided by 3 using the DIV3 control. This register is also used to enable the clock, which must be done prior to performing any IO transactions.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcfg`]
module"]
#[doc(alias = "CLKCFG")]
pub type Clkcfg = crate::Reg<clkcfg::ClkcfgSpec>;
#[doc = "Provides clock related controls used internal to the BLEIF module, and enablement of 32KHz clock to the BLE Core module. The internal clock sourced is selected via the FSEL and can be further divided by 3 using the DIV3 control. This register is also used to enable the clock, which must be done prior to performing any IO transactions."]
pub mod clkcfg;
#[doc = "SUBMODCTRL (rw) register accessor: Provides enable for each submodule. Only a sigle submodule can be enabled at one time.\n\nYou can [`read`](crate::Reg::read) this register and get [`submodctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`submodctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@submodctrl`]
module"]
#[doc(alias = "SUBMODCTRL")]
pub type Submodctrl = crate::Reg<submodctrl::SubmodctrlSpec>;
#[doc = "Provides enable for each submodule. Only a sigle submodule can be enabled at one time."]
pub mod submodctrl;
#[doc = "CMD (rw) register accessor: Writes to this register will start an IO transaction, as well as set various parameters for the command itself. Reads will return the command value written to the CMD register. To read the number of bytes that have yet to be transferred, refer to the CTSIZE field within the CMDSTAT register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Writes to this register will start an IO transaction, as well as set various parameters for the command itself. Reads will return the command value written to the CMD register. To read the number of bytes that have yet to be transferred, refer to the CTSIZE field within the CMDSTAT register."]
pub mod cmd;
#[doc = "DCXCTRL (rw) register accessor: Enables transmission of DCX signal with SPI transactions and selects which CE signals will be used to transmit the DCX signal.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcxctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcxctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcxctrl`]
module"]
#[doc(alias = "DCXCTRL")]
pub type Dcxctrl = crate::Reg<dcxctrl::DcxctrlSpec>;
#[doc = "Enables transmission of DCX signal with SPI transactions and selects which CE signals will be used to transmit the DCX signal."]
pub mod dcxctrl;
#[doc = "OFFSETHI (rw) register accessor: High order bytes of offset for IO transaction\n\nYou can [`read`](crate::Reg::read) this register and get [`offsethi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offsethi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@offsethi`]
module"]
#[doc(alias = "OFFSETHI")]
pub type Offsethi = crate::Reg<offsethi::OffsethiSpec>;
#[doc = "High order bytes of offset for IO transaction"]
pub mod offsethi;
#[doc = "CMDSTAT (rw) register accessor: Provides staus on the execution of the command currently in progress. The fields in this register will reflect the real time status of the internal state machines and data transfers within the IOM. These are read only fields and writes to the registers are ignored.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdstat`]
module"]
#[doc(alias = "CMDSTAT")]
pub type Cmdstat = crate::Reg<cmdstat::CmdstatSpec>;
#[doc = "Provides staus on the execution of the command currently in progress. The fields in this register will reflect the real time status of the internal state machines and data transfers within the IOM. These are read only fields and writes to the registers are ignored."]
pub mod cmdstat;
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
#[doc = "DMATRIGEN (rw) register accessor: Provides control on which event will trigger the DMA transfer after the DMA operation is setup and enabled. The trigger event will cause a number of bytes (depending on trigger event) to be transferred via the DMA operation, and can be used to adjust the latency of data to/from the IOM module to/from the dma target. DMA transfers are broken into smaller transfers internally of up to 16 bytes each, and multiple trigger events can be used to complete the entire programmed DMA transfer.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatrigen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatrigen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatrigen`]
module"]
#[doc(alias = "DMATRIGEN")]
pub type Dmatrigen = crate::Reg<dmatrigen::DmatrigenSpec>;
#[doc = "Provides control on which event will trigger the DMA transfer after the DMA operation is setup and enabled. The trigger event will cause a number of bytes (depending on trigger event) to be transferred via the DMA operation, and can be used to adjust the latency of data to/from the IOM module to/from the dma target. DMA transfers are broken into smaller transfers internally of up to 16 bytes each, and multiple trigger events can be used to complete the entire programmed DMA transfer."]
pub mod dmatrigen;
#[doc = "DMATRIGSTAT (rw) register accessor: Provides the status of trigger events that have occurred for the transaction. Some of the bits are read only and some can be reset via a write of 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatrigstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatrigstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatrigstat`]
module"]
#[doc(alias = "DMATRIGSTAT")]
pub type Dmatrigstat = crate::Reg<dmatrigstat::DmatrigstatSpec>;
#[doc = "Provides the status of trigger events that have occurred for the transaction. Some of the bits are read only and some can be reset via a write of 0."]
pub mod dmatrigstat;
#[doc = "DMACFG (rw) register accessor: Configuration control of the DMA process, including the direction of DMA, and enablement of DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacfg`]
module"]
#[doc(alias = "DMACFG")]
pub type Dmacfg = crate::Reg<dmacfg::DmacfgSpec>;
#[doc = "Configuration control of the DMA process, including the direction of DMA, and enablement of DMA"]
pub mod dmacfg;
#[doc = "DMATOTCOUNT (rw) register accessor: Contains the number of bytes to be transferred for this DMA transaction. This register is decremented as the data is transferred, and will be 0 at the completion of the DMA operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatotcount::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatotcount::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatotcount`]
module"]
#[doc(alias = "DMATOTCOUNT")]
pub type Dmatotcount = crate::Reg<dmatotcount::DmatotcountSpec>;
#[doc = "Contains the number of bytes to be transferred for this DMA transaction. This register is decremented as the data is transferred, and will be 0 at the completion of the DMA operation."]
pub mod dmatotcount;
#[doc = "DMATARGADDR (rw) register accessor: The source or destination address internal the SRAM for the DMA data. For write operations, this can only be SRAM data (ADDR bit 28 = 1); For read operations, this can ve either SRAM or FLASH (ADDR bit 28 = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatargaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatargaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatargaddr`]
module"]
#[doc(alias = "DMATARGADDR")]
pub type Dmatargaddr = crate::Reg<dmatargaddr::DmatargaddrSpec>;
#[doc = "The source or destination address internal the SRAM for the DMA data. For write operations, this can only be SRAM data (ADDR bit 28 = 1); For read operations, this can ve either SRAM or FLASH (ADDR bit 28 = 0)"]
pub mod dmatargaddr;
#[doc = "DMASTAT (rw) register accessor: Status of the DMA operation currently in progress.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmastat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmastat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmastat`]
module"]
#[doc(alias = "DMASTAT")]
pub type Dmastat = crate::Reg<dmastat::DmastatSpec>;
#[doc = "Status of the DMA operation currently in progress."]
pub mod dmastat;
#[doc = "CQCFG (rw) register accessor: Controls parameters and options for execution of the command queue operation. To enable command queue, create this in memory, set the address, and enable it with a write to CQEN\n\nYou can [`read`](crate::Reg::read) this register and get [`cqcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqcfg`]
module"]
#[doc(alias = "CQCFG")]
pub type Cqcfg = crate::Reg<cqcfg::CqcfgSpec>;
#[doc = "Controls parameters and options for execution of the command queue operation. To enable command queue, create this in memory, set the address, and enable it with a write to CQEN"]
pub mod cqcfg;
#[doc = "CQADDR (rw) register accessor: The SRAM address which will be fetched next execution of the CQ operation. This register is updated as the CQ operation progresses, and is the live version of the register. The register can also be written by the Command Queue operation itself, allowing the relocation of successive CQ fetches. In this case, the new CQ address will be used for the next CQ address/data fetch\n\nYou can [`read`](crate::Reg::read) this register and get [`cqaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqaddr`]
module"]
#[doc(alias = "CQADDR")]
pub type Cqaddr = crate::Reg<cqaddr::CqaddrSpec>;
#[doc = "The SRAM address which will be fetched next execution of the CQ operation. This register is updated as the CQ operation progresses, and is the live version of the register. The register can also be written by the Command Queue operation itself, allowing the relocation of successive CQ fetches. In this case, the new CQ address will be used for the next CQ address/data fetch"]
pub mod cqaddr;
#[doc = "CQSTAT (rw) register accessor: Provides the status of the command queue operation. If the command queue is disabled, these bits will be cleared. The bits are read only\n\nYou can [`read`](crate::Reg::read) this register and get [`cqstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqstat`]
module"]
#[doc(alias = "CQSTAT")]
pub type Cqstat = crate::Reg<cqstat::CqstatSpec>;
#[doc = "Provides the status of the command queue operation. If the command queue is disabled, these bits will be cleared. The bits are read only"]
pub mod cqstat;
#[doc = "CQFLAGS (rw) register accessor: Command Queue Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`cqflags::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqflags::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqflags`]
module"]
#[doc(alias = "CQFLAGS")]
pub type Cqflags = crate::Reg<cqflags::CqflagsSpec>;
#[doc = "Command Queue Flag"]
pub mod cqflags;
#[doc = "CQSETCLEAR (rw) register accessor: Set/Clear the command queue software pause flags on a per-bit basis. Contains 3 fields, allowing for setting, clearing or toggling the value in the software flags. Priority when the same bit is enabled in each field is toggle, then set, then clear.\n\nYou can [`read`](crate::Reg::read) this register and get [`cqsetclear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqsetclear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqsetclear`]
module"]
#[doc(alias = "CQSETCLEAR")]
pub type Cqsetclear = crate::Reg<cqsetclear::CqsetclearSpec>;
#[doc = "Set/Clear the command queue software pause flags on a per-bit basis. Contains 3 fields, allowing for setting, clearing or toggling the value in the software flags. Priority when the same bit is enabled in each field is toggle, then set, then clear."]
pub mod cqsetclear;
#[doc = "CQPAUSEEN (rw) register accessor: Enables a flag to pause an active command queue operation. If a bit is '1' and the corresponding bit in the CQFLAG register is '1', CQ processing will halt until either value is changed to '0'.\n\nYou can [`read`](crate::Reg::read) this register and get [`cqpauseen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqpauseen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqpauseen`]
module"]
#[doc(alias = "CQPAUSEEN")]
pub type Cqpauseen = crate::Reg<cqpauseen::CqpauseenSpec>;
#[doc = "Enables a flag to pause an active command queue operation. If a bit is '1' and the corresponding bit in the CQFLAG register is '1', CQ processing will halt until either value is changed to '0'."]
pub mod cqpauseen;
#[doc = "CQCURIDX (rw) register accessor: Current index value, targeted to be written by register write operations within the command queue. This is compared to the CQENDIDX and will stop the CQ operation if bit 15 of the CQPAUSEEN is '1' and this current index equals the CQENDIDX register value. This will only pause when the values are equal.\n\nYou can [`read`](crate::Reg::read) this register and get [`cqcuridx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqcuridx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqcuridx`]
module"]
#[doc(alias = "CQCURIDX")]
pub type Cqcuridx = crate::Reg<cqcuridx::CqcuridxSpec>;
#[doc = "Current index value, targeted to be written by register write operations within the command queue. This is compared to the CQENDIDX and will stop the CQ operation if bit 15 of the CQPAUSEEN is '1' and this current index equals the CQENDIDX register value. This will only pause when the values are equal."]
pub mod cqcuridx;
#[doc = "CQENDIDX (rw) register accessor: End index value, targeted to be written by software to indicate the last valid register pair contained within the command queue. rgister write operations within the command queue. This is compared to the CQCURIDX and will stop the CQ operation if bit 15 of the CQPAUSEEN is '1' and this current index equals the CQCURIDX register value. This will only pause when the values are equal.\n\nYou can [`read`](crate::Reg::read) this register and get [`cqendidx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqendidx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqendidx`]
module"]
#[doc(alias = "CQENDIDX")]
pub type Cqendidx = crate::Reg<cqendidx::CqendidxSpec>;
#[doc = "End index value, targeted to be written by software to indicate the last valid register pair contained within the command queue. rgister write operations within the command queue. This is compared to the CQCURIDX and will stop the CQ operation if bit 15 of the CQPAUSEEN is '1' and this current index equals the CQCURIDX register value. This will only pause when the values are equal."]
pub mod cqendidx;
#[doc = "STATUS (rw) register accessor: IOM Module Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "IOM Module Status"]
pub mod status;
#[doc = "MSPICFG (rw) register accessor: Controls the configuration of the SPI master module, including POL/PHA, LSB, flow control, and delays for MISO and MOSI\n\nYou can [`read`](crate::Reg::read) this register and get [`mspicfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspicfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspicfg`]
module"]
#[doc(alias = "MSPICFG")]
pub type Mspicfg = crate::Reg<mspicfg::MspicfgSpec>;
#[doc = "Controls the configuration of the SPI master module, including POL/PHA, LSB, flow control, and delays for MISO and MOSI"]
pub mod mspicfg;
#[doc = "MI2CCFG (rw) register accessor: Controls the configuration of the I2C bus master.\n\nYou can [`read`](crate::Reg::read) this register and get [`mi2ccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mi2ccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi2ccfg`]
module"]
#[doc(alias = "MI2CCFG")]
pub type Mi2ccfg = crate::Reg<mi2ccfg::Mi2ccfgSpec>;
#[doc = "Controls the configuration of the I2C bus master."]
pub mod mi2ccfg;
#[doc = "DEVCFG (rw) register accessor: Contains the I2C device address.\n\nYou can [`read`](crate::Reg::read) this register and get [`devcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devcfg`]
module"]
#[doc(alias = "DEVCFG")]
pub type Devcfg = crate::Reg<devcfg::DevcfgSpec>;
#[doc = "Contains the I2C device address."]
pub mod devcfg;
#[doc = "IOMDBG (rw) register accessor: Debug control\n\nYou can [`read`](crate::Reg::read) this register and get [`iomdbg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomdbg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomdbg`]
module"]
#[doc(alias = "IOMDBG")]
pub type Iomdbg = crate::Reg<iomdbg::IomdbgSpec>;
#[doc = "Debug control"]
pub mod iomdbg;
