#[doc = "Register `AESKEY05` reader"]
pub type R = crate::R<Aeskey05Spec>;
#[doc = "Register `AESKEY05` writer"]
pub type W = crate::W<Aeskey05Spec>;
#[doc = "Field `AESKEY05` reader - bits 191:160 of AES Key0."]
pub type Aeskey05R = crate::FieldReader<u32>;
#[doc = "Field `AESKEY05` writer - bits 191:160 of AES Key0."]
pub type Aeskey05W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 191:160 of AES Key0."]
    #[inline(always)]
    pub fn aeskey05(&self) -> Aeskey05R {
        Aeskey05R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 191:160 of AES Key0."]
    #[inline(always)]
    #[must_use]
    pub fn aeskey05(&mut self) -> Aeskey05W<Aeskey05Spec> {
        Aeskey05W::new(self, 0)
    }
}
#[doc = "bits 191:160 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey05::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey05::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aeskey05Spec;
impl crate::RegisterSpec for Aeskey05Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aeskey05::R`](R) reader structure"]
impl crate::Readable for Aeskey05Spec {}
#[doc = "`write(|w| ..)` method takes [`aeskey05::W`](W) writer structure"]
impl crate::Writable for Aeskey05Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESKEY05 to value 0"]
impl crate::Resettable for Aeskey05Spec {
    const RESET_VALUE: u32 = 0;
}