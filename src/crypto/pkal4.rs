#[doc = "Register `PKAL4` reader"]
pub type R = crate::R<Pkal4Spec>;
#[doc = "Register `PKAL4` writer"]
pub type W = crate::W<Pkal4Spec>;
#[doc = "Field `PKAL4` reader - Size of the operation in bytes."]
pub type Pkal4R = crate::FieldReader<u16>;
#[doc = "Field `PKAL4` writer - Size of the operation in bytes."]
pub type Pkal4W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Size of the operation in bytes."]
    #[inline(always)]
    pub fn pkal4(&self) -> Pkal4R {
        Pkal4R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Size of the operation in bytes."]
    #[inline(always)]
    #[must_use]
    pub fn pkal4(&mut self) -> Pkal4W<Pkal4Spec> {
        Pkal4W::new(self, 0)
    }
}
#[doc = "This register holds one of the optional size of the operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkal4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkal4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pkal4Spec;
impl crate::RegisterSpec for Pkal4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkal4::R`](R) reader structure"]
impl crate::Readable for Pkal4Spec {}
#[doc = "`write(|w| ..)` method takes [`pkal4::W`](W) writer structure"]
impl crate::Writable for Pkal4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKAL4 to value 0"]
impl crate::Resettable for Pkal4Spec {
    const RESET_VALUE: u32 = 0;
}
