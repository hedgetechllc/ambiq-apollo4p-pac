#[doc = "Register `AESCMACSIZE0KICK` reader"]
pub type R = crate::R<Aescmacsize0kickSpec>;
#[doc = "Register `AESCMACSIZE0KICK` writer"]
pub type W = crate::W<Aescmacsize0kickSpec>;
#[doc = "Field `AESCMACSIZE0KICK` reader - writing to this address triggers the AES engine to perform a CMAC operation with size 0. The CMAC result can be read from the AES_IV0 register."]
pub type Aescmacsize0kickR = crate::BitReader;
#[doc = "Field `AESCMACSIZE0KICK` writer - writing to this address triggers the AES engine to perform a CMAC operation with size 0. The CMAC result can be read from the AES_IV0 register."]
pub type Aescmacsize0kickW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - writing to this address triggers the AES engine to perform a CMAC operation with size 0. The CMAC result can be read from the AES_IV0 register."]
    #[inline(always)]
    pub fn aescmacsize0kick(&self) -> Aescmacsize0kickR {
        Aescmacsize0kickR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - writing to this address triggers the AES engine to perform a CMAC operation with size 0. The CMAC result can be read from the AES_IV0 register."]
    #[inline(always)]
    #[must_use]
    pub fn aescmacsize0kick(&mut self) -> Aescmacsize0kickW<Aescmacsize0kickSpec> {
        Aescmacsize0kickW::new(self, 0)
    }
}
#[doc = "writing to this address triggers the AES engine to perform a CMAC operation with size 0. The CMAC result can be read from the AES_IV0 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`aescmacsize0kick::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aescmacsize0kick::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aescmacsize0kickSpec;
impl crate::RegisterSpec for Aescmacsize0kickSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aescmacsize0kick::R`](R) reader structure"]
impl crate::Readable for Aescmacsize0kickSpec {}
#[doc = "`write(|w| ..)` method takes [`aescmacsize0kick::W`](W) writer structure"]
impl crate::Writable for Aescmacsize0kickSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESCMACSIZE0KICK to value 0"]
impl crate::Resettable for Aescmacsize0kickSpec {
    const RESET_VALUE: u32 = 0;
}
