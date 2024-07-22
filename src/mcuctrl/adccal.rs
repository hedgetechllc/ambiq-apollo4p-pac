#[doc = "Register `ADCCAL` reader"]
pub type R = crate::R<AdccalSpec>;
#[doc = "Register `ADCCAL` writer"]
pub type W = crate::W<AdccalSpec>;
#[doc = "Run ADC Calibration on initial power up sequence\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Calonpwrup {
    #[doc = "0: Disable automatic calibration on initial power up"]
    Dis = 0,
    #[doc = "1: Enable automatic calibration on initial power up"]
    En = 1,
}
impl From<Calonpwrup> for bool {
    #[inline(always)]
    fn from(variant: Calonpwrup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALONPWRUP` reader - Run ADC Calibration on initial power up sequence"]
pub type CalonpwrupR = crate::BitReader<Calonpwrup>;
impl CalonpwrupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Calonpwrup {
        match self.bits {
            false => Calonpwrup::Dis,
            true => Calonpwrup::En,
        }
    }
    #[doc = "Disable automatic calibration on initial power up"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Calonpwrup::Dis
    }
    #[doc = "Enable automatic calibration on initial power up"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Calonpwrup::En
    }
}
#[doc = "Field `CALONPWRUP` writer - Run ADC Calibration on initial power up sequence"]
pub type CalonpwrupW<'a, REG> = crate::BitWriter<'a, REG, Calonpwrup>;
impl<'a, REG> CalonpwrupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable automatic calibration on initial power up"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Calonpwrup::Dis)
    }
    #[doc = "Enable automatic calibration on initial power up"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Calonpwrup::En)
    }
}
#[doc = "Status for ADC Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adccalibrated {
    #[doc = "0: ADC is not calibrated"]
    False = 0,
    #[doc = "1: ADC is calibrated"]
    True = 1,
}
impl From<Adccalibrated> for bool {
    #[inline(always)]
    fn from(variant: Adccalibrated) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCCALIBRATED` reader - Status for ADC Calibration"]
pub type AdccalibratedR = crate::BitReader<Adccalibrated>;
impl AdccalibratedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adccalibrated {
        match self.bits {
            false => Adccalibrated::False,
            true => Adccalibrated::True,
        }
    }
    #[doc = "ADC is not calibrated"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Adccalibrated::False
    }
    #[doc = "ADC is calibrated"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Adccalibrated::True
    }
}
#[doc = "Field `ADCCALIBRATED` writer - Status for ADC Calibration"]
pub type AdccalibratedW<'a, REG> = crate::BitWriter<'a, REG, Adccalibrated>;
impl<'a, REG> AdccalibratedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC is not calibrated"]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Adccalibrated::False)
    }
    #[doc = "ADC is calibrated"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Adccalibrated::True)
    }
}
impl R {
    #[doc = "Bit 0 - Run ADC Calibration on initial power up sequence"]
    #[inline(always)]
    pub fn calonpwrup(&self) -> CalonpwrupR {
        CalonpwrupR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status for ADC Calibration"]
    #[inline(always)]
    pub fn adccalibrated(&self) -> AdccalibratedR {
        AdccalibratedR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Run ADC Calibration on initial power up sequence"]
    #[inline(always)]
    #[must_use]
    pub fn calonpwrup(&mut self) -> CalonpwrupW<AdccalSpec> {
        CalonpwrupW::new(self, 0)
    }
    #[doc = "Bit 1 - Status for ADC Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn adccalibrated(&mut self) -> AdccalibratedW<AdccalSpec> {
        AdccalibratedW::new(self, 1)
    }
}
#[doc = "ADC Calibration Control\n\nYou can [`read`](crate::Reg::read) this register and get [`adccal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adccal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdccalSpec;
impl crate::RegisterSpec for AdccalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adccal::R`](R) reader structure"]
impl crate::Readable for AdccalSpec {}
#[doc = "`write(|w| ..)` method takes [`adccal::W`](W) writer structure"]
impl crate::Writable for AdccalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCCAL to value 0x01"]
impl crate::Resettable for AdccalSpec {
    const RESET_VALUE: u32 = 0x01;
}
