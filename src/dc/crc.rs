#[doc = "Register `CRC` reader"]
pub type R = crate::R<CrcSpec>;
#[doc = "Register `CRC` writer"]
pub type W = crate::W<CrcSpec>;
#[doc = "Field `CRCREG` reader - CRC value if CRC error exists"]
pub type CrcregR = crate::FieldReader<u32>;
#[doc = "Field `CRCREG` writer - CRC value if CRC error exists"]
pub type CrcregW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC value if CRC error exists"]
    #[inline(always)]
    pub fn crcreg(&self) -> CrcregR {
        CrcregR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC value if CRC error exists"]
    #[inline(always)]
    #[must_use]
    pub fn crcreg(&mut self) -> CrcregW<CrcSpec> {
        CrcregW::new(self, 0)
    }
}
#[doc = "if cyclic redundancy errors occur, they are written in the CRC register.\n\nYou can [`read`](crate::Reg::read) this register and get [`crc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcSpec;
impl crate::RegisterSpec for CrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc::R`](R) reader structure"]
impl crate::Readable for CrcSpec {}
#[doc = "`write(|w| ..)` method takes [`crc::W`](W) writer structure"]
impl crate::Writable for CrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC to value 0"]
impl crate::Resettable for CrcSpec {
    const RESET_VALUE: u32 = 0;
}
