#[doc = "Register `DSP0INTENIRQ63TO32` reader"]
pub type R = crate::R<Dsp0intenirq63to32Spec>;
#[doc = "Register `DSP0INTENIRQ63TO32` writer"]
pub type W = crate::W<Dsp0intenirq63to32Spec>;
#[doc = "Field `DSP0INTENIRQ63TO32` reader - DSP0 INT Enable for IRQ63-32"]
pub type Dsp0intenirq63to32R = crate::FieldReader<u32>;
#[doc = "Field `DSP0INTENIRQ63TO32` writer - DSP0 INT Enable for IRQ63-32"]
pub type Dsp0intenirq63to32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP0 INT Enable for IRQ63-32"]
    #[inline(always)]
    pub fn dsp0intenirq63to32(&self) -> Dsp0intenirq63to32R {
        Dsp0intenirq63to32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP0 INT Enable for IRQ63-32"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0intenirq63to32(&mut self) -> Dsp0intenirq63to32W<Dsp0intenirq63to32Spec> {
        Dsp0intenirq63to32W::new(self, 0)
    }
}
#[doc = "DSP0 INT Enable for IRQ63-32\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0intenirq63to32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0intenirq63to32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0intenirq63to32Spec;
impl crate::RegisterSpec for Dsp0intenirq63to32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0intenirq63to32::R`](R) reader structure"]
impl crate::Readable for Dsp0intenirq63to32Spec {}
#[doc = "`write(|w| ..)` method takes [`dsp0intenirq63to32::W`](W) writer structure"]
impl crate::Writable for Dsp0intenirq63to32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0INTENIRQ63TO32 to value 0"]
impl crate::Resettable for Dsp0intenirq63to32Spec {
    const RESET_VALUE: u32 = 0;
}
