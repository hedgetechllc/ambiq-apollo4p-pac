#[doc = "Register `AESKEY04` reader"]
pub type R = crate::R<Aeskey04Spec>;
#[doc = "Register `AESKEY04` writer"]
pub type W = crate::W<Aeskey04Spec>;
#[doc = "Field `AESKEY04` reader - bits 159:128 of AES Key0 ."]
pub type Aeskey04R = crate::FieldReader<u32>;
#[doc = "Field `AESKEY04` writer - bits 159:128 of AES Key0 ."]
pub type Aeskey04W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 159:128 of AES Key0 ."]
    #[inline(always)]
    pub fn aeskey04(&self) -> Aeskey04R {
        Aeskey04R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 159:128 of AES Key0 ."]
    #[inline(always)]
    #[must_use]
    pub fn aeskey04(&mut self) -> Aeskey04W<Aeskey04Spec> {
        Aeskey04W::new(self, 0)
    }
}
#[doc = "bits 159:128 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aeskey04Spec;
impl crate::RegisterSpec for Aeskey04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aeskey04::R`](R) reader structure"]
impl crate::Readable for Aeskey04Spec {}
#[doc = "`write(|w| ..)` method takes [`aeskey04::W`](W) writer structure"]
impl crate::Writable for Aeskey04Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESKEY04 to value 0"]
impl crate::Resettable for Aeskey04Spec {
    const RESET_VALUE: u32 = 0;
}
