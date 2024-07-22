#[doc = "Register `DSP0IERSTAT` reader"]
pub type R = crate::R<Dsp0ierstatSpec>;
#[doc = "Register `DSP0IERSTAT` writer"]
pub type W = crate::W<Dsp0ierstatSpec>;
#[doc = "Field `DSP0INT` reader - DSP0 Watchdog Timer Interrupt."]
pub type Dsp0intR = crate::BitReader;
#[doc = "Field `DSP0INT` writer - DSP0 Watchdog Timer Interrupt."]
pub type Dsp0intW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DSP0 Watchdog Timer Interrupt."]
    #[inline(always)]
    pub fn dsp0int(&self) -> Dsp0intR {
        Dsp0intR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DSP0 Watchdog Timer Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dsp0int(&mut self) -> Dsp0intW<Dsp0ierstatSpec> {
        Dsp0intW::new(self, 0)
    }
}
#[doc = "Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0ierstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0ierstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0ierstatSpec;
impl crate::RegisterSpec for Dsp0ierstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0ierstat::R`](R) reader structure"]
impl crate::Readable for Dsp0ierstatSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0ierstat::W`](W) writer structure"]
impl crate::Writable for Dsp0ierstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0IERSTAT to value 0"]
impl crate::Resettable for Dsp0ierstatSpec {
    const RESET_VALUE: u32 = 0;
}
