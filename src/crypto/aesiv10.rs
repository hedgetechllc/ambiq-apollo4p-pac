#[doc = "Register `AESIV10` reader"]
pub type R = crate::R<Aesiv10Spec>;
#[doc = "Register `AESIV10` writer"]
pub type W = crate::W<Aesiv10Spec>;
#[doc = "Field `AESIV10` reader - bits 31:0 of AES_IV1 register."]
pub type Aesiv10R = crate::FieldReader<u32>;
#[doc = "Field `AESIV10` writer - bits 31:0 of AES_IV1 register."]
pub type Aesiv10W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 31:0 of AES_IV1 register."]
    #[inline(always)]
    pub fn aesiv10(&self) -> Aesiv10R {
        Aesiv10R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 31:0 of AES_IV1 register."]
    #[inline(always)]
    #[must_use]
    pub fn aesiv10(&mut self) -> Aesiv10W<Aesiv10Spec> {
        Aesiv10W::new(self, 0)
    }
}
#[doc = "bits 31:0 of AES_IV1 128b register.AES IV1 is used as the AES IV (Initialization Value) register as the second tunnel stage IV register in tunneling operations.The IV register should be loaded according to the AES mode:in AES CBC_CBC-MAC - the AES IV register should be loaded with the IV (initialization vector).in XTS-AES - the AES IV register should be loaded with the T value (unless the HW T calculation mode is active, in which the T value is calculated by the HW.\n\nYou can [`read`](crate::Reg::read) this register and get [`aesiv10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesiv10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesiv10Spec;
impl crate::RegisterSpec for Aesiv10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesiv10::R`](R) reader structure"]
impl crate::Readable for Aesiv10Spec {}
#[doc = "`write(|w| ..)` method takes [`aesiv10::W`](W) writer structure"]
impl crate::Writable for Aesiv10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESIV10 to value 0"]
impl crate::Resettable for Aesiv10Spec {
    const RESET_VALUE: u32 = 0;
}
