#[doc = "Register `DSP0RESETVEC` reader"]
pub type R = crate::R<Dsp0resetvecSpec>;
#[doc = "Register `DSP0RESETVEC` writer"]
pub type W = crate::W<Dsp0resetvecSpec>;
#[doc = "Field `DSP0RESETVEC` reader - DSP 0 Reset Vector Address."]
pub type Dsp0resetvecR = crate::FieldReader<u32>;
#[doc = "Field `DSP0RESETVEC` writer - DSP 0 Reset Vector Address."]
pub type Dsp0resetvecW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP 0 Reset Vector Address."]
    #[inline(always)]
    pub fn dsp0resetvec(&self) -> Dsp0resetvecR {
        Dsp0resetvecR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP 0 Reset Vector Address."]
    #[inline(always)]
    #[must_use]
    pub fn dsp0resetvec(&mut self) -> Dsp0resetvecW<Dsp0resetvecSpec> {
        Dsp0resetvecW::new(self, 0)
    }
}
#[doc = "DSP 0 Reset Vector\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0resetvec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0resetvec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0resetvecSpec;
impl crate::RegisterSpec for Dsp0resetvecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0resetvec::R`](R) reader structure"]
impl crate::Readable for Dsp0resetvecSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0resetvec::W`](W) writer structure"]
impl crate::Writable for Dsp0resetvecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0RESETVEC to value 0"]
impl crate::Resettable for Dsp0resetvecSpec {
    const RESET_VALUE: u32 = 0;
}
