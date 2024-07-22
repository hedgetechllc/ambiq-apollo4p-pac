#[doc = "Register `AESCTR03` reader"]
pub type R = crate::R<Aesctr03Spec>;
#[doc = "Register `AESCTR03` writer"]
pub type W = crate::W<Aesctr03Spec>;
#[doc = "Field `AESCTR03` reader - bits 127:96 of AES_CTR0 register."]
pub type Aesctr03R = crate::FieldReader<u32>;
#[doc = "Field `AESCTR03` writer - bits 127:96 of AES_CTR0 register."]
pub type Aesctr03W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 127:96 of AES_CTR0 register."]
    #[inline(always)]
    pub fn aesctr03(&self) -> Aesctr03R {
        Aesctr03R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 127:96 of AES_CTR0 register."]
    #[inline(always)]
    #[must_use]
    pub fn aesctr03(&mut self) -> Aesctr03W<Aesctr03Spec> {
        Aesctr03W::new(self, 0)
    }
}
#[doc = "bits 127:96 of AES_CTR0 128b register.For the description of AES_CTR0, see the AES_CTR_0_0 register description.\n\nYou can [`read`](crate::Reg::read) this register and get [`aesctr03::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesctr03::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesctr03Spec;
impl crate::RegisterSpec for Aesctr03Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesctr03::R`](R) reader structure"]
impl crate::Readable for Aesctr03Spec {}
#[doc = "`write(|w| ..)` method takes [`aesctr03::W`](W) writer structure"]
impl crate::Writable for Aesctr03Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESCTR03 to value 0"]
impl crate::Resettable for Aesctr03Spec {
    const RESET_VALUE: u32 = 0;
}
