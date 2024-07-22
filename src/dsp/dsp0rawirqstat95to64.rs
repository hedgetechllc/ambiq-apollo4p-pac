#[doc = "Register `DSP0RAWIRQSTAT95to64` reader"]
pub type R = crate::R<Dsp0rawirqstat95to64Spec>;
#[doc = "Register `DSP0RAWIRQSTAT95to64` writer"]
pub type W = crate::W<Dsp0rawirqstat95to64Spec>;
#[doc = "Field `DSP0RAWIRQSTAT95to64` reader - DSP 0 Raw IRQ95-64 Status"]
pub type Dsp0rawirqstat95to64R = crate::FieldReader<u32>;
#[doc = "Field `DSP0RAWIRQSTAT95to64` writer - DSP 0 Raw IRQ95-64 Status"]
pub type Dsp0rawirqstat95to64W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP 0 Raw IRQ95-64 Status"]
    #[inline(always)]
    pub fn dsp0rawirqstat95to64(&self) -> Dsp0rawirqstat95to64R {
        Dsp0rawirqstat95to64R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP 0 Raw IRQ95-64 Status"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0rawirqstat95to64(&mut self) -> Dsp0rawirqstat95to64W<Dsp0rawirqstat95to64Spec> {
        Dsp0rawirqstat95to64W::new(self, 0)
    }
}
#[doc = "DSP 0 Raw IRQ95-64 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0rawirqstat95to64::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0rawirqstat95to64::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0rawirqstat95to64Spec;
impl crate::RegisterSpec for Dsp0rawirqstat95to64Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0rawirqstat95to64::R`](R) reader structure"]
impl crate::Readable for Dsp0rawirqstat95to64Spec {}
#[doc = "`write(|w| ..)` method takes [`dsp0rawirqstat95to64::W`](W) writer structure"]
impl crate::Writable for Dsp0rawirqstat95to64Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0RAWIRQSTAT95to64 to value 0"]
impl crate::Resettable for Dsp0rawirqstat95to64Spec {
    const RESET_VALUE: u32 = 0;
}
