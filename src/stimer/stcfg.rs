#[doc = "Register `STCFG` reader"]
pub type R = crate::R<StcfgSpec>;
#[doc = "Register `STCFG` writer"]
pub type W = crate::W<StcfgSpec>;
#[doc = "Selects an appropriate clock source and divider to use for the System Timer clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "0: No clock enabled."]
    Noclk = 0,
    #[doc = "1: 6MHz from the HFRC clock divider."]
    Hfrc6mhz = 1,
    #[doc = "2: 375KHz from the HFRC clock divider."]
    Hfrc375khz = 2,
    #[doc = "3: 32768Hz from the crystal oscillator."]
    Xtal32khz = 3,
    #[doc = "4: 16384Hz from the crystal oscillator."]
    Xtal16khz = 4,
    #[doc = "5: 1024Hz from the crystal oscillator."]
    Xtal1khz = 5,
    #[doc = "6: Approximately 1KHz from the LFRC oscillator (uncalibrated)."]
    Lfrc1khz = 6,
    #[doc = "7: Use CTIMER 0 for the clock source (allows prescaling from other system clocks)."]
    Ctimer0 = 7,
    #[doc = "8: Use CTIMER 1 for the clock source (allows prescaling from other system clocks)."]
    Ctimer1 = 8,
}
impl From<Clksel> for u8 {
    #[inline(always)]
    fn from(variant: Clksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clksel {
    type Ux = u8;
}
impl crate::IsEnum for Clksel {}
#[doc = "Field `CLKSEL` reader - Selects an appropriate clock source and divider to use for the System Timer clock."]
pub type ClkselR = crate::FieldReader<Clksel>;
impl ClkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clksel> {
        match self.bits {
            0 => Some(Clksel::Noclk),
            1 => Some(Clksel::Hfrc6mhz),
            2 => Some(Clksel::Hfrc375khz),
            3 => Some(Clksel::Xtal32khz),
            4 => Some(Clksel::Xtal16khz),
            5 => Some(Clksel::Xtal1khz),
            6 => Some(Clksel::Lfrc1khz),
            7 => Some(Clksel::Ctimer0),
            8 => Some(Clksel::Ctimer1),
            _ => None,
        }
    }
    #[doc = "No clock enabled."]
    #[inline(always)]
    pub fn is_noclk(&self) -> bool {
        *self == Clksel::Noclk
    }
    #[doc = "6MHz from the HFRC clock divider."]
    #[inline(always)]
    pub fn is_hfrc_6mhz(&self) -> bool {
        *self == Clksel::Hfrc6mhz
    }
    #[doc = "375KHz from the HFRC clock divider."]
    #[inline(always)]
    pub fn is_hfrc_375khz(&self) -> bool {
        *self == Clksel::Hfrc375khz
    }
    #[doc = "32768Hz from the crystal oscillator."]
    #[inline(always)]
    pub fn is_xtal_32khz(&self) -> bool {
        *self == Clksel::Xtal32khz
    }
    #[doc = "16384Hz from the crystal oscillator."]
    #[inline(always)]
    pub fn is_xtal_16khz(&self) -> bool {
        *self == Clksel::Xtal16khz
    }
    #[doc = "1024Hz from the crystal oscillator."]
    #[inline(always)]
    pub fn is_xtal_1khz(&self) -> bool {
        *self == Clksel::Xtal1khz
    }
    #[doc = "Approximately 1KHz from the LFRC oscillator (uncalibrated)."]
    #[inline(always)]
    pub fn is_lfrc_1khz(&self) -> bool {
        *self == Clksel::Lfrc1khz
    }
    #[doc = "Use CTIMER 0 for the clock source (allows prescaling from other system clocks)."]
    #[inline(always)]
    pub fn is_ctimer0(&self) -> bool {
        *self == Clksel::Ctimer0
    }
    #[doc = "Use CTIMER 1 for the clock source (allows prescaling from other system clocks)."]
    #[inline(always)]
    pub fn is_ctimer1(&self) -> bool {
        *self == Clksel::Ctimer1
    }
}
#[doc = "Field `CLKSEL` writer - Selects an appropriate clock source and divider to use for the System Timer clock."]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Clksel>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock enabled."]
    #[inline(always)]
    pub fn noclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Noclk)
    }
    #[doc = "6MHz from the HFRC clock divider."]
    #[inline(always)]
    pub fn hfrc_6mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Hfrc6mhz)
    }
    #[doc = "375KHz from the HFRC clock divider."]
    #[inline(always)]
    pub fn hfrc_375khz(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Hfrc375khz)
    }
    #[doc = "32768Hz from the crystal oscillator."]
    #[inline(always)]
    pub fn xtal_32khz(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Xtal32khz)
    }
    #[doc = "16384Hz from the crystal oscillator."]
    #[inline(always)]
    pub fn xtal_16khz(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Xtal16khz)
    }
    #[doc = "1024Hz from the crystal oscillator."]
    #[inline(always)]
    pub fn xtal_1khz(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Xtal1khz)
    }
    #[doc = "Approximately 1KHz from the LFRC oscillator (uncalibrated)."]
    #[inline(always)]
    pub fn lfrc_1khz(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Lfrc1khz)
    }
    #[doc = "Use CTIMER 0 for the clock source (allows prescaling from other system clocks)."]
    #[inline(always)]
    pub fn ctimer0(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Ctimer0)
    }
    #[doc = "Use CTIMER 1 for the clock source (allows prescaling from other system clocks)."]
    #[inline(always)]
    pub fn ctimer1(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Ctimer1)
    }
}
#[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compareaen {
    #[doc = "0: Compare A disabled."]
    Disable = 0,
    #[doc = "1: Compare A enabled."]
    Enable = 1,
}
impl From<Compareaen> for bool {
    #[inline(always)]
    fn from(variant: Compareaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREAEN` reader - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub type CompareaenR = crate::BitReader<Compareaen>;
impl CompareaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compareaen {
        match self.bits {
            false => Compareaen::Disable,
            true => Compareaen::Enable,
        }
    }
    #[doc = "Compare A disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Compareaen::Disable
    }
    #[doc = "Compare A enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Compareaen::Enable
    }
}
#[doc = "Field `COMPAREAEN` writer - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub type CompareaenW<'a, REG> = crate::BitWriter<'a, REG, Compareaen>;
impl<'a, REG> CompareaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare A disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Compareaen::Disable)
    }
    #[doc = "Compare A enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Compareaen::Enable)
    }
}
#[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compareben {
    #[doc = "0: Compare B disabled."]
    Disable = 0,
    #[doc = "1: Compare B enabled."]
    Enable = 1,
}
impl From<Compareben> for bool {
    #[inline(always)]
    fn from(variant: Compareben) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREBEN` reader - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub type ComparebenR = crate::BitReader<Compareben>;
impl ComparebenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compareben {
        match self.bits {
            false => Compareben::Disable,
            true => Compareben::Enable,
        }
    }
    #[doc = "Compare B disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Compareben::Disable
    }
    #[doc = "Compare B enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Compareben::Enable
    }
}
#[doc = "Field `COMPAREBEN` writer - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub type ComparebenW<'a, REG> = crate::BitWriter<'a, REG, Compareben>;
impl<'a, REG> ComparebenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare B disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Compareben::Disable)
    }
    #[doc = "Compare B enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Compareben::Enable)
    }
}
#[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparecen {
    #[doc = "0: Compare C disabled."]
    Disable = 0,
    #[doc = "1: Compare C enabled."]
    Enable = 1,
}
impl From<Comparecen> for bool {
    #[inline(always)]
    fn from(variant: Comparecen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARECEN` reader - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub type ComparecenR = crate::BitReader<Comparecen>;
impl ComparecenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Comparecen {
        match self.bits {
            false => Comparecen::Disable,
            true => Comparecen::Enable,
        }
    }
    #[doc = "Compare C disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Comparecen::Disable
    }
    #[doc = "Compare C enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Comparecen::Enable
    }
}
#[doc = "Field `COMPARECEN` writer - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub type ComparecenW<'a, REG> = crate::BitWriter<'a, REG, Comparecen>;
impl<'a, REG> ComparecenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare C disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Comparecen::Disable)
    }
    #[doc = "Compare C enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Comparecen::Enable)
    }
}
#[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compareden {
    #[doc = "0: Compare D disabled."]
    Disable = 0,
    #[doc = "1: Compare D enabled."]
    Enable = 1,
}
impl From<Compareden> for bool {
    #[inline(always)]
    fn from(variant: Compareden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREDEN` reader - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub type ComparedenR = crate::BitReader<Compareden>;
impl ComparedenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compareden {
        match self.bits {
            false => Compareden::Disable,
            true => Compareden::Enable,
        }
    }
    #[doc = "Compare D disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Compareden::Disable
    }
    #[doc = "Compare D enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Compareden::Enable
    }
}
#[doc = "Field `COMPAREDEN` writer - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub type ComparedenW<'a, REG> = crate::BitWriter<'a, REG, Compareden>;
impl<'a, REG> ComparedenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare D disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Compareden::Disable)
    }
    #[doc = "Compare D enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Compareden::Enable)
    }
}
#[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compareeen {
    #[doc = "0: Compare E disabled."]
    Disable = 0,
    #[doc = "1: Compare E enabled."]
    Enable = 1,
}
impl From<Compareeen> for bool {
    #[inline(always)]
    fn from(variant: Compareeen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREEEN` reader - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub type CompareeenR = crate::BitReader<Compareeen>;
impl CompareeenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compareeen {
        match self.bits {
            false => Compareeen::Disable,
            true => Compareeen::Enable,
        }
    }
    #[doc = "Compare E disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Compareeen::Disable
    }
    #[doc = "Compare E enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Compareeen::Enable
    }
}
#[doc = "Field `COMPAREEEN` writer - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub type CompareeenW<'a, REG> = crate::BitWriter<'a, REG, Compareeen>;
impl<'a, REG> CompareeenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare E disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Compareeen::Disable)
    }
    #[doc = "Compare E enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Compareeen::Enable)
    }
}
#[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparefen {
    #[doc = "0: Compare F disabled."]
    Disable = 0,
    #[doc = "1: Compare F enabled."]
    Enable = 1,
}
impl From<Comparefen> for bool {
    #[inline(always)]
    fn from(variant: Comparefen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREFEN` reader - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub type ComparefenR = crate::BitReader<Comparefen>;
impl ComparefenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Comparefen {
        match self.bits {
            false => Comparefen::Disable,
            true => Comparefen::Enable,
        }
    }
    #[doc = "Compare F disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Comparefen::Disable
    }
    #[doc = "Compare F enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Comparefen::Enable
    }
}
#[doc = "Field `COMPAREFEN` writer - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub type ComparefenW<'a, REG> = crate::BitWriter<'a, REG, Comparefen>;
impl<'a, REG> ComparefenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare F disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Comparefen::Disable)
    }
    #[doc = "Compare F enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Comparefen::Enable)
    }
}
#[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparegen {
    #[doc = "0: Compare G disabled."]
    Disable = 0,
    #[doc = "1: Compare G enabled."]
    Enable = 1,
}
impl From<Comparegen> for bool {
    #[inline(always)]
    fn from(variant: Comparegen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREGEN` reader - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub type ComparegenR = crate::BitReader<Comparegen>;
impl ComparegenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Comparegen {
        match self.bits {
            false => Comparegen::Disable,
            true => Comparegen::Enable,
        }
    }
    #[doc = "Compare G disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Comparegen::Disable
    }
    #[doc = "Compare G enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Comparegen::Enable
    }
}
#[doc = "Field `COMPAREGEN` writer - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub type ComparegenW<'a, REG> = crate::BitWriter<'a, REG, Comparegen>;
impl<'a, REG> ComparegenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare G disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Comparegen::Disable)
    }
    #[doc = "Compare G enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Comparegen::Enable)
    }
}
#[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparehen {
    #[doc = "0: Compare H disabled."]
    Disable = 0,
    #[doc = "1: Compare H enabled."]
    Enable = 1,
}
impl From<Comparehen> for bool {
    #[inline(always)]
    fn from(variant: Comparehen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREHEN` reader - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub type ComparehenR = crate::BitReader<Comparehen>;
impl ComparehenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Comparehen {
        match self.bits {
            false => Comparehen::Disable,
            true => Comparehen::Enable,
        }
    }
    #[doc = "Compare H disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Comparehen::Disable
    }
    #[doc = "Compare H enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Comparehen::Enable
    }
}
#[doc = "Field `COMPAREHEN` writer - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
pub type ComparehenW<'a, REG> = crate::BitWriter<'a, REG, Comparehen>;
impl<'a, REG> ComparehenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare H disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Comparehen::Disable)
    }
    #[doc = "Compare H enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Comparehen::Enable)
    }
}
#[doc = "Set this bit to one to clear the System Timer register. If this bit is set to '1', the system timer register will stay cleared. It needs to be set to '0' for the system timer to start running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clear {
    #[doc = "0: Let the COUNTER register run on its input clock."]
    Run = 0,
    #[doc = "1: Stop the COUNTER register for loading."]
    Clear = 1,
}
impl From<Clear> for bool {
    #[inline(always)]
    fn from(variant: Clear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLEAR` reader - Set this bit to one to clear the System Timer register. If this bit is set to '1', the system timer register will stay cleared. It needs to be set to '0' for the system timer to start running."]
pub type ClearR = crate::BitReader<Clear>;
impl ClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clear {
        match self.bits {
            false => Clear::Run,
            true => Clear::Clear,
        }
    }
    #[doc = "Let the COUNTER register run on its input clock."]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Clear::Run
    }
    #[doc = "Stop the COUNTER register for loading."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Clear::Clear
    }
}
#[doc = "Field `CLEAR` writer - Set this bit to one to clear the System Timer register. If this bit is set to '1', the system timer register will stay cleared. It needs to be set to '0' for the system timer to start running."]
pub type ClearW<'a, REG> = crate::BitWriter<'a, REG, Clear>;
impl<'a, REG> ClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Let the COUNTER register run on its input clock."]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(Clear::Run)
    }
    #[doc = "Stop the COUNTER register for loading."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Clear::Clear)
    }
}
#[doc = "Set this bit to one to freeze the clock input to the COUNTER register. Once frozen, the value can be safely written from the MCU. Unfreeze to resume.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Freeze {
    #[doc = "0: Let the COUNTER register run on its input clock."]
    Thaw = 0,
    #[doc = "1: Stop the COUNTER register for loading."]
    Freeze = 1,
}
impl From<Freeze> for bool {
    #[inline(always)]
    fn from(variant: Freeze) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREEZE` reader - Set this bit to one to freeze the clock input to the COUNTER register. Once frozen, the value can be safely written from the MCU. Unfreeze to resume."]
pub type FreezeR = crate::BitReader<Freeze>;
impl FreezeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Freeze {
        match self.bits {
            false => Freeze::Thaw,
            true => Freeze::Freeze,
        }
    }
    #[doc = "Let the COUNTER register run on its input clock."]
    #[inline(always)]
    pub fn is_thaw(&self) -> bool {
        *self == Freeze::Thaw
    }
    #[doc = "Stop the COUNTER register for loading."]
    #[inline(always)]
    pub fn is_freeze(&self) -> bool {
        *self == Freeze::Freeze
    }
}
#[doc = "Field `FREEZE` writer - Set this bit to one to freeze the clock input to the COUNTER register. Once frozen, the value can be safely written from the MCU. Unfreeze to resume."]
pub type FreezeW<'a, REG> = crate::BitWriter<'a, REG, Freeze>;
impl<'a, REG> FreezeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Let the COUNTER register run on its input clock."]
    #[inline(always)]
    pub fn thaw(self) -> &'a mut crate::W<REG> {
        self.variant(Freeze::Thaw)
    }
    #[doc = "Stop the COUNTER register for loading."]
    #[inline(always)]
    pub fn freeze(self) -> &'a mut crate::W<REG> {
        self.variant(Freeze::Freeze)
    }
}
impl R {
    #[doc = "Bits 0:3 - Selects an appropriate clock source and divider to use for the System Timer clock."]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compareaen(&self) -> CompareaenR {
        CompareaenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compareben(&self) -> ComparebenR {
        ComparebenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn comparecen(&self) -> ComparecenR {
        ComparecenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compareden(&self) -> ComparedenR {
        ComparedenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compareeen(&self) -> CompareeenR {
        CompareeenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn comparefen(&self) -> ComparefenR {
        ComparefenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn comparegen(&self) -> ComparegenR {
        ComparegenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn comparehen(&self) -> ComparehenR {
        ComparehenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 30 - Set this bit to one to clear the System Timer register. If this bit is set to '1', the system timer register will stay cleared. It needs to be set to '0' for the system timer to start running."]
    #[inline(always)]
    pub fn clear(&self) -> ClearR {
        ClearR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set this bit to one to freeze the clock input to the COUNTER register. Once frozen, the value can be safely written from the MCU. Unfreeze to resume."]
    #[inline(always)]
    pub fn freeze(&self) -> FreezeR {
        FreezeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects an appropriate clock source and divider to use for the System Timer clock."]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> ClkselW<StcfgSpec> {
        ClkselW::new(self, 0)
    }
    #[doc = "Bit 8 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    #[must_use]
    pub fn compareaen(&mut self) -> CompareaenW<StcfgSpec> {
        CompareaenW::new(self, 8)
    }
    #[doc = "Bit 9 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    #[must_use]
    pub fn compareben(&mut self) -> ComparebenW<StcfgSpec> {
        ComparebenW::new(self, 9)
    }
    #[doc = "Bit 10 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    #[must_use]
    pub fn comparecen(&mut self) -> ComparecenW<StcfgSpec> {
        ComparecenW::new(self, 10)
    }
    #[doc = "Bit 11 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    #[must_use]
    pub fn compareden(&mut self) -> ComparedenW<StcfgSpec> {
        ComparedenW::new(self, 11)
    }
    #[doc = "Bit 12 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    #[must_use]
    pub fn compareeen(&mut self) -> CompareeenW<StcfgSpec> {
        CompareeenW::new(self, 12)
    }
    #[doc = "Bit 13 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    #[must_use]
    pub fn comparefen(&mut self) -> ComparefenW<StcfgSpec> {
        ComparefenW::new(self, 13)
    }
    #[doc = "Bit 14 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    #[must_use]
    pub fn comparegen(&mut self) -> ComparegenW<StcfgSpec> {
        ComparegenW::new(self, 14)
    }
    #[doc = "Bit 15 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    #[must_use]
    pub fn comparehen(&mut self) -> ComparehenW<StcfgSpec> {
        ComparehenW::new(self, 15)
    }
    #[doc = "Bit 30 - Set this bit to one to clear the System Timer register. If this bit is set to '1', the system timer register will stay cleared. It needs to be set to '0' for the system timer to start running."]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> ClearW<StcfgSpec> {
        ClearW::new(self, 30)
    }
    #[doc = "Bit 31 - Set this bit to one to freeze the clock input to the COUNTER register. Once frozen, the value can be safely written from the MCU. Unfreeze to resume."]
    #[inline(always)]
    #[must_use]
    pub fn freeze(&mut self) -> FreezeW<StcfgSpec> {
        FreezeW::new(self, 31)
    }
}
#[doc = "The STIMER Configuration Register contains the software control for selecting the clock divider and source feeding the system timer.\n\nYou can [`read`](crate::Reg::read) this register and get [`stcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StcfgSpec;
impl crate::RegisterSpec for StcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stcfg::R`](R) reader structure"]
impl crate::Readable for StcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`stcfg::W`](W) writer structure"]
impl crate::Writable for StcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STCFG to value 0x8000_0000"]
impl crate::Resettable for StcfgSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
