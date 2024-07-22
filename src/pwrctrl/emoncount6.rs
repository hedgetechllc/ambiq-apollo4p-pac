#[doc = "Register `EMONCOUNT6` reader"]
pub type R = crate::R<Emoncount6Spec>;
#[doc = "Register `EMONCOUNT6` writer"]
pub type W = crate::W<Emoncount6Spec>;
#[doc = "Field `EMONCOUNT6` reader - Energy Monitor count value counter 6"]
pub type Emoncount6R = crate::FieldReader<u32>;
#[doc = "Field `EMONCOUNT6` writer - Energy Monitor count value counter 6"]
pub type Emoncount6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Energy Monitor count value counter 6"]
    #[inline(always)]
    pub fn emoncount6(&self) -> Emoncount6R {
        Emoncount6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Energy Monitor count value counter 6"]
    #[inline(always)]
    #[must_use]
    pub fn emoncount6(&mut self) -> Emoncount6W<Emoncount6Spec> {
        Emoncount6W::new(self, 0)
    }
}
#[doc = "Energy Monitor count value for counter 6\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncount6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncount6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Emoncount6Spec;
impl crate::RegisterSpec for Emoncount6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emoncount6::R`](R) reader structure"]
impl crate::Readable for Emoncount6Spec {}
#[doc = "`write(|w| ..)` method takes [`emoncount6::W`](W) writer structure"]
impl crate::Writable for Emoncount6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMONCOUNT6 to value 0"]
impl crate::Resettable for Emoncount6Spec {
    const RESET_VALUE: u32 = 0;
}
