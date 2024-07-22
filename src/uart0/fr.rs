#[doc = "Register `FR` reader"]
pub type R = crate::R<FrSpec>;
#[doc = "Register `FR` writer"]
pub type W = crate::W<FrSpec>;
#[doc = "Clear to send. This bit is the complement of the UART clear to send, nUARTCTS, modem status input. That is, the bit is 1 when nUARTCTS is LOW.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cts {
    #[doc = "1: Clear to send is indicated."]
    Cleartosend = 1,
    #[doc = "0: Clear to send default value."]
    Default = 0,
}
impl From<Cts> for bool {
    #[inline(always)]
    fn from(variant: Cts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTS` reader - Clear to send. This bit is the complement of the UART clear to send, nUARTCTS, modem status input. That is, the bit is 1 when nUARTCTS is LOW."]
pub type CtsR = crate::BitReader<Cts>;
impl CtsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cts {
        match self.bits {
            true => Cts::Cleartosend,
            false => Cts::Default,
        }
    }
    #[doc = "Clear to send is indicated."]
    #[inline(always)]
    pub fn is_cleartosend(&self) -> bool {
        *self == Cts::Cleartosend
    }
    #[doc = "Clear to send default value."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Cts::Default
    }
}
#[doc = "Field `CTS` writer - Clear to send. This bit is the complement of the UART clear to send, nUARTCTS, modem status input. That is, the bit is 1 when nUARTCTS is LOW."]
pub type CtsW<'a, REG> = crate::BitWriter<'a, REG, Cts>;
impl<'a, REG> CtsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear to send is indicated."]
    #[inline(always)]
    pub fn cleartosend(self) -> &'a mut crate::W<REG> {
        self.variant(Cts::Cleartosend)
    }
    #[doc = "Clear to send default value."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Cts::Default)
    }
}
#[doc = "Data set ready. This bit is the complement of the UART data set ready, nUARTDSR, modem status input. That is, the bit is 1 when nUARTDSR is LOW.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsr {
    #[doc = "1: Data set ready."]
    Ready = 1,
    #[doc = "0: Data set not ready/default."]
    Notready = 0,
}
impl From<Dsr> for bool {
    #[inline(always)]
    fn from(variant: Dsr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSR` reader - Data set ready. This bit is the complement of the UART data set ready, nUARTDSR, modem status input. That is, the bit is 1 when nUARTDSR is LOW."]
pub type DsrR = crate::BitReader<Dsr>;
impl DsrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsr {
        match self.bits {
            true => Dsr::Ready,
            false => Dsr::Notready,
        }
    }
    #[doc = "Data set ready."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Dsr::Ready
    }
    #[doc = "Data set not ready/default."]
    #[inline(always)]
    pub fn is_notready(&self) -> bool {
        *self == Dsr::Notready
    }
}
#[doc = "Field `DSR` writer - Data set ready. This bit is the complement of the UART data set ready, nUARTDSR, modem status input. That is, the bit is 1 when nUARTDSR is LOW."]
pub type DsrW<'a, REG> = crate::BitWriter<'a, REG, Dsr>;
impl<'a, REG> DsrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data set ready."]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(Dsr::Ready)
    }
    #[doc = "Data set not ready/default."]
    #[inline(always)]
    pub fn notready(self) -> &'a mut crate::W<REG> {
        self.variant(Dsr::Notready)
    }
}
#[doc = "Data carrier detect. This bit is the complement of the UART data carrier detect, nUARTDCD, modem status input. That is, the bit is 1 when nUARTDCD is LOW.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcd {
    #[doc = "1: Data carrier detect detected."]
    Detected = 1,
    #[doc = "0: Data carrier detect not detected/default."]
    Default = 0,
}
impl From<Dcd> for bool {
    #[inline(always)]
    fn from(variant: Dcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCD` reader - Data carrier detect. This bit is the complement of the UART data carrier detect, nUARTDCD, modem status input. That is, the bit is 1 when nUARTDCD is LOW."]
pub type DcdR = crate::BitReader<Dcd>;
impl DcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcd {
        match self.bits {
            true => Dcd::Detected,
            false => Dcd::Default,
        }
    }
    #[doc = "Data carrier detect detected."]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Dcd::Detected
    }
    #[doc = "Data carrier detect not detected/default."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Dcd::Default
    }
}
#[doc = "Field `DCD` writer - Data carrier detect. This bit is the complement of the UART data carrier detect, nUARTDCD, modem status input. That is, the bit is 1 when nUARTDCD is LOW."]
pub type DcdW<'a, REG> = crate::BitWriter<'a, REG, Dcd>;
impl<'a, REG> DcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data carrier detect detected."]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Dcd::Detected)
    }
    #[doc = "Data carrier detect not detected/default."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Dcd::Default)
    }
}
#[doc = "UART busy. If this bit is set to 1, the UART is busy transmitting data. This bit remains set until the complete byte, including all the stop bits, has been sent from the shift register. This bit is set as soon as the transmit FIFO becomes non-empty, regardless of whether the UART is enabled or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "1: UART busy indicator."]
    Busy = 1,
    #[doc = "0: UART not busy."]
    Notbusy = 0,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - UART busy. If this bit is set to 1, the UART is busy transmitting data. This bit remains set until the complete byte, including all the stop bits, has been sent from the shift register. This bit is set as soon as the transmit FIFO becomes non-empty, regardless of whether the UART is enabled or not."]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            true => Busy::Busy,
            false => Busy::Notbusy,
        }
    }
    #[doc = "UART busy indicator."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Busy::Busy
    }
    #[doc = "UART not busy."]
    #[inline(always)]
    pub fn is_notbusy(&self) -> bool {
        *self == Busy::Notbusy
    }
}
#[doc = "Field `BUSY` writer - UART busy. If this bit is set to 1, the UART is busy transmitting data. This bit remains set until the complete byte, including all the stop bits, has been sent from the shift register. This bit is set as soon as the transmit FIFO becomes non-empty, regardless of whether the UART is enabled or not."]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG, Busy>;
impl<'a, REG> BusyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UART busy indicator."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(Busy::Busy)
    }
    #[doc = "UART not busy."]
    #[inline(always)]
    pub fn notbusy(self) -> &'a mut crate::W<REG> {
        self.variant(Busy::Notbusy)
    }
}
#[doc = "Receive FIFO empty. The meaning of this bit depends on the state of the FEN bit in the UARTLCRH Register. If the FIFO is disabled, this bit is set when the receive holding register is empty. If the FIFO is enabled, the RXFE bit is set when the receive FIFO is empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxfe {
    #[doc = "1: Receive fifo is empty."]
    RcvfifoEmpty = 1,
    #[doc = "0: Receive fifo is not empty."]
    RcvfifoNotempty = 0,
}
impl From<Rxfe> for bool {
    #[inline(always)]
    fn from(variant: Rxfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFE` reader - Receive FIFO empty. The meaning of this bit depends on the state of the FEN bit in the UARTLCRH Register. If the FIFO is disabled, this bit is set when the receive holding register is empty. If the FIFO is enabled, the RXFE bit is set when the receive FIFO is empty."]
pub type RxfeR = crate::BitReader<Rxfe>;
impl RxfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxfe {
        match self.bits {
            true => Rxfe::RcvfifoEmpty,
            false => Rxfe::RcvfifoNotempty,
        }
    }
    #[doc = "Receive fifo is empty."]
    #[inline(always)]
    pub fn is_rcvfifo_empty(&self) -> bool {
        *self == Rxfe::RcvfifoEmpty
    }
    #[doc = "Receive fifo is not empty."]
    #[inline(always)]
    pub fn is_rcvfifo_notempty(&self) -> bool {
        *self == Rxfe::RcvfifoNotempty
    }
}
#[doc = "Field `RXFE` writer - Receive FIFO empty. The meaning of this bit depends on the state of the FEN bit in the UARTLCRH Register. If the FIFO is disabled, this bit is set when the receive holding register is empty. If the FIFO is enabled, the RXFE bit is set when the receive FIFO is empty."]
pub type RxfeW<'a, REG> = crate::BitWriter<'a, REG, Rxfe>;
impl<'a, REG> RxfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive fifo is empty."]
    #[inline(always)]
    pub fn rcvfifo_empty(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfe::RcvfifoEmpty)
    }
    #[doc = "Receive fifo is not empty."]
    #[inline(always)]
    pub fn rcvfifo_notempty(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfe::RcvfifoNotempty)
    }
}
#[doc = "Transmit FIFO full. The meaning of this bit depends on the state of the FEN bit in the UARTLCRH Register. If the FIFO is disabled, this bit is set when the transmit holding register is full. If the FIFO is enabled, the TXFF bit is set when the transmit FIFO is full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txff {
    #[doc = "1: Transmit fifo is full."]
    XmtfifoFull = 1,
    #[doc = "0: Transmit fifo is not full."]
    XmtfifoNotfull = 0,
}
impl From<Txff> for bool {
    #[inline(always)]
    fn from(variant: Txff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFF` reader - Transmit FIFO full. The meaning of this bit depends on the state of the FEN bit in the UARTLCRH Register. If the FIFO is disabled, this bit is set when the transmit holding register is full. If the FIFO is enabled, the TXFF bit is set when the transmit FIFO is full."]
pub type TxffR = crate::BitReader<Txff>;
impl TxffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txff {
        match self.bits {
            true => Txff::XmtfifoFull,
            false => Txff::XmtfifoNotfull,
        }
    }
    #[doc = "Transmit fifo is full."]
    #[inline(always)]
    pub fn is_xmtfifo_full(&self) -> bool {
        *self == Txff::XmtfifoFull
    }
    #[doc = "Transmit fifo is not full."]
    #[inline(always)]
    pub fn is_xmtfifo_notfull(&self) -> bool {
        *self == Txff::XmtfifoNotfull
    }
}
#[doc = "Field `TXFF` writer - Transmit FIFO full. The meaning of this bit depends on the state of the FEN bit in the UARTLCRH Register. If the FIFO is disabled, this bit is set when the transmit holding register is full. If the FIFO is enabled, the TXFF bit is set when the transmit FIFO is full."]
pub type TxffW<'a, REG> = crate::BitWriter<'a, REG, Txff>;
impl<'a, REG> TxffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit fifo is full."]
    #[inline(always)]
    pub fn xmtfifo_full(self) -> &'a mut crate::W<REG> {
        self.variant(Txff::XmtfifoFull)
    }
    #[doc = "Transmit fifo is not full."]
    #[inline(always)]
    pub fn xmtfifo_notfull(self) -> &'a mut crate::W<REG> {
        self.variant(Txff::XmtfifoNotfull)
    }
}
#[doc = "Receive FIFO full. The meaning of this bit depends on the state of the FEN bit in the UARTLCRH Register. If the FIFO is disabled, this bit is set when the receive holding register is full. If the FIFO is enabled, the RXFF bit is set when the receive FIFO is full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxff {
    #[doc = "1: Receive fifo is full."]
    RcvfifoFull = 1,
    #[doc = "0: Receive fifo is not full."]
    RcvfifoNotfull = 0,
}
impl From<Rxff> for bool {
    #[inline(always)]
    fn from(variant: Rxff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFF` reader - Receive FIFO full. The meaning of this bit depends on the state of the FEN bit in the UARTLCRH Register. If the FIFO is disabled, this bit is set when the receive holding register is full. If the FIFO is enabled, the RXFF bit is set when the receive FIFO is full."]
pub type RxffR = crate::BitReader<Rxff>;
impl RxffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxff {
        match self.bits {
            true => Rxff::RcvfifoFull,
            false => Rxff::RcvfifoNotfull,
        }
    }
    #[doc = "Receive fifo is full."]
    #[inline(always)]
    pub fn is_rcvfifo_full(&self) -> bool {
        *self == Rxff::RcvfifoFull
    }
    #[doc = "Receive fifo is not full."]
    #[inline(always)]
    pub fn is_rcvfifo_notfull(&self) -> bool {
        *self == Rxff::RcvfifoNotfull
    }
}
#[doc = "Field `RXFF` writer - Receive FIFO full. The meaning of this bit depends on the state of the FEN bit in the UARTLCRH Register. If the FIFO is disabled, this bit is set when the receive holding register is full. If the FIFO is enabled, the RXFF bit is set when the receive FIFO is full."]
pub type RxffW<'a, REG> = crate::BitWriter<'a, REG, Rxff>;
impl<'a, REG> RxffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive fifo is full."]
    #[inline(always)]
    pub fn rcvfifo_full(self) -> &'a mut crate::W<REG> {
        self.variant(Rxff::RcvfifoFull)
    }
    #[doc = "Receive fifo is not full."]
    #[inline(always)]
    pub fn rcvfifo_notfull(self) -> &'a mut crate::W<REG> {
        self.variant(Rxff::RcvfifoNotfull)
    }
}
#[doc = "Transmit FIFO empty. The meaning of this bit depends on the state of the FEN bit in the Line Control Register, UARTLCRH. If the FIFO is disabled, this bit is set when the transmit holding register is empty. If the FIFO is enabled, the TXFE bit is set when the transmit FIFO is empty. This bit does not indicate if there is data in the transmit shift register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txfe {
    #[doc = "1: Transmit fifo is empty."]
    XmtfifoEmpty = 1,
    #[doc = "0: Transmit fifo is not empty."]
    XmtfifoNotempty = 0,
}
impl From<Txfe> for bool {
    #[inline(always)]
    fn from(variant: Txfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFE` reader - Transmit FIFO empty. The meaning of this bit depends on the state of the FEN bit in the Line Control Register, UARTLCRH. If the FIFO is disabled, this bit is set when the transmit holding register is empty. If the FIFO is enabled, the TXFE bit is set when the transmit FIFO is empty. This bit does not indicate if there is data in the transmit shift register."]
pub type TxfeR = crate::BitReader<Txfe>;
impl TxfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txfe {
        match self.bits {
            true => Txfe::XmtfifoEmpty,
            false => Txfe::XmtfifoNotempty,
        }
    }
    #[doc = "Transmit fifo is empty."]
    #[inline(always)]
    pub fn is_xmtfifo_empty(&self) -> bool {
        *self == Txfe::XmtfifoEmpty
    }
    #[doc = "Transmit fifo is not empty."]
    #[inline(always)]
    pub fn is_xmtfifo_notempty(&self) -> bool {
        *self == Txfe::XmtfifoNotempty
    }
}
#[doc = "Field `TXFE` writer - Transmit FIFO empty. The meaning of this bit depends on the state of the FEN bit in the Line Control Register, UARTLCRH. If the FIFO is disabled, this bit is set when the transmit holding register is empty. If the FIFO is enabled, the TXFE bit is set when the transmit FIFO is empty. This bit does not indicate if there is data in the transmit shift register."]
pub type TxfeW<'a, REG> = crate::BitWriter<'a, REG, Txfe>;
impl<'a, REG> TxfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit fifo is empty."]
    #[inline(always)]
    pub fn xmtfifo_empty(self) -> &'a mut crate::W<REG> {
        self.variant(Txfe::XmtfifoEmpty)
    }
    #[doc = "Transmit fifo is not empty."]
    #[inline(always)]
    pub fn xmtfifo_notempty(self) -> &'a mut crate::W<REG> {
        self.variant(Txfe::XmtfifoNotempty)
    }
}
#[doc = "Field `TXBUSY` reader - This bit holds the transmit BUSY indicator."]
pub type TxbusyR = crate::BitReader;
#[doc = "Field `TXBUSY` writer - This bit holds the transmit BUSY indicator."]
pub type TxbusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear to send. This bit is the complement of the UART clear to send, nUARTCTS, modem status input. That is, the bit is 1 when nUARTCTS is LOW."]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data set ready. This bit is the complement of the UART data set ready, nUARTDSR, modem status input. That is, the bit is 1 when nUARTDSR is LOW."]
    #[inline(always)]
    pub fn dsr(&self) -> DsrR {
        DsrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data carrier detect. This bit is the complement of the UART data carrier detect, nUARTDCD, modem status input. That is, the bit is 1 when nUARTDCD is LOW."]
    #[inline(always)]
    pub fn dcd(&self) -> DcdR {
        DcdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART busy. If this bit is set to 1, the UART is busy transmitting data. This bit remains set until the complete byte, including all the stop bits, has been sent from the shift register. This bit is set as soon as the transmit FIFO becomes non-empty, regardless of whether the UART is enabled or not."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO empty. The meaning of this bit depends on the state of the FEN bit in the UARTLCRH Register. If the FIFO is disabled, this bit is set when the receive holding register is empty. If the FIFO is enabled, the RXFE bit is set when the receive FIFO is empty."]
    #[inline(always)]
    pub fn rxfe(&self) -> RxfeR {
        RxfeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO full. The meaning of this bit depends on the state of the FEN bit in the UARTLCRH Register. If the FIFO is disabled, this bit is set when the transmit holding register is full. If the FIFO is enabled, the TXFF bit is set when the transmit FIFO is full."]
    #[inline(always)]
    pub fn txff(&self) -> TxffR {
        TxffR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO full. The meaning of this bit depends on the state of the FEN bit in the UARTLCRH Register. If the FIFO is disabled, this bit is set when the receive holding register is full. If the FIFO is enabled, the RXFF bit is set when the receive FIFO is full."]
    #[inline(always)]
    pub fn rxff(&self) -> RxffR {
        RxffR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO empty. The meaning of this bit depends on the state of the FEN bit in the Line Control Register, UARTLCRH. If the FIFO is disabled, this bit is set when the transmit holding register is empty. If the FIFO is enabled, the TXFE bit is set when the transmit FIFO is empty. This bit does not indicate if there is data in the transmit shift register."]
    #[inline(always)]
    pub fn txfe(&self) -> TxfeR {
        TxfeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit holds the transmit BUSY indicator."]
    #[inline(always)]
    pub fn txbusy(&self) -> TxbusyR {
        TxbusyR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear to send. This bit is the complement of the UART clear to send, nUARTCTS, modem status input. That is, the bit is 1 when nUARTCTS is LOW."]
    #[inline(always)]
    #[must_use]
    pub fn cts(&mut self) -> CtsW<FrSpec> {
        CtsW::new(self, 0)
    }
    #[doc = "Bit 1 - Data set ready. This bit is the complement of the UART data set ready, nUARTDSR, modem status input. That is, the bit is 1 when nUARTDSR is LOW."]
    #[inline(always)]
    #[must_use]
    pub fn dsr(&mut self) -> DsrW<FrSpec> {
        DsrW::new(self, 1)
    }
    #[doc = "Bit 2 - Data carrier detect. This bit is the complement of the UART data carrier detect, nUARTDCD, modem status input. That is, the bit is 1 when nUARTDCD is LOW."]
    #[inline(always)]
    #[must_use]
    pub fn dcd(&mut self) -> DcdW<FrSpec> {
        DcdW::new(self, 2)
    }
    #[doc = "Bit 3 - UART busy. If this bit is set to 1, the UART is busy transmitting data. This bit remains set until the complete byte, including all the stop bits, has been sent from the shift register. This bit is set as soon as the transmit FIFO becomes non-empty, regardless of whether the UART is enabled or not."]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BusyW<FrSpec> {
        BusyW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO empty. The meaning of this bit depends on the state of the FEN bit in the UARTLCRH Register. If the FIFO is disabled, this bit is set when the receive holding register is empty. If the FIFO is enabled, the RXFE bit is set when the receive FIFO is empty."]
    #[inline(always)]
    #[must_use]
    pub fn rxfe(&mut self) -> RxfeW<FrSpec> {
        RxfeW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit FIFO full. The meaning of this bit depends on the state of the FEN bit in the UARTLCRH Register. If the FIFO is disabled, this bit is set when the transmit holding register is full. If the FIFO is enabled, the TXFF bit is set when the transmit FIFO is full."]
    #[inline(always)]
    #[must_use]
    pub fn txff(&mut self) -> TxffW<FrSpec> {
        TxffW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive FIFO full. The meaning of this bit depends on the state of the FEN bit in the UARTLCRH Register. If the FIFO is disabled, this bit is set when the receive holding register is full. If the FIFO is enabled, the RXFF bit is set when the receive FIFO is full."]
    #[inline(always)]
    #[must_use]
    pub fn rxff(&mut self) -> RxffW<FrSpec> {
        RxffW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit FIFO empty. The meaning of this bit depends on the state of the FEN bit in the Line Control Register, UARTLCRH. If the FIFO is disabled, this bit is set when the transmit holding register is empty. If the FIFO is enabled, the TXFE bit is set when the transmit FIFO is empty. This bit does not indicate if there is data in the transmit shift register."]
    #[inline(always)]
    #[must_use]
    pub fn txfe(&mut self) -> TxfeW<FrSpec> {
        TxfeW::new(self, 7)
    }
    #[doc = "Bit 8 - This bit holds the transmit BUSY indicator."]
    #[inline(always)]
    #[must_use]
    pub fn txbusy(&mut self) -> TxbusyW<FrSpec> {
        TxbusyW::new(self, 8)
    }
}
#[doc = "Flags\n\nYou can [`read`](crate::Reg::read) this register and get [`fr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrSpec;
impl crate::RegisterSpec for FrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fr::R`](R) reader structure"]
impl crate::Readable for FrSpec {}
#[doc = "`write(|w| ..)` method takes [`fr::W`](W) writer structure"]
impl crate::Writable for FrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FR to value 0"]
impl crate::Resettable for FrSpec {
    const RESET_VALUE: u32 = 0;
}
