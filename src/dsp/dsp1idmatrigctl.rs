#[doc = "Register `DSP1IDMATRIGCTL` reader"]
pub type R = crate::R<Dsp1idmatrigctlSpec>;
#[doc = "Register `DSP1IDMATRIGCTL` writer"]
pub type W = crate::W<Dsp1idmatrigctlSpec>;
#[doc = "Field `DSP1IDMATRIGSTAT` reader - DSP 1 iDMA Trigger Status"]
pub type Dsp1idmatrigstatR = crate::BitReader;
#[doc = "Field `DSP1IDMATRIGSTAT` writer - DSP 1 iDMA Trigger Status"]
pub type Dsp1idmatrigstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSP1IDMATRIGPULSE` reader - DSP 1 iDMA Trigger Pulse - When written a '1', this will cause a single step enable (valid only when IDMATRIG is set to SSTEP)"]
pub type Dsp1idmatrigpulseR = crate::BitReader;
#[doc = "Field `DSP1IDMATRIGPULSE` writer - DSP 1 iDMA Trigger Pulse - When written a '1', this will cause a single step enable (valid only when IDMATRIG is set to SSTEP)"]
pub type Dsp1idmatrigpulseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DSP 1 iDMA Trigger Status"]
    #[inline(always)]
    pub fn dsp1idmatrigstat(&self) -> Dsp1idmatrigstatR {
        Dsp1idmatrigstatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - DSP 1 iDMA Trigger Pulse - When written a '1', this will cause a single step enable (valid only when IDMATRIG is set to SSTEP)"]
    #[inline(always)]
    pub fn dsp1idmatrigpulse(&self) -> Dsp1idmatrigpulseR {
        Dsp1idmatrigpulseR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DSP 1 iDMA Trigger Status"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1idmatrigstat(&mut self) -> Dsp1idmatrigstatW<Dsp1idmatrigctlSpec> {
        Dsp1idmatrigstatW::new(self, 0)
    }
    #[doc = "Bit 4 - DSP 1 iDMA Trigger Pulse - When written a '1', this will cause a single step enable (valid only when IDMATRIG is set to SSTEP)"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1idmatrigpulse(&mut self) -> Dsp1idmatrigpulseW<Dsp1idmatrigctlSpec> {
        Dsp1idmatrigpulseW::new(self, 4)
    }
}
#[doc = "DSP 1 IDMA Trigger Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1idmatrigctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1idmatrigctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1idmatrigctlSpec;
impl crate::RegisterSpec for Dsp1idmatrigctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1idmatrigctl::R`](R) reader structure"]
impl crate::Readable for Dsp1idmatrigctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1idmatrigctl::W`](W) writer structure"]
impl crate::Writable for Dsp1idmatrigctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1IDMATRIGCTL to value 0"]
impl crate::Resettable for Dsp1idmatrigctlSpec {
    const RESET_VALUE: u32 = 0;
}
