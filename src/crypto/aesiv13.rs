#[doc = "Register `AESIV13` reader"]
pub type R = crate::R<Aesiv13Spec>;
#[doc = "Register `AESIV13` writer"]
pub type W = crate::W<Aesiv13Spec>;
#[doc = "Field `AESIV13` reader - bits 127:96 of AES_IV1 register."]
pub type Aesiv13R = crate::FieldReader<u32>;
#[doc = "Field `AESIV13` writer - bits 127:96 of AES_IV1 register."]
pub type Aesiv13W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 127:96 of AES_IV1 register."]
    #[inline(always)]
    pub fn aesiv13(&self) -> Aesiv13R {
        Aesiv13R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 127:96 of AES_IV1 register."]
    #[inline(always)]
    #[must_use]
    pub fn aesiv13(&mut self) -> Aesiv13W<Aesiv13Spec> {
        Aesiv13W::new(self, 0)
    }
}
#[doc = "bits 127:96 of AES_IV1 128b register.For the description of AES_IV1, see the AES_IV_1_0 register description\n\nYou can [`read`](crate::Reg::read) this register and get [`aesiv13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesiv13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesiv13Spec;
impl crate::RegisterSpec for Aesiv13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesiv13::R`](R) reader structure"]
impl crate::Readable for Aesiv13Spec {}
#[doc = "`write(|w| ..)` method takes [`aesiv13::W`](W) writer structure"]
impl crate::Writable for Aesiv13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESIV13 to value 0"]
impl crate::Resettable for Aesiv13Spec {
    const RESET_VALUE: u32 = 0;
}
