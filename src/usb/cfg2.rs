#[doc = "Register `CFG2` reader"]
pub type R = crate::R<Cfg2Spec>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<Cfg2Spec>;
#[doc = "Out Endpoint 0 Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep0outIntEn {
    #[doc = "0: Out Endpoint interrupt disabled."]
    Dis = 0,
    #[doc = "1: Out Endpoint interrupt enabled."]
    En = 1,
}
impl From<Ep0outIntEn> for bool {
    #[inline(always)]
    fn from(variant: Ep0outIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0OutIntEn` reader - Out Endpoint 0 Interrupt Enable."]
pub type Ep0outIntEnR = crate::BitReader<Ep0outIntEn>;
impl Ep0outIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep0outIntEn {
        match self.bits {
            false => Ep0outIntEn::Dis,
            true => Ep0outIntEn::En,
        }
    }
    #[doc = "Out Endpoint interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ep0outIntEn::Dis
    }
    #[doc = "Out Endpoint interrupt enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ep0outIntEn::En
    }
}
#[doc = "Field `EP0OutIntEn` writer - Out Endpoint 0 Interrupt Enable."]
pub type Ep0outIntEnW<'a, REG> = crate::BitWriter<'a, REG, Ep0outIntEn>;
impl<'a, REG> Ep0outIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Out Endpoint interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ep0outIntEn::Dis)
    }
    #[doc = "Out Endpoint interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ep0outIntEn::En)
    }
}
#[doc = "Out Endpoint 1 Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep1outIntEn {
    #[doc = "0: Out Endpoint interrupt disabled."]
    Dis = 0,
    #[doc = "1: Out Endpoint interrupt enabled."]
    En = 1,
}
impl From<Ep1outIntEn> for bool {
    #[inline(always)]
    fn from(variant: Ep1outIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP1OutIntEn` reader - Out Endpoint 1 Interrupt Enable."]
pub type Ep1outIntEnR = crate::BitReader<Ep1outIntEn>;
impl Ep1outIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep1outIntEn {
        match self.bits {
            false => Ep1outIntEn::Dis,
            true => Ep1outIntEn::En,
        }
    }
    #[doc = "Out Endpoint interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ep1outIntEn::Dis
    }
    #[doc = "Out Endpoint interrupt enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ep1outIntEn::En
    }
}
#[doc = "Field `EP1OutIntEn` writer - Out Endpoint 1 Interrupt Enable."]
pub type Ep1outIntEnW<'a, REG> = crate::BitWriter<'a, REG, Ep1outIntEn>;
impl<'a, REG> Ep1outIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Out Endpoint interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ep1outIntEn::Dis)
    }
    #[doc = "Out Endpoint interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ep1outIntEn::En)
    }
}
#[doc = "Out Endpoint 2 Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep2outIntEn {
    #[doc = "0: Out Endpoint interrupt disabled."]
    Dis = 0,
    #[doc = "1: Out Endpoint interrupt enabled."]
    En = 1,
}
impl From<Ep2outIntEn> for bool {
    #[inline(always)]
    fn from(variant: Ep2outIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP2OutIntEn` reader - Out Endpoint 2 Interrupt Enable."]
pub type Ep2outIntEnR = crate::BitReader<Ep2outIntEn>;
impl Ep2outIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep2outIntEn {
        match self.bits {
            false => Ep2outIntEn::Dis,
            true => Ep2outIntEn::En,
        }
    }
    #[doc = "Out Endpoint interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ep2outIntEn::Dis
    }
    #[doc = "Out Endpoint interrupt enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ep2outIntEn::En
    }
}
#[doc = "Field `EP2OutIntEn` writer - Out Endpoint 2 Interrupt Enable."]
pub type Ep2outIntEnW<'a, REG> = crate::BitWriter<'a, REG, Ep2outIntEn>;
impl<'a, REG> Ep2outIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Out Endpoint interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ep2outIntEn::Dis)
    }
    #[doc = "Out Endpoint interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ep2outIntEn::En)
    }
}
#[doc = "Out Endpoint 3 Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep3outIntEn {
    #[doc = "0: Out Endpoint interrupt disabled."]
    Dis = 0,
    #[doc = "1: Out Endpoint interrupt enabled."]
    En = 1,
}
impl From<Ep3outIntEn> for bool {
    #[inline(always)]
    fn from(variant: Ep3outIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP3OutIntEn` reader - Out Endpoint 3 Interrupt Enable."]
pub type Ep3outIntEnR = crate::BitReader<Ep3outIntEn>;
impl Ep3outIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep3outIntEn {
        match self.bits {
            false => Ep3outIntEn::Dis,
            true => Ep3outIntEn::En,
        }
    }
    #[doc = "Out Endpoint interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ep3outIntEn::Dis
    }
    #[doc = "Out Endpoint interrupt enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ep3outIntEn::En
    }
}
#[doc = "Field `EP3OutIntEn` writer - Out Endpoint 3 Interrupt Enable."]
pub type Ep3outIntEnW<'a, REG> = crate::BitWriter<'a, REG, Ep3outIntEn>;
impl<'a, REG> Ep3outIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Out Endpoint interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ep3outIntEn::Dis)
    }
    #[doc = "Out Endpoint interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ep3outIntEn::En)
    }
}
#[doc = "Out Endpoint 4 Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep4outIntEn {
    #[doc = "0: Out Endpoint interrupt disabled."]
    Dis = 0,
    #[doc = "1: Out Endpoint interrupt enabled."]
    En = 1,
}
impl From<Ep4outIntEn> for bool {
    #[inline(always)]
    fn from(variant: Ep4outIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP4OutIntEn` reader - Out Endpoint 4 Interrupt Enable."]
pub type Ep4outIntEnR = crate::BitReader<Ep4outIntEn>;
impl Ep4outIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep4outIntEn {
        match self.bits {
            false => Ep4outIntEn::Dis,
            true => Ep4outIntEn::En,
        }
    }
    #[doc = "Out Endpoint interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ep4outIntEn::Dis
    }
    #[doc = "Out Endpoint interrupt enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ep4outIntEn::En
    }
}
#[doc = "Field `EP4OutIntEn` writer - Out Endpoint 4 Interrupt Enable."]
pub type Ep4outIntEnW<'a, REG> = crate::BitWriter<'a, REG, Ep4outIntEn>;
impl<'a, REG> Ep4outIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Out Endpoint interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ep4outIntEn::Dis)
    }
    #[doc = "Out Endpoint interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ep4outIntEn::En)
    }
}
#[doc = "Out Endpoint 5 Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep5outIntEn {
    #[doc = "0: Out Endpoint interrupt disabled."]
    Dis = 0,
    #[doc = "1: Out Endpoint interrupt enabled."]
    En = 1,
}
impl From<Ep5outIntEn> for bool {
    #[inline(always)]
    fn from(variant: Ep5outIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP5OutIntEn` reader - Out Endpoint 5 Interrupt Enable."]
pub type Ep5outIntEnR = crate::BitReader<Ep5outIntEn>;
impl Ep5outIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep5outIntEn {
        match self.bits {
            false => Ep5outIntEn::Dis,
            true => Ep5outIntEn::En,
        }
    }
    #[doc = "Out Endpoint interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ep5outIntEn::Dis
    }
    #[doc = "Out Endpoint interrupt enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ep5outIntEn::En
    }
}
#[doc = "Field `EP5OutIntEn` writer - Out Endpoint 5 Interrupt Enable."]
pub type Ep5outIntEnW<'a, REG> = crate::BitWriter<'a, REG, Ep5outIntEn>;
impl<'a, REG> Ep5outIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Out Endpoint interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ep5outIntEn::Dis)
    }
    #[doc = "Out Endpoint interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ep5outIntEn::En)
    }
}
#[doc = "Suspend Interrupt Status. Set when suspend signaling is detected on the bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suspend {
    #[doc = "0: Suspend interrupt inactive."]
    SuspendInactive = 0,
    #[doc = "1: Suspend interrupt active."]
    SuspendActive = 1,
}
impl From<Suspend> for bool {
    #[inline(always)]
    fn from(variant: Suspend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Suspend` reader - Suspend Interrupt Status. Set when suspend signaling is detected on the bus."]
pub type SuspendR = crate::BitReader<Suspend>;
impl SuspendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Suspend {
        match self.bits {
            false => Suspend::SuspendInactive,
            true => Suspend::SuspendActive,
        }
    }
    #[doc = "Suspend interrupt inactive."]
    #[inline(always)]
    pub fn is_suspend_inactive(&self) -> bool {
        *self == Suspend::SuspendInactive
    }
    #[doc = "Suspend interrupt active."]
    #[inline(always)]
    pub fn is_suspend_active(&self) -> bool {
        *self == Suspend::SuspendActive
    }
}
#[doc = "Field `Suspend` writer - Suspend Interrupt Status. Set when suspend signaling is detected on the bus."]
pub type SuspendW<'a, REG> = crate::BitWriter<'a, REG, Suspend>;
impl<'a, REG> SuspendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Suspend interrupt inactive."]
    #[inline(always)]
    pub fn suspend_inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Suspend::SuspendInactive)
    }
    #[doc = "Suspend interrupt active."]
    #[inline(always)]
    pub fn suspend_active(self) -> &'a mut crate::W<REG> {
        self.variant(Suspend::SuspendActive)
    }
}
#[doc = "Resume Interrupt Status. Set when resume signaling is detected on the bus while the USB Controller is in Suspend mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resume {
    #[doc = "0: Resume interrupt inactive."]
    ResumeInactive = 0,
    #[doc = "1: Resume interrupt active."]
    ResumeActive = 1,
}
impl From<Resume> for bool {
    #[inline(always)]
    fn from(variant: Resume) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Resume` reader - Resume Interrupt Status. Set when resume signaling is detected on the bus while the USB Controller is in Suspend mode."]
pub type ResumeR = crate::BitReader<Resume>;
impl ResumeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resume {
        match self.bits {
            false => Resume::ResumeInactive,
            true => Resume::ResumeActive,
        }
    }
    #[doc = "Resume interrupt inactive."]
    #[inline(always)]
    pub fn is_resume_inactive(&self) -> bool {
        *self == Resume::ResumeInactive
    }
    #[doc = "Resume interrupt active."]
    #[inline(always)]
    pub fn is_resume_active(&self) -> bool {
        *self == Resume::ResumeActive
    }
}
#[doc = "Field `Resume` writer - Resume Interrupt Status. Set when resume signaling is detected on the bus while the USB Controller is in Suspend mode."]
pub type ResumeW<'a, REG> = crate::BitWriter<'a, REG, Resume>;
impl<'a, REG> ResumeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resume interrupt inactive."]
    #[inline(always)]
    pub fn resume_inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Resume::ResumeInactive)
    }
    #[doc = "Resume interrupt active."]
    #[inline(always)]
    pub fn resume_active(self) -> &'a mut crate::W<REG> {
        self.variant(Resume::ResumeActive)
    }
}
#[doc = "Reset Detect Interrupt Status. Set when reset signaling is detected on the bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reset {
    #[doc = "0: Reset Detect interrupt inactive."]
    ResetInactive = 0,
    #[doc = "1: Reset Detect interrupt active."]
    ResetActive = 1,
}
impl From<Reset> for bool {
    #[inline(always)]
    fn from(variant: Reset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Reset` reader - Reset Detect Interrupt Status. Set when reset signaling is detected on the bus."]
pub type ResetR = crate::BitReader<Reset>;
impl ResetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reset {
        match self.bits {
            false => Reset::ResetInactive,
            true => Reset::ResetActive,
        }
    }
    #[doc = "Reset Detect interrupt inactive."]
    #[inline(always)]
    pub fn is_reset_inactive(&self) -> bool {
        *self == Reset::ResetInactive
    }
    #[doc = "Reset Detect interrupt active."]
    #[inline(always)]
    pub fn is_reset_active(&self) -> bool {
        *self == Reset::ResetActive
    }
}
#[doc = "Field `Reset` writer - Reset Detect Interrupt Status. Set when reset signaling is detected on the bus."]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG, Reset>;
impl<'a, REG> ResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset Detect interrupt inactive."]
    #[inline(always)]
    pub fn reset_inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::ResetInactive)
    }
    #[doc = "Reset Detect interrupt active."]
    #[inline(always)]
    pub fn reset_active(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::ResetActive)
    }
}
#[doc = "Start of Frame Interrupt Status. Set at the start of frame.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sof {
    #[doc = "0: SOF interrupt inactive."]
    SofInactive = 0,
    #[doc = "1: SOF interrupt active."]
    SofActive = 1,
}
impl From<Sof> for bool {
    #[inline(always)]
    fn from(variant: Sof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF` reader - Start of Frame Interrupt Status. Set at the start of frame."]
pub type SofR = crate::BitReader<Sof>;
impl SofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sof {
        match self.bits {
            false => Sof::SofInactive,
            true => Sof::SofActive,
        }
    }
    #[doc = "SOF interrupt inactive."]
    #[inline(always)]
    pub fn is_sof_inactive(&self) -> bool {
        *self == Sof::SofInactive
    }
    #[doc = "SOF interrupt active."]
    #[inline(always)]
    pub fn is_sof_active(&self) -> bool {
        *self == Sof::SofActive
    }
}
#[doc = "Field `SOF` writer - Start of Frame Interrupt Status. Set at the start of frame."]
pub type SofW<'a, REG> = crate::BitWriter<'a, REG, Sof>;
impl<'a, REG> SofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SOF interrupt inactive."]
    #[inline(always)]
    pub fn sof_inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Sof::SofInactive)
    }
    #[doc = "SOF interrupt active."]
    #[inline(always)]
    pub fn sof_active(self) -> &'a mut crate::W<REG> {
        self.variant(Sof::SofActive)
    }
}
#[doc = "Suspend Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SuspendE {
    #[doc = "0: Suspend interrupt disable."]
    Dis = 0,
    #[doc = "1: Suspend interrupt enable."]
    En = 1,
}
impl From<SuspendE> for bool {
    #[inline(always)]
    fn from(variant: SuspendE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SuspendE` reader - Suspend Interrupt Enable."]
pub type SuspendER = crate::BitReader<SuspendE>;
impl SuspendER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SuspendE {
        match self.bits {
            false => SuspendE::Dis,
            true => SuspendE::En,
        }
    }
    #[doc = "Suspend interrupt disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SuspendE::Dis
    }
    #[doc = "Suspend interrupt enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SuspendE::En
    }
}
#[doc = "Field `SuspendE` writer - Suspend Interrupt Enable."]
pub type SuspendEW<'a, REG> = crate::BitWriter<'a, REG, SuspendE>;
impl<'a, REG> SuspendEW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Suspend interrupt disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(SuspendE::Dis)
    }
    #[doc = "Suspend interrupt enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(SuspendE::En)
    }
}
#[doc = "Resume Interrupt Enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResumeE {
    #[doc = "0: Resume interrupt disable."]
    Dis = 0,
    #[doc = "1: Resume interrupt enable."]
    En = 1,
}
impl From<ResumeE> for bool {
    #[inline(always)]
    fn from(variant: ResumeE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ResumeE` reader - Resume Interrupt Enable."]
pub type ResumeER = crate::BitReader<ResumeE>;
impl ResumeER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ResumeE {
        match self.bits {
            false => ResumeE::Dis,
            true => ResumeE::En,
        }
    }
    #[doc = "Resume interrupt disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ResumeE::Dis
    }
    #[doc = "Resume interrupt enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ResumeE::En
    }
}
#[doc = "Field `ResumeE` writer - Resume Interrupt Enable."]
pub type ResumeEW<'a, REG> = crate::BitWriter<'a, REG, ResumeE>;
impl<'a, REG> ResumeEW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resume interrupt disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(ResumeE::Dis)
    }
    #[doc = "Resume interrupt enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(ResumeE::En)
    }
}
#[doc = "Reset Detect Interrupt Enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResetE {
    #[doc = "0: Reset detect interrupt disable."]
    Dis = 0,
    #[doc = "1: Reset detect interrupt enable."]
    En = 1,
}
impl From<ResetE> for bool {
    #[inline(always)]
    fn from(variant: ResetE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ResetE` reader - Reset Detect Interrupt Enable."]
pub type ResetER = crate::BitReader<ResetE>;
impl ResetER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ResetE {
        match self.bits {
            false => ResetE::Dis,
            true => ResetE::En,
        }
    }
    #[doc = "Reset detect interrupt disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ResetE::Dis
    }
    #[doc = "Reset detect interrupt enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ResetE::En
    }
}
#[doc = "Field `ResetE` writer - Reset Detect Interrupt Enable."]
pub type ResetEW<'a, REG> = crate::BitWriter<'a, REG, ResetE>;
impl<'a, REG> ResetEW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset detect interrupt disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(ResetE::Dis)
    }
    #[doc = "Reset detect interrupt enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(ResetE::En)
    }
}
#[doc = "Start of Frame interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sofe {
    #[doc = "0: SOF interrupt disable."]
    Dis = 0,
    #[doc = "1: SOF interrupt enable."]
    En = 1,
}
impl From<Sofe> for bool {
    #[inline(always)]
    fn from(variant: Sofe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFE` reader - Start of Frame interrupt enable."]
pub type SofeR = crate::BitReader<Sofe>;
impl SofeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sofe {
        match self.bits {
            false => Sofe::Dis,
            true => Sofe::En,
        }
    }
    #[doc = "SOF interrupt disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Sofe::Dis
    }
    #[doc = "SOF interrupt enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Sofe::En
    }
}
#[doc = "Field `SOFE` writer - Start of Frame interrupt enable."]
pub type SofeW<'a, REG> = crate::BitWriter<'a, REG, Sofe>;
impl<'a, REG> SofeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SOF interrupt disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Sofe::Dis)
    }
    #[doc = "SOF interrupt enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Sofe::En)
    }
}
impl R {
    #[doc = "Bit 0 - Out Endpoint 0 Interrupt Enable."]
    #[inline(always)]
    pub fn ep0out_int_en(&self) -> Ep0outIntEnR {
        Ep0outIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Out Endpoint 1 Interrupt Enable."]
    #[inline(always)]
    pub fn ep1out_int_en(&self) -> Ep1outIntEnR {
        Ep1outIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Out Endpoint 2 Interrupt Enable."]
    #[inline(always)]
    pub fn ep2out_int_en(&self) -> Ep2outIntEnR {
        Ep2outIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Out Endpoint 3 Interrupt Enable."]
    #[inline(always)]
    pub fn ep3out_int_en(&self) -> Ep3outIntEnR {
        Ep3outIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Out Endpoint 4 Interrupt Enable."]
    #[inline(always)]
    pub fn ep4out_int_en(&self) -> Ep4outIntEnR {
        Ep4outIntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Out Endpoint 5 Interrupt Enable."]
    #[inline(always)]
    pub fn ep5out_int_en(&self) -> Ep5outIntEnR {
        Ep5outIntEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Suspend Interrupt Status. Set when suspend signaling is detected on the bus."]
    #[inline(always)]
    pub fn suspend(&self) -> SuspendR {
        SuspendR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Resume Interrupt Status. Set when resume signaling is detected on the bus while the USB Controller is in Suspend mode."]
    #[inline(always)]
    pub fn resume(&self) -> ResumeR {
        ResumeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset Detect Interrupt Status. Set when reset signaling is detected on the bus."]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Start of Frame Interrupt Status. Set at the start of frame."]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Suspend Interrupt Enable."]
    #[inline(always)]
    pub fn suspend_e(&self) -> SuspendER {
        SuspendER::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Resume Interrupt Enable."]
    #[inline(always)]
    pub fn resume_e(&self) -> ResumeER {
        ResumeER::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reset Detect Interrupt Enable."]
    #[inline(always)]
    pub fn reset_e(&self) -> ResetER {
        ResetER::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Start of Frame interrupt enable."]
    #[inline(always)]
    pub fn sofe(&self) -> SofeR {
        SofeR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Out Endpoint 0 Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ep0out_int_en(&mut self) -> Ep0outIntEnW<Cfg2Spec> {
        Ep0outIntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Out Endpoint 1 Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ep1out_int_en(&mut self) -> Ep1outIntEnW<Cfg2Spec> {
        Ep1outIntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Out Endpoint 2 Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ep2out_int_en(&mut self) -> Ep2outIntEnW<Cfg2Spec> {
        Ep2outIntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Out Endpoint 3 Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ep3out_int_en(&mut self) -> Ep3outIntEnW<Cfg2Spec> {
        Ep3outIntEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Out Endpoint 4 Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ep4out_int_en(&mut self) -> Ep4outIntEnW<Cfg2Spec> {
        Ep4outIntEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Out Endpoint 5 Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ep5out_int_en(&mut self) -> Ep5outIntEnW<Cfg2Spec> {
        Ep5outIntEnW::new(self, 5)
    }
    #[doc = "Bit 16 - Suspend Interrupt Status. Set when suspend signaling is detected on the bus."]
    #[inline(always)]
    #[must_use]
    pub fn suspend(&mut self) -> SuspendW<Cfg2Spec> {
        SuspendW::new(self, 16)
    }
    #[doc = "Bit 17 - Resume Interrupt Status. Set when resume signaling is detected on the bus while the USB Controller is in Suspend mode."]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> ResumeW<Cfg2Spec> {
        ResumeW::new(self, 17)
    }
    #[doc = "Bit 18 - Reset Detect Interrupt Status. Set when reset signaling is detected on the bus."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<Cfg2Spec> {
        ResetW::new(self, 18)
    }
    #[doc = "Bit 19 - Start of Frame Interrupt Status. Set at the start of frame."]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SofW<Cfg2Spec> {
        SofW::new(self, 19)
    }
    #[doc = "Bit 24 - Suspend Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn suspend_e(&mut self) -> SuspendEW<Cfg2Spec> {
        SuspendEW::new(self, 24)
    }
    #[doc = "Bit 25 - Resume Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn resume_e(&mut self) -> ResumeEW<Cfg2Spec> {
        ResumeEW::new(self, 25)
    }
    #[doc = "Bit 26 - Reset Detect Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn reset_e(&mut self) -> ResetEW<Cfg2Spec> {
        ResetEW::new(self, 26)
    }
    #[doc = "Bit 27 - Start of Frame interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn sofe(&mut self) -> SofeW<Cfg2Spec> {
        SofeW::new(self, 27)
    }
}
#[doc = "Provides interrupt enable and (currently active) status bits for each of the state interrupts, as well as the IN Endpoint and OUT Endpoint nterrupts. All active interrupts are cleared when this register is read. On reset, all IN and OUT Endpoint interrupts, in addition to Endpoint 0, are set to 1 while the remaining bits are set to 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg2Spec;
impl crate::RegisterSpec for Cfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for Cfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for Cfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG2 to value 0x0600_0000"]
impl crate::Resettable for Cfg2Spec {
    const RESET_VALUE: u32 = 0x0600_0000;
}
