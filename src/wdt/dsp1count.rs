#[doc = "Register `DSP1COUNT` reader"]
pub type R = crate::R<Dsp1countSpec>;
#[doc = "Register `DSP1COUNT` writer"]
pub type W = crate::W<Dsp1countSpec>;
#[doc = "Field `DSP1COUNT` reader - Read-Only current value of the WDT counter"]
pub type Dsp1countR = crate::FieldReader;
#[doc = "Field `DSP1COUNT` writer - Read-Only current value of the WDT counter"]
pub type Dsp1countW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Read-Only current value of the WDT counter"]
    #[inline(always)]
    pub fn dsp1count(&self) -> Dsp1countR {
        Dsp1countR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read-Only current value of the WDT counter"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1count(&mut self) -> Dsp1countW<Dsp1countSpec> {
        Dsp1countW::new(self, 0)
    }
}
#[doc = "This register holds the current count for the watch dog timer. This is a read only register. SW cannot set the value in the counter, but can reset it.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1countSpec;
impl crate::RegisterSpec for Dsp1countSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1count::R`](R) reader structure"]
impl crate::Readable for Dsp1countSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1count::W`](W) writer structure"]
impl crate::Writable for Dsp1countSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1COUNT to value 0"]
impl crate::Resettable for Dsp1countSpec {
    const RESET_VALUE: u32 = 0;
}
