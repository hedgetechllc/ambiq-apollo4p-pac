#[doc = "Register `EMONCOUNT3` reader"]
pub type R = crate::R<Emoncount3Spec>;
#[doc = "Register `EMONCOUNT3` writer"]
pub type W = crate::W<Emoncount3Spec>;
#[doc = "Field `EMONCOUNT3` reader - Energy Monitor count value counter 3"]
pub type Emoncount3R = crate::FieldReader<u32>;
#[doc = "Field `EMONCOUNT3` writer - Energy Monitor count value counter 3"]
pub type Emoncount3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Energy Monitor count value counter 3"]
    #[inline(always)]
    pub fn emoncount3(&self) -> Emoncount3R {
        Emoncount3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Energy Monitor count value counter 3"]
    #[inline(always)]
    #[must_use]
    pub fn emoncount3(&mut self) -> Emoncount3W<Emoncount3Spec> {
        Emoncount3W::new(self, 0)
    }
}
#[doc = "Energy Monitor count value for counter 3\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncount3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncount3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Emoncount3Spec;
impl crate::RegisterSpec for Emoncount3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emoncount3::R`](R) reader structure"]
impl crate::Readable for Emoncount3Spec {}
#[doc = "`write(|w| ..)` method takes [`emoncount3::W`](W) writer structure"]
impl crate::Writable for Emoncount3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMONCOUNT3 to value 0"]
impl crate::Resettable for Emoncount3Spec {
    const RESET_VALUE: u32 = 0;
}
