#[doc = "Register `GHASHSUBKEY02` reader"]
pub type R = crate::R<Ghashsubkey02Spec>;
#[doc = "Register `GHASHSUBKEY02` writer"]
pub type W = crate::W<Ghashsubkey02Spec>;
#[doc = "Field `GHASHSUBKEY02` reader - Bits 95:64 of GHASH Key0."]
pub type Ghashsubkey02R = crate::FieldReader<u32>;
#[doc = "Field `GHASHSUBKEY02` writer - Bits 95:64 of GHASH Key0."]
pub type Ghashsubkey02W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bits 95:64 of GHASH Key0."]
    #[inline(always)]
    pub fn ghashsubkey02(&self) -> Ghashsubkey02R {
        Ghashsubkey02R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits 95:64 of GHASH Key0."]
    #[inline(always)]
    #[must_use]
    pub fn ghashsubkey02(&mut self) -> Ghashsubkey02W<Ghashsubkey02Spec> {
        Ghashsubkey02W::new(self, 0)
    }
}
#[doc = "Bits 95:64 of GHASH Key0 (used as the GHASH module key).\n\nYou can [`read`](crate::Reg::read) this register and get [`ghashsubkey02::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghashsubkey02::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ghashsubkey02Spec;
impl crate::RegisterSpec for Ghashsubkey02Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghashsubkey02::R`](R) reader structure"]
impl crate::Readable for Ghashsubkey02Spec {}
#[doc = "`write(|w| ..)` method takes [`ghashsubkey02::W`](W) writer structure"]
impl crate::Writable for Ghashsubkey02Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GHASHSUBKEY02 to value 0"]
impl crate::Resettable for Ghashsubkey02Spec {
    const RESET_VALUE: u32 = 0;
}
