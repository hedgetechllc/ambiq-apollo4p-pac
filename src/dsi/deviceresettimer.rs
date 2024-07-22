#[doc = "Register `DEVICERESETTIMER` reader"]
pub type R = crate::R<DeviceresettimerSpec>;
#[doc = "Register `DEVICERESETTIMER` writer"]
pub type W = crate::W<DeviceresettimerSpec>;
#[doc = "Field `TIMOUT` reader - If the timer expires the DSI Host enters normal operation; This time out value is used while contention recovery procedure; the time out value is equal to a value longer than the specified time required to complete the reset sequence"]
pub type TimoutR = crate::FieldReader<u16>;
#[doc = "Field `TIMOUT` writer - If the timer expires the DSI Host enters normal operation; This time out value is used while contention recovery procedure; the time out value is equal to a value longer than the specified time required to complete the reset sequence"]
pub type TimoutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - If the timer expires the DSI Host enters normal operation; This time out value is used while contention recovery procedure; the time out value is equal to a value longer than the specified time required to complete the reset sequence"]
    #[inline(always)]
    pub fn timout(&self) -> TimoutR {
        TimoutR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - If the timer expires the DSI Host enters normal operation; This time out value is used while contention recovery procedure; the time out value is equal to a value longer than the specified time required to complete the reset sequence"]
    #[inline(always)]
    #[must_use]
    pub fn timout(&mut self) -> TimoutW<DeviceresettimerSpec> {
        TimoutW::new(self, 0)
    }
}
#[doc = "Timeout value to be checked for device to be reset after issuing reset entry command\n\nYou can [`read`](crate::Reg::read) this register and get [`deviceresettimer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deviceresettimer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeviceresettimerSpec;
impl crate::RegisterSpec for DeviceresettimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deviceresettimer::R`](R) reader structure"]
impl crate::Readable for DeviceresettimerSpec {}
#[doc = "`write(|w| ..)` method takes [`deviceresettimer::W`](W) writer structure"]
impl crate::Writable for DeviceresettimerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVICERESETTIMER to value 0xff"]
impl crate::Resettable for DeviceresettimerSpec {
    const RESET_VALUE: u32 = 0xff;
}
