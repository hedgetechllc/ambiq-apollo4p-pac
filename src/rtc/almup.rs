#[doc = "Register `ALMUP` reader"]
pub type R = crate::R<AlmupSpec>;
#[doc = "Register `ALMUP` writer"]
pub type W = crate::W<AlmupSpec>;
#[doc = "Field `ALMDATE` reader - Date Alarm. Contains the current value for date alarm."]
pub type AlmdateR = crate::FieldReader;
#[doc = "Field `ALMDATE` writer - Date Alarm. Contains the current value for date alarm."]
pub type AlmdateW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ALMMO` reader - Months Alarm. Contains the current value for month alarm."]
pub type AlmmoR = crate::FieldReader;
#[doc = "Field `ALMMO` writer - Months Alarm. Contains the current value for month alarm."]
pub type AlmmoW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ALMWKDY` reader - Weekdays Alarm. Contains the current value for weekday alarm."]
pub type AlmwkdyR = crate::FieldReader;
#[doc = "Field `ALMWKDY` writer - Weekdays Alarm. Contains the current value for weekday alarm."]
pub type AlmwkdyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:5 - Date Alarm. Contains the current value for date alarm."]
    #[inline(always)]
    pub fn almdate(&self) -> AlmdateR {
        AlmdateR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Months Alarm. Contains the current value for month alarm."]
    #[inline(always)]
    pub fn almmo(&self) -> AlmmoR {
        AlmmoR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18 - Weekdays Alarm. Contains the current value for weekday alarm."]
    #[inline(always)]
    pub fn almwkdy(&self) -> AlmwkdyR {
        AlmwkdyR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Date Alarm. Contains the current value for date alarm."]
    #[inline(always)]
    #[must_use]
    pub fn almdate(&mut self) -> AlmdateW<AlmupSpec> {
        AlmdateW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Months Alarm. Contains the current value for month alarm."]
    #[inline(always)]
    #[must_use]
    pub fn almmo(&mut self) -> AlmmoW<AlmupSpec> {
        AlmmoW::new(self, 8)
    }
    #[doc = "Bits 16:18 - Weekdays Alarm. Contains the current value for weekday alarm."]
    #[inline(always)]
    #[must_use]
    pub fn almwkdy(&mut self) -> AlmwkdyW<AlmupSpec> {
        AlmwkdyW::new(self, 16)
    }
}
#[doc = "This register is the alarm settings for week, month and day.\n\nYou can [`read`](crate::Reg::read) this register and get [`almup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`almup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlmupSpec;
impl crate::RegisterSpec for AlmupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`almup::R`](R) reader structure"]
impl crate::Readable for AlmupSpec {}
#[doc = "`write(|w| ..)` method takes [`almup::W`](W) writer structure"]
impl crate::Writable for AlmupSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALMUP to value 0"]
impl crate::Resettable for AlmupSpec {
    const RESET_VALUE: u32 = 0;
}
