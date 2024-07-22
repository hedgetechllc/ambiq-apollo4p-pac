#[doc = "Register `AESIV00` reader"]
pub type R = crate::R<Aesiv00Spec>;
#[doc = "Register `AESIV00` writer"]
pub type W = crate::W<Aesiv00Spec>;
#[doc = "Field `AESIV00` reader - bits 31:0 of AES_IV0 register."]
pub type Aesiv00R = crate::FieldReader<u32>;
#[doc = "Field `AESIV00` writer - bits 31:0 of AES_IV0 register."]
pub type Aesiv00W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 31:0 of AES_IV0 register."]
    #[inline(always)]
    pub fn aesiv00(&self) -> Aesiv00R {
        Aesiv00R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 31:0 of AES_IV0 register."]
    #[inline(always)]
    #[must_use]
    pub fn aesiv00(&mut self) -> Aesiv00W<Aesiv00Spec> {
        Aesiv00W::new(self, 0)
    }
}
#[doc = "bits 31:0 of AES_IV0 register. AES IV0 is used as the AES IV (Initialization Value) register in non-tunneling operations,and as the first tunnel stage IV register in tunneling operations.The IV register should be loaded according to the AES mode:in AES CBC_CBC-MAC - the AES IV register should be loaded with the IV (initialization vector).in XTS-AES - the AES IV register should be loaded with the T value (unless the HW T calculation mode is active, in which the T value is calculated by the HW).\n\nYou can [`read`](crate::Reg::read) this register and get [`aesiv00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesiv00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesiv00Spec;
impl crate::RegisterSpec for Aesiv00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesiv00::R`](R) reader structure"]
impl crate::Readable for Aesiv00Spec {}
#[doc = "`write(|w| ..)` method takes [`aesiv00::W`](W) writer structure"]
impl crate::Writable for Aesiv00Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESIV00 to value 0"]
impl crate::Resettable for Aesiv00Spec {
    const RESET_VALUE: u32 = 0;
}
