#[doc = "Register `GHASHIV01` reader"]
pub type R = crate::R<Ghashiv01Spec>;
#[doc = "Register `GHASHIV01` writer"]
pub type W = crate::W<Ghashiv01Spec>;
#[doc = "Field `GHASHIV01` reader - Bits 63:32 of GHASH_IV0 register of the GHASH module."]
pub type Ghashiv01R = crate::FieldReader<u32>;
#[doc = "Field `GHASHIV01` writer - Bits 63:32 of GHASH_IV0 register of the GHASH module."]
pub type Ghashiv01W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bits 63:32 of GHASH_IV0 register of the GHASH module."]
    #[inline(always)]
    pub fn ghashiv01(&self) -> Ghashiv01R {
        Ghashiv01R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits 63:32 of GHASH_IV0 register of the GHASH module."]
    #[inline(always)]
    #[must_use]
    pub fn ghashiv01(&mut self) -> Ghashiv01W<Ghashiv01Spec> {
        Ghashiv01W::new(self, 0)
    }
}
#[doc = "Bits 63:32 of GHASH_IV0 register. GHASH IV0 is used as the GHASH IV (Initialization Value) register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ghashiv01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghashiv01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ghashiv01Spec;
impl crate::RegisterSpec for Ghashiv01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghashiv01::R`](R) reader structure"]
impl crate::Readable for Ghashiv01Spec {}
#[doc = "`write(|w| ..)` method takes [`ghashiv01::W`](W) writer structure"]
impl crate::Writable for Ghashiv01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GHASHIV01 to value 0"]
impl crate::Resettable for Ghashiv01Spec {
    const RESET_VALUE: u32 = 0;
}
