#[doc = "Register `ENS1` reader"]
pub type R = crate::R<Ens1Spec>;
#[doc = "Register `ENS1` writer"]
pub type W = crate::W<Ens1Spec>;
#[doc = "Field `ENS1` reader - GPIO63-32 Sets pin tri-state output enables. Writing a 1 to any bit sets the corresponding bit in the EN register. Writing a value of 0 has no effect on the corresponding bit in the EN register. Status reads should be made to the EN Register."]
pub type Ens1R = crate::FieldReader<u32>;
#[doc = "Field `ENS1` writer - GPIO63-32 Sets pin tri-state output enables. Writing a 1 to any bit sets the corresponding bit in the EN register. Writing a value of 0 has no effect on the corresponding bit in the EN register. Status reads should be made to the EN Register."]
pub type Ens1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO63-32 Sets pin tri-state output enables. Writing a 1 to any bit sets the corresponding bit in the EN register. Writing a value of 0 has no effect on the corresponding bit in the EN register. Status reads should be made to the EN Register."]
    #[inline(always)]
    pub fn ens1(&self) -> Ens1R {
        Ens1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO63-32 Sets pin tri-state output enables. Writing a 1 to any bit sets the corresponding bit in the EN register. Writing a value of 0 has no effect on the corresponding bit in the EN register. Status reads should be made to the EN Register."]
    #[inline(always)]
    #[must_use]
    pub fn ens1(&mut self) -> Ens1W<Ens1Spec> {
        Ens1W::new(self, 0)
    }
}
#[doc = "GPIO Enable Set 1 (63-32)\n\nYou can [`read`](crate::Reg::read) this register and get [`ens1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ens1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ens1Spec;
impl crate::RegisterSpec for Ens1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ens1::R`](R) reader structure"]
impl crate::Readable for Ens1Spec {}
#[doc = "`write(|w| ..)` method takes [`ens1::W`](W) writer structure"]
impl crate::Writable for Ens1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENS1 to value 0"]
impl crate::Resettable for Ens1Spec {
    const RESET_VALUE: u32 = 0;
}
