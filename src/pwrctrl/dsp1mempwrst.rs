#[doc = "Register `DSP1MEMPWRST` reader"]
pub type R = crate::R<Dsp1mempwrstSpec>;
#[doc = "Register `DSP1MEMPWRST` writer"]
pub type W = crate::W<Dsp1mempwrstSpec>;
#[doc = "Field `PWRSTDSP1RAM` reader - Status- 1:ON, 0:OFF"]
pub type Pwrstdsp1ramR = crate::BitReader;
#[doc = "Field `PWRSTDSP1RAM` writer - Status- 1:ON, 0:OFF"]
pub type Pwrstdsp1ramW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSTDSP1ICACHE` reader - Power Status- 1:ON, 0:OFF"]
pub type Pwrstdsp1icacheR = crate::BitReader;
#[doc = "Field `PWRSTDSP1ICACHE` writer - Power Status- 1:ON, 0:OFF"]
pub type Pwrstdsp1icacheW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Status- 1:ON, 0:OFF"]
    #[inline(always)]
    pub fn pwrstdsp1ram(&self) -> Pwrstdsp1ramR {
        Pwrstdsp1ramR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Status- 1:ON, 0:OFF"]
    #[inline(always)]
    pub fn pwrstdsp1icache(&self) -> Pwrstdsp1icacheR {
        Pwrstdsp1icacheR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Status- 1:ON, 0:OFF"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstdsp1ram(&mut self) -> Pwrstdsp1ramW<Dsp1mempwrstSpec> {
        Pwrstdsp1ramW::new(self, 0)
    }
    #[doc = "Bit 1 - Power Status- 1:ON, 0:OFF"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstdsp1icache(&mut self) -> Pwrstdsp1icacheW<Dsp1mempwrstSpec> {
        Pwrstdsp1icacheW::new(self, 1)
    }
}
#[doc = "It provides the power status for all the memories of DSP1 subsystem\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1mempwrst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1mempwrst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1mempwrstSpec;
impl crate::RegisterSpec for Dsp1mempwrstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1mempwrst::R`](R) reader structure"]
impl crate::Readable for Dsp1mempwrstSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1mempwrst::W`](W) writer structure"]
impl crate::Writable for Dsp1mempwrstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1MEMPWRST to value 0"]
impl crate::Resettable for Dsp1mempwrstSpec {
    const RESET_VALUE: u32 = 0;
}
