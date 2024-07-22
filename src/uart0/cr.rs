#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `UARTEN` reader - This bit is the UART enable. 0 = UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping. 1 = the UART is enabled. Data transmission and reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit."]
pub type UartenR = crate::BitReader;
#[doc = "Field `UARTEN` writer - This bit is the UART enable. 0 = UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping. 1 = the UART is enabled. Data transmission and reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit."]
pub type UartenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIREN` reader - This bit is the SIR ENDEC enable. If this bit is set to 1, the IrDA SIR ENDEC is enabled. This bit has no effect if the UART is not enabled by bit 0 being set to 1. When the IrDA SIR ENDEC is enabled, data is transmitted and received on nSIROUT and SIRIN. UARTTXD remains in the marking state (set to 1). Signal transitions on UARTRXD or modem status inputs have no effect. When the IrDA SIR ENDEC is disabled, nSIROUT remains cleared to 0 (no light pulse generated), and signal transitions on SIRIN have no effect."]
pub type SirenR = crate::BitReader;
#[doc = "Field `SIREN` writer - This bit is the SIR ENDEC enable. If this bit is set to 1, the IrDA SIR ENDEC is enabled. This bit has no effect if the UART is not enabled by bit 0 being set to 1. When the IrDA SIR ENDEC is enabled, data is transmitted and received on nSIROUT and SIRIN. UARTTXD remains in the marking state (set to 1). Signal transitions on UARTRXD or modem status inputs have no effect. When the IrDA SIR ENDEC is disabled, nSIROUT remains cleared to 0 (no light pulse generated), and signal transitions on SIRIN have no effect."]
pub type SirenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIRLP` reader - This bit is the SIR low power select. This bit selects the IrDA encoding mode. If this bit is cleared to 0, low-level bits are transmitted as an active high pulse with a width of 3/16th of the bit period. If this bit is set to 1, low-level bits are transmitted with a pulse width which is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. Setting this bit uses less power, but might reduce transmission distances."]
pub type SirlpR = crate::BitReader;
#[doc = "Field `SIRLP` writer - This bit is the SIR low power select. This bit selects the IrDA encoding mode. If this bit is cleared to 0, low-level bits are transmitted as an active high pulse with a width of 3/16th of the bit period. If this bit is set to 1, low-level bits are transmitted with a pulse width which is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. Setting this bit uses less power, but might reduce transmission distances."]
pub type SirlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKEN` reader - This bit is the UART clock enable."]
pub type ClkenR = crate::BitReader;
#[doc = "Field `CLKEN` writer - This bit is the UART clock enable."]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "This bitfield is the UART clock select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "0: No UART clock. This is the low power default."]
    Noclk = 0,
    #[doc = "1: 24 MHz clock."]
    _24mhz = 1,
    #[doc = "2: 12 MHz clock."]
    _12mhz = 2,
    #[doc = "3: 6 MHz clock."]
    _6mhz = 3,
    #[doc = "4: 3 MHz clock."]
    _3mhz = 4,
    #[doc = "5: Reserved."]
    _48mhz = 5,
}
impl From<Clksel> for u8 {
    #[inline(always)]
    fn from(variant: Clksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clksel {
    type Ux = u8;
}
impl crate::IsEnum for Clksel {}
#[doc = "Field `CLKSEL` reader - This bitfield is the UART clock select."]
pub type ClkselR = crate::FieldReader<Clksel>;
impl ClkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clksel> {
        match self.bits {
            0 => Some(Clksel::Noclk),
            1 => Some(Clksel::_24mhz),
            2 => Some(Clksel::_12mhz),
            3 => Some(Clksel::_6mhz),
            4 => Some(Clksel::_3mhz),
            5 => Some(Clksel::_48mhz),
            _ => None,
        }
    }
    #[doc = "No UART clock. This is the low power default."]
    #[inline(always)]
    pub fn is_noclk(&self) -> bool {
        *self == Clksel::Noclk
    }
    #[doc = "24 MHz clock."]
    #[inline(always)]
    pub fn is_24mhz(&self) -> bool {
        *self == Clksel::_24mhz
    }
    #[doc = "12 MHz clock."]
    #[inline(always)]
    pub fn is_12mhz(&self) -> bool {
        *self == Clksel::_12mhz
    }
    #[doc = "6 MHz clock."]
    #[inline(always)]
    pub fn is_6mhz(&self) -> bool {
        *self == Clksel::_6mhz
    }
    #[doc = "3 MHz clock."]
    #[inline(always)]
    pub fn is_3mhz(&self) -> bool {
        *self == Clksel::_3mhz
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn is_48mhz(&self) -> bool {
        *self == Clksel::_48mhz
    }
}
#[doc = "Field `CLKSEL` writer - This bitfield is the UART clock select."]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Clksel>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No UART clock. This is the low power default."]
    #[inline(always)]
    pub fn noclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Noclk)
    }
    #[doc = "24 MHz clock."]
    #[inline(always)]
    pub fn _24mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::_24mhz)
    }
    #[doc = "12 MHz clock."]
    #[inline(always)]
    pub fn _12mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::_12mhz)
    }
    #[doc = "6 MHz clock."]
    #[inline(always)]
    pub fn _6mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::_6mhz)
    }
    #[doc = "3 MHz clock."]
    #[inline(always)]
    pub fn _3mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::_3mhz)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _48mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::_48mhz)
    }
}
#[doc = "Field `LBE` reader - This bit is the loopback enable. If this bit is set to 1 and the SIREN bit is set to 1 and the SIRTEST bit in the Test Control Register, UARTTCR is set to 1, then the nSIROUT path is inverted, and fed through to the SIRIN path. The SIRTEST bit in the test register must be set to 1 to override the normal half-duplex SIR operation. This must be the requirement for accessing the test registers during normal operation, and SIRTEST must be cleared to 0 when loopback testing is finished. This feature reduces the amount of external coupling required during system test. If this bit is set to 1, and the SIRTEST bit is set to 0, the UARTTXD path is fed through to the UARTRXD path. In either SIR mode or UART mode, when this bit is set, the modem outputs are also fed through to the modem inputs. This bit is cleared to 0 on reset, to disable loopback."]
pub type LbeR = crate::BitReader;
#[doc = "Field `LBE` writer - This bit is the loopback enable. If this bit is set to 1 and the SIREN bit is set to 1 and the SIRTEST bit in the Test Control Register, UARTTCR is set to 1, then the nSIROUT path is inverted, and fed through to the SIRIN path. The SIRTEST bit in the test register must be set to 1 to override the normal half-duplex SIR operation. This must be the requirement for accessing the test registers during normal operation, and SIRTEST must be cleared to 0 when loopback testing is finished. This feature reduces the amount of external coupling required during system test. If this bit is set to 1, and the SIRTEST bit is set to 0, the UARTTXD path is fed through to the UARTRXD path. In either SIR mode or UART mode, when this bit is set, the modem outputs are also fed through to the modem inputs. This bit is cleared to 0 on reset, to disable loopback."]
pub type LbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXE` reader - This bit is the transmit enable. If this bit is set to 1, the transmit section of the UART is enabled. Data transmission occurs for either UART signals, or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of transmission, it completes the current character before stopping."]
pub type TxeR = crate::BitReader;
#[doc = "Field `TXE` writer - This bit is the transmit enable. If this bit is set to 1, the transmit section of the UART is enabled. Data transmission occurs for either UART signals, or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of transmission, it completes the current character before stopping."]
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXE` reader - This bit is the receive enable. If this bit is set to 1, the receive section of the UART is enabled. Data reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of reception, it completes the current character before stopping."]
pub type RxeR = crate::BitReader;
#[doc = "Field `RXE` writer - This bit is the receive enable. If this bit is set to 1, the receive section of the UART is enabled. Data reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of reception, it completes the current character before stopping."]
pub type RxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTR` reader - This bit enables data transmit ready. This bit is the complement of the UART data transmit ready, nUARTDTR, modem status output. That is, when the bit is programmed to a 1 then nUARTDTR is LOW."]
pub type DtrR = crate::BitReader;
#[doc = "Field `DTR` writer - This bit enables data transmit ready. This bit is the complement of the UART data transmit ready, nUARTDTR, modem status output. That is, when the bit is programmed to a 1 then nUARTDTR is LOW."]
pub type DtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS` reader - This bit enables request to send. This bit is the complement of the UART request to send, nUARTRTS, modem status output. That is, when the bit is programmed to a 1 then nUARTRTS is LOW."]
pub type RtsR = crate::BitReader;
#[doc = "Field `RTS` writer - This bit enables request to send. This bit is the complement of the UART request to send, nUARTRTS, modem status output. That is, when the bit is programmed to a 1 then nUARTRTS is LOW."]
pub type RtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT1` reader - This bit is the complement of the UART Out1 (nUARTOut1) modem status output. That is, when the bit is programmed to a 1 the output is 0. For DTE this can be used as Data Carrier Detect (DCD)."]
pub type Out1R = crate::BitReader;
#[doc = "Field `OUT1` writer - This bit is the complement of the UART Out1 (nUARTOut1) modem status output. That is, when the bit is programmed to a 1 the output is 0. For DTE this can be used as Data Carrier Detect (DCD)."]
pub type Out1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT2` reader - This bit is the complement of the UART Out2 (nUARTOut2) modem status output. That is, when the bit is programmed to a 1, the output is 0. For DTE this can be used as Ring Indicator (RI)."]
pub type Out2R = crate::BitReader;
#[doc = "Field `OUT2` writer - This bit is the complement of the UART Out2 (nUARTOut2) modem status output. That is, when the bit is programmed to a 1, the output is 0. For DTE this can be used as Ring Indicator (RI)."]
pub type Out2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSEN` reader - This bit enables RTS hardware flow control. If this bit is set to 1, RTS hardware flow control is enabled. Data is only requested when there is space in the receive FIFO for it to be received."]
pub type RtsenR = crate::BitReader;
#[doc = "Field `RTSEN` writer - This bit enables RTS hardware flow control. If this bit is set to 1, RTS hardware flow control is enabled. Data is only requested when there is space in the receive FIFO for it to be received."]
pub type RtsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSEN` reader - This bit enables CTS hardware flow control. If this bit is set to 1, CTS hardware flow control is enabled. Data is only transmitted when the nUARTCTS signal is asserted."]
pub type CtsenR = crate::BitReader;
#[doc = "Field `CTSEN` writer - This bit enables CTS hardware flow control. If this bit is set to 1, CTS hardware flow control is enabled. Data is only transmitted when the nUARTCTS signal is asserted."]
pub type CtsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is the UART enable. 0 = UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping. 1 = the UART is enabled. Data transmission and reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit."]
    #[inline(always)]
    pub fn uarten(&self) -> UartenR {
        UartenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is the SIR ENDEC enable. If this bit is set to 1, the IrDA SIR ENDEC is enabled. This bit has no effect if the UART is not enabled by bit 0 being set to 1. When the IrDA SIR ENDEC is enabled, data is transmitted and received on nSIROUT and SIRIN. UARTTXD remains in the marking state (set to 1). Signal transitions on UARTRXD or modem status inputs have no effect. When the IrDA SIR ENDEC is disabled, nSIROUT remains cleared to 0 (no light pulse generated), and signal transitions on SIRIN have no effect."]
    #[inline(always)]
    pub fn siren(&self) -> SirenR {
        SirenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is the SIR low power select. This bit selects the IrDA encoding mode. If this bit is cleared to 0, low-level bits are transmitted as an active high pulse with a width of 3/16th of the bit period. If this bit is set to 1, low-level bits are transmitted with a pulse width which is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. Setting this bit uses less power, but might reduce transmission distances."]
    #[inline(always)]
    pub fn sirlp(&self) -> SirlpR {
        SirlpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is the UART clock enable."]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - This bitfield is the UART clock select."]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - This bit is the loopback enable. If this bit is set to 1 and the SIREN bit is set to 1 and the SIRTEST bit in the Test Control Register, UARTTCR is set to 1, then the nSIROUT path is inverted, and fed through to the SIRIN path. The SIRTEST bit in the test register must be set to 1 to override the normal half-duplex SIR operation. This must be the requirement for accessing the test registers during normal operation, and SIRTEST must be cleared to 0 when loopback testing is finished. This feature reduces the amount of external coupling required during system test. If this bit is set to 1, and the SIRTEST bit is set to 0, the UARTTXD path is fed through to the UARTRXD path. In either SIR mode or UART mode, when this bit is set, the modem outputs are also fed through to the modem inputs. This bit is cleared to 0 on reset, to disable loopback."]
    #[inline(always)]
    pub fn lbe(&self) -> LbeR {
        LbeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit is the transmit enable. If this bit is set to 1, the transmit section of the UART is enabled. Data transmission occurs for either UART signals, or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of transmission, it completes the current character before stopping."]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit is the receive enable. If this bit is set to 1, the receive section of the UART is enabled. Data reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of reception, it completes the current character before stopping."]
    #[inline(always)]
    pub fn rxe(&self) -> RxeR {
        RxeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit enables data transmit ready. This bit is the complement of the UART data transmit ready, nUARTDTR, modem status output. That is, when the bit is programmed to a 1 then nUARTDTR is LOW."]
    #[inline(always)]
    pub fn dtr(&self) -> DtrR {
        DtrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit enables request to send. This bit is the complement of the UART request to send, nUARTRTS, modem status output. That is, when the bit is programmed to a 1 then nUARTRTS is LOW."]
    #[inline(always)]
    pub fn rts(&self) -> RtsR {
        RtsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit is the complement of the UART Out1 (nUARTOut1) modem status output. That is, when the bit is programmed to a 1 the output is 0. For DTE this can be used as Data Carrier Detect (DCD)."]
    #[inline(always)]
    pub fn out1(&self) -> Out1R {
        Out1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit is the complement of the UART Out2 (nUARTOut2) modem status output. That is, when the bit is programmed to a 1, the output is 0. For DTE this can be used as Ring Indicator (RI)."]
    #[inline(always)]
    pub fn out2(&self) -> Out2R {
        Out2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This bit enables RTS hardware flow control. If this bit is set to 1, RTS hardware flow control is enabled. Data is only requested when there is space in the receive FIFO for it to be received."]
    #[inline(always)]
    pub fn rtsen(&self) -> RtsenR {
        RtsenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This bit enables CTS hardware flow control. If this bit is set to 1, CTS hardware flow control is enabled. Data is only transmitted when the nUARTCTS signal is asserted."]
    #[inline(always)]
    pub fn ctsen(&self) -> CtsenR {
        CtsenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is the UART enable. 0 = UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping. 1 = the UART is enabled. Data transmission and reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit."]
    #[inline(always)]
    #[must_use]
    pub fn uarten(&mut self) -> UartenW<CrSpec> {
        UartenW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit is the SIR ENDEC enable. If this bit is set to 1, the IrDA SIR ENDEC is enabled. This bit has no effect if the UART is not enabled by bit 0 being set to 1. When the IrDA SIR ENDEC is enabled, data is transmitted and received on nSIROUT and SIRIN. UARTTXD remains in the marking state (set to 1). Signal transitions on UARTRXD or modem status inputs have no effect. When the IrDA SIR ENDEC is disabled, nSIROUT remains cleared to 0 (no light pulse generated), and signal transitions on SIRIN have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn siren(&mut self) -> SirenW<CrSpec> {
        SirenW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit is the SIR low power select. This bit selects the IrDA encoding mode. If this bit is cleared to 0, low-level bits are transmitted as an active high pulse with a width of 3/16th of the bit period. If this bit is set to 1, low-level bits are transmitted with a pulse width which is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. Setting this bit uses less power, but might reduce transmission distances."]
    #[inline(always)]
    #[must_use]
    pub fn sirlp(&mut self) -> SirlpW<CrSpec> {
        SirlpW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit is the UART clock enable."]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> ClkenW<CrSpec> {
        ClkenW::new(self, 3)
    }
    #[doc = "Bits 4:6 - This bitfield is the UART clock select."]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> ClkselW<CrSpec> {
        ClkselW::new(self, 4)
    }
    #[doc = "Bit 7 - This bit is the loopback enable. If this bit is set to 1 and the SIREN bit is set to 1 and the SIRTEST bit in the Test Control Register, UARTTCR is set to 1, then the nSIROUT path is inverted, and fed through to the SIRIN path. The SIRTEST bit in the test register must be set to 1 to override the normal half-duplex SIR operation. This must be the requirement for accessing the test registers during normal operation, and SIRTEST must be cleared to 0 when loopback testing is finished. This feature reduces the amount of external coupling required during system test. If this bit is set to 1, and the SIRTEST bit is set to 0, the UARTTXD path is fed through to the UARTRXD path. In either SIR mode or UART mode, when this bit is set, the modem outputs are also fed through to the modem inputs. This bit is cleared to 0 on reset, to disable loopback."]
    #[inline(always)]
    #[must_use]
    pub fn lbe(&mut self) -> LbeW<CrSpec> {
        LbeW::new(self, 7)
    }
    #[doc = "Bit 8 - This bit is the transmit enable. If this bit is set to 1, the transmit section of the UART is enabled. Data transmission occurs for either UART signals, or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of transmission, it completes the current character before stopping."]
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TxeW<CrSpec> {
        TxeW::new(self, 8)
    }
    #[doc = "Bit 9 - This bit is the receive enable. If this bit is set to 1, the receive section of the UART is enabled. Data reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of reception, it completes the current character before stopping."]
    #[inline(always)]
    #[must_use]
    pub fn rxe(&mut self) -> RxeW<CrSpec> {
        RxeW::new(self, 9)
    }
    #[doc = "Bit 10 - This bit enables data transmit ready. This bit is the complement of the UART data transmit ready, nUARTDTR, modem status output. That is, when the bit is programmed to a 1 then nUARTDTR is LOW."]
    #[inline(always)]
    #[must_use]
    pub fn dtr(&mut self) -> DtrW<CrSpec> {
        DtrW::new(self, 10)
    }
    #[doc = "Bit 11 - This bit enables request to send. This bit is the complement of the UART request to send, nUARTRTS, modem status output. That is, when the bit is programmed to a 1 then nUARTRTS is LOW."]
    #[inline(always)]
    #[must_use]
    pub fn rts(&mut self) -> RtsW<CrSpec> {
        RtsW::new(self, 11)
    }
    #[doc = "Bit 12 - This bit is the complement of the UART Out1 (nUARTOut1) modem status output. That is, when the bit is programmed to a 1 the output is 0. For DTE this can be used as Data Carrier Detect (DCD)."]
    #[inline(always)]
    #[must_use]
    pub fn out1(&mut self) -> Out1W<CrSpec> {
        Out1W::new(self, 12)
    }
    #[doc = "Bit 13 - This bit is the complement of the UART Out2 (nUARTOut2) modem status output. That is, when the bit is programmed to a 1, the output is 0. For DTE this can be used as Ring Indicator (RI)."]
    #[inline(always)]
    #[must_use]
    pub fn out2(&mut self) -> Out2W<CrSpec> {
        Out2W::new(self, 13)
    }
    #[doc = "Bit 14 - This bit enables RTS hardware flow control. If this bit is set to 1, RTS hardware flow control is enabled. Data is only requested when there is space in the receive FIFO for it to be received."]
    #[inline(always)]
    #[must_use]
    pub fn rtsen(&mut self) -> RtsenW<CrSpec> {
        RtsenW::new(self, 14)
    }
    #[doc = "Bit 15 - This bit enables CTS hardware flow control. If this bit is set to 1, CTS hardware flow control is enabled. Data is only transmitted when the nUARTCTS signal is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn ctsen(&mut self) -> CtsenW<CrSpec> {
        CtsenW::new(self, 15)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0x0300"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x0300;
}
