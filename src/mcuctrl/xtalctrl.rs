#[doc = "Register `XTALCTRL` reader"]
pub type R = crate::R<XtalctrlSpec>;
#[doc = "Register `XTALCTRL` writer"]
pub type W = crate::W<XtalctrlSpec>;
#[doc = "XTAL Software Override Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xtalswe {
    #[doc = "0: XTAL Software Override Disable."]
    OverrideDis = 0,
    #[doc = "1: XTAL Software Override Enable."]
    OverrideEn = 1,
}
impl From<Xtalswe> for bool {
    #[inline(always)]
    fn from(variant: Xtalswe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTALSWE` reader - XTAL Software Override Enable."]
pub type XtalsweR = crate::BitReader<Xtalswe>;
impl XtalsweR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xtalswe {
        match self.bits {
            false => Xtalswe::OverrideDis,
            true => Xtalswe::OverrideEn,
        }
    }
    #[doc = "XTAL Software Override Disable."]
    #[inline(always)]
    pub fn is_override_dis(&self) -> bool {
        *self == Xtalswe::OverrideDis
    }
    #[doc = "XTAL Software Override Enable."]
    #[inline(always)]
    pub fn is_override_en(&self) -> bool {
        *self == Xtalswe::OverrideEn
    }
}
#[doc = "Field `XTALSWE` writer - XTAL Software Override Enable."]
pub type XtalsweW<'a, REG> = crate::BitWriter<'a, REG, Xtalswe>;
impl<'a, REG> XtalsweW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "XTAL Software Override Disable."]
    #[inline(always)]
    pub fn override_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Xtalswe::OverrideDis)
    }
    #[doc = "XTAL Software Override Enable."]
    #[inline(always)]
    pub fn override_en(self) -> &'a mut crate::W<REG> {
        self.variant(Xtalswe::OverrideEn)
    }
}
#[doc = "XTAL Oscillator Disable Feedback.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xtalcoredisfb {
    #[doc = "0: Enable XTAL oscillator comparator."]
    En = 0,
    #[doc = "1: Disable XTAL oscillator comparator."]
    Dis = 1,
}
impl From<Xtalcoredisfb> for bool {
    #[inline(always)]
    fn from(variant: Xtalcoredisfb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTALCOREDISFB` reader - XTAL Oscillator Disable Feedback."]
pub type XtalcoredisfbR = crate::BitReader<Xtalcoredisfb>;
impl XtalcoredisfbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xtalcoredisfb {
        match self.bits {
            false => Xtalcoredisfb::En,
            true => Xtalcoredisfb::Dis,
        }
    }
    #[doc = "Enable XTAL oscillator comparator."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Xtalcoredisfb::En
    }
    #[doc = "Disable XTAL oscillator comparator."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Xtalcoredisfb::Dis
    }
}
#[doc = "Field `XTALCOREDISFB` writer - XTAL Oscillator Disable Feedback."]
pub type XtalcoredisfbW<'a, REG> = crate::BitWriter<'a, REG, Xtalcoredisfb>;
impl<'a, REG> XtalcoredisfbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable XTAL oscillator comparator."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Xtalcoredisfb::En)
    }
    #[doc = "Disable XTAL oscillator comparator."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Xtalcoredisfb::Dis)
    }
}
#[doc = "XTAL Oscillator Bypass Comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xtalcompbypass {
    #[doc = "0: Use the XTAL oscillator comparator."]
    Usecomp = 0,
    #[doc = "1: Bypass the XTAL oscillator comparator."]
    Bypcomp = 1,
}
impl From<Xtalcompbypass> for bool {
    #[inline(always)]
    fn from(variant: Xtalcompbypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTALCOMPBYPASS` reader - XTAL Oscillator Bypass Comparator."]
pub type XtalcompbypassR = crate::BitReader<Xtalcompbypass>;
impl XtalcompbypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xtalcompbypass {
        match self.bits {
            false => Xtalcompbypass::Usecomp,
            true => Xtalcompbypass::Bypcomp,
        }
    }
    #[doc = "Use the XTAL oscillator comparator."]
    #[inline(always)]
    pub fn is_usecomp(&self) -> bool {
        *self == Xtalcompbypass::Usecomp
    }
    #[doc = "Bypass the XTAL oscillator comparator."]
    #[inline(always)]
    pub fn is_bypcomp(&self) -> bool {
        *self == Xtalcompbypass::Bypcomp
    }
}
#[doc = "Field `XTALCOMPBYPASS` writer - XTAL Oscillator Bypass Comparator."]
pub type XtalcompbypassW<'a, REG> = crate::BitWriter<'a, REG, Xtalcompbypass>;
impl<'a, REG> XtalcompbypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use the XTAL oscillator comparator."]
    #[inline(always)]
    pub fn usecomp(self) -> &'a mut crate::W<REG> {
        self.variant(Xtalcompbypass::Usecomp)
    }
    #[doc = "Bypass the XTAL oscillator comparator."]
    #[inline(always)]
    pub fn bypcomp(self) -> &'a mut crate::W<REG> {
        self.variant(Xtalcompbypass::Bypcomp)
    }
}
#[doc = "XTAL Oscillator Power Down Core.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xtalpdnb {
    #[doc = "1: Power up XTAL oscillator core."]
    Pwrupcore = 1,
    #[doc = "0: Power down XTAL oscillator core."]
    Pwrdncore = 0,
}
impl From<Xtalpdnb> for bool {
    #[inline(always)]
    fn from(variant: Xtalpdnb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTALPDNB` reader - XTAL Oscillator Power Down Core."]
pub type XtalpdnbR = crate::BitReader<Xtalpdnb>;
impl XtalpdnbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xtalpdnb {
        match self.bits {
            true => Xtalpdnb::Pwrupcore,
            false => Xtalpdnb::Pwrdncore,
        }
    }
    #[doc = "Power up XTAL oscillator core."]
    #[inline(always)]
    pub fn is_pwrupcore(&self) -> bool {
        *self == Xtalpdnb::Pwrupcore
    }
    #[doc = "Power down XTAL oscillator core."]
    #[inline(always)]
    pub fn is_pwrdncore(&self) -> bool {
        *self == Xtalpdnb::Pwrdncore
    }
}
#[doc = "Field `XTALPDNB` writer - XTAL Oscillator Power Down Core."]
pub type XtalpdnbW<'a, REG> = crate::BitWriter<'a, REG, Xtalpdnb>;
impl<'a, REG> XtalpdnbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up XTAL oscillator core."]
    #[inline(always)]
    pub fn pwrupcore(self) -> &'a mut crate::W<REG> {
        self.variant(Xtalpdnb::Pwrupcore)
    }
    #[doc = "Power down XTAL oscillator core."]
    #[inline(always)]
    pub fn pwrdncore(self) -> &'a mut crate::W<REG> {
        self.variant(Xtalpdnb::Pwrdncore)
    }
}
#[doc = "XTAL Oscillator Power Down Comparator.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xtalcomppdnb {
    #[doc = "1: Power up XTAL oscillator comparator."]
    Pwrupcomp = 1,
    #[doc = "0: Power down XTAL oscillator comparator."]
    Pwrdncomp = 0,
}
impl From<Xtalcomppdnb> for bool {
    #[inline(always)]
    fn from(variant: Xtalcomppdnb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTALCOMPPDNB` reader - XTAL Oscillator Power Down Comparator."]
pub type XtalcomppdnbR = crate::BitReader<Xtalcomppdnb>;
impl XtalcomppdnbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xtalcomppdnb {
        match self.bits {
            true => Xtalcomppdnb::Pwrupcomp,
            false => Xtalcomppdnb::Pwrdncomp,
        }
    }
    #[doc = "Power up XTAL oscillator comparator."]
    #[inline(always)]
    pub fn is_pwrupcomp(&self) -> bool {
        *self == Xtalcomppdnb::Pwrupcomp
    }
    #[doc = "Power down XTAL oscillator comparator."]
    #[inline(always)]
    pub fn is_pwrdncomp(&self) -> bool {
        *self == Xtalcomppdnb::Pwrdncomp
    }
}
#[doc = "Field `XTALCOMPPDNB` writer - XTAL Oscillator Power Down Comparator."]
pub type XtalcomppdnbW<'a, REG> = crate::BitWriter<'a, REG, Xtalcomppdnb>;
impl<'a, REG> XtalcomppdnbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up XTAL oscillator comparator."]
    #[inline(always)]
    pub fn pwrupcomp(self) -> &'a mut crate::W<REG> {
        self.variant(Xtalcomppdnb::Pwrupcomp)
    }
    #[doc = "Power down XTAL oscillator comparator."]
    #[inline(always)]
    pub fn pwrdncomp(self) -> &'a mut crate::W<REG> {
        self.variant(Xtalcomppdnb::Pwrdncomp)
    }
}
#[doc = "Field `XTALIBUFTRIM` reader - XTAL IBUFF trim"]
pub type XtalibuftrimR = crate::FieldReader;
#[doc = "Field `XTALIBUFTRIM` writer - XTAL IBUFF trim"]
pub type XtalibuftrimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XTALICOMPTRIM` reader - XTAL ICOMP trim"]
pub type XtalicomptrimR = crate::FieldReader;
#[doc = "Field `XTALICOMPTRIM` writer - XTAL ICOMP trim"]
pub type XtalicomptrimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - XTAL Software Override Enable."]
    #[inline(always)]
    pub fn xtalswe(&self) -> XtalsweR {
        XtalsweR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XTAL Oscillator Disable Feedback."]
    #[inline(always)]
    pub fn xtalcoredisfb(&self) -> XtalcoredisfbR {
        XtalcoredisfbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XTAL Oscillator Bypass Comparator."]
    #[inline(always)]
    pub fn xtalcompbypass(&self) -> XtalcompbypassR {
        XtalcompbypassR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - XTAL Oscillator Power Down Core."]
    #[inline(always)]
    pub fn xtalpdnb(&self) -> XtalpdnbR {
        XtalpdnbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - XTAL Oscillator Power Down Comparator."]
    #[inline(always)]
    pub fn xtalcomppdnb(&self) -> XtalcomppdnbR {
        XtalcomppdnbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - XTAL IBUFF trim"]
    #[inline(always)]
    pub fn xtalibuftrim(&self) -> XtalibuftrimR {
        XtalibuftrimR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8 - XTAL ICOMP trim"]
    #[inline(always)]
    pub fn xtalicomptrim(&self) -> XtalicomptrimR {
        XtalicomptrimR::new(((self.bits >> 7) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - XTAL Software Override Enable."]
    #[inline(always)]
    #[must_use]
    pub fn xtalswe(&mut self) -> XtalsweW<XtalctrlSpec> {
        XtalsweW::new(self, 0)
    }
    #[doc = "Bit 1 - XTAL Oscillator Disable Feedback."]
    #[inline(always)]
    #[must_use]
    pub fn xtalcoredisfb(&mut self) -> XtalcoredisfbW<XtalctrlSpec> {
        XtalcoredisfbW::new(self, 1)
    }
    #[doc = "Bit 2 - XTAL Oscillator Bypass Comparator."]
    #[inline(always)]
    #[must_use]
    pub fn xtalcompbypass(&mut self) -> XtalcompbypassW<XtalctrlSpec> {
        XtalcompbypassW::new(self, 2)
    }
    #[doc = "Bit 3 - XTAL Oscillator Power Down Core."]
    #[inline(always)]
    #[must_use]
    pub fn xtalpdnb(&mut self) -> XtalpdnbW<XtalctrlSpec> {
        XtalpdnbW::new(self, 3)
    }
    #[doc = "Bit 4 - XTAL Oscillator Power Down Comparator."]
    #[inline(always)]
    #[must_use]
    pub fn xtalcomppdnb(&mut self) -> XtalcomppdnbW<XtalctrlSpec> {
        XtalcomppdnbW::new(self, 4)
    }
    #[doc = "Bits 5:6 - XTAL IBUFF trim"]
    #[inline(always)]
    #[must_use]
    pub fn xtalibuftrim(&mut self) -> XtalibuftrimW<XtalctrlSpec> {
        XtalibuftrimW::new(self, 5)
    }
    #[doc = "Bits 7:8 - XTAL ICOMP trim"]
    #[inline(always)]
    #[must_use]
    pub fn xtalicomptrim(&mut self) -> XtalicomptrimW<XtalctrlSpec> {
        XtalicomptrimW::new(self, 7)
    }
}
#[doc = "XTAL Oscillator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalctrlSpec;
impl crate::RegisterSpec for XtalctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtalctrl::R`](R) reader structure"]
impl crate::Readable for XtalctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`xtalctrl::W`](W) writer structure"]
impl crate::Writable for XtalctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XTALCTRL to value 0x01b8"]
impl crate::Resettable for XtalctrlSpec {
    const RESET_VALUE: u32 = 0x01b8;
}
