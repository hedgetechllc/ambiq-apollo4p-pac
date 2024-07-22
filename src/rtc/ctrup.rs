#[doc = "Register `CTRUP` reader"]
pub type R = crate::R<CtrupSpec>;
#[doc = "Register `CTRUP` writer"]
pub type W = crate::W<CtrupSpec>;
#[doc = "Field `CTRDATE` reader - Date Counter. Contains the current value for date."]
pub type CtrdateR = crate::FieldReader;
#[doc = "Field `CTRDATE` writer - Date Counter. Contains the current value for date."]
pub type CtrdateW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CTRMO` reader - Months Counter. Contains the current value for months."]
pub type CtrmoR = crate::FieldReader;
#[doc = "Field `CTRMO` writer - Months Counter. Contains the current value for months."]
pub type CtrmoW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CTRYR` reader - Years Counter. Contains the current value for year."]
pub type CtryrR = crate::FieldReader;
#[doc = "Field `CTRYR` writer - Years Counter. Contains the current value for year."]
pub type CtryrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CTRWKDY` reader - Weekdays Counter. Contains the current value for weekdays."]
pub type CtrwkdyR = crate::FieldReader;
#[doc = "Field `CTRWKDY` writer - Weekdays Counter. Contains the current value for weekdays."]
pub type CtrwkdyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Century Bit. CB == 0 assumes the century is 21xx, and CB == 1 assumes it is 20xx.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cb {
    #[doc = "0: Century is 2100s"]
    _2100 = 0,
    #[doc = "1: Century is 2000s"]
    _2000 = 1,
}
impl From<Cb> for bool {
    #[inline(always)]
    fn from(variant: Cb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CB` reader - Century Bit. CB == 0 assumes the century is 21xx, and CB == 1 assumes it is 20xx."]
