#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "This bitfield selects the positive input to the comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Psel {
    #[doc = "0: Use VDDADJ for the positive input."]
    Vddadj = 0,
    #[doc = "1: Use the temperature sensor output for the positive input. Note: If this channel is selected for PSEL, the bandap circuit required for temperature comparisons will automatically turn on. The bandgap circuit requires 11us to stabalize."]
    Vtemp = 1,
    #[doc = "2: Use external voltage 0 for positive input."]
    Vext1 = 2,
    #[doc = "3: Use external voltage 1 for positive input."]
    Vext2 = 3,
}
impl From<Psel> for u8 {
    #[inline(always)]
    fn from(variant: Psel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Psel {
    type Ux = u8;
}
impl crate::IsEnum for Psel {}
#[doc = "Field `PSEL` reader - This bitfield selects the positive input to the comparator."]
pub type PselR = crate::FieldReader<Psel>;
impl PselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psel {
        match self.bits {
            0 => Psel::Vddadj,
            1 => Psel::Vtemp,
            2 => Psel::Vext1,
            3 => Psel::Vext2,
            _ => unreachable!(),
        }
    }
    #[doc = "Use VDDADJ for the positive input."]
    #[inline(always)]
    pub fn is_vddadj(&self) -> bool {
        *self == Psel::Vddadj
    }
    #[doc = "Use the temperature sensor output for the positive input. Note: If this channel is selected for PSEL, the bandap circuit required for temperature comparisons will automatically turn on. The bandgap circuit requires 11us to stabalize."]
    #[inline(always)]
    pub fn is_vtemp(&self) -> bool {
        *self == Psel::Vtemp
    }
    #[doc = "Use external voltage 0 for positive input."]
    #[inline(always)]
    pub fn is_vext1(&self) -> bool {
        *self == Psel::Vext1
    }
    #[doc = "Use external voltage 1 for positive input."]
    #[inline(always)]
    pub fn is_vext2(&self) -> bool {
        *self == Psel::Vext2
    }
}
#[doc = "Field `PSEL` writer - This bitfield selects the positive input to the comparator."]
pub type PselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Psel, crate::Safe>;
impl<'a, REG> PselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use VDDADJ for the positive input."]
    #[inline(always)]
    pub fn vddadj(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::Vddadj)
    }
    #[doc = "Use the temperature sensor output for the positive input. Note: If this channel is selected for PSEL, the bandap circuit required for temperature comparisons will automatically turn on. The bandgap circuit requires 11us to stabalize."]
    #[inline(always)]
    pub fn vtemp(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::Vtemp)
    }
    #[doc = "Use external voltage 0 for positive input."]
    #[inline(always)]
    pub fn vext1(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::Vext1)
    }
    #[doc = "Use external voltage 1 for positive input."]
    #[inline(always)]
    pub fn vext2(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::Vext2)
    }
}
#[doc = "This bitfield selects the negative input to the comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nsel {
    #[doc = "0: Use external reference 1 for reference input."]
    Vrefext1 = 0,
    #[doc = "1: Use external reference 2 for reference input."]
    Vrefext2 = 1,
    #[doc = "2: Use external reference 3 for reference input."]
    Vrefext3 = 2,
    #[doc = "3: Use DAC output selected by LVLSEL for reference input."]
    Dac = 3,
}
impl From<Nsel> for u8 {
    #[inline(always)]
    fn from(variant: Nsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nsel {
    type Ux = u8;
}
impl crate::IsEnum for Nsel {}
#[doc = "Field `NSEL` reader - This bitfield selects the negative input to the comparator."]
pub type NselR = crate::FieldReader<Nsel>;
impl NselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nsel {
        match self.bits {
            0 => Nsel::Vrefext1,
            1 => Nsel::Vrefext2,
            2 => Nsel::Vrefext3,
            3 => Nsel::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "Use external reference 1 for reference input."]
    #[inline(always)]
    pub fn is_vrefext1(&self) -> bool {
        *self == Nsel::Vrefext1
    }
    #[doc = "Use external reference 2 for reference input."]
    #[inline(always)]
    pub fn is_vrefext2(&self) -> bool {
        *self == Nsel::Vrefext2
    }
    #[doc = "Use external reference 3 for reference input."]
    #[inline(always)]
    pub fn is_vrefext3(&self) -> bool {
        *self == Nsel::Vrefext3
    }
    #[doc = "Use DAC output selected by LVLSEL for reference input."]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Nsel::Dac
    }
}
#[doc = "Field `NSEL` writer - This bitfield selects the negative input to the comparator."]
pub type NselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nsel, crate::Safe>;
impl<'a, REG> NselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use external reference 1 for reference input."]
    #[inline(always)]
    pub fn vrefext1(self) -> &'a mut crate::W<REG> {
        self.variant(Nsel::Vrefext1)
    }
    #[doc = "Use external reference 2 for reference input."]
    #[inline(always)]
    pub fn vrefext2(self) -> &'a mut crate::W<REG> {
        self.variant(Nsel::Vrefext2)
    }
    #[doc = "Use external reference 3 for reference input."]
    #[inline(always)]
    pub fn vrefext3(self) -> &'a mut crate::W<REG> {
        self.variant(Nsel::Vrefext3)
    }
    #[doc = "Use DAC output selected by LVLSEL for reference input."]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Nsel::Dac)
    }
}
#[doc = "When the reference input NSEL is set to NSEL_DAC, this bitfield selects the voltage level for the negative input to the comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lvlsel {
    #[doc = "0: Set Reference input to 0.58 Volts."]
    _0p58v = 0,
    #[doc = "1: Set Reference input to 0.77 Volts."]
    _0p77v = 1,
    #[doc = "2: Set Reference input to 0.97 Volts."]
    _0p97v = 2,
    #[doc = "3: Set Reference input to 1.16 Volts."]
    _1p16v = 3,
    #[doc = "4: Set Reference input to 1.35 Volts."]
    _1p35v = 4,
    #[doc = "5: Set Reference input to 1.55 Volts."]
    _1p55v = 5,
    #[doc = "6: Set Reference input to 1.74 Volts."]
    _1p74v = 6,
    #[doc = "7: Set Reference input to 1.93 Volts."]
    _1p93v = 7,
    #[doc = "8: Set Reference input to 2.13 Volts."]
    _2p13v = 8,
}
impl From<Lvlsel> for u8 {
    #[inline(always)]
    fn from(variant: Lvlsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lvlsel {
    type Ux = u8;
}
impl crate::IsEnum for Lvlsel {}
#[doc = "Field `LVLSEL` reader - When the reference input NSEL is set to NSEL_DAC, this bitfield selects the voltage level for the negative input to the comparator."]
pub type LvlselR = crate::FieldReader<Lvlsel>;
impl LvlselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lvlsel> {
        match self.bits {
            0 => Some(Lvlsel::_0p58v),
            1 => Some(Lvlsel::_0p77v),
            2 => Some(Lvlsel::_0p97v),
            3 => Some(Lvlsel::_1p16v),
            4 => Some(Lvlsel::_1p35v),
            5 => Some(Lvlsel::_1p55v),
            6 => Some(Lvlsel::_1p74v),
            7 => Some(Lvlsel::_1p93v),
            8 => Some(Lvlsel::_2p13v),
            _ => None,
        }
    }
    #[doc = "Set Reference input to 0.58 Volts."]
    #[inline(always)]
    pub fn is_0p58v(&self) -> bool {
        *self == Lvlsel::_0p58v
    }
    #[doc = "Set Reference input to 0.77 Volts."]
    #[inline(always)]
    pub fn is_0p77v(&self) -> bool {
        *self == Lvlsel::_0p77v
    }
    #[doc = "Set Reference input to 0.97 Volts."]
    #[inline(always)]
    pub fn is_0p97v(&self) -> bool {
        *self == Lvlsel::_0p97v
    }
    #[doc = "Set Reference input to 1.16 Volts."]
    #[inline(always)]
    pub fn is_1p16v(&self) -> bool {
        *self == Lvlsel::_1p16v
    }
    #[doc = "Set Reference input to 1.35 Volts."]
    #[inline(always)]
    pub fn is_1p35v(&self) -> bool {
        *self == Lvlsel::_1p35v
    }
    #[doc = "Set Reference input to 1.55 Volts."]
    #[inline(always)]
    pub fn is_1p55v(&self) -> bool {
        *self == Lvlsel::_1p55v
    }
    #[doc = "Set Reference input to 1.74 Volts."]
    #[inline(always)]
    pub fn is_1p74v(&self) -> bool {
        *self == Lvlsel::_1p74v
    }
    #[doc = "Set Reference input to 1.93 Volts."]
    #[inline(always)]
    pub fn is_1p93v(&self) -> bool {
        *self == Lvlsel::_1p93v
    }
    #[doc = "Set Reference input to 2.13 Volts."]
    #[inline(always)]
    pub fn is_2p13v(&self) -> bool {
        *self == Lvlsel::_2p13v
    }
}
#[doc = "Field `LVLSEL` writer - When the reference input NSEL is set to NSEL_DAC, this bitfield selects the voltage level for the negative input to the comparator."]
pub type LvlselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Lvlsel>;
impl<'a, REG> LvlselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set Reference input to 0.58 Volts."]
    #[inline(always)]
    pub fn _0p58v(self) -> &'a mut crate::W<REG> {
        self.variant(Lvlsel::_0p58v)
    }
    #[doc = "Set Reference input to 0.77 Volts."]
    #[inline(always)]
    pub fn _0p77v(self) -> &'a mut crate::W<REG> {
        self.variant(Lvlsel::_0p77v)
    }
    #[doc = "Set Reference input to 0.97 Volts."]
    #[inline(always)]
    pub fn _0p97v(self) -> &'a mut crate::W<REG> {
        self.variant(Lvlsel::_0p97v)
    }
    #[doc = "Set Reference input to 1.16 Volts."]
    #[inline(always)]
    pub fn _1p16v(self) -> &'a mut crate::W<REG> {
        self.variant(Lvlsel::_1p16v)
    }
    #[doc = "Set Reference input to 1.35 Volts."]
    #[inline(always)]
    pub fn _1p35v(self) -> &'a mut crate::W<REG> {
        self.variant(Lvlsel::_1p35v)
    }
    #[doc = "Set Reference input to 1.55 Volts."]
    #[inline(always)]
    pub fn _1p55v(self) -> &'a mut crate::W<REG> {
        self.variant(Lvlsel::_1p55v)
    }
    #[doc = "Set Reference input to 1.74 Volts."]
    #[inline(always)]
    pub fn _1p74v(self) -> &'a mut crate::W<REG> {
        self.variant(Lvlsel::_1p74v)
    }
    #[doc = "Set Reference input to 1.93 Volts."]
    #[inline(always)]
    pub fn _1p93v(self) -> &'a mut crate::W<REG> {
        self.variant(Lvlsel::_1p93v)
    }
    #[doc = "Set Reference input to 2.13 Volts."]
    #[inline(always)]
    pub fn _2p13v(self) -> &'a mut crate::W<REG> {
        self.variant(Lvlsel::_2p13v)
    }
}
impl R {
    #[doc = "Bits 0:1 - This bitfield selects the positive input to the comparator."]
    #[inline(always)]
    pub fn psel(&self) -> PselR {
        PselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - This bitfield selects the negative input to the comparator."]
    #[inline(always)]
    pub fn nsel(&self) -> NselR {
        NselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:19 - When the reference input NSEL is set to NSEL_DAC, this bitfield selects the voltage level for the negative input to the comparator."]
    #[inline(always)]
    pub fn lvlsel(&self) -> LvlselR {
        LvlselR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - This bitfield selects the positive input to the comparator."]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PselW<CfgSpec> {
        PselW::new(self, 0)
    }
    #[doc = "Bits 8:9 - This bitfield selects the negative input to the comparator."]
    #[inline(always)]
    #[must_use]
    pub fn nsel(&mut self) -> NselW<CfgSpec> {
        NselW::new(self, 8)
    }
    #[doc = "Bits 16:19 - When the reference input NSEL is set to NSEL_DAC, this bitfield selects the voltage level for the negative input to the comparator."]
    #[inline(always)]
    #[must_use]
    pub fn lvlsel(&mut self) -> LvlselW<CfgSpec> {
        LvlselW::new(self, 16)
    }
}
#[doc = "The Voltage Comparator Configuration Register contains the software control for selecting beween the 4 options for the positive input as well as the multiple options for the reference input.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0;
}
