#[doc = "Register `PWRWEIGHTULP4` reader"]
pub type R = crate::R<Pwrweightulp4Spec>;
#[doc = "Register `PWRWEIGHTULP4` writer"]
pub type W = crate::W<Pwrweightulp4Spec>;
#[doc = "Field `WTULPI2S0` reader - Weight used for ULP mode I2S0"]
pub type Wtulpi2s0R = crate::FieldReader;
#[doc = "Field `WTULPI2S0` writer - Weight used for ULP mode I2S0"]
pub type Wtulpi2s0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPI2S1` reader - Weight used for ULP mode I2S1"]
pub type Wtulpi2s1R = crate::FieldReader;
#[doc = "Field `WTULPI2S1` writer - Weight used for ULP mode I2S1"]
pub type Wtulpi2s1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPPDM0` reader - Weight used for ULP mode PDM0"]
pub type Wtulppdm0R = crate::FieldReader;
#[doc = "Field `WTULPPDM0` writer - Weight used for ULP mode PDM0"]
pub type Wtulppdm0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPPDM1` reader - Weight used for ULP mode PDM1"]
pub type Wtulppdm1R = crate::FieldReader;
#[doc = "Field `WTULPPDM1` writer - Weight used for ULP mode PDM1"]
pub type Wtulppdm1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPPDM2` reader - Weight used for ULP mode PDM2"]
pub type Wtulppdm2R = crate::FieldReader;
#[doc = "Field `WTULPPDM2` writer - Weight used for ULP mode PDM2"]
pub type Wtulppdm2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPPDM3` reader - Weight used for ULP mode PDM3"]
pub type Wtulppdm3R = crate::FieldReader;
#[doc = "Field `WTULPPDM3` writer - Weight used for ULP mode PDM3"]
pub type Wtulppdm3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Weight used for ULP mode I2S0"]
    #[inline(always)]
    pub fn wtulpi2s0(&self) -> Wtulpi2s0R {
        Wtulpi2s0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Weight used for ULP mode I2S1"]
    #[inline(always)]
    pub fn wtulpi2s1(&self) -> Wtulpi2s1R {
        Wtulpi2s1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Weight used for ULP mode PDM0"]
    #[inline(always)]
    pub fn wtulppdm0(&self) -> Wtulppdm0R {
        Wtulppdm0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Weight used for ULP mode PDM1"]
    #[inline(always)]
    pub fn wtulppdm1(&self) -> Wtulppdm1R {
        Wtulppdm1R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Weight used for ULP mode PDM2"]
    #[inline(always)]
    pub fn wtulppdm2(&self) -> Wtulppdm2R {
        Wtulppdm2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Weight used for ULP mode PDM3"]
    #[inline(always)]
    pub fn wtulppdm3(&self) -> Wtulppdm3R {
        Wtulppdm3R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Weight used for ULP mode I2S0"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpi2s0(&mut self) -> Wtulpi2s0W<Pwrweightulp4Spec> {
        Wtulpi2s0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Weight used for ULP mode I2S1"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpi2s1(&mut self) -> Wtulpi2s1W<Pwrweightulp4Spec> {
        Wtulpi2s1W::new(self, 4)
    }
    #[doc = "Bits 16:19 - Weight used for ULP mode PDM0"]
    #[inline(always)]
    #[must_use]
    pub fn wtulppdm0(&mut self) -> Wtulppdm0W<Pwrweightulp4Spec> {
        Wtulppdm0W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Weight used for ULP mode PDM1"]
    #[inline(always)]
    #[must_use]
    pub fn wtulppdm1(&mut self) -> Wtulppdm1W<Pwrweightulp4Spec> {
        Wtulppdm1W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Weight used for ULP mode PDM2"]
    #[inline(always)]
    #[must_use]
    pub fn wtulppdm2(&mut self) -> Wtulppdm2W<Pwrweightulp4Spec> {
        Wtulppdm2W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Weight used for ULP mode PDM3"]
    #[inline(always)]
    #[must_use]
    pub fn wtulppdm3(&mut self) -> Wtulppdm3W<Pwrweightulp4Spec> {
        Wtulppdm3W::new(self, 28)
    }
}
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightulp4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightulp4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrweightulp4Spec;
impl crate::RegisterSpec for Pwrweightulp4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrweightulp4::R`](R) reader structure"]
impl crate::Readable for Pwrweightulp4Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrweightulp4::W`](W) writer structure"]
impl crate::Writable for Pwrweightulp4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRWEIGHTULP4 to value 0"]
impl crate::Resettable for Pwrweightulp4Spec {
    const RESET_VALUE: u32 = 0;
}
