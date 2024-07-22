#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `TXCMPMIM` reader - This bit holds the modem TXCMP interrupt enable."]
pub type TxcmpmimR = crate::BitReader;
#[doc = "Field `TXCMPMIM` writer - This bit holds the modem TXCMP interrupt enable."]
pub type TxcmpmimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSMIM` reader - This bit holds the modem CTS interrupt enable."]
pub type CtsmimR = crate::BitReader;
#[doc = "Field `CTSMIM` writer - This bit holds the modem CTS interrupt enable."]
pub type CtsmimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDMIM` reader - This bit holds the modem DCD interrupt enable."]
pub type DcdmimR = crate::BitReader;
#[doc = "Field `DCDMIM` writer - This bit holds the modem DCD interrupt enable."]
pub type DcdmimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSRMIM` reader - This bit holds the modem DSR interrupt enable."]
pub type DsrmimR = crate::BitReader;
#[doc = "Field `DSRMIM` writer - This bit holds the modem DSR interrupt enable."]
pub type DsrmimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIM` reader - This bit holds the receive interrupt enable."]
pub type RximR = crate::BitReader;
#[doc = "Field `RXIM` writer - This bit holds the receive interrupt enable."]
pub type RximW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIM` reader - This bit holds the transmit interrupt enable."]
pub type TximR = crate::BitReader;
#[doc = "Field `TXIM` writer - This bit holds the transmit interrupt enable."]
pub type TximW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIM` reader - This bit holds the receive timeout interrupt enable."]
pub type RtimR = crate::BitReader;
#[doc = "Field `RTIM` writer - This bit holds the receive timeout interrupt enable."]
pub type RtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEIM` reader - This bit holds the framing error interrupt enable."]
pub type FeimR = crate::BitReader;
#[doc = "Field `FEIM` writer - This bit holds the framing error interrupt enable."]
pub type FeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIM` reader - This bit holds the parity error interrupt enable."]
pub type PeimR = crate::BitReader;
#[doc = "Field `PEIM` writer - This bit holds the parity error interrupt enable."]
pub type PeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEIM` reader - This bit holds the break error interrupt enable."]
pub type BeimR = crate::BitReader;
#[doc = "Field `BEIM` writer - This bit holds the break error interrupt enable."]
pub type BeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEIM` reader - This bit holds the overflow interrupt enable."]
pub type OeimR = crate::BitReader;
#[doc = "Field `OEIM` writer - This bit holds the overflow interrupt enable."]
pub type OeimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit holds the modem TXCMP interrupt enable."]
    #[inline(always)]
    pub fn txcmpmim(&self) -> TxcmpmimR {
        TxcmpmimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit holds the modem CTS interrupt enable."]
    #[inline(always)]
    pub fn ctsmim(&self) -> CtsmimR {
        CtsmimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit holds the modem DCD interrupt enable."]
    #[inline(always)]
    pub fn dcdmim(&self) -> DcdmimR {
        DcdmimR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit holds the modem DSR interrupt enable."]
    #[inline(always)]
    pub fn dsrmim(&self) -> DsrmimR {
        DsrmimR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit holds the receive interrupt enable."]
    #[inline(always)]
    pub fn rxim(&self) -> RximR {
        RximR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit holds the transmit interrupt enable."]
    #[inline(always)]
    pub fn txim(&self) -> TximR {
        TximR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit holds the receive timeout interrupt enable."]
    #[inline(always)]
    pub fn rtim(&self) -> RtimR {
        RtimR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit holds the framing error interrupt enable."]
    #[inline(always)]
    pub fn feim(&self) -> FeimR {
        FeimR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit holds the parity error interrupt enable."]
    #[inline(always)]
    pub fn peim(&self) -> PeimR {
        PeimR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit holds the break error interrupt enable."]
    #[inline(always)]
    pub fn beim(&self) -> BeimR {
        BeimR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit holds the overflow interrupt enable."]
    #[inline(always)]
    pub fn oeim(&self) -> OeimR {
        OeimR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit holds the modem TXCMP interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn txcmpmim(&mut self) -> TxcmpmimW<IerSpec> {
        TxcmpmimW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit holds the modem CTS interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn ctsmim(&mut self) -> CtsmimW<IerSpec> {
        CtsmimW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit holds the modem DCD interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn dcdmim(&mut self) -> DcdmimW<IerSpec> {
        DcdmimW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit holds the modem DSR interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn dsrmim(&mut self) -> DsrmimW<IerSpec> {
        DsrmimW::new(self, 3)
    }
    #[doc = "Bit 4 - This bit holds the receive interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn rxim(&mut self) -> RximW<IerSpec> {
        RximW::new(self, 4)
    }
    #[doc = "Bit 5 - This bit holds the transmit interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn txim(&mut self) -> TximW<IerSpec> {
        TximW::new(self, 5)
    }
    #[doc = "Bit 6 - This bit holds the receive timeout interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn rtim(&mut self) -> RtimW<IerSpec> {
        RtimW::new(self, 6)
    }
    #[doc = "Bit 7 - This bit holds the framing error interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn feim(&mut self) -> FeimW<IerSpec> {
        FeimW::new(self, 7)
    }
    #[doc = "Bit 8 - This bit holds the parity error interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn peim(&mut self) -> PeimW<IerSpec> {
        PeimW::new(self, 8)
    }
    #[doc = "Bit 9 - This bit holds the break error interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn beim(&mut self) -> BeimW<IerSpec> {
        BeimW::new(self, 9)
    }
    #[doc = "Bit 10 - This bit holds the overflow interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn oeim(&mut self) -> OeimW<IerSpec> {
        OeimW::new(self, 10)
    }
}
#[doc = "Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {
    const RESET_VALUE: u32 = 0;
}
