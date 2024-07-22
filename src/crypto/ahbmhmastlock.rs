#[doc = "Register `AHBMHMASTLOCK` reader"]
pub type R = crate::R<AhbmhmastlockSpec>;
#[doc = "Register `AHBMHMASTLOCK` writer"]
pub type W = crate::W<AhbmhmastlockSpec>;
#[doc = "Field `AHBHMASTLOCK` reader - The hmastlock value."]
pub type AhbhmastlockR = crate::BitReader;
#[doc = "Field `AHBHMASTLOCK` writer - The hmastlock value."]
pub type AhbhmastlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The hmastlock value."]
    #[inline(always)]
    pub fn ahbhmastlock(&self) -> AhbhmastlockR {
        AhbhmastlockR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The hmastlock value."]
    #[inline(always)]
    #[must_use]
    pub fn ahbhmastlock(&mut self) -> AhbhmastlockW<AhbmhmastlockSpec> {
        AhbhmastlockW::new(self, 0)
    }
}
#[doc = "This register holds ahb hmastlock value\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbmhmastlock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbmhmastlock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbmhmastlockSpec;
impl crate::RegisterSpec for AhbmhmastlockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbmhmastlock::R`](R) reader structure"]
impl crate::Readable for AhbmhmastlockSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbmhmastlock::W`](W) writer structure"]
impl crate::Writable for AhbmhmastlockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBMHMASTLOCK to value 0"]
impl crate::Resettable for AhbmhmastlockSpec {
    const RESET_VALUE: u32 = 0;
}
