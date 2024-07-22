#[doc = "Register `AESKEY16` reader"]
pub type R = crate::R<Aeskey16Spec>;
#[doc = "Register `AESKEY16` writer"]
pub type W = crate::W<Aeskey16Spec>;
#[doc = "Field `AESKEY16` reader - bits 223:192 of AES Key1."]
pub type Aeskey16R = crate::FieldReader<u32>;
#[doc = "Field `AESKEY16` writer - bits 223:192 of AES Key1."]
pub type Aeskey16W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 223:192 of AES Key1."]
    #[inline(always)]
    pub fn aeskey16(&self) -> Aeskey16R {
        Aeskey16R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 223:192 of AES Key1."]
    #[inline(always)]
    #[must_use]
    pub fn aeskey16(&mut self) -> Aeskey16W<Aeskey16Spec> {
        Aeskey16W::new(self, 0)
    }
}
#[doc = "bits 223:192 of AES Key1 (used as the second AES tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aeskey16Spec;
impl crate::RegisterSpec for Aeskey16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aeskey16::R`](R) reader structure"]
impl crate::Readable for Aeskey16Spec {}
#[doc = "`write(|w| ..)` method takes [`aeskey16::W`](W) writer structure"]
impl crate::Writable for Aeskey16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESKEY16 to value 0"]
impl crate::Resettable for Aeskey16Spec {
    const RESET_VALUE: u32 = 0;
}
