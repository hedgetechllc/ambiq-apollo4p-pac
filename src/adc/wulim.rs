#[doc = "Register `WULIM` reader"]
pub type R = crate::R<WulimSpec>;
#[doc = "Register `WULIM` writer"]
pub type W = crate::W<WulimSpec>;
#[doc = "Field `ULIM` reader - Sets the upper limit for the window comparator."]
pub type UlimR = crate::FieldReader<u32>;
#[doc = "Field `ULIM` writer - Sets the upper limit for the window comparator."]
pub type UlimW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Sets the upper limit for the window comparator."]
    #[inline(always)]
    pub fn ulim(&self) -> UlimR {
        UlimR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Sets the upper limit for the window comparator."]
    #[inline(always)]
    #[must_use]
    pub fn ulim(&mut self) -> UlimW<WulimSpec> {
        UlimW::new(self, 0)
    }
}
#[doc = "Window Comparator Upper Limits\n\nYou can [`read`](crate::Reg::read) this register and get [`wulim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wulim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WulimSpec;
impl crate::RegisterSpec for WulimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wulim::R`](R) reader structure"]
impl crate::Readable for WulimSpec {}
#[doc = "`write(|w| ..)` method takes [`wulim::W`](W) writer structure"]
impl crate::Writable for WulimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WULIM to value 0"]
impl crate::Resettable for WulimSpec {
    const RESET_VALUE: u32 = 0;
}
