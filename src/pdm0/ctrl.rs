#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "PDM Clock enable. If multiple clocks are enabled, priority is HFRC2, HF XTAL, HFRC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clken {
    #[doc = "0: Disable serial clock"]
    Dis = 0,
    #[doc = "1: Enable serial clock"]
    En = 1,
}
impl From<Clken> for bool {
    #[inline(always)]
    fn from(variant: Clken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKEN` reader - PDM Clock enable. If multiple clocks are enabled, priority is HFRC2, HF XTAL, HFRC."]
pub type ClkenR = crate::BitReader<Clken>;
impl ClkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clken {
        match self.bits {
            false => Clken::Dis,
            true => Clken::En,
        }
    }
    #[doc = "Disable serial clock"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Clken::Dis
    }
    #[doc = "Enable serial clock"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Clken::En
    }
}
#[doc = "Field `CLKEN` writer - PDM Clock enable. If multiple clocks are enabled, priority is HFRC2, HF XTAL, HFRC."]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG, Clken>;
impl<'a, REG> ClkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable serial clock"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Clken::Dis)
    }
    #[doc = "Enable serial clock"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Clken::En)
    }
}
#[doc = "Field `CLKSEL` reader - PDM Master Clock select (24.576MHz). 0: HFRC2_192MHz div8 with HFAdj2 1: XTAL_HS Byapss 2: HFRC_96MHz div4"]
pub type ClkselR = crate::FieldReader;
#[doc = "Field `CLKSEL` writer - PDM Master Clock select (24.576MHz). 0: HFRC2_192MHz div8 with HFAdj2 1: XTAL_HS Byapss 2: HFRC_96MHz div4"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Reset IP core. 0 puts the core in reset; 1 takes the core out of reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rstb {
    #[doc = "0: Put the core in reset."]
    Reset = 0,
    #[doc = "1: Core not in reset."]
    Normal = 1,
}
impl From<Rstb> for bool {
    #[inline(always)]
    fn from(variant: Rstb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTB` reader - Reset IP core. 0 puts the core in reset; 1 takes the core out of reset."]
pub type RstbR = crate::BitReader<Rstb>;
impl RstbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rstb {
        match self.bits {
            false => Rstb::Reset,
            true => Rstb::Normal,
        }
    }
    #[doc = "Put the core in reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Rstb::Reset
    }
    #[doc = "Core not in reset."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Rstb::Normal
    }
}
#[doc = "Field `RSTB` writer - Reset IP core. 0 puts the core in reset; 1 takes the core out of reset."]
pub type RstbW<'a, REG> = crate::BitWriter<'a, REG, Rstb>;
impl<'a, REG> RstbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Put the core in reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Rstb::Reset)
    }
    #[doc = "Core not in reset."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Rstb::Normal)
    }
}
#[doc = "Enable PCM packing. Only 24-bit unpacked mode supported.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcmpack {
    #[doc = "0: Disable PCM packing."]
    Dis = 0,
    #[doc = "1: Enable PCM packing."]
    En = 1,
}
impl From<Pcmpack> for bool {
    #[inline(always)]
    fn from(variant: Pcmpack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCMPACK` reader - Enable PCM packing. Only 24-bit unpacked mode supported."]
pub type PcmpackR = crate::BitReader<Pcmpack>;
impl PcmpackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcmpack {
        match self.bits {
            false => Pcmpack::Dis,
            true => Pcmpack::En,
        }
    }
    #[doc = "Disable PCM packing."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pcmpack::Dis
    }
    #[doc = "Enable PCM packing."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pcmpack::En
    }
}
#[doc = "Field `PCMPACK` writer - Enable PCM packing. Only 24-bit unpacked mode supported."]
pub type PcmpackW<'a, REG> = crate::BitWriter<'a, REG, Pcmpack>;
impl<'a, REG> PcmpackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable PCM packing."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pcmpack::Dis)
    }
    #[doc = "Enable PCM packing."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pcmpack::En)
    }
}
#[doc = "PDM enable register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Disable PDM."]
    Dis = 0,
    #[doc = "1: Enable PDM."]
    En = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - PDM enable register"]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::Dis,
            true => En::En,
        }
    }
    #[doc = "Disable PDM."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En::Dis
    }
    #[doc = "Enable PDM."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En::En
    }
}
#[doc = "Field `EN` writer - PDM enable register"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable PDM."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En::Dis)
    }
    #[doc = "Enable PDM."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En::En)
    }
}
impl R {
    #[doc = "Bit 0 - PDM Clock enable. If multiple clocks are enabled, priority is HFRC2, HF XTAL, HFRC."]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - PDM Master Clock select (24.576MHz). 0: HFRC2_192MHz div8 with HFAdj2 1: XTAL_HS Byapss 2: HFRC_96MHz div4"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - Reset IP core. 0 puts the core in reset; 1 takes the core out of reset."]
    #[inline(always)]
    pub fn rstb(&self) -> RstbR {
        RstbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable PCM packing. Only 24-bit unpacked mode supported."]
    #[inline(always)]
    pub fn pcmpack(&self) -> PcmpackR {
        PcmpackR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PDM enable register"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDM Clock enable. If multiple clocks are enabled, priority is HFRC2, HF XTAL, HFRC."]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> ClkenW<CtrlSpec> {
        ClkenW::new(self, 0)
    }
    #[doc = "Bits 1:2 - PDM Master Clock select (24.576MHz). 0: HFRC2_192MHz div8 with HFAdj2 1: XTAL_HS Byapss 2: HFRC_96MHz div4"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> ClkselW<CtrlSpec> {
        ClkselW::new(self, 1)
    }
    #[doc = "Bit 4 - Reset IP core. 0 puts the core in reset; 1 takes the core out of reset."]
    #[inline(always)]
    #[must_use]
    pub fn rstb(&mut self) -> RstbW<CtrlSpec> {
        RstbW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable PCM packing. Only 24-bit unpacked mode supported."]
    #[inline(always)]
    #[must_use]
    pub fn pcmpack(&mut self) -> PcmpackW<CtrlSpec> {
        PcmpackW::new(self, 5)
    }
    #[doc = "Bit 6 - PDM enable register"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<CtrlSpec> {
        EnW::new(self, 6)
    }
}
#[doc = "PDM Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
