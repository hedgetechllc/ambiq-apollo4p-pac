#[doc = "Register `CFG3` reader"]
pub type R = crate::R<Cfg3Spec>;
#[doc = "Register `CFG3` writer"]
pub type W = crate::W<Cfg3Spec>;
#[doc = "Field `FRMNUM` reader - Frame Number. Read-only field containing the last received frame number in bits 10:0, 15:11 read 0."]
pub type FrmnumR = crate::FieldReader<u16>;
#[doc = "Field `FRMNUM` writer - Frame Number. Read-only field containing the last received frame number in bits 10:0, 15:11 read 0."]
pub type FrmnumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Index selected endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Endpoint {
    #[doc = "0: Endpoint 0 selected."]
    Endpoint0 = 0,
    #[doc = "1: Endpoint 1 selected."]
    Endpoint1 = 1,
    #[doc = "2: Endpoint 2 selected."]
    Endpoint2 = 2,
    #[doc = "3: Endpoint 3 selected."]
    Endpoint3 = 3,
    #[doc = "4: Endpoint 4 selected."]
    Endpoint4 = 4,
    #[doc = "5: Endpoint 5 selected."]
    Endpoint5 = 5,
}
impl From<Endpoint> for u8 {
    #[inline(always)]
    fn from(variant: Endpoint) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Endpoint {
    type Ux = u8;
}
impl crate::IsEnum for Endpoint {}
#[doc = "Field `ENDPOINT` reader - Index selected endpoint."]
pub type EndpointR = crate::FieldReader<Endpoint>;
impl EndpointR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Endpoint> {
        match self.bits {
            0 => Some(Endpoint::Endpoint0),
            1 => Some(Endpoint::Endpoint1),
            2 => Some(Endpoint::Endpoint2),
            3 => Some(Endpoint::Endpoint3),
            4 => Some(Endpoint::Endpoint4),
            5 => Some(Endpoint::Endpoint5),
            _ => None,
        }
    }
    #[doc = "Endpoint 0 selected."]
    #[inline(always)]
    pub fn is_endpoint0(&self) -> bool {
        *self == Endpoint::Endpoint0
    }
    #[doc = "Endpoint 1 selected."]
    #[inline(always)]
    pub fn is_endpoint1(&self) -> bool {
        *self == Endpoint::Endpoint1
    }
    #[doc = "Endpoint 2 selected."]
    #[inline(always)]
    pub fn is_endpoint2(&self) -> bool {
        *self == Endpoint::Endpoint2
    }
    #[doc = "Endpoint 3 selected."]
    #[inline(always)]
    pub fn is_endpoint3(&self) -> bool {
        *self == Endpoint::Endpoint3
    }
    #[doc = "Endpoint 4 selected."]
    #[inline(always)]
    pub fn is_endpoint4(&self) -> bool {
        *self == Endpoint::Endpoint4
    }
    #[doc = "Endpoint 5 selected."]
    #[inline(always)]
    pub fn is_endpoint5(&self) -> bool {
        *self == Endpoint::Endpoint5
    }
}
#[doc = "Field `ENDPOINT` writer - Index selected endpoint."]
pub type EndpointW<'a, REG> = crate::FieldWriter<'a, REG, 4, Endpoint>;
impl<'a, REG> EndpointW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Endpoint 0 selected."]
    #[inline(always)]
    pub fn endpoint0(self) -> &'a mut crate::W<REG> {
        self.variant(Endpoint::Endpoint0)
    }
    #[doc = "Endpoint 1 selected."]
    #[inline(always)]
    pub fn endpoint1(self) -> &'a mut crate::W<REG> {
        self.variant(Endpoint::Endpoint1)
    }
    #[doc = "Endpoint 2 selected."]
    #[inline(always)]
    pub fn endpoint2(self) -> &'a mut crate::W<REG> {
        self.variant(Endpoint::Endpoint2)
    }
    #[doc = "Endpoint 3 selected."]
    #[inline(always)]
    pub fn endpoint3(self) -> &'a mut crate::W<REG> {
        self.variant(Endpoint::Endpoint3)
    }
    #[doc = "Endpoint 4 selected."]
    #[inline(always)]
    pub fn endpoint4(self) -> &'a mut crate::W<REG> {
        self.variant(Endpoint::Endpoint4)
    }
    #[doc = "Endpoint 5 selected."]
    #[inline(always)]
    pub fn endpoint5(self) -> &'a mut crate::W<REG> {
        self.variant(Endpoint::Endpoint5)
    }
}
#[doc = "Test_SE0_NAK Test Mode. The CPU sets this bit to enter the Test_SE0_NAK test mode. In this mode, the USB Controller remains in high-speed mode and responds to any valid IN token with a NAK.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestSe0nak {
    #[doc = "0: Terminates Test_SE0_NAK Test Mode."]
    StopTestse0nak = 0,
    #[doc = "1: Initiates Test_SE0_NAK Test Mode."]
    StartTestse0nak = 1,
}
impl From<TestSe0nak> for bool {
    #[inline(always)]
    fn from(variant: TestSe0nak) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TestSE0NAK` reader - Test_SE0_NAK Test Mode. The CPU sets this bit to enter the Test_SE0_NAK test mode. In this mode, the USB Controller remains in high-speed mode and responds to any valid IN token with a NAK."]
pub type TestSe0nakR = crate::BitReader<TestSe0nak>;
impl TestSe0nakR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TestSe0nak {
        match self.bits {
            false => TestSe0nak::StopTestse0nak,
            true => TestSe0nak::StartTestse0nak,
        }
    }
    #[doc = "Terminates Test_SE0_NAK Test Mode."]
    #[inline(always)]
    pub fn is_stop_testse0nak(&self) -> bool {
        *self == TestSe0nak::StopTestse0nak
    }
    #[doc = "Initiates Test_SE0_NAK Test Mode."]
    #[inline(always)]
    pub fn is_start_testse0nak(&self) -> bool {
        *self == TestSe0nak::StartTestse0nak
    }
}
#[doc = "Field `TestSE0NAK` writer - Test_SE0_NAK Test Mode. The CPU sets this bit to enter the Test_SE0_NAK test mode. In this mode, the USB Controller remains in high-speed mode and responds to any valid IN token with a NAK."]
pub type TestSe0nakW<'a, REG> = crate::BitWriter<'a, REG, TestSe0nak>;
impl<'a, REG> TestSe0nakW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Terminates Test_SE0_NAK Test Mode."]
    #[inline(always)]
    pub fn stop_testse0nak(self) -> &'a mut crate::W<REG> {
        self.variant(TestSe0nak::StopTestse0nak)
    }
    #[doc = "Initiates Test_SE0_NAK Test Mode."]
    #[inline(always)]
    pub fn start_testse0nak(self) -> &'a mut crate::W<REG> {
        self.variant(TestSe0nak::StartTestse0nak)
    }
}
#[doc = "Test_J Test Mode. The CPU sets this bit to enter the Test_J test mode. In this mode, the USB Controller - in high-speed mode - transmits a continuous J on the bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestJ {
    #[doc = "0: Terminates Test_J Test Mode."]
    StopTestj = 0,
    #[doc = "1: Initiates Test_J Test Mode."]
    StartTestj = 1,
}
impl From<TestJ> for bool {
    #[inline(always)]
    fn from(variant: TestJ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TestJ` reader - Test_J Test Mode. The CPU sets this bit to enter the Test_J test mode. In this mode, the USB Controller - in high-speed mode - transmits a continuous J on the bus."]
pub type TestJR = crate::BitReader<TestJ>;
impl TestJR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TestJ {
        match self.bits {
            false => TestJ::StopTestj,
            true => TestJ::StartTestj,
        }
    }
    #[doc = "Terminates Test_J Test Mode."]
    #[inline(always)]
    pub fn is_stop_testj(&self) -> bool {
        *self == TestJ::StopTestj
    }
    #[doc = "Initiates Test_J Test Mode."]
    #[inline(always)]
    pub fn is_start_testj(&self) -> bool {
        *self == TestJ::StartTestj
    }
}
#[doc = "Field `TestJ` writer - Test_J Test Mode. The CPU sets this bit to enter the Test_J test mode. In this mode, the USB Controller - in high-speed mode - transmits a continuous J on the bus."]
pub type TestJW<'a, REG> = crate::BitWriter<'a, REG, TestJ>;
impl<'a, REG> TestJW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Terminates Test_J Test Mode."]
    #[inline(always)]
    pub fn stop_testj(self) -> &'a mut crate::W<REG> {
        self.variant(TestJ::StopTestj)
    }
    #[doc = "Initiates Test_J Test Mode."]
    #[inline(always)]
    pub fn start_testj(self) -> &'a mut crate::W<REG> {
        self.variant(TestJ::StartTestj)
    }
}
#[doc = "Test_K Test Mode. The CPU sets this bit to enter the Test_K test mode. In this mode, the USB Controller - in high-speed mode - transmits a continuous K on the bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestK {
    #[doc = "0: Terminates Test_K Test Mode."]
    StopTestk = 0,
    #[doc = "1: Initiates Test_K Test Mode."]
    StartTestk = 1,
}
impl From<TestK> for bool {
    #[inline(always)]
    fn from(variant: TestK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TestK` reader - Test_K Test Mode. The CPU sets this bit to enter the Test_K test mode. In this mode, the USB Controller - in high-speed mode - transmits a continuous K on the bus."]
pub type TestKR = crate::BitReader<TestK>;
impl TestKR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TestK {
        match self.bits {
            false => TestK::StopTestk,
            true => TestK::StartTestk,
        }
    }
    #[doc = "Terminates Test_K Test Mode."]
    #[inline(always)]
    pub fn is_stop_testk(&self) -> bool {
        *self == TestK::StopTestk
    }
    #[doc = "Initiates Test_K Test Mode."]
    #[inline(always)]
    pub fn is_start_testk(&self) -> bool {
        *self == TestK::StartTestk
    }
}
#[doc = "Field `TestK` writer - Test_K Test Mode. The CPU sets this bit to enter the Test_K test mode. In this mode, the USB Controller - in high-speed mode - transmits a continuous K on the bus."]
pub type TestKW<'a, REG> = crate::BitWriter<'a, REG, TestK>;
impl<'a, REG> TestKW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Terminates Test_K Test Mode."]
    #[inline(always)]
    pub fn stop_testk(self) -> &'a mut crate::W<REG> {
        self.variant(TestK::StopTestk)
    }
    #[doc = "Initiates Test_K Test Mode."]
    #[inline(always)]
    pub fn start_testk(self) -> &'a mut crate::W<REG> {
        self.variant(TestK::StartTestk)
    }
}
#[doc = "Test Packet Test Mode. The CPU sets this bit to enter the Test_Packet test mode. In this mode, the USB Controller - in high-speed mode - repetitively transmits on the bus a 53-byte test packet. Note: The 53-byte test packet must be loaded into the Endpoint 0 FIFO before the test mode is entered.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestPacket {
    #[doc = "0: Terminates Test Packet Test Mode."]
    StopTptm = 0,
    #[doc = "1: Initiates Test Packet Test Mode."]
    StartTptm = 1,
}
impl From<TestPacket> for bool {
    #[inline(always)]
    fn from(variant: TestPacket) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TestPacket` reader - Test Packet Test Mode. The CPU sets this bit to enter the Test_Packet test mode. In this mode, the USB Controller - in high-speed mode - repetitively transmits on the bus a 53-byte test packet. Note: The 53-byte test packet must be loaded into the Endpoint 0 FIFO before the test mode is entered."]
pub type TestPacketR = crate::BitReader<TestPacket>;
impl TestPacketR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TestPacket {
        match self.bits {
            false => TestPacket::StopTptm,
            true => TestPacket::StartTptm,
        }
    }
    #[doc = "Terminates Test Packet Test Mode."]
    #[inline(always)]
    pub fn is_stop_tptm(&self) -> bool {
        *self == TestPacket::StopTptm
    }
    #[doc = "Initiates Test Packet Test Mode."]
    #[inline(always)]
    pub fn is_start_tptm(&self) -> bool {
        *self == TestPacket::StartTptm
    }
}
#[doc = "Field `TestPacket` writer - Test Packet Test Mode. The CPU sets this bit to enter the Test_Packet test mode. In this mode, the USB Controller - in high-speed mode - repetitively transmits on the bus a 53-byte test packet. Note: The 53-byte test packet must be loaded into the Endpoint 0 FIFO before the test mode is entered."]
pub type TestPacketW<'a, REG> = crate::BitWriter<'a, REG, TestPacket>;
impl<'a, REG> TestPacketW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Terminates Test Packet Test Mode."]
    #[inline(always)]
    pub fn stop_tptm(self) -> &'a mut crate::W<REG> {
        self.variant(TestPacket::StopTptm)
    }
    #[doc = "Initiates Test Packet Test Mode."]
    #[inline(always)]
    pub fn start_tptm(self) -> &'a mut crate::W<REG> {
        self.variant(TestPacket::StartTptm)
    }
}
#[doc = "Force High-speed Mode. The CPU sets this bit to force the USB Controller into High-speed mode when it receives a USB reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForceHs {
    #[doc = "0: Do not force HS mode upon USB reset."]
    HsNotForced = 0,
    #[doc = "1: Force HS mode upon USB reset."]
    HsForced = 1,
}
impl From<ForceHs> for bool {
    #[inline(always)]
    fn from(variant: ForceHs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ForceHS` reader - Force High-speed Mode. The CPU sets this bit to force the USB Controller into High-speed mode when it receives a USB reset."]
pub type ForceHsR = crate::BitReader<ForceHs>;
impl ForceHsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ForceHs {
        match self.bits {
            false => ForceHs::HsNotForced,
            true => ForceHs::HsForced,
        }
    }
    #[doc = "Do not force HS mode upon USB reset."]
    #[inline(always)]
    pub fn is_hs_not_forced(&self) -> bool {
        *self == ForceHs::HsNotForced
    }
    #[doc = "Force HS mode upon USB reset."]
    #[inline(always)]
    pub fn is_hs_forced(&self) -> bool {
        *self == ForceHs::HsForced
    }
}
#[doc = "Field `ForceHS` writer - Force High-speed Mode. The CPU sets this bit to force the USB Controller into High-speed mode when it receives a USB reset."]
pub type ForceHsW<'a, REG> = crate::BitWriter<'a, REG, ForceHs>;
impl<'a, REG> ForceHsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not force HS mode upon USB reset."]
    #[inline(always)]
    pub fn hs_not_forced(self) -> &'a mut crate::W<REG> {
        self.variant(ForceHs::HsNotForced)
    }
    #[doc = "Force HS mode upon USB reset."]
    #[inline(always)]
    pub fn hs_forced(self) -> &'a mut crate::W<REG> {
        self.variant(ForceHs::HsForced)
    }
}
#[doc = "Force Full-speed Mode. The CPU sets this bit to force the USB Controller into Full-speed mode when it receives a USB reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForceFs {
    #[doc = "0: Do not force FS mode upon USB reset."]
    FsNotForced = 0,
    #[doc = "1: Force FS mode upon USB reset."]
    FsForced = 1,
}
impl From<ForceFs> for bool {
    #[inline(always)]
    fn from(variant: ForceFs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ForceFS` reader - Force Full-speed Mode. The CPU sets this bit to force the USB Controller into Full-speed mode when it receives a USB reset."]
pub type ForceFsR = crate::BitReader<ForceFs>;
impl ForceFsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ForceFs {
        match self.bits {
            false => ForceFs::FsNotForced,
            true => ForceFs::FsForced,
        }
    }
    #[doc = "Do not force FS mode upon USB reset."]
    #[inline(always)]
    pub fn is_fs_not_forced(&self) -> bool {
        *self == ForceFs::FsNotForced
    }
    #[doc = "Force FS mode upon USB reset."]
    #[inline(always)]
    pub fn is_fs_forced(&self) -> bool {
        *self == ForceFs::FsForced
    }
}
#[doc = "Field `ForceFS` writer - Force Full-speed Mode. The CPU sets this bit to force the USB Controller into Full-speed mode when it receives a USB reset."]
pub type ForceFsW<'a, REG> = crate::BitWriter<'a, REG, ForceFs>;
impl<'a, REG> ForceFsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not force FS mode upon USB reset."]
    #[inline(always)]
    pub fn fs_not_forced(self) -> &'a mut crate::W<REG> {
        self.variant(ForceFs::FsNotForced)
    }
    #[doc = "Force FS mode upon USB reset."]
    #[inline(always)]
    pub fn fs_forced(self) -> &'a mut crate::W<REG> {
        self.variant(ForceFs::FsForced)
    }
}
impl R {
    #[doc = "Bits 0:15 - Frame Number. Read-only field containing the last received frame number in bits 10:0, 15:11 read 0."]
    #[inline(always)]
    pub fn frmnum(&self) -> FrmnumR {
        FrmnumR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Index selected endpoint."]
    #[inline(always)]
    pub fn endpoint(&self) -> EndpointR {
        EndpointR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Test_SE0_NAK Test Mode. The CPU sets this bit to enter the Test_SE0_NAK test mode. In this mode, the USB Controller remains in high-speed mode and responds to any valid IN token with a NAK."]
    #[inline(always)]
    pub fn test_se0nak(&self) -> TestSe0nakR {
        TestSe0nakR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Test_J Test Mode. The CPU sets this bit to enter the Test_J test mode. In this mode, the USB Controller - in high-speed mode - transmits a continuous J on the bus."]
    #[inline(always)]
    pub fn test_j(&self) -> TestJR {
        TestJR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Test_K Test Mode. The CPU sets this bit to enter the Test_K test mode. In this mode, the USB Controller - in high-speed mode - transmits a continuous K on the bus."]
    #[inline(always)]
    pub fn test_k(&self) -> TestKR {
        TestKR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Test Packet Test Mode. The CPU sets this bit to enter the Test_Packet test mode. In this mode, the USB Controller - in high-speed mode - repetitively transmits on the bus a 53-byte test packet. Note: The 53-byte test packet must be loaded into the Endpoint 0 FIFO before the test mode is entered."]
    #[inline(always)]
    pub fn test_packet(&self) -> TestPacketR {
        TestPacketR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Force High-speed Mode. The CPU sets this bit to force the USB Controller into High-speed mode when it receives a USB reset."]
    #[inline(always)]
    pub fn force_hs(&self) -> ForceHsR {
        ForceHsR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Force Full-speed Mode. The CPU sets this bit to force the USB Controller into Full-speed mode when it receives a USB reset."]
    #[inline(always)]
    pub fn force_fs(&self) -> ForceFsR {
        ForceFsR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame Number. Read-only field containing the last received frame number in bits 10:0, 15:11 read 0."]
    #[inline(always)]
    #[must_use]
    pub fn frmnum(&mut self) -> FrmnumW<Cfg3Spec> {
        FrmnumW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Index selected endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn endpoint(&mut self) -> EndpointW<Cfg3Spec> {
        EndpointW::new(self, 16)
    }
    #[doc = "Bit 24 - Test_SE0_NAK Test Mode. The CPU sets this bit to enter the Test_SE0_NAK test mode. In this mode, the USB Controller remains in high-speed mode and responds to any valid IN token with a NAK."]
    #[inline(always)]
    #[must_use]
    pub fn test_se0nak(&mut self) -> TestSe0nakW<Cfg3Spec> {
        TestSe0nakW::new(self, 24)
    }
    #[doc = "Bit 25 - Test_J Test Mode. The CPU sets this bit to enter the Test_J test mode. In this mode, the USB Controller - in high-speed mode - transmits a continuous J on the bus."]
    #[inline(always)]
    #[must_use]
    pub fn test_j(&mut self) -> TestJW<Cfg3Spec> {
        TestJW::new(self, 25)
    }
    #[doc = "Bit 26 - Test_K Test Mode. The CPU sets this bit to enter the Test_K test mode. In this mode, the USB Controller - in high-speed mode - transmits a continuous K on the bus."]
    #[inline(always)]
    #[must_use]
    pub fn test_k(&mut self) -> TestKW<Cfg3Spec> {
        TestKW::new(self, 26)
    }
    #[doc = "Bit 27 - Test Packet Test Mode. The CPU sets this bit to enter the Test_Packet test mode. In this mode, the USB Controller - in high-speed mode - repetitively transmits on the bus a 53-byte test packet. Note: The 53-byte test packet must be loaded into the Endpoint 0 FIFO before the test mode is entered."]
    #[inline(always)]
    #[must_use]
    pub fn test_packet(&mut self) -> TestPacketW<Cfg3Spec> {
        TestPacketW::new(self, 27)
    }
    #[doc = "Bit 28 - Force High-speed Mode. The CPU sets this bit to force the USB Controller into High-speed mode when it receives a USB reset."]
    #[inline(always)]
    #[must_use]
    pub fn force_hs(&mut self) -> ForceHsW<Cfg3Spec> {
        ForceHsW::new(self, 28)
    }
    #[doc = "Bit 29 - Force Full-speed Mode. The CPU sets this bit to force the USB Controller into Full-speed mode when it receives a USB reset."]
    #[inline(always)]
    #[must_use]
    pub fn force_fs(&mut self) -> ForceFsW<Cfg3Spec> {
        ForceFsW::new(self, 29)
    }
}
#[doc = "Provides Test fields to put the USB Controller into one of four test modes described in the USB 2.0 specification. Only one of the Test fields should be set at any time. (Not used in normal operation.) Also includes an index field that determines which endpoint control,status registers are accessed via the IDXn register fields, and a Frame field that holds the last received frame number.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg3Spec;
impl crate::RegisterSpec for Cfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg3::R`](R) reader structure"]
impl crate::Readable for Cfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg3::W`](W) writer structure"]
impl crate::Writable for Cfg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG3 to value 0"]
impl crate::Resettable for Cfg3Spec {
    const RESET_VALUE: u32 = 0;
}
