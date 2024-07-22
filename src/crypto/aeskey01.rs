#[doc = "Register `AESKEY01` reader"]
pub type R = crate::R<Aeskey01Spec>;
#[doc = "Register `AESKEY01` writer"]
pub type W = crate::W<Aeskey01Spec>;
#[doc = "Field `AESKEY01` reader - bits 63:32 of AES Key0."]
pub type Aeskey01R = crate::FieldReader<u32>;
#[doc = "Field `AESKEY01` writer - bits 63:32 of AES Key0."]
pub type Aeskey01W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 63:32 of AES Key0."]
    #[inline(always)]
    pub fn aeskey01(&self) -> Aeskey01R {
        Aeskey01R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 63:32 of AES Key0."]
    #[inline(always)]
    #[must_use]
    pub fn aeskey01(&mut self) -> Aeskey01W<Aeskey01Spec> {
        Aeskey01W::new(self, 0)
    }
}
#[doc = "bits 63:32 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aeskey01Spec;
impl crate::RegisterSpec for Aeskey01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aeskey01::R`](R) reader structure"]
impl crate::Readable for Aeskey01Spec {}
#[doc = "`write(|w| ..)` method takes [`aeskey01::W`](W) writer structure"]
impl crate::Writable for Aeskey01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESKEY01 to value 0"]
impl crate::Resettable for Aeskey01Spec {
    const RESET_VALUE: u32 = 0;
}
