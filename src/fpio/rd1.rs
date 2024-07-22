#[doc = "Register `RD1` reader"]
pub type R = crate::R<Rd1Spec>;
#[doc = "Register `RD1` writer"]
pub type W = crate::W<Rd1Spec>;
#[doc = "Field `RD1` reader - GPIO63-32 Reads pin state - read only. Returns the pad pin state for pins 0-31 if the PINCFG's input enable (INPEN) is active and RDZERO is inactive."]
pub type Rd1R = crate::FieldReader<u32>;
#[doc = "Field `RD1` writer - GPIO63-32 Reads pin state - read only. Returns the pad pin state for pins 0-31 if the PINCFG's input enable (INPEN) is active and RDZERO is inactive."]
pub type Rd1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO63-32 Reads pin state - read only. Returns the pad pin state for pins 0-31 if the PINCFG's input enable (INPEN) is active and RDZERO is inactive."]
    #[inline(always)]
    pub fn rd1(&self) -> Rd1R {
        Rd1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO63-32 Reads pin state - read only. Returns the pad pin state for pins 0-31 if the PINCFG's input enable (INPEN) is active and RDZERO is inactive."]
    #[inline(always)]
    #[must_use]
    pub fn rd1(&mut self) -> Rd1W<Rd1Spec> {
        Rd1W::new(self, 0)
    }
}
#[doc = "GPIO Input 1 (63-32)\n\nYou can [`read`](crate::Reg::read) this register and get [`rd1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rd1Spec;
impl crate::RegisterSpec for Rd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd1::R`](R) reader structure"]
impl crate::Readable for Rd1Spec {}
#[doc = "`write(|w| ..)` method takes [`rd1::W`](W) writer structure"]
impl crate::Writable for Rd1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RD1 to value 0"]
impl crate::Resettable for Rd1Spec {
    const RESET_VALUE: u32 = 0;
}
