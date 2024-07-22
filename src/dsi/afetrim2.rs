#[doc = "Register `AFETRIM2` reader"]
pub type R = crate::R<Afetrim2Spec>;
#[doc = "Register `AFETRIM2` writer"]
pub type W = crate::W<Afetrim2Spec>;
#[doc = "Field `AFETRIM2` reader - Afe Trim reg2."]
pub type Afetrim2R = crate::FieldReader<u32>;
#[doc = "Field `AFETRIM2` writer - Afe Trim reg2."]
pub type Afetrim2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Afe Trim reg2."]
    #[inline(always)]
    pub fn afetrim2(&self) -> Afetrim2R {
        Afetrim2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Afe Trim reg2."]
    #[inline(always)]
    #[must_use]
    pub fn afetrim2(&mut self) -> Afetrim2W<Afetrim2Spec> {
        Afetrim2W::new(self, 0)
    }
}
#[doc = "Afe Trim reg2\n\nYou can [`read`](crate::Reg::read) this register and get [`afetrim2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afetrim2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Afetrim2Spec;
impl crate::RegisterSpec for Afetrim2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afetrim2::R`](R) reader structure"]
impl crate::Readable for Afetrim2Spec {}
#[doc = "`write(|w| ..)` method takes [`afetrim2::W`](W) writer structure"]
impl crate::Writable for Afetrim2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFETRIM2 to value 0x1000_0000"]
impl crate::Resettable for Afetrim2Spec {
    const RESET_VALUE: u32 = 0x1000_0000;
}
