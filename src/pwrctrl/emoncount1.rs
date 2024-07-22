#[doc = "Register `EMONCOUNT1` reader"]
pub type R = crate::R<Emoncount1Spec>;
#[doc = "Register `EMONCOUNT1` writer"]
pub type W = crate::W<Emoncount1Spec>;
#[doc = "Field `EMONCOUNT1` reader - Energy Monitor count value counter 1"]
pub type Emoncount1R = crate::FieldReader<u32>;
#[doc = "Field `EMONCOUNT1` writer - Energy Monitor count value counter 1"]
pub type Emoncount1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Energy Monitor count value counter 1"]
    #[inline(always)]
    pub fn emoncount1(&self) -> Emoncount1R {
        Emoncount1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Energy Monitor count value counter 1"]
    #[inline(always)]
    #[must_use]
    pub fn emoncount1(&mut self) -> Emoncount1W<Emoncount1Spec> {
        Emoncount1W::new(self, 0)
    }
}
#[doc = "Energy Monitor count value for counter 1\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncount1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncount1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Emoncount1Spec;
impl crate::RegisterSpec for Emoncount1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emoncount1::R`](R) reader structure"]
impl crate::Readable for Emoncount1Spec {}
#[doc = "`write(|w| ..)` method takes [`emoncount1::W`](W) writer structure"]
impl crate::Writable for Emoncount1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMONCOUNT1 to value 0"]
impl crate::Resettable for Emoncount1Spec {
    const RESET_VALUE: u32 = 0;
}