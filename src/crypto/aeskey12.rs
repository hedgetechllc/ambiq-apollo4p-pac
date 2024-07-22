#[doc = "Register `AESKEY12` reader"]
pub type R = crate::R<Aeskey12Spec>;
#[doc = "Register `AESKEY12` writer"]
pub type W = crate::W<Aeskey12Spec>;
#[doc = "Field `AESKEY12` reader - bits 95:64 of AES Key1."]
pub type Aeskey12R = crate::FieldReader<u32>;
#[doc = "Field `AESKEY12` writer - bits 95:64 of AES Key1."]
pub type Aeskey12W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 95:64 of AES Key1."]
    #[inline(always)]
    pub fn aeskey12(&self) -> Aeskey12R {
        Aeskey12R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 95:64 of AES Key1."]
    #[inline(always)]
    #[must_use]
    pub fn aeskey12(&mut self) -> Aeskey12W<Aeskey12Spec> {
        Aeskey12W::new(self, 0)
    }
}
#[doc = "bits 95:64 of AES Key1 (used as the second AES tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aeskey12Spec;
impl crate::RegisterSpec for Aeskey12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aeskey12::R`](R) reader structure"]
impl crate::Readable for Aeskey12Spec {}
#[doc = "`write(|w| ..)` method takes [`aeskey12::W`](W) writer structure"]
impl crate::Writable for Aeskey12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESKEY12 to value 0"]
impl crate::Resettable for Aeskey12Spec {
    const RESET_VALUE: u32 = 0;
}
