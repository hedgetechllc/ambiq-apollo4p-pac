#[doc = "Register `SATLIM` reader"]
pub type R = crate::R<SatlimSpec>;
#[doc = "Register `SATLIM` writer"]
pub type W = crate::W<SatlimSpec>;
#[doc = "Field `LSATC` reader - Sets the lower integer sample limit for the saturation comparator. Note that these values are raw ADC values whose bounds are specified by PRMODE but not manipulated by accumulate/divide logic. Therefore, there is no oversampling and no binary point in this value."]
pub type LsatcR = crate::FieldReader<u16>;
#[doc = "Field `LSATC` writer - Sets the lower integer sample limit for the saturation comparator. Note that these values are raw ADC values whose bounds are specified by PRMODE but not manipulated by accumulate/divide logic. Therefore, there is no oversampling and no binary point in this value."]
pub type LsatcW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `USATC` reader - Sets the upper integer sample limit for the saturation comparator. Note that these values are raw ADC values whose bounds are specified by PRMODE but not manipulated by accumulate/divide logic. Therefore, there is no oversampling and no binary point in this value."]
pub type UsatcR = crate::FieldReader<u16>;
#[doc = "Field `USATC` writer - Sets the upper integer sample limit for the saturation comparator. Note that these values are raw ADC values whose bounds are specified by PRMODE but not manipulated by accumulate/divide logic. Therefore, there is no oversampling and no binary point in this value."]
pub type UsatcW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Sets the lower integer sample limit for the saturation comparator. Note that these values are raw ADC values whose bounds are specified by PRMODE but not manipulated by accumulate/divide logic. Therefore, there is no oversampling and no binary point in this value."]
    #[inline(always)]
    pub fn lsatc(&self) -> LsatcR {
        LsatcR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Sets the upper integer sample limit for the saturation comparator. Note that these values are raw ADC values whose bounds are specified by PRMODE but not manipulated by accumulate/divide logic. Therefore, there is no oversampling and no binary point in this value."]
    #[inline(always)]
    pub fn usatc(&self) -> UsatcR {
        UsatcR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Sets the lower integer sample limit for the saturation comparator. Note that these values are raw ADC values whose bounds are specified by PRMODE but not manipulated by accumulate/divide logic. Therefore, there is no oversampling and no binary point in this value."]
    #[inline(always)]
    #[must_use]
    pub fn lsatc(&mut self) -> LsatcW<SatlimSpec> {
        LsatcW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Sets the upper integer sample limit for the saturation comparator. Note that these values are raw ADC values whose bounds are specified by PRMODE but not manipulated by accumulate/divide logic. Therefore, there is no oversampling and no binary point in this value."]
    #[inline(always)]
    #[must_use]
    pub fn usatc(&mut self) -> UsatcW<SatlimSpec> {
        UsatcW::new(self, 16)
    }
}
#[doc = "Saturation Comparator Limits\n\nYou can [`read`](crate::Reg::read) this register and get [`satlim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`satlim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SatlimSpec;
impl crate::RegisterSpec for SatlimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`satlim::R`](R) reader structure"]
impl crate::Readable for SatlimSpec {}
#[doc = "`write(|w| ..)` method takes [`satlim::W`](W) writer structure"]
impl crate::Writable for SatlimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SATLIM to value 0"]
impl crate::Resettable for SatlimSpec {
    const RESET_VALUE: u32 = 0;
}
