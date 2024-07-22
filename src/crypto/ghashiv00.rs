#[doc = "Register `GHASHIV00` reader"]
pub type R = crate::R<Ghashiv00Spec>;
#[doc = "Register `GHASHIV00` writer"]
pub type W = crate::W<Ghashiv00Spec>;
#[doc = "Field `GHASHIV00` reader - Bits 31:0 of GHASH_IV0 register of the GHASH module. For the description of GHASH_IV0, see the GHASH_0_0 register description"]
pub type Ghashiv00R = crate::FieldReader<u32>;
#[doc = "Field `GHASHIV00` writer - Bits 31:0 of GHASH_IV0 register of the GHASH module. For the description of GHASH_IV0, see the GHASH_0_0 register description"]
pub type Ghashiv00W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bits 31:0 of GHASH_IV0 register of the GHASH module. For the description of GHASH_IV0, see the GHASH_0_0 register description"]
    #[inline(always)]
    pub fn ghashiv00(&self) -> Ghashiv00R {
        Ghashiv00R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits 31:0 of GHASH_IV0 register of the GHASH module. For the description of GHASH_IV0, see the GHASH_0_0 register description"]
    #[inline(always)]
    #[must_use]
    pub fn ghashiv00(&mut self) -> Ghashiv00W<Ghashiv00Spec> {
        Ghashiv00W::new(self, 0)
    }
}
#[doc = "Bits 31:0 of GHASH_IV0 register. GHASH IV0 is used as the GHASH IV (Initialization Value) register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ghashiv00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghashiv00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ghashiv00Spec;
impl crate::RegisterSpec for Ghashiv00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghashiv00::R`](R) reader structure"]
impl crate::Readable for Ghashiv00Spec {}
#[doc = "`write(|w| ..)` method takes [`ghashiv00::W`](W) writer structure"]
impl crate::Writable for Ghashiv00Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GHASHIV00 to value 0"]
impl crate::Resettable for Ghashiv00Spec {
    const RESET_VALUE: u32 = 0;
}
