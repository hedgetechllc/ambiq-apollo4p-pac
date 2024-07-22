#[doc = "Register `DSP1IEREN` reader"]
pub type R = crate::R<Dsp1ierenSpec>;
#[doc = "Register `DSP1IEREN` writer"]
pub type W = crate::W<Dsp1ierenSpec>;
#[doc = "Field `DSP1INT` reader - DSP0 Watchdog Timer Interrupt."]
pub type Dsp1intR = crate::BitReader;
#[doc = "Field `DSP1INT` writer - DSP0 Watchdog Timer Interrupt."]
pub type Dsp1intW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DSP0 Watchdog Timer Interrupt."]
    #[inline(always)]
    pub fn dsp1int(&self) -> Dsp1intR {
        Dsp1intR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DSP0 Watchdog Timer Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dsp1int(&mut self) -> Dsp1intW<Dsp1ierenSpec> {
        Dsp1intW::new(self, 0)
    }
}
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1ieren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1ieren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1ierenSpec;
impl crate::RegisterSpec for Dsp1ierenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1ieren::R`](R) reader structure"]
impl crate::Readable for Dsp1ierenSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1ieren::W`](W) writer structure"]
impl crate::Writable for Dsp1ierenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1IEREN to value 0"]
impl crate::Resettable for Dsp1ierenSpec {
    const RESET_VALUE: u32 = 0;
}
