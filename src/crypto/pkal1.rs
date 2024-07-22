#[doc = "Register `PKAL1` reader"]
pub type R = crate::R<Pkal1Spec>;
#[doc = "Register `PKAL1` writer"]
pub type W = crate::W<Pkal1Spec>;
#[doc = "Field `PKAL1` reader - Size of the operation in bytes."]
pub type Pkal1R = crate::FieldReader<u16>;
#[doc = "Field `PKAL1` writer - Size of the operation in bytes."]
pub type Pkal1W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Size of the operation in bytes."]
    #[inline(always)]
    pub fn pkal1(&self) -> Pkal1R {
        Pkal1R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Size of the operation in bytes."]
    #[inline(always)]
    #[must_use]
    pub fn pkal1(&mut self) -> Pkal1W<Pkal1Spec> {
        Pkal1W::new(self, 0)
    }
}
#[doc = "This register holds one of the optional size of the operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkal1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkal1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pkal1Spec;
impl crate::RegisterSpec for Pkal1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkal1::R`](R) reader structure"]
impl crate::Readable for Pkal1Spec {}
#[doc = "`write(|w| ..)` method takes [`pkal1::W`](W) writer structure"]
impl crate::Writable for Pkal1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKAL1 to value 0"]
impl crate::Resettable for Pkal1Spec {
    const RESET_VALUE: u32 = 0;
}
