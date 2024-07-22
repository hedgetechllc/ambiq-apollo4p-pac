#[doc = "Register `ENC1` reader"]
pub type R = crate::R<Enc1Spec>;
#[doc = "Register `ENC1` writer"]
pub type W = crate::W<Enc1Spec>;
#[doc = "Field `ENC1` reader - GPIO63-32 Clears pin tri-state output enables. Writing a 1 to any bit clears the corresponding bit in the EN register. Writing a value of 0 has no effect on the corresponding bit in the EN register. Status reads should be made to the EN Register."]
pub type Enc1R = crate::FieldReader<u32>;
#[doc = "Field `ENC1` writer - GPIO63-32 Clears pin tri-state output enables. Writing a 1 to any bit clears the corresponding bit in the EN register. Writing a value of 0 has no effect on the corresponding bit in the EN register. Status reads should be made to the EN Register."]
pub type Enc1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO63-32 Clears pin tri-state output enables. Writing a 1 to any bit clears the corresponding bit in the EN register. Writing a value of 0 has no effect on the corresponding bit in the EN register. Status reads should be made to the EN Register."]
    #[inline(always)]
    pub fn enc1(&self) -> Enc1R {
        Enc1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO63-32 Clears pin tri-state output enables. Writing a 1 to any bit clears the corresponding bit in the EN register. Writing a value of 0 has no effect on the corresponding bit in the EN register. Status reads should be made to the EN Register."]
    #[inline(always)]
    #[must_use]
    pub fn enc1(&mut self) -> Enc1W<Enc1Spec> {
        Enc1W::new(self, 0)
    }
}
#[doc = "GPIO Enable Clear 1 (63-32)\n\nYou can [`read`](crate::Reg::read) this register and get [`enc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Enc1Spec;
impl crate::RegisterSpec for Enc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enc1::R`](R) reader structure"]
impl crate::Readable for Enc1Spec {}
#[doc = "`write(|w| ..)` method takes [`enc1::W`](W) writer structure"]
impl crate::Writable for Enc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENC1 to value 0"]
impl crate::Resettable for Enc1Spec {
    const RESET_VALUE: u32 = 0;
}
