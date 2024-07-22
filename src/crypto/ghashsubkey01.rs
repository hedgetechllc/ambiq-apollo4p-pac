#[doc = "Register `GHASHSUBKEY01` reader"]
pub type R = crate::R<Ghashsubkey01Spec>;
#[doc = "Register `GHASHSUBKEY01` writer"]
pub type W = crate::W<Ghashsubkey01Spec>;
#[doc = "Field `GHASHSUBKEY01` reader - Bits 63:32 of GHASH Key0."]
pub type Ghashsubkey01R = crate::FieldReader<u32>;
#[doc = "Field `GHASHSUBKEY01` writer - Bits 63:32 of GHASH Key0."]
pub type Ghashsubkey01W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bits 63:32 of GHASH Key0."]
    #[inline(always)]
    pub fn ghashsubkey01(&self) -> Ghashsubkey01R {
        Ghashsubkey01R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits 63:32 of GHASH Key0."]
    #[inline(always)]
    #[must_use]
    pub fn ghashsubkey01(&mut self) -> Ghashsubkey01W<Ghashsubkey01Spec> {
        Ghashsubkey01W::new(self, 0)
    }
}
#[doc = "Bits 63:32 of GHASH Key0 (used as the GHASH module key).\n\nYou can [`read`](crate::Reg::read) this register and get [`ghashsubkey01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghashsubkey01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ghashsubkey01Spec;
impl crate::RegisterSpec for Ghashsubkey01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghashsubkey01::R`](R) reader structure"]
impl crate::Readable for Ghashsubkey01Spec {}
#[doc = "`write(|w| ..)` method takes [`ghashsubkey01::W`](W) writer structure"]
impl crate::Writable for Ghashsubkey01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GHASHSUBKEY01 to value 0"]
impl crate::Resettable for Ghashsubkey01Spec {
    const RESET_VALUE: u32 = 0;
}
