#[doc = "Register `RTCSTAT` reader"]
pub type R = crate::R<RtcstatSpec>;
#[doc = "Register `RTCSTAT` writer"]
pub type W = crate::W<RtcstatSpec>;
#[doc = "Field `WRITEBUSY` reader - Indicates that an RTC update (write) is still in progress. Writes are initiated by writing the CTTLOW register - CTRUP must be written before CTRLOW to be updated (otherwise it will retain its current value)"]
pub type WritebusyR = crate::BitReader;
#[doc = "Field `WRITEBUSY` writer - Indicates that an RTC update (write) is still in progress. Writes are initiated by writing the CTTLOW register - CTRUP must be written before CTRLOW to be updated (otherwise it will retain its current value)"]
pub type WritebusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates that an RTC update (write) is still in progress. Writes are initiated by writing the CTTLOW register - CTRUP must be written before CTRLOW to be updated (otherwise it will retain its current value)"]
    #[inline(always)]
    pub fn writebusy(&self) -> WritebusyR {
        WritebusyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates that an RTC update (write) is still in progress. Writes are initiated by writing the CTTLOW register - CTRUP must be written before CTRLOW to be updated (otherwise it will retain its current value)"]
    #[inline(always)]
    #[must_use]
    pub fn writebusy(&mut self) -> WritebusyW<RtcstatSpec> {
        WritebusyW::new(self, 0)
    }
}
#[doc = "This is the register status for the RTC module.\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcstatSpec;
impl crate::RegisterSpec for RtcstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtcstat::R`](R) reader structure"]
impl crate::Readable for RtcstatSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcstat::W`](W) writer structure"]
impl crate::Writable for RtcstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTCSTAT to value 0"]
impl crate::Resettable for RtcstatSpec {
    const RESET_VALUE: u32 = 0;
}
