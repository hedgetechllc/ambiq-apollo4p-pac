#[doc = "Register `AESKEY14` reader"]
pub type R = crate::R<Aeskey14Spec>;
#[doc = "Register `AESKEY14` writer"]
pub type W = crate::W<Aeskey14Spec>;
#[doc = "Field `AESKEY14` reader - bits 159:128 of AES Key1."]
pub type Aeskey14R = crate::FieldReader<u32>;
#[doc = "Field `AESKEY14` writer - bits 159:128 of AES Key1."]
pub type Aeskey14W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 159:128 of AES Key1."]
    #[inline(always)]
    pub fn aeskey14(&self) -> Aeskey14R {
        Aeskey14R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 159:128 of AES Key1."]
    #[inline(always)]
    #[must_use]
    pub fn aeskey14(&mut self) -> Aeskey14W<Aeskey14Spec> {
        Aeskey14W::new(self, 0)
    }
}
#[doc = "bits 159:128 of AES Key1 (used as the second AES tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aeskey14Spec;
impl crate::RegisterSpec for Aeskey14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aeskey14::R`](R) reader structure"]
impl crate::Readable for Aeskey14Spec {}
#[doc = "`write(|w| ..)` method takes [`aeskey14::W`](W) writer structure"]
impl crate::Writable for Aeskey14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESKEY14 to value 0"]
impl crate::Resettable for Aeskey14Spec {
    const RESET_VALUE: u32 = 0;
}
