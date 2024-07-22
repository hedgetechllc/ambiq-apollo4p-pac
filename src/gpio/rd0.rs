#[doc = "Register `RD0` reader"]
pub type R = crate::R<Rd0Spec>;
#[doc = "Register `RD0` writer"]
pub type W = crate::W<Rd0Spec>;
#[doc = "Field `RD0` reader - GPIO31-0 Reads pin state - read only. Returns the pad pin state for pins 0-31 if the PINCFG's input enable (INPEN) is active and RDZERO is inactive."]
pub type Rd0R = crate::FieldReader<u32>;
#[doc = "Field `RD0` writer - GPIO31-0 Reads pin state - read only. Returns the pad pin state for pins 0-31 if the PINCFG's input enable (INPEN) is active and RDZERO is inactive."]
pub type Rd0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO31-0 Reads pin state - read only. Returns the pad pin state for pins 0-31 if the PINCFG's input enable (INPEN) is active and RDZERO is inactive."]
    #[inline(always)]
    pub fn rd0(&self) -> Rd0R {
        Rd0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO31-0 Reads pin state - read only. Returns the pad pin state for pins 0-31 if the PINCFG's input enable (INPEN) is active and RDZERO is inactive."]
    #[inline(always)]
    #[must_use]
    pub fn rd0(&mut self) -> Rd0W<Rd0Spec> {
        Rd0W::new(self, 0)
    }
}
#[doc = "GPIO Input 0 (31-0)\n\nYou can [`read`](crate::Reg::read) this register and get [`rd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rd0Spec;
impl crate::RegisterSpec for Rd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd0::R`](R) reader structure"]
impl crate::Readable for Rd0Spec {}
#[doc = "`write(|w| ..)` method takes [`rd0::W`](W) writer structure"]
impl crate::Writable for Rd0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RD0 to value 0"]
impl crate::Resettable for Rd0Spec {
    const RESET_VALUE: u32 = 0;
}
