#[doc = "Register `DSP1RESETVEC` reader"]
pub type R = crate::R<Dsp1resetvecSpec>;
#[doc = "Register `DSP1RESETVEC` writer"]
pub type W = crate::W<Dsp1resetvecSpec>;
#[doc = "Field `DSP1RESETVEC` reader - DSP 1 Reset Vector Address."]
pub type Dsp1resetvecR = crate::FieldReader<u32>;
#[doc = "Field `DSP1RESETVEC` writer - DSP 1 Reset Vector Address."]
pub type Dsp1resetvecW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP 1 Reset Vector Address."]
    #[inline(always)]
    pub fn dsp1resetvec(&self) -> Dsp1resetvecR {
        Dsp1resetvecR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP 1 Reset Vector Address."]
    #[inline(always)]
    #[must_use]
    pub fn dsp1resetvec(&mut self) -> Dsp1resetvecW<Dsp1resetvecSpec> {
        Dsp1resetvecW::new(self, 0)
    }
}
#[doc = "DSP 1 Reset Vector\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1resetvec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1resetvec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1resetvecSpec;
impl crate::RegisterSpec for Dsp1resetvecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1resetvec::R`](R) reader structure"]
impl crate::Readable for Dsp1resetvecSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1resetvec::W`](W) writer structure"]
impl crate::Writable for Dsp1resetvecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1RESETVEC to value 0"]
impl crate::Resettable for Dsp1resetvecSpec {
    const RESET_VALUE: u32 = 0;
}
