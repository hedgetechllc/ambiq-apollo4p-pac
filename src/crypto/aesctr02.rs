#[doc = "Register `AESCTR02` reader"]
pub type R = crate::R<Aesctr02Spec>;
#[doc = "Register `AESCTR02` writer"]
pub type W = crate::W<Aesctr02Spec>;
#[doc = "Field `AESCTR02` reader - bits 95:64 of AES_CTR0 register."]
pub type Aesctr02R = crate::FieldReader<u32>;
#[doc = "Field `AESCTR02` writer - bits 95:64 of AES_CTR0 register."]
pub type Aesctr02W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 95:64 of AES_CTR0 register."]
    #[inline(always)]
    pub fn aesctr02(&self) -> Aesctr02R {
        Aesctr02R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 95:64 of AES_CTR0 register."]
    #[inline(always)]
    #[must_use]
    pub fn aesctr02(&mut self) -> Aesctr02W<Aesctr02Spec> {
        Aesctr02W::new(self, 0)
    }
}
#[doc = "bits 95:64 of AES_CTR0 128b register.For the description of AES_CTR0, see the AES_CTR_0_0 register description.\n\nYou can [`read`](crate::Reg::read) this register and get [`aesctr02::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesctr02::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesctr02Spec;
impl crate::RegisterSpec for Aesctr02Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesctr02::R`](R) reader structure"]
impl crate::Readable for Aesctr02Spec {}
#[doc = "`write(|w| ..)` method takes [`aesctr02::W`](W) writer structure"]
impl crate::Writable for Aesctr02Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESCTR02 to value 0"]
impl crate::Resettable for Aesctr02Spec {
    const RESET_VALUE: u32 = 0;
}
