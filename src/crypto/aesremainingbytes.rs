#[doc = "Register `AESREMAININGBYTES` reader"]
pub type R = crate::R<AesremainingbytesSpec>;
#[doc = "Register `AESREMAININGBYTES` writer"]
pub type W = crate::W<AesremainingbytesSpec>;
#[doc = "Field `AESREMAININGBYTES` reader - This register should be set with the amount of remaining bytes until the end of the current AES operation. The AES engine counts down from this value to determine the last _ one before last blocks in AES CMAC, XTS AES and AES CCM."]
pub type AesremainingbytesR = crate::FieldReader<u32>;
#[doc = "Field `AESREMAININGBYTES` writer - This register should be set with the amount of remaining bytes until the end of the current AES operation. The AES engine counts down from this value to determine the last _ one before last blocks in AES CMAC, XTS AES and AES CCM."]
pub type AesremainingbytesW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register should be set with the amount of remaining bytes until the end of the current AES operation. The AES engine counts down from this value to determine the last _ one before last blocks in AES CMAC, XTS AES and AES CCM."]
    #[inline(always)]
    pub fn aesremainingbytes(&self) -> AesremainingbytesR {
        AesremainingbytesR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register should be set with the amount of remaining bytes until the end of the current AES operation. The AES engine counts down from this value to determine the last _ one before last blocks in AES CMAC, XTS AES and AES CCM."]
    #[inline(always)]
    #[must_use]
    pub fn aesremainingbytes(&mut self) -> AesremainingbytesW<AesremainingbytesSpec> {
        AesremainingbytesW::new(self, 0)
    }
}
#[doc = "This register should be set with the amount of remaining bytes until the end of the current AES operation. The AES engine counts down from this value to determine the last _ one before last blocks in AES CMAC, XTS AES and AES CCM.\n\nYou can [`read`](crate::Reg::read) this register and get [`aesremainingbytes::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesremainingbytes::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesremainingbytesSpec;
impl crate::RegisterSpec for AesremainingbytesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesremainingbytes::R`](R) reader structure"]
impl crate::Readable for AesremainingbytesSpec {}
#[doc = "`write(|w| ..)` method takes [`aesremainingbytes::W`](W) writer structure"]
impl crate::Writable for AesremainingbytesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESREMAININGBYTES to value 0"]
impl crate::Resettable for AesremainingbytesSpec {
    const RESET_VALUE: u32 = 0;
}
