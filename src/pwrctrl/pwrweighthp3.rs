#[doc = "Register `PWRWEIGHTHP3` reader"]
pub type R = crate::R<Pwrweighthp3Spec>;
#[doc = "Register `PWRWEIGHTHP3` writer"]
pub type W = crate::W<Pwrweighthp3Spec>;
#[doc = "Field `WTHPDSPA` reader - Weight used for HP mode DSPA"]
pub type WthpdspaR = crate::FieldReader;
#[doc = "Field `WTHPDSPA` writer - Weight used for HP mode DSPA"]
pub type WthpdspaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPDBG` reader - Weight used for HP mode DBG"]
pub type WthpdbgR = crate::FieldReader;
#[doc = "Field `WTHPDBG` writer - Weight used for HP mode DBG"]
pub type WthpdbgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPAUDREC` reader - Weight used for HP mode AUDREC"]
pub type WthpaudrecR = crate::FieldReader;
#[doc = "Field `WTHPAUDREC` writer - Weight used for HP mode AUDREC"]
pub type WthpaudrecW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPAUDPB` reader - Weight used for HP mode AUDPB"]
pub type WthpaudpbR = crate::FieldReader;
#[doc = "Field `WTHPAUDPB` writer - Weight used for HP mode AUDPB"]
pub type WthpaudpbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPAUDADC` reader - Weight used for HP mode AUDADC"]
pub type WthpaudadcR = crate::FieldReader;
#[doc = "Field `WTHPAUDADC` writer - Weight used for HP mode AUDADC"]
pub type WthpaudadcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPMSPI2` reader - Weight used for HP mode MSPI2"]
pub type Wthpmspi2R = crate::FieldReader;
#[doc = "Field `WTHPMSPI2` writer - Weight used for HP mode MSPI2"]
pub type Wthpmspi2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Weight used for HP mode DSPA"]
    #[inline(always)]
    pub fn wthpdspa(&self) -> WthpdspaR {
        WthpdspaR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Weight used for HP mode DBG"]
    #[inline(always)]
    pub fn wthpdbg(&self) -> WthpdbgR {
        WthpdbgR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Weight used for HP mode AUDREC"]
    #[inline(always)]
    pub fn wthpaudrec(&self) -> WthpaudrecR {
        WthpaudrecR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Weight used for HP mode AUDPB"]
    #[inline(always)]
    pub fn wthpaudpb(&self) -> WthpaudpbR {
        WthpaudpbR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Weight used for HP mode AUDADC"]
    #[inline(always)]
    pub fn wthpaudadc(&self) -> WthpaudadcR {
        WthpaudadcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Weight used for HP mode MSPI2"]
    #[inline(always)]
    pub fn wthpmspi2(&self) -> Wthpmspi2R {
        Wthpmspi2R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Weight used for HP mode DSPA"]
    #[inline(always)]
    #[must_use]
    pub fn wthpdspa(&mut self) -> WthpdspaW<Pwrweighthp3Spec> {
        WthpdspaW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Weight used for HP mode DBG"]
    #[inline(always)]
    #[must_use]
    pub fn wthpdbg(&mut self) -> WthpdbgW<Pwrweighthp3Spec> {
        WthpdbgW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Weight used for HP mode AUDREC"]
    #[inline(always)]
    #[must_use]
    pub fn wthpaudrec(&mut self) -> WthpaudrecW<Pwrweighthp3Spec> {
        WthpaudrecW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Weight used for HP mode AUDPB"]
    #[inline(always)]
    #[must_use]
    pub fn wthpaudpb(&mut self) -> WthpaudpbW<Pwrweighthp3Spec> {
        WthpaudpbW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Weight used for HP mode AUDADC"]
    #[inline(always)]
    #[must_use]
    pub fn wthpaudadc(&mut self) -> WthpaudadcW<Pwrweighthp3Spec> {
        WthpaudadcW::new(self, 16)
    }
    #[doc = "Bits 28:31 - Weight used for HP mode MSPI2"]
    #[inline(always)]
    #[must_use]
    pub fn wthpmspi2(&mut self) -> Wthpmspi2W<Pwrweighthp3Spec> {
        Wthpmspi2W::new(self, 28)
    }
}
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweighthp3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweighthp3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrweighthp3Spec;
impl crate::RegisterSpec for Pwrweighthp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrweighthp3::R`](R) reader structure"]
impl crate::Readable for Pwrweighthp3Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrweighthp3::W`](W) writer structure"]
impl crate::Writable for Pwrweighthp3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRWEIGHTHP3 to value 0"]
impl crate::Resettable for Pwrweighthp3Spec {
    const RESET_VALUE: u32 = 0;
}
