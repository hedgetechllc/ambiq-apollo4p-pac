#[doc = "Register `AESKEY17` reader"]
pub type R = crate::R<Aeskey17Spec>;
#[doc = "Register `AESKEY17` writer"]
pub type W = crate::W<Aeskey17Spec>;
#[doc = "Field `AESKEY17` reader - bits 255:224 of AES Key1."]
pub type Aeskey17R = crate::FieldReader<u32>;
#[doc = "Field `AESKEY17` writer - bits 255:224 of AES Key1."]
pub type Aeskey17W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 255:224 of AES Key1."]
    #[inline(always)]
    pub fn aeskey17(&self) -> Aeskey17R {
        Aeskey17R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 255:224 of AES Key1."]
    #[inline(always)]
    #[must_use]
    pub fn aeskey17(&mut self) -> Aeskey17W<Aeskey17Spec> {
        Aeskey17W::new(self, 0)
    }
}
#[doc = "bits 255:224 of AES Key1 (used as the second AES tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aeskey17Spec;
impl crate::RegisterSpec for Aeskey17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aeskey17::R`](R) reader structure"]
impl crate::Readable for Aeskey17Spec {}
#[doc = "`write(|w| ..)` method takes [`aeskey17::W`](W) writer structure"]
impl crate::Writable for Aeskey17Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESKEY17 to value 0"]
impl crate::Resettable for Aeskey17Spec {
    const RESET_VALUE: u32 = 0;
}
