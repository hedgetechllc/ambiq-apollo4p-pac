#[doc = "Register `AESKEY00` reader"]
pub type R = crate::R<Aeskey00Spec>;
#[doc = "Register `AESKEY00` writer"]
pub type W = crate::W<Aeskey00Spec>;
#[doc = "Field `AESKEY00` reader - bits 31:0 of AES Key0."]
pub type Aeskey00R = crate::FieldReader<u32>;
#[doc = "Field `AESKEY00` writer - bits 31:0 of AES Key0."]
pub type Aeskey00W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 31:0 of AES Key0."]
    #[inline(always)]
    pub fn aeskey00(&self) -> Aeskey00R {
        Aeskey00R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 31:0 of AES Key0."]
    #[inline(always)]
    #[must_use]
    pub fn aeskey00(&mut self) -> Aeskey00W<Aeskey00Spec> {
        Aeskey00W::new(self, 0)
    }
}
#[doc = "bits 31:0 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aeskey00Spec;
impl crate::RegisterSpec for Aeskey00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aeskey00::R`](R) reader structure"]
impl crate::Readable for Aeskey00Spec {}
#[doc = "`write(|w| ..)` method takes [`aeskey00::W`](W) writer structure"]
impl crate::Writable for Aeskey00Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESKEY00 to value 0"]
impl crate::Resettable for Aeskey00Spec {
    const RESET_VALUE: u32 = 0;
}
