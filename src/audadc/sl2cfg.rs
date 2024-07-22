#[doc = "Register `SL2CFG` reader"]
pub type R = crate::R<Sl2cfgSpec>;
#[doc = "Register `SL2CFG` writer"]
pub type W = crate::W<Sl2cfgSpec>;
#[doc = "This bit enables slot 2 for AUDADC conversions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slen2 {
    #[doc = "1: Enable slot 2 for AUDADC conversions."]
    Slen = 1,
    #[doc = "0: Disable slot 2 for AUDADC conversions."]
    Sldis = 0,
}
impl From<Slen2> for bool {
    #[inline(always)]
    fn from(variant: Slen2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEN2` reader - This bit enables slot 2 for AUDADC conversions."]
pub type Slen2R = crate::BitReader<Slen2>;
impl Slen2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slen2 {
        match self.bits {
            true => Slen2::Slen,
            false => Slen2::Sldis,
        }
    }
    #[doc = "Enable slot 2 for AUDADC conversions."]
    #[inline(always)]
    pub fn is_slen(&self) -> bool {
        *self == Slen2::Slen
    }
    #[doc = "Disable slot 2 for AUDADC conversions."]
    #[inline(always)]
    pub fn is_sldis(&self) -> bool {
        *self == Slen2::Sldis
    }
}
#[doc = "Field `SLEN2` writer - This bit enables slot 2 for AUDADC conversions."]
pub type Slen2W<'a, REG> = crate::BitWriter<'a, REG, Slen2>;
impl<'a, REG> Slen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable slot 2 for AUDADC conversions."]
    #[inline(always)]
    pub fn slen(self) -> &'a mut crate::W<REG> {
        self.variant(Slen2::Slen)
    }
    #[doc = "Disable slot 2 for AUDADC conversions."]
    #[inline(always)]
    pub fn sldis(self) -> &'a mut crate::W<REG> {
        self.variant(Slen2::Sldis)
    }
}
#[doc = "This bit enables the window compare function for slot 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wcen2 {
    #[doc = "1: Enable the window compare for slot 2."]
    Wcen = 1,
    #[doc = "0: Disable the window compare for slot 2."]
    Wcdis = 0,
}
impl From<Wcen2> for bool {
    #[inline(always)]
    fn from(variant: Wcen2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WCEN2` reader - This bit enables the window compare function for slot 2."]
pub type Wcen2R = crate::BitReader<Wcen2>;
impl Wcen2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wcen2 {
        match self.bits {
            true => Wcen2::Wcen,
            false => Wcen2::Wcdis,
        }
    }
    #[doc = "Enable the window compare for slot 2."]
    #[inline(always)]
    pub fn is_wcen(&self) -> bool {
        *self == Wcen2::Wcen
    }
    #[doc = "Disable the window compare for slot 2."]
    #[inline(always)]
    pub fn is_wcdis(&self) -> bool {
        *self == Wcen2::Wcdis
    }
}
#[doc = "Field `WCEN2` writer - This bit enables the window compare function for slot 2."]
pub type Wcen2W<'a, REG> = crate::BitWriter<'a, REG, Wcen2>;
impl<'a, REG> Wcen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the window compare for slot 2."]
    #[inline(always)]
    pub fn wcen(self) -> &'a mut crate::W<REG> {
        self.variant(Wcen2::Wcen)
    }
    #[doc = "Disable the window compare for slot 2."]
    #[inline(always)]
    pub fn wcdis(self) -> &'a mut crate::W<REG> {
        self.variant(Wcen2::Wcdis)
    }
}
#[doc = "Select one of the 14 channel inputs for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chsel2 {
    #[doc = "0: PGA channel A0 output"]
    Se0 = 0,
    #[doc = "1: PGA channel A1 output"]
    Se1 = 1,
    #[doc = "2: PGA channel B0 output"]
    Se2 = 2,
    #[doc = "3: PGA channel B1 output"]
    Se3 = 3,
}
impl From<Chsel2> for u8 {
    #[inline(always)]
    fn from(variant: Chsel2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chsel2 {
    type Ux = u8;
}
impl crate::IsEnum for Chsel2 {}
#[doc = "Field `CHSEL2` reader - Select one of the 14 channel inputs for this slot."]
pub type Chsel2R = crate::FieldReader<Chsel2>;
impl Chsel2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Chsel2> {
        match self.bits {
            0 => Some(Chsel2::Se0),
            1 => Some(Chsel2::Se1),
            2 => Some(Chsel2::Se2),
            3 => Some(Chsel2::Se3),
            _ => None,
        }
    }
    #[doc = "PGA channel A0 output"]
    #[inline(always)]
    pub fn is_se0(&self) -> bool {
        *self == Chsel2::Se0
    }
    #[doc = "PGA channel A1 output"]
    #[inline(always)]
    pub fn is_se1(&self) -> bool {
        *self == Chsel2::Se1
    }
    #[doc = "PGA channel B0 output"]
    #[inline(always)]
    pub fn is_se2(&self) -> bool {
        *self == Chsel2::Se2
    }
    #[doc = "PGA channel B1 output"]
    #[inline(always)]
    pub fn is_se3(&self) -> bool {
        *self == Chsel2::Se3
    }
}
#[doc = "Field `CHSEL2` writer - Select one of the 14 channel inputs for this slot."]
pub type Chsel2W<'a, REG> = crate::FieldWriter<'a, REG, 4, Chsel2>;
impl<'a, REG> Chsel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PGA channel A0 output"]
    #[inline(always)]
    pub fn se0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel2::Se0)
    }
    #[doc = "PGA channel A1 output"]
    #[inline(always)]
    pub fn se1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel2::Se1)
    }
    #[doc = "PGA channel B0 output"]
    #[inline(always)]
    pub fn se2(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel2::Se2)
    }
    #[doc = "PGA channel B1 output"]
    #[inline(always)]
    pub fn se3(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel2::Se3)
    }
}
#[doc = "Set the Precision Mode For Slot 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prmode2 {
    #[doc = "0: 12-bit precision mode"]
    P12b0 = 0,
    #[doc = "1: 12-bit precision mode"]
    P12b1 = 1,
    #[doc = "2: 10-bit precision mode"]
    P10b = 2,
    #[doc = "3: 8-bit precision mode"]
    P8b = 3,
}
impl From<Prmode2> for u8 {
    #[inline(always)]
    fn from(variant: Prmode2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prmode2 {
    type Ux = u8;
}
impl crate::IsEnum for Prmode2 {}
#[doc = "Field `PRMODE2` reader - Set the Precision Mode For Slot 2."]
pub type Prmode2R = crate::FieldReader<Prmode2>;
impl Prmode2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prmode2 {
        match self.bits {
            0 => Prmode2::P12b0,
            1 => Prmode2::P12b1,
            2 => Prmode2::P10b,
            3 => Prmode2::P8b,
            _ => unreachable!(),
        }
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn is_p12b0(&self) -> bool {
        *self == Prmode2::P12b0
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn is_p12b1(&self) -> bool {
        *self == Prmode2::P12b1
    }
    #[doc = "10-bit precision mode"]
    #[inline(always)]
    pub fn is_p10b(&self) -> bool {
        *self == Prmode2::P10b
    }
    #[doc = "8-bit precision mode"]
    #[inline(always)]
    pub fn is_p8b(&self) -> bool {
        *self == Prmode2::P8b
    }
}
#[doc = "Field `PRMODE2` writer - Set the Precision Mode For Slot 2."]
pub type Prmode2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Prmode2, crate::Safe>;
impl<'a, REG> Prmode2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn p12b0(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode2::P12b0)
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn p12b1(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode2::P12b1)
    }
    #[doc = "10-bit precision mode"]
    #[inline(always)]
    pub fn p10b(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode2::P10b)
    }
    #[doc = "8-bit precision mode"]
    #[inline(always)]
    pub fn p8b(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode2::P8b)
    }
}
#[doc = "Field `TRKCYC2` reader - Set additional input signal sampling/tracking time to the specified number of AUDADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
pub type Trkcyc2R = crate::FieldReader;
#[doc = "Field `TRKCYC2` writer - Set additional input signal sampling/tracking time to the specified number of AUDADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
pub type Trkcyc2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Select the number of measurements to average in the accumulate divide module for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adsel2 {
    #[doc = "0: Average in 1 measurement in the accumulate divide module for this slot."]
    Avg1Msrmt = 0,
    #[doc = "1: Average in 2 measurements in the accumulate divide module for this slot."]
    Avg2Msrmts = 1,
    #[doc = "2: Average in 4 measurements in the accumulate divide module for this slot."]
    Avg4Msrmts = 2,
    #[doc = "3: Average in 8 measurements in the accumulate divide module for this slot."]
    Avg8Msrmt = 3,
    #[doc = "4: Average in 16 measurements in the accumulate divide module for this slot."]
    Avg16Msrmts = 4,
    #[doc = "5: Average in 32 measurements in the accumulate divide module for this slot."]
    Avg32Msrmts = 5,
    #[doc = "6: Average in 64 measurements in the accumulate divide module for this slot."]
    Avg64Msrmts = 6,
    #[doc = "7: Average in 128 measurements in the accumulate divide module for this slot."]
    Avg128Msrmts = 7,
}
impl From<Adsel2> for u8 {
    #[inline(always)]
    fn from(variant: Adsel2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adsel2 {
    type Ux = u8;
}
impl crate::IsEnum for Adsel2 {}
#[doc = "Field `ADSEL2` reader - Select the number of measurements to average in the accumulate divide module for this slot."]
pub type Adsel2R = crate::FieldReader<Adsel2>;
impl Adsel2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adsel2 {
        match self.bits {
            0 => Adsel2::Avg1Msrmt,
            1 => Adsel2::Avg2Msrmts,
            2 => Adsel2::Avg4Msrmts,
            3 => Adsel2::Avg8Msrmt,
            4 => Adsel2::Avg16Msrmts,
            5 => Adsel2::Avg32Msrmts,
            6 => Adsel2::Avg64Msrmts,
            7 => Adsel2::Avg128Msrmts,
            _ => unreachable!(),
        }
    }
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_1_msrmt(&self) -> bool {
        *self == Adsel2::Avg1Msrmt
    }
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_2_msrmts(&self) -> bool {
        *self == Adsel2::Avg2Msrmts
    }
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_4_msrmts(&self) -> bool {
        *self == Adsel2::Avg4Msrmts
    }
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_8_msrmt(&self) -> bool {
        *self == Adsel2::Avg8Msrmt
    }
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_16_msrmts(&self) -> bool {
        *self == Adsel2::Avg16Msrmts
    }
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_32_msrmts(&self) -> bool {
        *self == Adsel2::Avg32Msrmts
    }
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_64_msrmts(&self) -> bool {
        *self == Adsel2::Avg64Msrmts
    }
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_128_msrmts(&self) -> bool {
        *self == Adsel2::Avg128Msrmts
    }
}
#[doc = "Field `ADSEL2` writer - Select the number of measurements to average in the accumulate divide module for this slot."]
pub type Adsel2W<'a, REG> = crate::FieldWriter<'a, REG, 3, Adsel2, crate::Safe>;
impl<'a, REG> Adsel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_1_msrmt(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel2::Avg1Msrmt)
    }
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_2_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel2::Avg2Msrmts)
    }
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_4_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel2::Avg4Msrmts)
    }
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_8_msrmt(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel2::Avg8Msrmt)
    }
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_16_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel2::Avg16Msrmts)
    }
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_32_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel2::Avg32Msrmts)
    }
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_64_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel2::Avg64Msrmts)
    }
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_128_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel2::Avg128Msrmts)
    }
}
impl R {
    #[doc = "Bit 0 - This bit enables slot 2 for AUDADC conversions."]
    #[inline(always)]
    pub fn slen2(&self) -> Slen2R {
        Slen2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 2."]
    #[inline(always)]
    pub fn wcen2(&self) -> Wcen2R {
        Wcen2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Select one of the 14 channel inputs for this slot."]
    #[inline(always)]
    pub fn chsel2(&self) -> Chsel2R {
        Chsel2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot 2."]
    #[inline(always)]
    pub fn prmode2(&self) -> Prmode2R {
        Prmode2R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:23 - Set additional input signal sampling/tracking time to the specified number of AUDADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
    #[inline(always)]
    pub fn trkcyc2(&self) -> Trkcyc2R {
        Trkcyc2R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn adsel2(&self) -> Adsel2R {
        Adsel2R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This bit enables slot 2 for AUDADC conversions."]
    #[inline(always)]
    #[must_use]
    pub fn slen2(&mut self) -> Slen2W<Sl2cfgSpec> {
        Slen2W::new(self, 0)
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 2."]
    #[inline(always)]
    #[must_use]
    pub fn wcen2(&mut self) -> Wcen2W<Sl2cfgSpec> {
        Wcen2W::new(self, 1)
    }
    #[doc = "Bits 8:11 - Select one of the 14 channel inputs for this slot."]
    #[inline(always)]
    #[must_use]
    pub fn chsel2(&mut self) -> Chsel2W<Sl2cfgSpec> {
        Chsel2W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot 2."]
    #[inline(always)]
    #[must_use]
    pub fn prmode2(&mut self) -> Prmode2W<Sl2cfgSpec> {
        Prmode2W::new(self, 16)
    }
    #[doc = "Bits 18:23 - Set additional input signal sampling/tracking time to the specified number of AUDADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
    #[inline(always)]
    #[must_use]
    pub fn trkcyc2(&mut self) -> Trkcyc2W<Sl2cfgSpec> {
        Trkcyc2W::new(self, 18)
    }
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    #[must_use]
    pub fn adsel2(&mut self) -> Adsel2W<Sl2cfgSpec> {
        Adsel2W::new(self, 24)
    }
}
#[doc = "Slot 2 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sl2cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl2cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sl2cfgSpec;
impl crate::RegisterSpec for Sl2cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sl2cfg::R`](R) reader structure"]
impl crate::Readable for Sl2cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sl2cfg::W`](W) writer structure"]
impl crate::Writable for Sl2cfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SL2CFG to value 0"]
impl crate::Resettable for Sl2cfgSpec {
    const RESET_VALUE: u32 = 0;
}
