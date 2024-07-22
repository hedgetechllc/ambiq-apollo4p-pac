#[doc = "Register `DSP0IDMATRIGCTL` reader"]
pub type R = crate::R<Dsp0idmatrigctlSpec>;
#[doc = "Register `DSP0IDMATRIGCTL` writer"]
pub type W = crate::W<Dsp0idmatrigctlSpec>;
#[doc = "Field `DSP0IDMATRIGSTAT` reader - DSP 0 iDMA Trigger Status"]
pub type Dsp0idmatrigstatR = crate::BitReader;
#[doc = "Field `DSP0IDMATRIGSTAT` writer - DSP 0 iDMA Trigger Status"]
pub type Dsp0idmatrigstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSP0IDMATRIGPULSE` reader - DSP 0 iDMA Trigger Pulse - When written a '1', this will cause a single step enable (valid only when IDMATRIG is set to SSTEP)"]
pub type Dsp0idmatrigpulseR = crate::BitReader;
#[doc = "Field `DSP0IDMATRIGPULSE` writer - DSP 0 iDMA Trigger Pulse - When written a '1', this will cause a single step enable (valid only when IDMATRIG is set to SSTEP)"]
pub type Dsp0idmatrigpulseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DSP 0 iDMA Trigger Status"]
    #[inline(always)]
    pub fn dsp0idmatrigstat(&self) -> Dsp0idmatrigstatR {
        Dsp0idmatrigstatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - DSP 0 iDMA Trigger Pulse - When written a '1', this will cause a single step enable (valid only when IDMATRIG is set to SSTEP)"]
    #[inline(always)]
    pub fn dsp0idmatrigpulse(&self) -> Dsp0idmatrigpulseR {
        Dsp0idmatrigpulseR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DSP 0 iDMA Trigger Status"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0idmatrigstat(&mut self) -> Dsp0idmatrigstatW<Dsp0idmatrigctlSpec> {
        Dsp0idmatrigstatW::new(self, 0)
    }
    #[doc = "Bit 4 - DSP 0 iDMA Trigger Pulse - When written a '1', this will cause a single step enable (valid only when IDMATRIG is set to SSTEP)"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0idmatrigpulse(&mut self) -> Dsp0idmatrigpulseW<Dsp0idmatrigctlSpec> {
        Dsp0idmatrigpulseW::new(self, 4)
    }
}
#[doc = "DSP 0 IDMA Trigger Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0idmatrigctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0idmatrigctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0idmatrigctlSpec;
impl crate::RegisterSpec for Dsp0idmatrigctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0idmatrigctl::R`](R) reader structure"]
impl crate::Readable for Dsp0idmatrigctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0idmatrigctl::W`](W) writer structure"]
impl crate::Writable for Dsp0idmatrigctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0IDMATRIGCTL to value 0"]
impl crate::Resettable for Dsp0idmatrigctlSpec {
    const RESET_VALUE: u32 = 0;
}
