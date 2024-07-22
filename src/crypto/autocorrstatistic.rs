#[doc = "Register `AUTOCORRSTATISTIC` reader"]
pub type R = crate::R<AutocorrstatisticSpec>;
#[doc = "Register `AUTOCORRSTATISTIC` writer"]
pub type W = crate::W<AutocorrstatisticSpec>;
#[doc = "Field `AUTOCORRTRYS` reader - Count each time an autocorrelation test starts. Any write to the register resets the counter. Stops collecting statistics if one of the counters has reached the limit."]
pub type AutocorrtrysR = crate::FieldReader<u16>;
#[doc = "Field `AUTOCORRTRYS` writer - Count each time an autocorrelation test starts. Any write to the register resets the counter. Stops collecting statistics if one of the counters has reached the limit."]
pub type AutocorrtrysW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `AUTOCORRFAILS` reader - Count each time an autocorrelation test fails. Any write to the register resets the counter. Stops collecting statistics if one of the counters has reached the limit."]
pub type AutocorrfailsR = crate::FieldReader;
#[doc = "Field `AUTOCORRFAILS` writer - Count each time an autocorrelation test fails. Any write to the register resets the counter. Stops collecting statistics if one of the counters has reached the limit."]
pub type AutocorrfailsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:13 - Count each time an autocorrelation test starts. Any write to the register resets the counter. Stops collecting statistics if one of the counters has reached the limit."]
    #[inline(always)]
    pub fn autocorrtrys(&self) -> AutocorrtrysR {
        AutocorrtrysR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:21 - Count each time an autocorrelation test fails. Any write to the register resets the counter. Stops collecting statistics if one of the counters has reached the limit."]
    #[inline(always)]
    pub fn autocorrfails(&self) -> AutocorrfailsR {
        AutocorrfailsR::new(((self.bits >> 14) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - Count each time an autocorrelation test starts. Any write to the register resets the counter. Stops collecting statistics if one of the counters has reached the limit."]
    #[inline(always)]
    #[must_use]
    pub fn autocorrtrys(&mut self) -> AutocorrtrysW<AutocorrstatisticSpec> {
        AutocorrtrysW::new(self, 0)
    }
    #[doc = "Bits 14:21 - Count each time an autocorrelation test fails. Any write to the register resets the counter. Stops collecting statistics if one of the counters has reached the limit."]
    #[inline(always)]
    #[must_use]
    pub fn autocorrfails(&mut self) -> AutocorrfailsW<AutocorrstatisticSpec> {
        AutocorrfailsW::new(self, 14)
    }
}
#[doc = "Statistics about autocorrelation test activations.\n\nYou can [`read`](crate::Reg::read) this register and get [`autocorrstatistic::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocorrstatistic::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AutocorrstatisticSpec;
impl crate::RegisterSpec for AutocorrstatisticSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autocorrstatistic::R`](R) reader structure"]
impl crate::Readable for AutocorrstatisticSpec {}
#[doc = "`write(|w| ..)` method takes [`autocorrstatistic::W`](W) writer structure"]
impl crate::Writable for AutocorrstatisticSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUTOCORRSTATISTIC to value 0"]
impl crate::Resettable for AutocorrstatisticSpec {
    const RESET_VALUE: u32 = 0;
}
