#[doc = "Register `AESIV12` reader"]
pub type R = crate::R<Aesiv12Spec>;
#[doc = "Register `AESIV12` writer"]
pub type W = crate::W<Aesiv12Spec>;
#[doc = "Field `AESIV12` reader - bits 95:64 of AES_IV1 register."]
pub type Aesiv12R = crate::FieldReader<u32>;
#[doc = "Field `AESIV12` writer - bits 95:64 of AES_IV1 register."]
pub type Aesiv12W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 95:64 of AES_IV1 register."]
    #[inline(always)]
    pub fn aesiv12(&self) -> Aesiv12R {
        Aesiv12R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 95:64 of AES_IV1 register."]
    #[inline(always)]
    #[must_use]
    pub fn aesiv12(&mut self) -> Aesiv12W<Aesiv12Spec> {
        Aesiv12W::new(self, 0)
    }
}
#[doc = "bits 95:64 of AES_IV1 128b register.For the description of AES_IV1, see the AES_IV_1_0 register description\n\nYou can [`read`](crate::Reg::read) this register and get [`aesiv12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesiv12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesiv12Spec;
impl crate::RegisterSpec for Aesiv12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesiv12::R`](R) reader structure"]
impl crate::Readable for Aesiv12Spec {}
#[doc = "`write(|w| ..)` method takes [`aesiv12::W`](W) writer structure"]
impl crate::Writable for Aesiv12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESIV12 to value 0"]
impl crate::Resettable for Aesiv12Spec {
    const RESET_VALUE: u32 = 0;
}
