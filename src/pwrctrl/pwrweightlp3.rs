#[doc = "Register `PWRWEIGHTLP3` reader"]
pub type R = crate::R<Pwrweightlp3Spec>;
#[doc = "Register `PWRWEIGHTLP3` writer"]
pub type W = crate::W<Pwrweightlp3Spec>;
#[doc = "Field `WTLPDSPA` reader - Weight used for LP mode DSPA"]
pub type WtlpdspaR = crate::FieldReader;
#[doc = "Field `WTLPDSPA` writer - Weight used for LP mode DSPA"]
pub type WtlpdspaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPDBG` reader - Weight used for LP mode DBG"]
pub type WtlpdbgR = crate::FieldReader;
#[doc = "Field `WTLPDBG` writer - Weight used for LP mode DBG"]
pub type WtlpdbgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPAUDREC` reader - Weight used for LP mode AUDREC"]
pub type WtlpaudrecR = crate::FieldReader;
#[doc = "Field `WTLPAUDREC` writer - Weight used for LP mode AUDREC"]
pub type WtlpaudrecW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPAUDPB` reader - Weight used for LP mode AUDPB"]
pub type WtlpaudpbR = crate::FieldReader;
#[doc = "Field `WTLPAUDPB` writer - Weight used for LP mode AUDPB"]
pub type WtlpaudpbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPAUDADC` reader - Weight used for LP mode AUDADC"]
pub type WtlpaudadcR = crate::FieldReader;
#[doc = "Field `WTLPAUDADC` writer - Weight used for LP mode AUDADC"]
pub type WtlpaudadcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPMSPI2` reader - Weight used for LP mode MSPI2"]
pub type Wtlpmspi2R = crate::FieldReader;
#[doc = "Field `WTLPMSPI2` writer - Weight used for LP mode MSPI2"]
pub type Wtlpmspi2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Weight used for LP mode DSPA"]
    #[inline(always)]
    pub fn wtlpdspa(&self) -> WtlpdspaR {
        WtlpdspaR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Weight used for LP mode DBG"]
    #[inline(always)]
    pub fn wtlpdbg(&self) -> WtlpdbgR {
        WtlpdbgR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Weight used for LP mode AUDREC"]
    #[inline(always)]
    pub fn wtlpaudrec(&self) -> WtlpaudrecR {
        WtlpaudrecR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Weight used for LP mode AUDPB"]
    #[inline(always)]
    pub fn wtlpaudpb(&self) -> WtlpaudpbR {
        WtlpaudpbR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Weight used for LP mode AUDADC"]
    #[inline(always)]
    pub fn wtlpaudadc(&self) -> WtlpaudadcR {
        WtlpaudadcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Weight used for LP mode MSPI2"]
    #[inline(always)]
    pub fn wtlpmspi2(&self) -> Wtlpmspi2R {
        Wtlpmspi2R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Weight used for LP mode DSPA"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpdspa(&mut self) -> WtlpdspaW<Pwrweightlp3Spec> {
        WtlpdspaW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Weight used for LP mode DBG"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpdbg(&mut self) -> WtlpdbgW<Pwrweightlp3Spec> {
        WtlpdbgW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Weight used for LP mode AUDREC"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpaudrec(&mut self) -> WtlpaudrecW<Pwrweightlp3Spec> {
        WtlpaudrecW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Weight used for LP mode AUDPB"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpaudpb(&mut self) -> WtlpaudpbW<Pwrweightlp3Spec> {
        WtlpaudpbW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Weight used for LP mode AUDADC"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpaudadc(&mut self) -> WtlpaudadcW<Pwrweightlp3Spec> {
        WtlpaudadcW::new(self, 16)
    }
    #[doc = "Bits 28:31 - Weight used for LP mode MSPI2"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpmspi2(&mut self) -> Wtlpmspi2W<Pwrweightlp3Spec> {
        Wtlpmspi2W::new(self, 28)
    }
}
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightlp3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightlp3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrweightlp3Spec;
impl crate::RegisterSpec for Pwrweightlp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrweightlp3::R`](R) reader structure"]
impl crate::Readable for Pwrweightlp3Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrweightlp3::W`](W) writer structure"]
impl crate::Writable for Pwrweightlp3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRWEIGHTLP3 to value 0"]
impl crate::Resettable for Pwrweightlp3Spec {
    const RESET_VALUE: u32 = 0;
}
