#[doc = "Register `PKAL5` reader"]
pub type R = crate::R<Pkal5Spec>;
#[doc = "Register `PKAL5` writer"]
pub type W = crate::W<Pkal5Spec>;
#[doc = "Field `PKAL5` reader - Size of the operation in bytes."]
pub type Pkal5R = crate::FieldReader<u16>;
#[doc = "Field `PKAL5` writer - Size of the operation in bytes."]
pub type Pkal5W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Size of the operation in bytes."]
    #[inline(always)]
    pub fn pkal5(&self) -> Pkal5R {
        Pkal5R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Size of the operation in bytes."]
    #[inline(always)]
    #[must_use]
    pub fn pkal5(&mut self) -> Pkal5W<Pkal5Spec> {
        Pkal5W::new(self, 0)
    }
}
#[doc = "This register holds one of the optional size of the operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkal5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkal5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pkal5Spec;
impl crate::RegisterSpec for Pkal5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkal5::R`](R) reader structure"]
impl crate::Readable for Pkal5Spec {}
#[doc = "`write(|w| ..)` method takes [`pkal5::W`](W) writer structure"]
impl crate::Writable for Pkal5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKAL5 to value 0"]
impl crate::Resettable for Pkal5Spec {
    const RESET_VALUE: u32 = 0;
}
