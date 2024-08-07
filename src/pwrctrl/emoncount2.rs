#[doc = "Register `EMONCOUNT2` reader"]
pub type R = crate::R<Emoncount2Spec>;
#[doc = "Register `EMONCOUNT2` writer"]
pub type W = crate::W<Emoncount2Spec>;
#[doc = "Field `EMONCOUNT2` reader - Energy Monitor count value counter 2"]
pub type Emoncount2R = crate::FieldReader<u32>;
#[doc = "Field `EMONCOUNT2` writer - Energy Monitor count value counter 2"]
pub type Emoncount2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Energy Monitor count value counter 2"]
    #[inline(always)]
    pub fn emoncount2(&self) -> Emoncount2R {
        Emoncount2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Energy Monitor count value counter 2"]
    #[inline(always)]
    #[must_use]
    pub fn emoncount2(&mut self) -> Emoncount2W<Emoncount2Spec> {
        Emoncount2W::new(self, 0)
    }
}
#[doc = "Energy Monitor count value for counter 2\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncount2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncount2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Emoncount2Spec;
impl crate::RegisterSpec for Emoncount2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emoncount2::R`](R) reader structure"]
impl crate::Readable for Emoncount2Spec {}
#[doc = "`write(|w| ..)` method takes [`emoncount2::W`](W) writer structure"]
impl crate::Writable for Emoncount2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMONCOUNT2 to value 0"]
impl crate::Resettable for Emoncount2Spec {
    const RESET_VALUE: u32 = 0;
}
