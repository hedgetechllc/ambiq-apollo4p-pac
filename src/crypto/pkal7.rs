#[doc = "Register `PKAL7` reader"]
pub type R = crate::R<Pkal7Spec>;
#[doc = "Register `PKAL7` writer"]
pub type W = crate::W<Pkal7Spec>;
#[doc = "Field `PKAL7` reader - Size of the operation in bytes."]
pub type Pkal7R = crate::FieldReader<u16>;
#[doc = "Field `PKAL7` writer - Size of the operation in bytes."]
pub type Pkal7W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Size of the operation in bytes."]
    #[inline(always)]
    pub fn pkal7(&self) -> Pkal7R {
        Pkal7R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Size of the operation in bytes."]
    #[inline(always)]
    #[must_use]
    pub fn pkal7(&mut self) -> Pkal7W<Pkal7Spec> {
        Pkal7W::new(self, 0)
    }
}
#[doc = "This register holds one of the optional size of the operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkal7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkal7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pkal7Spec;
impl crate::RegisterSpec for Pkal7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkal7::R`](R) reader structure"]
impl crate::Readable for Pkal7Spec {}
#[doc = "`write(|w| ..)` method takes [`pkal7::W`](W) writer structure"]
impl crate::Writable for Pkal7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKAL7 to value 0"]
impl crate::Resettable for Pkal7Spec {
    const RESET_VALUE: u32 = 0;
}
