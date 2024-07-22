#[doc = "Register `DSP1MEMPWREN` reader"]
pub type R = crate::R<Dsp1mempwrenSpec>;
#[doc = "Register `DSP1MEMPWREN` writer"]
pub type W = crate::W<Dsp1mempwrenSpec>;
#[doc = "Power up DSP1 IRAM and DRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrendsp1ram {
    #[doc = "0: Do not power ON any of the IRAM/DRAM"]
    Off = 0,
    #[doc = "1: Power up all IRAM (32K) and DRAM (64K)"]
    On = 1,
}
impl From<Pwrendsp1ram> for bool {
    #[inline(always)]
    fn from(variant: Pwrendsp1ram) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENDSP1RAM` reader - Power up DSP1 IRAM and DRAM"]
pub type Pwrendsp1ramR = crate::BitReader<Pwrendsp1ram>;
impl Pwrendsp1ramR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrendsp1ram {
        match self.bits {
            false => Pwrendsp1ram::Off,
            true => Pwrendsp1ram::On,
        }
    }
    #[doc = "Do not power ON any of the IRAM/DRAM"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrendsp1ram::Off
    }
    #[doc = "Power up all IRAM (32K) and DRAM (64K)"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrendsp1ram::On
    }
}
#[doc = "Field `PWRENDSP1RAM` writer - Power up DSP1 IRAM and DRAM"]
pub type Pwrendsp1ramW<'a, REG> = crate::BitWriter<'a, REG, Pwrendsp1ram>;
impl<'a, REG> Pwrendsp1ramW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not power ON any of the IRAM/DRAM"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrendsp1ram::Off)
    }
    #[doc = "Power up all IRAM (32K) and DRAM (64K)"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrendsp1ram::On)
    }
}
#[doc = "Power up DSP1 ICACHE banks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrendsp1icache {
    #[doc = "0: Do not power up ICACHE"]
    Off = 0,
    #[doc = "1: Power up ICACHE"]
    On = 1,
}
impl From<Pwrendsp1icache> for bool {
    #[inline(always)]
    fn from(variant: Pwrendsp1icache) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENDSP1ICACHE` reader - Power up DSP1 ICACHE banks"]
pub type Pwrendsp1icacheR = crate::BitReader<Pwrendsp1icache>;
impl Pwrendsp1icacheR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrendsp1icache {
        match self.bits {
            false => Pwrendsp1icache::Off,
            true => Pwrendsp1icache::On,
        }
    }
    #[doc = "Do not power up ICACHE"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrendsp1icache::Off
    }
    #[doc = "Power up ICACHE"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrendsp1icache::On
    }
}
#[doc = "Field `PWRENDSP1ICACHE` writer - Power up DSP1 ICACHE banks"]
pub type Pwrendsp1icacheW<'a, REG> = crate::BitWriter<'a, REG, Pwrendsp1icache>;
impl<'a, REG> Pwrendsp1icacheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not power up ICACHE"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrendsp1icache::Off)
    }
    #[doc = "Power up ICACHE"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrendsp1icache::On)
    }
}
impl R {
    #[doc = "Bit 0 - Power up DSP1 IRAM and DRAM"]
    #[inline(always)]
    pub fn pwrendsp1ram(&self) -> Pwrendsp1ramR {
        Pwrendsp1ramR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power up DSP1 ICACHE banks"]
    #[inline(always)]
    pub fn pwrendsp1icache(&self) -> Pwrendsp1icacheR {
        Pwrendsp1icacheR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power up DSP1 IRAM and DRAM"]
    #[inline(always)]
    #[must_use]
    pub fn pwrendsp1ram(&mut self) -> Pwrendsp1ramW<Dsp1mempwrenSpec> {
        Pwrendsp1ramW::new(self, 0)
    }
    #[doc = "Bit 1 - Power up DSP1 ICACHE banks"]
    #[inline(always)]
    #[must_use]
    pub fn pwrendsp1icache(&mut self) -> Pwrendsp1icacheW<Dsp1mempwrenSpec> {
        Pwrendsp1icacheW::new(self, 1)
    }
}
#[doc = "This register enables the individual banks for the memories. When set, power will be enabled to the banks. This register works in conjunction with the DSP1MEMRETCFG register when DSP1 is OFF.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1mempwren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1mempwren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1mempwrenSpec;
impl crate::RegisterSpec for Dsp1mempwrenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1mempwren::R`](R) reader structure"]
impl crate::Readable for Dsp1mempwrenSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1mempwren::W`](W) writer structure"]
impl crate::Writable for Dsp1mempwrenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1MEMPWREN to value 0"]
impl crate::Resettable for Dsp1mempwrenSpec {
    const RESET_VALUE: u32 = 0;
}
