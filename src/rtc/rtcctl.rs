#[doc = "Register `RTCCTL` reader"]
pub type R = crate::R<RtcctlSpec>;
#[doc = "Register `RTCCTL` writer"]
pub type W = crate::W<RtcctlSpec>;
#[doc = "Counter write control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrtc {
    #[doc = "0: Counter writes are disabled"]
    Dis = 0,
    #[doc = "1: Counter writes are enabled"]
    En = 1,
}
impl From<Wrtc> for bool {
    #[inline(always)]
    fn from(variant: Wrtc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRTC` reader - Counter write control"]
pub type WrtcR = crate::BitReader<Wrtc>;
impl WrtcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wrtc {
        match self.bits {
            false => Wrtc::Dis,
            true => Wrtc::En,
        }
    }
    #[doc = "Counter writes are disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wrtc::Dis
    }
    #[doc = "Counter writes are enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wrtc::En
    }
}
#[doc = "Field `WRTC` writer - Counter write control"]
pub type WrtcW<'a, REG> = crate::BitWriter<'a, REG, Wrtc>;
impl<'a, REG> WrtcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter writes are disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Wrtc::Dis)
    }
    #[doc = "Counter writes are enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Wrtc::En)
    }
}
#[doc = "Alarm repeat interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rpt {
    #[doc = "0: Alarm interrupt disabled"]
    Dis = 0,
    #[doc = "1: Interrupt every year"]
    Year = 1,
    #[doc = "2: Interrupt every month"]
    Month = 2,
    #[doc = "3: Interrupt every week"]
    Week = 3,
    #[doc = "4: Interrupt every day"]
    Day = 4,
    #[doc = "5: Interrupt every hour"]
    Hr = 5,
    #[doc = "6: Interrupt every minute"]
    Min = 6,
    #[doc = "7: Interrupt every second/10th/100th"]
    Sec = 7,
}
impl From<Rpt> for u8 {
    #[inline(always)]
    fn from(variant: Rpt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rpt {
    type Ux = u8;
}
impl crate::IsEnum for Rpt {}
#[doc = "Field `RPT` reader - Alarm repeat interval"]
pub type RptR = crate::FieldReader<Rpt>;
impl RptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rpt {
        match self.bits {
            0 => Rpt::Dis,
            1 => Rpt::Year,
            2 => Rpt::Month,
            3 => Rpt::Week,
            4 => Rpt::Day,
            5 => Rpt::Hr,
            6 => Rpt::Min,
            7 => Rpt::Sec,
            _ => unreachable!(),
        }
    }
    #[doc = "Alarm interrupt disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rpt::Dis
    }
    #[doc = "Interrupt every year"]
    #[inline(always)]
    pub fn is_year(&self) -> bool {
        *self == Rpt::Year
    }
    #[doc = "Interrupt every month"]
    #[inline(always)]
    pub fn is_month(&self) -> bool {
        *self == Rpt::Month
    }
    #[doc = "Interrupt every week"]
    #[inline(always)]
    pub fn is_week(&self) -> bool {
        *self == Rpt::Week
    }
    #[doc = "Interrupt every day"]
    #[inline(always)]
    pub fn is_day(&self) -> bool {
        *self == Rpt::Day
    }
    #[doc = "Interrupt every hour"]
    #[inline(always)]
    pub fn is_hr(&self) -> bool {
        *self == Rpt::Hr
    }
    #[doc = "Interrupt every minute"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == Rpt::Min
    }
    #[doc = "Interrupt every second/10th/100th"]
    #[inline(always)]
    pub fn is_sec(&self) -> bool {
        *self == Rpt::Sec
    }
}
#[doc = "Field `RPT` writer - Alarm repeat interval"]
pub type RptW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rpt, crate::Safe>;
impl<'a, REG> RptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alarm interrupt disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rpt::Dis)
    }
    #[doc = "Interrupt every year"]
    #[inline(always)]
    pub fn year(self) -> &'a mut crate::W<REG> {
        self.variant(Rpt::Year)
    }
    #[doc = "Interrupt every month"]
    #[inline(always)]
    pub fn month(self) -> &'a mut crate::W<REG> {
        self.variant(Rpt::Month)
    }
    #[doc = "Interrupt every week"]
    #[inline(always)]
    pub fn week(self) -> &'a mut crate::W<REG> {
        self.variant(Rpt::Week)
    }
    #[doc = "Interrupt every day"]
    #[inline(always)]
    pub fn day(self) -> &'a mut crate::W<REG> {
        self.variant(Rpt::Day)
    }
    #[doc = "Interrupt every hour"]
    #[inline(always)]
    pub fn hr(self) -> &'a mut crate::W<REG> {
        self.variant(Rpt::Hr)
    }
    #[doc = "Interrupt every minute"]
    #[inline(always)]
    pub fn min(self) -> &'a mut crate::W<REG> {
        self.variant(Rpt::Min)
    }
    #[doc = "Interrupt every second/10th/100th"]
    #[inline(always)]
    pub fn sec(self) -> &'a mut crate::W<REG> {
        self.variant(Rpt::Sec)
    }
}
#[doc = "RTC input clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rstop {
    #[doc = "0: Allow the RTC input clock to run"]
    Run = 0,
    #[doc = "1: Stop the RTC input clock"]
    Stop = 1,
}
impl From<Rstop> for bool {
    #[inline(always)]
    fn from(variant: Rstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTOP` reader - RTC input clock control"]
pub type RstopR = crate::BitReader<Rstop>;
impl RstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rstop {
        match self.bits {
            false => Rstop::Run,
            true => Rstop::Stop,
        }
    }
    #[doc = "Allow the RTC input clock to run"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Rstop::Run
    }
    #[doc = "Stop the RTC input clock"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Rstop::Stop
    }
}
#[doc = "Field `RSTOP` writer - RTC input clock control"]
pub type RstopW<'a, REG> = crate::BitWriter<'a, REG, Rstop>;
impl<'a, REG> RstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Allow the RTC input clock to run"]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(Rstop::Run)
    }
    #[doc = "Stop the RTC input clock"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Rstop::Stop)
    }
}
#[doc = "Hours Counter mode Only 24HR mode supported\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hr1224 {
    #[doc = "0: Hours in 24 hour mode"]
    _24hr = 0,
    #[doc = "1: Disable the 24 hour mode"]
    Disabled = 1,
}
impl From<Hr1224> for bool {
    #[inline(always)]
    fn from(variant: Hr1224) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HR1224` reader - Hours Counter mode Only 24HR mode supported"]
pub type Hr1224R = crate::BitReader<Hr1224>;
impl Hr1224R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hr1224 {
        match self.bits {
            false => Hr1224::_24hr,
            true => Hr1224::Disabled,
        }
    }
    #[doc = "Hours in 24 hour mode"]
    #[inline(always)]
    pub fn is_24hr(&self) -> bool {
        *self == Hr1224::_24hr
    }
    #[doc = "Disable the 24 hour mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Hr1224::Disabled
    }
}
#[doc = "Field `HR1224` writer - Hours Counter mode Only 24HR mode supported"]
pub type Hr1224W<'a, REG> = crate::BitWriter<'a, REG, Hr1224>;
impl<'a, REG> Hr1224W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hours in 24 hour mode"]
    #[inline(always)]
    pub fn _24hr(self) -> &'a mut crate::W<REG> {
        self.variant(Hr1224::_24hr)
    }
    #[doc = "Disable the 24 hour mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hr1224::Disabled)
    }
}
impl R {
    #[doc = "Bit 0 - Counter write control"]
    #[inline(always)]
    pub fn wrtc(&self) -> WrtcR {
        WrtcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Alarm repeat interval"]
    #[inline(always)]
    pub fn rpt(&self) -> RptR {
        RptR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - RTC input clock control"]
    #[inline(always)]
    pub fn rstop(&self) -> RstopR {
        RstopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hours Counter mode Only 24HR mode supported"]
    #[inline(always)]
    pub fn hr1224(&self) -> Hr1224R {
        Hr1224R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter write control"]
    #[inline(always)]
    #[must_use]
    pub fn wrtc(&mut self) -> WrtcW<RtcctlSpec> {
        WrtcW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Alarm repeat interval"]
    #[inline(always)]
    #[must_use]
    pub fn rpt(&mut self) -> RptW<RtcctlSpec> {
        RptW::new(self, 1)
    }
    #[doc = "Bit 4 - RTC input clock control"]
    #[inline(always)]
    #[must_use]
    pub fn rstop(&mut self) -> RstopW<RtcctlSpec> {
        RstopW::new(self, 4)
    }
    #[doc = "Bit 5 - Hours Counter mode Only 24HR mode supported"]
    #[inline(always)]
    #[must_use]
    pub fn hr1224(&mut self) -> Hr1224W<RtcctlSpec> {
        Hr1224W::new(self, 5)
    }
}
#[doc = "This is the register control for the RTC module. It enables counter writes and sets the alarm repeat interval.\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcctlSpec;
impl crate::RegisterSpec for RtcctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtcctl::R`](R) reader structure"]
impl crate::Readable for RtcctlSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcctl::W`](W) writer structure"]
impl crate::Writable for RtcctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTCCTL to value 0"]
impl crate::Resettable for RtcctlSpec {
    const RESET_VALUE: u32 = 0;
}
