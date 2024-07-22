#[doc = "Register `SL5CFG` reader"]
pub type R = crate::R<Sl5cfgSpec>;
#[doc = "Register `SL5CFG` writer"]
pub type W = crate::W<Sl5cfgSpec>;
#[doc = "This bit enables slot 5 for ADC conversions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slen5 {
    #[doc = "1: Enable slot 5 for ADC conversions."]
    Slen = 1,
    #[doc = "0: Disable slot 5 for ADC conversions."]
    Sldis = 0,
}
impl From<Slen5> for bool {
    #[inline(always)]
    fn from(variant: Slen5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEN5` reader - This bit enables slot 5 for ADC conversions."]
pub type Slen5R = crate::BitReader<Slen5>;
impl Slen5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slen5 {
        match self.bits {
            true => Slen5::Slen,
            false => Slen5::Sldis,
        }
    }
    #[doc = "Enable slot 5 for ADC conversions."]
    #[inline(always)]
    pub fn is_slen(&self) -> bool {
        *self == Slen5::Slen
    }
    #[doc = "Disable slot 5 for ADC conversions."]
    #[inline(always)]
    pub fn is_sldis(&self) -> bool {
        *self == Slen5::Sldis
    }
}
#[doc = "Field `SLEN5` writer - This bit enables slot 5 for ADC conversions."]
pub type Slen5W<'a, REG> = crate::BitWriter<'a, REG, Slen5>;
impl<'a, REG> Slen5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable slot 5 for ADC conversions."]
    #[inline(always)]
    pub fn slen(self) -> &'a mut crate::W<REG> {
        self.variant(Slen5::Slen)
    }
    #[doc = "Disable slot 5 for ADC conversions."]
    #[inline(always)]
    pub fn sldis(self) -> &'a mut crate::W<REG> {
        self.variant(Slen5::Sldis)
    }
}
#[doc = "This bit enables the window compare function for slot 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wcen5 {
    #[doc = "1: Enable the window compare for slot 5."]
    Wcen = 1,
    #[doc = "0: Disable the window compare for slot 5."]
    Wcdis = 0,
}
impl From<Wcen5> for bool {
    #[inline(always)]
    fn from(variant: Wcen5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WCEN5` reader - This bit enables the window compare function for slot 5."]
pub type Wcen5R = crate::BitReader<Wcen5>;
impl Wcen5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wcen5 {
        match self.bits {
            true => Wcen5::Wcen,
            false => Wcen5::Wcdis,
        }
    }
    #[doc = "Enable the window compare for slot 5."]
    #[inline(always)]
    pub fn is_wcen(&self) -> bool {
        *self == Wcen5::Wcen
    }
    #[doc = "Disable the window compare for slot 5."]
    #[inline(always)]
    pub fn is_wcdis(&self) -> bool {
        *self == Wcen5::Wcdis
    }
}
#[doc = "Field `WCEN5` writer - This bit enables the window compare function for slot 5."]
pub type Wcen5W<'a, REG> = crate::BitWriter<'a, REG, Wcen5>;
impl<'a, REG> Wcen5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the window compare for slot 5."]
    #[inline(always)]
    pub fn wcen(self) -> &'a mut crate::W<REG> {
        self.variant(Wcen5::Wcen)
    }
    #[doc = "Disable the window compare for slot 5."]
    #[inline(always)]
    pub fn wcdis(self) -> &'a mut crate::W<REG> {
        self.variant(Wcen5::Wcdis)
    }
}
#[doc = "Select one of the 11 channel inputs for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chsel5 {
    #[doc = "0: Single ended external GPIO connection."]
    Se0 = 0,
    #[doc = "1: Single ended external GPIO connection."]
    Se1 = 1,
    #[doc = "2: Single ended external GPIO connection."]
    Se2 = 2,
    #[doc = "3: Single ended external GPIO connection."]
    Se3 = 3,
    #[doc = "4: Single ended external GPIO connection."]
    Se4 = 4,
    #[doc = "5: Single ended external GPIO connection."]
    Se5 = 5,
    #[doc = "6: Single ended external GPIO connection."]
    Se6 = 6,
    #[doc = "7: Single ended external GPIO connection."]
    Se7 = 7,
    #[doc = "8: Internal temperature sensor."]
    Temp = 8,
    #[doc = "9: Internal voltage divide-by-3 connection."]
    Batt = 9,
    #[doc = "11: Input VSS."]
    Vss = 11,
}
impl From<Chsel5> for u8 {
    #[inline(always)]
    fn from(variant: Chsel5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chsel5 {
    type Ux = u8;
}
impl crate::IsEnum for Chsel5 {}
#[doc = "Field `CHSEL5` reader - Select one of the 11 channel inputs for this slot."]
pub type Chsel5R = crate::FieldReader<Chsel5>;
impl Chsel5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Chsel5> {
        match self.bits {
            0 => Some(Chsel5::Se0),
            1 => Some(Chsel5::Se1),
            2 => Some(Chsel5::Se2),
            3 => Some(Chsel5::Se3),
            4 => Some(Chsel5::Se4),
            5 => Some(Chsel5::Se5),
            6 => Some(Chsel5::Se6),
            7 => Some(Chsel5::Se7),
            8 => Some(Chsel5::Temp),
            9 => Some(Chsel5::Batt),
            11 => Some(Chsel5::Vss),
            _ => None,
        }
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se0(&self) -> bool {
        *self == Chsel5::Se0
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se1(&self) -> bool {
        *self == Chsel5::Se1
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se2(&self) -> bool {
        *self == Chsel5::Se2
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se3(&self) -> bool {
        *self == Chsel5::Se3
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se4(&self) -> bool {
        *self == Chsel5::Se4
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se5(&self) -> bool {
        *self == Chsel5::Se5
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se6(&self) -> bool {
        *self == Chsel5::Se6
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se7(&self) -> bool {
        *self == Chsel5::Se7
    }
    #[doc = "Internal temperature sensor."]
    #[inline(always)]
    pub fn is_temp(&self) -> bool {
        *self == Chsel5::Temp
    }
    #[doc = "Internal voltage divide-by-3 connection."]
    #[inline(always)]
    pub fn is_batt(&self) -> bool {
        *self == Chsel5::Batt
    }
    #[doc = "Input VSS."]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        *self == Chsel5::Vss
    }
}
#[doc = "Field `CHSEL5` writer - Select one of the 11 channel inputs for this slot."]
pub type Chsel5W<'a, REG> = crate::FieldWriter<'a, REG, 4, Chsel5>;
impl<'a, REG> Chsel5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel5::Se0)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel5::Se1)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se2(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel5::Se2)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se3(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel5::Se3)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se4(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel5::Se4)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se5(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel5::Se5)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se6(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel5::Se6)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se7(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel5::Se7)
    }
    #[doc = "Internal temperature sensor."]
    #[inline(always)]
    pub fn temp(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel5::Temp)
    }
    #[doc = "Internal voltage divide-by-3 connection."]
    #[inline(always)]
    pub fn batt(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel5::Batt)
    }
    #[doc = "Input VSS."]
    #[inline(always)]
    pub fn vss(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel5::Vss)
    }
}
#[doc = "Set the Precision Mode For Slot 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prmode5 {
    #[doc = "0: 12-bit precision mode"]
    P12b0 = 0,
    #[doc = "1: 12-bit precision mode"]
    P12b1 = 1,
    #[doc = "2: 10-bit precision mode"]
    P10b = 2,
    #[doc = "3: 8-bit precision mode"]
    P8b = 3,
}
impl From<Prmode5> for u8 {
    #[inline(always)]
    fn from(variant: Prmode5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prmode5 {
    type Ux = u8;
}
impl crate::IsEnum for Prmode5 {}
#[doc = "Field `PRMODE5` reader - Set the Precision Mode For Slot 5."]
pub type Prmode5R = crate::FieldReader<Prmode5>;
impl Prmode5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prmode5 {
        match self.bits {
            0 => Prmode5::P12b0,
            1 => Prmode5::P12b1,
            2 => Prmode5::P10b,
            3 => Prmode5::P8b,
            _ => unreachable!(),
        }
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn is_p12b0(&self) -> bool {
        *self == Prmode5::P12b0
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn is_p12b1(&self) -> bool {
        *self == Prmode5::P12b1
    }
    #[doc = "10-bit precision mode"]
    #[inline(always)]
    pub fn is_p10b(&self) -> bool {
        *self == Prmode5::P10b
    }
    #[doc = "8-bit precision mode"]
    #[inline(always)]
    pub fn is_p8b(&self) -> bool {
        *self == Prmode5::P8b
    }
}
#[doc = "Field `PRMODE5` writer - Set the Precision Mode For Slot 5."]
pub type Prmode5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Prmode5, crate::Safe>;
impl<'a, REG> Prmode5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn p12b0(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode5::P12b0)
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn p12b1(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode5::P12b1)
    }
    #[doc = "10-bit precision mode"]
    #[inline(always)]
    pub fn p10b(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode5::P10b)
    }
    #[doc = "8-bit precision mode"]
    #[inline(always)]
    pub fn p8b(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode5::P8b)
    }
}
#[doc = "Field `TRKCYC5` reader - Set additional input signal sampling/tracking time to the specified number of ADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
pub type Trkcyc5R = crate::FieldReader;
#[doc = "Field `TRKCYC5` writer - Set additional input signal sampling/tracking time to the specified number of ADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
pub type Trkcyc5W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Select the number of measurements to average in the accumulate divide module for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adsel5 {
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
impl From<Adsel5> for u8 {
    #[inline(always)]
    fn from(variant: Adsel5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adsel5 {
    type Ux = u8;
}
impl crate::IsEnum for Adsel5 {}
#[doc = "Field `ADSEL5` reader - Select the number of measurements to average in the accumulate divide module for this slot."]
pub type Adsel5R = crate::FieldReader<Adsel5>;
impl Adsel5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adsel5 {
        match self.bits {
            0 => Adsel5::Avg1Msrmt,
            1 => Adsel5::Avg2Msrmts,
            2 => Adsel5::Avg4Msrmts,
            3 => Adsel5::Avg8Msrmt,
            4 => Adsel5::Avg16Msrmts,
            5 => Adsel5::Avg32Msrmts,
            6 => Adsel5::Avg64Msrmts,
            7 => Adsel5::Avg128Msrmts,
            _ => unreachable!(),
        }
    }
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_1_msrmt(&self) -> bool {
        *self == Adsel5::Avg1Msrmt
    }
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_2_msrmts(&self) -> bool {
        *self == Adsel5::Avg2Msrmts
    }
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_4_msrmts(&self) -> bool {
        *self == Adsel5::Avg4Msrmts
    }
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_8_msrmt(&self) -> bool {
        *self == Adsel5::Avg8Msrmt
    }
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_16_msrmts(&self) -> bool {
        *self == Adsel5::Avg16Msrmts
    }
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_32_msrmts(&self) -> bool {
        *self == Adsel5::Avg32Msrmts
    }
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_64_msrmts(&self) -> bool {
        *self == Adsel5::Avg64Msrmts
    }
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_128_msrmts(&self) -> bool {
        *self == Adsel5::Avg128Msrmts
    }
}
#[doc = "Field `ADSEL5` writer - Select the number of measurements to average in the accumulate divide module for this slot."]
pub type Adsel5W<'a, REG> = crate::FieldWriter<'a, REG, 3, Adsel5, crate::Safe>;
impl<'a, REG> Adsel5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_1_msrmt(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel5::Avg1Msrmt)
    }
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_2_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel5::Avg2Msrmts)
    }
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_4_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel5::Avg4Msrmts)
    }
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_8_msrmt(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel5::Avg8Msrmt)
    }
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_16_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel5::Avg16Msrmts)
    }
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_32_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel5::Avg32Msrmts)
    }
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_64_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel5::Avg64Msrmts)
    }
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_128_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel5::Avg128Msrmts)
    }
}
impl R {
    #[doc = "Bit 0 - This bit enables slot 5 for ADC conversions."]
    #[inline(always)]
    pub fn slen5(&self) -> Slen5R {
        Slen5R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 5."]
    #[inline(always)]
    pub fn wcen5(&self) -> Wcen5R {
        Wcen5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Select one of the 11 channel inputs for this slot."]
    #[inline(always)]
    pub fn chsel5(&self) -> Chsel5R {
        Chsel5R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot 5."]
    #[inline(always)]
    pub fn prmode5(&self) -> Prmode5R {
        Prmode5R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:23 - Set additional input signal sampling/tracking time to the specified number of ADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
    #[inline(always)]
    pub fn trkcyc5(&self) -> Trkcyc5R {
        Trkcyc5R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn adsel5(&self) -> Adsel5R {
        Adsel5R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This bit enables slot 5 for ADC conversions."]
    #[inline(always)]
    #[must_use]
    pub fn slen5(&mut self) -> Slen5W<Sl5cfgSpec> {
        Slen5W::new(self, 0)
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 5."]
    #[inline(always)]
    #[must_use]
    pub fn wcen5(&mut self) -> Wcen5W<Sl5cfgSpec> {
        Wcen5W::new(self, 1)
    }
    #[doc = "Bits 8:11 - Select one of the 11 channel inputs for this slot."]
    #[inline(always)]
    #[must_use]
    pub fn chsel5(&mut self) -> Chsel5W<Sl5cfgSpec> {
        Chsel5W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot 5."]
    #[inline(always)]
    #[must_use]
    pub fn prmode5(&mut self) -> Prmode5W<Sl5cfgSpec> {
        Prmode5W::new(self, 16)
    }
    #[doc = "Bits 18:23 - Set additional input signal sampling/tracking time to the specified number of ADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
    #[inline(always)]
    #[must_use]
    pub fn trkcyc5(&mut self) -> Trkcyc5W<Sl5cfgSpec> {
        Trkcyc5W::new(self, 18)
    }
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    #[must_use]
    pub fn adsel5(&mut self) -> Adsel5W<Sl5cfgSpec> {
        Adsel5W::new(self, 24)
    }
}
#[doc = "Slot 5 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sl5cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl5cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sl5cfgSpec;
impl crate::RegisterSpec for Sl5cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sl5cfg::R`](R) reader structure"]
impl crate::Readable for Sl5cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sl5cfg::W`](W) writer structure"]
impl crate::Writable for Sl5cfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SL5CFG to value 0"]
impl crate::Resettable for Sl5cfgSpec {
    const RESET_VALUE: u32 = 0;
}
