#[doc = "Register `EMONCOUNT7` reader"]
pub type R = crate::R<Emoncount7Spec>;
#[doc = "Register `EMONCOUNT7` writer"]
pub type W = crate::W<Emoncount7Spec>;
#[doc = "Field `EMONCOUNT7` reader - Energy Monitor count value counter 7"]
pub type Emoncount7R = crate::FieldReader<u32>;
#[doc = "Field `EMONCOUNT7` writer - Energy Monitor count value counter 7"]
pub type Emoncount7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Energy Monitor count value counter 7"]
    #[inline(always)]
    pub fn emoncount7(&self) -> Emoncount7R {
        Emoncount7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Energy Monitor count value counter 7"]
    #[inline(always)]
    #[must_use]
    pub fn emoncount7(&mut self) -> Emoncount7W<Emoncount7Spec> {
        Emoncount7W::new(self, 0)
    }
}
#[doc = "Energy Monitor count value for counter 7\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncount7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncount7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Emoncount7Spec;
impl crate::RegisterSpec for Emoncount7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emoncount7::R`](R) reader structure"]
impl crate::Readable for Emoncount7Spec {}
#[doc = "`write(|w| ..)` method takes [`emoncount7::W`](W) writer structure"]
impl crate::Writable for Emoncount7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMONCOUNT7 to value 0"]
impl crate::Resettable for Emoncount7Spec {
    const RESET_VALUE: u32 = 0;
}
