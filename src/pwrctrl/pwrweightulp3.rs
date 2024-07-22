#[doc = "Register `PWRWEIGHTULP3` reader"]
pub type R = crate::R<Pwrweightulp3Spec>;
#[doc = "Register `PWRWEIGHTULP3` writer"]
pub type W = crate::W<Pwrweightulp3Spec>;
#[doc = "Field `WTULPDSPA` reader - Weight used for ULP mode DSPA"]
pub type WtulpdspaR = crate::FieldReader;
#[doc = "Field `WTULPDSPA` writer - Weight used for ULP mode DSPA"]
pub type WtulpdspaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPDBG` reader - Weight used for ULP mode DBG"]
pub type WtulpdbgR = crate::FieldReader;
#[doc = "Field `WTULPDBG` writer - Weight used for ULP mode DBG"]
pub type WtulpdbgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPAUDREC` reader - Weight used for ULP mode AUDREC"]
pub type WtulpaudrecR = crate::FieldReader;
#[doc = "Field `WTULPAUDREC` writer - Weight used for ULP mode AUDREC"]
pub type WtulpaudrecW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPAUDPB` reader - Weight used for ULP mode AUDPB"]
pub type WtulpaudpbR = crate::FieldReader;
#[doc = "Field `WTULPAUDPB` writer - Weight used for ULP mode AUDPB"]
pub type WtulpaudpbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPAUDADC` reader - Weight used for ULP mode AUDADC"]
pub type WtulpaudadcR = crate::FieldReader;
#[doc = "Field `WTULPAUDADC` writer - Weight used for ULP mode AUDADC"]
pub type WtulpaudadcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPMSPI2` reader - Weight used for ULP mode MSPI2"]
pub type Wtulpmspi2R = crate::FieldReader;
#[doc = "Field `WTULPMSPI2` writer - Weight used for ULP mode MSPI2"]
pub type Wtulpmspi2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Weight used for ULP mode DSPA"]
    #[inline(always)]
    pub fn wtulpdspa(&self) -> WtulpdspaR {
        WtulpdspaR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Weight used for ULP mode DBG"]
    #[inline(always)]
    pub fn wtulpdbg(&self) -> WtulpdbgR {
        WtulpdbgR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Weight used for ULP mode AUDREC"]
    #[inline(always)]
    pub fn wtulpaudrec(&self) -> WtulpaudrecR {
        WtulpaudrecR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Weight used for ULP mode AUDPB"]
    #[inline(always)]
    pub fn wtulpaudpb(&self) -> WtulpaudpbR {
        WtulpaudpbR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Weight used for ULP mode AUDADC"]
    #[inline(always)]
    pub fn wtulpaudadc(&self) -> WtulpaudadcR {
        WtulpaudadcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Weight used for ULP mode MSPI2"]
    #[inline(always)]
    pub fn wtulpmspi2(&self) -> Wtulpmspi2R {
        Wtulpmspi2R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Weight used for ULP mode DSPA"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpdspa(&mut self) -> WtulpdspaW<Pwrweightulp3Spec> {
        WtulpdspaW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Weight used for ULP mode DBG"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpdbg(&mut self) -> WtulpdbgW<Pwrweightulp3Spec> {
        WtulpdbgW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Weight used for ULP mode AUDREC"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpaudrec(&mut self) -> WtulpaudrecW<Pwrweightulp3Spec> {
        WtulpaudrecW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Weight used for ULP mode AUDPB"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpaudpb(&mut self) -> WtulpaudpbW<Pwrweightulp3Spec> {
        WtulpaudpbW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Weight used for ULP mode AUDADC"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpaudadc(&mut self) -> WtulpaudadcW<Pwrweightulp3Spec> {
        WtulpaudadcW::new(self, 16)
    }
    #[doc = "Bits 28:31 - Weight used for ULP mode MSPI2"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpmspi2(&mut self) -> Wtulpmspi2W<Pwrweightulp3Spec> {
        Wtulpmspi2W::new(self, 28)
    }
}
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightulp3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightulp3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrweightulp3Spec;
impl crate::RegisterSpec for Pwrweightulp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrweightulp3::R`](R) reader structure"]
impl crate::Readable for Pwrweightulp3Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrweightulp3::W`](W) writer structure"]
impl crate::Writable for Pwrweightulp3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRWEIGHTULP3 to value 0"]
impl crate::Resettable for Pwrweightulp3Spec {
    const RESET_VALUE: u32 = 0;
}
