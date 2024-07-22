#[doc = "Register `SYSPWRSTATUS` reader"]
pub type R = crate::R<SyspwrstatusSpec>;
#[doc = "Register `SYSPWRSTATUS` writer"]
pub type W = crate::W<SyspwrstatusSpec>;
#[doc = "Field `PWRSTMCUL` reader - Power Domain status for MCUL"]
pub type PwrstmculR = crate::BitReader;
#[doc = "Field `PWRSTMCUL` writer - Power Domain status for MCUL"]
pub type PwrstmculW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSTMCUH` reader - Power Domain status for MCUH"]
pub type PwrstmcuhR = crate::BitReader;
#[doc = "Field `PWRSTMCUH` writer - Power Domain status for MCUH"]
pub type PwrstmcuhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSTDSP0H` reader - Power Domain status for DSP0H"]
pub type Pwrstdsp0hR = crate::BitReader;
#[doc = "Field `PWRSTDSP0H` writer - Power Domain status for DSP0H"]
pub type Pwrstdsp0hW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSTDSP1H` reader - Power Domain status for DSP1H"]
pub type Pwrstdsp1hR = crate::BitReader;
#[doc = "Field `PWRSTDSP1H` writer - Power Domain status for DSP1H"]
pub type Pwrstdsp1hW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORESLEEP` reader - Indicates MCU entered SLEEP state since it was last cleared. Write 1 to to clear it."]
pub type CoresleepR = crate::BitReader;
#[doc = "Field `CORESLEEP` writer - Indicates MCU entered SLEEP state since it was last cleared. Write 1 to to clear it."]
pub type CoresleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COREDEEPSLEEP` reader - Indicates MCU entered DEEPSLEEP state since it was last cleared. Write 1 to to clear it."]
pub type CoredeepsleepR = crate::BitReader;
#[doc = "Field `COREDEEPSLEEP` writer - Indicates MCU entered DEEPSLEEP state since it was last cleared. Write 1 to to clear it."]
pub type CoredeepsleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSDEEPSLEEP` reader - Indicates all device domains powered down and MCU entered DEEPSLEEP state since it was last cleared. Write 1 to to clear it."]
pub type SysdeepsleepR = crate::BitReader;
#[doc = "Field `SYSDEEPSLEEP` writer - Indicates all device domains powered down and MCU entered DEEPSLEEP state since it was last cleared. Write 1 to to clear it."]
pub type SysdeepsleepW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power Domain status for MCUL"]
    #[inline(always)]
    pub fn pwrstmcul(&self) -> PwrstmculR {
        PwrstmculR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Domain status for MCUH"]
    #[inline(always)]
    pub fn pwrstmcuh(&self) -> PwrstmcuhR {
        PwrstmcuhR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power Domain status for DSP0H"]
    #[inline(always)]
    pub fn pwrstdsp0h(&self) -> Pwrstdsp0hR {
        Pwrstdsp0hR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power Domain status for DSP1H"]
    #[inline(always)]
    pub fn pwrstdsp1h(&self) -> Pwrstdsp1hR {
        Pwrstdsp1hR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 29 - Indicates MCU entered SLEEP state since it was last cleared. Write 1 to to clear it."]
    #[inline(always)]
    pub fn coresleep(&self) -> CoresleepR {
        CoresleepR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Indicates MCU entered DEEPSLEEP state since it was last cleared. Write 1 to to clear it."]
    #[inline(always)]
    pub fn coredeepsleep(&self) -> CoredeepsleepR {
        CoredeepsleepR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Indicates all device domains powered down and MCU entered DEEPSLEEP state since it was last cleared. Write 1 to to clear it."]
    #[inline(always)]
    pub fn sysdeepsleep(&self) -> SysdeepsleepR {
        SysdeepsleepR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Domain status for MCUL"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstmcul(&mut self) -> PwrstmculW<SyspwrstatusSpec> {
        PwrstmculW::new(self, 0)
    }
    #[doc = "Bit 1 - Power Domain status for MCUH"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstmcuh(&mut self) -> PwrstmcuhW<SyspwrstatusSpec> {
        PwrstmcuhW::new(self, 1)
    }
    #[doc = "Bit 2 - Power Domain status for DSP0H"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstdsp0h(&mut self) -> Pwrstdsp0hW<SyspwrstatusSpec> {
        Pwrstdsp0hW::new(self, 2)
    }
    #[doc = "Bit 3 - Power Domain status for DSP1H"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstdsp1h(&mut self) -> Pwrstdsp1hW<SyspwrstatusSpec> {
        Pwrstdsp1hW::new(self, 3)
    }
    #[doc = "Bit 29 - Indicates MCU entered SLEEP state since it was last cleared. Write 1 to to clear it."]
    #[inline(always)]
    #[must_use]
    pub fn coresleep(&mut self) -> CoresleepW<SyspwrstatusSpec> {
        CoresleepW::new(self, 29)
    }
    #[doc = "Bit 30 - Indicates MCU entered DEEPSLEEP state since it was last cleared. Write 1 to to clear it."]
    #[inline(always)]
    #[must_use]
    pub fn coredeepsleep(&mut self) -> CoredeepsleepW<SyspwrstatusSpec> {
        CoredeepsleepW::new(self, 30)
    }
    #[doc = "Bit 31 - Indicates all device domains powered down and MCU entered DEEPSLEEP state since it was last cleared. Write 1 to to clear it."]
    #[inline(always)]
    #[must_use]
    pub fn sysdeepsleep(&mut self) -> SysdeepsleepW<SyspwrstatusSpec> {
        SysdeepsleepW::new(self, 31)
    }
}
#[doc = "Power ON Status for domains that are not part of devpwrstatus or mempwrstatus\n\nYou can [`read`](crate::Reg::read) this register and get [`syspwrstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspwrstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyspwrstatusSpec;
impl crate::RegisterSpec for SyspwrstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syspwrstatus::R`](R) reader structure"]
impl crate::Readable for SyspwrstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`syspwrstatus::W`](W) writer structure"]
impl crate::Writable for SyspwrstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSPWRSTATUS to value 0x0f"]
impl crate::Resettable for SyspwrstatusSpec {
    const RESET_VALUE: u32 = 0x0f;
}
