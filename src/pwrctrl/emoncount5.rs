#[doc = "Register `EMONCOUNT5` reader"]
pub type R = crate::R<Emoncount5Spec>;
#[doc = "Register `EMONCOUNT5` writer"]
pub type W = crate::W<Emoncount5Spec>;
#[doc = "Field `EMONCOUNT5` reader - Energy Monitor count value counter 5"]
pub type Emoncount5R = crate::FieldReader<u32>;
#[doc = "Field `EMONCOUNT5` writer - Energy Monitor count value counter 5"]
pub type Emoncount5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Energy Monitor count value counter 5"]
    #[inline(always)]
    pub fn emoncount5(&self) -> Emoncount5R {
        Emoncount5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Energy Monitor count value counter 5"]
    #[inline(always)]
    #[must_use]
    pub fn emoncount5(&mut self) -> Emoncount5W<Emoncount5Spec> {
        Emoncount5W::new(self, 0)
    }
}
#[doc = "Energy Monitor count value for counter 5\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncount5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncount5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Emoncount5Spec;
impl crate::RegisterSpec for Emoncount5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emoncount5::R`](R) reader structure"]
impl crate::Readable for Emoncount5Spec {}
#[doc = "`write(|w| ..)` method takes [`emoncount5::W`](W) writer structure"]
impl crate::Writable for Emoncount5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMONCOUNT5 to value 0"]
impl crate::Resettable for Emoncount5Spec {
    const RESET_VALUE: u32 = 0;
}
