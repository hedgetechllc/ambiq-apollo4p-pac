#[doc = "Register `AUDADCPWRDLY` reader"]
pub type R = crate::R<AudadcpwrdlySpec>;
#[doc = "Register `AUDADCPWRDLY` writer"]
pub type W = crate::W<AudadcpwrdlySpec>;
#[doc = "Field `AUDADCPWR0` reader - ADC Reference Buffer Power Enable delay in 64 ADC CLK increments for ADC_CLKSEL = 0x1, 32 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
pub type Audadcpwr0R = crate::FieldReader;
#[doc = "Field `AUDADCPWR0` writer - ADC Reference Buffer Power Enable delay in 64 ADC CLK increments for ADC_CLKSEL = 0x1, 32 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
pub type Audadcpwr0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AUDADCPWR1` reader - ADC Reference Keeper enable delay in 16 ADC CLK increments for ADC_CLKSEL = 0x1, 8 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
pub type Audadcpwr1R = crate::FieldReader;
#[doc = "Field `AUDADCPWR1` writer - ADC Reference Keeper enable delay in 16 ADC CLK increments for ADC_CLKSEL = 0x1, 8 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
pub type Audadcpwr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ADC Reference Buffer Power Enable delay in 64 ADC CLK increments for ADC_CLKSEL = 0x1, 32 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
    #[inline(always)]
    pub fn audadcpwr0(&self) -> Audadcpwr0R {
        Audadcpwr0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ADC Reference Keeper enable delay in 16 ADC CLK increments for ADC_CLKSEL = 0x1, 8 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
    #[inline(always)]
    pub fn audadcpwr1(&self) -> Audadcpwr1R {
        Audadcpwr1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADC Reference Buffer Power Enable delay in 64 ADC CLK increments for ADC_CLKSEL = 0x1, 32 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
    #[inline(always)]
    #[must_use]
    pub fn audadcpwr0(&mut self) -> Audadcpwr0W<AudadcpwrdlySpec> {
        Audadcpwr0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - ADC Reference Keeper enable delay in 16 ADC CLK increments for ADC_CLKSEL = 0x1, 8 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
    #[inline(always)]
    #[must_use]
    pub fn audadcpwr1(&mut self) -> Audadcpwr1W<AudadcpwrdlySpec> {
        Audadcpwr1W::new(self, 8)
    }
}
#[doc = "Audio ADC Power Up Delay Control\n\nYou can [`read`](crate::Reg::read) this register and get [`audadcpwrdly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audadcpwrdly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudadcpwrdlySpec;
impl crate::RegisterSpec for AudadcpwrdlySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audadcpwrdly::R`](R) reader structure"]
impl crate::Readable for AudadcpwrdlySpec {}
#[doc = "`write(|w| ..)` method takes [`audadcpwrdly::W`](W) writer structure"]
impl crate::Writable for AudadcpwrdlySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUDADCPWRDLY to value 0x0200"]
impl crate::Resettable for AudadcpwrdlySpec {
    const RESET_VALUE: u32 = 0x0200;
}
