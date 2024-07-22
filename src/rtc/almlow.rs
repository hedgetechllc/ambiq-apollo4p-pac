#[doc = "Register `ALMLOW` reader"]
pub type R = crate::R<AlmlowSpec>;
#[doc = "Register `ALMLOW` writer"]
pub type W = crate::W<AlmlowSpec>;
#[doc = "Field `ALM100` reader - 100ths of a second Alarm. Contains the current value for hundredths of a second."]
pub type Alm100R = crate::FieldReader;
#[doc = "Field `ALM100` writer - 100ths of a second Alarm. Contains the current value for hundredths of a second."]
pub type Alm100W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ALMSEC` reader - Seconds Alarm. Contains the current value for seconds."]
pub type AlmsecR = crate::FieldReader;
#[doc = "Field `ALMSEC` writer - Seconds Alarm. Contains the current value for seconds."]
pub type AlmsecW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ALMMIN` reader - Minutes Alarm. Contains the current value for minutes."]
pub type AlmminR = crate::FieldReader;
#[doc = "Field `ALMMIN` writer - Minutes Alarm. Contains the current value for minutes."]
pub type AlmminW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ALMHR` reader - Hours Alarm. Contains the current value for hour."]
pub type AlmhrR = crate::FieldReader;
#[doc = "Field `ALMHR` writer - Hours Alarm. Contains the current value for hour."]
pub type AlmhrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - 100ths of a second Alarm. Contains the current value for hundredths of a second."]
    #[inline(always)]
    pub fn alm100(&self) -> Alm100R {
        Alm100R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Seconds Alarm. Contains the current value for seconds."]
    #[inline(always)]
    pub fn almsec(&self) -> AlmsecR {
        AlmsecR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Minutes Alarm. Contains the current value for minutes."]
    #[inline(always)]
    pub fn almmin(&self) -> AlmminR {
        AlmminR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:29 - Hours Alarm. Contains the current value for hour."]
    #[inline(always)]
    pub fn almhr(&self) -> AlmhrR {
        AlmhrR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 100ths of a second Alarm. Contains the current value for hundredths of a second."]
    #[inline(always)]
    #[must_use]
    pub fn alm100(&mut self) -> Alm100W<AlmlowSpec> {
        Alm100W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Seconds Alarm. Contains the current value for seconds."]
    #[inline(always)]
    #[must_use]
    pub fn almsec(&mut self) -> AlmsecW<AlmlowSpec> {
        AlmsecW::new(self, 8)
    }
    #[doc = "Bits 16:22 - Minutes Alarm. Contains the current value for minutes."]
    #[inline(always)]
    #[must_use]
    pub fn almmin(&mut self) -> AlmminW<AlmlowSpec> {
        AlmminW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Hours Alarm. Contains the current value for hour."]
    #[inline(always)]
    #[must_use]
    pub fn almhr(&mut self) -> AlmhrW<AlmlowSpec> {
        AlmhrW::new(self, 24)
    }
}
#[doc = "This register is the Alarm settings for hours, minutes, second and 1/100th seconds settings.\n\nYou can [`read`](crate::Reg::read) this register and get [`almlow::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`almlow::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlmlowSpec;
impl crate::RegisterSpec for AlmlowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`almlow::R`](R) reader structure"]
impl crate::Readable for AlmlowSpec {}
#[doc = "`write(|w| ..)` method takes [`almlow::W`](W) writer structure"]
impl crate::Writable for AlmlowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALMLOW to value 0"]
impl crate::Resettable for AlmlowSpec {
    const RESET_VALUE: u32 = 0;
}
