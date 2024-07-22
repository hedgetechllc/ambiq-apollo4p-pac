#[doc = "Register `DSP1INTORMASK63TO32A` reader"]
pub type R = crate::R<Dsp1intormask63to32aSpec>;
#[doc = "Register `DSP1INTORMASK63TO32A` writer"]
pub type W = crate::W<Dsp1intormask63to32aSpec>;
#[doc = "Field `DSP1TMRORMASKA` reader - DSP1 Timer Interrupt OR Mask A"]
pub type Dsp1tmrormaskaR = crate::FieldReader<u16>;
#[doc = "Field `DSP1TMRORMASKA` writer - DSP1 Timer Interrupt OR Mask A"]
pub type Dsp1tmrormaskaW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DSP1I2SORMASKA` reader - DSP1 I2S Interrupt OR Mask A"]
pub type Dsp1i2sormaskaR = crate::FieldReader;
#[doc = "Field `DSP1I2SORMASKA` writer - DSP1 I2S Interrupt OR Mask A"]
pub type Dsp1i2sormaskaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DSP1PDMORMASKA` reader - DSP1 PDM Interrupt OR Mask A"]
pub type Dsp1pdmormaskaR = crate::FieldReader;
#[doc = "Field `DSP1PDMORMASKA` writer - DSP1 PDM Interrupt OR Mask A"]
pub type Dsp1pdmormaskaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DSP1GPIOORMASKA` reader - DSP1 GPIO Interrupt OR Mask A"]
pub type Dsp1gpioormaskaR = crate::FieldReader;
#[doc = "Field `DSP1GPIOORMASKA` writer - DSP1 GPIO Interrupt OR Mask A"]
pub type Dsp1gpioormaskaW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:9 - DSP1 Timer Interrupt OR Mask A"]
    #[inline(always)]
    pub fn dsp1tmrormaska(&self) -> Dsp1tmrormaskaR {
        Dsp1tmrormaskaR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:15 - DSP1 I2S Interrupt OR Mask A"]
    #[inline(always)]
    pub fn dsp1i2sormaska(&self) -> Dsp1i2sormaskaR {
        Dsp1i2sormaskaR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DSP1 PDM Interrupt OR Mask A"]
    #[inline(always)]
    pub fn dsp1pdmormaska(&self) -> Dsp1pdmormaskaR {
        Dsp1pdmormaskaR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:29 - DSP1 GPIO Interrupt OR Mask A"]
    #[inline(always)]
    pub fn dsp1gpioormaska(&self) -> Dsp1gpioormaskaR {
        Dsp1gpioormaskaR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - DSP1 Timer Interrupt OR Mask A"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1tmrormaska(&mut self) -> Dsp1tmrormaskaW<Dsp1intormask63to32aSpec> {
        Dsp1tmrormaskaW::new(self, 0)
    }
    #[doc = "Bits 12:15 - DSP1 I2S Interrupt OR Mask A"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1i2sormaska(&mut self) -> Dsp1i2sormaskaW<Dsp1intormask63to32aSpec> {
        Dsp1i2sormaskaW::new(self, 12)
    }
    #[doc = "Bits 16:19 - DSP1 PDM Interrupt OR Mask A"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1pdmormaska(&mut self) -> Dsp1pdmormaskaW<Dsp1intormask63to32aSpec> {
        Dsp1pdmormaskaW::new(self, 16)
    }
    #[doc = "Bits 24:29 - DSP1 GPIO Interrupt OR Mask A"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1gpioormaska(&mut self) -> Dsp1gpioormaskaW<Dsp1intormask63to32aSpec> {
        Dsp1gpioormaskaW::new(self, 24)
    }
}
#[doc = "DSP1 Interrupt OR Mask A for IRQ63-32\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1intormask63to32a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1intormask63to32a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1intormask63to32aSpec;
impl crate::RegisterSpec for Dsp1intormask63to32aSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1intormask63to32a::R`](R) reader structure"]
impl crate::Readable for Dsp1intormask63to32aSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1intormask63to32a::W`](W) writer structure"]
impl crate::Writable for Dsp1intormask63to32aSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1INTORMASK63TO32A to value 0"]
impl crate::Resettable for Dsp1intormask63to32aSpec {
    const RESET_VALUE: u32 = 0;
}
