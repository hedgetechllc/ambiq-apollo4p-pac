#[doc = "Register `DSP1INTORMASK63TO32B` reader"]
pub type R = crate::R<Dsp1intormask63to32bSpec>;
#[doc = "Register `DSP1INTORMASK63TO32B` writer"]
pub type W = crate::W<Dsp1intormask63to32bSpec>;
#[doc = "Field `DSP1TMRORMASKB` reader - DSP1 Timer Interrupt OR Mask B"]
pub type Dsp1tmrormaskbR = crate::FieldReader<u16>;
#[doc = "Field `DSP1TMRORMASKB` writer - DSP1 Timer Interrupt OR Mask B"]
pub type Dsp1tmrormaskbW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DSP1I2SORMASKB` reader - DSP1 I2S Interrupt OR Mask B"]
pub type Dsp1i2sormaskbR = crate::FieldReader;
#[doc = "Field `DSP1I2SORMASKB` writer - DSP1 I2S Interrupt OR Mask B"]
pub type Dsp1i2sormaskbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DSP1PDMORMASKB` reader - DSP1 PDM Interrupt OR Mask B"]
pub type Dsp1pdmormaskbR = crate::FieldReader;
#[doc = "Field `DSP1PDMORMASKB` writer - DSP1 PDM Interrupt OR Mask B"]
pub type Dsp1pdmormaskbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DSP1GPIOORMASKB` reader - DSP1 GPIO Interrupt OR Mask B"]
pub type Dsp1gpioormaskbR = crate::FieldReader;
#[doc = "Field `DSP1GPIOORMASKB` writer - DSP1 GPIO Interrupt OR Mask B"]
pub type Dsp1gpioormaskbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:9 - DSP1 Timer Interrupt OR Mask B"]
    #[inline(always)]
    pub fn dsp1tmrormaskb(&self) -> Dsp1tmrormaskbR {
        Dsp1tmrormaskbR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:15 - DSP1 I2S Interrupt OR Mask B"]
    #[inline(always)]
    pub fn dsp1i2sormaskb(&self) -> Dsp1i2sormaskbR {
        Dsp1i2sormaskbR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DSP1 PDM Interrupt OR Mask B"]
    #[inline(always)]
    pub fn dsp1pdmormaskb(&self) -> Dsp1pdmormaskbR {
        Dsp1pdmormaskbR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:29 - DSP1 GPIO Interrupt OR Mask B"]
    #[inline(always)]
    pub fn dsp1gpioormaskb(&self) -> Dsp1gpioormaskbR {
        Dsp1gpioormaskbR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - DSP1 Timer Interrupt OR Mask B"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1tmrormaskb(&mut self) -> Dsp1tmrormaskbW<Dsp1intormask63to32bSpec> {
        Dsp1tmrormaskbW::new(self, 0)
    }
    #[doc = "Bits 12:15 - DSP1 I2S Interrupt OR Mask B"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1i2sormaskb(&mut self) -> Dsp1i2sormaskbW<Dsp1intormask63to32bSpec> {
        Dsp1i2sormaskbW::new(self, 12)
    }
    #[doc = "Bits 16:19 - DSP1 PDM Interrupt OR Mask B"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1pdmormaskb(&mut self) -> Dsp1pdmormaskbW<Dsp1intormask63to32bSpec> {
        Dsp1pdmormaskbW::new(self, 16)
    }
    #[doc = "Bits 24:29 - DSP1 GPIO Interrupt OR Mask B"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1gpioormaskb(&mut self) -> Dsp1gpioormaskbW<Dsp1intormask63to32bSpec> {
        Dsp1gpioormaskbW::new(self, 24)
    }
}
#[doc = "DSP1 Interrupt OR Mask A for IRQ63-32\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1intormask63to32b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1intormask63to32b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1intormask63to32bSpec;
impl crate::RegisterSpec for Dsp1intormask63to32bSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1intormask63to32b::R`](R) reader structure"]
impl crate::Readable for Dsp1intormask63to32bSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1intormask63to32b::W`](W) writer structure"]
impl crate::Writable for Dsp1intormask63to32bSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1INTORMASK63TO32B to value 0"]
impl crate::Resettable for Dsp1intormask63to32bSpec {
    const RESET_VALUE: u32 = 0;
}