pub type CbR = crate::BitReader<Cb>;
impl CbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cb {
        match self.bits {
            false => Cb::_2100,
            true => Cb::_2000,
        }
    }
    #[doc = "Century is 2100s"]
    #[inline(always)]
    pub fn is_2100(&self) -> bool {
        *self == Cb::_2100
    }
    #[doc = "Century is 2000s"]
    #[inline(always)]
    pub fn is_2000(&self) -> bool {
        *self == Cb::_2000
    }
}
#[doc = "Field `CB` writer - Century Bit. CB == 0 assumes the century is 21xx, and CB == 1 assumes it is 20xx."]
pub type CbW<'a, REG> = crate::BitWriter<'a, REG, Cb>;
impl<'a, REG> CbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Century is 2100s"]
    #[inline(always)]
    pub fn _2100(self) -> &'a mut crate::W<REG> {
        self.variant(Cb::_2100)
    }
    #[doc = "Century is 2000s"]
    #[inline(always)]
    pub fn _2000(self) -> &'a mut crate::W<REG> {
        self.variant(Cb::_2000)
    }
}
#[doc = "Field `CEB` reader - Century Enable Bit. ERR129 - RTC cannot perform rollover. Century Bit toggles when CEB == 1 and Year == 99 and hence cannot handle Year rollover from xx99 -> xx00"]
pub type CebR = crate::BitReader;
#[doc = "Field `CEB` writer - Century Enable Bit. ERR129 - RTC cannot perform rollover. Century Bit toggles when CEB == 1 and Year == 99 and hence cannot handle Year rollover from xx99 -> xx00"]
pub type CebW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Counter read error status. Error is triggered when software reads the lower word of the counters, and fails to read the upper counter within 1/100 second. This is because when the lower counter is read, the upper counter is held off from incrementing until it is read so that the full time stamp can be read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cterr {
    #[doc = "0: No read error occurred"]
    Noerr = 0,
    #[doc = "1: Read error occurred"]
    Rderr = 1,
}
impl From<Cterr> for bool {
    #[inline(always)]
    fn from(variant: Cterr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTERR` reader - Counter read error status. Error is triggered when software reads the lower word of the counters, and fails to read the upper counter within 1/100 second. This is because when the lower counter is read, the upper counter is held off from incrementing until it is read so that the full time stamp can be read."]
pub type CterrR = crate::BitReader<Cterr>;
impl CterrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cterr {
        match self.bits {
            false => Cterr::Noerr,
            true => Cterr::Rderr,
        }
    }
    #[doc = "No read error occurred"]
    #[inline(always)]
    pub fn is_noerr(&self) -> bool {
        *self == Cterr::Noerr
    }
    #[doc = "Read error occurred"]
    #[inline(always)]
    pub fn is_rderr(&self) -> bool {
        *self == Cterr::Rderr
    }
}
#[doc = "Field `CTERR` writer - Counter read error status. Error is triggered when software reads the lower word of the counters, and fails to read the upper counter within 1/100 second. This is because when the lower counter is read, the upper counter is held off from incrementing until it is read so that the full time stamp can be read."]
pub type CterrW<'a, REG> = crate::BitWriter<'a, REG, Cterr>;
impl<'a, REG> CterrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read error occurred"]
    #[inline(always)]
    pub fn noerr(self) -> &'a mut crate::W<REG> {
        self.variant(Cterr::Noerr)
    }
    #[doc = "Read error occurred"]
    #[inline(always)]
    pub fn rderr(self) -> &'a mut crate::W<REG> {
        self.variant(Cterr::Rderr)
    }
}
impl R {
    #[doc = "Bits 0:5 - Date Counter. Contains the current value for date."]
    #[inline(always)]
    pub fn ctrdate(&self) -> CtrdateR {
        CtrdateR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Months Counter. Contains the current value for months."]
    #[inline(always)]
    pub fn ctrmo(&self) -> CtrmoR {
        CtrmoR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - Years Counter. Contains the current value for year."]
    #[inline(always)]
    pub fn ctryr(&self) -> CtryrR {
        CtryrR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - Weekdays Counter. Contains the current value for weekdays."]
    #[inline(always)]
    pub fn ctrwkdy(&self) -> CtrwkdyR {
        CtrwkdyR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - Century Bit. CB == 0 assumes the century is 21xx, and CB == 1 assumes it is 20xx."]
    #[inline(always)]
    pub fn cb(&self) -> CbR {
        CbR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Century Enable Bit. ERR129 - RTC cannot perform rollover. Century Bit toggles when CEB == 1 and Year == 99 and hence cannot handle Year rollover from xx99 -> xx00"]
    #[inline(always)]
    pub fn ceb(&self) -> CebR {
        CebR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Counter read error status. Error is triggered when software reads the lower word of the counters, and fails to read the upper counter within 1/100 second. This is because when the lower counter is read, the upper counter is held off from incrementing until it is read so that the full time stamp can be read."]
    #[inline(always)]
    pub fn cterr(&self) -> CterrR {
        CterrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Date Counter. Contains the current value for date."]
    #[inline(always)]
    #[must_use]
    pub fn ctrdate(&mut self) -> CtrdateW<CtrupSpec> {
        CtrdateW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Months Counter. Contains the current value for months."]
    #[inline(always)]
    #[must_use]
    pub fn ctrmo(&mut self) -> CtrmoW<CtrupSpec> {
        CtrmoW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Years Counter. Contains the current value for year."]
    #[inline(always)]
    #[must_use]
    pub fn ctryr(&mut self) -> CtryrW<CtrupSpec> {
        CtryrW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Weekdays Counter. Contains the current value for weekdays."]
    #[inline(always)]
    #[must_use]
    pub fn ctrwkdy(&mut self) -> CtrwkdyW<CtrupSpec> {
        CtrwkdyW::new(self, 24)
    }
    #[doc = "Bit 28 - Century Bit. CB == 0 assumes the century is 21xx, and CB == 1 assumes it is 20xx."]
    #[inline(always)]
    #[must_use]
    pub fn cb(&mut self) -> CbW<CtrupSpec> {
        CbW::new(self, 28)
    }
    #[doc = "Bit 29 - Century Enable Bit. ERR129 - RTC cannot perform rollover. Century Bit toggles when CEB == 1 and Year == 99 and hence cannot handle Year rollover from xx99 -> xx00"]
    #[inline(always)]
    #[must_use]
    pub fn ceb(&mut self) -> CebW<CtrupSpec> {
        CebW::new(self, 29)
    }
    #[doc = "Bit 31 - Counter read error status. Error is triggered when software reads the lower word of the counters, and fails to read the upper counter within 1/100 second. This is because when the lower counter is read, the upper counter is held off from incrementing until it is read so that the full time stamp can be read."]
    #[inline(always)]
    #[must_use]
    pub fn cterr(&mut self) -> CterrW<CtrupSpec> {
        CterrW::new(self, 31)
    }
}
#[doc = "This register contains the day, month and year information. It contains which day in the week, and the century as well. The information of the century can also be derived from the year information. The 31st bit contains the error bit. See description in the register bit for condition when error is triggered.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrupSpec;
impl crate::RegisterSpec for CtrupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrup::R`](R) reader structure"]
impl crate::Readable for CtrupSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrup::W`](W) writer structure"]
impl crate::Writable for CtrupSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRUP to value 0"]
impl crate::Resettable for CtrupSpec {
    const RESET_VALUE: u32 = 0;
}
