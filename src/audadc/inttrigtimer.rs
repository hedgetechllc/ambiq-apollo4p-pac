#[doc = "Register `INTTRIGTIMER` reader"]
pub type R = crate::R<InttrigtimerSpec>;
#[doc = "Register `INTTRIGTIMER` writer"]
pub type W = crate::W<InttrigtimerSpec>;
#[doc = "Field `TIMERMAX` reader - Trigger counter count max, used as initial condition to trigger. Also used repeatedly each time counter reaches it to restart trigger timer at zero. To update this value, first disable the INTTRIGTIMER by setting TIMEREN to DIS, change TIMERMAX, and then reenable it INTTRIGTIMER by setting TIMEREN to EN again."]
pub type TimermaxR = crate::FieldReader<u16>;
#[doc = "Field `TIMERMAX` writer - Trigger counter count max, used as initial condition to trigger. Also used repeatedly each time counter reaches it to restart trigger timer at zero. To update this value, first disable the INTTRIGTIMER by setting TIMEREN to DIS, change TIMERMAX, and then reenable it INTTRIGTIMER by setting TIMEREN to EN again."]
pub type TimermaxW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CLKDIV` reader - Configure number of divide-by-2 of clock source as input to trigger counter. (Max value of 5.) A value of 0 in this register would not divide down the AUDADC input clock. A value of 1 would divide the AUDADC input clock frequency by 2. A value of 5 would divide the AUDADC input clock frequency by 2^5 = 32. To update this value, first disable the INTTRIGTIMER by setting TIMEREN to DIS, change CLKDIV, and then reenable it INTTRIGTIMER by setting TIMEREN to EN again."]
pub type ClkdivR = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Configure number of divide-by-2 of clock source as input to trigger counter. (Max value of 5.) A value of 0 in this register would not divide down the AUDADC input clock. A value of 1 would divide the AUDADC input clock frequency by 2. A value of 5 would divide the AUDADC input clock frequency by 2^5 = 32. To update this value, first disable the INTTRIGTIMER by setting TIMEREN to DIS, change CLKDIV, and then reenable it INTTRIGTIMER by setting TIMEREN to EN again."]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "AUDADC-internal trigger timer enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timeren {
    #[doc = "0: Disable the AUDADC-internal trigger timer."]
    Dis = 0,
    #[doc = "1: Enable the AUDADC-internal trigger timer."]
    En = 1,
}
impl From<Timeren> for bool {
    #[inline(always)]
    fn from(variant: Timeren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEREN` reader - AUDADC-internal trigger timer enable."]
pub type TimerenR = crate::BitReader<Timeren>;
impl TimerenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timeren {
        match self.bits {
            false => Timeren::Dis,
            true => Timeren::En,
        }
    }
    #[doc = "Disable the AUDADC-internal trigger timer."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Timeren::Dis
    }
    #[doc = "Enable the AUDADC-internal trigger timer."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Timeren::En
    }
}
#[doc = "Field `TIMEREN` writer - AUDADC-internal trigger timer enable."]
pub type TimerenW<'a, REG> = crate::BitWriter<'a, REG, Timeren>;
impl<'a, REG> TimerenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the AUDADC-internal trigger timer."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Timeren::Dis)
    }
    #[doc = "Enable the AUDADC-internal trigger timer."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Timeren::En)
    }
}
impl R {
    #[doc = "Bits 0:9 - Trigger counter count max, used as initial condition to trigger. Also used repeatedly each time counter reaches it to restart trigger timer at zero. To update this value, first disable the INTTRIGTIMER by setting TIMEREN to DIS, change TIMERMAX, and then reenable it INTTRIGTIMER by setting TIMEREN to EN again."]
    #[inline(always)]
    pub fn timermax(&self) -> TimermaxR {
        TimermaxR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:18 - Configure number of divide-by-2 of clock source as input to trigger counter. (Max value of 5.) A value of 0 in this register would not divide down the AUDADC input clock. A value of 1 would divide the AUDADC input clock frequency by 2. A value of 5 would divide the AUDADC input clock frequency by 2^5 = 32. To update this value, first disable the INTTRIGTIMER by setting TIMEREN to DIS, change CLKDIV, and then reenable it INTTRIGTIMER by setting TIMEREN to EN again."]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 31 - AUDADC-internal trigger timer enable."]
    #[inline(always)]
    pub fn timeren(&self) -> TimerenR {
        TimerenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Trigger counter count max, used as initial condition to trigger. Also used repeatedly each time counter reaches it to restart trigger timer at zero. To update this value, first disable the INTTRIGTIMER by setting TIMEREN to DIS, change TIMERMAX, and then reenable it INTTRIGTIMER by setting TIMEREN to EN again."]
    #[inline(always)]
    #[must_use]
    pub fn timermax(&mut self) -> TimermaxW<InttrigtimerSpec> {
        TimermaxW::new(self, 0)
    }
    #[doc = "Bits 16:18 - Configure number of divide-by-2 of clock source as input to trigger counter. (Max value of 5.) A value of 0 in this register would not divide down the AUDADC input clock. A value of 1 would divide the AUDADC input clock frequency by 2. A value of 5 would divide the AUDADC input clock frequency by 2^5 = 32. To update this value, first disable the INTTRIGTIMER by setting TIMEREN to DIS, change CLKDIV, and then reenable it INTTRIGTIMER by setting TIMEREN to EN again."]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> ClkdivW<InttrigtimerSpec> {
        ClkdivW::new(self, 16)
    }
    #[doc = "Bit 31 - AUDADC-internal trigger timer enable."]
    #[inline(always)]
    #[must_use]
    pub fn timeren(&mut self) -> TimerenW<InttrigtimerSpec> {
        TimerenW::new(self, 31)
    }
}
#[doc = "AUDADC-Internal Repeating Trigger Timer Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`inttrigtimer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inttrigtimer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InttrigtimerSpec;
impl crate::RegisterSpec for InttrigtimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inttrigtimer::R`](R) reader structure"]
impl crate::Readable for InttrigtimerSpec {}
#[doc = "`write(|w| ..)` method takes [`inttrigtimer::W`](W) writer structure"]
impl crate::Writable for InttrigtimerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTTRIGTIMER to value 0"]
impl crate::Resettable for InttrigtimerSpec {
    const RESET_VALUE: u32 = 0;
}
