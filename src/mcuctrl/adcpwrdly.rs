#[doc = "Register `ADCPWRDLY` reader"]
pub type R = crate::R<AdcpwrdlySpec>;
#[doc = "Register `ADCPWRDLY` writer"]
pub type W = crate::W<AdcpwrdlySpec>;
#[doc = "Field `ADCPWR0` reader - Additional delay time for ADC Reference Buffer settling time in 64x ADC clock increments. Delay = 64*ADCPWR0*ADC_CLK."]
pub type Adcpwr0R = crate::FieldReader;
#[doc = "Field `ADCPWR0` writer - Additional delay time for ADC Reference Buffer settling time in 64x ADC clock increments. Delay = 64*ADCPWR0*ADC_CLK."]
pub type Adcpwr0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADCPWR1` reader - Delay time for ADC Reference Buffer settling time in 64x ADC clock increments. Delay = 64*ADCPWR1*ADC_CLK"]
pub type Adcpwr1R = crate::FieldReader;
#[doc = "Field `ADCPWR1` writer - Delay time for ADC Reference Buffer settling time in 64x ADC clock increments. Delay = 64*ADCPWR1*ADC_CLK"]
pub type Adcpwr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Additional delay time for ADC Reference Buffer settling time in 64x ADC clock increments. Delay = 64*ADCPWR0*ADC_CLK."]
    #[inline(always)]
    pub fn adcpwr0(&self) -> Adcpwr0R {
        Adcpwr0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Delay time for ADC Reference Buffer settling time in 64x ADC clock increments. Delay = 64*ADCPWR1*ADC_CLK"]
    #[inline(always)]
    pub fn adcpwr1(&self) -> Adcpwr1R {
        Adcpwr1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Additional delay time for ADC Reference Buffer settling time in 64x ADC clock increments. Delay = 64*ADCPWR0*ADC_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn adcpwr0(&mut self) -> Adcpwr0W<AdcpwrdlySpec> {
        Adcpwr0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Delay time for ADC Reference Buffer settling time in 64x ADC clock increments. Delay = 64*ADCPWR1*ADC_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn adcpwr1(&mut self) -> Adcpwr1W<AdcpwrdlySpec> {
        Adcpwr1W::new(self, 8)
    }
}
#[doc = "ADC Power Up Delay Control\n\nYou can [`read`](crate::Reg::read) this register and get [`adcpwrdly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcpwrdly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcpwrdlySpec;
impl crate::RegisterSpec for AdcpwrdlySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcpwrdly::R`](R) reader structure"]
impl crate::Readable for AdcpwrdlySpec {}
#[doc = "`write(|w| ..)` method takes [`adcpwrdly::W`](W) writer structure"]
impl crate::Writable for AdcpwrdlySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCPWRDLY to value 0x18a9"]
impl crate::Resettable for AdcpwrdlySpec {
    const RESET_VALUE: u32 = 0x18a9;
}
