#[doc = "Register `OTAPOINTER` reader"]
pub type R = crate::R<OtapointerSpec>;
#[doc = "Register `OTAPOINTER` writer"]
pub type W = crate::W<OtapointerSpec>;
#[doc = "Field `OTAVALID` reader - Indicates that an OTA update is valid"]
pub type OtavalidR = crate::BitReader;
#[doc = "Field `OTAVALID` writer - Indicates that an OTA update is valid"]
pub type OtavalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTASBLUPDATE` reader - Indicates that the sbl_init has been updated"]
pub type OtasblupdateR = crate::BitReader;
#[doc = "Field `OTASBLUPDATE` writer - Indicates that the sbl_init has been updated"]
pub type OtasblupdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTAPOINTER` reader - Flash page pointer with updated OTA image"]
pub type OtapointerR = crate::FieldReader<u32>;
#[doc = "Field `OTAPOINTER` writer - Flash page pointer with updated OTA image"]
pub type OtapointerW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - Indicates that an OTA update is valid"]
    #[inline(always)]
    pub fn otavalid(&self) -> OtavalidR {
        OtavalidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates that the sbl_init has been updated"]
    #[inline(always)]
    pub fn otasblupdate(&self) -> OtasblupdateR {
        OtasblupdateR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - Flash page pointer with updated OTA image"]
    #[inline(always)]
    pub fn otapointer(&self) -> OtapointerR {
        OtapointerR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates that an OTA update is valid"]
    #[inline(always)]
    #[must_use]
    pub fn otavalid(&mut self) -> OtavalidW<OtapointerSpec> {
        OtavalidW::new(self, 0)
    }
    #[doc = "Bit 1 - Indicates that the sbl_init has been updated"]
    #[inline(always)]
    #[must_use]
    pub fn otasblupdate(&mut self) -> OtasblupdateW<OtapointerSpec> {
        OtasblupdateW::new(self, 1)
    }
    #[doc = "Bits 2:31 - Flash page pointer with updated OTA image"]
    #[inline(always)]
    #[must_use]
    pub fn otapointer(&mut self) -> OtapointerW<OtapointerSpec> {
        OtapointerW::new(self, 2)
    }
}
#[doc = "OTA (Over the Air) Update Pointer/Status. Reset only by POA\n\nYou can [`read`](crate::Reg::read) this register and get [`otapointer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otapointer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtapointerSpec;
impl crate::RegisterSpec for OtapointerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otapointer::R`](R) reader structure"]
impl crate::Readable for OtapointerSpec {}
#[doc = "`write(|w| ..)` method takes [`otapointer::W`](W) writer structure"]
impl crate::Writable for OtapointerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTAPOINTER to value 0"]
impl crate::Resettable for OtapointerSpec {
    const RESET_VALUE: u32 = 0;
}
