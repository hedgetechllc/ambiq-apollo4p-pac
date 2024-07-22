#[doc = "Register `VREFGEN2` reader"]
pub type R = crate::R<Vrefgen2Spec>;
#[doc = "Register `VREFGEN2` writer"]
pub type W = crate::W<Vrefgen2Spec>;
#[doc = "Field `TVRGTEMPCOTRIM` reader - Calibrated Voltage Reference Generator tc trim (bottom transistor)"]
pub type TvrgtempcotrimR = crate::FieldReader;
#[doc = "Field `TVRGTEMPCOTRIM` writer - Calibrated Voltage Reference Generator tc trim (bottom transistor)"]
pub type TvrgtempcotrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Power Down, Calibrated Voltage Reference Generator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tvrgpwd {
    #[doc = "1: Powers down the CVRG."]
    PwrDn = 1,
    #[doc = "0: Power up the CVRG."]
    PwrUp = 0,
}
impl From<Tvrgpwd> for bool {
    #[inline(always)]
    fn from(variant: Tvrgpwd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TVRGPWD` reader - Power Down, Calibrated Voltage Reference Generator."]
pub type TvrgpwdR = crate::BitReader<Tvrgpwd>;
impl TvrgpwdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tvrgpwd {
        match self.bits {
            true => Tvrgpwd::PwrDn,
            false => Tvrgpwd::PwrUp,
        }
    }
    #[doc = "Powers down the CVRG."]
    #[inline(always)]
    pub fn is_pwr_dn(&self) -> bool {
        *self == Tvrgpwd::PwrDn
    }
    #[doc = "Power up the CVRG."]
    #[inline(always)]
    pub fn is_pwr_up(&self) -> bool {
        *self == Tvrgpwd::PwrUp
    }
}
#[doc = "Field `TVRGPWD` writer - Power Down, Calibrated Voltage Reference Generator."]
pub type TvrgpwdW<'a, REG> = crate::BitWriter<'a, REG, Tvrgpwd>;
impl<'a, REG> TvrgpwdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Powers down the CVRG."]
    #[inline(always)]
    pub fn pwr_dn(self) -> &'a mut crate::W<REG> {
        self.variant(Tvrgpwd::PwrDn)
    }
    #[doc = "Power up the CVRG."]
    #[inline(always)]
    pub fn pwr_up(self) -> &'a mut crate::W<REG> {
        self.variant(Tvrgpwd::PwrUp)
    }
}
#[doc = "Field `TVRGCURRENTTRIM` reader - Calibrated voltage reference current trim."]
pub type TvrgcurrenttrimR = crate::BitReader;
#[doc = "Field `TVRGCURRENTTRIM` writer - Calibrated voltage reference current trim."]
pub type TvrgcurrenttrimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TVRGVREFTRIM` reader - Calibrated voltage reference 580m trim"]
pub type TvrgvreftrimR = crate::FieldReader;
#[doc = "Field `TVRGVREFTRIM` writer - Calibrated voltage reference 580m trim"]
pub type TvrgvreftrimW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TVRG2TEMPCOTRIM` reader - Calibrated Voltage Reference Generator tc trim (bottom transistor)"]
pub type Tvrg2tempcotrimR = crate::FieldReader;
#[doc = "Field `TVRG2TEMPCOTRIM` writer - Calibrated Voltage Reference Generator tc trim (bottom transistor)"]
pub type Tvrg2tempcotrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Power Down, Calibrated Voltage Reference Generator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tvrg2pwd {
    #[doc = "1: Powers down the CVRG."]
    PwrDn = 1,
    #[doc = "0: Power up the CVRG."]
    PwrUp = 0,
}
impl From<Tvrg2pwd> for bool {
    #[inline(always)]
    fn from(variant: Tvrg2pwd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TVRG2PWD` reader - Power Down, Calibrated Voltage Reference Generator."]
pub type Tvrg2pwdR = crate::BitReader<Tvrg2pwd>;
impl Tvrg2pwdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tvrg2pwd {
        match self.bits {
            true => Tvrg2pwd::PwrDn,
            false => Tvrg2pwd::PwrUp,
        }
    }
    #[doc = "Powers down the CVRG."]
    #[inline(always)]
    pub fn is_pwr_dn(&self) -> bool {
        *self == Tvrg2pwd::PwrDn
    }
    #[doc = "Power up the CVRG."]
    #[inline(always)]
    pub fn is_pwr_up(&self) -> bool {
        *self == Tvrg2pwd::PwrUp
    }
}
#[doc = "Field `TVRG2PWD` writer - Power Down, Calibrated Voltage Reference Generator."]
pub type Tvrg2pwdW<'a, REG> = crate::BitWriter<'a, REG, Tvrg2pwd>;
impl<'a, REG> Tvrg2pwdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Powers down the CVRG."]
    #[inline(always)]
    pub fn pwr_dn(self) -> &'a mut crate::W<REG> {
        self.variant(Tvrg2pwd::PwrDn)
    }
    #[doc = "Power up the CVRG."]
    #[inline(always)]
    pub fn pwr_up(self) -> &'a mut crate::W<REG> {
        self.variant(Tvrg2pwd::PwrUp)
    }
}
#[doc = "Field `TVRG2CURRENTTRIM` reader - Calibrated voltage reference current trim."]
pub type Tvrg2currenttrimR = crate::BitReader;
#[doc = "Field `TVRG2CURRENTTRIM` writer - Calibrated voltage reference current trim."]
pub type Tvrg2currenttrimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TVRG2VREFTRIM` reader - Calibrated voltage reference 580m trim"]
pub type Tvrg2vreftrimR = crate::FieldReader;
#[doc = "Field `TVRG2VREFTRIM` writer - Calibrated voltage reference 580m trim"]
pub type Tvrg2vreftrimW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TVRGSELVREF` reader - TVRG SEL VREF"]
pub type TvrgselvrefR = crate::BitReader;
#[doc = "Field `TVRGSELVREF` writer - TVRG SEL VREF"]
pub type TvrgselvrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TVRG2SELVREF` reader - TVRG2 SEL VREF"]
pub type Tvrg2selvrefR = crate::BitReader;
#[doc = "Field `TVRG2SELVREF` writer - TVRG2 SEL VREF"]
pub type Tvrg2selvrefW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Calibrated Voltage Reference Generator tc trim (bottom transistor)"]
    #[inline(always)]
    pub fn tvrgtempcotrim(&self) -> TvrgtempcotrimR {
        TvrgtempcotrimR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Power Down, Calibrated Voltage Reference Generator."]
    #[inline(always)]
    pub fn tvrgpwd(&self) -> TvrgpwdR {
        TvrgpwdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Calibrated voltage reference current trim."]
    #[inline(always)]
    pub fn tvrgcurrenttrim(&self) -> TvrgcurrenttrimR {
        TvrgcurrenttrimR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:13 - Calibrated voltage reference 580m trim"]
    #[inline(always)]
    pub fn tvrgvreftrim(&self) -> TvrgvreftrimR {
        TvrgvreftrimR::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:18 - Calibrated Voltage Reference Generator tc trim (bottom transistor)"]
    #[inline(always)]
    pub fn tvrg2tempcotrim(&self) -> Tvrg2tempcotrimR {
        Tvrg2tempcotrimR::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bit 19 - Power Down, Calibrated Voltage Reference Generator."]
    #[inline(always)]
    pub fn tvrg2pwd(&self) -> Tvrg2pwdR {
        Tvrg2pwdR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Calibrated voltage reference current trim."]
    #[inline(always)]
    pub fn tvrg2currenttrim(&self) -> Tvrg2currenttrimR {
        Tvrg2currenttrimR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:27 - Calibrated voltage reference 580m trim"]
    #[inline(always)]
    pub fn tvrg2vreftrim(&self) -> Tvrg2vreftrimR {
        Tvrg2vreftrimR::new(((self.bits >> 21) & 0x7f) as u8)
    }
    #[doc = "Bit 28 - TVRG SEL VREF"]
    #[inline(always)]
    pub fn tvrgselvref(&self) -> TvrgselvrefR {
        TvrgselvrefR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TVRG2 SEL VREF"]
    #[inline(always)]
    pub fn tvrg2selvref(&self) -> Tvrg2selvrefR {
        Tvrg2selvrefR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Calibrated Voltage Reference Generator tc trim (bottom transistor)"]
    #[inline(always)]
    #[must_use]
    pub fn tvrgtempcotrim(&mut self) -> TvrgtempcotrimW<Vrefgen2Spec> {
        TvrgtempcotrimW::new(self, 0)
    }
    #[doc = "Bit 5 - Power Down, Calibrated Voltage Reference Generator."]
    #[inline(always)]
    #[must_use]
    pub fn tvrgpwd(&mut self) -> TvrgpwdW<Vrefgen2Spec> {
        TvrgpwdW::new(self, 5)
    }
    #[doc = "Bit 6 - Calibrated voltage reference current trim."]
    #[inline(always)]
    #[must_use]
    pub fn tvrgcurrenttrim(&mut self) -> TvrgcurrenttrimW<Vrefgen2Spec> {
        TvrgcurrenttrimW::new(self, 6)
    }
    #[doc = "Bits 7:13 - Calibrated voltage reference 580m trim"]
    #[inline(always)]
    #[must_use]
    pub fn tvrgvreftrim(&mut self) -> TvrgvreftrimW<Vrefgen2Spec> {
        TvrgvreftrimW::new(self, 7)
    }
    #[doc = "Bits 14:18 - Calibrated Voltage Reference Generator tc trim (bottom transistor)"]
    #[inline(always)]
    #[must_use]
    pub fn tvrg2tempcotrim(&mut self) -> Tvrg2tempcotrimW<Vrefgen2Spec> {
        Tvrg2tempcotrimW::new(self, 14)
    }
    #[doc = "Bit 19 - Power Down, Calibrated Voltage Reference Generator."]
    #[inline(always)]
    #[must_use]
    pub fn tvrg2pwd(&mut self) -> Tvrg2pwdW<Vrefgen2Spec> {
        Tvrg2pwdW::new(self, 19)
    }
    #[doc = "Bit 20 - Calibrated voltage reference current trim."]
    #[inline(always)]
    #[must_use]
    pub fn tvrg2currenttrim(&mut self) -> Tvrg2currenttrimW<Vrefgen2Spec> {
        Tvrg2currenttrimW::new(self, 20)
    }
    #[doc = "Bits 21:27 - Calibrated voltage reference 580m trim"]
    #[inline(always)]
    #[must_use]
    pub fn tvrg2vreftrim(&mut self) -> Tvrg2vreftrimW<Vrefgen2Spec> {
        Tvrg2vreftrimW::new(self, 21)
    }
    #[doc = "Bit 28 - TVRG SEL VREF"]
    #[inline(always)]
    #[must_use]
    pub fn tvrgselvref(&mut self) -> TvrgselvrefW<Vrefgen2Spec> {
        TvrgselvrefW::new(self, 28)
    }
    #[doc = "Bit 29 - TVRG2 SEL VREF"]
    #[inline(always)]
    #[must_use]
    pub fn tvrg2selvref(&mut self) -> Tvrg2selvrefW<Vrefgen2Spec> {
        Tvrg2selvrefW::new(self, 29)
    }
}
#[doc = "Voltage Reference Generator 2 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vrefgen2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrefgen2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vrefgen2Spec;
impl crate::RegisterSpec for Vrefgen2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vrefgen2::R`](R) reader structure"]
impl crate::Readable for Vrefgen2Spec {}
#[doc = "`write(|w| ..)` method takes [`vrefgen2::W`](W) writer structure"]
impl crate::Writable for Vrefgen2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VREFGEN2 to value 0x07c4_1f10"]
impl crate::Resettable for Vrefgen2Spec {
    const RESET_VALUE: u32 = 0x07c4_1f10;
}
