#[doc = "Register `AESKEY06` reader"]
pub type R = crate::R<Aeskey06Spec>;
#[doc = "Register `AESKEY06` writer"]
pub type W = crate::W<Aeskey06Spec>;
#[doc = "Field `AESKEY06` reader - bits 223:192 of AES Key0."]
pub type Aeskey06R = crate::FieldReader<u32>;
#[doc = "Field `AESKEY06` writer - bits 223:192 of AES Key0."]
pub type Aeskey06W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 223:192 of AES Key0."]
    #[inline(always)]
    pub fn aeskey06(&self) -> Aeskey06R {
        Aeskey06R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 223:192 of AES Key0."]
    #[inline(always)]
    #[must_use]
    pub fn aeskey06(&mut self) -> Aeskey06W<Aeskey06Spec> {
        Aeskey06W::new(self, 0)
    }
}
#[doc = "bits 223:192 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey06::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey06::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aeskey06Spec;
impl crate::RegisterSpec for Aeskey06Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aeskey06::R`](R) reader structure"]
impl crate::Readable for Aeskey06Spec {}
#[doc = "`write(|w| ..)` method takes [`aeskey06::W`](W) writer structure"]
impl crate::Writable for Aeskey06Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESKEY06 to value 0"]
impl crate::Resettable for Aeskey06Spec {
    const RESET_VALUE: u32 = 0;
}
