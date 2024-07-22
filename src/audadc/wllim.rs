#[doc = "Register `WLLIM` reader"]
pub type R = crate::R<WllimSpec>;
#[doc = "Register `WLLIM` writer"]
pub type W = crate::W<WllimSpec>;
#[doc = "Field `LLIM` reader - Sets the lower limit for the window comparator."]
pub type LlimR = crate::FieldReader<u32>;
#[doc = "Field `LLIM` writer - Sets the lower limit for the window comparator."]
pub type LlimW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Sets the lower limit for the window comparator."]
    #[inline(always)]
    pub fn llim(&self) -> LlimR {
        LlimR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Sets the lower limit for the window comparator."]
    #[inline(always)]
    #[must_use]
    pub fn llim(&mut self) -> LlimW<WllimSpec> {
        LlimW::new(self, 0)
    }
}
#[doc = "Window Comparator Lower Limits\n\nYou can [`read`](crate::Reg::read) this register and get [`wllim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wllim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WllimSpec;
impl crate::RegisterSpec for WllimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wllim::R`](R) reader structure"]
impl crate::Readable for WllimSpec {}
#[doc = "`write(|w| ..)` method takes [`wllim::W`](W) writer structure"]
impl crate::Writable for WllimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WLLIM to value 0"]
impl crate::Resettable for WllimSpec {
    const RESET_VALUE: u32 = 0;
}
