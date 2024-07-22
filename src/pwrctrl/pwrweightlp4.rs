#[doc = "Register `PWRWEIGHTLP4` reader"]
pub type R = crate::R<Pwrweightlp4Spec>;
#[doc = "Register `PWRWEIGHTLP4` writer"]
pub type W = crate::W<Pwrweightlp4Spec>;
#[doc = "Field `WTLPI2S0` reader - Weight used for LP mode I2S0"]
pub type Wtlpi2s0R = crate::FieldReader;
#[doc = "Field `WTLPI2S0` writer - Weight used for LP mode I2S0"]
pub type Wtlpi2s0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPI2S1` reader - Weight used for LP mode I2S1"]
pub type Wtlpi2s1R = crate::FieldReader;
#[doc = "Field `WTLPI2S1` writer - Weight used for LP mode I2S1"]
pub type Wtlpi2s1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPPDM0` reader - Weight used for LP mode PDM0"]
pub type Wtlppdm0R = crate::FieldReader;
#[doc = "Field `WTLPPDM0` writer - Weight used for LP mode PDM0"]
pub type Wtlppdm0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPPDM1` reader - Weight used for LP mode PDM1"]
pub type Wtlppdm1R = crate::FieldReader;
#[doc = "Field `WTLPPDM1` writer - Weight used for LP mode PDM1"]
pub type Wtlppdm1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPPDM2` reader - Weight used for LP mode PDM2"]
pub type Wtlppdm2R = crate::FieldReader;
#[doc = "Field `WTLPPDM2` writer - Weight used for LP mode PDM2"]
pub type Wtlppdm2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPPDM3` reader - Weight used for LP mode PDM3"]
pub type Wtlppdm3R = crate::FieldReader;
#[doc = "Field `WTLPPDM3` writer - Weight used for LP mode PDM3"]
pub type Wtlppdm3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Weight used for LP mode I2S0"]
    #[inline(always)]
    pub fn wtlpi2s0(&self) -> Wtlpi2s0R {
        Wtlpi2s0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Weight used for LP mode I2S1"]
    #[inline(always)]
    pub fn wtlpi2s1(&self) -> Wtlpi2s1R {
        Wtlpi2s1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Weight used for LP mode PDM0"]
    #[inline(always)]
    pub fn wtlppdm0(&self) -> Wtlppdm0R {
        Wtlppdm0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Weight used for LP mode PDM1"]
    #[inline(always)]
    pub fn wtlppdm1(&self) -> Wtlppdm1R {
        Wtlppdm1R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Weight used for LP mode PDM2"]
    #[inline(always)]
    pub fn wtlppdm2(&self) -> Wtlppdm2R {
        Wtlppdm2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Weight used for LP mode PDM3"]
    #[inline(always)]
    pub fn wtlppdm3(&self) -> Wtlppdm3R {
        Wtlppdm3R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Weight used for LP mode I2S0"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpi2s0(&mut self) -> Wtlpi2s0W<Pwrweightlp4Spec> {
        Wtlpi2s0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Weight used for LP mode I2S1"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpi2s1(&mut self) -> Wtlpi2s1W<Pwrweightlp4Spec> {
        Wtlpi2s1W::new(self, 4)
    }
    #[doc = "Bits 16:19 - Weight used for LP mode PDM0"]
    #[inline(always)]
    #[must_use]
    pub fn wtlppdm0(&mut self) -> Wtlppdm0W<Pwrweightlp4Spec> {
        Wtlppdm0W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Weight used for LP mode PDM1"]
    #[inline(always)]
    #[must_use]
    pub fn wtlppdm1(&mut self) -> Wtlppdm1W<Pwrweightlp4Spec> {
        Wtlppdm1W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Weight used for LP mode PDM2"]
    #[inline(always)]
    #[must_use]
    pub fn wtlppdm2(&mut self) -> Wtlppdm2W<Pwrweightlp4Spec> {
        Wtlppdm2W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Weight used for LP mode PDM3"]
    #[inline(always)]
    #[must_use]
    pub fn wtlppdm3(&mut self) -> Wtlppdm3W<Pwrweightlp4Spec> {
        Wtlppdm3W::new(self, 28)
    }
}
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightlp4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightlp4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrweightlp4Spec;
impl crate::RegisterSpec for Pwrweightlp4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrweightlp4::R`](R) reader structure"]
impl crate::Readable for Pwrweightlp4Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrweightlp4::W`](W) writer structure"]
impl crate::Writable for Pwrweightlp4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRWEIGHTLP4 to value 0"]
impl crate::Resettable for Pwrweightlp4Spec {
    const RESET_VALUE: u32 = 0;
}
