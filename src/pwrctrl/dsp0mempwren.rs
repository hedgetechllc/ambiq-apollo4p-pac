#[doc = "Register `DSP0MEMPWREN` reader"]
pub type R = crate::R<Dsp0mempwrenSpec>;
#[doc = "Register `DSP0MEMPWREN` writer"]
pub type W = crate::W<Dsp0mempwrenSpec>;
#[doc = "Power up DSP0 IRAM and DRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrendsp0ram {
    #[doc = "0: Do not power ON any of the IRAM/DRAM"]
    Off = 0,
    #[doc = "1: Power up all IRAM (128K) and DRAM (256K)"]
    On = 1,
}
impl From<Pwrendsp0ram> for bool {
    #[inline(always)]
    fn from(variant: Pwrendsp0ram) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENDSP0RAM` reader - Power up DSP0 IRAM and DRAM"]
pub type Pwrendsp0ramR = crate::BitReader<Pwrendsp0ram>;
impl Pwrendsp0ramR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrendsp0ram {
        match self.bits {
            false => Pwrendsp0ram::Off,
            true => Pwrendsp0ram::On,
        }
    }
    #[doc = "Do not power ON any of the IRAM/DRAM"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrendsp0ram::Off
    }
    #[doc = "Power up all IRAM (128K) and DRAM (256K)"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrendsp0ram::On
    }
}
#[doc = "Field `PWRENDSP0RAM` writer - Power up DSP0 IRAM and DRAM"]
pub type Pwrendsp0ramW<'a, REG> = crate::BitWriter<'a, REG, Pwrendsp0ram>;
impl<'a, REG> Pwrendsp0ramW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not power ON any of the IRAM/DRAM"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrendsp0ram::Off)
    }
    #[doc = "Power up all IRAM (128K) and DRAM (256K)"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrendsp0ram::On)
    }
}
#[doc = "Power up DSP0 ICACHE banks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrendsp0icache {
    #[doc = "0: Do not power up ICACHE"]
    Off = 0,
    #[doc = "1: Power up ICACHE"]
    On = 1,
}
impl From<Pwrendsp0icache> for bool {
    #[inline(always)]
    fn from(variant: Pwrendsp0icache) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENDSP0ICACHE` reader - Power up DSP0 ICACHE banks"]
pub type Pwrendsp0icacheR = crate::BitReader<Pwrendsp0icache>;
impl Pwrendsp0icacheR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrendsp0icache {
        match self.bits {
            false => Pwrendsp0icache::Off,
            true => Pwrendsp0icache::On,
        }
    }
    #[doc = "Do not power up ICACHE"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrendsp0icache::Off
    }
    #[doc = "Power up ICACHE"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrendsp0icache::On
    }
}
#[doc = "Field `PWRENDSP0ICACHE` writer - Power up DSP0 ICACHE banks"]
pub type Pwrendsp0icacheW<'a, REG> = crate::BitWriter<'a, REG, Pwrendsp0icache>;
impl<'a, REG> Pwrendsp0icacheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not power up ICACHE"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrendsp0icache::Off)
    }
    #[doc = "Power up ICACHE"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrendsp0icache::On)
    }
}
impl R {
    #[doc = "Bit 0 - Power up DSP0 IRAM and DRAM"]
    #[inline(always)]
    pub fn pwrendsp0ram(&self) -> Pwrendsp0ramR {
        Pwrendsp0ramR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power up DSP0 ICACHE banks"]
    #[inline(always)]
    pub fn pwrendsp0icache(&self) -> Pwrendsp0icacheR {
        Pwrendsp0icacheR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power up DSP0 IRAM and DRAM"]
    #[inline(always)]
    #[must_use]
    pub fn pwrendsp0ram(&mut self) -> Pwrendsp0ramW<Dsp0mempwrenSpec> {
        Pwrendsp0ramW::new(self, 0)
    }
    #[doc = "Bit 1 - Power up DSP0 ICACHE banks"]
    #[inline(always)]
    #[must_use]
    pub fn pwrendsp0icache(&mut self) -> Pwrendsp0icacheW<Dsp0mempwrenSpec> {
        Pwrendsp0icacheW::new(self, 1)
    }
}
#[doc = "This register enables the individual banks for the memories. When set, power will be enabled to the banks. This register works in conjunction with the DSP0MEMRETCFG register when DSP0 is OFF.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0mempwren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0mempwren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0mempwrenSpec;
impl crate::RegisterSpec for Dsp0mempwrenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0mempwren::R`](R) reader structure"]
impl crate::Readable for Dsp0mempwrenSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0mempwren::W`](W) writer structure"]
impl crate::Writable for Dsp0mempwrenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0MEMPWREN to value 0"]
impl crate::Resettable for Dsp0mempwrenSpec {
    const RESET_VALUE: u32 = 0;
}
