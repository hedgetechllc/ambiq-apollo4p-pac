#[doc = "Register `IES` reader"]
pub type R = crate::R<IesSpec>;
#[doc = "Register `IES` writer"]
pub type W = crate::W<IesSpec>;
#[doc = "Field `TXCMPMRIS` reader - This bit holds the modem TXCMP interrupt status."]
pub type TxcmpmrisR = crate::BitReader;
#[doc = "Field `TXCMPMRIS` writer - This bit holds the modem TXCMP interrupt status."]
pub type TxcmpmrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSMRIS` reader - This bit holds the nUARTCTS modem interrupt status. Returns the raw interrupt state of the UARTCTSINTR interrupt."]
pub type CtsmrisR = crate::BitReader;
#[doc = "Field `CTSMRIS` writer - This bit holds the nUARTCTS modem interrupt status. Returns the raw interrupt state of the UARTCTSINTR interrupt."]
pub type CtsmrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDMRIS` reader - This bit holds the nUARTDCD modem interrupt status. Returns the raw interrupt state of the UARTDCDINTR interrupt."]
pub type DcdmrisR = crate::BitReader;
#[doc = "Field `DCDMRIS` writer - This bit holds the nUARTDCD modem interrupt status. Returns the raw interrupt state of the UARTDCDINTR interrupt."]
pub type DcdmrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSRMRIS` reader - This bit holds the nUARTDSR modem interrupt status. Returns the raw interrupt state of the UARTDSRINTR interrupt."]
pub type DsrmrisR = crate::BitReader;
#[doc = "Field `DSRMRIS` writer - This bit holds the nUARTDSR modem interrupt status. Returns the raw interrupt state of the UARTDSRINTR interrupt."]
pub type DsrmrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRIS` reader - This bit holds the receive interrupt status. Returns the raw interrupt state of the UARTRXINTR interrupt."]
pub type RxrisR = crate::BitReader;
#[doc = "Field `RXRIS` writer - This bit holds the receive interrupt status. Returns the raw interrupt state of the UARTRXINTR interrupt."]
pub type RxrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRIS` reader - This bit holds the transmit interrupt status. Returns the raw interrupt state of the UARTTXINTR interrupt."]
pub type TxrisR = crate::BitReader;
#[doc = "Field `TXRIS` writer - This bit holds the transmit interrupt status. Returns the raw interrupt state of the UARTTXINTR interrupt."]
pub type TxrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTRIS` reader - This bit holds the receive timeout interrupt status. Returns the raw interrupt state of the UARTRTINTR interrupt."]
pub type RtrisR = crate::BitReader;
#[doc = "Field `RTRIS` writer - This bit holds the receive timeout interrupt status. Returns the raw interrupt state of the UARTRTINTR interrupt."]
pub type RtrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERIS` reader - This bit holds the framing error interrupt status. Returns the raw interrupt state of the UARTFEINTR interrupt."]
pub type FerisR = crate::BitReader;
#[doc = "Field `FERIS` writer - This bit holds the framing error interrupt status. Returns the raw interrupt state of the UARTFEINTR interrupt."]
pub type FerisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERIS` reader - This bit holds the parity error interrupt status. Returns the raw interrupt state of the UARTPEINTR interrupt."]
pub type PerisR = crate::BitReader;
#[doc = "Field `PERIS` writer - This bit holds the parity error interrupt status. Returns the raw interrupt state of the UARTPEINTR interrupt."]
pub type PerisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERIS` reader - This bit holds the break error interrupt status. Returns the raw interrupt state of the UARTBEINTR interrupt."]
pub type BerisR = crate::BitReader;
#[doc = "Field `BERIS` writer - This bit holds the break error interrupt status. Returns the raw interrupt state of the UARTBEINTR interrupt."]
pub type BerisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OERIS` reader - This bit holds the overrun interrupt status. Returns the raw interrupt state of the UARTOEINTR interrupt."]
pub type OerisR = crate::BitReader;
#[doc = "Field `OERIS` writer - This bit holds the overrun interrupt status. Returns the raw interrupt state of the UARTOEINTR interrupt."]
pub type OerisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit holds the modem TXCMP interrupt status."]
    #[inline(always)]
    pub fn txcmpmris(&self) -> TxcmpmrisR {
        TxcmpmrisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit holds the nUARTCTS modem interrupt status. Returns the raw interrupt state of the UARTCTSINTR interrupt."]
    #[inline(always)]
    pub fn ctsmris(&self) -> CtsmrisR {
        CtsmrisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit holds the nUARTDCD modem interrupt status. Returns the raw interrupt state of the UARTDCDINTR interrupt."]
    #[inline(always)]
    pub fn dcdmris(&self) -> DcdmrisR {
        DcdmrisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit holds the nUARTDSR modem interrupt status. Returns the raw interrupt state of the UARTDSRINTR interrupt."]
    #[inline(always)]
    pub fn dsrmris(&self) -> DsrmrisR {
        DsrmrisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit holds the receive interrupt status. Returns the raw interrupt state of the UARTRXINTR interrupt."]
    #[inline(always)]
    pub fn rxris(&self) -> RxrisR {
        RxrisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit holds the transmit interrupt status. Returns the raw interrupt state of the UARTTXINTR interrupt."]
    #[inline(always)]
    pub fn txris(&self) -> TxrisR {
        TxrisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit holds the receive timeout interrupt status. Returns the raw interrupt state of the UARTRTINTR interrupt."]
    #[inline(always)]
    pub fn rtris(&self) -> RtrisR {
        RtrisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit holds the framing error interrupt status. Returns the raw interrupt state of the UARTFEINTR interrupt."]
    #[inline(always)]
    pub fn feris(&self) -> FerisR {
        FerisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit holds the parity error interrupt status. Returns the raw interrupt state of the UARTPEINTR interrupt."]
    #[inline(always)]
    pub fn peris(&self) -> PerisR {
        PerisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit holds the break error interrupt status. Returns the raw interrupt state of the UARTBEINTR interrupt."]
    #[inline(always)]
    pub fn beris(&self) -> BerisR {
        BerisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit holds the overrun interrupt status. Returns the raw interrupt state of the UARTOEINTR interrupt."]
    #[inline(always)]
    pub fn oeris(&self) -> OerisR {
        OerisR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit holds the modem TXCMP interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn txcmpmris(&mut self) -> TxcmpmrisW<IesSpec> {
        TxcmpmrisW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit holds the nUARTCTS modem interrupt status. Returns the raw interrupt state of the UARTCTSINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ctsmris(&mut self) -> CtsmrisW<IesSpec> {
        CtsmrisW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit holds the nUARTDCD modem interrupt status. Returns the raw interrupt state of the UARTDCDINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dcdmris(&mut self) -> DcdmrisW<IesSpec> {
        DcdmrisW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit holds the nUARTDSR modem interrupt status. Returns the raw interrupt state of the UARTDSRINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dsrmris(&mut self) -> DsrmrisW<IesSpec> {
        DsrmrisW::new(self, 3)
    }
    #[doc = "Bit 4 - This bit holds the receive interrupt status. Returns the raw interrupt state of the UARTRXINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxris(&mut self) -> RxrisW<IesSpec> {
        RxrisW::new(self, 4)
    }
    #[doc = "Bit 5 - This bit holds the transmit interrupt status. Returns the raw interrupt state of the UARTTXINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn txris(&mut self) -> TxrisW<IesSpec> {
        TxrisW::new(self, 5)
    }
    #[doc = "Bit 6 - This bit holds the receive timeout interrupt status. Returns the raw interrupt state of the UARTRTINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rtris(&mut self) -> RtrisW<IesSpec> {
        RtrisW::new(self, 6)
    }
    #[doc = "Bit 7 - This bit holds the framing error interrupt status. Returns the raw interrupt state of the UARTFEINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn feris(&mut self) -> FerisW<IesSpec> {
        FerisW::new(self, 7)
    }
    #[doc = "Bit 8 - This bit holds the parity error interrupt status. Returns the raw interrupt state of the UARTPEINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn peris(&mut self) -> PerisW<IesSpec> {
        PerisW::new(self, 8)
    }
    #[doc = "Bit 9 - This bit holds the break error interrupt status. Returns the raw interrupt state of the UARTBEINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn beris(&mut self) -> BerisW<IesSpec> {
        BerisW::new(self, 9)
    }
    #[doc = "Bit 10 - This bit holds the overrun interrupt status. Returns the raw interrupt state of the UARTOEINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn oeris(&mut self) -> OerisW<IesSpec> {
        OerisW::new(self, 10)
    }
}
#[doc = "Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`ies::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ies::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IesSpec;
impl crate::RegisterSpec for IesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ies::R`](R) reader structure"]
impl crate::Readable for IesSpec {}
#[doc = "`write(|w| ..)` method takes [`ies::W`](W) writer structure"]
impl crate::Writable for IesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IES to value 0"]
impl crate::Resettable for IesSpec {
    const RESET_VALUE: u32 = 0;
}
