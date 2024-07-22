#[doc = "Register `AESIV03` reader"]
pub type R = crate::R<Aesiv03Spec>;
#[doc = "Register `AESIV03` writer"]
pub type W = crate::W<Aesiv03Spec>;
#[doc = "Field `AESIV03` reader - bits 127:96 of AES_IV0 register."]
pub type Aesiv03R = crate::FieldReader<u32>;
#[doc = "Field `AESIV03` writer - bits 127:96 of AES_IV0 register."]
pub type Aesiv03W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 127:96 of AES_IV0 register."]
    #[inline(always)]
    pub fn aesiv03(&self) -> Aesiv03R {
        Aesiv03R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 127:96 of AES_IV0 register."]
    #[inline(always)]
    #[must_use]
    pub fn aesiv03(&mut self) -> Aesiv03W<Aesiv03Spec> {
        Aesiv03W::new(self, 0)
    }
}
#[doc = "bits 127:96 of AES_IV0 128b register.For the description of AES_IV0, see the AES_IV_0_0 register description\n\nYou can [`read`](crate::Reg::read) this register and get [`aesiv03::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesiv03::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesiv03Spec;
impl crate::RegisterSpec for Aesiv03Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesiv03::R`](R) reader structure"]
impl crate::Readable for Aesiv03Spec {}
#[doc = "`write(|w| ..)` method takes [`aesiv03::W`](W) writer structure"]
impl crate::Writable for Aesiv03Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESIV03 to value 0"]
impl crate::Resettable for Aesiv03Spec {
    const RESET_VALUE: u32 = 0;
}
