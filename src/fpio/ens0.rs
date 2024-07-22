#[doc = "Register `ENS0` reader"]
pub type R = crate::R<Ens0Spec>;
#[doc = "Register `ENS0` writer"]
pub type W = crate::W<Ens0Spec>;
#[doc = "Field `ENS0` reader - GPIO31-0 Sets pin tri-state output enables. Writing a 1 to any bit sets the corresponding bit in the EN register. Writing a value of 0 has no effect on the corresponding bit in the EN register. Status reads should be made to the EN Register."]
pub type Ens0R = crate::FieldReader<u32>;
#[doc = "Field `ENS0` writer - GPIO31-0 Sets pin tri-state output enables. Writing a 1 to any bit sets the corresponding bit in the EN register. Writing a value of 0 has no effect on the corresponding bit in the EN register. Status reads should be made to the EN Register."]
pub type Ens0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO31-0 Sets pin tri-state output enables. Writing a 1 to any bit sets the corresponding bit in the EN register. Writing a value of 0 has no effect on the corresponding bit in the EN register. Status reads should be made to the EN Register."]
    #[inline(always)]
    pub fn ens0(&self) -> Ens0R {
        Ens0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO31-0 Sets pin tri-state output enables. Writing a 1 to any bit sets the corresponding bit in the EN register. Writing a value of 0 has no effect on the corresponding bit in the EN register. Status reads should be made to the EN Register."]
    #[inline(always)]
    #[must_use]
    pub fn ens0(&mut self) -> Ens0W<Ens0Spec> {
        Ens0W::new(self, 0)
    }
}
#[doc = "GPIO Enable Set 0 (31-0)\n\nYou can [`read`](crate::Reg::read) this register and get [`ens0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ens0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ens0Spec;
impl crate::RegisterSpec for Ens0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ens0::R`](R) reader structure"]
impl crate::Readable for Ens0Spec {}
#[doc = "`write(|w| ..)` method takes [`ens0::W`](W) writer structure"]
impl crate::Writable for Ens0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENS0 to value 0"]
impl crate::Resettable for Ens0Spec {
    const RESET_VALUE: u32 = 0;
}
