#[doc = "Register `DSP0COUNT` reader"]
pub type R = crate::R<Dsp0countSpec>;
#[doc = "Register `DSP0COUNT` writer"]
pub type W = crate::W<Dsp0countSpec>;
#[doc = "Field `DSP0COUNT` reader - Read-Only current value of the WDT counter"]
pub type Dsp0countR = crate::FieldReader;
#[doc = "Field `DSP0COUNT` writer - Read-Only current value of the WDT counter"]
pub type Dsp0countW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Read-Only current value of the WDT counter"]
    #[inline(always)]
    pub fn dsp0count(&self) -> Dsp0countR {
        Dsp0countR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read-Only current value of the WDT counter"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0count(&mut self) -> Dsp0countW<Dsp0countSpec> {
        Dsp0countW::new(self, 0)
    }
}
#[doc = "This register holds the current count for the watch dog timer. This is a read only register. SW cannot set the value in the counter, but can reset it.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0countSpec;
impl crate::RegisterSpec for Dsp0countSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0count::R`](R) reader structure"]
impl crate::Readable for Dsp0countSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0count::W`](W) writer structure"]
impl crate::Writable for Dsp0countSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0COUNT to value 0"]
impl crate::Resettable for Dsp0countSpec {
    const RESET_VALUE: u32 = 0;
}
