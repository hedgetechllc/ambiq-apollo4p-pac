#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "This bit is 1 if the positive input of the comparator is greater than the negative input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpout {
    #[doc = "0: The negative input of the comparator is greater than the positive input."]
    VoutLow = 0,
    #[doc = "1: The positive input of the comparator is greater than the negative input."]
    VoutHigh = 1,
}
impl From<Cmpout> for bool {
    #[inline(always)]
    fn from(variant: Cmpout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPOUT` reader - This bit is 1 if the positive input of the comparator is greater than the negative input."]
pub type CmpoutR = crate::BitReader<Cmpout>;
impl CmpoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpout {
        match self.bits {
            false => Cmpout::VoutLow,
            true => Cmpout::VoutHigh,
        }
    }
    #[doc = "The negative input of the comparator is greater than the positive input."]
    #[inline(always)]
    pub fn is_vout_low(&self) -> bool {
        *self == Cmpout::VoutLow
    }
    #[doc = "The positive input of the comparator is greater than the negative input."]
    #[inline(always)]
    pub fn is_vout_high(&self) -> bool {
        *self == Cmpout::VoutHigh
    }
}
#[doc = "Field `CMPOUT` writer - This bit is 1 if the positive input of the comparator is greater than the negative input."]
pub type CmpoutW<'a, REG> = crate::BitWriter<'a, REG, Cmpout>;
impl<'a, REG> CmpoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The negative input of the comparator is greater than the positive input."]
    #[inline(always)]
    pub fn vout_low(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpout::VoutLow)
    }
    #[doc = "The positive input of the comparator is greater than the negative input."]
    #[inline(always)]
    pub fn vout_high(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpout::VoutHigh)
    }
}
#[doc = "This bit indicates the power down state of the voltage comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwdstat {
    #[doc = "1: The voltage comparator is powered down."]
    PoweredDown = 1,
    #[doc = "0: The voltage comparator is powered up."]
    PoweredUp = 0,
}
impl From<Pwdstat> for bool {
    #[inline(always)]
    fn from(variant: Pwdstat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWDSTAT` reader - This bit indicates the power down state of the voltage comparator."]
pub type PwdstatR = crate::BitReader<Pwdstat>;
impl PwdstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwdstat {
        match self.bits {
            true => Pwdstat::PoweredDown,
            false => Pwdstat::PoweredUp,
        }
    }
    #[doc = "The voltage comparator is powered down."]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == Pwdstat::PoweredDown
    }
    #[doc = "The voltage comparator is powered up."]
    #[inline(always)]
    pub fn is_powered_up(&self) -> bool {
        *self == Pwdstat::PoweredUp
    }
}
#[doc = "Field `PWDSTAT` writer - This bit indicates the power down state of the voltage comparator."]
pub type PwdstatW<'a, REG> = crate::BitWriter<'a, REG, Pwdstat>;
impl<'a, REG> PwdstatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The voltage comparator is powered down."]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut crate::W<REG> {
        self.variant(Pwdstat::PoweredDown)
    }
    #[doc = "The voltage comparator is powered up."]
    #[inline(always)]
    pub fn powered_up(self) -> &'a mut crate::W<REG> {
        self.variant(Pwdstat::PoweredUp)
    }
}
impl R {
    #[doc = "Bit 0 - This bit is 1 if the positive input of the comparator is greater than the negative input."]
    #[inline(always)]
    pub fn cmpout(&self) -> CmpoutR {
        CmpoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit indicates the power down state of the voltage comparator."]
    #[inline(always)]
    pub fn pwdstat(&self) -> PwdstatR {
        PwdstatR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is 1 if the positive input of the comparator is greater than the negative input."]
    #[inline(always)]
    #[must_use]
    pub fn cmpout(&mut self) -> CmpoutW<StatSpec> {
        CmpoutW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit indicates the power down state of the voltage comparator."]
    #[inline(always)]
    #[must_use]
    pub fn pwdstat(&mut self) -> PwdstatW<StatSpec> {
        PwdstatW::new(self, 1)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}
