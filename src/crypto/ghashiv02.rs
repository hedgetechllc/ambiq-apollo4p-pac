#[doc = "Register `GHASHIV02` reader"]
pub type R = crate::R<Ghashiv02Spec>;
#[doc = "Register `GHASHIV02` writer"]
pub type W = crate::W<Ghashiv02Spec>;
#[doc = "Field `GHASHIV02` reader - Bits 95:64 of GHASH_IV0 register of the GHASH module."]
pub type Ghashiv02R = crate::FieldReader<u32>;
#[doc = "Field `GHASHIV02` writer - Bits 95:64 of GHASH_IV0 register of the GHASH module."]
pub type Ghashiv02W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bits 95:64 of GHASH_IV0 register of the GHASH module."]
    #[inline(always)]
    pub fn ghashiv02(&self) -> Ghashiv02R {
        Ghashiv02R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits 95:64 of GHASH_IV0 register of the GHASH module."]
    #[inline(always)]
    #[must_use]
    pub fn ghashiv02(&mut self) -> Ghashiv02W<Ghashiv02Spec> {
        Ghashiv02W::new(self, 0)
    }
}
#[doc = "Bits 95:64 of GHASH_IV0 register. GHASH IV0 is used as the GHASH IV (Initialization Value) register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ghashiv02::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghashiv02::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ghashiv02Spec;
impl crate::RegisterSpec for Ghashiv02Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghashiv02::R`](R) reader structure"]
impl crate::Readable for Ghashiv02Spec {}
#[doc = "`write(|w| ..)` method takes [`ghashiv02::W`](W) writer structure"]
impl crate::Writable for Ghashiv02Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GHASHIV02 to value 0"]
impl crate::Resettable for Ghashiv02Spec {
    const RESET_VALUE: u32 = 0;
}
