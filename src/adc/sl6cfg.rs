#[doc = "Register `SL6CFG` reader"]
pub type R = crate::R<Sl6cfgSpec>;
#[doc = "Register `SL6CFG` writer"]
pub type W = crate::W<Sl6cfgSpec>;
#[doc = "This bit enables slot 6 for ADC conversions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slen6 {
    #[doc = "1: Enable slot 6 for ADC conversions."]
    Slen = 1,
    #[doc = "0: Disable slot 6 for ADC conversions."]
    Sldis = 0,
}
impl From<Slen6> for bool {
    #[inline(always)]
    fn from(variant: Slen6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEN6` reader - This bit enables slot 6 for ADC conversions."]
pub type Slen6R = crate::BitReader<Slen6>;
impl Slen6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slen6 {
        match self.bits {
            true => Slen6::Slen,
            false => Slen6::Sldis,
        }
    }
    #[doc = "Enable slot 6 for ADC conversions."]
    #[inline(always)]
    pub fn is_slen(&self) -> bool {
        *self == Slen6::Slen
    }
    #[doc = "Disable slot 6 for ADC conversions."]
    #[inline(always)]
    pub fn is_sldis(&self) -> bool {
        *self == Slen6::Sldis
    }
}
#[doc = "Field `SLEN6` writer - This bit enables slot 6 for ADC conversions."]
pub type Slen6W<'a, REG> = crate::BitWriter<'a, REG, Slen6>;
impl<'a, REG> Slen6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable slot 6 for ADC conversions."]
    #[inline(always)]
    pub fn slen(self) -> &'a mut crate::W<REG> {
        self.variant(Slen6::Slen)
    }
    #[doc = "Disable slot 6 for ADC conversions."]
    #[inline(always)]
    pub fn sldis(self) -> &'a mut crate::W<REG> {
        self.variant(Slen6::Sldis)
    }
}
#[doc = "This bit enables the window compare function for slot 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wcen6 {
    #[doc = "1: Enable the window compare for slot 6."]
    Wcen = 1,
    #[doc = "0: Disable the window compare for slot 6."]
    Wcdis = 0,
}
impl From<Wcen6> for bool {
    #[inline(always)]
    fn from(variant: Wcen6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WCEN6` reader - This bit enables the window compare function for slot 6."]
pub type Wcen6R = crate::BitReader<Wcen6>;
impl Wcen6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wcen6 {
        match self.bits {
            true => Wcen6::Wcen,
            false => Wcen6::Wcdis,
        }
    }
    #[doc = "Enable the window compare for slot 6."]
    #[inline(always)]
    pub fn is_wcen(&self) -> bool {
        *self == Wcen6::Wcen
    }
    #[doc = "Disable the window compare for slot 6."]
    #[inline(always)]
    pub fn is_wcdis(&self) -> bool {
        *self == Wcen6::Wcdis
    }
}
#[doc = "Field `WCEN6` writer - This bit enables the window compare function for slot 6."]
pub type Wcen6W<'a, REG> = crate::BitWriter<'a, REG, Wcen6>;
impl<'a, REG> Wcen6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the window compare for slot 6."]
    #[inline(always)]
    pub fn wcen(self) -> &'a mut crate::W<REG> {
        self.variant(Wcen6::Wcen)
    }
    #[doc = "Disable the window compare for slot 6."]
    #[inline(always)]
    pub fn wcdis(self) -> &'a mut crate::W<REG> {
        self.variant(Wcen6::Wcdis)
    }
}
#[doc = "Select one of the 11 channel inputs for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chsel6 {
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
impl From<Chsel6> for u8 {
    #[inline(always)]
    fn from(variant: Chsel6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chsel6 {
    type Ux = u8;
}
impl crate::IsEnum for Chsel6 {}
#[doc = "Field `CHSEL6` reader - Select one of the 11 channel inputs for this slot."]
pub type Chsel6R = crate::FieldReader<Chsel6>;
impl Chsel6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Chsel6> {
        match self.bits {
            0 => Some(Chsel6::Se0),
            1 => Some(Chsel6::Se1),
            2 => Some(Chsel6::Se2),
            3 => Some(Chsel6::Se3),
            4 => Some(Chsel6::Se4),
            5 => Some(Chsel6::Se5),
            6 => Some(Chsel6::Se6),
            7 => Some(Chsel6::Se7),
            8 => Some(Chsel6::Temp),
            9 => Some(Chsel6::Batt),
            11 => Some(Chsel6::Vss),
            _ => None,
        }
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se0(&self) -> bool {
        *self == Chsel6::Se0
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se1(&self) -> bool {
        *self == Chsel6::Se1
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se2(&self) -> bool {
        *self == Chsel6::Se2
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se3(&self) -> bool {
        *self == Chsel6::Se3
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se4(&self) -> bool {
        *self == Chsel6::Se4
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se5(&self) -> bool {
        *self == Chsel6::Se5
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se6(&self) -> bool {
        *self == Chsel6::Se6
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se7(&self) -> bool {
        *self == Chsel6::Se7
    }
    #[doc = "Internal temperature sensor."]
    #[inline(always)]
    pub fn is_temp(&self) -> bool {
        *self == Chsel6::Temp
    }
    #[doc = "Internal voltage divide-by-3 connection."]
    #[inline(always)]
    pub fn is_batt(&self) -> bool {
        *self == Chsel6::Batt
    }
    #[doc = "Input VSS."]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        *self == Chsel6::Vss
    }
}
#[doc = "Field `CHSEL6` writer - Select one of the 11 channel inputs for this slot."]
pub type Chsel6W<'a, REG> = crate::FieldWriter<'a, REG, 4, Chsel6>;
impl<'a, REG> Chsel6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel6::Se0)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel6::Se1)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se2(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel6::Se2)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se3(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel6::Se3)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se4(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel6::Se4)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se5(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel6::Se5)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se6(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel6::Se6)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se7(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel6::Se7)
    }
    #[doc = "Internal temperature sensor."]
    #[inline(always)]
    pub fn temp(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel6::Temp)
    }
    #[doc = "Internal voltage divide-by-3 connection."]
    #[inline(always)]
    pub fn batt(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel6::Batt)
    }
    #[doc = "Input VSS."]
    #[inline(always)]
    pub fn vss(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel6::Vss)
    }
}
#[doc = "Set the Precision Mode For Slot 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prmode6 {
    #[doc = "0: 12-bit precision mode"]
    P12b0 = 0,
    #[doc = "1: 12-bit precision mode"]
    P12b1 = 1,
    #[doc = "2: 10-bit precision mode"]
    P10b = 2,
    #[doc = "3: 8-bit precision mode"]
    P8b = 3,
}
impl From<Prmode6> for u8 {
    #[inline(always)]
    fn from(variant: Prmode6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prmode6 {
    type Ux = u8;
}
impl crate::IsEnum for Prmode6 {}
#[doc = "Field `PRMODE6` reader - Set the Precision Mode For Slot 6."]
pub type Prmode6R = crate::FieldReader<Prmode6>;
impl Prmode6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prmode6 {
        match self.bits {
            0 => Prmode6::P12b0,
            1 => Prmode6::P12b1,
            2 => Prmode6::P10b,
            3 => Prmode6::P8b,
            _ => unreachable!(),
        }
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn is_p12b0(&self) -> bool {
        *self == Prmode6::P12b0
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn is_p12b1(&self) -> bool {
        *self == Prmode6::P12b1
    }
    #[doc = "10-bit precision mode"]
    #[inline(always)]
    pub fn is_p10b(&self) -> bool {
        *self == Prmode6::P10b
    }
    #[doc = "8-bit precision mode"]
    #[inline(always)]
    pub fn is_p8b(&self) -> bool {
        *self == Prmode6::P8b
    }
}
#[doc = "Field `PRMODE6` writer - Set the Precision Mode For Slot 6."]
pub type Prmode6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Prmode6, crate::Safe>;
impl<'a, REG> Prmode6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn p12b0(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode6::P12b0)
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn p12b1(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode6::P12b1)
    }
    #[doc = "10-bit precision mode"]
    #[inline(always)]
    pub fn p10b(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode6::P10b)
    }
    #[doc = "8-bit precision mode"]
    #[inline(always)]
    pub fn p8b(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode6::P8b)
    }
}
#[doc = "Field `TRKCYC6` reader - Set additional input signal sampling/tracking time to the specified number of ADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
pub type Trkcyc6R = crate::FieldReader;
#[doc = "Field `TRKCYC6` writer - Set additional input signal sampling/tracking time to the specified number of ADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
pub type Trkcyc6W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Select the number of measurements to average in the accumulate divide module for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adsel6 {
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
impl From<Adsel6> for u8 {
    #[inline(always)]
    fn from(variant: Adsel6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adsel6 {
    type Ux = u8;
}
impl crate::IsEnum for Adsel6 {}
#[doc = "Field `ADSEL6` reader - Select the number of measurements to average in the accumulate divide module for this slot."]
pub type Adsel6R = crate::FieldReader<Adsel6>;
impl Adsel6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adsel6 {
        match self.bits {
            0 => Adsel6::Avg1Msrmt,
            1 => Adsel6::Avg2Msrmts,
            2 => Adsel6::Avg4Msrmts,
            3 => Adsel6::Avg8Msrmt,
            4 => Adsel6::Avg16Msrmts,
            5 => Adsel6::Avg32Msrmts,
            6 => Adsel6::Avg64Msrmts,
            7 => Adsel6::Avg128Msrmts,
            _ => unreachable!(),
        }
    }
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_1_msrmt(&self) -> bool {
        *self == Adsel6::Avg1Msrmt
    }
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_2_msrmts(&self) -> bool {
        *self == Adsel6::Avg2Msrmts
    }
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_4_msrmts(&self) -> bool {
        *self == Adsel6::Avg4Msrmts
    }
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_8_msrmt(&self) -> bool {
        *self == Adsel6::Avg8Msrmt
    }
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_16_msrmts(&self) -> bool {
        *self == Adsel6::Avg16Msrmts
    }
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_32_msrmts(&self) -> bool {
        *self == Adsel6::Avg32Msrmts
    }
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_64_msrmts(&self) -> bool {
        *self == Adsel6::Avg64Msrmts
    }
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_128_msrmts(&self) -> bool {
        *self == Adsel6::Avg128Msrmts
    }
}
#[doc = "Field `ADSEL6` writer - Select the number of measurements to average in the accumulate divide module for this slot."]
pub type Adsel6W<'a, REG> = crate::FieldWriter<'a, REG, 3, Adsel6, crate::Safe>;
impl<'a, REG> Adsel6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_1_msrmt(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel6::Avg1Msrmt)
    }
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_2_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel6::Avg2Msrmts)
    }
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_4_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel6::Avg4Msrmts)
    }
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_8_msrmt(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel6::Avg8Msrmt)
    }
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_16_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel6::Avg16Msrmts)
    }
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_32_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel6::Avg32Msrmts)
    }
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_64_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel6::Avg64Msrmts)
    }
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_128_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel6::Avg128Msrmts)
    }
}
impl R {
    #[doc = "Bit 0 - This bit enables slot 6 for ADC conversions."]
    #[inline(always)]
    pub fn slen6(&self) -> Slen6R {
        Slen6R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 6."]
    #[inline(always)]
    pub fn wcen6(&self) -> Wcen6R {
        Wcen6R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Select one of the 11 channel inputs for this slot."]
    #[inline(always)]
    pub fn chsel6(&self) -> Chsel6R {
        Chsel6R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot 6."]
    #[inline(always)]
    pub fn prmode6(&self) -> Prmode6R {
        Prmode6R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:23 - Set additional input signal sampling/tracking time to the specified number of ADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
    #[inline(always)]
    pub fn trkcyc6(&self) -> Trkcyc6R {
        Trkcyc6R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn adsel6(&self) -> Adsel6R {
        Adsel6R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This bit enables slot 6 for ADC conversions."]
    #[inline(always)]
    #[must_use]
    pub fn slen6(&mut self) -> Slen6W<Sl6cfgSpec> {
        Slen6W::new(self, 0)
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 6."]
    #[inline(always)]
    #[must_use]
    pub fn wcen6(&mut self) -> Wcen6W<Sl6cfgSpec> {
        Wcen6W::new(self, 1)
    }
    #[doc = "Bits 8:11 - Select one of the 11 channel inputs for this slot."]
    #[inline(always)]
    #[must_use]
    pub fn chsel6(&mut self) -> Chsel6W<Sl6cfgSpec> {
        Chsel6W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot 6."]
    #[inline(always)]
    #[must_use]
    pub fn prmode6(&mut self) -> Prmode6W<Sl6cfgSpec> {
        Prmode6W::new(self, 16)
    }
    #[doc = "Bits 18:23 - Set additional input signal sampling/tracking time to the specified number of ADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
    #[inline(always)]
    #[must_use]
    pub fn trkcyc6(&mut self) -> Trkcyc6W<Sl6cfgSpec> {
        Trkcyc6W::new(self, 18)
    }
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    #[must_use]
    pub fn adsel6(&mut self) -> Adsel6W<Sl6cfgSpec> {
        Adsel6W::new(self, 24)
    }
}
#[doc = "Slot 6 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sl6cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl6cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sl6cfgSpec;
impl crate::RegisterSpec for Sl6cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sl6cfg::R`](R) reader structure"]
impl crate::Readable for Sl6cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sl6cfg::W`](W) writer structure"]
impl crate::Writable for Sl6cfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SL6CFG to value 0"]
impl crate::Resettable for Sl6cfgSpec {
    const RESET_VALUE: u32 = 0;
}
