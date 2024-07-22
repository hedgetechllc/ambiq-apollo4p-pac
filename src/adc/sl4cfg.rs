#[doc = "Register `SL4CFG` reader"]
pub type R = crate::R<Sl4cfgSpec>;
#[doc = "Register `SL4CFG` writer"]
pub type W = crate::W<Sl4cfgSpec>;
#[doc = "This bit enables slot 4 for ADC conversions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slen4 {
    #[doc = "1: Enable slot 4 for ADC conversions."]
    Slen = 1,
    #[doc = "0: Disable slot 4 for ADC conversions."]
    Sldis = 0,
}
impl From<Slen4> for bool {
    #[inline(always)]
    fn from(variant: Slen4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEN4` reader - This bit enables slot 4 for ADC conversions."]
pub type Slen4R = crate::BitReader<Slen4>;
impl Slen4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slen4 {
        match self.bits {
            true => Slen4::Slen,
            false => Slen4::Sldis,
        }
    }
    #[doc = "Enable slot 4 for ADC conversions."]
    #[inline(always)]
    pub fn is_slen(&self) -> bool {
        *self == Slen4::Slen
    }
    #[doc = "Disable slot 4 for ADC conversions."]
    #[inline(always)]
    pub fn is_sldis(&self) -> bool {
        *self == Slen4::Sldis
    }
}
#[doc = "Field `SLEN4` writer - This bit enables slot 4 for ADC conversions."]
pub type Slen4W<'a, REG> = crate::BitWriter<'a, REG, Slen4>;
impl<'a, REG> Slen4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable slot 4 for ADC conversions."]
    #[inline(always)]
    pub fn slen(self) -> &'a mut crate::W<REG> {
        self.variant(Slen4::Slen)
    }
    #[doc = "Disable slot 4 for ADC conversions."]
    #[inline(always)]
    pub fn sldis(self) -> &'a mut crate::W<REG> {
        self.variant(Slen4::Sldis)
    }
}
#[doc = "This bit enables the window compare function for slot 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wcen4 {
    #[doc = "1: Enable the window compare for slot 4."]
    Wcen = 1,
    #[doc = "0: Disable the window compare for slot 4."]
    Wcdis = 0,
}
impl From<Wcen4> for bool {
    #[inline(always)]
    fn from(variant: Wcen4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WCEN4` reader - This bit enables the window compare function for slot 4."]
pub type Wcen4R = crate::BitReader<Wcen4>;
impl Wcen4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wcen4 {
        match self.bits {
            true => Wcen4::Wcen,
            false => Wcen4::Wcdis,
        }
    }
    #[doc = "Enable the window compare for slot 4."]
    #[inline(always)]
    pub fn is_wcen(&self) -> bool {
        *self == Wcen4::Wcen
    }
    #[doc = "Disable the window compare for slot 4."]
    #[inline(always)]
    pub fn is_wcdis(&self) -> bool {
        *self == Wcen4::Wcdis
    }
}
#[doc = "Field `WCEN4` writer - This bit enables the window compare function for slot 4."]
pub type Wcen4W<'a, REG> = crate::BitWriter<'a, REG, Wcen4>;
impl<'a, REG> Wcen4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the window compare for slot 4."]
    #[inline(always)]
    pub fn wcen(self) -> &'a mut crate::W<REG> {
        self.variant(Wcen4::Wcen)
    }
    #[doc = "Disable the window compare for slot 4."]
    #[inline(always)]
    pub fn wcdis(self) -> &'a mut crate::W<REG> {
        self.variant(Wcen4::Wcdis)
    }
}
#[doc = "Select one of the 11 channel inputs for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chsel4 {
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
impl From<Chsel4> for u8 {
    #[inline(always)]
    fn from(variant: Chsel4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chsel4 {
    type Ux = u8;
}
impl crate::IsEnum for Chsel4 {}
#[doc = "Field `CHSEL4` reader - Select one of the 11 channel inputs for this slot."]
pub type Chsel4R = crate::FieldReader<Chsel4>;
impl Chsel4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Chsel4> {
        match self.bits {
            0 => Some(Chsel4::Se0),
            1 => Some(Chsel4::Se1),
            2 => Some(Chsel4::Se2),
            3 => Some(Chsel4::Se3),
            4 => Some(Chsel4::Se4),
            5 => Some(Chsel4::Se5),
            6 => Some(Chsel4::Se6),
            7 => Some(Chsel4::Se7),
            8 => Some(Chsel4::Temp),
            9 => Some(Chsel4::Batt),
            11 => Some(Chsel4::Vss),
            _ => None,
        }
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se0(&self) -> bool {
        *self == Chsel4::Se0
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se1(&self) -> bool {
        *self == Chsel4::Se1
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se2(&self) -> bool {
        *self == Chsel4::Se2
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se3(&self) -> bool {
        *self == Chsel4::Se3
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se4(&self) -> bool {
        *self == Chsel4::Se4
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se5(&self) -> bool {
        *self == Chsel4::Se5
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se6(&self) -> bool {
        *self == Chsel4::Se6
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se7(&self) -> bool {
        *self == Chsel4::Se7
    }
    #[doc = "Internal temperature sensor."]
    #[inline(always)]
    pub fn is_temp(&self) -> bool {
        *self == Chsel4::Temp
    }
    #[doc = "Internal voltage divide-by-3 connection."]
    #[inline(always)]
    pub fn is_batt(&self) -> bool {
        *self == Chsel4::Batt
    }
    #[doc = "Input VSS."]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        *self == Chsel4::Vss
    }
}
#[doc = "Field `CHSEL4` writer - Select one of the 11 channel inputs for this slot."]
pub type Chsel4W<'a, REG> = crate::FieldWriter<'a, REG, 4, Chsel4>;
impl<'a, REG> Chsel4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel4::Se0)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel4::Se1)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se2(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel4::Se2)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se3(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel4::Se3)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se4(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel4::Se4)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se5(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel4::Se5)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se6(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel4::Se6)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se7(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel4::Se7)
    }
    #[doc = "Internal temperature sensor."]
    #[inline(always)]
    pub fn temp(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel4::Temp)
    }
    #[doc = "Internal voltage divide-by-3 connection."]
    #[inline(always)]
    pub fn batt(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel4::Batt)
    }
    #[doc = "Input VSS."]
    #[inline(always)]
    pub fn vss(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel4::Vss)
    }
}
#[doc = "Set the Precision Mode For Slot 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prmode4 {
    #[doc = "0: 12-bit precision mode"]
    P12b0 = 0,
    #[doc = "1: 12-bit precision mode"]
    P12b1 = 1,
    #[doc = "2: 10-bit precision mode"]
    P10b = 2,
    #[doc = "3: 8-bit precision mode"]
    P8b = 3,
}
impl From<Prmode4> for u8 {
    #[inline(always)]
    fn from(variant: Prmode4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prmode4 {
    type Ux = u8;
}
impl crate::IsEnum for Prmode4 {}
#[doc = "Field `PRMODE4` reader - Set the Precision Mode For Slot 4."]
pub type Prmode4R = crate::FieldReader<Prmode4>;
impl Prmode4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prmode4 {
        match self.bits {
            0 => Prmode4::P12b0,
            1 => Prmode4::P12b1,
            2 => Prmode4::P10b,
            3 => Prmode4::P8b,
            _ => unreachable!(),
        }
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn is_p12b0(&self) -> bool {
        *self == Prmode4::P12b0
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn is_p12b1(&self) -> bool {
        *self == Prmode4::P12b1
    }
    #[doc = "10-bit precision mode"]
    #[inline(always)]
    pub fn is_p10b(&self) -> bool {
        *self == Prmode4::P10b
    }
    #[doc = "8-bit precision mode"]
    #[inline(always)]
    pub fn is_p8b(&self) -> bool {
        *self == Prmode4::P8b
    }
}
#[doc = "Field `PRMODE4` writer - Set the Precision Mode For Slot 4."]
pub type Prmode4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Prmode4, crate::Safe>;
impl<'a, REG> Prmode4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn p12b0(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode4::P12b0)
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn p12b1(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode4::P12b1)
    }
    #[doc = "10-bit precision mode"]
    #[inline(always)]
    pub fn p10b(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode4::P10b)
    }
    #[doc = "8-bit precision mode"]
    #[inline(always)]
    pub fn p8b(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode4::P8b)
    }
}
#[doc = "Field `TRKCYC4` reader - Set additional input signal sampling/tracking time to the specified number of ADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
pub type Trkcyc4R = crate::FieldReader;
#[doc = "Field `TRKCYC4` writer - Set additional input signal sampling/tracking time to the specified number of ADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
pub type Trkcyc4W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Select the number of measurements to average in the accumulate divide module for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adsel4 {
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
impl From<Adsel4> for u8 {
    #[inline(always)]
    fn from(variant: Adsel4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adsel4 {
    type Ux = u8;
}
impl crate::IsEnum for Adsel4 {}
#[doc = "Field `ADSEL4` reader - Select the number of measurements to average in the accumulate divide module for this slot."]
pub type Adsel4R = crate::FieldReader<Adsel4>;
impl Adsel4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adsel4 {
        match self.bits {
            0 => Adsel4::Avg1Msrmt,
            1 => Adsel4::Avg2Msrmts,
            2 => Adsel4::Avg4Msrmts,
            3 => Adsel4::Avg8Msrmt,
            4 => Adsel4::Avg16Msrmts,
            5 => Adsel4::Avg32Msrmts,
            6 => Adsel4::Avg64Msrmts,
            7 => Adsel4::Avg128Msrmts,
            _ => unreachable!(),
        }
    }
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_1_msrmt(&self) -> bool {
        *self == Adsel4::Avg1Msrmt
    }
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_2_msrmts(&self) -> bool {
        *self == Adsel4::Avg2Msrmts
    }
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_4_msrmts(&self) -> bool {
        *self == Adsel4::Avg4Msrmts
    }
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_8_msrmt(&self) -> bool {
        *self == Adsel4::Avg8Msrmt
    }
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_16_msrmts(&self) -> bool {
        *self == Adsel4::Avg16Msrmts
    }
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_32_msrmts(&self) -> bool {
        *self == Adsel4::Avg32Msrmts
    }
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_64_msrmts(&self) -> bool {
        *self == Adsel4::Avg64Msrmts
    }
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_128_msrmts(&self) -> bool {
        *self == Adsel4::Avg128Msrmts
    }
}
#[doc = "Field `ADSEL4` writer - Select the number of measurements to average in the accumulate divide module for this slot."]
pub type Adsel4W<'a, REG> = crate::FieldWriter<'a, REG, 3, Adsel4, crate::Safe>;
impl<'a, REG> Adsel4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_1_msrmt(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel4::Avg1Msrmt)
    }
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_2_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel4::Avg2Msrmts)
    }
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_4_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel4::Avg4Msrmts)
    }
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_8_msrmt(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel4::Avg8Msrmt)
    }
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_16_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel4::Avg16Msrmts)
    }
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_32_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel4::Avg32Msrmts)
    }
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_64_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel4::Avg64Msrmts)
    }
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_128_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel4::Avg128Msrmts)
    }
}
impl R {
    #[doc = "Bit 0 - This bit enables slot 4 for ADC conversions."]
    #[inline(always)]
    pub fn slen4(&self) -> Slen4R {
        Slen4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 4."]
    #[inline(always)]
    pub fn wcen4(&self) -> Wcen4R {
        Wcen4R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Select one of the 11 channel inputs for this slot."]
    #[inline(always)]
    pub fn chsel4(&self) -> Chsel4R {
        Chsel4R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot 4."]
    #[inline(always)]
    pub fn prmode4(&self) -> Prmode4R {
        Prmode4R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:23 - Set additional input signal sampling/tracking time to the specified number of ADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
    #[inline(always)]
    pub fn trkcyc4(&self) -> Trkcyc4R {
        Trkcyc4R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn adsel4(&self) -> Adsel4R {
        Adsel4R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This bit enables slot 4 for ADC conversions."]
    #[inline(always)]
    #[must_use]
    pub fn slen4(&mut self) -> Slen4W<Sl4cfgSpec> {
        Slen4W::new(self, 0)
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 4."]
    #[inline(always)]
    #[must_use]
    pub fn wcen4(&mut self) -> Wcen4W<Sl4cfgSpec> {
        Wcen4W::new(self, 1)
    }
    #[doc = "Bits 8:11 - Select one of the 11 channel inputs for this slot."]
    #[inline(always)]
    #[must_use]
    pub fn chsel4(&mut self) -> Chsel4W<Sl4cfgSpec> {
        Chsel4W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot 4."]
    #[inline(always)]
    #[must_use]
    pub fn prmode4(&mut self) -> Prmode4W<Sl4cfgSpec> {
        Prmode4W::new(self, 16)
    }
    #[doc = "Bits 18:23 - Set additional input signal sampling/tracking time to the specified number of ADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
    #[inline(always)]
    #[must_use]
    pub fn trkcyc4(&mut self) -> Trkcyc4W<Sl4cfgSpec> {
        Trkcyc4W::new(self, 18)
    }
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    #[must_use]
    pub fn adsel4(&mut self) -> Adsel4W<Sl4cfgSpec> {
        Adsel4W::new(self, 24)
    }
}
#[doc = "Slot 4 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sl4cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl4cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sl4cfgSpec;
impl crate::RegisterSpec for Sl4cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sl4cfg::R`](R) reader structure"]
impl crate::Readable for Sl4cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sl4cfg::W`](W) writer structure"]
impl crate::Writable for Sl4cfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SL4CFG to value 0"]
impl crate::Resettable for Sl4cfgSpec {
    const RESET_VALUE: u32 = 0;
}
