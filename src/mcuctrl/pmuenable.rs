#[doc = "Register `PMUENABLE` reader"]
pub type R = crate::R<PmuenableSpec>;
#[doc = "Register `PMUENABLE` writer"]
pub type W = crate::W<PmuenableSpec>;
#[doc = "PMU Enable Control bit. When set, the MCU's PMU will place the MCU into the lowest power consuming Deep Sleep mode upon execution of a WFI instruction (dependent on the setting of the SLEEPDEEP bit in the ARM SCR register). When cleared, regardless of the requested sleep mode, the PMU will not enter the lowest power Deep Sleep mode, instead entering the Sleep mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Disable MCU power management."]
    Dis = 0,
    #[doc = "1: Enable MCU power management."]
    En = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - PMU Enable Control bit. When set, the MCU's PMU will place the MCU into the lowest power consuming Deep Sleep mode upon execution of a WFI instruction (dependent on the setting of the SLEEPDEEP bit in the ARM SCR register). When cleared, regardless of the requested sleep mode, the PMU will not enter the lowest power Deep Sleep mode, instead entering the Sleep mode."]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Dis,
            true => Enable::En,
        }
    }
    #[doc = "Disable MCU power management."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Enable::Dis
    }
    #[doc = "Enable MCU power management."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Enable::En
    }
}
#[doc = "Field `ENABLE` writer - PMU Enable Control bit. When set, the MCU's PMU will place the MCU into the lowest power consuming Deep Sleep mode upon execution of a WFI instruction (dependent on the setting of the SLEEPDEEP bit in the ARM SCR register). When cleared, regardless of the requested sleep mode, the PMU will not enter the lowest power Deep Sleep mode, instead entering the Sleep mode."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable MCU power management."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Dis)
    }
    #[doc = "Enable MCU power management."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::En)
    }
}
impl R {
    #[doc = "Bit 0 - PMU Enable Control bit. When set, the MCU's PMU will place the MCU into the lowest power consuming Deep Sleep mode upon execution of a WFI instruction (dependent on the setting of the SLEEPDEEP bit in the ARM SCR register). When cleared, regardless of the requested sleep mode, the PMU will not enter the lowest power Deep Sleep mode, instead entering the Sleep mode."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PMU Enable Control bit. When set, the MCU's PMU will place the MCU into the lowest power consuming Deep Sleep mode upon execution of a WFI instruction (dependent on the setting of the SLEEPDEEP bit in the ARM SCR register). When cleared, regardless of the requested sleep mode, the PMU will not enter the lowest power Deep Sleep mode, instead entering the Sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<PmuenableSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "Control bit to enable/disable the PMU\n\nYou can [`read`](crate::Reg::read) this register and get [`pmuenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmuenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuenableSpec;
impl crate::RegisterSpec for PmuenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmuenable::R`](R) reader structure"]
impl crate::Readable for PmuenableSpec {}
#[doc = "`write(|w| ..)` method takes [`pmuenable::W`](W) writer structure"]
impl crate::Writable for PmuenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUENABLE to value 0x01"]
impl crate::Resettable for PmuenableSpec {
    const RESET_VALUE: u32 = 0x01;
}
