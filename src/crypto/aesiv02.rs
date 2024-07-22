#[doc = "Register `AESIV02` reader"]
pub type R = crate::R<Aesiv02Spec>;
#[doc = "Register `AESIV02` writer"]
pub type W = crate::W<Aesiv02Spec>;
#[doc = "Field `AESIV02` reader - bits 95:64 of AES_IV0 register."]
pub type Aesiv02R = crate::FieldReader<u32>;
#[doc = "Field `AESIV02` writer - bits 95:64 of AES_IV0 register."]
pub type Aesiv02W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 95:64 of AES_IV0 register."]
    #[inline(always)]
    pub fn aesiv02(&self) -> Aesiv02R {
        Aesiv02R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 95:64 of AES_IV0 register."]
    #[inline(always)]
    #[must_use]
    pub fn aesiv02(&mut self) -> Aesiv02W<Aesiv02Spec> {
        Aesiv02W::new(self, 0)
    }
}
#[doc = "bits 95:64 of AES_IV0 128b register.For the description of AES_IV0, see the AES_IV_0_0 register description\n\nYou can [`read`](crate::Reg::read) this register and get [`aesiv02::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesiv02::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesiv02Spec;
impl crate::RegisterSpec for Aesiv02Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesiv02::R`](R) reader structure"]
impl crate::Readable for Aesiv02Spec {}
#[doc = "`write(|w| ..)` method takes [`aesiv02::W`](W) writer structure"]
impl crate::Writable for Aesiv02Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESIV02 to value 0"]
impl crate::Resettable for Aesiv02Spec {
    const RESET_VALUE: u32 = 0;
}
