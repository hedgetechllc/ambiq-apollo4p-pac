#[doc = "Register `DSP0IRQMASK` reader"]
pub type R = crate::R<Dsp0irqmaskSpec>;
#[doc = "Register `DSP0IRQMASK` writer"]
pub type W = crate::W<Dsp0irqmaskSpec>;
#[doc = "Field `DSP0IRQMASK` reader - DSP 0 IRQ Mask"]
pub type Dsp0irqmaskR = crate::FieldReader<u32>;
#[doc = "Field `DSP0IRQMASK` writer - DSP 0 IRQ Mask"]
pub type Dsp0irqmaskW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:22 - DSP 0 IRQ Mask"]
    #[inline(always)]
    pub fn dsp0irqmask(&self) -> Dsp0irqmaskR {
        Dsp0irqmaskR::new(self.bits & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:22 - DSP 0 IRQ Mask"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0irqmask(&mut self) -> Dsp0irqmaskW<Dsp0irqmaskSpec> {
        Dsp0irqmaskW::new(self, 0)
    }
}
#[doc = "DSP 0 IRQ Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0irqmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0irqmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0irqmaskSpec;
impl crate::RegisterSpec for Dsp0irqmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0irqmask::R`](R) reader structure"]
impl crate::Readable for Dsp0irqmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0irqmask::W`](W) writer structure"]
impl crate::Writable for Dsp0irqmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0IRQMASK to value 0"]
impl crate::Resettable for Dsp0irqmaskSpec {
    const RESET_VALUE: u32 = 0;
}
