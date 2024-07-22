#[doc = "Register `RD2` reader"]
pub type R = crate::R<Rd2Spec>;
#[doc = "Register `RD2` writer"]
pub type W = crate::W<Rd2Spec>;
#[doc = "Field `RD2` reader - GPIO95-64 Reads pin state - read only. Returns the pad pin state for pins 0-31 if the PINCFG's input enable (INPEN) is active and RDZERO is inactive."]
pub type Rd2R = crate::FieldReader<u32>;
#[doc = "Field `RD2` writer - GPIO95-64 Reads pin state - read only. Returns the pad pin state for pins 0-31 if the PINCFG's input enable (INPEN) is active and RDZERO is inactive."]
pub type Rd2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO95-64 Reads pin state - read only. Returns the pad pin state for pins 0-31 if the PINCFG's input enable (INPEN) is active and RDZERO is inactive."]
    #[inline(always)]
    pub fn rd2(&self) -> Rd2R {
        Rd2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO95-64 Reads pin state - read only. Returns the pad pin state for pins 0-31 if the PINCFG's input enable (INPEN) is active and RDZERO is inactive."]
    #[inline(always)]
    #[must_use]
    pub fn rd2(&mut self) -> Rd2W<Rd2Spec> {
        Rd2W::new(self, 0)
    }
}
#[doc = "GPIO Input 2 (95-64)\n\nYou can [`read`](crate::Reg::read) this register and get [`rd2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rd2Spec;
impl crate::RegisterSpec for Rd2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd2::R`](R) reader structure"]
impl crate::Readable for Rd2Spec {}
#[doc = "`write(|w| ..)` method takes [`rd2::W`](W) writer structure"]
impl crate::Writable for Rd2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RD2 to value 0"]
impl crate::Resettable for Rd2Spec {
    const RESET_VALUE: u32 = 0;
}
