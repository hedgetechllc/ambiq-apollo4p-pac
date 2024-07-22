#[doc = "Register `DSP0INTENIRQ31TO0` reader"]
pub type R = crate::R<Dsp0intenirq31to0Spec>;
#[doc = "Register `DSP0INTENIRQ31TO0` writer"]
pub type W = crate::W<Dsp0intenirq31to0Spec>;
#[doc = "Field `DSP0INTENIRQ31TO0` reader - DSP0 INT Enable for IRQ31-0"]
pub type Dsp0intenirq31to0R = crate::FieldReader<u32>;
#[doc = "Field `DSP0INTENIRQ31TO0` writer - DSP0 INT Enable for IRQ31-0"]
pub type Dsp0intenirq31to0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP0 INT Enable for IRQ31-0"]
    #[inline(always)]
    pub fn dsp0intenirq31to0(&self) -> Dsp0intenirq31to0R {
        Dsp0intenirq31to0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP0 INT Enable for IRQ31-0"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0intenirq31to0(&mut self) -> Dsp0intenirq31to0W<Dsp0intenirq31to0Spec> {
        Dsp0intenirq31to0W::new(self, 0)
    }
}
#[doc = "DSP0 INT Enable for IRQ31-0\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0intenirq31to0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0intenirq31to0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0intenirq31to0Spec;
impl crate::RegisterSpec for Dsp0intenirq31to0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0intenirq31to0::R`](R) reader structure"]
impl crate::Readable for Dsp0intenirq31to0Spec {}
#[doc = "`write(|w| ..)` method takes [`dsp0intenirq31to0::W`](W) writer structure"]
impl crate::Writable for Dsp0intenirq31to0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0INTENIRQ31TO0 to value 0"]
impl crate::Resettable for Dsp0intenirq31to0Spec {
    const RESET_VALUE: u32 = 0;
}
