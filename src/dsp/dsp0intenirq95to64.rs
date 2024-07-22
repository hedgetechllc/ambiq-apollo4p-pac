#[doc = "Register `DSP0INTENIRQ95TO64` reader"]
pub type R = crate::R<Dsp0intenirq95to64Spec>;
#[doc = "Register `DSP0INTENIRQ95TO64` writer"]
pub type W = crate::W<Dsp0intenirq95to64Spec>;
#[doc = "Field `DSP0INTENIRQ95TO64` reader - DSP0 INT Enable for IRQ95-64"]
pub type Dsp0intenirq95to64R = crate::FieldReader<u32>;
#[doc = "Field `DSP0INTENIRQ95TO64` writer - DSP0 INT Enable for IRQ95-64"]
pub type Dsp0intenirq95to64W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP0 INT Enable for IRQ95-64"]
    #[inline(always)]
    pub fn dsp0intenirq95to64(&self) -> Dsp0intenirq95to64R {
        Dsp0intenirq95to64R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP0 INT Enable for IRQ95-64"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0intenirq95to64(&mut self) -> Dsp0intenirq95to64W<Dsp0intenirq95to64Spec> {
        Dsp0intenirq95to64W::new(self, 0)
    }
}
#[doc = "DSP0 INT Enable for IRQ95-64\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0intenirq95to64::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0intenirq95to64::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0intenirq95to64Spec;
impl crate::RegisterSpec for Dsp0intenirq95to64Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0intenirq95to64::R`](R) reader structure"]
impl crate::Readable for Dsp0intenirq95to64Spec {}
#[doc = "`write(|w| ..)` method takes [`dsp0intenirq95to64::W`](W) writer structure"]
impl crate::Writable for Dsp0intenirq95to64Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0INTENIRQ95TO64 to value 0"]
impl crate::Resettable for Dsp0intenirq95to64Spec {
    const RESET_VALUE: u32 = 0;
}
