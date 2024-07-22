#[doc = "Register `RESULT` reader"]
pub type R = crate::R<ResultSpec>;
#[doc = "Register `RESULT` writer"]
pub type W = crate::W<ResultSpec>;
#[doc = "Field `CRC` reader - CRC Seed/Result. Software must seed the CRC with 0xFFFFFFFF before starting a CRC operation (unless the CRC is continued from a previous operation)."]
pub type CrcR = crate::FieldReader<u32>;
#[doc = "Field `CRC` writer - CRC Seed/Result. Software must seed the CRC with 0xFFFFFFFF before starting a CRC operation (unless the CRC is continued from a previous operation)."]
pub type CrcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC Seed/Result. Software must seed the CRC with 0xFFFFFFFF before starting a CRC operation (unless the CRC is continued from a previous operation)."]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Seed/Result. Software must seed the CRC with 0xFFFFFFFF before starting a CRC operation (unless the CRC is continued from a previous operation)."]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CrcW<ResultSpec> {
        CrcW::new(self, 0)
    }
}
#[doc = "CRC Seed/Result\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`result::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResultSpec;
impl crate::RegisterSpec for ResultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result::R`](R) reader structure"]
impl crate::Readable for ResultSpec {}
#[doc = "`write(|w| ..)` method takes [`result::W`](W) writer structure"]
impl crate::Writable for ResultSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESULT to value 0"]
impl crate::Resettable for ResultSpec {
    const RESET_VALUE: u32 = 0;
}
