#[doc = "Register `DSP0INTORMASK31TO0A` reader"]
pub type R = crate::R<Dsp0intormask31to0aSpec>;
#[doc = "Register `DSP0INTORMASK31TO0A` writer"]
pub type W = crate::W<Dsp0intormask31to0aSpec>;
#[doc = "Field `DSP0INTMCUIOORMASKA` reader - DSP0 MCU IO Interrupt OR Mask A"]
pub type Dsp0intmcuioormaskaR = crate::FieldReader<u32>;
#[doc = "Field `DSP0INTMCUIOORMASKA` writer - DSP0 MCU IO Interrupt OR Mask A"]
pub type Dsp0intmcuioormaskaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP0 MCU IO Interrupt OR Mask A"]
    #[inline(always)]
    pub fn dsp0intmcuioormaska(&self) -> Dsp0intmcuioormaskaR {
        Dsp0intmcuioormaskaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP0 MCU IO Interrupt OR Mask A"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0intmcuioormaska(&mut self) -> Dsp0intmcuioormaskaW<Dsp0intormask31to0aSpec> {
        Dsp0intmcuioormaskaW::new(self, 0)
    }
}
#[doc = "DSP0 Interrupt OR Mask A for IRQ31-0\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0intormask31to0a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0intormask31to0a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0intormask31to0aSpec;
impl crate::RegisterSpec for Dsp0intormask31to0aSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0intormask31to0a::R`](R) reader structure"]
impl crate::Readable for Dsp0intormask31to0aSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0intormask31to0a::W`](W) writer structure"]
impl crate::Writable for Dsp0intormask31to0aSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0INTORMASK31TO0A to value 0"]
impl crate::Resettable for Dsp0intormask31to0aSpec {
    const RESET_VALUE: u32 = 0;
}
