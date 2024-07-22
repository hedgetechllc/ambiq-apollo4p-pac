#[doc = "Register `SL3CFG` reader"]
pub type R = crate::R<Sl3cfgSpec>;
#[doc = "Register `SL3CFG` writer"]
pub type W = crate::W<Sl3cfgSpec>;
#[doc = "This bit enables slot 3 for ADC conversions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slen3 {
    #[doc = "1: Enable slot 3 for ADC conversions."]
    Slen = 1,
    #[doc = "0: Disable slot 3 for ADC conversions."]
    Sldis = 0,
}
impl From<Slen3> for bool {
    #[inline(always)]
    fn from(variant: Slen3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEN3` reader - This bit enables slot 3 for ADC conversions."]
pub type Slen3R = crate::BitReader<Slen3>;
impl Slen3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slen3 {
        match self.bits {
            true => Slen3::Slen,
            false => Slen3::Sldis,
        }
    }
    #[doc = "Enable slot 3 for ADC conversions."]
    #[inline(always)]
    pub fn is_slen(&self) -> bool {
        *self == Slen3::Slen
    }
    #[doc = "Disable slot 3 for ADC conversions."]
    #[inline(always)]
    pub fn is_sldis(&self) -> bool {
        *self == Slen3::Sldis
    }
}
#[doc = "Field `SLEN3` writer - This bit enables slot 3 for ADC conversions."]
pub type Slen3W<'a, REG> = crate::BitWriter<'a, REG, Slen3>;
impl<'a, REG> Slen3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable slot 3 for ADC conversions."]
    #[inline(always)]
    pub fn slen(self) -> &'a mut crate::W<REG> {
        self.variant(Slen3::Slen)
    }
    #[doc = "Disable slot 3 for ADC conversions."]
    #[inline(always)]
    pub fn sldis(self) -> &'a mut crate::W<REG> {
        self.variant(Slen3::Sldis)
    }
}
#[doc = "This bit enables the window compare function for slot 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wcen3 {
    #[doc = "1: Enable the window compare for slot 3."]
    Wcen = 1,
    #[doc = "0: Disable the window compare for slot 3."]
    Wcdis = 0,
}
impl From<Wcen3> for bool {
    #[inline(always)]
    fn from(variant: Wcen3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WCEN3` reader - This bit enables the window compare function for slot 3."]
pub type Wcen3R = crate::BitReader<Wcen3>;
impl Wcen3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wcen3 {
        match self.bits {
            true => Wcen3::Wcen,
            false => Wcen3::Wcdis,
        }
    }
    #[doc = "Enable the window compare for slot 3."]
    #[inline(always)]
    pub fn is_wcen(&self) -> bool {
        *self == Wcen3::Wcen
    }
    #[doc = "Disable the window compare for slot 3."]
    #[inline(always)]
    pub fn is_wcdis(&self) -> bool {
        *self == Wcen3::Wcdis
    }
}
#[doc = "Field `WCEN3` writer - This bit enables the window compare function for slot 3."]
pub type Wcen3W<'a, REG> = crate::BitWriter<'a, REG, Wcen3>;
impl<'a, REG> Wcen3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the window compare for slot 3."]
    #[inline(always)]
    pub fn wcen(self) -> &'a mut crate::W<REG> {
        self.variant(Wcen3::Wcen)
    }
    #[doc = "Disable the window compare for slot 3."]
    #[inline(always)]
    pub fn wcdis(self) -> &'a mut crate::W<REG> {
        self.variant(Wcen3::Wcdis)
    }
}
#[doc = "Select one of the 11 channel inputs for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chsel3 {
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
impl From<Chsel3> for u8 {
    #[inline(always)]
    fn from(variant: Chsel3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chsel3 {
    type Ux = u8;
}
impl crate::IsEnum for Chsel3 {}
#[doc = "Field `CHSEL3` reader - Select one of the 11 channel inputs for this slot."]
pub type Chsel3R = crate::FieldReader<Chsel3>;
impl Chsel3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Chsel3> {
        match self.bits {
            0 => Some(Chsel3::Se0),
            1 => Some(Chsel3::Se1),
            2 => Some(Chsel3::Se2),
            3 => Some(Chsel3::Se3),
            4 => Some(Chsel3::Se4),
            5 => Some(Chsel3::Se5),
            6 => Some(Chsel3::Se6),
            7 => Some(Chsel3::Se7),
            8 => Some(Chsel3::Temp),
            9 => Some(Chsel3::Batt),
            11 => Some(Chsel3::Vss),
            _ => None,
        }
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se0(&self) -> bool {
        *self == Chsel3::Se0
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se1(&self) -> bool {
        *self == Chsel3::Se1
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se2(&self) -> bool {
        *self == Chsel3::Se2
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se3(&self) -> bool {
        *self == Chsel3::Se3
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se4(&self) -> bool {
        *self == Chsel3::Se4
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se5(&self) -> bool {
        *self == Chsel3::Se5
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se6(&self) -> bool {
        *self == Chsel3::Se6
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se7(&self) -> bool {
        *self == Chsel3::Se7
    }
    #[doc = "Internal temperature sensor."]
    #[inline(always)]
    pub fn is_temp(&self) -> bool {
        *self == Chsel3::Temp
    }
    #[doc = "Internal voltage divide-by-3 connection."]
    #[inline(always)]
    pub fn is_batt(&self) -> bool {
        *self == Chsel3::Batt
    }
    #[doc = "Input VSS."]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        *self == Chsel3::Vss
    }
}
#[doc = "Field `CHSEL3` writer - Select one of the 11 channel inputs for this slot."]
pub type Chsel3W<'a, REG> = crate::FieldWriter<'a, REG, 4, Chsel3>;
impl<'a, REG> Chsel3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel3::Se0)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel3::Se1)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se2(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel3::Se2)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se3(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel3::Se3)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se4(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel3::Se4)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se5(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel3::Se5)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se6(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel3::Se6)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se7(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel3::Se7)
    }
    #[doc = "Internal temperature sensor."]
    #[inline(always)]
    pub fn temp(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel3::Temp)
    }
    #[doc = "Internal voltage divide-by-3 connection."]
    #[inline(always)]
    pub fn batt(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel3::Batt)
    }
    #[doc = "Input VSS."]
    #[inline(always)]
    pub fn vss(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel3::Vss)
    }
}
#[doc = "Set the Precision Mode For Slot 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prmode3 {
    #[doc = "0: 12-bit precision mode"]
    P12b0 = 0,
    #[doc = "1: 12-bit precision mode"]
    P12b1 = 1,
    #[doc = "2: 10-bit precision mode"]
    P10b = 2,
    #[doc = "3: 8-bit precision mode"]
    P8b = 3,
}
impl From<Prmode3> for u8 {
    #[inline(always)]
    fn from(variant: Prmode3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prmode3 {
    type Ux = u8;
}
impl crate::IsEnum for Prmode3 {}
#[doc = "Field `PRMODE3` reader - Set the Precision Mode For Slot 3."]
pub type Prmode3R = crate::FieldReader<Prmode3>;
impl Prmode3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prmode3 {
        match self.bits {
            0 => Prmode3::P12b0,
            1 => Prmode3::P12b1,
            2 => Prmode3::P10b,
            3 => Prmode3::P8b,
            _ => unreachable!(),
        }
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn is_p12b0(&self) -> bool {
        *self == Prmode3::P12b0
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn is_p12b1(&self) -> bool {
        *self == Prmode3::P12b1
    }
    #[doc = "10-bit precision mode"]
    #[inline(always)]
    pub fn is_p10b(&self) -> bool {
        *self == Prmode3::P10b
    }
    #[doc = "8-bit precision mode"]
    #[inline(always)]
    pub fn is_p8b(&self) -> bool {
        *self == Prmode3::P8b
    }
}
#[doc = "Field `PRMODE3` writer - Set the Precision Mode For Slot 3."]
pub type Prmode3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Prmode3, crate::Safe>;
impl<'a, REG> Prmode3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn p12b0(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode3::P12b0)
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn p12b1(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode3::P12b1)
    }
    #[doc = "10-bit precision mode"]
    #[inline(always)]
    pub fn p10b(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode3::P10b)
    }
    #[doc = "8-bit precision mode"]
    #[inline(always)]
    pub fn p8b(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode3::P8b)
    }
}
#[doc = "Field `TRKCYC3` reader - Set additional input signal sampling/tracking time to the specified number of ADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
pub type Trkcyc3R = crate::FieldReader;
#[doc = "Field `TRKCYC3` writer - Set additional input signal sampling/tracking time to the specified number of ADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
pub type Trkcyc3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Select the number of measurements to average in the accumulate divide module for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adsel3 {
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
impl From<Adsel3> for u8 {
    #[inline(always)]
    fn from(variant: Adsel3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adsel3 {
    type Ux = u8;
}
impl crate::IsEnum for Adsel3 {}
#[doc = "Field `ADSEL3` reader - Select the number of measurements to average in the accumulate divide module for this slot."]
pub type Adsel3R = crate::FieldReader<Adsel3>;
impl Adsel3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adsel3 {
        match self.bits {
            0 => Adsel3::Avg1Msrmt,
            1 => Adsel3::Avg2Msrmts,
            2 => Adsel3::Avg4Msrmts,
            3 => Adsel3::Avg8Msrmt,
            4 => Adsel3::Avg16Msrmts,
            5 => Adsel3::Avg32Msrmts,
            6 => Adsel3::Avg64Msrmts,
            7 => Adsel3::Avg128Msrmts,
            _ => unreachable!(),
        }
    }
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_1_msrmt(&self) -> bool {
        *self == Adsel3::Avg1Msrmt
    }
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_2_msrmts(&self) -> bool {
        *self == Adsel3::Avg2Msrmts
    }
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_4_msrmts(&self) -> bool {
        *self == Adsel3::Avg4Msrmts
    }
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_8_msrmt(&self) -> bool {
        *self == Adsel3::Avg8Msrmt
    }
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_16_msrmts(&self) -> bool {
        *self == Adsel3::Avg16Msrmts
    }
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_32_msrmts(&self) -> bool {
        *self == Adsel3::Avg32Msrmts
    }
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_64_msrmts(&self) -> bool {
        *self == Adsel3::Avg64Msrmts
    }
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_128_msrmts(&self) -> bool {
        *self == Adsel3::Avg128Msrmts
    }
}
#[doc = "Field `ADSEL3` writer - Select the number of measurements to average in the accumulate divide module for this slot."]
pub type Adsel3W<'a, REG> = crate::FieldWriter<'a, REG, 3, Adsel3, crate::Safe>;
impl<'a, REG> Adsel3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_1_msrmt(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel3::Avg1Msrmt)
    }
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_2_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel3::Avg2Msrmts)
    }
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_4_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel3::Avg4Msrmts)
    }
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_8_msrmt(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel3::Avg8Msrmt)
    }
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_16_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel3::Avg16Msrmts)
    }
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_32_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel3::Avg32Msrmts)
    }
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_64_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel3::Avg64Msrmts)
    }
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_128_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel3::Avg128Msrmts)
    }
}
impl R {
    #[doc = "Bit 0 - This bit enables slot 3 for ADC conversions."]
    #[inline(always)]
    pub fn slen3(&self) -> Slen3R {
        Slen3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 3."]
    #[inline(always)]
    pub fn wcen3(&self) -> Wcen3R {
        Wcen3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Select one of the 11 channel inputs for this slot."]
    #[inline(always)]
    pub fn chsel3(&self) -> Chsel3R {
        Chsel3R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot 3."]
    #[inline(always)]
    pub fn prmode3(&self) -> Prmode3R {
        Prmode3R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:23 - Set additional input signal sampling/tracking time to the specified number of ADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
    #[inline(always)]
    pub fn trkcyc3(&self) -> Trkcyc3R {
        Trkcyc3R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn adsel3(&self) -> Adsel3R {
        Adsel3R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This bit enables slot 3 for ADC conversions."]
    #[inline(always)]
    #[must_use]
    pub fn slen3(&mut self) -> Slen3W<Sl3cfgSpec> {
        Slen3W::new(self, 0)
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 3."]
    #[inline(always)]
    #[must_use]
    pub fn wcen3(&mut self) -> Wcen3W<Sl3cfgSpec> {
        Wcen3W::new(self, 1)
    }
    #[doc = "Bits 8:11 - Select one of the 11 channel inputs for this slot."]
    #[inline(always)]
    #[must_use]
    pub fn chsel3(&mut self) -> Chsel3W<Sl3cfgSpec> {
        Chsel3W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot 3."]
    #[inline(always)]
    #[must_use]
    pub fn prmode3(&mut self) -> Prmode3W<Sl3cfgSpec> {
        Prmode3W::new(self, 16)
    }
    #[doc = "Bits 18:23 - Set additional input signal sampling/tracking time to the specified number of ADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
    #[inline(always)]
    #[must_use]
    pub fn trkcyc3(&mut self) -> Trkcyc3W<Sl3cfgSpec> {
        Trkcyc3W::new(self, 18)
    }
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    #[must_use]
    pub fn adsel3(&mut self) -> Adsel3W<Sl3cfgSpec> {
        Adsel3W::new(self, 24)
    }
}
#[doc = "Slot 3 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sl3cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl3cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sl3cfgSpec;
impl crate::RegisterSpec for Sl3cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sl3cfg::R`](R) reader structure"]
impl crate::Readable for Sl3cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sl3cfg::W`](W) writer structure"]
impl crate::Writable for Sl3cfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SL3CFG to value 0"]
impl crate::Resettable for Sl3cfgSpec {
    const RESET_VALUE: u32 = 0;
}
