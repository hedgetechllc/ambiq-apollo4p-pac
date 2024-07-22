#[doc = "Register `DSP1WAKEMASK` reader"]
pub type R = crate::R<Dsp1wakemaskSpec>;
#[doc = "Register `DSP1WAKEMASK` writer"]
pub type W = crate::W<Dsp1wakemaskSpec>;
#[doc = "Field `DSP1WAKEMASK` reader - DSP 1 IRQ Wake Mask"]
pub type Dsp1wakemaskR = crate::FieldReader<u32>;
#[doc = "Field `DSP1WAKEMASK` writer - DSP 1 IRQ Wake Mask"]
pub type Dsp1wakemaskW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:22 - DSP 1 IRQ Wake Mask"]
    #[inline(always)]
    pub fn dsp1wakemask(&self) -> Dsp1wakemaskR {
        Dsp1wakemaskR::new(self.bits & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:22 - DSP 1 IRQ Wake Mask"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1wakemask(&mut self) -> Dsp1wakemaskW<Dsp1wakemaskSpec> {
        Dsp1wakemaskW::new(self, 0)
    }
}
#[doc = "DSP 1 IRQ Wake Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1wakemask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1wakemask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1wakemaskSpec;
impl crate::RegisterSpec for Dsp1wakemaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1wakemask::R`](R) reader structure"]
impl crate::Readable for Dsp1wakemaskSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1wakemask::W`](W) writer structure"]
impl crate::Writable for Dsp1wakemaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1WAKEMASK to value 0"]
impl crate::Resettable for Dsp1wakemaskSpec {
    const RESET_VALUE: u32 = 0;
}
