#[doc = "Register `AESKEY13` reader"]
pub type R = crate::R<Aeskey13Spec>;
#[doc = "Register `AESKEY13` writer"]
pub type W = crate::W<Aeskey13Spec>;
#[doc = "Field `AESKEY13` reader - bits 127:96 of AES Key1."]
pub type Aeskey13R = crate::FieldReader<u32>;
#[doc = "Field `AESKEY13` writer - bits 127:96 of AES Key1."]
pub type Aeskey13W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 127:96 of AES Key1."]
    #[inline(always)]
    pub fn aeskey13(&self) -> Aeskey13R {
        Aeskey13R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 127:96 of AES Key1."]
    #[inline(always)]
    #[must_use]
    pub fn aeskey13(&mut self) -> Aeskey13W<Aeskey13Spec> {
        Aeskey13W::new(self, 0)
    }
}
#[doc = "bits 127:96 of AES Key1 (used as the second AES tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aeskey13Spec;
impl crate::RegisterSpec for Aeskey13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aeskey13::R`](R) reader structure"]
impl crate::Readable for Aeskey13Spec {}
#[doc = "`write(|w| ..)` method takes [`aeskey13::W`](W) writer structure"]
impl crate::Writable for Aeskey13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESKEY13 to value 0"]
impl crate::Resettable for Aeskey13Spec {
    const RESET_VALUE: u32 = 0;
}
