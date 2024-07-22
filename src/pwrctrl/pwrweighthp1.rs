#[doc = "Register `PWRWEIGHTHP1` reader"]
pub type R = crate::R<Pwrweighthp1Spec>;
#[doc = "Register `PWRWEIGHTHP1` writer"]
pub type W = crate::W<Pwrweighthp1Spec>;
#[doc = "Field `WTHPIOM0` reader - Weight used for HP mode IOM0"]
pub type Wthpiom0R = crate::FieldReader;
#[doc = "Field `WTHPIOM0` writer - Weight used for HP mode IOM0"]
pub type Wthpiom0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPIOM1` reader - Weight used for HP mode IOM1"]
pub type Wthpiom1R = crate::FieldReader;
#[doc = "Field `WTHPIOM1` writer - Weight used for HP mode IOM1"]
pub type Wthpiom1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPIOM2` reader - Weight used for HP mode IOM2"]
pub type Wthpiom2R = crate::FieldReader;
#[doc = "Field `WTHPIOM2` writer - Weight used for HP mode IOM2"]
pub type Wthpiom2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPIOM3` reader - Weight used for HP mode IOM3"]
pub type Wthpiom3R = crate::FieldReader;
#[doc = "Field `WTHPIOM3` writer - Weight used for HP mode IOM3"]
pub type Wthpiom3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPIOM4` reader - Weight used for HP mode IOM4"]
pub type Wthpiom4R = crate::FieldReader;
#[doc = "Field `WTHPIOM4` writer - Weight used for HP mode IOM4"]
pub type Wthpiom4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPIOM5` reader - Weight used for HP mode IOM5"]
pub type Wthpiom5R = crate::FieldReader;
#[doc = "Field `WTHPIOM5` writer - Weight used for HP mode IOM5"]
pub type Wthpiom5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPIOM6` reader - Weight used for HP mode IOM6"]
pub type Wthpiom6R = crate::FieldReader;
#[doc = "Field `WTHPIOM6` writer - Weight used for HP mode IOM6"]
pub type Wthpiom6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPIOM7` reader - Weight used for HP mode IOM7"]
pub type Wthpiom7R = crate::FieldReader;
#[doc = "Field `WTHPIOM7` writer - Weight used for HP mode IOM7"]
pub type Wthpiom7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Weight used for HP mode IOM0"]
    #[inline(always)]
    pub fn wthpiom0(&self) -> Wthpiom0R {
        Wthpiom0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Weight used for HP mode IOM1"]
    #[inline(always)]
    pub fn wthpiom1(&self) -> Wthpiom1R {
        Wthpiom1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Weight used for HP mode IOM2"]
    #[inline(always)]
    pub fn wthpiom2(&self) -> Wthpiom2R {
        Wthpiom2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Weight used for HP mode IOM3"]
    #[inline(always)]
    pub fn wthpiom3(&self) -> Wthpiom3R {
        Wthpiom3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Weight used for HP mode IOM4"]
    #[inline(always)]
    pub fn wthpiom4(&self) -> Wthpiom4R {
        Wthpiom4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Weight used for HP mode IOM5"]
    #[inline(always)]
    pub fn wthpiom5(&self) -> Wthpiom5R {
        Wthpiom5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Weight used for HP mode IOM6"]
    #[inline(always)]
    pub fn wthpiom6(&self) -> Wthpiom6R {
        Wthpiom6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Weight used for HP mode IOM7"]
    #[inline(always)]
    pub fn wthpiom7(&self) -> Wthpiom7R {
        Wthpiom7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Weight used for HP mode IOM0"]
    #[inline(always)]
    #[must_use]
    pub fn wthpiom0(&mut self) -> Wthpiom0W<Pwrweighthp1Spec> {
        Wthpiom0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Weight used for HP mode IOM1"]
    #[inline(always)]
    #[must_use]
    pub fn wthpiom1(&mut self) -> Wthpiom1W<Pwrweighthp1Spec> {
        Wthpiom1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Weight used for HP mode IOM2"]
    #[inline(always)]
    #[must_use]
    pub fn wthpiom2(&mut self) -> Wthpiom2W<Pwrweighthp1Spec> {
        Wthpiom2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Weight used for HP mode IOM3"]
    #[inline(always)]
    #[must_use]
    pub fn wthpiom3(&mut self) -> Wthpiom3W<Pwrweighthp1Spec> {
        Wthpiom3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Weight used for HP mode IOM4"]
    #[inline(always)]
    #[must_use]
    pub fn wthpiom4(&mut self) -> Wthpiom4W<Pwrweighthp1Spec> {
        Wthpiom4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Weight used for HP mode IOM5"]
    #[inline(always)]
    #[must_use]
    pub fn wthpiom5(&mut self) -> Wthpiom5W<Pwrweighthp1Spec> {
        Wthpiom5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Weight used for HP mode IOM6"]
    #[inline(always)]
    #[must_use]
    pub fn wthpiom6(&mut self) -> Wthpiom6W<Pwrweighthp1Spec> {
        Wthpiom6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Weight used for HP mode IOM7"]
    #[inline(always)]
    #[must_use]
    pub fn wthpiom7(&mut self) -> Wthpiom7W<Pwrweighthp1Spec> {
        Wthpiom7W::new(self, 28)
    }
}
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweighthp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweighthp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrweighthp1Spec;
impl crate::RegisterSpec for Pwrweighthp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrweighthp1::R`](R) reader structure"]
impl crate::Readable for Pwrweighthp1Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrweighthp1::W`](W) writer structure"]
impl crate::Writable for Pwrweighthp1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRWEIGHTHP1 to value 0"]
impl crate::Resettable for Pwrweighthp1Spec {
    const RESET_VALUE: u32 = 0;
}
