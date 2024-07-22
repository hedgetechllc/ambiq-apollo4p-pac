#[doc = "Register `CFG0` reader"]
pub type R = crate::R<Cfg0Spec>;
#[doc = "Register `CFG0` writer"]
pub type W = crate::W<Cfg0Spec>;
#[doc = "Field `FuncAddr` reader - The function address. This field should be written with the address value contained in the SET_ADDRESS standard device request, when it is received on Endpoint 0. The new address will not take effect immediately as the host will still be using the old address for the Status stage of the device request. The USB Controller will continue to use the old address for decoding packets until the device request has completed. The status of the device request can be determined by reading the Update bit. When a new address is written to this field, the Update bit will be automatically set. It will remain high until the device request has completed and will be cleared when the new address takes effect Note: While the firmware may write the new address to the FuncAddr field immediately when it is received, it is recommended to leave this operation to the Status phase of the operation in case the host aborts the command."]
pub type FuncAddrR = crate::FieldReader;
#[doc = "Field `FuncAddr` writer - The function address. This field should be written with the address value contained in the SET_ADDRESS standard device request, when it is received on Endpoint 0. The new address will not take effect immediately as the host will still be using the old address for the Status stage of the device request. The USB Controller will continue to use the old address for decoding packets until the device request has completed. The status of the device request can be determined by reading the Update bit. When a new address is written to this field, the Update bit will be automatically set. It will remain high until the device request has completed and will be cleared when the new address takes effect Note: While the firmware may write the new address to the FuncAddr field immediately when it is received, it is recommended to leave this operation to the Status phase of the operation in case the host aborts the command."]
pub type FuncAddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Function Address Update. Set when FuncAddr is written. Cleared when the new address takes effect (at the end of the current transfer).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Update {
    #[doc = "0: Indicates that the new address has taken effect."]
    NewAddrSet = 0,
    #[doc = "1: Indicates that a new function address has been written to the FuncAddr field."]
    NewAddrWritten = 1,
}
impl From<Update> for bool {
    #[inline(always)]
    fn from(variant: Update) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Update` reader - Function Address Update. Set when FuncAddr is written. Cleared when the new address takes effect (at the end of the current transfer)."]
pub type UpdateR = crate::BitReader<Update>;
impl UpdateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Update {
        match self.bits {
            false => Update::NewAddrSet,
            true => Update::NewAddrWritten,
        }
    }
    #[doc = "Indicates that the new address has taken effect."]
    #[inline(always)]
    pub fn is_new_addr_set(&self) -> bool {
        *self == Update::NewAddrSet
    }
    #[doc = "Indicates that a new function address has been written to the FuncAddr field."]
    #[inline(always)]
    pub fn is_new_addr_written(&self) -> bool {
        *self == Update::NewAddrWritten
    }
}
#[doc = "Field `Update` writer - Function Address Update. Set when FuncAddr is written. Cleared when the new address takes effect (at the end of the current transfer)."]
pub type UpdateW<'a, REG> = crate::BitWriter<'a, REG, Update>;
impl<'a, REG> UpdateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Indicates that the new address has taken effect."]
    #[inline(always)]
    pub fn new_addr_set(self) -> &'a mut crate::W<REG> {
        self.variant(Update::NewAddrSet)
    }
    #[doc = "Indicates that a new function address has been written to the FuncAddr field."]
    #[inline(always)]
    pub fn new_addr_written(self) -> &'a mut crate::W<REG> {
        self.variant(Update::NewAddrWritten)
    }
}
#[doc = "Set by the CPU to enable the SUSPENDM signal. The Enabl bit is set to enable the SUSPENDM signal to put the UTM (and any other hardware which uses the SUSPENDM signal) into Suspend mode. If this bit is not set, Suspend mode will be detected as normal but the SUSPENDM signal will remain high so that the UTM does not go into its low-power mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enabl {
    #[doc = "0: Clear to disable SUSPENDM signal - UTM does not go into its low-power mode."]
    DisableSuspendm = 0,
    #[doc = "1: Set to enable the SUSPENDM signal to put the UTM (and any other HW which uses the SUSPENDM signal) into Suspend mode."]
    EnableSuspendm = 1,
}
impl From<Enabl> for bool {
    #[inline(always)]
    fn from(variant: Enabl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Enabl` reader - Set by the CPU to enable the SUSPENDM signal. The Enabl bit is set to enable the SUSPENDM signal to put the UTM (and any other hardware which uses the SUSPENDM signal) into Suspend mode. If this bit is not set, Suspend mode will be detected as normal but the SUSPENDM signal will remain high so that the UTM does not go into its low-power mode."]
pub type EnablR = crate::BitReader<Enabl>;
impl EnablR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enabl {
        match self.bits {
            false => Enabl::DisableSuspendm,
            true => Enabl::EnableSuspendm,
        }
    }
    #[doc = "Clear to disable SUSPENDM signal - UTM does not go into its low-power mode."]
    #[inline(always)]
    pub fn is_disable_suspendm(&self) -> bool {
        *self == Enabl::DisableSuspendm
    }
    #[doc = "Set to enable the SUSPENDM signal to put the UTM (and any other HW which uses the SUSPENDM signal) into Suspend mode."]
    #[inline(always)]
    pub fn is_enable_suspendm(&self) -> bool {
        *self == Enabl::EnableSuspendm
    }
}
#[doc = "Field `Enabl` writer - Set by the CPU to enable the SUSPENDM signal. The Enabl bit is set to enable the SUSPENDM signal to put the UTM (and any other hardware which uses the SUSPENDM signal) into Suspend mode. If this bit is not set, Suspend mode will be detected as normal but the SUSPENDM signal will remain high so that the UTM does not go into its low-power mode."]
pub type EnablW<'a, REG> = crate::BitWriter<'a, REG, Enabl>;
impl<'a, REG> EnablW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear to disable SUSPENDM signal - UTM does not go into its low-power mode."]
    #[inline(always)]
    pub fn disable_suspendm(self) -> &'a mut crate::W<REG> {
        self.variant(Enabl::DisableSuspendm)
    }
    #[doc = "Set to enable the SUSPENDM signal to put the UTM (and any other HW which uses the SUSPENDM signal) into Suspend mode."]
    #[inline(always)]
    pub fn enable_suspendm(self) -> &'a mut crate::W<REG> {
        self.variant(Enabl::EnableSuspendm)
    }
}
#[doc = "Suspend Status. This read-only bit is set when Suspend mode is entered. It is cleared when the CPU reads the interrupt register, or sets the Resume bit of this register. The Suspen bit is set by the USB Controller when Suspend mode is entered. It will be cleared when the CFG2_Suspend field is read (as a result of receiving a Suspend interrupt). It will also be cleared if Suspend mode is left by setting the Resume bit to initiate a remote wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suspen {
    #[doc = "0: Indicates that Suspend Mode exited."]
    Resumed = 0,
    #[doc = "1: Indicates that Suspend Mode entered."]
    Suspended = 1,
}
impl From<Suspen> for bool {
    #[inline(always)]
    fn from(variant: Suspen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Suspen` reader - Suspend Status. This read-only bit is set when Suspend mode is entered. It is cleared when the CPU reads the interrupt register, or sets the Resume bit of this register. The Suspen bit is set by the USB Controller when Suspend mode is entered. It will be cleared when the CFG2_Suspend field is read (as a result of receiving a Suspend interrupt). It will also be cleared if Suspend mode is left by setting the Resume bit to initiate a remote wake-up."]
pub type SuspenR = crate::BitReader<Suspen>;
impl SuspenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Suspen {
        match self.bits {
            false => Suspen::Resumed,
            true => Suspen::Suspended,
        }
    }
    #[doc = "Indicates that Suspend Mode exited."]
    #[inline(always)]
    pub fn is_resumed(&self) -> bool {
        *self == Suspen::Resumed
    }
    #[doc = "Indicates that Suspend Mode entered."]
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == Suspen::Suspended
    }
}
#[doc = "Field `Suspen` writer - Suspend Status. This read-only bit is set when Suspend mode is entered. It is cleared when the CPU reads the interrupt register, or sets the Resume bit of this register. The Suspen bit is set by the USB Controller when Suspend mode is entered. It will be cleared when the CFG2_Suspend field is read (as a result of receiving a Suspend interrupt). It will also be cleared if Suspend mode is left by setting the Resume bit to initiate a remote wake-up."]
pub type SuspenW<'a, REG> = crate::BitWriter<'a, REG, Suspen>;
impl<'a, REG> SuspenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Indicates that Suspend Mode exited."]
    #[inline(always)]
    pub fn resumed(self) -> &'a mut crate::W<REG> {
        self.variant(Suspen::Resumed)
    }
    #[doc = "Indicates that Suspend Mode entered."]
    #[inline(always)]
    pub fn suspended(self) -> &'a mut crate::W<REG> {
        self.variant(Suspen::Suspended)
    }
}
#[doc = "Resume. Set should clear this bit after 10 ms (a maximum of 15 ms) to end Resume signaling. The Resume bit is used to force the USB Controller to generate Resume signaling on the USB to perform remote wake-up from Suspend mode. Once set high, it should be left high for approximately 10 ms (at least 1 ms and no more than 15 ms), then cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resume {
    #[doc = "0: Cleared automatically 10-15 ms after being manually set."]
    EndResume = 0,
    #[doc = "1: Set to force USB Controller to generate Resume signal on the USB to cause remote wake-up from Suspend mode."]
    Resume = 1,
}
impl From<Resume> for bool {
    #[inline(always)]
    fn from(variant: Resume) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Resume` reader - Resume. Set should clear this bit after 10 ms (a maximum of 15 ms) to end Resume signaling. The Resume bit is used to force the USB Controller to generate Resume signaling on the USB to perform remote wake-up from Suspend mode. Once set high, it should be left high for approximately 10 ms (at least 1 ms and no more than 15 ms), then cleared."]
pub type ResumeR = crate::BitReader<Resume>;
impl ResumeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resume {
        match self.bits {
            false => Resume::EndResume,
            true => Resume::Resume,
        }
    }
    #[doc = "Cleared automatically 10-15 ms after being manually set."]
    #[inline(always)]
    pub fn is_end_resume(&self) -> bool {
        *self == Resume::EndResume
    }
    #[doc = "Set to force USB Controller to generate Resume signal on the USB to cause remote wake-up from Suspend mode."]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == Resume::Resume
    }
}
#[doc = "Field `Resume` writer - Resume. Set should clear this bit after 10 ms (a maximum of 15 ms) to end Resume signaling. The Resume bit is used to force the USB Controller to generate Resume signaling on the USB to perform remote wake-up from Suspend mode. Once set high, it should be left high for approximately 10 ms (at least 1 ms and no more than 15 ms), then cleared."]
pub type ResumeW<'a, REG> = crate::BitWriter<'a, REG, Resume>;
impl<'a, REG> ResumeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cleared automatically 10-15 ms after being manually set."]
    #[inline(always)]
    pub fn end_resume(self) -> &'a mut crate::W<REG> {
        self.variant(Resume::EndResume)
    }
    #[doc = "Set to force USB Controller to generate Resume signal on the USB to cause remote wake-up from Suspend mode."]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(Resume::Resume)
    }
}
#[doc = "Reset Status. Cleared when either HS negotiation has completed successfully or after 2.1 ms of reset signaling if HS negotiation fails. The Reset bit can be used to determine when reset signaling is present on the USB. Set when Reset signaling is detected and remains high until the bus reverts to an idle state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reset {
    #[doc = "0: Indicates that HS negotiation has completed successfully, or 2.1 ms of reset signaling has elapsed."]
    NegResetComplete = 0,
    #[doc = "1: Indicates that Reset signaling is detected and remains high until the bus reverts to an idle state."]
    Resetting = 1,
}
impl From<Reset> for bool {
    #[inline(always)]
    fn from(variant: Reset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Reset` reader - Reset Status. Cleared when either HS negotiation has completed successfully or after 2.1 ms of reset signaling if HS negotiation fails. The Reset bit can be used to determine when reset signaling is present on the USB. Set when Reset signaling is detected and remains high until the bus reverts to an idle state."]
pub type ResetR = crate::BitReader<Reset>;
impl ResetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reset {
        match self.bits {
            false => Reset::NegResetComplete,
            true => Reset::Resetting,
        }
    }
    #[doc = "Indicates that HS negotiation has completed successfully, or 2.1 ms of reset signaling has elapsed."]
    #[inline(always)]
    pub fn is_neg_reset_complete(&self) -> bool {
        *self == Reset::NegResetComplete
    }
    #[doc = "Indicates that Reset signaling is detected and remains high until the bus reverts to an idle state."]
    #[inline(always)]
    pub fn is_resetting(&self) -> bool {
        *self == Reset::Resetting
    }
}
#[doc = "Field `Reset` writer - Reset Status. Cleared when either HS negotiation has completed successfully or after 2.1 ms of reset signaling if HS negotiation fails. The Reset bit can be used to determine when reset signaling is present on the USB. Set when Reset signaling is detected and remains high until the bus reverts to an idle state."]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG, Reset>;
impl<'a, REG> ResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Indicates that HS negotiation has completed successfully, or 2.1 ms of reset signaling has elapsed."]
    #[inline(always)]
    pub fn neg_reset_complete(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::NegResetComplete)
    }
    #[doc = "Indicates that Reset signaling is detected and remains high until the bus reverts to an idle state."]
    #[inline(always)]
    pub fn resetting(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::Resetting)
    }
}
#[doc = "This read-only bit is set when the USB Controller has successfully negotiated for High-speed mode. The HSMode bit can be used to determine whether the USB Controller is in High-speed mode or Full-speed mode. It will go high when the function has successfully negotiated for high-speed operation during a USB reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsmode {
    #[doc = "0: Indicates USB Controller is in Full-speed mode only."]
    FsMode = 0,
    #[doc = "1: Indicates USB Controller is in High-speed mode."]
    HsMode = 1,
}
impl From<Hsmode> for bool {
    #[inline(always)]
    fn from(variant: Hsmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSMode` reader - This read-only bit is set when the USB Controller has successfully negotiated for High-speed mode. The HSMode bit can be used to determine whether the USB Controller is in High-speed mode or Full-speed mode. It will go high when the function has successfully negotiated for high-speed operation during a USB reset."]
pub type HsmodeR = crate::BitReader<Hsmode>;
impl HsmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsmode {
        match self.bits {
            false => Hsmode::FsMode,
            true => Hsmode::HsMode,
        }
    }
    #[doc = "Indicates USB Controller is in Full-speed mode only."]
    #[inline(always)]
    pub fn is_fs_mode(&self) -> bool {
        *self == Hsmode::FsMode
    }
    #[doc = "Indicates USB Controller is in High-speed mode."]
    #[inline(always)]
    pub fn is_hs_mode(&self) -> bool {
        *self == Hsmode::HsMode
    }
}
#[doc = "Field `HSMode` writer - This read-only bit is set when the USB Controller has successfully negotiated for High-speed mode. The HSMode bit can be used to determine whether the USB Controller is in High-speed mode or Full-speed mode. It will go high when the function has successfully negotiated for high-speed operation during a USB reset."]
pub type HsmodeW<'a, REG> = crate::BitWriter<'a, REG, Hsmode>;
impl<'a, REG> HsmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Indicates USB Controller is in Full-speed mode only."]
    #[inline(always)]
    pub fn fs_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Hsmode::FsMode)
    }
    #[doc = "Indicates USB Controller is in High-speed mode."]
    #[inline(always)]
    pub fn hs_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Hsmode::HsMode)
    }
}
#[doc = "High-speed Enable. When set by the CPU, the USB Controller will negotiate for high-speed mode when the device is reset by the hub. If not set, the device will only operate in Full-speed mode. The HSEnab bit can be used to disable high-speed operation. Normally the USB Controller will automatically negotiate for high speed operation, when it is reset, by sending a 'chirp' to the hub. However if this bit is cleared then the USB Controller will not send any 'chirps' to the hub so the function will remain in Full-speed mode, even when connected to a high-speed-capable USB.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsenab {
    #[doc = "0: Clear to disable High-speed mode (Full-speed mode only)."]
    DisHs = 0,
    #[doc = "1: Set to enable High-speed mode."]
    EnHs = 1,
}
impl From<Hsenab> for bool {
    #[inline(always)]
    fn from(variant: Hsenab) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSEnab` reader - High-speed Enable. When set by the CPU, the USB Controller will negotiate for high-speed mode when the device is reset by the hub. If not set, the device will only operate in Full-speed mode. The HSEnab bit can be used to disable high-speed operation. Normally the USB Controller will automatically negotiate for high speed operation, when it is reset, by sending a 'chirp' to the hub. However if this bit is cleared then the USB Controller will not send any 'chirps' to the hub so the function will remain in Full-speed mode, even when connected to a high-speed-capable USB."]
pub type HsenabR = crate::BitReader<Hsenab>;
impl HsenabR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsenab {
        match self.bits {
            false => Hsenab::DisHs,
            true => Hsenab::EnHs,
        }
    }
    #[doc = "Clear to disable High-speed mode (Full-speed mode only)."]
    #[inline(always)]
    pub fn is_dis_hs(&self) -> bool {
        *self == Hsenab::DisHs
    }
    #[doc = "Set to enable High-speed mode."]
    #[inline(always)]
    pub fn is_en_hs(&self) -> bool {
        *self == Hsenab::EnHs
    }
}
#[doc = "Field `HSEnab` writer - High-speed Enable. When set by the CPU, the USB Controller will negotiate for high-speed mode when the device is reset by the hub. If not set, the device will only operate in Full-speed mode. The HSEnab bit can be used to disable high-speed operation. Normally the USB Controller will automatically negotiate for high speed operation, when it is reset, by sending a 'chirp' to the hub. However if this bit is cleared then the USB Controller will not send any 'chirps' to the hub so the function will remain in Full-speed mode, even when connected to a high-speed-capable USB."]
pub type HsenabW<'a, REG> = crate::BitWriter<'a, REG, Hsenab>;
impl<'a, REG> HsenabW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear to disable High-speed mode (Full-speed mode only)."]
    #[inline(always)]
    pub fn dis_hs(self) -> &'a mut crate::W<REG> {
        self.variant(Hsenab::DisHs)
    }
    #[doc = "Set to enable High-speed mode."]
    #[inline(always)]
    pub fn en_hs(self) -> &'a mut crate::W<REG> {
        self.variant(Hsenab::EnHs)
    }
}
#[doc = "Software-enabled Connection (SoftConn). When set to 1, the PHY is placed in its normal mode and the D+/D- lines of the USB bus are enabled. When bit is cleared, the PHY is put into non-driving mode and D+ and D- are tri-stated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Amspecific {
    #[doc = "0: Clear to disable/disconnect USB lines."]
    NotConnected = 0,
    #[doc = "1: Set to enable USB lines."]
    Connected = 1,
}
impl From<Amspecific> for bool {
    #[inline(always)]
    fn from(variant: Amspecific) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMSPECIFIC` reader - Software-enabled Connection (SoftConn). When set to 1, the PHY is placed in its normal mode and the D+/D- lines of the USB bus are enabled. When bit is cleared, the PHY is put into non-driving mode and D+ and D- are tri-stated."]
pub type AmspecificR = crate::BitReader<Amspecific>;
impl AmspecificR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Amspecific {
        match self.bits {
            false => Amspecific::NotConnected,
            true => Amspecific::Connected,
        }
    }
    #[doc = "Clear to disable/disconnect USB lines."]
    #[inline(always)]
    pub fn is_not_connected(&self) -> bool {
        *self == Amspecific::NotConnected
    }
    #[doc = "Set to enable USB lines."]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == Amspecific::Connected
    }
}
#[doc = "Field `AMSPECIFIC` writer - Software-enabled Connection (SoftConn). When set to 1, the PHY is placed in its normal mode and the D+/D- lines of the USB bus are enabled. When bit is cleared, the PHY is put into non-driving mode and D+ and D- are tri-stated."]
pub type AmspecificW<'a, REG> = crate::BitWriter<'a, REG, Amspecific>;
impl<'a, REG> AmspecificW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear to disable/disconnect USB lines."]
    #[inline(always)]
    pub fn not_connected(self) -> &'a mut crate::W<REG> {
        self.variant(Amspecific::NotConnected)
    }
    #[doc = "Set to enable USB lines."]
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(Amspecific::Connected)
    }
}
#[doc = "Isochronous Transfer Update. When set by the CPU, the USB Controller will wait for an SOF token from the time InPktRdy is set before sending the packet. If an IN token is received before an SOF token, then a zero length data packet will be sent. Note: This bit only affects endpoints performing Isochronous transfers. The ISOUpdate bit affects all IN Isochronous endpoints in the USB Controller. It is normally used as a method of ensuring 'clean' start-up of an IN Isochronous pipe.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isoupdate {
    #[doc = "0: Clear for USB Controller not to wait for SOF token before sending packet."]
    DontWait = 0,
    #[doc = "1: Set to have USB Controller wait for SOF token before sending packet."]
    Wait = 1,
}
impl From<Isoupdate> for bool {
    #[inline(always)]
    fn from(variant: Isoupdate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISOUpdate` reader - Isochronous Transfer Update. When set by the CPU, the USB Controller will wait for an SOF token from the time InPktRdy is set before sending the packet. If an IN token is received before an SOF token, then a zero length data packet will be sent. Note: This bit only affects endpoints performing Isochronous transfers. The ISOUpdate bit affects all IN Isochronous endpoints in the USB Controller. It is normally used as a method of ensuring 'clean' start-up of an IN Isochronous pipe."]
pub type IsoupdateR = crate::BitReader<Isoupdate>;
impl IsoupdateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isoupdate {
        match self.bits {
            false => Isoupdate::DontWait,
            true => Isoupdate::Wait,
        }
    }
    #[doc = "Clear for USB Controller not to wait for SOF token before sending packet."]
    #[inline(always)]
    pub fn is_dont_wait(&self) -> bool {
        *self == Isoupdate::DontWait
    }
    #[doc = "Set to have USB Controller wait for SOF token before sending packet."]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == Isoupdate::Wait
    }
}
#[doc = "Field `ISOUpdate` writer - Isochronous Transfer Update. When set by the CPU, the USB Controller will wait for an SOF token from the time InPktRdy is set before sending the packet. If an IN token is received before an SOF token, then a zero length data packet will be sent. Note: This bit only affects endpoints performing Isochronous transfers. The ISOUpdate bit affects all IN Isochronous endpoints in the USB Controller. It is normally used as a method of ensuring 'clean' start-up of an IN Isochronous pipe."]
pub type IsoupdateW<'a, REG> = crate::BitWriter<'a, REG, Isoupdate>;
impl<'a, REG> IsoupdateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear for USB Controller not to wait for SOF token before sending packet."]
    #[inline(always)]
    pub fn dont_wait(self) -> &'a mut crate::W<REG> {
        self.variant(Isoupdate::DontWait)
    }
    #[doc = "Set to have USB Controller wait for SOF token before sending packet."]
    #[inline(always)]
    pub fn wait(self) -> &'a mut crate::W<REG> {
        self.variant(Isoupdate::Wait)
    }
}
#[doc = "IN Endpoint 0 interrupt status. All interrupts are cleared when the register is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep0inIntStat {
    #[doc = "0: Interrupt inactive."]
    Inactive = 0,
    #[doc = "1: Interrupt active."]
    Active = 1,
}
impl From<Ep0inIntStat> for bool {
    #[inline(always)]
    fn from(variant: Ep0inIntStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0InIntStat` reader - IN Endpoint 0 interrupt status. All interrupts are cleared when the register is read."]
pub type Ep0inIntStatR = crate::BitReader<Ep0inIntStat>;
impl Ep0inIntStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep0inIntStat {
        match self.bits {
            false => Ep0inIntStat::Inactive,
            true => Ep0inIntStat::Active,
        }
    }
    #[doc = "Interrupt inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ep0inIntStat::Inactive
    }
    #[doc = "Interrupt active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ep0inIntStat::Active
    }
}
#[doc = "Field `EP0InIntStat` writer - IN Endpoint 0 interrupt status. All interrupts are cleared when the register is read."]
pub type Ep0inIntStatW<'a, REG> = crate::BitWriter<'a, REG, Ep0inIntStat>;
impl<'a, REG> Ep0inIntStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inactive."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Ep0inIntStat::Inactive)
    }
    #[doc = "Interrupt active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Ep0inIntStat::Active)
    }
}
#[doc = "IN Endpoint 1 interrupt status. All interrupts are cleared when the register is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep1inIntStat {
    #[doc = "0: Interrupt inactive."]
    Inactive = 0,
    #[doc = "1: Interrupt active."]
    Active = 1,
}
impl From<Ep1inIntStat> for bool {
    #[inline(always)]
    fn from(variant: Ep1inIntStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP1InIntStat` reader - IN Endpoint 1 interrupt status. All interrupts are cleared when the register is read."]
pub type Ep1inIntStatR = crate::BitReader<Ep1inIntStat>;
impl Ep1inIntStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep1inIntStat {
        match self.bits {
            false => Ep1inIntStat::Inactive,
            true => Ep1inIntStat::Active,
        }
    }
    #[doc = "Interrupt inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ep1inIntStat::Inactive
    }
    #[doc = "Interrupt active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ep1inIntStat::Active
    }
}
#[doc = "Field `EP1InIntStat` writer - IN Endpoint 1 interrupt status. All interrupts are cleared when the register is read."]
pub type Ep1inIntStatW<'a, REG> = crate::BitWriter<'a, REG, Ep1inIntStat>;
impl<'a, REG> Ep1inIntStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inactive."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Ep1inIntStat::Inactive)
    }
    #[doc = "Interrupt active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Ep1inIntStat::Active)
    }
}
#[doc = "IN Endpoint 2 interrupt status. All interrupts are cleared when the register is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep2inIntStat {
    #[doc = "0: Interrupt inactive."]
    Inactive = 0,
    #[doc = "1: Interrupt active."]
    Active = 1,
}
impl From<Ep2inIntStat> for bool {
    #[inline(always)]
    fn from(variant: Ep2inIntStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP2InIntStat` reader - IN Endpoint 2 interrupt status. All interrupts are cleared when the register is read."]
pub type Ep2inIntStatR = crate::BitReader<Ep2inIntStat>;
impl Ep2inIntStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep2inIntStat {
        match self.bits {
            false => Ep2inIntStat::Inactive,
            true => Ep2inIntStat::Active,
        }
    }
    #[doc = "Interrupt inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ep2inIntStat::Inactive
    }
    #[doc = "Interrupt active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ep2inIntStat::Active
    }
}
#[doc = "Field `EP2InIntStat` writer - IN Endpoint 2 interrupt status. All interrupts are cleared when the register is read."]
pub type Ep2inIntStatW<'a, REG> = crate::BitWriter<'a, REG, Ep2inIntStat>;
impl<'a, REG> Ep2inIntStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inactive."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Ep2inIntStat::Inactive)
    }
    #[doc = "Interrupt active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Ep2inIntStat::Active)
    }
}
#[doc = "IN Endpoint 3 interrupt status. All interrupts are cleared when the register is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep3inIntStat {
    #[doc = "0: Interrupt inactive."]
    Inactive = 0,
    #[doc = "1: Interrupt active."]
    Active = 1,
}
impl From<Ep3inIntStat> for bool {
    #[inline(always)]
    fn from(variant: Ep3inIntStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP3InIntStat` reader - IN Endpoint 3 interrupt status. All interrupts are cleared when the register is read."]
pub type Ep3inIntStatR = crate::BitReader<Ep3inIntStat>;
impl Ep3inIntStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep3inIntStat {
        match self.bits {
            false => Ep3inIntStat::Inactive,
            true => Ep3inIntStat::Active,
        }
    }
    #[doc = "Interrupt inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ep3inIntStat::Inactive
    }
    #[doc = "Interrupt active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ep3inIntStat::Active
    }
}
#[doc = "Field `EP3InIntStat` writer - IN Endpoint 3 interrupt status. All interrupts are cleared when the register is read."]
pub type Ep3inIntStatW<'a, REG> = crate::BitWriter<'a, REG, Ep3inIntStat>;
impl<'a, REG> Ep3inIntStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inactive."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Ep3inIntStat::Inactive)
    }
    #[doc = "Interrupt active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Ep3inIntStat::Active)
    }
}
#[doc = "IN Endpoint 4 interrupt status. All interrupts are cleared when the register is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep4inIntStat {
    #[doc = "0: Interrupt inactive."]
    Inactive = 0,
    #[doc = "1: Interrupt active."]
    Active = 1,
}
impl From<Ep4inIntStat> for bool {
    #[inline(always)]
    fn from(variant: Ep4inIntStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP4InIntStat` reader - IN Endpoint 4 interrupt status. All interrupts are cleared when the register is read."]
pub type Ep4inIntStatR = crate::BitReader<Ep4inIntStat>;
impl Ep4inIntStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep4inIntStat {
        match self.bits {
            false => Ep4inIntStat::Inactive,
            true => Ep4inIntStat::Active,
        }
    }
    #[doc = "Interrupt inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ep4inIntStat::Inactive
    }
    #[doc = "Interrupt active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ep4inIntStat::Active
    }
}
#[doc = "Field `EP4InIntStat` writer - IN Endpoint 4 interrupt status. All interrupts are cleared when the register is read."]
pub type Ep4inIntStatW<'a, REG> = crate::BitWriter<'a, REG, Ep4inIntStat>;
impl<'a, REG> Ep4inIntStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inactive."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Ep4inIntStat::Inactive)
    }
    #[doc = "Interrupt active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Ep4inIntStat::Active)
    }
}
#[doc = "IN Endpoint 5 interrupt status. All interrupts are cleared when the register is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep5inIntStat {
    #[doc = "0: Interrupt inactive."]
    Inactive = 0,
    #[doc = "1: Interrupt active."]
    Active = 1,
}
impl From<Ep5inIntStat> for bool {
    #[inline(always)]
    fn from(variant: Ep5inIntStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP5InIntStat` reader - IN Endpoint 5 interrupt status. All interrupts are cleared when the register is read."]
pub type Ep5inIntStatR = crate::BitReader<Ep5inIntStat>;
impl Ep5inIntStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep5inIntStat {
        match self.bits {
            false => Ep5inIntStat::Inactive,
            true => Ep5inIntStat::Active,
        }
    }
    #[doc = "Interrupt inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ep5inIntStat::Inactive
    }
    #[doc = "Interrupt active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ep5inIntStat::Active
    }
}
#[doc = "Field `EP5InIntStat` writer - IN Endpoint 5 interrupt status. All interrupts are cleared when the register is read."]
pub type Ep5inIntStatW<'a, REG> = crate::BitWriter<'a, REG, Ep5inIntStat>;
impl<'a, REG> Ep5inIntStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inactive."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Ep5inIntStat::Inactive)
    }
    #[doc = "Interrupt active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Ep5inIntStat::Active)
    }
}
impl R {
    #[doc = "Bits 0:6 - The function address. This field should be written with the address value contained in the SET_ADDRESS standard device request, when it is received on Endpoint 0. The new address will not take effect immediately as the host will still be using the old address for the Status stage of the device request. The USB Controller will continue to use the old address for decoding packets until the device request has completed. The status of the device request can be determined by reading the Update bit. When a new address is written to this field, the Update bit will be automatically set. It will remain high until the device request has completed and will be cleared when the new address takes effect Note: While the firmware may write the new address to the FuncAddr field immediately when it is received, it is recommended to leave this operation to the Status phase of the operation in case the host aborts the command."]
    #[inline(always)]
    pub fn func_addr(&self) -> FuncAddrR {
        FuncAddrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Function Address Update. Set when FuncAddr is written. Cleared when the new address takes effect (at the end of the current transfer)."]
    #[inline(always)]
    pub fn update(&self) -> UpdateR {
        UpdateR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set by the CPU to enable the SUSPENDM signal. The Enabl bit is set to enable the SUSPENDM signal to put the UTM (and any other hardware which uses the SUSPENDM signal) into Suspend mode. If this bit is not set, Suspend mode will be detected as normal but the SUSPENDM signal will remain high so that the UTM does not go into its low-power mode."]
    #[inline(always)]
    pub fn enabl(&self) -> EnablR {
        EnablR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Suspend Status. This read-only bit is set when Suspend mode is entered. It is cleared when the CPU reads the interrupt register, or sets the Resume bit of this register. The Suspen bit is set by the USB Controller when Suspend mode is entered. It will be cleared when the CFG2_Suspend field is read (as a result of receiving a Suspend interrupt). It will also be cleared if Suspend mode is left by setting the Resume bit to initiate a remote wake-up."]
    #[inline(always)]
    pub fn suspen(&self) -> SuspenR {
        SuspenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Resume. Set should clear this bit after 10 ms (a maximum of 15 ms) to end Resume signaling. The Resume bit is used to force the USB Controller to generate Resume signaling on the USB to perform remote wake-up from Suspend mode. Once set high, it should be left high for approximately 10 ms (at least 1 ms and no more than 15 ms), then cleared."]
    #[inline(always)]
    pub fn resume(&self) -> ResumeR {
        ResumeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reset Status. Cleared when either HS negotiation has completed successfully or after 2.1 ms of reset signaling if HS negotiation fails. The Reset bit can be used to determine when reset signaling is present on the USB. Set when Reset signaling is detected and remains high until the bus reverts to an idle state."]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This read-only bit is set when the USB Controller has successfully negotiated for High-speed mode. The HSMode bit can be used to determine whether the USB Controller is in High-speed mode or Full-speed mode. It will go high when the function has successfully negotiated for high-speed operation during a USB reset."]
    #[inline(always)]
    pub fn hsmode(&self) -> HsmodeR {
        HsmodeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - High-speed Enable. When set by the CPU, the USB Controller will negotiate for high-speed mode when the device is reset by the hub. If not set, the device will only operate in Full-speed mode. The HSEnab bit can be used to disable high-speed operation. Normally the USB Controller will automatically negotiate for high speed operation, when it is reset, by sending a 'chirp' to the hub. However if this bit is cleared then the USB Controller will not send any 'chirps' to the hub so the function will remain in Full-speed mode, even when connected to a high-speed-capable USB."]
    #[inline(always)]
    pub fn hsenab(&self) -> HsenabR {
        HsenabR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software-enabled Connection (SoftConn). When set to 1, the PHY is placed in its normal mode and the D+/D- lines of the USB bus are enabled. When bit is cleared, the PHY is put into non-driving mode and D+ and D- are tri-stated."]
    #[inline(always)]
    pub fn amspecific(&self) -> AmspecificR {
        AmspecificR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Isochronous Transfer Update. When set by the CPU, the USB Controller will wait for an SOF token from the time InPktRdy is set before sending the packet. If an IN token is received before an SOF token, then a zero length data packet will be sent. Note: This bit only affects endpoints performing Isochronous transfers. The ISOUpdate bit affects all IN Isochronous endpoints in the USB Controller. It is normally used as a method of ensuring 'clean' start-up of an IN Isochronous pipe."]
    #[inline(always)]
    pub fn isoupdate(&self) -> IsoupdateR {
        IsoupdateR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - IN Endpoint 0 interrupt status. All interrupts are cleared when the register is read."]
    #[inline(always)]
    pub fn ep0in_int_stat(&self) -> Ep0inIntStatR {
        Ep0inIntStatR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - IN Endpoint 1 interrupt status. All interrupts are cleared when the register is read."]
    #[inline(always)]
    pub fn ep1in_int_stat(&self) -> Ep1inIntStatR {
        Ep1inIntStatR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IN Endpoint 2 interrupt status. All interrupts are cleared when the register is read."]
    #[inline(always)]
    pub fn ep2in_int_stat(&self) -> Ep2inIntStatR {
        Ep2inIntStatR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - IN Endpoint 3 interrupt status. All interrupts are cleared when the register is read."]
    #[inline(always)]
    pub fn ep3in_int_stat(&self) -> Ep3inIntStatR {
        Ep3inIntStatR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - IN Endpoint 4 interrupt status. All interrupts are cleared when the register is read."]
    #[inline(always)]
    pub fn ep4in_int_stat(&self) -> Ep4inIntStatR {
        Ep4inIntStatR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - IN Endpoint 5 interrupt status. All interrupts are cleared when the register is read."]
    #[inline(always)]
    pub fn ep5in_int_stat(&self) -> Ep5inIntStatR {
        Ep5inIntStatR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - The function address. This field should be written with the address value contained in the SET_ADDRESS standard device request, when it is received on Endpoint 0. The new address will not take effect immediately as the host will still be using the old address for the Status stage of the device request. The USB Controller will continue to use the old address for decoding packets until the device request has completed. The status of the device request can be determined by reading the Update bit. When a new address is written to this field, the Update bit will be automatically set. It will remain high until the device request has completed and will be cleared when the new address takes effect Note: While the firmware may write the new address to the FuncAddr field immediately when it is received, it is recommended to leave this operation to the Status phase of the operation in case the host aborts the command."]
    #[inline(always)]
    #[must_use]
    pub fn func_addr(&mut self) -> FuncAddrW<Cfg0Spec> {
        FuncAddrW::new(self, 0)
    }
    #[doc = "Bit 7 - Function Address Update. Set when FuncAddr is written. Cleared when the new address takes effect (at the end of the current transfer)."]
    #[inline(always)]
    #[must_use]
    pub fn update(&mut self) -> UpdateW<Cfg0Spec> {
        UpdateW::new(self, 7)
    }
    #[doc = "Bit 8 - Set by the CPU to enable the SUSPENDM signal. The Enabl bit is set to enable the SUSPENDM signal to put the UTM (and any other hardware which uses the SUSPENDM signal) into Suspend mode. If this bit is not set, Suspend mode will be detected as normal but the SUSPENDM signal will remain high so that the UTM does not go into its low-power mode."]
    #[inline(always)]
    #[must_use]
    pub fn enabl(&mut self) -> EnablW<Cfg0Spec> {
        EnablW::new(self, 8)
    }
    #[doc = "Bit 9 - Suspend Status. This read-only bit is set when Suspend mode is entered. It is cleared when the CPU reads the interrupt register, or sets the Resume bit of this register. The Suspen bit is set by the USB Controller when Suspend mode is entered. It will be cleared when the CFG2_Suspend field is read (as a result of receiving a Suspend interrupt). It will also be cleared if Suspend mode is left by setting the Resume bit to initiate a remote wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn suspen(&mut self) -> SuspenW<Cfg0Spec> {
        SuspenW::new(self, 9)
    }
    #[doc = "Bit 10 - Resume. Set should clear this bit after 10 ms (a maximum of 15 ms) to end Resume signaling. The Resume bit is used to force the USB Controller to generate Resume signaling on the USB to perform remote wake-up from Suspend mode. Once set high, it should be left high for approximately 10 ms (at least 1 ms and no more than 15 ms), then cleared."]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> ResumeW<Cfg0Spec> {
        ResumeW::new(self, 10)
    }
    #[doc = "Bit 11 - Reset Status. Cleared when either HS negotiation has completed successfully or after 2.1 ms of reset signaling if HS negotiation fails. The Reset bit can be used to determine when reset signaling is present on the USB. Set when Reset signaling is detected and remains high until the bus reverts to an idle state."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<Cfg0Spec> {
        ResetW::new(self, 11)
    }
    #[doc = "Bit 12 - This read-only bit is set when the USB Controller has successfully negotiated for High-speed mode. The HSMode bit can be used to determine whether the USB Controller is in High-speed mode or Full-speed mode. It will go high when the function has successfully negotiated for high-speed operation during a USB reset."]
    #[inline(always)]
    #[must_use]
    pub fn hsmode(&mut self) -> HsmodeW<Cfg0Spec> {
        HsmodeW::new(self, 12)
    }
    #[doc = "Bit 13 - High-speed Enable. When set by the CPU, the USB Controller will negotiate for high-speed mode when the device is reset by the hub. If not set, the device will only operate in Full-speed mode. The HSEnab bit can be used to disable high-speed operation. Normally the USB Controller will automatically negotiate for high speed operation, when it is reset, by sending a 'chirp' to the hub. However if this bit is cleared then the USB Controller will not send any 'chirps' to the hub so the function will remain in Full-speed mode, even when connected to a high-speed-capable USB."]
    #[inline(always)]
    #[must_use]
    pub fn hsenab(&mut self) -> HsenabW<Cfg0Spec> {
        HsenabW::new(self, 13)
    }
    #[doc = "Bit 14 - Software-enabled Connection (SoftConn). When set to 1, the PHY is placed in its normal mode and the D+/D- lines of the USB bus are enabled. When bit is cleared, the PHY is put into non-driving mode and D+ and D- are tri-stated."]
    #[inline(always)]
    #[must_use]
    pub fn amspecific(&mut self) -> AmspecificW<Cfg0Spec> {
        AmspecificW::new(self, 14)
    }
    #[doc = "Bit 15 - Isochronous Transfer Update. When set by the CPU, the USB Controller will wait for an SOF token from the time InPktRdy is set before sending the packet. If an IN token is received before an SOF token, then a zero length data packet will be sent. Note: This bit only affects endpoints performing Isochronous transfers. The ISOUpdate bit affects all IN Isochronous endpoints in the USB Controller. It is normally used as a method of ensuring 'clean' start-up of an IN Isochronous pipe."]
    #[inline(always)]
    #[must_use]
    pub fn isoupdate(&mut self) -> IsoupdateW<Cfg0Spec> {
        IsoupdateW::new(self, 15)
    }
    #[doc = "Bit 16 - IN Endpoint 0 interrupt status. All interrupts are cleared when the register is read."]
    #[inline(always)]
    #[must_use]
    pub fn ep0in_int_stat(&mut self) -> Ep0inIntStatW<Cfg0Spec> {
        Ep0inIntStatW::new(self, 16)
    }
    #[doc = "Bit 17 - IN Endpoint 1 interrupt status. All interrupts are cleared when the register is read."]
    #[inline(always)]
    #[must_use]
    pub fn ep1in_int_stat(&mut self) -> Ep1inIntStatW<Cfg0Spec> {
        Ep1inIntStatW::new(self, 17)
    }
    #[doc = "Bit 18 - IN Endpoint 2 interrupt status. All interrupts are cleared when the register is read."]
    #[inline(always)]
    #[must_use]
    pub fn ep2in_int_stat(&mut self) -> Ep2inIntStatW<Cfg0Spec> {
        Ep2inIntStatW::new(self, 18)
    }
    #[doc = "Bit 19 - IN Endpoint 3 interrupt status. All interrupts are cleared when the register is read."]
    #[inline(always)]
    #[must_use]
    pub fn ep3in_int_stat(&mut self) -> Ep3inIntStatW<Cfg0Spec> {
        Ep3inIntStatW::new(self, 19)
    }
    #[doc = "Bit 20 - IN Endpoint 4 interrupt status. All interrupts are cleared when the register is read."]
    #[inline(always)]
    #[must_use]
    pub fn ep4in_int_stat(&mut self) -> Ep4inIntStatW<Cfg0Spec> {
        Ep4inIntStatW::new(self, 20)
    }
    #[doc = "Bit 21 - IN Endpoint 5 interrupt status. All interrupts are cleared when the register is read."]
    #[inline(always)]
    #[must_use]
    pub fn ep5in_int_stat(&mut self) -> Ep5inIntStatW<Cfg0Spec> {
        Ep5inIntStatW::new(self, 21)
    }
}
#[doc = "Function address, power management, interrupt status register for EP0 and IN Endpoints 1 to 5\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Spec;
impl crate::RegisterSpec for Cfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0::R`](R) reader structure"]
impl crate::Readable for Cfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0::W`](W) writer structure"]
impl crate::Writable for Cfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0 to value 0"]
impl crate::Resettable for Cfg0Spec {
    const RESET_VALUE: u32 = 0;
}
