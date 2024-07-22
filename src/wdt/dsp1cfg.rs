#[doc = "Register `DSP1CFG` reader"]
pub type R = crate::R<Dsp1cfgSpec>;
#[doc = "Register `DSP1CFG` writer"]
pub type W = crate::W<Dsp1cfgSpec>;
#[doc = "Field `DSP1WDTEN` reader - This bitfield enables the WDT. Setting the lock implicitly sets the WTDEN bit as well."]
pub type Dsp1wdtenR = crate::BitReader;
#[doc = "Field `DSP1WDTEN` writer - This bitfield enables the WDT. Setting the lock implicitly sets the WTDEN bit as well."]
pub type Dsp1wdtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSP1INTEN` reader - This bitfield enables the DSP1 WDT interrupt. Note : This bit must be set before the interrupt status bit will reflect a watchdog timer expiration. The IER interrupt register must also be enabled for a WDT interrupt to be sent to the NVIC."]
pub type Dsp1intenR = crate::BitReader;
#[doc = "Field `DSP1INTEN` writer - This bitfield enables the DSP1 WDT interrupt. Note : This bit must be set before the interrupt status bit will reflect a watchdog timer expiration. The IER interrupt register must also be enabled for a WDT interrupt to be sent to the NVIC."]
pub type Dsp1intenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSP1RESEN` reader - This bitfield enables the DSP1 reset."]
pub type Dsp1resenR = crate::BitReader;
#[doc = "Field `DSP1RESEN` writer - This bitfield enables the DSP1 reset."]
pub type Dsp1resenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSP1PMRESEN` reader - This bitfield enables the DSP1 Power Controller (PM) reset. This needs to be set together with the DSP1WDTEN bit to allow the reset to trigger."]
pub type Dsp1pmresenR = crate::BitReader;
#[doc = "Field `DSP1PMRESEN` writer - This bitfield enables the DSP1 Power Controller (PM) reset. This needs to be set together with the DSP1WDTEN bit to allow the reset to trigger."]
pub type Dsp1pmresenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSP1RESVAL` reader - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset for the DSP logic. This will cause a software reset to the DSP core if the RESEN bit is set and optionally interrupt the CPU."]
pub type Dsp1resvalR = crate::FieldReader;
#[doc = "Field `DSP1RESVAL` writer - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset for the DSP logic. This will cause a software reset to the DSP core if the RESEN bit is set and optionally interrupt the CPU."]
pub type Dsp1resvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DSP1INTVAL` reader - This bitfield is the compare value for counter bits 7:0 to generate a watchdog interrupt."]
pub type Dsp1intvalR = crate::FieldReader;
#[doc = "Field `DSP1INTVAL` writer - This bitfield is the compare value for counter bits 7:0 to generate a watchdog interrupt."]
pub type Dsp1intvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DSP1PMRESVAL` reader - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset. This will cause a software reset to the DSP Power Management logic if the PMRESEN bit is set and optionally interrupt the CPU."]
pub type Dsp1pmresvalR = crate::FieldReader;
#[doc = "Field `DSP1PMRESVAL` writer - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset. This will cause a software reset to the DSP Power Management logic if the PMRESEN bit is set and optionally interrupt the CPU."]
pub type Dsp1pmresvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - This bitfield enables the WDT. Setting the lock implicitly sets the WTDEN bit as well."]
    #[inline(always)]
    pub fn dsp1wdten(&self) -> Dsp1wdtenR {
        Dsp1wdtenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bitfield enables the DSP1 WDT interrupt. Note : This bit must be set before the interrupt status bit will reflect a watchdog timer expiration. The IER interrupt register must also be enabled for a WDT interrupt to be sent to the NVIC."]
    #[inline(always)]
    pub fn dsp1inten(&self) -> Dsp1intenR {
        Dsp1intenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bitfield enables the DSP1 reset."]
    #[inline(always)]
    pub fn dsp1resen(&self) -> Dsp1resenR {
        Dsp1resenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bitfield enables the DSP1 Power Controller (PM) reset. This needs to be set together with the DSP1WDTEN bit to allow the reset to trigger."]
    #[inline(always)]
    pub fn dsp1pmresen(&self) -> Dsp1pmresenR {
        Dsp1pmresenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:15 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset for the DSP logic. This will cause a software reset to the DSP core if the RESEN bit is set and optionally interrupt the CPU."]
    #[inline(always)]
    pub fn dsp1resval(&self) -> Dsp1resvalR {
        Dsp1resvalR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog interrupt."]
    #[inline(always)]
    pub fn dsp1intval(&self) -> Dsp1intvalR {
        Dsp1intvalR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset. This will cause a software reset to the DSP Power Management logic if the PMRESEN bit is set and optionally interrupt the CPU."]
    #[inline(always)]
    pub fn dsp1pmresval(&self) -> Dsp1pmresvalR {
        Dsp1pmresvalR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This bitfield enables the WDT. Setting the lock implicitly sets the WTDEN bit as well."]
    #[inline(always)]
    #[must_use]
    pub fn dsp1wdten(&mut self) -> Dsp1wdtenW<Dsp1cfgSpec> {
        Dsp1wdtenW::new(self, 0)
    }
    #[doc = "Bit 1 - This bitfield enables the DSP1 WDT interrupt. Note : This bit must be set before the interrupt status bit will reflect a watchdog timer expiration. The IER interrupt register must also be enabled for a WDT interrupt to be sent to the NVIC."]
    #[inline(always)]
    #[must_use]
    pub fn dsp1inten(&mut self) -> Dsp1intenW<Dsp1cfgSpec> {
        Dsp1intenW::new(self, 1)
    }
    #[doc = "Bit 2 - This bitfield enables the DSP1 reset."]
    #[inline(always)]
    #[must_use]
    pub fn dsp1resen(&mut self) -> Dsp1resenW<Dsp1cfgSpec> {
        Dsp1resenW::new(self, 2)
    }
    #[doc = "Bit 3 - This bitfield enables the DSP1 Power Controller (PM) reset. This needs to be set together with the DSP1WDTEN bit to allow the reset to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn dsp1pmresen(&mut self) -> Dsp1pmresenW<Dsp1cfgSpec> {
        Dsp1pmresenW::new(self, 3)
    }
    #[doc = "Bits 8:15 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset for the DSP logic. This will cause a software reset to the DSP core if the RESEN bit is set and optionally interrupt the CPU."]
    #[inline(always)]
    #[must_use]
    pub fn dsp1resval(&mut self) -> Dsp1resvalW<Dsp1cfgSpec> {
        Dsp1resvalW::new(self, 8)
    }
    #[doc = "Bits 16:23 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dsp1intval(&mut self) -> Dsp1intvalW<Dsp1cfgSpec> {
        Dsp1intvalW::new(self, 16)
    }
    #[doc = "Bits 24:31 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset. This will cause a software reset to the DSP Power Management logic if the PMRESEN bit is set and optionally interrupt the CPU."]
    #[inline(always)]
    #[must_use]
    pub fn dsp1pmresval(&mut self) -> Dsp1pmresvalW<Dsp1cfgSpec> {
        Dsp1pmresvalW::new(self, 24)
    }
}
#[doc = "This is the configuration register for the DSP1 watch dog timer. It controls the enable, interrupt set, clocks for the timer, the compare values for the counters to trigger a reset or interrupt. This register can only be written to if the associated DSP1TLOCK is not set.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1cfgSpec;
impl crate::RegisterSpec for Dsp1cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1cfg::R`](R) reader structure"]
impl crate::Readable for Dsp1cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1cfg::W`](W) writer structure"]
impl crate::Writable for Dsp1cfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1CFG to value 0xffff_ff00"]
impl crate::Resettable for Dsp1cfgSpec {
    const RESET_VALUE: u32 = 0xffff_ff00;
}
