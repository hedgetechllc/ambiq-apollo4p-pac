#[doc = "Register `GHASHIV03` reader"]
pub type R = crate::R<Ghashiv03Spec>;
#[doc = "Register `GHASHIV03` writer"]
pub type W = crate::W<Ghashiv03Spec>;
#[doc = "Field `GHASHIV03` reader - Bits 127:96 of GHASH_IV0 register of the GHASH module."]
pub type Ghashiv03R = crate::FieldReader<u32>;
#[doc = "Field `GHASHIV03` writer - Bits 127:96 of GHASH_IV0 register of the GHASH module."]
pub type Ghashiv03W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bits 127:96 of GHASH_IV0 register of the GHASH module."]
    #[inline(always)]
    pub fn ghashiv03(&self) -> Ghashiv03R {
        Ghashiv03R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits 127:96 of GHASH_IV0 register of the GHASH module."]
    #[inline(always)]
    #[must_use]
    pub fn ghashiv03(&mut self) -> Ghashiv03W<Ghashiv03Spec> {
        Ghashiv03W::new(self, 0)
    }
}
#[doc = "Bits 127:96 of GHASH_IV0 register.GHASH IV0 is used as the GHASH IV (Initialization Value) register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ghashiv03::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghashiv03::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ghashiv03Spec;
impl crate::RegisterSpec for Ghashiv03Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghashiv03::R`](R) reader structure"]
impl crate::Readable for Ghashiv03Spec {}
#[doc = "`write(|w| ..)` method takes [`ghashiv03::W`](W) writer structure"]
impl crate::Writable for Ghashiv03Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GHASHIV03 to value 0"]
impl crate::Resettable for Ghashiv03Spec {
    const RESET_VALUE: u32 = 0;
}
