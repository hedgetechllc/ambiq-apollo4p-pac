#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `START` reader - Write to 1 to initiate a PIO transaction on the bus (typically the entire register should be written at once with this bit set)."]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Write to 1 to initiate a PIO transaction on the bus (typically the entire register should be written at once with this bit set)."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATUS` reader - Command status: 1 indicates command has completed. Cleared by writing 1 to this bit or starting a new transfer."]
pub type StatusR = crate::BitReader;
#[doc = "Field `STATUS` writer - Command status: 1 indicates command has completed. Cleared by writing 1 to this bit or starting a new transfer."]
pub type StatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - Command status: 1 indicates controller is busy (command in progress)"]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - Command status: 1 indicates controller is busy (command in progress)"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Selects the Device configutation to use for PIO requests\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piodev {
    #[doc = "0: Use DEVICE0 Configuration"]
    Device0 = 0,
    #[doc = "1: Use DEVICE1 CONFIGURATION"]
    Device1 = 1,
}
impl From<Piodev> for bool {
    #[inline(always)]
    fn from(variant: Piodev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIODEV` reader - Selects the Device configutation to use for PIO requests"]
pub type PiodevR = crate::BitReader<Piodev>;
impl PiodevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Piodev {
        match self.bits {
            false => Piodev::Device0,
            true => Piodev::Device1,
        }
    }
    #[doc = "Use DEVICE0 Configuration"]
    #[inline(always)]
    pub fn is_device0(&self) -> bool {
        *self == Piodev::Device0
    }
    #[doc = "Use DEVICE1 CONFIGURATION"]
    #[inline(always)]
    pub fn is_device1(&self) -> bool {
        *self == Piodev::Device1
    }
}
#[doc = "Field `PIODEV` writer - Selects the Device configutation to use for PIO requests"]
pub type PiodevW<'a, REG> = crate::BitWriter<'a, REG, Piodev>;
impl<'a, REG> PiodevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use DEVICE0 Configuration"]
    #[inline(always)]
    pub fn device0(self) -> &'a mut crate::W<REG> {
        self.variant(Piodev::Device0)
    }
    #[doc = "Use DEVICE1 CONFIGURATION"]
    #[inline(always)]
    pub fn device1(self) -> &'a mut crate::W<REG> {
        self.variant(Piodev::Device1)
    }
}
#[doc = "Field `SENDA` reader - Indicates whether an address phase should be sent (see ADDR register and ASIZE field in CFG register)"]
pub type SendaR = crate::BitReader;
#[doc = "Field `SENDA` writer - Indicates whether an address phase should be sent (see ADDR register and ASIZE field in CFG register)"]
pub type SendaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENDI` reader - Indicates whether an instruction phase should be sent (see INSTR field and ISIZE field in CFG register)"]
pub type SendiR = crate::BitReader;
#[doc = "Field `SENDI` writer - Indicates whether an instruction phase should be sent (see INSTR field and ISIZE field in CFG register)"]
pub type SendiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRX` reader - 1 Indicates a TX operation, 0 indicates an RX operation of XFERBYTES"]
pub type TxrxR = crate::BitReader;
#[doc = "Field `TXRX` writer - 1 Indicates a TX operation, 0 indicates an RX operation of XFERBYTES"]
pub type TxrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIGENDIAN` reader - 1 indicates data in FIFO is in big endian format (MSB first); 0 indicates little endian data (default, LSB first)."]
pub type BigendianR = crate::BitReader;
#[doc = "Field `BIGENDIAN` writer - 1 indicates data in FIFO is in big endian format (MSB first); 0 indicates little endian data (default, LSB first)."]
pub type BigendianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIOSCRAMBLE` reader - Enables data scrambling for PIO opertions. This should only be used for data operations and never for commands to a device."]
pub type PioscrambleR = crate::BitReader;
#[doc = "Field `PIOSCRAMBLE` writer - Enables data scrambling for PIO opertions. This should only be used for data operations and never for commands to a device."]
pub type PioscrambleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENTURN` reader - Indicates whether TX->RX turnaround cycles should be enabled for this operation (see TURNAROUND field in CFG register)."]
pub type EnturnR = crate::BitReader;
#[doc = "Field `ENTURN` writer - Indicates whether TX->RX turnaround cycles should be enabled for this operation (see TURNAROUND field in CFG register)."]
pub type EnturnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDCX` reader - Enable DCX signal on data \\[1\\]"]
pub type EndcxR = crate::BitReader;
#[doc = "Field `ENDCX` writer - Enable DCX signal on data \\[1\\]"]
pub type EndcxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENWLAT` reader - Enable Write Latency Counter (time between address and first data byte). Counter value is WRITELATENCY."]
pub type EnwlatR = crate::BitReader;
#[doc = "Field `ENWLAT` writer - Enable Write Latency Counter (time between address and first data byte). Counter value is WRITELATENCY."]
pub type EnwlatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XFERBYTES` reader - Number of bytes to transmit or receive (based on TXRX bit)"]
pub type XferbytesR = crate::FieldReader<u16>;
#[doc = "Field `XFERBYTES` writer - Number of bytes to transmit or receive (based on TXRX bit)"]
pub type XferbytesW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Write to 1 to initiate a PIO transaction on the bus (typically the entire register should be written at once with this bit set)."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command status: 1 indicates command has completed. Cleared by writing 1 to this bit or starting a new transfer."]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command status: 1 indicates controller is busy (command in progress)"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects the Device configutation to use for PIO requests"]
    #[inline(always)]
    pub fn piodev(&self) -> PiodevR {
        PiodevR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates whether an address phase should be sent (see ADDR register and ASIZE field in CFG register)"]
    #[inline(always)]
    pub fn senda(&self) -> SendaR {
        SendaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates whether an instruction phase should be sent (see INSTR field and ISIZE field in CFG register)"]
    #[inline(always)]
    pub fn sendi(&self) -> SendiR {
        SendiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1 Indicates a TX operation, 0 indicates an RX operation of XFERBYTES"]
    #[inline(always)]
    pub fn txrx(&self) -> TxrxR {
        TxrxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1 indicates data in FIFO is in big endian format (MSB first); 0 indicates little endian data (default, LSB first)."]
    #[inline(always)]
    pub fn bigendian(&self) -> BigendianR {
        BigendianR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enables data scrambling for PIO opertions. This should only be used for data operations and never for commands to a device."]
    #[inline(always)]
    pub fn pioscramble(&self) -> PioscrambleR {
        PioscrambleR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates whether TX->RX turnaround cycles should be enabled for this operation (see TURNAROUND field in CFG register)."]
    #[inline(always)]
    pub fn enturn(&self) -> EnturnR {
        EnturnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable DCX signal on data \\[1\\]"]
    #[inline(always)]
    pub fn endcx(&self) -> EndcxR {
        EndcxR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Latency Counter (time between address and first data byte). Counter value is WRITELATENCY."]
    #[inline(always)]
    pub fn enwlat(&self) -> EnwlatR {
        EnwlatR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Number of bytes to transmit or receive (based on TXRX bit)"]
    #[inline(always)]
    pub fn xferbytes(&self) -> XferbytesR {
        XferbytesR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Write to 1 to initiate a PIO transaction on the bus (typically the entire register should be written at once with this bit set)."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<CtrlSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Command status: 1 indicates command has completed. Cleared by writing 1 to this bit or starting a new transfer."]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> StatusW<CtrlSpec> {
        StatusW::new(self, 1)
    }
    #[doc = "Bit 2 - Command status: 1 indicates controller is busy (command in progress)"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BusyW<CtrlSpec> {
        BusyW::new(self, 2)
    }
    #[doc = "Bit 4 - Selects the Device configutation to use for PIO requests"]
    #[inline(always)]
    #[must_use]
    pub fn piodev(&mut self) -> PiodevW<CtrlSpec> {
        PiodevW::new(self, 4)
    }
    #[doc = "Bit 5 - Indicates whether an address phase should be sent (see ADDR register and ASIZE field in CFG register)"]
    #[inline(always)]
    #[must_use]
    pub fn senda(&mut self) -> SendaW<CtrlSpec> {
        SendaW::new(self, 5)
    }
    #[doc = "Bit 6 - Indicates whether an instruction phase should be sent (see INSTR field and ISIZE field in CFG register)"]
    #[inline(always)]
    #[must_use]
    pub fn sendi(&mut self) -> SendiW<CtrlSpec> {
        SendiW::new(self, 6)
    }
    #[doc = "Bit 7 - 1 Indicates a TX operation, 0 indicates an RX operation of XFERBYTES"]
    #[inline(always)]
    #[must_use]
    pub fn txrx(&mut self) -> TxrxW<CtrlSpec> {
        TxrxW::new(self, 7)
    }
    #[doc = "Bit 8 - 1 indicates data in FIFO is in big endian format (MSB first); 0 indicates little endian data (default, LSB first)."]
    #[inline(always)]
    #[must_use]
    pub fn bigendian(&mut self) -> BigendianW<CtrlSpec> {
        BigendianW::new(self, 8)
    }
    #[doc = "Bit 9 - Enables data scrambling for PIO opertions. This should only be used for data operations and never for commands to a device."]
    #[inline(always)]
    #[must_use]
    pub fn pioscramble(&mut self) -> PioscrambleW<CtrlSpec> {
        PioscrambleW::new(self, 9)
    }
    #[doc = "Bit 10 - Indicates whether TX->RX turnaround cycles should be enabled for this operation (see TURNAROUND field in CFG register)."]
    #[inline(always)]
    #[must_use]
    pub fn enturn(&mut self) -> EnturnW<CtrlSpec> {
        EnturnW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable DCX signal on data \\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn endcx(&mut self) -> EndcxW<CtrlSpec> {
        EndcxW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Latency Counter (time between address and first data byte). Counter value is WRITELATENCY."]
    #[inline(always)]
    #[must_use]
    pub fn enwlat(&mut self) -> EnwlatW<CtrlSpec> {
        EnwlatW::new(self, 12)
    }
    #[doc = "Bits 16:31 - Number of bytes to transmit or receive (based on TXRX bit)"]
    #[inline(always)]
    #[must_use]
    pub fn xferbytes(&mut self) -> XferbytesW<CtrlSpec> {
        XferbytesW::new(self, 16)
    }
}
#[doc = "This register is used to enable individual PIO based transactions to a device on the bus. The CFG register must be programmed properly for the transfer, and the ADDR and INSTR registers should be programmed if the SENDI and SENDA fields are enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
