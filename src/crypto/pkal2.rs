#[doc = "Register `PKAL2` reader"]
pub type R = crate::R<Pkal2Spec>;
#[doc = "Register `PKAL2` writer"]
pub type W = crate::W<Pkal2Spec>;
#[doc = "Field `PKAL2` reader - Size of the operation in bytes."]
pub type Pkal2R = crate::FieldReader<u16>;
#[doc = "Field `PKAL2` writer - Size of the operation in bytes."]
pub type Pkal2W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Size of the operation in bytes."]
    #[inline(always)]
    pub fn pkal2(&self) -> Pkal2R {
        Pkal2R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Size of the operation in bytes."]
    #[inline(always)]
    #[must_use]
    pub fn pkal2(&mut self) -> Pkal2W<Pkal2Spec> {
        Pkal2W::new(self, 0)
    }
}
#[doc = "This register holds one of the optional size of the operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkal2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkal2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pkal2Spec;
impl crate::RegisterSpec for Pkal2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkal2::R`](R) reader structure"]
impl crate::Readable for Pkal2Spec {}
#[doc = "`write(|w| ..)` method takes [`pkal2::W`](W) writer structure"]
impl crate::Writable for Pkal2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKAL2 to value 0"]
impl crate::Resettable for Pkal2Spec {
    const RESET_VALUE: u32 = 0;
}
