#[doc = "Register `SL0CFG` reader"]
pub type R = crate::R<Sl0cfgSpec>;
#[doc = "Register `SL0CFG` writer"]
pub type W = crate::W<Sl0cfgSpec>;
#[doc = "This bit enables slot 0 for ADC conversions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slen0 {
    #[doc = "1: Enable slot 0 for ADC conversions."]
    Slen = 1,
    #[doc = "0: Disable slot 0 for ADC conversions."]
    Sldis = 0,
}
impl From<Slen0> for bool {
    #[inline(always)]
    fn from(variant: Slen0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEN0` reader - This bit enables slot 0 for ADC conversions."]
pub type Slen0R = crate::BitReader<Slen0>;
impl Slen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slen0 {
        match self.bits {
            true => Slen0::Slen,
            false => Slen0::Sldis,
        }
    }
    #[doc = "Enable slot 0 for ADC conversions."]
    #[inline(always)]
    pub fn is_slen(&self) -> bool {
        *self == Slen0::Slen
    }
    #[doc = "Disable slot 0 for ADC conversions."]
    #[inline(always)]
    pub fn is_sldis(&self) -> bool {
        *self == Slen0::Sldis
    }
}
#[doc = "Field `SLEN0` writer - This bit enables slot 0 for ADC conversions."]
pub type Slen0W<'a, REG> = crate::BitWriter<'a, REG, Slen0>;
impl<'a, REG> Slen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable slot 0 for ADC conversions."]
    #[inline(always)]
    pub fn slen(self) -> &'a mut crate::W<REG> {
        self.variant(Slen0::Slen)
    }
    #[doc = "Disable slot 0 for ADC conversions."]
    #[inline(always)]
    pub fn sldis(self) -> &'a mut crate::W<REG> {
        self.variant(Slen0::Sldis)
    }
}
#[doc = "This bit enables the window compare function for slot 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wcen0 {
    #[doc = "1: Enable the window compare for slot 0."]
    Wcen = 1,
    #[doc = "0: Disable the window compare for slot 0."]
    Wcdis = 0,
}
impl From<Wcen0> for bool {
    #[inline(always)]
    fn from(variant: Wcen0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WCEN0` reader - This bit enables the window compare function for slot 0."]
pub type Wcen0R = crate::BitReader<Wcen0>;
impl Wcen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wcen0 {
        match self.bits {
            true => Wcen0::Wcen,
            false => Wcen0::Wcdis,
        }
    }
    #[doc = "Enable the window compare for slot 0."]
    #[inline(always)]
    pub fn is_wcen(&self) -> bool {
        *self == Wcen0::Wcen
    }
    #[doc = "Disable the window compare for slot 0."]
    #[inline(always)]
    pub fn is_wcdis(&self) -> bool {
        *self == Wcen0::Wcdis
    }
}
#[doc = "Field `WCEN0` writer - This bit enables the window compare function for slot 0."]
pub type Wcen0W<'a, REG> = crate::BitWriter<'a, REG, Wcen0>;
impl<'a, REG> Wcen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the window compare for slot 0."]
    #[inline(always)]
    pub fn wcen(self) -> &'a mut crate::W<REG> {
        self.variant(Wcen0::Wcen)
    }
    #[doc = "Disable the window compare for slot 0."]
    #[inline(always)]
    pub fn wcdis(self) -> &'a mut crate::W<REG> {
        self.variant(Wcen0::Wcdis)
    }
}
#[doc = "Select one of the 11 channel inputs for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chsel0 {
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
impl From<Chsel0> for u8 {
    #[inline(always)]
    fn from(variant: Chsel0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chsel0 {
    type Ux = u8;
}
impl crate::IsEnum for Chsel0 {}
#[doc = "Field `CHSEL0` reader - Select one of the 11 channel inputs for this slot."]
pub type Chsel0R = crate::FieldReader<Chsel0>;
impl Chsel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Chsel0> {
        match self.bits {
            0 => Some(Chsel0::Se0),
            1 => Some(Chsel0::Se1),
            2 => Some(Chsel0::Se2),
            3 => Some(Chsel0::Se3),
            4 => Some(Chsel0::Se4),
            5 => Some(Chsel0::Se5),
            6 => Some(Chsel0::Se6),
            7 => Some(Chsel0::Se7),
            8 => Some(Chsel0::Temp),
            9 => Some(Chsel0::Batt),
            11 => Some(Chsel0::Vss),
            _ => None,
        }
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se0(&self) -> bool {
        *self == Chsel0::Se0
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se1(&self) -> bool {
        *self == Chsel0::Se1
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se2(&self) -> bool {
        *self == Chsel0::Se2
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se3(&self) -> bool {
        *self == Chsel0::Se3
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se4(&self) -> bool {
        *self == Chsel0::Se4
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se5(&self) -> bool {
        *self == Chsel0::Se5
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se6(&self) -> bool {
        *self == Chsel0::Se6
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se7(&self) -> bool {
        *self == Chsel0::Se7
    }
    #[doc = "Internal temperature sensor."]
    #[inline(always)]
    pub fn is_temp(&self) -> bool {
        *self == Chsel0::Temp
    }
    #[doc = "Internal voltage divide-by-3 connection."]
    #[inline(always)]
    pub fn is_batt(&self) -> bool {
        *self == Chsel0::Batt
    }
    #[doc = "Input VSS."]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        *self == Chsel0::Vss
    }
}
#[doc = "Field `CHSEL0` writer - Select one of the 11 channel inputs for this slot."]
pub type Chsel0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Chsel0>;
impl<'a, REG> Chsel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel0::Se0)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel0::Se1)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se2(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel0::Se2)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se3(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel0::Se3)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se4(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel0::Se4)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se5(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel0::Se5)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se6(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel0::Se6)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se7(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel0::Se7)
    }
    #[doc = "Internal temperature sensor."]
    #[inline(always)]
    pub fn temp(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel0::Temp)
    }
    #[doc = "Internal voltage divide-by-3 connection."]
    #[inline(always)]
    pub fn batt(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel0::Batt)
    }
    #[doc = "Input VSS."]
    #[inline(always)]
    pub fn vss(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel0::Vss)
    }
}
#[doc = "Set the Precision Mode For Slot 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prmode0 {
    #[doc = "0: 12-bit precision mode"]
    P12b0 = 0,
    #[doc = "1: 12-bit precision mode"]
    P12b1 = 1,
    #[doc = "2: 10-bit precision mode"]
    P10b = 2,
    #[doc = "3: 8-bit precision mode"]
    P8b = 3,
}
impl From<Prmode0> for u8 {
    #[inline(always)]
    fn from(variant: Prmode0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prmode0 {
    type Ux = u8;
}
impl crate::IsEnum for Prmode0 {}
#[doc = "Field `PRMODE0` reader - Set the Precision Mode For Slot 0."]
pub type Prmode0R = crate::FieldReader<Prmode0>;
impl Prmode0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prmode0 {
        match self.bits {
            0 => Prmode0::P12b0,
            1 => Prmode0::P12b1,
            2 => Prmode0::P10b,
            3 => Prmode0::P8b,
            _ => unreachable!(),
        }
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn is_p12b0(&self) -> bool {
        *self == Prmode0::P12b0
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn is_p12b1(&self) -> bool {
        *self == Prmode0::P12b1
    }
    #[doc = "10-bit precision mode"]
    #[inline(always)]
    pub fn is_p10b(&self) -> bool {
        *self == Prmode0::P10b
    }
    #[doc = "8-bit precision mode"]
    #[inline(always)]
    pub fn is_p8b(&self) -> bool {
        *self == Prmode0::P8b
    }
}
#[doc = "Field `PRMODE0` writer - Set the Precision Mode For Slot 0."]
pub type Prmode0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Prmode0, crate::Safe>;
impl<'a, REG> Prmode0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn p12b0(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode0::P12b0)
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn p12b1(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode0::P12b1)
    }
    #[doc = "10-bit precision mode"]
    #[inline(always)]
    pub fn p10b(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode0::P10b)
    }
    #[doc = "8-bit precision mode"]
    #[inline(always)]
    pub fn p8b(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode0::P8b)
    }
}
#[doc = "Field `TRKCYC0` reader - Set additional input signal sampling/tracking time to the specified number of ADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
pub type Trkcyc0R = crate::FieldReader;
#[doc = "Field `TRKCYC0` writer - Set additional input signal sampling/tracking time to the specified number of ADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
pub type Trkcyc0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Select the number of measurements to average in the accumulate divide module for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adsel0 {
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
impl From<Adsel0> for u8 {
    #[inline(always)]
    fn from(variant: Adsel0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adsel0 {
    type Ux = u8;
}
impl crate::IsEnum for Adsel0 {}
#[doc = "Field `ADSEL0` reader - Select the number of measurements to average in the accumulate divide module for this slot."]
pub type Adsel0R = crate::FieldReader<Adsel0>;
impl Adsel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adsel0 {
        match self.bits {
            0 => Adsel0::Avg1Msrmt,
            1 => Adsel0::Avg2Msrmts,
            2 => Adsel0::Avg4Msrmts,
            3 => Adsel0::Avg8Msrmt,
            4 => Adsel0::Avg16Msrmts,
            5 => Adsel0::Avg32Msrmts,
            6 => Adsel0::Avg64Msrmts,
            7 => Adsel0::Avg128Msrmts,
            _ => unreachable!(),
        }
    }
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_1_msrmt(&self) -> bool {
        *self == Adsel0::Avg1Msrmt
    }
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_2_msrmts(&self) -> bool {
        *self == Adsel0::Avg2Msrmts
    }
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_4_msrmts(&self) -> bool {
        *self == Adsel0::Avg4Msrmts
    }
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_8_msrmt(&self) -> bool {
        *self == Adsel0::Avg8Msrmt
    }
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_16_msrmts(&self) -> bool {
        *self == Adsel0::Avg16Msrmts
    }
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_32_msrmts(&self) -> bool {
        *self == Adsel0::Avg32Msrmts
    }
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_64_msrmts(&self) -> bool {
        *self == Adsel0::Avg64Msrmts
    }
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_128_msrmts(&self) -> bool {
        *self == Adsel0::Avg128Msrmts
    }
}
#[doc = "Field `ADSEL0` writer - Select the number of measurements to average in the accumulate divide module for this slot."]
pub type Adsel0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Adsel0, crate::Safe>;
impl<'a, REG> Adsel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_1_msrmt(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel0::Avg1Msrmt)
    }
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_2_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel0::Avg2Msrmts)
    }
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_4_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel0::Avg4Msrmts)
    }
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_8_msrmt(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel0::Avg8Msrmt)
    }
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_16_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel0::Avg16Msrmts)
    }
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_32_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel0::Avg32Msrmts)
    }
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_64_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel0::Avg64Msrmts)
    }
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_128_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel0::Avg128Msrmts)
    }
}
impl R {
    #[doc = "Bit 0 - This bit enables slot 0 for ADC conversions."]
    #[inline(always)]
    pub fn slen0(&self) -> Slen0R {
        Slen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 0."]
    #[inline(always)]
    pub fn wcen0(&self) -> Wcen0R {
        Wcen0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Select one of the 11 channel inputs for this slot."]
    #[inline(always)]
    pub fn chsel0(&self) -> Chsel0R {
        Chsel0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot 0."]
    #[inline(always)]
    pub fn prmode0(&self) -> Prmode0R {
        Prmode0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:23 - Set additional input signal sampling/tracking time to the specified number of ADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
    #[inline(always)]
    pub fn trkcyc0(&self) -> Trkcyc0R {
        Trkcyc0R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn adsel0(&self) -> Adsel0R {
        Adsel0R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This bit enables slot 0 for ADC conversions."]
    #[inline(always)]
    #[must_use]
    pub fn slen0(&mut self) -> Slen0W<Sl0cfgSpec> {
        Slen0W::new(self, 0)
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 0."]
    #[inline(always)]
    #[must_use]
    pub fn wcen0(&mut self) -> Wcen0W<Sl0cfgSpec> {
        Wcen0W::new(self, 1)
    }
    #[doc = "Bits 8:11 - Select one of the 11 channel inputs for this slot."]
    #[inline(always)]
    #[must_use]
    pub fn chsel0(&mut self) -> Chsel0W<Sl0cfgSpec> {
        Chsel0W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot 0."]
    #[inline(always)]
    #[must_use]
    pub fn prmode0(&mut self) -> Prmode0W<Sl0cfgSpec> {
        Prmode0W::new(self, 16)
    }
    #[doc = "Bits 18:23 - Set additional input signal sampling/tracking time to the specified number of ADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
    #[inline(always)]
    #[must_use]
    pub fn trkcyc0(&mut self) -> Trkcyc0W<Sl0cfgSpec> {
        Trkcyc0W::new(self, 18)
    }
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    #[must_use]
    pub fn adsel0(&mut self) -> Adsel0W<Sl0cfgSpec> {
        Adsel0W::new(self, 24)
    }
}
#[doc = "Slot 0 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sl0cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl0cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sl0cfgSpec;
impl crate::RegisterSpec for Sl0cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sl0cfg::R`](R) reader structure"]
impl crate::Readable for Sl0cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sl0cfg::W`](W) writer structure"]
impl crate::Writable for Sl0cfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SL0CFG to value 0"]
impl crate::Resettable for Sl0cfgSpec {
    const RESET_VALUE: u32 = 0;
}
