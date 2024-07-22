#[doc = "Register `CTRLOW` reader"]
pub type R = crate::R<CtrlowSpec>;
#[doc = "Register `CTRLOW` writer"]
pub type W = crate::W<CtrlowSpec>;
#[doc = "Field `CTR100` reader - 100ths of a second Counter. Contains the current value for hundredths of a second."]
pub type Ctr100R = crate::FieldReader;
#[doc = "Field `CTR100` writer - 100ths of a second Counter. Contains the current value for hundredths of a second."]
pub type Ctr100W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CTRSEC` reader - Seconds Counter. Contains the current value for seconds."]
pub type CtrsecR = crate::FieldReader;
#[doc = "Field `CTRSEC` writer - Seconds Counter. Contains the current value for seconds."]
pub type CtrsecW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CTRMIN` reader - Minutes Counter. Contains the current value for minutes."]
pub type CtrminR = crate::FieldReader;
#[doc = "Field `CTRMIN` writer - Minutes Counter. Contains the current value for minutes."]
pub type CtrminW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CTRHR` reader - Hours Counter. Contains the current value for hour."]
pub type CtrhrR = crate::FieldReader;
#[doc = "Field `CTRHR` writer - Hours Counter. Contains the current value for hour."]
pub type CtrhrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - 100ths of a second Counter. Contains the current value for hundredths of a second."]
    #[inline(always)]
    pub fn ctr100(&self) -> Ctr100R {
        Ctr100R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Seconds Counter. Contains the current value for seconds."]
    #[inline(always)]
    pub fn ctrsec(&self) -> CtrsecR {
        CtrsecR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Minutes Counter. Contains the current value for minutes."]
    #[inline(always)]
    pub fn ctrmin(&self) -> CtrminR {
        CtrminR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:29 - Hours Counter. Contains the current value for hour."]
    #[inline(always)]
    pub fn ctrhr(&self) -> CtrhrR {
        CtrhrR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 100ths of a second Counter. Contains the current value for hundredths of a second."]
    #[inline(always)]
    #[must_use]
    pub fn ctr100(&mut self) -> Ctr100W<CtrlowSpec> {
        Ctr100W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Seconds Counter. Contains the current value for seconds."]
    #[inline(always)]
    #[must_use]
    pub fn ctrsec(&mut self) -> CtrsecW<CtrlowSpec> {
        CtrsecW::new(self, 8)
    }
    #[doc = "Bits 16:22 - Minutes Counter. Contains the current value for minutes."]
    #[inline(always)]
    #[must_use]
    pub fn ctrmin(&mut self) -> CtrminW<CtrlowSpec> {
        CtrminW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Hours Counter. Contains the current value for hour."]
    #[inline(always)]
    #[must_use]
    pub fn ctrhr(&mut self) -> CtrhrW<CtrlowSpec> {
        CtrhrW::new(self, 24)
    }
}
#[doc = "This counter contains the values for hour, minutes, seconds and 100ths of a second Counter.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlow::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlow::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlowSpec;
impl crate::RegisterSpec for CtrlowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlow::R`](R) reader structure"]
impl crate::Readable for CtrlowSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlow::W`](W) writer structure"]
impl crate::Writable for CtrlowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRLOW to value 0x0100_0000"]
impl crate::Resettable for CtrlowSpec {
    const RESET_VALUE: u32 = 0x0100_0000;
}
