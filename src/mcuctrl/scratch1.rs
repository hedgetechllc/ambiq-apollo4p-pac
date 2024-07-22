#[doc = "Register `SCRATCH1` reader"]
pub type R = crate::R<Scratch1Spec>;
#[doc = "Register `SCRATCH1` writer"]
pub type W = crate::W<Scratch1Spec>;
#[doc = "Field `SCRATCH1` reader - Scratch register 1."]
pub type Scratch1R = crate::FieldReader<u32>;
#[doc = "Field `SCRATCH1` writer - Scratch register 1."]
pub type Scratch1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Scratch register 1."]
    #[inline(always)]
    pub fn scratch1(&self) -> Scratch1R {
        Scratch1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Scratch register 1."]
    #[inline(always)]
    #[must_use]
    pub fn scratch1(&mut self) -> Scratch1W<Scratch1Spec> {
        Scratch1W::new(self, 0)
    }
}
#[doc = "Scratch register that is not reset by any reset\n\nYou can [`read`](crate::Reg::read) this register and get [`scratch1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scratch1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scratch1Spec;
impl crate::RegisterSpec for Scratch1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scratch1::R`](R) reader structure"]
impl crate::Readable for Scratch1Spec {}
#[doc = "`write(|w| ..)` method takes [`scratch1::W`](W) writer structure"]
impl crate::Writable for Scratch1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCRATCH1 to value 0"]
impl crate::Resettable for Scratch1Spec {
    const RESET_VALUE: u32 = 0;
}
