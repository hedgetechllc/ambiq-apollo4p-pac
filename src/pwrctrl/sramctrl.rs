#[doc = "Register `SRAMCTRL` reader"]
pub type R = crate::R<SramctrlSpec>;
#[doc = "Register `SRAMCTRL` writer"]
pub type W = crate::W<SramctrlSpec>;
#[doc = "This bit is 1 if clock gating is allowed for individual system SRAMs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sramclkgate {
    #[doc = "1: Enable Individual SRAM Clock Gating"]
    En = 1,
    #[doc = "0: Disables Individual SRAM Clock Gating"]
    Dis = 0,
}
impl From<Sramclkgate> for bool {
    #[inline(always)]
    fn from(variant: Sramclkgate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAMCLKGATE` reader - This bit is 1 if clock gating is allowed for individual system SRAMs"]
pub type SramclkgateR = crate::BitReader<Sramclkgate>;
impl SramclkgateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sramclkgate {
        match self.bits {
            true => Sramclkgate::En,
            false => Sramclkgate::Dis,
        }
    }
    #[doc = "Enable Individual SRAM Clock Gating"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Sramclkgate::En
    }
    #[doc = "Disables Individual SRAM Clock Gating"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Sramclkgate::Dis
    }
}
#[doc = "Field `SRAMCLKGATE` writer - This bit is 1 if clock gating is allowed for individual system SRAMs"]
pub type SramclkgateW<'a, REG> = crate::BitWriter<'a, REG, Sramclkgate>;
impl<'a, REG> SramclkgateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Individual SRAM Clock Gating"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Sramclkgate::En)
    }
    #[doc = "Disables Individual SRAM Clock Gating"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Sramclkgate::Dis)
    }
}
#[doc = "This bit is 1 when the master clock gate is enabled (top-level clock gate for entire SRAM block)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srammasterclkgate {
    #[doc = "1: Enable Master SRAM Clock Gate"]
    En = 1,
    #[doc = "0: Disables Master SRAM Clock Gating"]
    Dis = 0,
}
impl From<Srammasterclkgate> for bool {
    #[inline(always)]
    fn from(variant: Srammasterclkgate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAMMASTERCLKGATE` reader - This bit is 1 when the master clock gate is enabled (top-level clock gate for entire SRAM block)"]
pub type SrammasterclkgateR = crate::BitReader<Srammasterclkgate>;
impl SrammasterclkgateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srammasterclkgate {
        match self.bits {
            true => Srammasterclkgate::En,
            false => Srammasterclkgate::Dis,
        }
    }
    #[doc = "Enable Master SRAM Clock Gate"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Srammasterclkgate::En
    }
    #[doc = "Disables Master SRAM Clock Gating"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Srammasterclkgate::Dis
    }
}
#[doc = "Field `SRAMMASTERCLKGATE` writer - This bit is 1 when the master clock gate is enabled (top-level clock gate for entire SRAM block)"]
pub type SrammasterclkgateW<'a, REG> = crate::BitWriter<'a, REG, Srammasterclkgate>;
impl<'a, REG> SrammasterclkgateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Master SRAM Clock Gate"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Srammasterclkgate::En)
    }
    #[doc = "Disables Master SRAM Clock Gating"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Srammasterclkgate::Dis)
    }
}
#[doc = "Light Sleep enable for each TCM/SRAM bank. When 1, corresponding bank will be put into light sleep. For optimal power, banks should be put into light sleep while the system is active but the bank has minimal or no accesses.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Sramlightsleep {
    #[doc = "4095: Enable LIGHT SLEEP for ALL SRAMs"]
    All = 4095,
    #[doc = "0: Disables LIGHT SLEEP for ALL SRAMs"]
    Dis = 0,
}
impl From<Sramlightsleep> for u16 {
    #[inline(always)]
    fn from(variant: Sramlightsleep) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sramlightsleep {
    type Ux = u16;
}
impl crate::IsEnum for Sramlightsleep {}
#[doc = "Field `SRAMLIGHTSLEEP` reader - Light Sleep enable for each TCM/SRAM bank. When 1, corresponding bank will be put into light sleep. For optimal power, banks should be put into light sleep while the system is active but the bank has minimal or no accesses."]
pub type SramlightsleepR = crate::FieldReader<Sramlightsleep>;
impl SramlightsleepR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sramlightsleep> {
        match self.bits {
            4095 => Some(Sramlightsleep::All),
            0 => Some(Sramlightsleep::Dis),
            _ => None,
        }
    }
    #[doc = "Enable LIGHT SLEEP for ALL SRAMs"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == Sramlightsleep::All
    }
    #[doc = "Disables LIGHT SLEEP for ALL SRAMs"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Sramlightsleep::Dis
    }
}
#[doc = "Field `SRAMLIGHTSLEEP` writer - Light Sleep enable for each TCM/SRAM bank. When 1, corresponding bank will be put into light sleep. For optimal power, banks should be put into light sleep while the system is active but the bank has minimal or no accesses."]
pub type SramlightsleepW<'a, REG> = crate::FieldWriter<'a, REG, 12, Sramlightsleep>;
impl<'a, REG> SramlightsleepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Enable LIGHT SLEEP for ALL SRAMs"]
    #[inline(always)]
    pub fn all(self) -> &'a mut crate::W<REG> {
        self.variant(Sramlightsleep::All)
    }
    #[doc = "Disables LIGHT SLEEP for ALL SRAMs"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Sramlightsleep::Dis)
    }
}
impl R {
    #[doc = "Bit 1 - This bit is 1 if clock gating is allowed for individual system SRAMs"]
    #[inline(always)]
    pub fn sramclkgate(&self) -> SramclkgateR {
        SramclkgateR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is 1 when the master clock gate is enabled (top-level clock gate for entire SRAM block)"]
    #[inline(always)]
    pub fn srammasterclkgate(&self) -> SrammasterclkgateR {
        SrammasterclkgateR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:19 - Light Sleep enable for each TCM/SRAM bank. When 1, corresponding bank will be put into light sleep. For optimal power, banks should be put into light sleep while the system is active but the bank has minimal or no accesses."]
    #[inline(always)]
    pub fn sramlightsleep(&self) -> SramlightsleepR {
        SramlightsleepR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 1 - This bit is 1 if clock gating is allowed for individual system SRAMs"]
    #[inline(always)]
    #[must_use]
    pub fn sramclkgate(&mut self) -> SramclkgateW<SramctrlSpec> {
        SramclkgateW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit is 1 when the master clock gate is enabled (top-level clock gate for entire SRAM block)"]
    #[inline(always)]
    #[must_use]
    pub fn srammasterclkgate(&mut self) -> SrammasterclkgateW<SramctrlSpec> {
        SrammasterclkgateW::new(self, 2)
    }
    #[doc = "Bits 8:19 - Light Sleep enable for each TCM/SRAM bank. When 1, corresponding bank will be put into light sleep. For optimal power, banks should be put into light sleep while the system is active but the bank has minimal or no accesses."]
    #[inline(always)]
    #[must_use]
    pub fn sramlightsleep(&mut self) -> SramlightsleepW<SramctrlSpec> {
        SramlightsleepW::new(self, 8)
    }
}
#[doc = "This register provides additional fine-tune power management controls for the SRAMs and the SRAM controller. This includes enabling light sleep for the SRAM and TCM banks, and clock gating for reduced dynamic power.\n\nYou can [`read`](crate::Reg::read) this register and get [`sramctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramctrlSpec;
impl crate::RegisterSpec for SramctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sramctrl::R`](R) reader structure"]
impl crate::Readable for SramctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sramctrl::W`](W) writer structure"]
impl crate::Writable for SramctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRAMCTRL to value 0"]
impl crate::Resettable for SramctrlSpec {
    const RESET_VALUE: u32 = 0;
}
