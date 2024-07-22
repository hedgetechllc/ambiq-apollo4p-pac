#[doc = "Register `DISPCLKCTRL` reader"]
pub type R = crate::R<DispclkctrlSpec>;
#[doc = "Register `DISPCLKCTRL` writer"]
pub type W = crate::W<DispclkctrlSpec>;
#[doc = "Selection for PLL reference clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pllclksel {
    #[doc = "0: Static value of 0 selected for DPHY clock input"]
    Off = 0,
    #[doc = "1: 12MHz sourced from the HFRC"]
    Hfrc12 = 1,
    #[doc = "2: 6MHz sourced from the HFRC"]
    Hfrc6 = 2,
    #[doc = "3: High Frequency XTAL input (16MHz)"]
    Hfxt16 = 3,
}
impl From<Pllclksel> for u8 {
    #[inline(always)]
    fn from(variant: Pllclksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pllclksel {
    type Ux = u8;
}
impl crate::IsEnum for Pllclksel {}
#[doc = "Field `PLLCLKSEL` reader - Selection for PLL reference clock."]
pub type PllclkselR = crate::FieldReader<Pllclksel>;
impl PllclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllclksel {
        match self.bits {
            0 => Pllclksel::Off,
            1 => Pllclksel::Hfrc12,
            2 => Pllclksel::Hfrc6,
            3 => Pllclksel::Hfxt16,
            _ => unreachable!(),
        }
    }
    #[doc = "Static value of 0 selected for DPHY clock input"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pllclksel::Off
    }
    #[doc = "12MHz sourced from the HFRC"]
    #[inline(always)]
    pub fn is_hfrc12(&self) -> bool {
        *self == Pllclksel::Hfrc12
    }
    #[doc = "6MHz sourced from the HFRC"]
    #[inline(always)]
    pub fn is_hfrc6(&self) -> bool {
        *self == Pllclksel::Hfrc6
    }
    #[doc = "High Frequency XTAL input (16MHz)"]
    #[inline(always)]
    pub fn is_hfxt_16(&self) -> bool {
        *self == Pllclksel::Hfxt16
    }
}
#[doc = "Field `PLLCLKSEL` writer - Selection for PLL reference clock."]
pub type PllclkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pllclksel, crate::Safe>;
impl<'a, REG> PllclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Static value of 0 selected for DPHY clock input"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pllclksel::Off)
    }
    #[doc = "12MHz sourced from the HFRC"]
    #[inline(always)]
    pub fn hfrc12(self) -> &'a mut crate::W<REG> {
        self.variant(Pllclksel::Hfrc12)
    }
    #[doc = "6MHz sourced from the HFRC"]
    #[inline(always)]
    pub fn hfrc6(self) -> &'a mut crate::W<REG> {
        self.variant(Pllclksel::Hfrc6)
    }
    #[doc = "High Frequency XTAL input (16MHz)"]
    #[inline(always)]
    pub fn hfxt_16(self) -> &'a mut crate::W<REG> {
        self.variant(Pllclksel::Hfxt16)
    }
}
#[doc = "Field `PLLCLKEN` reader - Enable for the PLL clock through clkgen"]
pub type PllclkenR = crate::BitReader;
#[doc = "Field `PLLCLKEN` writer - Enable for the PLL clock through clkgen"]
pub type PllclkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Selection for PLL reference clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dispclksel {
    #[doc = "0: Static value of 0 selected for DPHY clock input"]
    Off = 0,
    #[doc = "1: 48MHz sourced from the HFRC"]
    Hfrc48 = 1,
    #[doc = "2: 96MHz sourced from the HFRC"]
    Hfrc96 = 2,
    #[doc = "3: DPHY PLL"]
    Dphypll = 3,
}
impl From<Dispclksel> for u8 {
    #[inline(always)]
    fn from(variant: Dispclksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dispclksel {
    type Ux = u8;
}
impl crate::IsEnum for Dispclksel {}
#[doc = "Field `DISPCLKSEL` reader - Selection for PLL reference clock."]
pub type DispclkselR = crate::FieldReader<Dispclksel>;
impl DispclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dispclksel {
        match self.bits {
            0 => Dispclksel::Off,
            1 => Dispclksel::Hfrc48,
            2 => Dispclksel::Hfrc96,
            3 => Dispclksel::Dphypll,
            _ => unreachable!(),
        }
    }
    #[doc = "Static value of 0 selected for DPHY clock input"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Dispclksel::Off
    }
    #[doc = "48MHz sourced from the HFRC"]
    #[inline(always)]
    pub fn is_hfrc48(&self) -> bool {
        *self == Dispclksel::Hfrc48
    }
    #[doc = "96MHz sourced from the HFRC"]
    #[inline(always)]
    pub fn is_hfrc96(&self) -> bool {
        *self == Dispclksel::Hfrc96
    }
    #[doc = "DPHY PLL"]
    #[inline(always)]
    pub fn is_dphypll(&self) -> bool {
        *self == Dispclksel::Dphypll
    }
}
#[doc = "Field `DISPCLKSEL` writer - Selection for PLL reference clock."]
pub type DispclkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dispclksel, crate::Safe>;
impl<'a, REG> DispclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Static value of 0 selected for DPHY clock input"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Dispclksel::Off)
    }
    #[doc = "48MHz sourced from the HFRC"]
    #[inline(always)]
    pub fn hfrc48(self) -> &'a mut crate::W<REG> {
        self.variant(Dispclksel::Hfrc48)
    }
    #[doc = "96MHz sourced from the HFRC"]
    #[inline(always)]
    pub fn hfrc96(self) -> &'a mut crate::W<REG> {
        self.variant(Dispclksel::Hfrc96)
    }
    #[doc = "DPHY PLL"]
    #[inline(always)]
    pub fn dphypll(self) -> &'a mut crate::W<REG> {
        self.variant(Dispclksel::Dphypll)
    }
}
#[doc = "Field `DCCLKEN` reader - Enable for the PLL clock through clkgen"]
pub type DcclkenR = crate::BitReader;
#[doc = "Field `DCCLKEN` writer - Enable for the PLL clock through clkgen"]
pub type DcclkenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Selection for PLL reference clock."]
    #[inline(always)]
    pub fn pllclksel(&self) -> PllclkselR {
        PllclkselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Enable for the PLL clock through clkgen"]
    #[inline(always)]
    pub fn pllclken(&self) -> PllclkenR {
        PllclkenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Selection for PLL reference clock."]
    #[inline(always)]
    pub fn dispclksel(&self) -> DispclkselR {
        DispclkselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Enable for the PLL clock through clkgen"]
    #[inline(always)]
    pub fn dcclken(&self) -> DcclkenR {
        DcclkenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selection for PLL reference clock."]
    #[inline(always)]
    #[must_use]
    pub fn pllclksel(&mut self) -> PllclkselW<DispclkctrlSpec> {
        PllclkselW::new(self, 0)
    }
    #[doc = "Bit 3 - Enable for the PLL clock through clkgen"]
    #[inline(always)]
    #[must_use]
    pub fn pllclken(&mut self) -> PllclkenW<DispclkctrlSpec> {
        PllclkenW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Selection for PLL reference clock."]
    #[inline(always)]
    #[must_use]
    pub fn dispclksel(&mut self) -> DispclkselW<DispclkctrlSpec> {
        DispclkselW::new(self, 4)
    }
    #[doc = "Bit 7 - Enable for the PLL clock through clkgen"]
    #[inline(always)]
    #[must_use]
    pub fn dcclken(&mut self) -> DcclkenW<DispclkctrlSpec> {
        DcclkenW::new(self, 7)
    }
}
#[doc = "Provides ability to select the PLL reference clock, and derivative of the display clock\n\nYou can [`read`](crate::Reg::read) this register and get [`dispclkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dispclkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DispclkctrlSpec;
impl crate::RegisterSpec for DispclkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dispclkctrl::R`](R) reader structure"]
impl crate::Readable for DispclkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dispclkctrl::W`](W) writer structure"]
impl crate::Writable for DispclkctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DISPCLKCTRL to value 0"]
impl crate::Resettable for DispclkctrlSpec {
    const RESET_VALUE: u32 = 0;
}
