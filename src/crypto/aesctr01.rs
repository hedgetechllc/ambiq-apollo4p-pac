#[doc = "Register `AESCTR01` reader"]
pub type R = crate::R<Aesctr01Spec>;
#[doc = "Register `AESCTR01` writer"]
pub type W = crate::W<Aesctr01Spec>;
#[doc = "Field `AESCTR01` reader - bits 63:32 of AES_CTR0 register."]
pub type Aesctr01R = crate::FieldReader<u32>;
#[doc = "Field `AESCTR01` writer - bits 63:32 of AES_CTR0 register."]
pub type Aesctr01W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 63:32 of AES_CTR0 register."]
    #[inline(always)]
    pub fn aesctr01(&self) -> Aesctr01R {
        Aesctr01R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 63:32 of AES_CTR0 register."]
    #[inline(always)]
    #[must_use]
    pub fn aesctr01(&mut self) -> Aesctr01W<Aesctr01Spec> {
        Aesctr01W::new(self, 0)
    }
}
#[doc = "bits 63:32 of AES_CTR0 128b register.For the description of AES_CTR0, see the AES_CTR_0_0 register description.\n\nYou can [`read`](crate::Reg::read) this register and get [`aesctr01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesctr01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesctr01Spec;
impl crate::RegisterSpec for Aesctr01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesctr01::R`](R) reader structure"]
impl crate::Readable for Aesctr01Spec {}
#[doc = "`write(|w| ..)` method takes [`aesctr01::W`](W) writer structure"]
impl crate::Writable for Aesctr01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESCTR01 to value 0"]
impl crate::Resettable for Aesctr01Spec {
    const RESET_VALUE: u32 = 0;
}
