#[doc = "Register `AESKEY11` reader"]
pub type R = crate::R<Aeskey11Spec>;
#[doc = "Register `AESKEY11` writer"]
pub type W = crate::W<Aeskey11Spec>;
#[doc = "Field `AESKEY11` reader - bits 63:32 of AES Key1."]
pub type Aeskey11R = crate::FieldReader<u32>;
#[doc = "Field `AESKEY11` writer - bits 63:32 of AES Key1."]
pub type Aeskey11W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 63:32 of AES Key1."]
    #[inline(always)]
    pub fn aeskey11(&self) -> Aeskey11R {
        Aeskey11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 63:32 of AES Key1."]
    #[inline(always)]
    #[must_use]
    pub fn aeskey11(&mut self) -> Aeskey11W<Aeskey11Spec> {
        Aeskey11W::new(self, 0)
    }
}
#[doc = "bits 63:32 of AES Key1 (used as the second AES tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aeskey11Spec;
impl crate::RegisterSpec for Aeskey11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aeskey11::R`](R) reader structure"]
impl crate::Readable for Aeskey11Spec {}
#[doc = "`write(|w| ..)` method takes [`aeskey11::W`](W) writer structure"]
impl crate::Writable for Aeskey11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESKEY11 to value 0"]
impl crate::Resettable for Aeskey11Spec {
    const RESET_VALUE: u32 = 0;
}
