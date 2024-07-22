#[doc = "Register `CLKCFG` reader"]
pub type R = crate::R<ClkcfgSpec>;
#[doc = "Register `CLKCFG` writer"]
pub type W = crate::W<ClkcfgSpec>;
#[doc = "Field `IOCLKEN` reader - Enable for the interface clock. Must be enabled prior to executing any IO operations."]
pub type IoclkenR = crate::BitReader;
#[doc = "Field `IOCLKEN` writer - Enable for the interface clock. Must be enabled prior to executing any IO operations."]
pub type IoclkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Select the input clock frequency.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel {
    #[doc = "0: Selects the minimum power clock. This setting should be used whenever the IOM is not active."]
    MinPwr = 0,
    #[doc = "1: Selects static 0 as the input clock. Previously 96Mhz setting"]
    Off = 1,
    #[doc = "2: Selects the HFRC 48MHz as the input clock."]
    Hfrc48mhz = 2,
    #[doc = "3: Selects the HFRC 24MHz as the input clock."]
    Hfrc24mhz = 3,
    #[doc = "4: Selects the HFRC 12MHz as the input clock."]
    Hfrc12mhz = 4,
    #[doc = "5: Selects the HFRC 6MHz as the input clock."]
    Hfrc6mhz = 5,
    #[doc = "6: Selects the HFRC 3MHz as the input clock."]
    Hfrc3mhz = 6,
    #[doc = "7: Selects the HFRC 1.5MHz as the input clock."]
    Hfrc1p5mhz = 7,
}
impl From<Fsel> for u8 {
    #[inline(always)]
    fn from(variant: Fsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel {
    type Ux = u8;
}
impl crate::IsEnum for Fsel {}
#[doc = "Field `FSEL` reader - Select the input clock frequency."]
pub type FselR = crate::FieldReader<Fsel>;
impl FselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel {
        match self.bits {
            0 => Fsel::MinPwr,
            1 => Fsel::Off,
            2 => Fsel::Hfrc48mhz,
            3 => Fsel::Hfrc24mhz,
            4 => Fsel::Hfrc12mhz,
            5 => Fsel::Hfrc6mhz,
            6 => Fsel::Hfrc3mhz,
            7 => Fsel::Hfrc1p5mhz,
            _ => unreachable!(),
        }
    }
    #[doc = "Selects the minimum power clock. This setting should be used whenever the IOM is not active."]
    #[inline(always)]
    pub fn is_min_pwr(&self) -> bool {
        *self == Fsel::MinPwr
    }
    #[doc = "Selects static 0 as the input clock. Previously 96Mhz setting"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Fsel::Off
    }
    #[doc = "Selects the HFRC 48MHz as the input clock."]
    #[inline(always)]
    pub fn is_hfrc48mhz(&self) -> bool {
        *self == Fsel::Hfrc48mhz
    }
    #[doc = "Selects the HFRC 24MHz as the input clock."]
    #[inline(always)]
    pub fn is_hfrc24mhz(&self) -> bool {
        *self == Fsel::Hfrc24mhz
    }
    #[doc = "Selects the HFRC 12MHz as the input clock."]
    #[inline(always)]
    pub fn is_hfrc12mhz(&self) -> bool {
        *self == Fsel::Hfrc12mhz
    }
    #[doc = "Selects the HFRC 6MHz as the input clock."]
    #[inline(always)]
    pub fn is_hfrc6mhz(&self) -> bool {
        *self == Fsel::Hfrc6mhz
    }
    #[doc = "Selects the HFRC 3MHz as the input clock."]
    #[inline(always)]
    pub fn is_hfrc3mhz(&self) -> bool {
        *self == Fsel::Hfrc3mhz
    }
    #[doc = "Selects the HFRC 1.5MHz as the input clock."]
    #[inline(always)]
    pub fn is_hfrc1p5mhz(&self) -> bool {
        *self == Fsel::Hfrc1p5mhz
    }
}
#[doc = "Field `FSEL` writer - Select the input clock frequency."]
pub type FselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel, crate::Safe>;
impl<'a, REG> FselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects the minimum power clock. This setting should be used whenever the IOM is not active."]
    #[inline(always)]
    pub fn min_pwr(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::MinPwr)
    }
    #[doc = "Selects static 0 as the input clock. Previously 96Mhz setting"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Off)
    }
    #[doc = "Selects the HFRC 48MHz as the input clock."]
    #[inline(always)]
    pub fn hfrc48mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Hfrc48mhz)
    }
    #[doc = "Selects the HFRC 24MHz as the input clock."]
    #[inline(always)]
    pub fn hfrc24mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Hfrc24mhz)
    }
    #[doc = "Selects the HFRC 12MHz as the input clock."]
    #[inline(always)]
    pub fn hfrc12mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Hfrc12mhz)
    }
    #[doc = "Selects the HFRC 6MHz as the input clock."]
    #[inline(always)]
    pub fn hfrc6mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Hfrc6mhz)
    }
    #[doc = "Selects the HFRC 3MHz as the input clock."]
    #[inline(always)]
    pub fn hfrc3mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Hfrc3mhz)
    }
    #[doc = "Selects the HFRC 1.5MHz as the input clock."]
    #[inline(always)]
    pub fn hfrc1p5mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Hfrc1p5mhz)
    }
}
#[doc = "Enable divide by 3 of the source IOCLK. Division by 3 is done before the DIVEN programmable divider, and if enabled will provide the divided by 3 clock as the source to the programmable divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Div3 {
    #[doc = "0: Select divide by 1."]
    Dis = 0,
    #[doc = "1: Select divide by 3."]
    En = 1,
}
impl From<Div3> for bool {
    #[inline(always)]
    fn from(variant: Div3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIV3` reader - Enable divide by 3 of the source IOCLK. Division by 3 is done before the DIVEN programmable divider, and if enabled will provide the divided by 3 clock as the source to the programmable divider."]
pub type Div3R = crate::BitReader<Div3>;
impl Div3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Div3 {
        match self.bits {
            false => Div3::Dis,
            true => Div3::En,
        }
    }
    #[doc = "Select divide by 1."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Div3::Dis
    }
    #[doc = "Select divide by 3."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Div3::En
    }
}
#[doc = "Field `DIV3` writer - Enable divide by 3 of the source IOCLK. Division by 3 is done before the DIVEN programmable divider, and if enabled will provide the divided by 3 clock as the source to the programmable divider."]
pub type Div3W<'a, REG> = crate::BitWriter<'a, REG, Div3>;
impl<'a, REG> Div3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select divide by 1."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Div3::Dis)
    }
    #[doc = "Select divide by 3."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Div3::En)
    }
}
#[doc = "Enable clock division by TOTPER and LOWPER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Diven {
    #[doc = "0: Disable TOTPER division."]
    Dis = 0,
    #[doc = "1: Enable TOTPER division."]
    En = 1,
}
impl From<Diven> for bool {
    #[inline(always)]
    fn from(variant: Diven) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIVEN` reader - Enable clock division by TOTPER and LOWPER"]
pub type DivenR = crate::BitReader<Diven>;
impl DivenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Diven {
        match self.bits {
            false => Diven::Dis,
            true => Diven::En,
        }
    }
    #[doc = "Disable TOTPER division."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Diven::Dis
    }
    #[doc = "Enable TOTPER division."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Diven::En
    }
}
#[doc = "Field `DIVEN` writer - Enable clock division by TOTPER and LOWPER"]
pub type DivenW<'a, REG> = crate::BitWriter<'a, REG, Diven>;
impl<'a, REG> DivenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable TOTPER division."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Diven::Dis)
    }
    #[doc = "Enable TOTPER division."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Diven::En)
    }
}
#[doc = "Field `LOWPER` reader - Clock low clock count minus 1. This provides the number of clocks the divided clock will be low when the DIVEN = 1. Only applicable when DIVEN = 1."]
pub type LowperR = crate::FieldReader;
#[doc = "Field `LOWPER` writer - Clock low clock count minus 1. This provides the number of clocks the divided clock will be low when the DIVEN = 1. Only applicable when DIVEN = 1."]
pub type LowperW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TOTPER` reader - Clock total clock count minus 1. This provides the total period of the divided clock -1 when the DIVEN is active. The source clock is selected by FSEL. Only applicable when DIVEN = 1."]
pub type TotperR = crate::FieldReader;
#[doc = "Field `TOTPER` writer - Clock total clock count minus 1. This provides the total period of the divided clock -1 when the DIVEN is active. The source clock is selected by FSEL. Only applicable when DIVEN = 1."]
pub type TotperW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Enable for the interface clock. Must be enabled prior to executing any IO operations."]
    #[inline(always)]
    pub fn ioclken(&self) -> IoclkenR {
        IoclkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - Select the input clock frequency."]
    #[inline(always)]
    pub fn fsel(&self) -> FselR {
        FselR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Enable divide by 3 of the source IOCLK. Division by 3 is done before the DIVEN programmable divider, and if enabled will provide the divided by 3 clock as the source to the programmable divider."]
    #[inline(always)]
    pub fn div3(&self) -> Div3R {
        Div3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable clock division by TOTPER and LOWPER"]
    #[inline(always)]
    pub fn diven(&self) -> DivenR {
        DivenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Clock low clock count minus 1. This provides the number of clocks the divided clock will be low when the DIVEN = 1. Only applicable when DIVEN = 1."]
    #[inline(always)]
    pub fn lowper(&self) -> LowperR {
        LowperR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Clock total clock count minus 1. This provides the total period of the divided clock -1 when the DIVEN is active. The source clock is selected by FSEL. Only applicable when DIVEN = 1."]
    #[inline(always)]
    pub fn totper(&self) -> TotperR {
        TotperR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable for the interface clock. Must be enabled prior to executing any IO operations."]
    #[inline(always)]
    #[must_use]
    pub fn ioclken(&mut self) -> IoclkenW<ClkcfgSpec> {
        IoclkenW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Select the input clock frequency."]
    #[inline(always)]
    #[must_use]
    pub fn fsel(&mut self) -> FselW<ClkcfgSpec> {
        FselW::new(self, 8)
    }
    #[doc = "Bit 11 - Enable divide by 3 of the source IOCLK. Division by 3 is done before the DIVEN programmable divider, and if enabled will provide the divided by 3 clock as the source to the programmable divider."]
    #[inline(always)]
    #[must_use]
    pub fn div3(&mut self) -> Div3W<ClkcfgSpec> {
        Div3W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable clock division by TOTPER and LOWPER"]
    #[inline(always)]
    #[must_use]
    pub fn diven(&mut self) -> DivenW<ClkcfgSpec> {
        DivenW::new(self, 12)
    }
    #[doc = "Bits 16:23 - Clock low clock count minus 1. This provides the number of clocks the divided clock will be low when the DIVEN = 1. Only applicable when DIVEN = 1."]
    #[inline(always)]
    #[must_use]
    pub fn lowper(&mut self) -> LowperW<ClkcfgSpec> {
        LowperW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Clock total clock count minus 1. This provides the total period of the divided clock -1 when the DIVEN is active. The source clock is selected by FSEL. Only applicable when DIVEN = 1."]
    #[inline(always)]
    #[must_use]
    pub fn totper(&mut self) -> TotperW<ClkcfgSpec> {
        TotperW::new(self, 24)
    }
}
#[doc = "Provides clock related controls used internal to the BLEIF module, and enablement of 32KHz clock to the BLE Core module. The internal clock sourced is selected via the FSEL and can be further divided by 3 using the DIV3 control. This register is also used to enable the clock, which must be done prior to performing any IO transactions.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkcfgSpec;
impl crate::RegisterSpec for ClkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkcfg::R`](R) reader structure"]
impl crate::Readable for ClkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`clkcfg::W`](W) writer structure"]
impl crate::Writable for ClkcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCFG to value 0"]
impl crate::Resettable for ClkcfgSpec {
    const RESET_VALUE: u32 = 0;
}
