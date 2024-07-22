#[doc = "Register `AESKEY10` reader"]
pub type R = crate::R<Aeskey10Spec>;
#[doc = "Register `AESKEY10` writer"]
pub type W = crate::W<Aeskey10Spec>;
#[doc = "Field `AESKEY10` reader - bits 31:0 of AES Key1."]
pub type Aeskey10R = crate::FieldReader<u32>;
#[doc = "Field `AESKEY10` writer - bits 31:0 of AES Key1."]
pub type Aeskey10W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 31:0 of AES Key1."]
    #[inline(always)]
    pub fn aeskey10(&self) -> Aeskey10R {
        Aeskey10R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 31:0 of AES Key1."]
    #[inline(always)]
    #[must_use]
    pub fn aeskey10(&mut self) -> Aeskey10W<Aeskey10Spec> {
        Aeskey10W::new(self, 0)
    }
}
#[doc = "bits 31:0 of AES Key1 (used as the second AES tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aeskey10Spec;
impl crate::RegisterSpec for Aeskey10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aeskey10::R`](R) reader structure"]
impl crate::Readable for Aeskey10Spec {}
#[doc = "`write(|w| ..)` method takes [`aeskey10::W`](W) writer structure"]
impl crate::Writable for Aeskey10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESKEY10 to value 0"]
impl crate::Resettable for Aeskey10Spec {
    const RESET_VALUE: u32 = 0;
}
