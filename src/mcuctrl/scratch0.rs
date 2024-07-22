#[doc = "Register `SCRATCH0` reader"]
pub type R = crate::R<Scratch0Spec>;
#[doc = "Register `SCRATCH0` writer"]
pub type W = crate::W<Scratch0Spec>;
#[doc = "Field `SCRATCH0` reader - Scratch register 0."]
pub type Scratch0R = crate::FieldReader<u32>;
#[doc = "Field `SCRATCH0` writer - Scratch register 0."]
pub type Scratch0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Scratch register 0."]
    #[inline(always)]
    pub fn scratch0(&self) -> Scratch0R {
        Scratch0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Scratch register 0."]
    #[inline(always)]
    #[must_use]
    pub fn scratch0(&mut self) -> Scratch0W<Scratch0Spec> {
        Scratch0W::new(self, 0)
    }
}
#[doc = "Scratch register that is not reset by any reset\n\nYou can [`read`](crate::Reg::read) this register and get [`scratch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scratch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scratch0Spec;
impl crate::RegisterSpec for Scratch0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scratch0::R`](R) reader structure"]
impl crate::Readable for Scratch0Spec {}
#[doc = "`write(|w| ..)` method takes [`scratch0::W`](W) writer structure"]
impl crate::Writable for Scratch0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCRATCH0 to value 0"]
impl crate::Resettable for Scratch0Spec {
    const RESET_VALUE: u32 = 0;
}
