#[doc = "Register `AFETRIM3` reader"]
pub type R = crate::R<Afetrim3Spec>;
#[doc = "Register `AFETRIM3` writer"]
pub type W = crate::W<Afetrim3Spec>;
#[doc = "Field `AFETRIM3` reader - Afe Trim reg3."]
pub type Afetrim3R = crate::FieldReader<u32>;
#[doc = "Field `AFETRIM3` writer - Afe Trim reg3."]
pub type Afetrim3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Afe Trim reg3."]
    #[inline(always)]
    pub fn afetrim3(&self) -> Afetrim3R {
        Afetrim3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Afe Trim reg3."]
    #[inline(always)]
    #[must_use]
    pub fn afetrim3(&mut self) -> Afetrim3W<Afetrim3Spec> {
        Afetrim3W::new(self, 0)
    }
}
#[doc = "Afe Trim reg3\n\nYou can [`read`](crate::Reg::read) this register and get [`afetrim3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afetrim3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Afetrim3Spec;
impl crate::RegisterSpec for Afetrim3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afetrim3::R`](R) reader structure"]
impl crate::Readable for Afetrim3Spec {}
#[doc = "`write(|w| ..)` method takes [`afetrim3::W`](W) writer structure"]
impl crate::Writable for Afetrim3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFETRIM3 to value 0x40f8"]
impl crate::Resettable for Afetrim3Spec {
    const RESET_VALUE: u32 = 0x40f8;
}
