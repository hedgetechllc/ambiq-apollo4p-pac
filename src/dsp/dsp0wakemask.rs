#[doc = "Register `DSP0WAKEMASK` reader"]
pub type R = crate::R<Dsp0wakemaskSpec>;
#[doc = "Register `DSP0WAKEMASK` writer"]
pub type W = crate::W<Dsp0wakemaskSpec>;
#[doc = "Field `DSP0WAKEMASK` reader - DSP 0 IRQ Wake Mask"]
pub type Dsp0wakemaskR = crate::FieldReader<u32>;
#[doc = "Field `DSP0WAKEMASK` writer - DSP 0 IRQ Wake Mask"]
pub type Dsp0wakemaskW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:22 - DSP 0 IRQ Wake Mask"]
    #[inline(always)]
    pub fn dsp0wakemask(&self) -> Dsp0wakemaskR {
        Dsp0wakemaskR::new(self.bits & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:22 - DSP 0 IRQ Wake Mask"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0wakemask(&mut self) -> Dsp0wakemaskW<Dsp0wakemaskSpec> {
        Dsp0wakemaskW::new(self, 0)
    }
}
#[doc = "DSP 0 IRQ Wake Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0wakemask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0wakemask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0wakemaskSpec;
impl crate::RegisterSpec for Dsp0wakemaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0wakemask::R`](R) reader structure"]
impl crate::Readable for Dsp0wakemaskSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0wakemask::W`](W) writer structure"]
impl crate::Writable for Dsp0wakemaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0WAKEMASK to value 0"]
impl crate::Resettable for Dsp0wakemaskSpec {
    const RESET_VALUE: u32 = 0;
}
