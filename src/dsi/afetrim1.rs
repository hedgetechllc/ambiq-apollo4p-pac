#[doc = "Register `AFETRIM1` reader"]
pub type R = crate::R<Afetrim1Spec>;
#[doc = "Register `AFETRIM1` writer"]
pub type W = crate::W<Afetrim1Spec>;
#[doc = "Field `AFETRIM1` reader - Afe Trim reg1."]
pub type Afetrim1R = crate::FieldReader<u32>;
#[doc = "Field `AFETRIM1` writer - Afe Trim reg1."]
pub type Afetrim1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Afe Trim reg1."]
    #[inline(always)]
    pub fn afetrim1(&self) -> Afetrim1R {
        Afetrim1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Afe Trim reg1."]
    #[inline(always)]
    #[must_use]
    pub fn afetrim1(&mut self) -> Afetrim1W<Afetrim1Spec> {
        Afetrim1W::new(self, 0)
    }
}
#[doc = "Afe Trim reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`afetrim1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afetrim1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Afetrim1Spec;
impl crate::RegisterSpec for Afetrim1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afetrim1::R`](R) reader structure"]
impl crate::Readable for Afetrim1Spec {}
#[doc = "`write(|w| ..)` method takes [`afetrim1::W`](W) writer structure"]
impl crate::Writable for Afetrim1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFETRIM1 to value 0x1564_0152"]
impl crate::Resettable for Afetrim1Spec {
    const RESET_VALUE: u32 = 0x1564_0152;
}
