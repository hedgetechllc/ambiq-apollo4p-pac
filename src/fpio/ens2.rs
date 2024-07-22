#[doc = "Register `ENS2` reader"]
pub type R = crate::R<Ens2Spec>;
#[doc = "Register `ENS2` writer"]
pub type W = crate::W<Ens2Spec>;
#[doc = "Field `ENS2` reader - GPIO95-64 Sets pin tri-state output enables. Writing a 1 to any bit sets the corresponding bit in the EN register. Writing a value of 0 has no effect on the corresponding bit in the EN register. Status reads should be made to the EN Register."]
pub type Ens2R = crate::FieldReader<u32>;
#[doc = "Field `ENS2` writer - GPIO95-64 Sets pin tri-state output enables. Writing a 1 to any bit sets the corresponding bit in the EN register. Writing a value of 0 has no effect on the corresponding bit in the EN register. Status reads should be made to the EN Register."]
pub type Ens2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO95-64 Sets pin tri-state output enables. Writing a 1 to any bit sets the corresponding bit in the EN register. Writing a value of 0 has no effect on the corresponding bit in the EN register. Status reads should be made to the EN Register."]
    #[inline(always)]
    pub fn ens2(&self) -> Ens2R {
        Ens2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO95-64 Sets pin tri-state output enables. Writing a 1 to any bit sets the corresponding bit in the EN register. Writing a value of 0 has no effect on the corresponding bit in the EN register. Status reads should be made to the EN Register."]
    #[inline(always)]
    #[must_use]
    pub fn ens2(&mut self) -> Ens2W<Ens2Spec> {
        Ens2W::new(self, 0)
    }
}
#[doc = "GPIO Enable Set 2 (95-64)\n\nYou can [`read`](crate::Reg::read) this register and get [`ens2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ens2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ens2Spec;
impl crate::RegisterSpec for Ens2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ens2::R`](R) reader structure"]
impl crate::Readable for Ens2Spec {}
#[doc = "`write(|w| ..)` method takes [`ens2::W`](W) writer structure"]
impl crate::Writable for Ens2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENS2 to value 0"]
impl crate::Resettable for Ens2Spec {
    const RESET_VALUE: u32 = 0;
}
