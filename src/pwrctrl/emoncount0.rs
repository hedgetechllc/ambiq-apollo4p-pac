#[doc = "Register `EMONCOUNT0` reader"]
pub type R = crate::R<Emoncount0Spec>;
#[doc = "Register `EMONCOUNT0` writer"]
pub type W = crate::W<Emoncount0Spec>;
#[doc = "Field `EMONCOUNT0` reader - Energy Monitor count value counter 0"]
pub type Emoncount0R = crate::FieldReader<u32>;
#[doc = "Field `EMONCOUNT0` writer - Energy Monitor count value counter 0"]
pub type Emoncount0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Energy Monitor count value counter 0"]
    #[inline(always)]
    pub fn emoncount0(&self) -> Emoncount0R {
        Emoncount0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Energy Monitor count value counter 0"]
    #[inline(always)]
    #[must_use]
    pub fn emoncount0(&mut self) -> Emoncount0W<Emoncount0Spec> {
        Emoncount0W::new(self, 0)
    }
}
#[doc = "Energy Monitor count value for counter 0\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncount0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncount0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Emoncount0Spec;
impl crate::RegisterSpec for Emoncount0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emoncount0::R`](R) reader structure"]
impl crate::Readable for Emoncount0Spec {}
#[doc = "`write(|w| ..)` method takes [`emoncount0::W`](W) writer structure"]
impl crate::Writable for Emoncount0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMONCOUNT0 to value 0"]
impl crate::Resettable for Emoncount0Spec {
    const RESET_VALUE: u32 = 0;
}
