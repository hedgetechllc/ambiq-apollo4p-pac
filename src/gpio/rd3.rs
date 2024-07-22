#[doc = "Register `RD3` reader"]
pub type R = crate::R<Rd3Spec>;
#[doc = "Register `RD3` writer"]
pub type W = crate::W<Rd3Spec>;
#[doc = "Field `RD3` reader - GPIO127-96 Reads pin state - read only. Returns the pad pin state for pins 0-31 if the PINCFG's input enable (INPEN) is active and RDZERO is inactive."]
pub type Rd3R = crate::FieldReader<u32>;
#[doc = "Field `RD3` writer - GPIO127-96 Reads pin state - read only. Returns the pad pin state for pins 0-31 if the PINCFG's input enable (INPEN) is active and RDZERO is inactive."]
pub type Rd3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO127-96 Reads pin state - read only. Returns the pad pin state for pins 0-31 if the PINCFG's input enable (INPEN) is active and RDZERO is inactive."]
    #[inline(always)]
    pub fn rd3(&self) -> Rd3R {
        Rd3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO127-96 Reads pin state - read only. Returns the pad pin state for pins 0-31 if the PINCFG's input enable (INPEN) is active and RDZERO is inactive."]
    #[inline(always)]
    #[must_use]
    pub fn rd3(&mut self) -> Rd3W<Rd3Spec> {
        Rd3W::new(self, 0)
    }
}
#[doc = "GPIO Input 3 (127-96)\n\nYou can [`read`](crate::Reg::read) this register and get [`rd3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rd3Spec;
impl crate::RegisterSpec for Rd3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd3::R`](R) reader structure"]
impl crate::Readable for Rd3Spec {}
#[doc = "`write(|w| ..)` method takes [`rd3::W`](W) writer structure"]
impl crate::Writable for Rd3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RD3 to value 0"]
impl crate::Resettable for Rd3Spec {
    const RESET_VALUE: u32 = 0;
}
