#[doc = "Register `DSP0RAWIRQSTAT31to0` reader"]
pub type R = crate::R<Dsp0rawirqstat31to0Spec>;
#[doc = "Register `DSP0RAWIRQSTAT31to0` writer"]
pub type W = crate::W<Dsp0rawirqstat31to0Spec>;
#[doc = "Field `DSP0RAWIRQSTAT31to0` reader - DSP 0 Raw IRQ31-0 Status"]
pub type Dsp0rawirqstat31to0R = crate::FieldReader<u32>;
#[doc = "Field `DSP0RAWIRQSTAT31to0` writer - DSP 0 Raw IRQ31-0 Status"]
pub type Dsp0rawirqstat31to0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP 0 Raw IRQ31-0 Status"]
    #[inline(always)]
    pub fn dsp0rawirqstat31to0(&self) -> Dsp0rawirqstat31to0R {
        Dsp0rawirqstat31to0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP 0 Raw IRQ31-0 Status"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0rawirqstat31to0(&mut self) -> Dsp0rawirqstat31to0W<Dsp0rawirqstat31to0Spec> {
        Dsp0rawirqstat31to0W::new(self, 0)
    }
}
#[doc = "DSP 0 Raw IRQ31-0 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0rawirqstat31to0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0rawirqstat31to0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0rawirqstat31to0Spec;
impl crate::RegisterSpec for Dsp0rawirqstat31to0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0rawirqstat31to0::R`](R) reader structure"]
impl crate::Readable for Dsp0rawirqstat31to0Spec {}
#[doc = "`write(|w| ..)` method takes [`dsp0rawirqstat31to0::W`](W) writer structure"]
impl crate::Writable for Dsp0rawirqstat31to0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0RAWIRQSTAT31to0 to value 0"]
impl crate::Resettable for Dsp0rawirqstat31to0Spec {
    const RESET_VALUE: u32 = 0;
}
