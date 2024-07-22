#[doc = "Register `DSP0INTORMASK63TO32A` reader"]
pub type R = crate::R<Dsp0intormask63to32aSpec>;
#[doc = "Register `DSP0INTORMASK63TO32A` writer"]
pub type W = crate::W<Dsp0intormask63to32aSpec>;
#[doc = "Field `DSP0TMRORMASKA` reader - DSP0 Timer Interrupt OR Mask A"]
pub type Dsp0tmrormaskaR = crate::FieldReader<u16>;
#[doc = "Field `DSP0TMRORMASKA` writer - DSP0 Timer Interrupt OR Mask A"]
pub type Dsp0tmrormaskaW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DSP0I2SORMASKA` reader - DSP0 I2S Interrupt OR Mask A"]
pub type Dsp0i2sormaskaR = crate::FieldReader;
#[doc = "Field `DSP0I2SORMASKA` writer - DSP0 I2S Interrupt OR Mask A"]
pub type Dsp0i2sormaskaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DSP0PDMORMASKA` reader - DSP0 PDM Interrupt OR Mask A"]
pub type Dsp0pdmormaskaR = crate::FieldReader;
#[doc = "Field `DSP0PDMORMASKA` writer - DSP0 PDM Interrupt OR Mask A"]
pub type Dsp0pdmormaskaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DSP0GPIOORMASKA` reader - DSP0 GPIO Interrupt OR Mask A"]
pub type Dsp0gpioormaskaR = crate::FieldReader;
#[doc = "Field `DSP0GPIOORMASKA` writer - DSP0 GPIO Interrupt OR Mask A"]
pub type Dsp0gpioormaskaW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:9 - DSP0 Timer Interrupt OR Mask A"]
    #[inline(always)]
    pub fn dsp0tmrormaska(&self) -> Dsp0tmrormaskaR {
        Dsp0tmrormaskaR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:15 - DSP0 I2S Interrupt OR Mask A"]
    #[inline(always)]
    pub fn dsp0i2sormaska(&self) -> Dsp0i2sormaskaR {
        Dsp0i2sormaskaR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DSP0 PDM Interrupt OR Mask A"]
    #[inline(always)]
    pub fn dsp0pdmormaska(&self) -> Dsp0pdmormaskaR {
        Dsp0pdmormaskaR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:29 - DSP0 GPIO Interrupt OR Mask A"]
    #[inline(always)]
    pub fn dsp0gpioormaska(&self) -> Dsp0gpioormaskaR {
        Dsp0gpioormaskaR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - DSP0 Timer Interrupt OR Mask A"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0tmrormaska(&mut self) -> Dsp0tmrormaskaW<Dsp0intormask63to32aSpec> {
        Dsp0tmrormaskaW::new(self, 0)
    }
    #[doc = "Bits 12:15 - DSP0 I2S Interrupt OR Mask A"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0i2sormaska(&mut self) -> Dsp0i2sormaskaW<Dsp0intormask63to32aSpec> {
        Dsp0i2sormaskaW::new(self, 12)
    }
    #[doc = "Bits 16:19 - DSP0 PDM Interrupt OR Mask A"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0pdmormaska(&mut self) -> Dsp0pdmormaskaW<Dsp0intormask63to32aSpec> {
        Dsp0pdmormaskaW::new(self, 16)
    }
    #[doc = "Bits 24:29 - DSP0 GPIO Interrupt OR Mask A"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0gpioormaska(&mut self) -> Dsp0gpioormaskaW<Dsp0intormask63to32aSpec> {
        Dsp0gpioormaskaW::new(self, 24)
    }
}
#[doc = "DSP0 Interrupt OR Mask A for IRQ63-32\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0intormask63to32a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0intormask63to32a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0intormask63to32aSpec;
impl crate::RegisterSpec for Dsp0intormask63to32aSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0intormask63to32a::R`](R) reader structure"]
impl crate::Readable for Dsp0intormask63to32aSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0intormask63to32a::W`](W) writer structure"]
impl crate::Writable for Dsp0intormask63to32aSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0INTORMASK63TO32A to value 0"]
impl crate::Resettable for Dsp0intormask63to32aSpec {
    const RESET_VALUE: u32 = 0;
}
