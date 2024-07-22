#[doc = "Register `DSP0MEMPWRST` reader"]
pub type R = crate::R<Dsp0mempwrstSpec>;
#[doc = "Register `DSP0MEMPWRST` writer"]
pub type W = crate::W<Dsp0mempwrstSpec>;
#[doc = "Field `PWRSTDSP0RAM` reader - Status- 1:ON, 0:OFF"]
pub type Pwrstdsp0ramR = crate::BitReader;
#[doc = "Field `PWRSTDSP0RAM` writer - Status- 1:ON, 0:OFF"]
pub type Pwrstdsp0ramW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSTDSP0ICACHE` reader - Power Status- 1:ON, 0:OFF"]
pub type Pwrstdsp0icacheR = crate::BitReader;
#[doc = "Field `PWRSTDSP0ICACHE` writer - Power Status- 1:ON, 0:OFF"]
pub type Pwrstdsp0icacheW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Status- 1:ON, 0:OFF"]
    #[inline(always)]
    pub fn pwrstdsp0ram(&self) -> Pwrstdsp0ramR {
        Pwrstdsp0ramR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Status- 1:ON, 0:OFF"]
    #[inline(always)]
    pub fn pwrstdsp0icache(&self) -> Pwrstdsp0icacheR {
        Pwrstdsp0icacheR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Status- 1:ON, 0:OFF"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstdsp0ram(&mut self) -> Pwrstdsp0ramW<Dsp0mempwrstSpec> {
        Pwrstdsp0ramW::new(self, 0)
    }
    #[doc = "Bit 1 - Power Status- 1:ON, 0:OFF"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstdsp0icache(&mut self) -> Pwrstdsp0icacheW<Dsp0mempwrstSpec> {
        Pwrstdsp0icacheW::new(self, 1)
    }
}
#[doc = "It provides the power status for all the memories of DSP0 subsystem\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0mempwrst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0mempwrst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0mempwrstSpec;
impl crate::RegisterSpec for Dsp0mempwrstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0mempwrst::R`](R) reader structure"]
impl crate::Readable for Dsp0mempwrstSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0mempwrst::W`](W) writer structure"]
impl crate::Writable for Dsp0mempwrstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0MEMPWRST to value 0"]
impl crate::Resettable for Dsp0mempwrstSpec {
    const RESET_VALUE: u32 = 0;
}
