#[doc = "Register `GHASHSUBKEY00` reader"]
pub type R = crate::R<Ghashsubkey00Spec>;
#[doc = "Register `GHASHSUBKEY00` writer"]
pub type W = crate::W<Ghashsubkey00Spec>;
#[doc = "Field `GHASHSUBKEY00` reader - Bits 31:0 of GHASH Key0."]
pub type Ghashsubkey00R = crate::FieldReader<u32>;
#[doc = "Field `GHASHSUBKEY00` writer - Bits 31:0 of GHASH Key0."]
pub type Ghashsubkey00W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bits 31:0 of GHASH Key0."]
    #[inline(always)]
    pub fn ghashsubkey00(&self) -> Ghashsubkey00R {
        Ghashsubkey00R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits 31:0 of GHASH Key0."]
    #[inline(always)]
    #[must_use]
    pub fn ghashsubkey00(&mut self) -> Ghashsubkey00W<Ghashsubkey00Spec> {
        Ghashsubkey00W::new(self, 0)
    }
}
#[doc = "Bits 31:0 of GHASH Key0 (used as the GHASH module key).\n\nYou can [`read`](crate::Reg::read) this register and get [`ghashsubkey00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghashsubkey00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ghashsubkey00Spec;
impl crate::RegisterSpec for Ghashsubkey00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghashsubkey00::R`](R) reader structure"]
impl crate::Readable for Ghashsubkey00Spec {}
#[doc = "`write(|w| ..)` method takes [`ghashsubkey00::W`](W) writer structure"]
impl crate::Writable for Ghashsubkey00Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GHASHSUBKEY00 to value 0"]
impl crate::Resettable for Ghashsubkey00Spec {
    const RESET_VALUE: u32 = 0;
}
