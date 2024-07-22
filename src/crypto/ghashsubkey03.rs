#[doc = "Register `GHASHSUBKEY03` reader"]
pub type R = crate::R<Ghashsubkey03Spec>;
#[doc = "Register `GHASHSUBKEY03` writer"]
pub type W = crate::W<Ghashsubkey03Spec>;
#[doc = "Field `GHASHSUBKEY03` reader - Bits 127:96 of GHASH Key0."]
pub type Ghashsubkey03R = crate::FieldReader<u32>;
#[doc = "Field `GHASHSUBKEY03` writer - Bits 127:96 of GHASH Key0."]
pub type Ghashsubkey03W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bits 127:96 of GHASH Key0."]
    #[inline(always)]
    pub fn ghashsubkey03(&self) -> Ghashsubkey03R {
        Ghashsubkey03R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits 127:96 of GHASH Key0."]
    #[inline(always)]
    #[must_use]
    pub fn ghashsubkey03(&mut self) -> Ghashsubkey03W<Ghashsubkey03Spec> {
        Ghashsubkey03W::new(self, 0)
    }
}
#[doc = "Bits 127:96 of GHASH Key0 (used as the GHASH module key).\n\nYou can [`read`](crate::Reg::read) this register and get [`ghashsubkey03::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghashsubkey03::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ghashsubkey03Spec;
impl crate::RegisterSpec for Ghashsubkey03Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghashsubkey03::R`](R) reader structure"]
impl crate::Readable for Ghashsubkey03Spec {}
#[doc = "`write(|w| ..)` method takes [`ghashsubkey03::W`](W) writer structure"]
impl crate::Writable for Ghashsubkey03Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GHASHSUBKEY03 to value 0"]
impl crate::Resettable for Ghashsubkey03Spec {
    const RESET_VALUE: u32 = 0;
}
