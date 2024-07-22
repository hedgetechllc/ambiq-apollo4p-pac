#[doc = "Register `AESKEY02` reader"]
pub type R = crate::R<Aeskey02Spec>;
#[doc = "Register `AESKEY02` writer"]
pub type W = crate::W<Aeskey02Spec>;
#[doc = "Field `AESKEY02` reader - bits 95:64 of AES Key0."]
pub type Aeskey02R = crate::FieldReader<u32>;
#[doc = "Field `AESKEY02` writer - bits 95:64 of AES Key0."]
pub type Aeskey02W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 95:64 of AES Key0."]
    #[inline(always)]
    pub fn aeskey02(&self) -> Aeskey02R {
        Aeskey02R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 95:64 of AES Key0."]
    #[inline(always)]
    #[must_use]
    pub fn aeskey02(&mut self) -> Aeskey02W<Aeskey02Spec> {
        Aeskey02W::new(self, 0)
    }
}
#[doc = "bits 95:64 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey02::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey02::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aeskey02Spec;
impl crate::RegisterSpec for Aeskey02Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aeskey02::R`](R) reader structure"]
impl crate::Readable for Aeskey02Spec {}
#[doc = "`write(|w| ..)` method takes [`aeskey02::W`](W) writer structure"]
impl crate::Writable for Aeskey02Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESKEY02 to value 0"]
impl crate::Resettable for Aeskey02Spec {
    const RESET_VALUE: u32 = 0;
}
