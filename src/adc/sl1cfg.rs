#[doc = "Register `SL1CFG` reader"]
pub type R = crate::R<Sl1cfgSpec>;
#[doc = "Register `SL1CFG` writer"]
pub type W = crate::W<Sl1cfgSpec>;
#[doc = "This bit enables slot 1 for ADC conversions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slen1 {
    #[doc = "1: Enable slot 1 for ADC conversions."]
    Slen = 1,
    #[doc = "0: Disable slot 1 for ADC conversions."]
    Sldis = 0,
}
impl From<Slen1> for bool {
    #[inline(always)]
    fn from(variant: Slen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEN1` reader - This bit enables slot 1 for ADC conversions."]
pub type Slen1R = crate::BitReader<Slen1>;
impl Slen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slen1 {
        match self.bits {
            true => Slen1::Slen,
            false => Slen1::Sldis,
        }
    }
    #[doc = "Enable slot 1 for ADC conversions."]
    #[inline(always)]
    pub fn is_slen(&self) -> bool {
        *self == Slen1::Slen
    }
    #[doc = "Disable slot 1 for ADC conversions."]
    #[inline(always)]
    pub fn is_sldis(&self) -> bool {
        *self == Slen1::Sldis
    }
}
#[doc = "Field `SLEN1` writer - This bit enables slot 1 for ADC conversions."]
pub type Slen1W<'a, REG> = crate::BitWriter<'a, REG, Slen1>;
impl<'a, REG> Slen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable slot 1 for ADC conversions."]
    #[inline(always)]
    pub fn slen(self) -> &'a mut crate::W<REG> {
        self.variant(Slen1::Slen)
    }
    #[doc = "Disable slot 1 for ADC conversions."]
    #[inline(always)]
    pub fn sldis(self) -> &'a mut crate::W<REG> {
        self.variant(Slen1::Sldis)
    }
}
#[doc = "This bit enables the window compare function for slot 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wcen1 {
    #[doc = "1: Enable the window compare for slot 1."]
    Wcen = 1,
    #[doc = "0: Disable the window compare for slot 1."]
    Wcdis = 0,
}
impl From<Wcen1> for bool {
    #[inline(always)]
    fn from(variant: Wcen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WCEN1` reader - This bit enables the window compare function for slot 1."]
pub type Wcen1R = crate::BitReader<Wcen1>;
impl Wcen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wcen1 {
        match self.bits {
            true => Wcen1::Wcen,
            false => Wcen1::Wcdis,
        }
    }
    #[doc = "Enable the window compare for slot 1."]
    #[inline(always)]
    pub fn is_wcen(&self) -> bool {
        *self == Wcen1::Wcen
    }
    #[doc = "Disable the window compare for slot 1."]
    #[inline(always)]
    pub fn is_wcdis(&self) -> bool {
        *self == Wcen1::Wcdis
    }
}
#[doc = "Field `WCEN1` writer - This bit enables the window compare function for slot 1."]
pub type Wcen1W<'a, REG> = crate::BitWriter<'a, REG, Wcen1>;
impl<'a, REG> Wcen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the window compare for slot 1."]
    #[inline(always)]
    pub fn wcen(self) -> &'a mut crate::W<REG> {
        self.variant(Wcen1::Wcen)
    }
    #[doc = "Disable the window compare for slot 1."]
    #[inline(always)]
    pub fn wcdis(self) -> &'a mut crate::W<REG> {
        self.variant(Wcen1::Wcdis)
    }
}
#[doc = "Select one of the 11 channel inputs for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chsel1 {
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
impl From<Chsel1> for u8 {
    #[inline(always)]
    fn from(variant: Chsel1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chsel1 {
    type Ux = u8;
}
impl crate::IsEnum for Chsel1 {}
#[doc = "Field `CHSEL1` reader - Select one of the 11 channel inputs for this slot."]
pub type Chsel1R = crate::FieldReader<Chsel1>;
impl Chsel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Chsel1> {
        match self.bits {
            0 => Some(Chsel1::Se0),
            1 => Some(Chsel1::Se1),
            2 => Some(Chsel1::Se2),
            3 => Some(Chsel1::Se3),
            4 => Some(Chsel1::Se4),
            5 => Some(Chsel1::Se5),
            6 => Some(Chsel1::Se6),
            7 => Some(Chsel1::Se7),
            8 => Some(Chsel1::Temp),
            9 => Some(Chsel1::Batt),
            11 => Some(Chsel1::Vss),
            _ => None,
        }
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se0(&self) -> bool {
        *self == Chsel1::Se0
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se1(&self) -> bool {
        *self == Chsel1::Se1
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se2(&self) -> bool {
        *self == Chsel1::Se2
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se3(&self) -> bool {
        *self == Chsel1::Se3
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se4(&self) -> bool {
        *self == Chsel1::Se4
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se5(&self) -> bool {
        *self == Chsel1::Se5
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se6(&self) -> bool {
        *self == Chsel1::Se6
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn is_se7(&self) -> bool {
        *self == Chsel1::Se7
    }
    #[doc = "Internal temperature sensor."]
    #[inline(always)]
    pub fn is_temp(&self) -> bool {
        *self == Chsel1::Temp
    }
    #[doc = "Internal voltage divide-by-3 connection."]
    #[inline(always)]
    pub fn is_batt(&self) -> bool {
        *self == Chsel1::Batt
    }
    #[doc = "Input VSS."]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        *self == Chsel1::Vss
    }
}
#[doc = "Field `CHSEL1` writer - Select one of the 11 channel inputs for this slot."]
pub type Chsel1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Chsel1>;
impl<'a, REG> Chsel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se0(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel1::Se0)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel1::Se1)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se2(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel1::Se2)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se3(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel1::Se3)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se4(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel1::Se4)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se5(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel1::Se5)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se6(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel1::Se6)
    }
    #[doc = "Single ended external GPIO connection."]
    #[inline(always)]
    pub fn se7(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel1::Se7)
    }
    #[doc = "Internal temperature sensor."]
    #[inline(always)]
    pub fn temp(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel1::Temp)
    }
    #[doc = "Internal voltage divide-by-3 connection."]
    #[inline(always)]
    pub fn batt(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel1::Batt)
    }
    #[doc = "Input VSS."]
    #[inline(always)]
    pub fn vss(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel1::Vss)
    }
}
#[doc = "Set the Precision Mode For Slot 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prmode1 {
    #[doc = "0: 12-bit precision mode"]
    P12b0 = 0,
    #[doc = "1: 12-bit precision mode"]
    P12b1 = 1,
    #[doc = "2: 10-bit precision mode"]
    P10b = 2,
    #[doc = "3: 8-bit precision mode"]
    P8b = 3,
}
impl From<Prmode1> for u8 {
    #[inline(always)]
    fn from(variant: Prmode1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prmode1 {
    type Ux = u8;
}
impl crate::IsEnum for Prmode1 {}
#[doc = "Field `PRMODE1` reader - Set the Precision Mode For Slot 1."]
pub type Prmode1R = crate::FieldReader<Prmode1>;
impl Prmode1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prmode1 {
        match self.bits {
            0 => Prmode1::P12b0,
            1 => Prmode1::P12b1,
            2 => Prmode1::P10b,
            3 => Prmode1::P8b,
            _ => unreachable!(),
        }
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn is_p12b0(&self) -> bool {
        *self == Prmode1::P12b0
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn is_p12b1(&self) -> bool {
        *self == Prmode1::P12b1
    }
    #[doc = "10-bit precision mode"]
    #[inline(always)]
    pub fn is_p10b(&self) -> bool {
        *self == Prmode1::P10b
    }
    #[doc = "8-bit precision mode"]
    #[inline(always)]
    pub fn is_p8b(&self) -> bool {
        *self == Prmode1::P8b
    }
}
#[doc = "Field `PRMODE1` writer - Set the Precision Mode For Slot 1."]
pub type Prmode1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Prmode1, crate::Safe>;
impl<'a, REG> Prmode1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn p12b0(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode1::P12b0)
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn p12b1(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode1::P12b1)
    }
    #[doc = "10-bit precision mode"]
    #[inline(always)]
    pub fn p10b(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode1::P10b)
    }
    #[doc = "8-bit precision mode"]
    #[inline(always)]
    pub fn p8b(self) -> &'a mut crate::W<REG> {
        self.variant(Prmode1::P8b)
    }
}
#[doc = "Field `TRKCYC1` reader - Set additional input signal sampling/tracking time to the specified number of ADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
pub type Trkcyc1R = crate::FieldReader;
#[doc = "Field `TRKCYC1` writer - Set additional input signal sampling/tracking time to the specified number of ADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
pub type Trkcyc1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Select the number of measurements to average in the accumulate divide module for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adsel1 {
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
impl From<Adsel1> for u8 {
    #[inline(always)]
    fn from(variant: Adsel1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adsel1 {
    type Ux = u8;
}
impl crate::IsEnum for Adsel1 {}
#[doc = "Field `ADSEL1` reader - Select the number of measurements to average in the accumulate divide module for this slot."]
pub type Adsel1R = crate::FieldReader<Adsel1>;
impl Adsel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adsel1 {
        match self.bits {
            0 => Adsel1::Avg1Msrmt,
            1 => Adsel1::Avg2Msrmts,
            2 => Adsel1::Avg4Msrmts,
            3 => Adsel1::Avg8Msrmt,
            4 => Adsel1::Avg16Msrmts,
            5 => Adsel1::Avg32Msrmts,
            6 => Adsel1::Avg64Msrmts,
            7 => Adsel1::Avg128Msrmts,
            _ => unreachable!(),
        }
    }
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_1_msrmt(&self) -> bool {
        *self == Adsel1::Avg1Msrmt
    }
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_2_msrmts(&self) -> bool {
        *self == Adsel1::Avg2Msrmts
    }
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_4_msrmts(&self) -> bool {
        *self == Adsel1::Avg4Msrmts
    }
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_8_msrmt(&self) -> bool {
        *self == Adsel1::Avg8Msrmt
    }
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_16_msrmts(&self) -> bool {
        *self == Adsel1::Avg16Msrmts
    }
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_32_msrmts(&self) -> bool {
        *self == Adsel1::Avg32Msrmts
    }
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_64_msrmts(&self) -> bool {
        *self == Adsel1::Avg64Msrmts
    }
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn is_avg_128_msrmts(&self) -> bool {
        *self == Adsel1::Avg128Msrmts
    }
}
#[doc = "Field `ADSEL1` writer - Select the number of measurements to average in the accumulate divide module for this slot."]
pub type Adsel1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Adsel1, crate::Safe>;
impl<'a, REG> Adsel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_1_msrmt(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel1::Avg1Msrmt)
    }
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_2_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel1::Avg2Msrmts)
    }
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_4_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel1::Avg4Msrmts)
    }
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_8_msrmt(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel1::Avg8Msrmt)
    }
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_16_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel1::Avg16Msrmts)
    }
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_32_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel1::Avg32Msrmts)
    }
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_64_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel1::Avg64Msrmts)
    }
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_128_msrmts(self) -> &'a mut crate::W<REG> {
        self.variant(Adsel1::Avg128Msrmts)
    }
}
impl R {
    #[doc = "Bit 0 - This bit enables slot 1 for ADC conversions."]
    #[inline(always)]
    pub fn slen1(&self) -> Slen1R {
        Slen1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 1."]
    #[inline(always)]
    pub fn wcen1(&self) -> Wcen1R {
        Wcen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Select one of the 11 channel inputs for this slot."]
    #[inline(always)]
    pub fn chsel1(&self) -> Chsel1R {
        Chsel1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot 1."]
    #[inline(always)]
    pub fn prmode1(&self) -> Prmode1R {
        Prmode1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:23 - Set additional input signal sampling/tracking time to the specified number of ADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
    #[inline(always)]
    pub fn trkcyc1(&self) -> Trkcyc1R {
        Trkcyc1R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn adsel1(&self) -> Adsel1R {
        Adsel1R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This bit enables slot 1 for ADC conversions."]
    #[inline(always)]
    #[must_use]
    pub fn slen1(&mut self) -> Slen1W<Sl1cfgSpec> {
        Slen1W::new(self, 0)
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 1."]
    #[inline(always)]
    #[must_use]
    pub fn wcen1(&mut self) -> Wcen1W<Sl1cfgSpec> {
        Wcen1W::new(self, 1)
    }
    #[doc = "Bits 8:11 - Select one of the 11 channel inputs for this slot."]
    #[inline(always)]
    #[must_use]
    pub fn chsel1(&mut self) -> Chsel1W<Sl1cfgSpec> {
        Chsel1W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot 1."]
    #[inline(always)]
    #[must_use]
    pub fn prmode1(&mut self) -> Prmode1W<Sl1cfgSpec> {
        Prmode1W::new(self, 16)
    }
    #[doc = "Bits 18:23 - Set additional input signal sampling/tracking time to the specified number of ADC clock cycles. (Note that a value of 0 in this register specifies the minimum required 5 cycles. A maximum of 64 specifies 69 cycles.)"]
    #[inline(always)]
    #[must_use]
    pub fn trkcyc1(&mut self) -> Trkcyc1W<Sl1cfgSpec> {
        Trkcyc1W::new(self, 18)
    }
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    #[must_use]
    pub fn adsel1(&mut self) -> Adsel1W<Sl1cfgSpec> {
        Adsel1W::new(self, 24)
    }
}
#[doc = "Slot 1 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sl1cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl1cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sl1cfgSpec;
impl crate::RegisterSpec for Sl1cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sl1cfg::R`](R) reader structure"]
impl crate::Readable for Sl1cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sl1cfg::W`](W) writer structure"]
impl crate::Writable for Sl1cfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SL1CFG to value 0"]
impl crate::Resettable for Sl1cfgSpec {
    const RESET_VALUE: u32 = 0;
}
