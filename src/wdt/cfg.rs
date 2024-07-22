#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `WDTEN` reader - This bitfield enables the WDT."]
pub type WdtenR = crate::BitReader;
#[doc = "Field `WDTEN` writer - This bitfield enables the WDT."]
pub type WdtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN` reader - This bitfield enables the WDT interrupt. Note : This bit must be set before the interrupt status bit will reflect a watchdog timer expiration. The IER interrupt register must also be enabled for a WDT interrupt to be sent to the NVIC."]
pub type IntenR = crate::BitReader;
#[doc = "Field `INTEN` writer - This bitfield enables the WDT interrupt. Note : This bit must be set before the interrupt status bit will reflect a watchdog timer expiration. The IER interrupt register must also be enabled for a WDT interrupt to be sent to the NVIC."]
pub type IntenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESEN` reader - This bitfield enables the WDT reset. This needs to be set together with the WDREN bit in REG_RSTGEN_CFG register (in reset gen) to trigger the reset."]
pub type ResenR = crate::BitReader;
#[doc = "Field `RESEN` writer - This bitfield enables the WDT reset. This needs to be set together with the WDREN bit in REG_RSTGEN_CFG register (in reset gen) to trigger the reset."]
pub type ResenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSPRESETINTEN` reader - This bitfield enables the DSP Reset Interrupt. This interrupt is provided to the ARM CPU to notify it that a DSP's WDT has expired and a reset has been issued to one of the DSP cores."]
pub type DspresetintenR = crate::BitReader;
#[doc = "Field `DSPRESETINTEN` writer - This bitfield enables the DSP Reset Interrupt. This interrupt is provided to the ARM CPU to notify it that a DSP's WDT has expired and a reset has been issued to one of the DSP cores."]
pub type DspresetintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESVAL` reader - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset. This will cause a software reset."]
pub type ResvalR = crate::FieldReader;
#[doc = "Field `RESVAL` writer - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset. This will cause a software reset."]
pub type ResvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INTVAL` reader - This bitfield is the compare value for counter bits 7:0 to generate a watchdog interrupt."]
pub type IntvalR = crate::FieldReader;
#[doc = "Field `INTVAL` writer - This bitfield is the compare value for counter bits 7:0 to generate a watchdog interrupt."]
pub type IntvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Select the frequency for the WDT. All values not enumerated below are undefined.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "0: Low Power Mode. This setting disables the watch dog timer."]
    Off = 0,
    #[doc = "1: 128 Hz LFRC clock."]
    _128hz = 1,
    #[doc = "2: 16 Hz LFRC clock."]
    _16hz = 2,
    #[doc = "3: 1 Hz LFRC clock."]
    _1hz = 3,
    #[doc = "4: 1/16th Hz LFRC clock."]
    _1_16hz = 4,
}
impl From<Clksel> for u8 {
    #[inline(always)]
    fn from(variant: Clksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clksel {
    type Ux = u8;
}
impl crate::IsEnum for Clksel {}
#[doc = "Field `CLKSEL` reader - Select the frequency for the WDT. All values not enumerated below are undefined."]
pub type ClkselR = crate::FieldReader<Clksel>;
impl ClkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clksel> {
        match self.bits {
            0 => Some(Clksel::Off),
            1 => Some(Clksel::_128hz),
            2 => Some(Clksel::_16hz),
            3 => Some(Clksel::_1hz),
            4 => Some(Clksel::_1_16hz),
            _ => None,
        }
    }
    #[doc = "Low Power Mode. This setting disables the watch dog timer."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Clksel::Off
    }
    #[doc = "128 Hz LFRC clock."]
    #[inline(always)]
    pub fn is_128hz(&self) -> bool {
        *self == Clksel::_128hz
    }
    #[doc = "16 Hz LFRC clock."]
    #[inline(always)]
    pub fn is_16hz(&self) -> bool {
        *self == Clksel::_16hz
    }
    #[doc = "1 Hz LFRC clock."]
    #[inline(always)]
    pub fn is_1hz(&self) -> bool {
        *self == Clksel::_1hz
    }
    #[doc = "1/16th Hz LFRC clock."]
    #[inline(always)]
    pub fn is_1_16hz(&self) -> bool {
        *self == Clksel::_1_16hz
    }
}
#[doc = "Field `CLKSEL` writer - Select the frequency for the WDT. All values not enumerated below are undefined."]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Clksel>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low Power Mode. This setting disables the watch dog timer."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Off)
    }
    #[doc = "128 Hz LFRC clock."]
    #[inline(always)]
    pub fn _128hz(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::_128hz)
    }
    #[doc = "16 Hz LFRC clock."]
    #[inline(always)]
    pub fn _16hz(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::_16hz)
    }
    #[doc = "1 Hz LFRC clock."]
    #[inline(always)]
    pub fn _1hz(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::_1hz)
    }
    #[doc = "1/16th Hz LFRC clock."]
    #[inline(always)]
    pub fn _1_16hz(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::_1_16hz)
    }
}
impl R {
    #[doc = "Bit 0 - This bitfield enables the WDT."]
    #[inline(always)]
    pub fn wdten(&self) -> WdtenR {
        WdtenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bitfield enables the WDT interrupt. Note : This bit must be set before the interrupt status bit will reflect a watchdog timer expiration. The IER interrupt register must also be enabled for a WDT interrupt to be sent to the NVIC."]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bitfield enables the WDT reset. This needs to be set together with the WDREN bit in REG_RSTGEN_CFG register (in reset gen) to trigger the reset."]
    #[inline(always)]
    pub fn resen(&self) -> ResenR {
        ResenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bitfield enables the DSP Reset Interrupt. This interrupt is provided to the ARM CPU to notify it that a DSP's WDT has expired and a reset has been issued to one of the DSP cores."]
    #[inline(always)]
    pub fn dspresetinten(&self) -> DspresetintenR {
        DspresetintenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:15 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset. This will cause a software reset."]
    #[inline(always)]
    pub fn resval(&self) -> ResvalR {
        ResvalR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog interrupt."]
    #[inline(always)]
    pub fn intval(&self) -> IntvalR {
        IntvalR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - Select the frequency for the WDT. All values not enumerated below are undefined."]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This bitfield enables the WDT."]
    #[inline(always)]
    #[must_use]
    pub fn wdten(&mut self) -> WdtenW<CfgSpec> {
        WdtenW::new(self, 0)
    }
    #[doc = "Bit 1 - This bitfield enables the WDT interrupt. Note : This bit must be set before the interrupt status bit will reflect a watchdog timer expiration. The IER interrupt register must also be enabled for a WDT interrupt to be sent to the NVIC."]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> IntenW<CfgSpec> {
        IntenW::new(self, 1)
    }
    #[doc = "Bit 2 - This bitfield enables the WDT reset. This needs to be set together with the WDREN bit in REG_RSTGEN_CFG register (in reset gen) to trigger the reset."]
    #[inline(always)]
    #[must_use]
    pub fn resen(&mut self) -> ResenW<CfgSpec> {
        ResenW::new(self, 2)
    }
    #[doc = "Bit 3 - This bitfield enables the DSP Reset Interrupt. This interrupt is provided to the ARM CPU to notify it that a DSP's WDT has expired and a reset has been issued to one of the DSP cores."]
    #[inline(always)]
    #[must_use]
    pub fn dspresetinten(&mut self) -> DspresetintenW<CfgSpec> {
        DspresetintenW::new(self, 3)
    }
    #[doc = "Bits 8:15 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset. This will cause a software reset."]
    #[inline(always)]
    #[must_use]
    pub fn resval(&mut self) -> ResvalW<CfgSpec> {
        ResvalW::new(self, 8)
    }
    #[doc = "Bits 16:23 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn intval(&mut self) -> IntvalW<CfgSpec> {
        IntvalW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Select the frequency for the WDT. All values not enumerated below are undefined."]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> ClkselW<CfgSpec> {
        ClkselW::new(self, 24)
    }
}
#[doc = "This is the configuration register for the watch dog timer. It controls the enable, interrupt set, clocks for the timer, the compare values for the counters to trigger a reset or interrupt. This register can only be written to if the watch dog timer is unlocked (WDTLOCK is not set).\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0x00ff_ff00"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0x00ff_ff00;
}
