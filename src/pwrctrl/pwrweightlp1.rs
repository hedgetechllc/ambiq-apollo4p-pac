#[doc = "Register `PWRWEIGHTLP1` reader"]
pub type R = crate::R<Pwrweightlp1Spec>;
#[doc = "Register `PWRWEIGHTLP1` writer"]
pub type W = crate::W<Pwrweightlp1Spec>;
#[doc = "Field `WTLPIOM0` reader - Weight used for LP mode IOM0"]
pub type Wtlpiom0R = crate::FieldReader;
#[doc = "Field `WTLPIOM0` writer - Weight used for LP mode IOM0"]
pub type Wtlpiom0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPIOM1` reader - Weight used for LP mode IOM1"]
pub type Wtlpiom1R = crate::FieldReader;
#[doc = "Field `WTLPIOM1` writer - Weight used for LP mode IOM1"]
pub type Wtlpiom1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPIOM2` reader - Weight used for LP mode IOM2"]
pub type Wtlpiom2R = crate::FieldReader;
#[doc = "Field `WTLPIOM2` writer - Weight used for LP mode IOM2"]
pub type Wtlpiom2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPIOM3` reader - Weight used for LP mode IOM3"]
pub type Wtlpiom3R = crate::FieldReader;
#[doc = "Field `WTLPIOM3` writer - Weight used for LP mode IOM3"]
pub type Wtlpiom3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPIOM4` reader - Weight used for LP mode IOM4"]
pub type Wtlpiom4R = crate::FieldReader;
#[doc = "Field `WTLPIOM4` writer - Weight used for LP mode IOM4"]
pub type Wtlpiom4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPIOM5` reader - Weight used for LP mode IOM5"]
pub type Wtlpiom5R = crate::FieldReader;
#[doc = "Field `WTLPIOM5` writer - Weight used for LP mode IOM5"]
pub type Wtlpiom5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPIOM6` reader - Weight used for LP mode IOM6"]
pub type Wtlpiom6R = crate::FieldReader;
#[doc = "Field `WTLPIOM6` writer - Weight used for LP mode IOM6"]
pub type Wtlpiom6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPIOM7` reader - Weight used for LP mode IOM7"]
pub type Wtlpiom7R = crate::FieldReader;
#[doc = "Field `WTLPIOM7` writer - Weight used for LP mode IOM7"]
pub type Wtlpiom7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Weight used for LP mode IOM0"]
    #[inline(always)]
    pub fn wtlpiom0(&self) -> Wtlpiom0R {
        Wtlpiom0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Weight used for LP mode IOM1"]
    #[inline(always)]
    pub fn wtlpiom1(&self) -> Wtlpiom1R {
        Wtlpiom1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Weight used for LP mode IOM2"]
    #[inline(always)]
    pub fn wtlpiom2(&self) -> Wtlpiom2R {
        Wtlpiom2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Weight used for LP mode IOM3"]
    #[inline(always)]
    pub fn wtlpiom3(&self) -> Wtlpiom3R {
        Wtlpiom3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Weight used for LP mode IOM4"]
    #[inline(always)]
    pub fn wtlpiom4(&self) -> Wtlpiom4R {
        Wtlpiom4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Weight used for LP mode IOM5"]
    #[inline(always)]
    pub fn wtlpiom5(&self) -> Wtlpiom5R {
        Wtlpiom5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Weight used for LP mode IOM6"]
    #[inline(always)]
    pub fn wtlpiom6(&self) -> Wtlpiom6R {
        Wtlpiom6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Weight used for LP mode IOM7"]
    #[inline(always)]
    pub fn wtlpiom7(&self) -> Wtlpiom7R {
        Wtlpiom7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Weight used for LP mode IOM0"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpiom0(&mut self) -> Wtlpiom0W<Pwrweightlp1Spec> {
        Wtlpiom0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Weight used for LP mode IOM1"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpiom1(&mut self) -> Wtlpiom1W<Pwrweightlp1Spec> {
        Wtlpiom1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Weight used for LP mode IOM2"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpiom2(&mut self) -> Wtlpiom2W<Pwrweightlp1Spec> {
        Wtlpiom2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Weight used for LP mode IOM3"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpiom3(&mut self) -> Wtlpiom3W<Pwrweightlp1Spec> {
        Wtlpiom3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Weight used for LP mode IOM4"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpiom4(&mut self) -> Wtlpiom4W<Pwrweightlp1Spec> {
        Wtlpiom4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Weight used for LP mode IOM5"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpiom5(&mut self) -> Wtlpiom5W<Pwrweightlp1Spec> {
        Wtlpiom5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Weight used for LP mode IOM6"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpiom6(&mut self) -> Wtlpiom6W<Pwrweightlp1Spec> {
        Wtlpiom6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Weight used for LP mode IOM7"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpiom7(&mut self) -> Wtlpiom7W<Pwrweightlp1Spec> {
        Wtlpiom7W::new(self, 28)
    }
}
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightlp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightlp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrweightlp1Spec;
impl crate::RegisterSpec for Pwrweightlp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrweightlp1::R`](R) reader structure"]
impl crate::Readable for Pwrweightlp1Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrweightlp1::W`](W) writer structure"]
impl crate::Writable for Pwrweightlp1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRWEIGHTLP1 to value 0"]
impl crate::Resettable for Pwrweightlp1Spec {
    const RESET_VALUE: u32 = 0;
}
