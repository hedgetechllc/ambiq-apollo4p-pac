#[doc = "Register `CLOCKENSTAT` reader"]
pub type R = crate::R<ClockenstatSpec>;
#[doc = "Register `CLOCKENSTAT` writer"]
pub type W = crate::W<ClockenstatSpec>;
#[doc = "Clock enable status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Clockenstat {
    #[doc = "16777216: \\[24\\]
Clock enable for PERIPH_ALL_XTAL_EN"]
    PeriphAllXtalEn = 16777216,
    #[doc = "33554432: \\[25\\]
Clock enable for PERIPH_ALL_HFRC_EN"]
    PeriphAllHfrcEn = 33554432,
    #[doc = "67108864: \\[26\\]
HFRC Adjust enabled"]
    Hfadjen = 67108864,
    #[doc = "134217728: \\[27\\]
HFRC HFADJ enabled"]
    HfrcEnHfadj = 134217728,
    #[doc = "268435456: \\[28\\]
~OSEL"]
    NOsel = 268435456,
    #[doc = "536870912: \\[29\\]
XTAL clkout enabled"]
    ClkoutXtalEn = 536870912,
    #[doc = "1073741824: \\[30\\]
HFRC clkout enabled"]
    ClkoutHfrcEn = 1073741824,
}
impl From<Clockenstat> for u32 {
    #[inline(always)]
    fn from(variant: Clockenstat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clockenstat {
    type Ux = u32;
}
impl crate::IsEnum for Clockenstat {}
#[doc = "Field `CLOCKENSTAT` reader - Clock enable status"]
pub type ClockenstatR = crate::FieldReader<Clockenstat>;
impl ClockenstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clockenstat> {
        match self.bits {
            16777216 => Some(Clockenstat::PeriphAllXtalEn),
            33554432 => Some(Clockenstat::PeriphAllHfrcEn),
            67108864 => Some(Clockenstat::Hfadjen),
            134217728 => Some(Clockenstat::HfrcEnHfadj),
            268435456 => Some(Clockenstat::NOsel),
            536870912 => Some(Clockenstat::ClkoutXtalEn),
            1073741824 => Some(Clockenstat::ClkoutHfrcEn),
            _ => None,
        }
    }
    #[doc = "24\\]
Clock enable for PERIPH_ALL_XTAL_EN"]
    #[inline(always)]
    pub fn is_periph_all_xtal_en(&self) -> bool {
        *self == Clockenstat::PeriphAllXtalEn
    }
    #[doc = "25\\]
Clock enable for PERIPH_ALL_HFRC_EN"]
    #[inline(always)]
    pub fn is_periph_all_hfrc_en(&self) -> bool {
        *self == Clockenstat::PeriphAllHfrcEn
    }
    #[doc = "26\\]
HFRC Adjust enabled"]
    #[inline(always)]
    pub fn is_hfadjen(&self) -> bool {
        *self == Clockenstat::Hfadjen
    }
    #[doc = "27\\]
HFRC HFADJ enabled"]
    #[inline(always)]
    pub fn is_hfrc_en_hfadj(&self) -> bool {
        *self == Clockenstat::HfrcEnHfadj
    }
    #[doc = "28\\]
~OSEL"]
    #[inline(always)]
    pub fn is_n_osel(&self) -> bool {
        *self == Clockenstat::NOsel
    }
    #[doc = "29\\]
XTAL clkout enabled"]
    #[inline(always)]
    pub fn is_clkout_xtal_en(&self) -> bool {
        *self == Clockenstat::ClkoutXtalEn
    }
    #[doc = "30\\]
HFRC clkout enabled"]
    #[inline(always)]
    pub fn is_clkout_hfrc_en(&self) -> bool {
        *self == Clockenstat::ClkoutHfrcEn
    }
}
#[doc = "Field `CLOCKENSTAT` writer - Clock enable status"]
pub type ClockenstatW<'a, REG> = crate::FieldWriter<'a, REG, 32, Clockenstat>;
impl<'a, REG> ClockenstatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "24\\]
Clock enable for PERIPH_ALL_XTAL_EN"]
    #[inline(always)]
    pub fn periph_all_xtal_en(self) -> &'a mut crate::W<REG> {
        self.variant(Clockenstat::PeriphAllXtalEn)
    }
    #[doc = "25\\]
Clock enable for PERIPH_ALL_HFRC_EN"]
    #[inline(always)]
    pub fn periph_all_hfrc_en(self) -> &'a mut crate::W<REG> {
        self.variant(Clockenstat::PeriphAllHfrcEn)
    }
    #[doc = "26\\]
HFRC Adjust enabled"]
    #[inline(always)]
    pub fn hfadjen(self) -> &'a mut crate::W<REG> {
        self.variant(Clockenstat::Hfadjen)
    }
    #[doc = "27\\]
HFRC HFADJ enabled"]
    #[inline(always)]
    pub fn hfrc_en_hfadj(self) -> &'a mut crate::W<REG> {
        self.variant(Clockenstat::HfrcEnHfadj)
    }
    #[doc = "28\\]
~OSEL"]
    #[inline(always)]
    pub fn n_osel(self) -> &'a mut crate::W<REG> {
        self.variant(Clockenstat::NOsel)
    }
    #[doc = "29\\]
XTAL clkout enabled"]
    #[inline(always)]
    pub fn clkout_xtal_en(self) -> &'a mut crate::W<REG> {
        self.variant(Clockenstat::ClkoutXtalEn)
    }
    #[doc = "30\\]
HFRC clkout enabled"]
    #[inline(always)]
    pub fn clkout_hfrc_en(self) -> &'a mut crate::W<REG> {
        self.variant(Clockenstat::ClkoutHfrcEn)
    }
}
impl R {
    #[doc = "Bits 0:31 - Clock enable status"]
    #[inline(always)]
    pub fn clockenstat(&self) -> ClockenstatR {
        ClockenstatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Clock enable status"]
    #[inline(always)]
    #[must_use]
    pub fn clockenstat(&mut self) -> ClockenstatW<ClockenstatSpec> {
        ClockenstatW::new(self, 0)
    }
}
#[doc = "This register provides the enable status to all the peripheral clocks.\n\nYou can [`read`](crate::Reg::read) this register and get [`clockenstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clockenstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockenstatSpec;
impl crate::RegisterSpec for ClockenstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clockenstat::R`](R) reader structure"]
impl crate::Readable for ClockenstatSpec {}
#[doc = "`write(|w| ..)` method takes [`clockenstat::W`](W) writer structure"]
impl crate::Writable for ClockenstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLOCKENSTAT to value 0"]
impl crate::Resettable for ClockenstatSpec {
    const RESET_VALUE: u32 = 0;
}
