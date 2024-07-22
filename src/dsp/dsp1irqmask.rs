#[doc = "Register `DSP1IRQMASK` reader"]
pub type R = crate::R<Dsp1irqmaskSpec>;
#[doc = "Register `DSP1IRQMASK` writer"]
pub type W = crate::W<Dsp1irqmaskSpec>;
#[doc = "Field `DSP1IRQMASK` reader - DSP 1 IRQ Mask"]
pub type Dsp1irqmaskR = crate::FieldReader<u32>;
#[doc = "Field `DSP1IRQMASK` writer - DSP 1 IRQ Mask"]
pub type Dsp1irqmaskW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:22 - DSP 1 IRQ Mask"]
    #[inline(always)]
    pub fn dsp1irqmask(&self) -> Dsp1irqmaskR {
        Dsp1irqmaskR::new(self.bits & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:22 - DSP 1 IRQ Mask"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1irqmask(&mut self) -> Dsp1irqmaskW<Dsp1irqmaskSpec> {
        Dsp1irqmaskW::new(self, 0)
    }
}
#[doc = "DSP 1 IRQ Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1irqmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1irqmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1irqmaskSpec;
impl crate::RegisterSpec for Dsp1irqmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1irqmask::R`](R) reader structure"]
impl crate::Readable for Dsp1irqmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1irqmask::W`](W) writer structure"]
impl crate::Writable for Dsp1irqmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1IRQMASK to value 0"]
impl crate::Resettable for Dsp1irqmaskSpec {
    const RESET_VALUE: u32 = 0;
}
