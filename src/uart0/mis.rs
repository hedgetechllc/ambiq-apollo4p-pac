#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Register `MIS` writer"]
pub type W = crate::W<MisSpec>;
#[doc = "Field `TXCMPMMIS` reader - This bit holds the modem TXCMP interrupt status masked."]
pub type TxcmpmmisR = crate::BitReader;
#[doc = "Field `TXCMPMMIS` writer - This bit holds the modem TXCMP interrupt status masked."]
pub type TxcmpmmisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSMMIS` reader - This bit holds the nUARTCTS modem masked interrupt status. Returns the masked interrupt state of the UARTCTSINTR interrupt."]
pub type CtsmmisR = crate::BitReader;
#[doc = "Field `CTSMMIS` writer - This bit holds the nUARTCTS modem masked interrupt status. Returns the masked interrupt state of the UARTCTSINTR interrupt."]
pub type CtsmmisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDMMIS` reader - This bit holds the nUARTDCD modem masked interrupt status. Returns the masked interrupt state of the UARTDCDINTR interrupt."]
pub type DcdmmisR = crate::BitReader;
#[doc = "Field `DCDMMIS` writer - This bit holds the nUARTDCD modem masked interrupt status. Returns the masked interrupt state of the UARTDCDINTR interrupt."]
pub type DcdmmisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSRMMIS` reader - This bit holds the nUARTDSR modem masked interrupt status. Returns the masked interrupt state of the UARTDSRINTR interrupt."]
pub type DsrmmisR = crate::BitReader;
#[doc = "Field `DSRMMIS` writer - This bit holds the nUARTDSR modem masked interrupt status. Returns the masked interrupt state of the UARTDSRINTR interrupt."]
pub type DsrmmisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXMIS` reader - This bit holds the receive interrupt status masked. Returns the masked interrupt state of the UARTRXINTR interrupt."]
pub type RxmisR = crate::BitReader;
#[doc = "Field `RXMIS` writer - This bit holds the receive interrupt status masked. Returns the masked interrupt state of the UARTRXINTR interrupt."]
pub type RxmisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMIS` reader - This bit holds the transmit interrupt status masked. Returns the masked interrupt state of the UARTTXINTR interrupt."]
pub type TxmisR = crate::BitReader;
#[doc = "Field `TXMIS` writer - This bit holds the transmit interrupt status masked. Returns the masked interrupt state of the UARTTXINTR interrupt."]
pub type TxmisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTMIS` reader - This bit holds the receive timeout interrupt status masked. Returns the masked interrupt state of the UARTRTINTR interrupt."]
pub type RtmisR = crate::BitReader;
#[doc = "Field `RTMIS` writer - This bit holds the receive timeout interrupt status masked. Returns the masked interrupt state of the UARTRTINTR interrupt."]
pub type RtmisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEMIS` reader - This bit holds the framing error interrupt status masked. Returns the masked interrupt state of the UARTFEINTR interrupt."]
pub type FemisR = crate::BitReader;
#[doc = "Field `FEMIS` writer - This bit holds the framing error interrupt status masked. Returns the masked interrupt state of the UARTFEINTR interrupt."]
pub type FemisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEMIS` reader - This bit holds the parity error interrupt status masked. Returns the masked interrupt state of the UARTPEINTR interrupt."]
pub type PemisR = crate::BitReader;
#[doc = "Field `PEMIS` writer - This bit holds the parity error interrupt status masked. Returns the masked interrupt state of the UARTPEINTR interrupt."]
pub type PemisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEMIS` reader - This bit holds the break error interrupt status masked. Returns the masked interrupt state of the UARTBEINTR interrupt."]
pub type BemisR = crate::BitReader;
#[doc = "Field `BEMIS` writer - This bit holds the break error interrupt status masked. Returns the masked interrupt state of the UARTBEINTR interrupt."]
pub type BemisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEMIS` reader - This bit holds the overrun interrupt status masked. Returns the masked interrupt state of the UARTOEINTR interrupt."]
pub type OemisR = crate::BitReader;
#[doc = "Field `OEMIS` writer - This bit holds the overrun interrupt status masked. Returns the masked interrupt state of the UARTOEINTR interrupt."]
pub type OemisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit holds the modem TXCMP interrupt status masked."]
    #[inline(always)]
    pub fn txcmpmmis(&self) -> TxcmpmmisR {
        TxcmpmmisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit holds the nUARTCTS modem masked interrupt status. Returns the masked interrupt state of the UARTCTSINTR interrupt."]
    #[inline(always)]
    pub fn ctsmmis(&self) -> CtsmmisR {
        CtsmmisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit holds the nUARTDCD modem masked interrupt status. Returns the masked interrupt state of the UARTDCDINTR interrupt."]
    #[inline(always)]
    pub fn dcdmmis(&self) -> DcdmmisR {
        DcdmmisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit holds the nUARTDSR modem masked interrupt status. Returns the masked interrupt state of the UARTDSRINTR interrupt."]
    #[inline(always)]
    pub fn dsrmmis(&self) -> DsrmmisR {
        DsrmmisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit holds the receive interrupt status masked. Returns the masked interrupt state of the UARTRXINTR interrupt."]
    #[inline(always)]
    pub fn rxmis(&self) -> RxmisR {
        RxmisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit holds the transmit interrupt status masked. Returns the masked interrupt state of the UARTTXINTR interrupt."]
    #[inline(always)]
    pub fn txmis(&self) -> TxmisR {
        TxmisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit holds the receive timeout interrupt status masked. Returns the masked interrupt state of the UARTRTINTR interrupt."]
    #[inline(always)]
    pub fn rtmis(&self) -> RtmisR {
        RtmisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit holds the framing error interrupt status masked. Returns the masked interrupt state of the UARTFEINTR interrupt."]
    #[inline(always)]
    pub fn femis(&self) -> FemisR {
        FemisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit holds the parity error interrupt status masked. Returns the masked interrupt state of the UARTPEINTR interrupt."]
    #[inline(always)]
    pub fn pemis(&self) -> PemisR {
        PemisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit holds the break error interrupt status masked. Returns the masked interrupt state of the UARTBEINTR interrupt."]
    #[inline(always)]
    pub fn bemis(&self) -> BemisR {
        BemisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit holds the overrun interrupt status masked. Returns the masked interrupt state of the UARTOEINTR interrupt."]
    #[inline(always)]
    pub fn oemis(&self) -> OemisR {
        OemisR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit holds the modem TXCMP interrupt status masked."]
    #[inline(always)]
    #[must_use]
    pub fn txcmpmmis(&mut self) -> TxcmpmmisW<MisSpec> {
        TxcmpmmisW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit holds the nUARTCTS modem masked interrupt status. Returns the masked interrupt state of the UARTCTSINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ctsmmis(&mut self) -> CtsmmisW<MisSpec> {
        CtsmmisW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit holds the nUARTDCD modem masked interrupt status. Returns the masked interrupt state of the UARTDCDINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dcdmmis(&mut self) -> DcdmmisW<MisSpec> {
        DcdmmisW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit holds the nUARTDSR modem masked interrupt status. Returns the masked interrupt state of the UARTDSRINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dsrmmis(&mut self) -> DsrmmisW<MisSpec> {
        DsrmmisW::new(self, 3)
    }
    #[doc = "Bit 4 - This bit holds the receive interrupt status masked. Returns the masked interrupt state of the UARTRXINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxmis(&mut self) -> RxmisW<MisSpec> {
        RxmisW::new(self, 4)
    }
    #[doc = "Bit 5 - This bit holds the transmit interrupt status masked. Returns the masked interrupt state of the UARTTXINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn txmis(&mut self) -> TxmisW<MisSpec> {
        TxmisW::new(self, 5)
    }
    #[doc = "Bit 6 - This bit holds the receive timeout interrupt status masked. Returns the masked interrupt state of the UARTRTINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rtmis(&mut self) -> RtmisW<MisSpec> {
        RtmisW::new(self, 6)
    }
    #[doc = "Bit 7 - This bit holds the framing error interrupt status masked. Returns the masked interrupt state of the UARTFEINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn femis(&mut self) -> FemisW<MisSpec> {
        FemisW::new(self, 7)
    }
    #[doc = "Bit 8 - This bit holds the parity error interrupt status masked. Returns the masked interrupt state of the UARTPEINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pemis(&mut self) -> PemisW<MisSpec> {
        PemisW::new(self, 8)
    }
    #[doc = "Bit 9 - This bit holds the break error interrupt status masked. Returns the masked interrupt state of the UARTBEINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn bemis(&mut self) -> BemisW<MisSpec> {
        BemisW::new(self, 9)
    }
    #[doc = "Bit 10 - This bit holds the overrun interrupt status masked. Returns the masked interrupt state of the UARTOEINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn oemis(&mut self) -> OemisW<MisSpec> {
        OemisW::new(self, 10)
    }
}
#[doc = "Masked Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`write(|w| ..)` method takes [`mis::W`](W) writer structure"]
impl crate::Writable for MisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {
    const RESET_VALUE: u32 = 0;
}
