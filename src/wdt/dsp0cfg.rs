#[doc = "Register `DSP0CFG` reader"]
pub type R = crate::R<Dsp0cfgSpec>;
#[doc = "Register `DSP0CFG` writer"]
pub type W = crate::W<Dsp0cfgSpec>;
#[doc = "Field `DSP0WDTEN` reader - This bitfield enables the WDT. Setting the lock implicitly sets the WTDEN bit as well."]
pub type Dsp0wdtenR = crate::BitReader;
#[doc = "Field `DSP0WDTEN` writer - This bitfield enables the WDT. Setting the lock implicitly sets the WTDEN bit as well."]
pub type Dsp0wdtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSP0INTEN` reader - This bitfield enables the DSP0 WDT interrupt. Note : This bit must be set before the interrupt status bit will reflect a watchdog timer expiration. The IER interrupt register must also be enabled for a WDT interrupt to be sent to the NVIC."]
pub type Dsp0intenR = crate::BitReader;
#[doc = "Field `DSP0INTEN` writer - This bitfield enables the DSP0 WDT interrupt. Note : This bit must be set before the interrupt status bit will reflect a watchdog timer expiration. The IER interrupt register must also be enabled for a WDT interrupt to be sent to the NVIC."]
pub type Dsp0intenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSP0RESEN` reader - This bitfield enables the DSP0 reset."]
pub type Dsp0resenR = crate::BitReader;
#[doc = "Field `DSP0RESEN` writer - This bitfield enables the DSP0 reset."]
pub type Dsp0resenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSP0PMRESEN` reader - This bitfield enables the DSP0 Power Controller (PM) reset. This needs to be set together with the DSP0WDTEN bit to allow the reset to trigger."]
pub type Dsp0pmresenR = crate::BitReader;
#[doc = "Field `DSP0PMRESEN` writer - This bitfield enables the DSP0 Power Controller (PM) reset. This needs to be set together with the DSP0WDTEN bit to allow the reset to trigger."]
pub type Dsp0pmresenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSP0RESVAL` reader - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset for the DSP logic. This will cause a software reset to the DSP core if the RESEN bit is set and optionally interrupt the CPU."]
pub type Dsp0resvalR = crate::FieldReader;
#[doc = "Field `DSP0RESVAL` writer - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset for the DSP logic. This will cause a software reset to the DSP core if the RESEN bit is set and optionally interrupt the CPU."]
pub type Dsp0resvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DSP0INTVAL` reader - This bitfield is the compare value for counter bits 7:0 to generate a watchdog interrupt."]
pub type Dsp0intvalR = crate::FieldReader;
#[doc = "Field `DSP0INTVAL` writer - This bitfield is the compare value for counter bits 7:0 to generate a watchdog interrupt."]
pub type Dsp0intvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DSP0PMRESVAL` reader - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset. This will cause a software reset to the DSP Power Management logic if the PMRESEN bit is set and optionally interrupt the CPU."]
pub type Dsp0pmresvalR = crate::FieldReader;
#[doc = "Field `DSP0PMRESVAL` writer - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset. This will cause a software reset to the DSP Power Management logic if the PMRESEN bit is set and optionally interrupt the CPU."]
pub type Dsp0pmresvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - This bitfield enables the WDT. Setting the lock implicitly sets the WTDEN bit as well."]
    #[inline(always)]
    pub fn dsp0wdten(&self) -> Dsp0wdtenR {
        Dsp0wdtenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bitfield enables the DSP0 WDT interrupt. Note : This bit must be set before the interrupt status bit will reflect a watchdog timer expiration. The IER interrupt register must also be enabled for a WDT interrupt to be sent to the NVIC."]
    #[inline(always)]
    pub fn dsp0inten(&self) -> Dsp0intenR {
        Dsp0intenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bitfield enables the DSP0 reset."]
    #[inline(always)]
    pub fn dsp0resen(&self) -> Dsp0resenR {
        Dsp0resenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bitfield enables the DSP0 Power Controller (PM) reset. This needs to be set together with the DSP0WDTEN bit to allow the reset to trigger."]
    #[inline(always)]
    pub fn dsp0pmresen(&self) -> Dsp0pmresenR {
        Dsp0pmresenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:15 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset for the DSP logic. This will cause a software reset to the DSP core if the RESEN bit is set and optionally interrupt the CPU."]
    #[inline(always)]
    pub fn dsp0resval(&self) -> Dsp0resvalR {
        Dsp0resvalR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog interrupt."]
    #[inline(always)]
    pub fn dsp0intval(&self) -> Dsp0intvalR {
        Dsp0intvalR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset. This will cause a software reset to the DSP Power Management logic if the PMRESEN bit is set and optionally interrupt the CPU."]
    #[inline(always)]
    pub fn dsp0pmresval(&self) -> Dsp0pmresvalR {
        Dsp0pmresvalR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This bitfield enables the WDT. Setting the lock implicitly sets the WTDEN bit as well."]
    #[inline(always)]
    #[must_use]
    pub fn dsp0wdten(&mut self) -> Dsp0wdtenW<Dsp0cfgSpec> {
        Dsp0wdtenW::new(self, 0)
    }
    #[doc = "Bit 1 - This bitfield enables the DSP0 WDT interrupt. Note : This bit must be set before the interrupt status bit will reflect a watchdog timer expiration. The IER interrupt register must also be enabled for a WDT interrupt to be sent to the NVIC."]
    #[inline(always)]
    #[must_use]
    pub fn dsp0inten(&mut self) -> Dsp0intenW<Dsp0cfgSpec> {
        Dsp0intenW::new(self, 1)
    }
    #[doc = "Bit 2 - This bitfield enables the DSP0 reset."]
    #[inline(always)]
    #[must_use]
    pub fn dsp0resen(&mut self) -> Dsp0resenW<Dsp0cfgSpec> {
        Dsp0resenW::new(self, 2)
    }
    #[doc = "Bit 3 - This bitfield enables the DSP0 Power Controller (PM) reset. This needs to be set together with the DSP0WDTEN bit to allow the reset to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn dsp0pmresen(&mut self) -> Dsp0pmresenW<Dsp0cfgSpec> {
        Dsp0pmresenW::new(self, 3)
    }
    #[doc = "Bits 8:15 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset for the DSP logic. This will cause a software reset to the DSP core if the RESEN bit is set and optionally interrupt the CPU."]
    #[inline(always)]
    #[must_use]
    pub fn dsp0resval(&mut self) -> Dsp0resvalW<Dsp0cfgSpec> {
        Dsp0resvalW::new(self, 8)
    }
    #[doc = "Bits 16:23 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dsp0intval(&mut self) -> Dsp0intvalW<Dsp0cfgSpec> {
        Dsp0intvalW::new(self, 16)
    }
    #[doc = "Bits 24:31 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset. This will cause a software reset to the DSP Power Management logic if the PMRESEN bit is set and optionally interrupt the CPU."]
    #[inline(always)]
    #[must_use]
    pub fn dsp0pmresval(&mut self) -> Dsp0pmresvalW<Dsp0cfgSpec> {
        Dsp0pmresvalW::new(self, 24)
    }
}
#[doc = "This is the configuration register for the DSP0 watch dog timer. It controls the enable, interrupt set, clocks for the timer, the compare values for the counters to trigger a reset or interrupt. This register can only be written to if the associated DSP0TLOCK is not set.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0cfgSpec;
impl crate::RegisterSpec for Dsp0cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0cfg::R`](R) reader structure"]
impl crate::Readable for Dsp0cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0cfg::W`](W) writer structure"]
impl crate::Writable for Dsp0cfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0CFG to value 0xffff_ff00"]
impl crate::Resettable for Dsp0cfgSpec {
    const RESET_VALUE: u32 = 0xffff_ff00;
}
