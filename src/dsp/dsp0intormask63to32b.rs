#[doc = "Register `DSP0INTORMASK63TO32B` reader"]
pub type R = crate::R<Dsp0intormask63to32bSpec>;
#[doc = "Register `DSP0INTORMASK63TO32B` writer"]
pub type W = crate::W<Dsp0intormask63to32bSpec>;
#[doc = "Field `DSP0TMRORMASKB` reader - DSP0 Timer Interrupt OR Mask B"]
pub type Dsp0tmrormaskbR = crate::FieldReader<u16>;
#[doc = "Field `DSP0TMRORMASKB` writer - DSP0 Timer Interrupt OR Mask B"]
pub type Dsp0tmrormaskbW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DSP0I2SORMASKB` reader - DSP0 I2S Interrupt OR Mask B"]
pub type Dsp0i2sormaskbR = crate::FieldReader;
#[doc = "Field `DSP0I2SORMASKB` writer - DSP0 I2S Interrupt OR Mask B"]
pub type Dsp0i2sormaskbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DSP0PDMORMASKB` reader - DSP0 PDM Interrupt OR Mask B"]
pub type Dsp0pdmormaskbR = crate::FieldReader;
#[doc = "Field `DSP0PDMORMASKB` writer - DSP0 PDM Interrupt OR Mask B"]
pub type Dsp0pdmormaskbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DSP0GPIOORMASKB` reader - DSP0 GPIO Interrupt OR Mask B"]
pub type Dsp0gpioormaskbR = crate::FieldReader;
#[doc = "Field `DSP0GPIOORMASKB` writer - DSP0 GPIO Interrupt OR Mask B"]
pub type Dsp0gpioormaskbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:9 - DSP0 Timer Interrupt OR Mask B"]
    #[inline(always)]
    pub fn dsp0tmrormaskb(&self) -> Dsp0tmrormaskbR {
        Dsp0tmrormaskbR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:15 - DSP0 I2S Interrupt OR Mask B"]
    #[inline(always)]
    pub fn dsp0i2sormaskb(&self) -> Dsp0i2sormaskbR {
        Dsp0i2sormaskbR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DSP0 PDM Interrupt OR Mask B"]
    #[inline(always)]
    pub fn dsp0pdmormaskb(&self) -> Dsp0pdmormaskbR {
        Dsp0pdmormaskbR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:29 - DSP0 GPIO Interrupt OR Mask B"]
    #[inline(always)]
    pub fn dsp0gpioormaskb(&self) -> Dsp0gpioormaskbR {
        Dsp0gpioormaskbR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - DSP0 Timer Interrupt OR Mask B"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0tmrormaskb(&mut self) -> Dsp0tmrormaskbW<Dsp0intormask63to32bSpec> {
        Dsp0tmrormaskbW::new(self, 0)
    }
    #[doc = "Bits 12:15 - DSP0 I2S Interrupt OR Mask B"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0i2sormaskb(&mut self) -> Dsp0i2sormaskbW<Dsp0intormask63to32bSpec> {
        Dsp0i2sormaskbW::new(self, 12)
    }
    #[doc = "Bits 16:19 - DSP0 PDM Interrupt OR Mask B"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0pdmormaskb(&mut self) -> Dsp0pdmormaskbW<Dsp0intormask63to32bSpec> {
        Dsp0pdmormaskbW::new(self, 16)
    }
    #[doc = "Bits 24:29 - DSP0 GPIO Interrupt OR Mask B"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0gpioormaskb(&mut self) -> Dsp0gpioormaskbW<Dsp0intormask63to32bSpec> {
        Dsp0gpioormaskbW::new(self, 24)
    }
}
#[doc = "DSP0 Interrupt OR Mask A for IRQ63-32\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0intormask63to32b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0intormask63to32b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0intormask63to32bSpec;
impl crate::RegisterSpec for Dsp0intormask63to32bSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0intormask63to32b::R`](R) reader structure"]
impl crate::Readable for Dsp0intormask63to32bSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0intormask63to32b::W`](W) writer structure"]
impl crate::Writable for Dsp0intormask63to32bSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0INTORMASK63TO32B to value 0"]
impl crate::Resettable for Dsp0intormask63to32bSpec {
    const RESET_VALUE: u32 = 0;
}
