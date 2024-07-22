#[doc = "Register `IEC` reader"]
pub type R = crate::R<IecSpec>;
#[doc = "Register `IEC` writer"]
pub type W = crate::W<IecSpec>;
#[doc = "Field `TXCMPMIC` reader - This bit holds the modem TXCMP interrupt clear."]
pub type TxcmpmicR = crate::BitReader;
#[doc = "Field `TXCMPMIC` writer - This bit holds the modem TXCMP interrupt clear."]
pub type TxcmpmicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSMIC` reader - This bit holds the nUARTCTS modem interrupt clear. Clears the UARTCTSINTR interrupt."]
pub type CtsmicR = crate::BitReader;
#[doc = "Field `CTSMIC` writer - This bit holds the nUARTCTS modem interrupt clear. Clears the UARTCTSINTR interrupt."]
pub type CtsmicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDMIC` reader - This bit holds the nUARTDCD modem interrupt clear. Clears the UARTDCDINTR interrupt."]
pub type DcdmicR = crate::BitReader;
#[doc = "Field `DCDMIC` writer - This bit holds the nUARTDCD modem interrupt clear. Clears the UARTDCDINTR interrupt."]
pub type DcdmicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSRMIC` reader - This bit holds the nUARTDSR modem interrupt clear. Clears the UARTDSRINTR interrupt."]
pub type DsrmicR = crate::BitReader;
#[doc = "Field `DSRMIC` writer - This bit holds the nUARTDSR modem interrupt clear. Clears the UARTDSRINTR interrupt."]
pub type DsrmicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIC` reader - This bit holds the receive interrupt clear. Clears the UARTRXINTR interrupt."]
pub type RxicR = crate::BitReader;
#[doc = "Field `RXIC` writer - This bit holds the receive interrupt clear. Clears the UARTRXINTR interrupt."]
pub type RxicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIC` reader - This bit holds the transmit interrupt clear. Clears the UARTTXINTR interrupt."]
pub type TxicR = crate::BitReader;
#[doc = "Field `TXIC` writer - This bit holds the transmit interrupt clear. Clears the UARTTXINTR interrupt."]
pub type TxicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIC` reader - This bit holds the receive timeout interrupt clear. Clears the UARTRTINTR interrupt."]
pub type RticR = crate::BitReader;
#[doc = "Field `RTIC` writer - This bit holds the receive timeout interrupt clear. Clears the UARTRTINTR interrupt."]
pub type RticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEIC` reader - This bit holds the framing error interrupt clear. Clears the UARTFEINTR interrupt."]
pub type FeicR = crate::BitReader;
#[doc = "Field `FEIC` writer - This bit holds the framing error interrupt clear. Clears the UARTFEINTR interrupt."]
pub type FeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIC` reader - This bit holds the parity error interrupt clear. Clears the UARTPEINTR interrupt."]
pub type PeicR = crate::BitReader;
#[doc = "Field `PEIC` writer - This bit holds the parity error interrupt clear. Clears the UARTPEINTR interrupt."]
pub type PeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEIC` reader - This bit holds the break error interrupt clear. Clears the UARTBEINTR interrupt."]
pub type BeicR = crate::BitReader;
#[doc = "Field `BEIC` writer - This bit holds the break error interrupt clear. Clears the UARTBEINTR interrupt."]
pub type BeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEIC` reader - This bit holds the overrun interrupt clear. Clears the UARTOEINTR interrupt."]
pub type OeicR = crate::BitReader;
#[doc = "Field `OEIC` writer - This bit holds the overrun interrupt clear. Clears the UARTOEINTR interrupt."]
pub type OeicW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit holds the modem TXCMP interrupt clear."]
    #[inline(always)]
    pub fn txcmpmic(&self) -> TxcmpmicR {
        TxcmpmicR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit holds the nUARTCTS modem interrupt clear. Clears the UARTCTSINTR interrupt."]
    #[inline(always)]
    pub fn ctsmic(&self) -> CtsmicR {
        CtsmicR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit holds the nUARTDCD modem interrupt clear. Clears the UARTDCDINTR interrupt."]
    #[inline(always)]
    pub fn dcdmic(&self) -> DcdmicR {
        DcdmicR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit holds the nUARTDSR modem interrupt clear. Clears the UARTDSRINTR interrupt."]
    #[inline(always)]
    pub fn dsrmic(&self) -> DsrmicR {
        DsrmicR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit holds the receive interrupt clear. Clears the UARTRXINTR interrupt."]
    #[inline(always)]
    pub fn rxic(&self) -> RxicR {
        RxicR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit holds the transmit interrupt clear. Clears the UARTTXINTR interrupt."]
    #[inline(always)]
    pub fn txic(&self) -> TxicR {
        TxicR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit holds the receive timeout interrupt clear. Clears the UARTRTINTR interrupt."]
    #[inline(always)]
    pub fn rtic(&self) -> RticR {
        RticR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit holds the framing error interrupt clear. Clears the UARTFEINTR interrupt."]
    #[inline(always)]
    pub fn feic(&self) -> FeicR {
        FeicR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit holds the parity error interrupt clear. Clears the UARTPEINTR interrupt."]
    #[inline(always)]
    pub fn peic(&self) -> PeicR {
        PeicR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit holds the break error interrupt clear. Clears the UARTBEINTR interrupt."]
    #[inline(always)]
    pub fn beic(&self) -> BeicR {
        BeicR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit holds the overrun interrupt clear. Clears the UARTOEINTR interrupt."]
    #[inline(always)]
    pub fn oeic(&self) -> OeicR {
        OeicR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit holds the modem TXCMP interrupt clear."]
    #[inline(always)]
    #[must_use]
    pub fn txcmpmic(&mut self) -> TxcmpmicW<IecSpec> {
        TxcmpmicW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit holds the nUARTCTS modem interrupt clear. Clears the UARTCTSINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ctsmic(&mut self) -> CtsmicW<IecSpec> {
        CtsmicW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit holds the nUARTDCD modem interrupt clear. Clears the UARTDCDINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dcdmic(&mut self) -> DcdmicW<IecSpec> {
        DcdmicW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit holds the nUARTDSR modem interrupt clear. Clears the UARTDSRINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dsrmic(&mut self) -> DsrmicW<IecSpec> {
        DsrmicW::new(self, 3)
    }
    #[doc = "Bit 4 - This bit holds the receive interrupt clear. Clears the UARTRXINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxic(&mut self) -> RxicW<IecSpec> {
        RxicW::new(self, 4)
    }
    #[doc = "Bit 5 - This bit holds the transmit interrupt clear. Clears the UARTTXINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn txic(&mut self) -> TxicW<IecSpec> {
        TxicW::new(self, 5)
    }
    #[doc = "Bit 6 - This bit holds the receive timeout interrupt clear. Clears the UARTRTINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rtic(&mut self) -> RticW<IecSpec> {
        RticW::new(self, 6)
    }
    #[doc = "Bit 7 - This bit holds the framing error interrupt clear. Clears the UARTFEINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn feic(&mut self) -> FeicW<IecSpec> {
        FeicW::new(self, 7)
    }
    #[doc = "Bit 8 - This bit holds the parity error interrupt clear. Clears the UARTPEINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn peic(&mut self) -> PeicW<IecSpec> {
        PeicW::new(self, 8)
    }
    #[doc = "Bit 9 - This bit holds the break error interrupt clear. Clears the UARTBEINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn beic(&mut self) -> BeicW<IecSpec> {
        BeicW::new(self, 9)
    }
    #[doc = "Bit 10 - This bit holds the overrun interrupt clear. Clears the UARTOEINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn oeic(&mut self) -> OeicW<IecSpec> {
        OeicW::new(self, 10)
    }
}
#[doc = "Interrupt Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`iec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IecSpec;
impl crate::RegisterSpec for IecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iec::R`](R) reader structure"]
impl crate::Readable for IecSpec {}
#[doc = "`write(|w| ..)` method takes [`iec::W`](W) writer structure"]
impl crate::Writable for IecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEC to value 0"]
impl crate::Resettable for IecSpec {
    const RESET_VALUE: u32 = 0;
}
