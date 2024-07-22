#[doc = "Register `AESKEY15` reader"]
pub type R = crate::R<Aeskey15Spec>;
#[doc = "Register `AESKEY15` writer"]
pub type W = crate::W<Aeskey15Spec>;
#[doc = "Field `AESKEY15` reader - bits 191:160 of AES Key1."]
pub type Aeskey15R = crate::FieldReader<u32>;
#[doc = "Field `AESKEY15` writer - bits 191:160 of AES Key1."]
pub type Aeskey15W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 191:160 of AES Key1."]
    #[inline(always)]
    pub fn aeskey15(&self) -> Aeskey15R {
        Aeskey15R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 191:160 of AES Key1."]
    #[inline(always)]
    #[must_use]
    pub fn aeskey15(&mut self) -> Aeskey15W<Aeskey15Spec> {
        Aeskey15W::new(self, 0)
    }
}
#[doc = "bits 191:160 of AES Key1 (used as the second AES tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aeskey15Spec;
impl crate::RegisterSpec for Aeskey15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aeskey15::R`](R) reader structure"]
impl crate::Readable for Aeskey15Spec {}
#[doc = "`write(|w| ..)` method takes [`aeskey15::W`](W) writer structure"]
impl crate::Writable for Aeskey15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESKEY15 to value 0"]
impl crate::Resettable for Aeskey15Spec {
    const RESET_VALUE: u32 = 0;
}
