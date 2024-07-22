#[doc = "Register `LFRC` reader"]
pub type R = crate::R<LfrcSpec>;
#[doc = "Register `LFRC` writer"]
pub type W = crate::W<LfrcSpec>;
#[doc = "LFRC Software Override Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfrcswe {
    #[doc = "0: LFRC Software Override Disable."]
    OverrideDis = 0,
    #[doc = "1: LFRC Software Override Enable."]
    OverrideEn = 1,
}
impl From<Lfrcswe> for bool {
    #[inline(always)]
    fn from(variant: Lfrcswe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFRCSWE` reader - LFRC Software Override Enable."]
pub type LfrcsweR = crate::BitReader<Lfrcswe>;
impl LfrcsweR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfrcswe {
        match self.bits {
            false => Lfrcswe::OverrideDis,
            true => Lfrcswe::OverrideEn,
        }
    }
    #[doc = "LFRC Software Override Disable."]
    #[inline(always)]
    pub fn is_override_dis(&self) -> bool {
        *self == Lfrcswe::OverrideDis
    }
    #[doc = "LFRC Software Override Enable."]
    #[inline(always)]
    pub fn is_override_en(&self) -> bool {
        *self == Lfrcswe::OverrideEn
    }
}
#[doc = "Field `LFRCSWE` writer - LFRC Software Override Enable."]
pub type LfrcsweW<'a, REG> = crate::BitWriter<'a, REG, Lfrcswe>;
impl<'a, REG> LfrcsweW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LFRC Software Override Disable."]
    #[inline(always)]
    pub fn override_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Lfrcswe::OverrideDis)
    }
    #[doc = "LFRC Software Override Enable."]
    #[inline(always)]
    pub fn override_en(self) -> &'a mut crate::W<REG> {
        self.variant(Lfrcswe::OverrideEn)
    }
}
#[doc = "Field `TRIMTUNELFRC` reader - LFRC Frequency Tune trim bits"]
pub type TrimtunelfrcR = crate::FieldReader;
#[doc = "Field `TRIMTUNELFRC` writer - LFRC Frequency Tune trim bits"]
pub type TrimtunelfrcW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Power Down LFRC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwdlfrc {
    #[doc = "0: Power up LFRC."]
    Pwrup = 0,
    #[doc = "1: Power down LFRC."]
    Pwrdn = 1,
}
impl From<Pwdlfrc> for bool {
    #[inline(always)]
    fn from(variant: Pwdlfrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWDLFRC` reader - Power Down LFRC."]
pub type PwdlfrcR = crate::BitReader<Pwdlfrc>;
impl PwdlfrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwdlfrc {
        match self.bits {
            false => Pwdlfrc::Pwrup,
            true => Pwdlfrc::Pwrdn,
        }
    }
    #[doc = "Power up LFRC."]
    #[inline(always)]
    pub fn is_pwrup(&self) -> bool {
        *self == Pwdlfrc::Pwrup
    }
    #[doc = "Power down LFRC."]
    #[inline(always)]
    pub fn is_pwrdn(&self) -> bool {
        *self == Pwdlfrc::Pwrdn
    }
}
#[doc = "Field `PWDLFRC` writer - Power Down LFRC."]
pub type PwdlfrcW<'a, REG> = crate::BitWriter<'a, REG, Pwdlfrc>;
impl<'a, REG> PwdlfrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up LFRC."]
    #[inline(always)]
    pub fn pwrup(self) -> &'a mut crate::W<REG> {
        self.variant(Pwdlfrc::Pwrup)
    }
    #[doc = "Power down LFRC."]
    #[inline(always)]
    pub fn pwrdn(self) -> &'a mut crate::W<REG> {
        self.variant(Pwdlfrc::Pwrdn)
    }
}
#[doc = "LFRC Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resetlfrc {
    #[doc = "0: Enable LFRC."]
    En = 0,
    #[doc = "1: Reset LFRC."]
    Reset = 1,
}
impl From<Resetlfrc> for bool {
    #[inline(always)]
    fn from(variant: Resetlfrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESETLFRC` reader - LFRC Reset."]
pub type ResetlfrcR = crate::BitReader<Resetlfrc>;
impl ResetlfrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resetlfrc {
        match self.bits {
            false => Resetlfrc::En,
            true => Resetlfrc::Reset,
        }
    }
    #[doc = "Enable LFRC."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Resetlfrc::En
    }
    #[doc = "Reset LFRC."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Resetlfrc::Reset
    }
}
#[doc = "Field `RESETLFRC` writer - LFRC Reset."]
pub type ResetlfrcW<'a, REG> = crate::BitWriter<'a, REG, Resetlfrc>;
impl<'a, REG> ResetlfrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable LFRC."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Resetlfrc::En)
    }
    #[doc = "Reset LFRC."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Resetlfrc::Reset)
    }
}
#[doc = "Field `LFRCITAILTRIM` reader - LFRC ITAIL trim"]
pub type LfrcitailtrimR = crate::FieldReader;
#[doc = "Field `LFRCITAILTRIM` writer - LFRC ITAIL trim"]
pub type LfrcitailtrimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "SIMOBUCK LP mode clock divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lfrcsimoclkdiv {
    #[doc = "0: Divide by 1"]
    Div1 = 0,
    #[doc = "1: Divide by 2"]
    Div2 = 1,
    #[doc = "2: Divide by 4"]
    Div4 = 2,
    #[doc = "3: Divide by 8"]
    Div8 = 3,
    #[doc = "4: Divide by 16"]
    Div16 = 4,
    #[doc = "5: Divide by 32"]
    Div32 = 5,
}
impl From<Lfrcsimoclkdiv> for u8 {
    #[inline(always)]
    fn from(variant: Lfrcsimoclkdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lfrcsimoclkdiv {
    type Ux = u8;
}
impl crate::IsEnum for Lfrcsimoclkdiv {}
#[doc = "Field `LFRCSIMOCLKDIV` reader - SIMOBUCK LP mode clock divider"]
pub type LfrcsimoclkdivR = crate::FieldReader<Lfrcsimoclkdiv>;
impl LfrcsimoclkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lfrcsimoclkdiv> {
        match self.bits {
            0 => Some(Lfrcsimoclkdiv::Div1),
            1 => Some(Lfrcsimoclkdiv::Div2),
            2 => Some(Lfrcsimoclkdiv::Div4),
            3 => Some(Lfrcsimoclkdiv::Div8),
            4 => Some(Lfrcsimoclkdiv::Div16),
            5 => Some(Lfrcsimoclkdiv::Div32),
            _ => None,
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Lfrcsimoclkdiv::Div1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Lfrcsimoclkdiv::Div2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Lfrcsimoclkdiv::Div4
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Lfrcsimoclkdiv::Div8
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Lfrcsimoclkdiv::Div16
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Lfrcsimoclkdiv::Div32
    }
}
#[doc = "Field `LFRCSIMOCLKDIV` writer - SIMOBUCK LP mode clock divider"]
pub type LfrcsimoclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lfrcsimoclkdiv>;
impl<'a, REG> LfrcsimoclkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Lfrcsimoclkdiv::Div1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Lfrcsimoclkdiv::Div2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Lfrcsimoclkdiv::Div4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Lfrcsimoclkdiv::Div8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Lfrcsimoclkdiv::Div16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Lfrcsimoclkdiv::Div32)
    }
}
impl R {
    #[doc = "Bit 0 - LFRC Software Override Enable."]
    #[inline(always)]
    pub fn lfrcswe(&self) -> LfrcsweR {
        LfrcsweR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - LFRC Frequency Tune trim bits"]
    #[inline(always)]
    pub fn trimtunelfrc(&self) -> TrimtunelfrcR {
        TrimtunelfrcR::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Power Down LFRC."]
    #[inline(always)]
    pub fn pwdlfrc(&self) -> PwdlfrcR {
        PwdlfrcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LFRC Reset."]
    #[inline(always)]
    pub fn resetlfrc(&self) -> ResetlfrcR {
        ResetlfrcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - LFRC ITAIL trim"]
    #[inline(always)]
    pub fn lfrcitailtrim(&self) -> LfrcitailtrimR {
        LfrcitailtrimR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - SIMOBUCK LP mode clock divider"]
    #[inline(always)]
    pub fn lfrcsimoclkdiv(&self) -> LfrcsimoclkdivR {
        LfrcsimoclkdivR::new(((self.bits >> 10) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LFRC Software Override Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lfrcswe(&mut self) -> LfrcsweW<LfrcSpec> {
        LfrcsweW::new(self, 0)
    }
    #[doc = "Bits 1:5 - LFRC Frequency Tune trim bits"]
    #[inline(always)]
    #[must_use]
    pub fn trimtunelfrc(&mut self) -> TrimtunelfrcW<LfrcSpec> {
        TrimtunelfrcW::new(self, 1)
    }
    #[doc = "Bit 6 - Power Down LFRC."]
    #[inline(always)]
    #[must_use]
    pub fn pwdlfrc(&mut self) -> PwdlfrcW<LfrcSpec> {
        PwdlfrcW::new(self, 6)
    }
    #[doc = "Bit 7 - LFRC Reset."]
    #[inline(always)]
    #[must_use]
    pub fn resetlfrc(&mut self) -> ResetlfrcW<LfrcSpec> {
        ResetlfrcW::new(self, 7)
    }
    #[doc = "Bits 8:9 - LFRC ITAIL trim"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcitailtrim(&mut self) -> LfrcitailtrimW<LfrcSpec> {
        LfrcitailtrimW::new(self, 8)
    }
    #[doc = "Bits 10:12 - SIMOBUCK LP mode clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcsimoclkdiv(&mut self) -> LfrcsimoclkdivW<LfrcSpec> {
        LfrcsimoclkdivW::new(self, 10)
    }
}
#[doc = "LFRC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`lfrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfrcSpec;
impl crate::RegisterSpec for LfrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfrc::R`](R) reader structure"]
impl crate::Readable for LfrcSpec {}
#[doc = "`write(|w| ..)` method takes [`lfrc::W`](W) writer structure"]
impl crate::Writable for LfrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFRC to value 0x0120"]
impl crate::Resettable for LfrcSpec {
    const RESET_VALUE: u32 = 0x0120;
}
