#[doc = "Register `AESIV01` reader"]
pub type R = crate::R<Aesiv01Spec>;
#[doc = "Register `AESIV01` writer"]
pub type W = crate::W<Aesiv01Spec>;
#[doc = "Field `AESIV01` reader - bits 63:32 of AES_IV0 register."]
pub type Aesiv01R = crate::FieldReader<u32>;
#[doc = "Field `AESIV01` writer - bits 63:32 of AES_IV0 register."]
pub type Aesiv01W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 63:32 of AES_IV0 register."]
    #[inline(always)]
    pub fn aesiv01(&self) -> Aesiv01R {
        Aesiv01R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 63:32 of AES_IV0 register."]
    #[inline(always)]
    #[must_use]
    pub fn aesiv01(&mut self) -> Aesiv01W<Aesiv01Spec> {
        Aesiv01W::new(self, 0)
    }
}
#[doc = "bits 63:32 of AES_IV0 128b register.For the description of AES_IV0, see the AES_IV_0_0 register description\n\nYou can [`read`](crate::Reg::read) this register and get [`aesiv01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesiv01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesiv01Spec;
impl crate::RegisterSpec for Aesiv01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesiv01::R`](R) reader structure"]
impl crate::Readable for Aesiv01Spec {}
#[doc = "`write(|w| ..)` method takes [`aesiv01::W`](W) writer structure"]
impl crate::Writable for Aesiv01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESIV01 to value 0"]
impl crate::Resettable for Aesiv01Spec {
    const RESET_VALUE: u32 = 0;
}
