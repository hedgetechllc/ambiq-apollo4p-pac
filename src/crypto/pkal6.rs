#[doc = "Register `PKAL6` reader"]
pub type R = crate::R<Pkal6Spec>;
#[doc = "Register `PKAL6` writer"]
pub type W = crate::W<Pkal6Spec>;
#[doc = "Field `PKAL6` reader - Size of the operation in bytes."]
pub type Pkal6R = crate::FieldReader<u16>;
#[doc = "Field `PKAL6` writer - Size of the operation in bytes."]
pub type Pkal6W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Size of the operation in bytes."]
    #[inline(always)]
    pub fn pkal6(&self) -> Pkal6R {
        Pkal6R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Size of the operation in bytes."]
    #[inline(always)]
    #[must_use]
    pub fn pkal6(&mut self) -> Pkal6W<Pkal6Spec> {
        Pkal6W::new(self, 0)
    }
}
#[doc = "This register holds one of the optional size of the operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkal6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkal6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pkal6Spec;
impl crate::RegisterSpec for Pkal6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkal6::R`](R) reader structure"]
impl crate::Readable for Pkal6Spec {}
#[doc = "`write(|w| ..)` method takes [`pkal6::W`](W) writer structure"]
impl crate::Writable for Pkal6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKAL6 to value 0"]
impl crate::Resettable for Pkal6Spec {
    const RESET_VALUE: u32 = 0;
}
