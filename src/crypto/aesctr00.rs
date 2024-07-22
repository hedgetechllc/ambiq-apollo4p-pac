#[doc = "Register `AESCTR00` reader"]
pub type R = crate::R<Aesctr00Spec>;
#[doc = "Register `AESCTR00` writer"]
pub type W = crate::W<Aesctr00Spec>;
#[doc = "Field `AESCTR00` reader - bits 31:0 of AES_CTR0 register."]
pub type Aesctr00R = crate::FieldReader<u32>;
#[doc = "Field `AESCTR00` writer - bits 31:0 of AES_CTR0 register."]
pub type Aesctr00W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 31:0 of AES_CTR0 register."]
    #[inline(always)]
    pub fn aesctr00(&self) -> Aesctr00R {
        Aesctr00R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 31:0 of AES_CTR0 register."]
    #[inline(always)]
    #[must_use]
    pub fn aesctr00(&mut self) -> Aesctr00W<Aesctr00Spec> {
        Aesctr00W::new(self, 0)
    }
}
#[doc = "bits 31:0 of AES_CTR0 128b register.AES CTR0 is used as the AES CTR (counter) register in non-tunneling operations, and as the first tunnel stage CTR register in tunneling operations.The CTR register should be loaded according to the AES mode:in AES CTR_GCTR - the AES CTR register should be loaded with the counter value.in XTS-AES - the AES CTR register should be loaded with the i value (in order to calculate the T value from it, if HW T calculation is supported).\n\nYou can [`read`](crate::Reg::read) this register and get [`aesctr00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesctr00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesctr00Spec;
impl crate::RegisterSpec for Aesctr00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesctr00::R`](R) reader structure"]
impl crate::Readable for Aesctr00Spec {}
#[doc = "`write(|w| ..)` method takes [`aesctr00::W`](W) writer structure"]
impl crate::Writable for Aesctr00Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESCTR00 to value 0"]
impl crate::Resettable for Aesctr00Spec {
    const RESET_VALUE: u32 = 0;
}
