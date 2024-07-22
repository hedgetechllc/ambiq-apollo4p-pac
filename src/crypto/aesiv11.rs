#[doc = "Register `AESIV11` reader"]
pub type R = crate::R<Aesiv11Spec>;
#[doc = "Register `AESIV11` writer"]
pub type W = crate::W<Aesiv11Spec>;
#[doc = "Field `AESIV11` reader - bits 63:32 of AES_IV1 register."]
pub type Aesiv11R = crate::FieldReader<u32>;
#[doc = "Field `AESIV11` writer - bits 63:32 of AES_IV1 register."]
pub type Aesiv11W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 63:32 of AES_IV1 register."]
    #[inline(always)]
    pub fn aesiv11(&self) -> Aesiv11R {
        Aesiv11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 63:32 of AES_IV1 register."]
    #[inline(always)]
    #[must_use]
    pub fn aesiv11(&mut self) -> Aesiv11W<Aesiv11Spec> {
        Aesiv11W::new(self, 0)
    }
}
#[doc = "bits 63:32 of AES_IV1 128b register.For the description of AES_IV1, see the AES_IV_1_0 register description\n\nYou can [`read`](crate::Reg::read) this register and get [`aesiv11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesiv11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesiv11Spec;
impl crate::RegisterSpec for Aesiv11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesiv11::R`](R) reader structure"]
impl crate::Readable for Aesiv11Spec {}
#[doc = "`write(|w| ..)` method takes [`aesiv11::W`](W) writer structure"]
impl crate::Writable for Aesiv11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESIV11 to value 0"]
impl crate::Resettable for Aesiv11Spec {
    const RESET_VALUE: u32 = 0;
}
