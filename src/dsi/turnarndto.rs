#[doc = "Register `TURNARNDTO` reader"]
pub type R = crate::R<TurnarndtoSpec>;
#[doc = "Register `TURNARNDTO` writer"]
pub type W = crate::W<TurnarndtoSpec>;
#[doc = "Field `TIMOUT` reader - If the counter expires, processor is interrupted with Turn_around_ack timeout interrupt; this specified period shall be longer then the maximum possible turnaround delay for the unit to which the turnaround request was sent, which is 23 clock cycles of txclkesc; any number greater than or equal to 23 is an acceptable number."]
pub type TimoutR = crate::FieldReader;
#[doc = "Field `TIMOUT` writer - If the counter expires, processor is interrupted with Turn_around_ack timeout interrupt; this specified period shall be longer then the maximum possible turnaround delay for the unit to which the turnaround request was sent, which is 23 clock cycles of txclkesc; any number greater than or equal to 23 is an acceptable number."]
pub type TimoutW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - If the counter expires, processor is interrupted with Turn_around_ack timeout interrupt; this specified period shall be longer then the maximum possible turnaround delay for the unit to which the turnaround request was sent, which is 23 clock cycles of txclkesc; any number greater than or equal to 23 is an acceptable number."]
    #[inline(always)]
    pub fn timout(&self) -> TimoutR {
        TimoutR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - If the counter expires, processor is interrupted with Turn_around_ack timeout interrupt; this specified period shall be longer then the maximum possible turnaround delay for the unit to which the turnaround request was sent, which is 23 clock cycles of txclkesc; any number greater than or equal to 23 is an acceptable number."]
    #[inline(always)]
    #[must_use]
    pub fn timout(&mut self) -> TimoutW<TurnarndtoSpec> {
        TimoutW::new(self, 0)
    }
}
#[doc = "Timeout value to be checked after the DSI host makes a trun around in the direction of transfers.\n\nYou can [`read`](crate::Reg::read) this register and get [`turnarndto::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`turnarndto::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TurnarndtoSpec;
impl crate::RegisterSpec for TurnarndtoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`turnarndto::R`](R) reader structure"]
impl crate::Readable for TurnarndtoSpec {}
#[doc = "`write(|w| ..)` method takes [`turnarndto::W`](W) writer structure"]
impl crate::Writable for TurnarndtoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TURNARNDTO to value 0x17"]
impl crate::Resettable for TurnarndtoSpec {
    const RESET_VALUE: u32 = 0x17;
}
