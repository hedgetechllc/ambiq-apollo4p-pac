#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Indicates the power-status of the ADC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwdstat {
    #[doc = "0: Powered on."]
    On = 0,
    #[doc = "1: ADC Low Power Mode 1."]
    PoweredDown = 1,
}
impl From<Pwdstat> for bool {
    #[inline(always)]
    fn from(variant: Pwdstat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWDSTAT` reader - Indicates the power-status of the ADC."]
pub type PwdstatR = crate::BitReader<Pwdstat>;
impl PwdstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwdstat {
        match self.bits {
            false => Pwdstat::On,
            true => Pwdstat::PoweredDown,
        }
    }
    #[doc = "Powered on."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwdstat::On
    }
    #[doc = "ADC Low Power Mode 1."]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == Pwdstat::PoweredDown
    }
}
#[doc = "Field `PWDSTAT` writer - Indicates the power-status of the ADC."]
pub type PwdstatW<'a, REG> = crate::BitWriter<'a, REG, Pwdstat>;
impl<'a, REG> PwdstatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Powered on."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwdstat::On)
    }
    #[doc = "ADC Low Power Mode 1."]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut crate::W<REG> {
        self.variant(Pwdstat::PoweredDown)
    }
}
impl R {
    #[doc = "Bit 0 - Indicates the power-status of the ADC."]
    #[inline(always)]
    pub fn pwdstat(&self) -> PwdstatR {
        PwdstatR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates the power-status of the ADC."]
    #[inline(always)]
    #[must_use]
    pub fn pwdstat(&mut self) -> PwdstatW<StatSpec> {
        PwdstatW::new(self, 0)
    }
}
#[doc = "This register indicates the basic power status for the ADC. For detailed power status, see the power control power status register. ADC power mode 0 indicates the ADC is in its full power state and is ready to process scans. ADC Power mode 1 indicates the ADC enabled and in a low power state.\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
