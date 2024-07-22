#[doc = "Register `OCTRL` reader"]
pub type R = crate::R<OctrlSpec>;
#[doc = "Register `OCTRL` writer"]
pub type W = crate::W<OctrlSpec>;
#[doc = "Selects the RTC oscillator (1=LFRC, 0=XT)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Osel {
    #[doc = "0: RTC uses the XT"]
    RtcXt = 0,
    #[doc = "1: RTC uses the LFRC"]
    RtcLfrc = 1,
}
impl From<Osel> for bool {
    #[inline(always)]
    fn from(variant: Osel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSEL` reader - Selects the RTC oscillator (1=LFRC, 0=XT)"]
pub type OselR = crate::BitReader<Osel>;
impl OselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Osel {
        match self.bits {
            false => Osel::RtcXt,
            true => Osel::RtcLfrc,
        }
    }
    #[doc = "RTC uses the XT"]
    #[inline(always)]
    pub fn is_rtc_xt(&self) -> bool {
        *self == Osel::RtcXt
    }
    #[doc = "RTC uses the LFRC"]
    #[inline(always)]
    pub fn is_rtc_lfrc(&self) -> bool {
        *self == Osel::RtcLfrc
    }
}
#[doc = "Field `OSEL` writer - Selects the RTC oscillator (1=LFRC, 0=XT)"]
pub type OselW<'a, REG> = crate::BitWriter<'a, REG, Osel>;
impl<'a, REG> OselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC uses the XT"]
    #[inline(always)]
    pub fn rtc_xt(self) -> &'a mut crate::W<REG> {
        self.variant(Osel::RtcXt)
    }
    #[doc = "RTC uses the LFRC"]
    #[inline(always)]
    pub fn rtc_lfrc(self) -> &'a mut crate::W<REG> {
        self.variant(Osel::RtcLfrc)
    }
}
impl R {
    #[doc = "Bit 7 - Selects the RTC oscillator (1=LFRC, 0=XT)"]
    #[inline(always)]
    pub fn osel(&self) -> OselR {
        OselR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Selects the RTC oscillator (1=LFRC, 0=XT)"]
    #[inline(always)]
    #[must_use]
    pub fn osel(&mut self) -> OselW<OctrlSpec> {
        OselW::new(self, 7)
    }
}
#[doc = "This register includes controls for autocalibration in addition to the RTC oscillator controls.\n\nYou can [`read`](crate::Reg::read) this register and get [`octrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`octrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OctrlSpec;
impl crate::RegisterSpec for OctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`octrl::R`](R) reader structure"]
impl crate::Readable for OctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`octrl::W`](W) writer structure"]
impl crate::Writable for OctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OCTRL to value 0"]
impl crate::Resettable for OctrlSpec {
    const RESET_VALUE: u32 = 0;
}
