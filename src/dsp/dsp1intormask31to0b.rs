#[doc = "Register `DSP1INTORMASK31to0B` reader"]
pub type R = crate::R<Dsp1intormask31to0bSpec>;
#[doc = "Register `DSP1INTORMASK31to0B` writer"]
pub type W = crate::W<Dsp1intormask31to0bSpec>;
#[doc = "Field `DSP1INTMCUIOORMASKB` reader - DSP1 MCU IO Interrupt OR Mask B"]
pub type Dsp1intmcuioormaskbR = crate::FieldReader<u32>;
#[doc = "Field `DSP1INTMCUIOORMASKB` writer - DSP1 MCU IO Interrupt OR Mask B"]
pub type Dsp1intmcuioormaskbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP1 MCU IO Interrupt OR Mask B"]
    #[inline(always)]
    pub fn dsp1intmcuioormaskb(&self) -> Dsp1intmcuioormaskbR {
        Dsp1intmcuioormaskbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP1 MCU IO Interrupt OR Mask B"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1intmcuioormaskb(&mut self) -> Dsp1intmcuioormaskbW<Dsp1intormask31to0bSpec> {
        Dsp1intmcuioormaskbW::new(self, 0)
    }
}
#[doc = "DSP1 Interrupt OR Mask B for IRQ31-0\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1intormask31to0b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1intormask31to0b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1intormask31to0bSpec;
impl crate::RegisterSpec for Dsp1intormask31to0bSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1intormask31to0b::R`](R) reader structure"]
impl crate::Readable for Dsp1intormask31to0bSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1intormask31to0b::W`](W) writer structure"]
impl crate::Writable for Dsp1intormask31to0bSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1INTORMASK31to0B to value 0"]
impl crate::Resettable for Dsp1intormask31to0bSpec {
    const RESET_VALUE: u32 = 0;
}
