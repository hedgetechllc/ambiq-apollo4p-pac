#[doc = "Register `SL7CFG` reader"]
pub type R = crate::R<Sl7cfgSpec>;
#[doc = "Register `SL7CFG` writer"]
pub type W = crate::W<Sl7cfgSpec>;
#[doc = "This bit enables slot 7 for AUDADC conversions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slen7 {
    #[doc = "1: Enable slot 7 for AUDADC conversions."]
    Slen = 1,
    #[doc = "0: Disable slot 7 for AUDADC conversions."]
    Sldis = 0,
}
impl From<Slen7> for bool {
    #[inline(always)]
    fn from(variant: Slen7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEN7` reader - This bit enables slot 7 for AUDADC conversions."]
pub type Slen7R = crate::BitReader<Slen7>;
impl Slen7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slen7 {
        match self.bits {
            true => Slen7::Slen,
            false => Slen7::Sldis,
        }
    }
    #[doc = "Enable slot 7 for AUDADC conversions."]
    #[inline(always)]
    pub fn is_slen(&self) -> bool {
        *self == Slen7::Slen
    }
    #[doc = "Disable slot 7 for AUDADC conversions."]
    #[inline(always)]
    pub fn is_sldis(&self) -> bool {
        *self == Slen7::Sldis
    }
}
#[doc = "Field `SLEN7` writer - This bit enables slot 7 for AUDADC conversions."]
pub type Slen7W<'a, REG> = crate::BitWriter<'a, REG, Slen7>;
impl<'a, REG> Slen7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable slot 7 for AUDADC conversions."]
    #[inline(always)]
    pub fn slen(self) -> &'a mut crate::W<REG> {
        self.variant(Slen7::Slen)
    }
    #[doc = "Disable slot 7 for AUDADC conversions."]
    #[inline(always)]
    pub fn sldis(self) -> &'a mut crate::W<REG> {
        self.variant(Slen7::Sldis)
    }
}
#[doc = "This bit enables the window compare function for slot 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wcen7 {
    #[doc = "1: Enable the window compare for slot 7."]
    Wcen = 1,
    #[doc = "0: Disable the window compare for slot 7."]
    Wcdis = 0,
}
impl From<Wcen7> for bool {
    #[inline(always)]
    fn from(variant: Wcen7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WCEN7` reader - This bit enables the window compare function for slot 7."]
pub type Wcen7R = crate::BitReader<Wcen7>;
impl Wcen7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wcen7 {
        match self.bits {
            true => Wcen7::Wcen,
            false => Wcen7::Wcdis,
        }
    }
    #[doc = "Enable the window compare for slot 7."]
    #[inline(always)]
    pub fn is_wcen(&self) -> bool {
        *self == Wcen7::Wcen
    }
    #[doc = "Disable the window compare for slot 7."]
    #[inline(always)]
    pub fn is_wcdis(&self) -> bool {
        *self == Wcen7::Wcdis
    }
}
#[doc = "Field `WCEN7` writer - This bit enables the window compare function for slot 7."]
pub type Wcen7W<'a, REG> = crate::BitWriter<'a, REG, Wcen7>;
impl<'a, REG> Wcen7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the window compare for slot 7."]
    #[inline(always)]
    pub fn wcen(self) -> &'a mut crate::W<REG> {
        self.variant(Wcen7::Wcen)
    }
    #[doc = "Disable the window compare for slot 7."]
    #[inline(always)]
    pub fn wcdis(self) -> &'a mut crate::W<REG> {
        self.variant(Wcen7::Wcdis)
    }
}
#[doc = "Select one of the 14 channel inputs for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chsel7 {
    #[doc = "0: PGA channel A0 output"]
    Se0 = 0,
    #[doc = "1: PGA channel A1 output"]
    Se1 = 1,
    #[doc = "2: PGA channel B0 output"]
    Se2 = 2,
    #[doc = "3: PGA channel B1 output"]
    Se3 = 3,
}
impl From<Chsel7> for u8 {
    #[inline(always)]
    fn from(variant: Chsel7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chsel7 {
    type Ux = u8;
}
impl crate::IsEnum for Chsel7 {}
#[doc = "Field `CHSEL7` reader - Select one of the 14 channel inputs for this slot."]
pub type Chsel7R = crate::FieldReader<Chsel7>;
impl Chsel7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Chsel7> {
        match self.bits {
            0 => Some(Chsel7::Se0),
            1 => Some(Chsel7::Se1),
            2 => Some(Chsel7::Se2),
            3 => Some(Chsel7::Se3),
            _ => None,
        }
    }
    #[doc = "PGA channel A0 output"]
    #[inline(always)]
    pub fn is_se0(&self) -> bool {
        *self == Chsel7::Se0
    }
    #[doc = "PGA channel A1 output"]
    #[inline(always)]
    pub fn is_se1(&self) -> bool {
        *self == Chsel7::Se1
    }
    #[doc = "PGA channel B0 output"]
    #[inline(always)]
    pub fn is_se2(&self) -> bool {
        *self == Chsel7::Se2
    }
    #[doc = "PGA channel B1 output"]
    #[inline(always)]
    pub fn is_se3(&self) -> bool {
        *self == Chsel7::Se3
    }
}
#[doc = "Field `CHSEL7` writer - Select one of the 14 channel inputs for this slot."]
pub type Chsel7W<'a, REG> = crate::FieldWriter<'a, REG, 4, Chsel7>;
impl<'a, REG> Chsel7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PGA channel A0 output"]
    #[inline(always)]
    pub fn se0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel7::Se0)
    }
    #[doc = "PGA channel A1 output"]
    #[inline(always)]
    pub fn se1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel7::Se1)
    }
    #[doc = "PGA channel B0 output"]
    #[inline(always)]
    pub fn se2(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel7::Se2)
    }
    #[doc = "PGA channel B1 output"]
    #[inline(always)]
    pub fn se3(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel7::Se3)
    }
}
#[doc = "Set the Precision Mode For Slot 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prmode7 {
    #[doc = "0: 12-bit precision mode"]
    P12b0 = 0,
    #[doc = "1: 12-bit precision mode"]
    P12b1 = 1,
    #[doc = "2: 10-bit precision mode"]
    P10b = 2,
    #[doc = "3: 8-bit precision mode"]
    P8b = 3,
}
impl From<Prmode7> for u8 {
    #[inline(always)]
    fn from(variant: Prmode7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prmode7 {
    type Ux = u8;
}
impl crate::IsEnum for Prmode7 {}
#[doc = "Field `PRMODE7` reader - Set the Precision Mode For Slot 7."]
pub type Prmode7R = crate::FieldReader<Prmode7>;
impl Prmode7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prmode7 {
        match self.bits {
            0 => Prmode7::P12b0,
            1 => Prmode7::P12b1,
            2 => Prmode7::P10b,
            3 => Prmode7::P8b,
            _ => unreachable!(),
        }
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn is_p12b0(&self) -> bool {
        *self == Prmode7::P12b0
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn is_p12b1(&self) -> bool {
        *self == Prmode7::P12b1
    }
    #[doc = "10-bit precision mode"]
    #[inline(always)]
    pub fn is_p10b(&self) -> bool {
        *self == Prmode7::P10b
    }
    #[doc = "8-bit precision mode"]
    #[inline(always)]
    pub fn is_p8b(&self) -> bool {
        *self == Prmode7::P8b
    }
}
#[doc = "Field `PRMODE7` writer - Set the Precision Mode For Slot 7."]
pub type Prmode7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Prmode7, crate::Safe>;
impl<'a, REG> Prmode7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn p12b0(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode7::P12b0)
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn p12b1(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode7::P12b1)
    }
    #[doc = "10-bit precision mode"]
    #[inline(always)]
    pub fn p10b(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode7::P10b)
    }
    #[doc = "8-bit precision mode"]
    #[inline(always)]
    pub fn p8b(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode7::P8b)
    }
}
#[doc = "Field `TRKCYC7` reader - Set additional input signal sampling/tracking time to the specified number of AUDADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
pub type Trkcyc7R = crate::FieldReader;
#[doc = "Field `TRKCYC7` writer - Set additional input signal sampling/tracking time to the specified number of AUDADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
pub type Trkcyc7W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Select the number of measurements to average in the accumulate divide module for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adsel7 {
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
impl From<Adsel7> for u8 {
    #[inline(always)]
    fn from(variant: Adsel7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adsel7 {
    type Ux = u8;
}
impl crate::IsEnum for Adsel7 {}
#[doc = "Field `ADSEL7` reader - Select the number of measurements to average in the accumulate divide module for this slot."]
pub type Adsel7R = crate::FieldReader<Adsel7>;
impl Adsel7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adsel7 {
        match self.bits {
            0 => Adsel7::Avg1Msrmt,
            1 => Adsel7::Avg2Msrmts,
            2 => Adsel7::Avg4Msrmts,
            3 => Adsel7::Avg8Msrmt,
            4 => Adsel7::Avg16Msrmts,
            5 => Adsel7::Avg32Msrmts,
            6 => Adsel7::Avg64Msrmts,
            7 => Adsel7::Avg128Msrmts,
            _ => unreachable!(),
        }
    }
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_1_msrmt(&self) -> bool {
        *self == Adsel7::Avg1Msrmt
    }
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_2_msrmts(&self) -> bool {
        *self == Adsel7::Avg2Msrmts
    }
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_4_msrmts(&self) -> bool {
        *self == Adsel7::Avg4Msrmts
    }
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_8_msrmt(&self) -> bool {
        *self == Adsel7::Avg8Msrmt
    }
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_16_msrmts(&self) -> bool {
        *self == Adsel7::Avg16Msrmts
    }
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_32_msrmts(&self) -> bool {
        *self == Adsel7::Avg32Msrmts
    }
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_64_msrmts(&self) -> bool {
        *self == Adsel7::Avg64Msrmts
    }
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_128_msrmts(&self) -> bool {
        *self == Adsel7::Avg128Msrmts
    }
}
#[doc = "Field `ADSEL7` writer - Select the number of measurements to average in the accumulate divide module for this slot."]
pub type Adsel7W<'a, REG> = crate::FieldWriter<'a, REG, 3, Adsel7, crate::Safe>;
impl<'a, REG> Adsel7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_1_msrmt(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel7::Avg1Msrmt)
    }
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_2_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel7::Avg2Msrmts)
    }
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_4_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel7::Avg4Msrmts)
    }
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_8_msrmt(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel7::Avg8Msrmt)
    }
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_16_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel7::Avg16Msrmts)
    }
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_32_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel7::Avg32Msrmts)
    }
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_64_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel7::Avg64Msrmts)
    }
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_128_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel7::Avg128Msrmts)
    }
}
impl R {
    #[doc = "Bit 0 - This bit enables slot 7 for AUDADC conversions."]
    #[inline(always)]
    pub fn slen7(&self) -> Slen7R {
        Slen7R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 7."]
    #[inline(always)]
    pub fn wcen7(&self) -> Wcen7R {
        Wcen7R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Select one of the 14 channel inputs for this slot."]
    #[inline(always)]
    pub fn chsel7(&self) -> Chsel7R {
        Chsel7R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot 7."]
    #[inline(always)]
    pub fn prmode7(&self) -> Prmode7R {
        Prmode7R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:23 - Set additional input signal sampling/tracking time to the specified number of AUDADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
    #[inline(always)]
    pub fn trkcyc7(&self) -> Trkcyc7R {
        Trkcyc7R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn adsel7(&self) -> Adsel7R {
        Adsel7R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This bit enables slot 7 for AUDADC conversions."]
    #[inline(always)]
    #[must_use]
    pub fn slen7(&mut self) -> Slen7W<Sl7cfgSpec> {
        Slen7W::new(self, 0)
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 7."]
    #[inline(always)]
    #[must_use]
    pub fn wcen7(&mut self) -> Wcen7W<Sl7cfgSpec> {
        Wcen7W::new(self, 1)
    }
    #[doc = "Bits 8:11 - Select one of the 14 channel inputs for this slot."]
    #[inline(always)]
    #[must_use]
    pub fn chsel7(&mut self) -> Chsel7W<Sl7cfgSpec> {
        Chsel7W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot 7."]
    #[inline(always)]
    #[must_use]
    pub fn prmode7(&mut self) -> Prmode7W<Sl7cfgSpec> {
        Prmode7W::new(self, 16)
    }
    #[doc = "Bits 18:23 - Set additional input signal sampling/tracking time to the specified number of AUDADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
    #[inline(always)]
    #[must_use]
    pub fn trkcyc7(&mut self) -> Trkcyc7W<Sl7cfgSpec> {
        Trkcyc7W::new(self, 18)
    }
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    #[must_use]
    pub fn adsel7(&mut self) -> Adsel7W<Sl7cfgSpec> {
        Adsel7W::new(self, 24)
    }
}
#[doc = "Slot 7 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sl7cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl7cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sl7cfgSpec;
impl crate::RegisterSpec for Sl7cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sl7cfg::R`](R) reader structure"]
impl crate::Readable for Sl7cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sl7cfg::W`](W) writer structure"]
impl crate::Writable for Sl7cfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SL7CFG to value 0"]
impl crate::Resettable for Sl7cfgSpec {
    const RESET_VALUE: u32 = 0;
}
