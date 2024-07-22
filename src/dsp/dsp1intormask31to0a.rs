#[doc = "Register `DSP1INTORMASK31TO0A` reader"]
pub type R = crate::R<Dsp1intormask31to0aSpec>;
#[doc = "Register `DSP1INTORMASK31TO0A` writer"]
pub type W = crate::W<Dsp1intormask31to0aSpec>;
#[doc = "Field `DSP1INTMCUIOORMASKA` reader - DSP1 MCU IO Interrupt OR Mask A"]
pub type Dsp1intmcuioormaskaR = crate::FieldReader<u32>;
#[doc = "Field `DSP1INTMCUIOORMASKA` writer - DSP1 MCU IO Interrupt OR Mask A"]
pub type Dsp1intmcuioormaskaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP1 MCU IO Interrupt OR Mask A"]
    #[inline(always)]
    pub fn dsp1intmcuioormaska(&self) -> Dsp1intmcuioormaskaR {
        Dsp1intmcuioormaskaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP1 MCU IO Interrupt OR Mask A"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1intmcuioormaska(&mut self) -> Dsp1intmcuioormaskaW<Dsp1intormask31to0aSpec> {
        Dsp1intmcuioormaskaW::new(self, 0)
    }
}
#[doc = "DSP1 Interrupt OR Mask A for IRQ31-0\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1intormask31to0a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1intormask31to0a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1intormask31to0aSpec;
impl crate::RegisterSpec for Dsp1intormask31to0aSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1intormask31to0a::R`](R) reader structure"]
impl crate::Readable for Dsp1intormask31to0aSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1intormask31to0a::W`](W) writer structure"]
impl crate::Writable for Dsp1intormask31to0aSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1INTORMASK31TO0A to value 0"]
impl crate::Resettable for Dsp1intormask31to0aSpec {
    const RESET_VALUE: u32 = 0;
}
