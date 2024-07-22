#[doc = "Register `EMONCOUNT4` reader"]
pub type R = crate::R<Emoncount4Spec>;
#[doc = "Register `EMONCOUNT4` writer"]
pub type W = crate::W<Emoncount4Spec>;
#[doc = "Field `EMONCOUNT4` reader - Energy Monitor count value counter 4"]
pub type Emoncount4R = crate::FieldReader<u32>;
#[doc = "Field `EMONCOUNT4` writer - Energy Monitor count value counter 4"]
pub type Emoncount4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Energy Monitor count value counter 4"]
    #[inline(always)]
    pub fn emoncount4(&self) -> Emoncount4R {
        Emoncount4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Energy Monitor count value counter 4"]
    #[inline(always)]
    #[must_use]
    pub fn emoncount4(&mut self) -> Emoncount4W<Emoncount4Spec> {
        Emoncount4W::new(self, 0)
    }
}
#[doc = "Energy Monitor count value for counter 4\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncount4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncount4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Emoncount4Spec;
impl crate::RegisterSpec for Emoncount4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emoncount4::R`](R) reader structure"]
impl crate::Readable for Emoncount4Spec {}
#[doc = "`write(|w| ..)` method takes [`emoncount4::W`](W) writer structure"]
impl crate::Writable for Emoncount4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMONCOUNT4 to value 0"]
impl crate::Resettable for Emoncount4Spec {
    const RESET_VALUE: u32 = 0;
}
