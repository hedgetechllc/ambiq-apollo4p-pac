#[doc = "Register `PWRWEIGHTULP1` reader"]
pub type R = crate::R<Pwrweightulp1Spec>;
#[doc = "Register `PWRWEIGHTULP1` writer"]
pub type W = crate::W<Pwrweightulp1Spec>;
#[doc = "Field `WTULPIOM0` reader - Weight used for ULP mode IOM0"]
pub type Wtulpiom0R = crate::FieldReader;
#[doc = "Field `WTULPIOM0` writer - Weight used for ULP mode IOM0"]
pub type Wtulpiom0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPIOM1` reader - Weight used for ULP mode IOM1"]
pub type Wtulpiom1R = crate::FieldReader;
#[doc = "Field `WTULPIOM1` writer - Weight used for ULP mode IOM1"]
pub type Wtulpiom1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPIOM2` reader - Weight used for ULP mode IOM2"]
pub type Wtulpiom2R = crate::FieldReader;
#[doc = "Field `WTULPIOM2` writer - Weight used for ULP mode IOM2"]
pub type Wtulpiom2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPIOM3` reader - Weight used for ULP mode IOM3"]
pub type Wtulpiom3R = crate::FieldReader;
#[doc = "Field `WTULPIOM3` writer - Weight used for ULP mode IOM3"]
pub type Wtulpiom3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPIOM4` reader - Weight used for ULP mode IOM4"]
pub type Wtulpiom4R = crate::FieldReader;
#[doc = "Field `WTULPIOM4` writer - Weight used for ULP mode IOM4"]
pub type Wtulpiom4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPIOM5` reader - Weight used for ULP mode IOM5"]
pub type Wtulpiom5R = crate::FieldReader;
#[doc = "Field `WTULPIOM5` writer - Weight used for ULP mode IOM5"]
pub type Wtulpiom5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPIOM6` reader - Weight used for ULP mode IOM6"]
pub type Wtulpiom6R = crate::FieldReader;
#[doc = "Field `WTULPIOM6` writer - Weight used for ULP mode IOM6"]
pub type Wtulpiom6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPIOM7` reader - Weight used for ULP mode IOM7"]
pub type Wtulpiom7R = crate::FieldReader;
#[doc = "Field `WTULPIOM7` writer - Weight used for ULP mode IOM7"]
pub type Wtulpiom7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Weight used for ULP mode IOM0"]
    #[inline(always)]
    pub fn wtulpiom0(&self) -> Wtulpiom0R {
        Wtulpiom0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Weight used for ULP mode IOM1"]
    #[inline(always)]
    pub fn wtulpiom1(&self) -> Wtulpiom1R {
        Wtulpiom1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Weight used for ULP mode IOM2"]
    #[inline(always)]
    pub fn wtulpiom2(&self) -> Wtulpiom2R {
        Wtulpiom2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Weight used for ULP mode IOM3"]
    #[inline(always)]
    pub fn wtulpiom3(&self) -> Wtulpiom3R {
        Wtulpiom3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Weight used for ULP mode IOM4"]
    #[inline(always)]
    pub fn wtulpiom4(&self) -> Wtulpiom4R {
        Wtulpiom4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Weight used for ULP mode IOM5"]
    #[inline(always)]
    pub fn wtulpiom5(&self) -> Wtulpiom5R {
        Wtulpiom5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Weight used for ULP mode IOM6"]
    #[inline(always)]
    pub fn wtulpiom6(&self) -> Wtulpiom6R {
        Wtulpiom6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Weight used for ULP mode IOM7"]
    #[inline(always)]
    pub fn wtulpiom7(&self) -> Wtulpiom7R {
        Wtulpiom7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Weight used for ULP mode IOM0"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpiom0(&mut self) -> Wtulpiom0W<Pwrweightulp1Spec> {
        Wtulpiom0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Weight used for ULP mode IOM1"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpiom1(&mut self) -> Wtulpiom1W<Pwrweightulp1Spec> {
        Wtulpiom1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Weight used for ULP mode IOM2"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpiom2(&mut self) -> Wtulpiom2W<Pwrweightulp1Spec> {
        Wtulpiom2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Weight used for ULP mode IOM3"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpiom3(&mut self) -> Wtulpiom3W<Pwrweightulp1Spec> {
        Wtulpiom3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Weight used for ULP mode IOM4"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpiom4(&mut self) -> Wtulpiom4W<Pwrweightulp1Spec> {
        Wtulpiom4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Weight used for ULP mode IOM5"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpiom5(&mut self) -> Wtulpiom5W<Pwrweightulp1Spec> {
        Wtulpiom5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Weight used for ULP mode IOM6"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpiom6(&mut self) -> Wtulpiom6W<Pwrweightulp1Spec> {
        Wtulpiom6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Weight used for ULP mode IOM7"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpiom7(&mut self) -> Wtulpiom7W<Pwrweightulp1Spec> {
        Wtulpiom7W::new(self, 28)
    }
}
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightulp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightulp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrweightulp1Spec;
impl crate::RegisterSpec for Pwrweightulp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrweightulp1::R`](R) reader structure"]
impl crate::Readable for Pwrweightulp1Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrweightulp1::W`](W) writer structure"]
impl crate::Writable for Pwrweightulp1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRWEIGHTULP1 to value 0"]
impl crate::Resettable for Pwrweightulp1Spec {
    const RESET_VALUE: u32 = 0;
}
