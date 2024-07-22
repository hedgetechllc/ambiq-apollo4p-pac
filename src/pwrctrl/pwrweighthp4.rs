#[doc = "Register `PWRWEIGHTHP4` reader"]
pub type R = crate::R<Pwrweighthp4Spec>;
#[doc = "Register `PWRWEIGHTHP4` writer"]
pub type W = crate::W<Pwrweighthp4Spec>;
#[doc = "Field `WTHPI2S0` reader - Weight used for HP mode I2S0"]
pub type Wthpi2s0R = crate::FieldReader;
#[doc = "Field `WTHPI2S0` writer - Weight used for HP mode I2S0"]
pub type Wthpi2s0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPI2S1` reader - Weight used for HP mode I2S1"]
pub type Wthpi2s1R = crate::FieldReader;
#[doc = "Field `WTHPI2S1` writer - Weight used for HP mode I2S1"]
pub type Wthpi2s1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPPDM0` reader - Weight used for HP mode PDM0"]
pub type Wthppdm0R = crate::FieldReader;
#[doc = "Field `WTHPPDM0` writer - Weight used for HP mode PDM0"]
pub type Wthppdm0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPPDM1` reader - Weight used for HP mode PDM1"]
pub type Wthppdm1R = crate::FieldReader;
#[doc = "Field `WTHPPDM1` writer - Weight used for HP mode PDM1"]
pub type Wthppdm1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPPDM2` reader - Weight used for HP mode PDM2"]
pub type Wthppdm2R = crate::FieldReader;
#[doc = "Field `WTHPPDM2` writer - Weight used for HP mode PDM2"]
pub type Wthppdm2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPPDM3` reader - Weight used for HP mode PDM3"]
pub type Wthppdm3R = crate::FieldReader;
#[doc = "Field `WTHPPDM3` writer - Weight used for HP mode PDM3"]
pub type Wthppdm3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Weight used for HP mode I2S0"]
    #[inline(always)]
    pub fn wthpi2s0(&self) -> Wthpi2s0R {
        Wthpi2s0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Weight used for HP mode I2S1"]
    #[inline(always)]
    pub fn wthpi2s1(&self) -> Wthpi2s1R {
        Wthpi2s1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Weight used for HP mode PDM0"]
    #[inline(always)]
    pub fn wthppdm0(&self) -> Wthppdm0R {
        Wthppdm0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Weight used for HP mode PDM1"]
    #[inline(always)]
    pub fn wthppdm1(&self) -> Wthppdm1R {
        Wthppdm1R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Weight used for HP mode PDM2"]
    #[inline(always)]
    pub fn wthppdm2(&self) -> Wthppdm2R {
        Wthppdm2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Weight used for HP mode PDM3"]
    #[inline(always)]
    pub fn wthppdm3(&self) -> Wthppdm3R {
        Wthppdm3R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Weight used for HP mode I2S0"]
    #[inline(always)]
    #[must_use]
    pub fn wthpi2s0(&mut self) -> Wthpi2s0W<Pwrweighthp4Spec> {
        Wthpi2s0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Weight used for HP mode I2S1"]
    #[inline(always)]
    #[must_use]
    pub fn wthpi2s1(&mut self) -> Wthpi2s1W<Pwrweighthp4Spec> {
        Wthpi2s1W::new(self, 4)
    }
    #[doc = "Bits 16:19 - Weight used for HP mode PDM0"]
    #[inline(always)]
    #[must_use]
    pub fn wthppdm0(&mut self) -> Wthppdm0W<Pwrweighthp4Spec> {
        Wthppdm0W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Weight used for HP mode PDM1"]
    #[inline(always)]
    #[must_use]
    pub fn wthppdm1(&mut self) -> Wthppdm1W<Pwrweighthp4Spec> {
        Wthppdm1W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Weight used for HP mode PDM2"]
    #[inline(always)]
    #[must_use]
    pub fn wthppdm2(&mut self) -> Wthppdm2W<Pwrweighthp4Spec> {
        Wthppdm2W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Weight used for HP mode PDM3"]
    #[inline(always)]
    #[must_use]
    pub fn wthppdm3(&mut self) -> Wthppdm3W<Pwrweighthp4Spec> {
        Wthppdm3W::new(self, 28)
    }
}
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweighthp4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweighthp4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrweighthp4Spec;
impl crate::RegisterSpec for Pwrweighthp4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrweighthp4::R`](R) reader structure"]
impl crate::Readable for Pwrweighthp4Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrweighthp4::W`](W) writer structure"]
impl crate::Writable for Pwrweighthp4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRWEIGHTHP4 to value 0"]
impl crate::Resettable for Pwrweighthp4Spec {
    const RESET_VALUE: u32 = 0;
}
