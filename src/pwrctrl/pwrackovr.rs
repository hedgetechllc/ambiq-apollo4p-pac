#[doc = "Register `PWRACKOVR` reader"]
pub type R = crate::R<PwrackovrSpec>;
#[doc = "Register `PWRACKOVR` writer"]
pub type W = crate::W<PwrackovrSpec>;
#[doc = "Power switch acknowledgement override from Power switch to power control ST MC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrackoverrideadc {
    #[doc = "0: Hardware generates Power switch ack"]
    Default = 0,
    #[doc = "1: Software override or defeaure mode. Will bypass HW generated power ack"]
    Swovrride = 1,
}
impl From<Pwrackoverrideadc> for bool {
    #[inline(always)]
    fn from(variant: Pwrackoverrideadc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRACKOVERRIDEADC` reader - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverrideadcR = crate::BitReader<Pwrackoverrideadc>;
impl PwrackoverrideadcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrackoverrideadc {
        match self.bits {
            false => Pwrackoverrideadc::Default,
            true => Pwrackoverrideadc::Swovrride,
        }
    }
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Pwrackoverrideadc::Default
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn is_swovrride(&self) -> bool {
        *self == Pwrackoverrideadc::Swovrride
    }
}
#[doc = "Field `PWRACKOVERRIDEADC` writer - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverrideadcW<'a, REG> = crate::BitWriter<'a, REG, Pwrackoverrideadc>;
impl<'a, REG> PwrackoverrideadcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverrideadc::Default)
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn swovrride(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverrideadc::Swovrride)
    }
}
#[doc = "Power switch acknowledgement override from Power switch to power control ST MC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrackoverrideaud {
    #[doc = "0: Hardware generates Power switch ack"]
    Default = 0,
    #[doc = "1: Software override or defeaure mode. Will bypass HW generated power ack"]
    Swovrride = 1,
}
impl From<Pwrackoverrideaud> for bool {
    #[inline(always)]
    fn from(variant: Pwrackoverrideaud) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRACKOVERRIDEAUD` reader - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverrideaudR = crate::BitReader<Pwrackoverrideaud>;
impl PwrackoverrideaudR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrackoverrideaud {
        match self.bits {
            false => Pwrackoverrideaud::Default,
            true => Pwrackoverrideaud::Swovrride,
        }
    }
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Pwrackoverrideaud::Default
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn is_swovrride(&self) -> bool {
        *self == Pwrackoverrideaud::Swovrride
    }
}
#[doc = "Field `PWRACKOVERRIDEAUD` writer - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverrideaudW<'a, REG> = crate::BitWriter<'a, REG, Pwrackoverrideaud>;
impl<'a, REG> PwrackoverrideaudW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverrideaud::Default)
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn swovrride(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverrideaud::Swovrride)
    }
}
#[doc = "Power switch acknowledgement override from Power switch to power control ST MC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrackoverrideaudadc {
    #[doc = "0: Hardware generates Power switch ack"]
    Default = 0,
    #[doc = "1: Software override or defeaure mode. Will bypass HW generated power ack"]
    Swovrride = 1,
}
impl From<Pwrackoverrideaudadc> for bool {
    #[inline(always)]
    fn from(variant: Pwrackoverrideaudadc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRACKOVERRIDEAUDADC` reader - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverrideaudadcR = crate::BitReader<Pwrackoverrideaudadc>;
impl PwrackoverrideaudadcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrackoverrideaudadc {
        match self.bits {
            false => Pwrackoverrideaudadc::Default,
            true => Pwrackoverrideaudadc::Swovrride,
        }
    }
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Pwrackoverrideaudadc::Default
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn is_swovrride(&self) -> bool {
        *self == Pwrackoverrideaudadc::Swovrride
    }
}
#[doc = "Field `PWRACKOVERRIDEAUDADC` writer - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverrideaudadcW<'a, REG> = crate::BitWriter<'a, REG, Pwrackoverrideaudadc>;
impl<'a, REG> PwrackoverrideaudadcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverrideaudadc::Default)
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn swovrride(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverrideaudadc::Swovrride)
    }
}
#[doc = "Power switch acknowledgement override from Power switch to power control ST MC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrackoverridecrypto {
    #[doc = "0: Hardware generates Power switch ack"]
    Default = 0,
    #[doc = "1: Software override or defeaure mode. Will bypass HW generated power ack"]
    Swovrride = 1,
}
impl From<Pwrackoverridecrypto> for bool {
    #[inline(always)]
    fn from(variant: Pwrackoverridecrypto) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRACKOVERRIDECRYPTO` reader - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverridecryptoR = crate::BitReader<Pwrackoverridecrypto>;
impl PwrackoverridecryptoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrackoverridecrypto {
        match self.bits {
            false => Pwrackoverridecrypto::Default,
            true => Pwrackoverridecrypto::Swovrride,
        }
    }
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Pwrackoverridecrypto::Default
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn is_swovrride(&self) -> bool {
        *self == Pwrackoverridecrypto::Swovrride
    }
}
#[doc = "Field `PWRACKOVERRIDECRYPTO` writer - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverridecryptoW<'a, REG> = crate::BitWriter<'a, REG, Pwrackoverridecrypto>;
impl<'a, REG> PwrackoverridecryptoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverridecrypto::Default)
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn swovrride(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverridecrypto::Swovrride)
    }
}
#[doc = "Power switch acknowledgement override from Power switch to power control ST MC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrackoverridedbg {
    #[doc = "0: Hardware generates Power switch ack"]
    Default = 0,
    #[doc = "1: Software override or defeaure mode. Will bypass HW generated power ack"]
    Swovrride = 1,
}
impl From<Pwrackoverridedbg> for bool {
    #[inline(always)]
    fn from(variant: Pwrackoverridedbg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRACKOVERRIDEDBG` reader - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverridedbgR = crate::BitReader<Pwrackoverridedbg>;
impl PwrackoverridedbgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrackoverridedbg {
        match self.bits {
            false => Pwrackoverridedbg::Default,
            true => Pwrackoverridedbg::Swovrride,
        }
    }
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Pwrackoverridedbg::Default
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn is_swovrride(&self) -> bool {
        *self == Pwrackoverridedbg::Swovrride
    }
}
#[doc = "Field `PWRACKOVERRIDEDBG` writer - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverridedbgW<'a, REG> = crate::BitWriter<'a, REG, Pwrackoverridedbg>;
impl<'a, REG> PwrackoverridedbgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverridedbg::Default)
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn swovrride(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverridedbg::Swovrride)
    }
}
#[doc = "Power switch acknowledgement override from Power switch to power control ST MC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrackoverridedisp {
    #[doc = "0: Hardware generates Power switch ack"]
    Default = 0,
    #[doc = "1: Software override or defeaure mode. Will bypass HW generated power ack"]
    Swovrride = 1,
}
impl From<Pwrackoverridedisp> for bool {
    #[inline(always)]
    fn from(variant: Pwrackoverridedisp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRACKOVERRIDEDISP` reader - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverridedispR = crate::BitReader<Pwrackoverridedisp>;
impl PwrackoverridedispR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrackoverridedisp {
        match self.bits {
            false => Pwrackoverridedisp::Default,
            true => Pwrackoverridedisp::Swovrride,
        }
    }
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Pwrackoverridedisp::Default
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn is_swovrride(&self) -> bool {
        *self == Pwrackoverridedisp::Swovrride
    }
}
#[doc = "Field `PWRACKOVERRIDEDISP` writer - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverridedispW<'a, REG> = crate::BitWriter<'a, REG, Pwrackoverridedisp>;
impl<'a, REG> PwrackoverridedispW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverridedisp::Default)
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn swovrride(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverridedisp::Swovrride)
    }
}
#[doc = "Power switch acknowledgement override from Power switch to power control ST MC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrackoverridedispphy {
    #[doc = "0: Hardware generates Power switch ack"]
    Default = 0,
    #[doc = "1: Software override or defeaure mode. Will bypass HW generated power ack"]
    Swovrride = 1,
}
impl From<Pwrackoverridedispphy> for bool {
    #[inline(always)]
    fn from(variant: Pwrackoverridedispphy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRACKOVERRIDEDISPPHY` reader - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverridedispphyR = crate::BitReader<Pwrackoverridedispphy>;
impl PwrackoverridedispphyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrackoverridedispphy {
        match self.bits {
            false => Pwrackoverridedispphy::Default,
            true => Pwrackoverridedispphy::Swovrride,
        }
    }
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Pwrackoverridedispphy::Default
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn is_swovrride(&self) -> bool {
        *self == Pwrackoverridedispphy::Swovrride
    }
}
#[doc = "Field `PWRACKOVERRIDEDISPPHY` writer - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverridedispphyW<'a, REG> = crate::BitWriter<'a, REG, Pwrackoverridedispphy>;
impl<'a, REG> PwrackoverridedispphyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverridedispphy::Default)
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn swovrride(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverridedispphy::Swovrride)
    }
}
#[doc = "Power switch acknowledgement override from Power switch to power control ST MC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrackoverridegfx {
    #[doc = "0: Hardware generates Power switch ack"]
    Default = 0,
    #[doc = "1: Software override or defeaure mode. Will bypass HW generated power ack"]
    Swovrride = 1,
}
impl From<Pwrackoverridegfx> for bool {
    #[inline(always)]
    fn from(variant: Pwrackoverridegfx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRACKOVERRIDEGFX` reader - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverridegfxR = crate::BitReader<Pwrackoverridegfx>;
impl PwrackoverridegfxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrackoverridegfx {
        match self.bits {
            false => Pwrackoverridegfx::Default,
            true => Pwrackoverridegfx::Swovrride,
        }
    }
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Pwrackoverridegfx::Default
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn is_swovrride(&self) -> bool {
        *self == Pwrackoverridegfx::Swovrride
    }
}
#[doc = "Field `PWRACKOVERRIDEGFX` writer - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverridegfxW<'a, REG> = crate::BitWriter<'a, REG, Pwrackoverridegfx>;
impl<'a, REG> PwrackoverridegfxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverridegfx::Default)
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn swovrride(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverridegfx::Swovrride)
    }
}
#[doc = "Power switch acknowledgement override from Power switch to power control ST MC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrackoverridehcpa {
    #[doc = "0: Hardware generates Power switch ack"]
    Default = 0,
    #[doc = "1: Software override or defeaure mode. Will bypass HW generated power ack"]
    Swovrride = 1,
}
impl From<Pwrackoverridehcpa> for bool {
    #[inline(always)]
    fn from(variant: Pwrackoverridehcpa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRACKOVERRIDEHCPA` reader - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverridehcpaR = crate::BitReader<Pwrackoverridehcpa>;
impl PwrackoverridehcpaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrackoverridehcpa {
        match self.bits {
            false => Pwrackoverridehcpa::Default,
            true => Pwrackoverridehcpa::Swovrride,
        }
    }
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Pwrackoverridehcpa::Default
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn is_swovrride(&self) -> bool {
        *self == Pwrackoverridehcpa::Swovrride
    }
}
#[doc = "Field `PWRACKOVERRIDEHCPA` writer - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverridehcpaW<'a, REG> = crate::BitWriter<'a, REG, Pwrackoverridehcpa>;
impl<'a, REG> PwrackoverridehcpaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverridehcpa::Default)
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn swovrride(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverridehcpa::Swovrride)
    }
}
#[doc = "Power switch acknowledgement override from Power switch to power control ST MC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrackoverridehcpb {
    #[doc = "0: Hardware generates Power switch ack"]
    Default = 0,
    #[doc = "1: Software override or defeaure mode. Will bypass HW generated power ack"]
    Swovrride = 1,
}
impl From<Pwrackoverridehcpb> for bool {
    #[inline(always)]
    fn from(variant: Pwrackoverridehcpb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRACKOVERRIDEHCPB` reader - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverridehcpbR = crate::BitReader<Pwrackoverridehcpb>;
impl PwrackoverridehcpbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrackoverridehcpb {
        match self.bits {
            false => Pwrackoverridehcpb::Default,
            true => Pwrackoverridehcpb::Swovrride,
        }
    }
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Pwrackoverridehcpb::Default
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn is_swovrride(&self) -> bool {
        *self == Pwrackoverridehcpb::Swovrride
    }
}
#[doc = "Field `PWRACKOVERRIDEHCPB` writer - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverridehcpbW<'a, REG> = crate::BitWriter<'a, REG, Pwrackoverridehcpb>;
impl<'a, REG> PwrackoverridehcpbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverridehcpb::Default)
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn swovrride(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverridehcpb::Swovrride)
    }
}
#[doc = "Power switch acknowledgement override from Power switch to power control ST MC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrackoverridehcpc {
    #[doc = "0: Hardware generates Power switch ack"]
    Default = 0,
    #[doc = "1: Software override or defeaure mode. Will bypass HW generated power ack"]
    Swovrride = 1,
}
impl From<Pwrackoverridehcpc> for bool {
    #[inline(always)]
    fn from(variant: Pwrackoverridehcpc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRACKOVERRIDEHCPC` reader - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverridehcpcR = crate::BitReader<Pwrackoverridehcpc>;
impl PwrackoverridehcpcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrackoverridehcpc {
        match self.bits {
            false => Pwrackoverridehcpc::Default,
            true => Pwrackoverridehcpc::Swovrride,
        }
    }
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Pwrackoverridehcpc::Default
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn is_swovrride(&self) -> bool {
        *self == Pwrackoverridehcpc::Swovrride
    }
}
#[doc = "Field `PWRACKOVERRIDEHCPC` writer - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverridehcpcW<'a, REG> = crate::BitWriter<'a, REG, Pwrackoverridehcpc>;
impl<'a, REG> PwrackoverridehcpcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverridehcpc::Default)
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn swovrride(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverridehcpc::Swovrride)
    }
}
#[doc = "Power switch acknowledgement override from Power switch to power control ST MC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrackoverrideios {
    #[doc = "0: Hardware generates Power switch ack"]
    Default = 0,
    #[doc = "1: Software override or defeaure mode. Will bypass HW generated power ack"]
    Swovrride = 1,
}
impl From<Pwrackoverrideios> for bool {
    #[inline(always)]
    fn from(variant: Pwrackoverrideios) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRACKOVERRIDEIOS` reader - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverrideiosR = crate::BitReader<Pwrackoverrideios>;
impl PwrackoverrideiosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrackoverrideios {
        match self.bits {
            false => Pwrackoverrideios::Default,
            true => Pwrackoverrideios::Swovrride,
        }
    }
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Pwrackoverrideios::Default
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn is_swovrride(&self) -> bool {
        *self == Pwrackoverrideios::Swovrride
    }
}
#[doc = "Field `PWRACKOVERRIDEIOS` writer - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverrideiosW<'a, REG> = crate::BitWriter<'a, REG, Pwrackoverrideios>;
impl<'a, REG> PwrackoverrideiosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverrideios::Default)
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn swovrride(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverrideios::Swovrride)
    }
}
#[doc = "Power switch acknowledgement override from Power switch to power control ST MC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrackoverridemcul {
    #[doc = "0: Hardware generates Power switch ack"]
    Default = 0,
    #[doc = "1: Software override or defeaure mode. Will bypass HW generated power ack"]
    Swovrride = 1,
}
impl From<Pwrackoverridemcul> for bool {
    #[inline(always)]
    fn from(variant: Pwrackoverridemcul) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRACKOVERRIDEMCUL` reader - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverridemculR = crate::BitReader<Pwrackoverridemcul>;
impl PwrackoverridemculR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrackoverridemcul {
        match self.bits {
            false => Pwrackoverridemcul::Default,
            true => Pwrackoverridemcul::Swovrride,
        }
    }
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Pwrackoverridemcul::Default
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn is_swovrride(&self) -> bool {
        *self == Pwrackoverridemcul::Swovrride
    }
}
#[doc = "Field `PWRACKOVERRIDEMCUL` writer - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverridemculW<'a, REG> = crate::BitWriter<'a, REG, Pwrackoverridemcul>;
impl<'a, REG> PwrackoverridemculW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverridemcul::Default)
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn swovrride(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverridemcul::Swovrride)
    }
}
#[doc = "Power switch acknowledgement override from Power switch to power control ST MC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrackoverridemspi {
    #[doc = "0: Hardware generates Power switch ack"]
    Default = 0,
    #[doc = "1: Software override or defeaure mode. Will bypass HW generated power ack"]
    Swovrride = 1,
}
impl From<Pwrackoverridemspi> for bool {
    #[inline(always)]
    fn from(variant: Pwrackoverridemspi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRACKOVERRIDEMSPI` reader - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverridemspiR = crate::BitReader<Pwrackoverridemspi>;
impl PwrackoverridemspiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrackoverridemspi {
        match self.bits {
            false => Pwrackoverridemspi::Default,
            true => Pwrackoverridemspi::Swovrride,
        }
    }
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Pwrackoverridemspi::Default
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn is_swovrride(&self) -> bool {
        *self == Pwrackoverridemspi::Swovrride
    }
}
#[doc = "Field `PWRACKOVERRIDEMSPI` writer - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverridemspiW<'a, REG> = crate::BitWriter<'a, REG, Pwrackoverridemspi>;
impl<'a, REG> PwrackoverridemspiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverridemspi::Default)
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn swovrride(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverridemspi::Swovrride)
    }
}
#[doc = "Power switch acknowledgement override from Power switch to power control ST MC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrackoverridesdio {
    #[doc = "0: Hardware generates Power switch ack"]
    Default = 0,
    #[doc = "1: Software override or defeaure mode. Will bypass HW generated power ack"]
    Swovrride = 1,
}
impl From<Pwrackoverridesdio> for bool {
    #[inline(always)]
    fn from(variant: Pwrackoverridesdio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRACKOVERRIDESDIO` reader - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverridesdioR = crate::BitReader<Pwrackoverridesdio>;
impl PwrackoverridesdioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrackoverridesdio {
        match self.bits {
            false => Pwrackoverridesdio::Default,
            true => Pwrackoverridesdio::Swovrride,
        }
    }
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Pwrackoverridesdio::Default
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn is_swovrride(&self) -> bool {
        *self == Pwrackoverridesdio::Swovrride
    }
}
#[doc = "Field `PWRACKOVERRIDESDIO` writer - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverridesdioW<'a, REG> = crate::BitWriter<'a, REG, Pwrackoverridesdio>;
impl<'a, REG> PwrackoverridesdioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverridesdio::Default)
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn swovrride(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverridesdio::Swovrride)
    }
}
#[doc = "Power switch acknowledgement override from Power switch to power control ST MC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrackoverrideusb {
    #[doc = "0: Hardware generates Power switch ack"]
    Default = 0,
    #[doc = "1: Software override or defeaure mode. Will bypass HW generated power ack"]
    Swovrride = 1,
}
impl From<Pwrackoverrideusb> for bool {
    #[inline(always)]
    fn from(variant: Pwrackoverrideusb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRACKOVERRIDEUSB` reader - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverrideusbR = crate::BitReader<Pwrackoverrideusb>;
impl PwrackoverrideusbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrackoverrideusb {
        match self.bits {
            false => Pwrackoverrideusb::Default,
            true => Pwrackoverrideusb::Swovrride,
        }
    }
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Pwrackoverrideusb::Default
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn is_swovrride(&self) -> bool {
        *self == Pwrackoverrideusb::Swovrride
    }
}
#[doc = "Field `PWRACKOVERRIDEUSB` writer - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverrideusbW<'a, REG> = crate::BitWriter<'a, REG, Pwrackoverrideusb>;
impl<'a, REG> PwrackoverrideusbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverrideusb::Default)
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn swovrride(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverrideusb::Swovrride)
    }
}
#[doc = "Power switch acknowledgement override from Power switch to power control ST MC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrackoverrideusbphy {
    #[doc = "0: Hardware generates Power switch ack"]
    Default = 0,
    #[doc = "1: Software override or defeaure mode. Will bypass HW generated power ack"]
    Swovrride = 1,
}
impl From<Pwrackoverrideusbphy> for bool {
    #[inline(always)]
    fn from(variant: Pwrackoverrideusbphy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRACKOVERRIDEUSBPHY` reader - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverrideusbphyR = crate::BitReader<Pwrackoverrideusbphy>;
impl PwrackoverrideusbphyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrackoverrideusbphy {
        match self.bits {
            false => Pwrackoverrideusbphy::Default,
            true => Pwrackoverrideusbphy::Swovrride,
        }
    }
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Pwrackoverrideusbphy::Default
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn is_swovrride(&self) -> bool {
        *self == Pwrackoverrideusbphy::Swovrride
    }
}
#[doc = "Field `PWRACKOVERRIDEUSBPHY` writer - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverrideusbphyW<'a, REG> = crate::BitWriter<'a, REG, Pwrackoverrideusbphy>;
impl<'a, REG> PwrackoverrideusbphyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverrideusbphy::Default)
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn swovrride(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverrideusbphy::Swovrride)
    }
}
#[doc = "Power switch acknowledgement override from Power switch to power control ST MC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrackoverridedspa {
    #[doc = "0: Hardware generates Power switch ack"]
    Default = 0,
    #[doc = "1: Software override or defeaure mode. Will bypass HW generated power ack"]
    Swovrride = 1,
}
impl From<Pwrackoverridedspa> for bool {
    #[inline(always)]
    fn from(variant: Pwrackoverridedspa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRACKOVERRIDEDSPA` reader - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverridedspaR = crate::BitReader<Pwrackoverridedspa>;
impl PwrackoverridedspaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrackoverridedspa {
        match self.bits {
            false => Pwrackoverridedspa::Default,
            true => Pwrackoverridedspa::Swovrride,
        }
    }
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Pwrackoverridedspa::Default
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn is_swovrride(&self) -> bool {
        *self == Pwrackoverridedspa::Swovrride
    }
}
#[doc = "Field `PWRACKOVERRIDEDSPA` writer - Power switch acknowledgement override from Power switch to power control ST MC"]
pub type PwrackoverridedspaW<'a, REG> = crate::BitWriter<'a, REG, Pwrackoverridedspa>;
impl<'a, REG> PwrackoverridedspaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware generates Power switch ack"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverridedspa::Default)
    }
    #[doc = "Software override or defeaure mode. Will bypass HW generated power ack"]
    #[inline(always)]
    pub fn swovrride(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrackoverridedspa::Swovrride)
    }
}
impl R {
    #[doc = "Bit 0 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    pub fn pwrackoverrideadc(&self) -> PwrackoverrideadcR {
        PwrackoverrideadcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    pub fn pwrackoverrideaud(&self) -> PwrackoverrideaudR {
        PwrackoverrideaudR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    pub fn pwrackoverrideaudadc(&self) -> PwrackoverrideaudadcR {
        PwrackoverrideaudadcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    pub fn pwrackoverridecrypto(&self) -> PwrackoverridecryptoR {
        PwrackoverridecryptoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    pub fn pwrackoverridedbg(&self) -> PwrackoverridedbgR {
        PwrackoverridedbgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    pub fn pwrackoverridedisp(&self) -> PwrackoverridedispR {
        PwrackoverridedispR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    pub fn pwrackoverridedispphy(&self) -> PwrackoverridedispphyR {
        PwrackoverridedispphyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    pub fn pwrackoverridegfx(&self) -> PwrackoverridegfxR {
        PwrackoverridegfxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    pub fn pwrackoverridehcpa(&self) -> PwrackoverridehcpaR {
        PwrackoverridehcpaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    pub fn pwrackoverridehcpb(&self) -> PwrackoverridehcpbR {
        PwrackoverridehcpbR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    pub fn pwrackoverridehcpc(&self) -> PwrackoverridehcpcR {
        PwrackoverridehcpcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    pub fn pwrackoverrideios(&self) -> PwrackoverrideiosR {
        PwrackoverrideiosR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    pub fn pwrackoverridemcul(&self) -> PwrackoverridemculR {
        PwrackoverridemculR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    pub fn pwrackoverridemspi(&self) -> PwrackoverridemspiR {
        PwrackoverridemspiR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    pub fn pwrackoverridesdio(&self) -> PwrackoverridesdioR {
        PwrackoverridesdioR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    pub fn pwrackoverrideusb(&self) -> PwrackoverrideusbR {
        PwrackoverrideusbR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    pub fn pwrackoverrideusbphy(&self) -> PwrackoverrideusbphyR {
        PwrackoverrideusbphyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    pub fn pwrackoverridedspa(&self) -> PwrackoverridedspaR {
        PwrackoverridedspaR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    #[must_use]
    pub fn pwrackoverrideadc(&mut self) -> PwrackoverrideadcW<PwrackovrSpec> {
        PwrackoverrideadcW::new(self, 0)
    }
    #[doc = "Bit 1 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    #[must_use]
    pub fn pwrackoverrideaud(&mut self) -> PwrackoverrideaudW<PwrackovrSpec> {
        PwrackoverrideaudW::new(self, 1)
    }
    #[doc = "Bit 2 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    #[must_use]
    pub fn pwrackoverrideaudadc(&mut self) -> PwrackoverrideaudadcW<PwrackovrSpec> {
        PwrackoverrideaudadcW::new(self, 2)
    }
    #[doc = "Bit 3 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    #[must_use]
    pub fn pwrackoverridecrypto(&mut self) -> PwrackoverridecryptoW<PwrackovrSpec> {
        PwrackoverridecryptoW::new(self, 3)
    }
    #[doc = "Bit 4 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    #[must_use]
    pub fn pwrackoverridedbg(&mut self) -> PwrackoverridedbgW<PwrackovrSpec> {
        PwrackoverridedbgW::new(self, 4)
    }
    #[doc = "Bit 5 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    #[must_use]
    pub fn pwrackoverridedisp(&mut self) -> PwrackoverridedispW<PwrackovrSpec> {
        PwrackoverridedispW::new(self, 5)
    }
    #[doc = "Bit 6 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    #[must_use]
    pub fn pwrackoverridedispphy(&mut self) -> PwrackoverridedispphyW<PwrackovrSpec> {
        PwrackoverridedispphyW::new(self, 6)
    }
    #[doc = "Bit 7 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    #[must_use]
    pub fn pwrackoverridegfx(&mut self) -> PwrackoverridegfxW<PwrackovrSpec> {
        PwrackoverridegfxW::new(self, 7)
    }
    #[doc = "Bit 8 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    #[must_use]
    pub fn pwrackoverridehcpa(&mut self) -> PwrackoverridehcpaW<PwrackovrSpec> {
        PwrackoverridehcpaW::new(self, 8)
    }
    #[doc = "Bit 9 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    #[must_use]
    pub fn pwrackoverridehcpb(&mut self) -> PwrackoverridehcpbW<PwrackovrSpec> {
        PwrackoverridehcpbW::new(self, 9)
    }
    #[doc = "Bit 10 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    #[must_use]
    pub fn pwrackoverridehcpc(&mut self) -> PwrackoverridehcpcW<PwrackovrSpec> {
        PwrackoverridehcpcW::new(self, 10)
    }
    #[doc = "Bit 11 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    #[must_use]
    pub fn pwrackoverrideios(&mut self) -> PwrackoverrideiosW<PwrackovrSpec> {
        PwrackoverrideiosW::new(self, 11)
    }
    #[doc = "Bit 12 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    #[must_use]
    pub fn pwrackoverridemcul(&mut self) -> PwrackoverridemculW<PwrackovrSpec> {
        PwrackoverridemculW::new(self, 12)
    }
    #[doc = "Bit 13 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    #[must_use]
    pub fn pwrackoverridemspi(&mut self) -> PwrackoverridemspiW<PwrackovrSpec> {
        PwrackoverridemspiW::new(self, 13)
    }
    #[doc = "Bit 14 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    #[must_use]
    pub fn pwrackoverridesdio(&mut self) -> PwrackoverridesdioW<PwrackovrSpec> {
        PwrackoverridesdioW::new(self, 14)
    }
    #[doc = "Bit 15 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    #[must_use]
    pub fn pwrackoverrideusb(&mut self) -> PwrackoverrideusbW<PwrackovrSpec> {
        PwrackoverrideusbW::new(self, 15)
    }
    #[doc = "Bit 16 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    #[must_use]
    pub fn pwrackoverrideusbphy(&mut self) -> PwrackoverrideusbphyW<PwrackovrSpec> {
        PwrackoverrideusbphyW::new(self, 16)
    }
    #[doc = "Bit 17 - Power switch acknowledgement override from Power switch to power control ST MC"]
    #[inline(always)]
    #[must_use]
    pub fn pwrackoverridedspa(&mut self) -> PwrackoverridedspaW<PwrackovrSpec> {
        PwrackoverridedspaW::new(self, 17)
    }
}
#[doc = "This register contains override bit fields for various power domain power switch acknowledge notification. As a part of power up sequnce, Power controller will look for power switch ack to advance power up sequence. It is possible for power controller to be a forever wait state in case of a unforeseen HW bug. This register defines a bit fileds to work around if needed in such a situation. The default behavior is to use HW power switch ack. Software can set it to override this feature for each power switch.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrackovr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrackovr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrackovrSpec;
impl crate::RegisterSpec for PwrackovrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrackovr::R`](R) reader structure"]
impl crate::Readable for PwrackovrSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrackovr::W`](W) writer structure"]
impl crate::Writable for PwrackovrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRACKOVR to value 0"]
impl crate::Resettable for PwrackovrSpec {
    const RESET_VALUE: u32 = 0;
}
