#[doc = "Register `BOOTLOADER` reader"]
pub type R = crate::R<BootloaderSpec>;
#[doc = "Register `BOOTLOADER` writer"]
pub type W = crate::W<BootloaderSpec>;
#[doc = "Determines whether the bootloader code is visible at address 0x00000000 or not. Resets to 1, write 1 to clear.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bootloaderlow {
    #[doc = "1: Bootloader code at 0x00000000."]
    Addr0 = 1,
    #[doc = "0: Bootloader code is not visible at address"]
    Noaddr0 = 0,
}
impl From<Bootloaderlow> for bool {
    #[inline(always)]
    fn from(variant: Bootloaderlow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOTLOADERLOW` reader - Determines whether the bootloader code is visible at address 0x00000000 or not. Resets to 1, write 1 to clear."]
pub type BootloaderlowR = crate::BitReader<Bootloaderlow>;
impl BootloaderlowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bootloaderlow {
        match self.bits {
            true => Bootloaderlow::Addr0,
            false => Bootloaderlow::Noaddr0,
        }
    }
    #[doc = "Bootloader code at 0x00000000."]
    #[inline(always)]
    pub fn is_addr0(&self) -> bool {
        *self == Bootloaderlow::Addr0
    }
    #[doc = "Bootloader code is not visible at address"]
    #[inline(always)]
    pub fn is_noaddr0(&self) -> bool {
        *self == Bootloaderlow::Noaddr0
    }
}
#[doc = "Field `BOOTLOADERLOW` writer - Determines whether the bootloader code is visible at address 0x00000000 or not. Resets to 1, write 1 to clear."]
pub type BootloaderlowW<'a, REG> = crate::BitWriter<'a, REG, Bootloaderlow>;
impl<'a, REG> BootloaderlowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bootloader code at 0x00000000."]
    #[inline(always)]
    pub fn addr0(self) -> &'a mut crate::W<REG> {
        self.variant(Bootloaderlow::Addr0)
    }
    #[doc = "Bootloader code is not visible at address"]
    #[inline(always)]
    pub fn noaddr0(self) -> &'a mut crate::W<REG> {
        self.variant(Bootloaderlow::Noaddr0)
    }
}
#[doc = "Secure boot ROM lock. Always resets to 1, write 1 to clear. Enables system visibility to bootloader until set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbrlock {
    #[doc = "1: Enable the secure boot lock"]
    Lock = 1,
    #[doc = "0: Default value."]
    Default = 0,
}
impl From<Sbrlock> for bool {
    #[inline(always)]
    fn from(variant: Sbrlock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBRLOCK` reader - Secure boot ROM lock. Always resets to 1, write 1 to clear. Enables system visibility to bootloader until set."]
pub type SbrlockR = crate::BitReader<Sbrlock>;
impl SbrlockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbrlock {
        match self.bits {
            true => Sbrlock::Lock,
            false => Sbrlock::Default,
        }
    }
    #[doc = "Enable the secure boot lock"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == Sbrlock::Lock
    }
    #[doc = "Default value."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Sbrlock::Default
    }
}
#[doc = "Field `SBRLOCK` writer - Secure boot ROM lock. Always resets to 1, write 1 to clear. Enables system visibility to bootloader until set."]
pub type SbrlockW<'a, REG> = crate::BitWriter<'a, REG, Sbrlock>;
impl<'a, REG> SbrlockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the secure boot lock"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(Sbrlock::Lock)
    }
    #[doc = "Default value."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Sbrlock::Default)
    }
}
#[doc = "This field controls access to OEM keybank and restricts further application of NVM protections via the FLASHWPROTn and FLASHRPROTn registers. On read, 0 indicates locked, 1 indicates unlocked. Write 1 to clear the field (assert the lock). Default value of the hardware is 1 (unlocked), but the default value is also affected by INFOC_SECURITY->PLONEXIT. If PLONEXIT is set, PROTLOCK is asserted by SBL and the OEM application will see the value as 0 (locked).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Protlock {
    #[doc = "1: On write, this value asserts the lock. On read, see above description."]
    Lock = 1,
    #[doc = "0: On write, this value (writing 0) has no effect. On read, see above description."]
    Default = 0,
}
impl From<Protlock> for bool {
    #[inline(always)]
    fn from(variant: Protlock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROTLOCK` reader - This field controls access to OEM keybank and restricts further application of NVM protections via the FLASHWPROTn and FLASHRPROTn registers. On read, 0 indicates locked, 1 indicates unlocked. Write 1 to clear the field (assert the lock). Default value of the hardware is 1 (unlocked), but the default value is also affected by INFOC_SECURITY->PLONEXIT. If PLONEXIT is set, PROTLOCK is asserted by SBL and the OEM application will see the value as 0 (locked)."]
pub type ProtlockR = crate::BitReader<Protlock>;
impl ProtlockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Protlock {
        match self.bits {
            true => Protlock::Lock,
            false => Protlock::Default,
        }
    }
    #[doc = "On write, this value asserts the lock. On read, see above description."]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == Protlock::Lock
    }
    #[doc = "On write, this value (writing 0) has no effect. On read, see above description."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Protlock::Default
    }
}
#[doc = "Field `PROTLOCK` writer - This field controls access to OEM keybank and restricts further application of NVM protections via the FLASHWPROTn and FLASHRPROTn registers. On read, 0 indicates locked, 1 indicates unlocked. Write 1 to clear the field (assert the lock). Default value of the hardware is 1 (unlocked), but the default value is also affected by INFOC_SECURITY->PLONEXIT. If PLONEXIT is set, PROTLOCK is asserted by SBL and the OEM application will see the value as 0 (locked)."]
pub type ProtlockW<'a, REG> = crate::BitWriter<'a, REG, Protlock>;
impl<'a, REG> ProtlockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "On write, this value asserts the lock. On read, see above description."]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(Protlock::Lock)
    }
    #[doc = "On write, this value (writing 0) has no effect. On read, see above description."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Protlock::Default)
    }
}
#[doc = "Secure boot loader lock. Always resets to 1, write 1 to clear. Enables system visibility to bootloader until set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbllock {
    #[doc = "1: Enable the secure boot lock"]
    Lock = 1,
    #[doc = "0: Disable the secure boot lock"]
    Unlock = 0,
}
impl From<Sbllock> for bool {
    #[inline(always)]
    fn from(variant: Sbllock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBLLOCK` reader - Secure boot loader lock. Always resets to 1, write 1 to clear. Enables system visibility to bootloader until set."]
pub type SbllockR = crate::BitReader<Sbllock>;
impl SbllockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbllock {
        match self.bits {
            true => Sbllock::Lock,
            false => Sbllock::Unlock,
        }
    }
    #[doc = "Enable the secure boot lock"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == Sbllock::Lock
    }
    #[doc = "Disable the secure boot lock"]
    #[inline(always)]
    pub fn is_unlock(&self) -> bool {
        *self == Sbllock::Unlock
    }
}
#[doc = "Field `SBLLOCK` writer - Secure boot loader lock. Always resets to 1, write 1 to clear. Enables system visibility to bootloader until set."]
pub type SbllockW<'a, REG> = crate::BitWriter<'a, REG, Sbllock>;
impl<'a, REG> SbllockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the secure boot lock"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(Sbllock::Lock)
    }
    #[doc = "Disable the secure boot lock"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Sbllock::Unlock)
    }
}
#[doc = "Indicates whether the secure boot feature is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Secbootfeature {
    #[doc = "0: Secure boot disabled"]
    Disabled = 0,
    #[doc = "1: Secure boot enabled"]
    Enabled = 1,
    #[doc = "2: Error in secure boot configuration"]
    Error = 2,
}
impl From<Secbootfeature> for u8 {
    #[inline(always)]
    fn from(variant: Secbootfeature) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Secbootfeature {
    type Ux = u8;
}
impl crate::IsEnum for Secbootfeature {}
#[doc = "Field `SECBOOTFEATURE` reader - Indicates whether the secure boot feature is enabled."]
pub type SecbootfeatureR = crate::FieldReader<Secbootfeature>;
impl SecbootfeatureR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Secbootfeature> {
        match self.bits {
            0 => Some(Secbootfeature::Disabled),
            1 => Some(Secbootfeature::Enabled),
            2 => Some(Secbootfeature::Error),
            _ => None,
        }
    }
    #[doc = "Secure boot disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Secbootfeature::Disabled
    }
    #[doc = "Secure boot enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Secbootfeature::Enabled
    }
    #[doc = "Error in secure boot configuration"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Secbootfeature::Error
    }
}
#[doc = "Field `SECBOOTFEATURE` writer - Indicates whether the secure boot feature is enabled."]
pub type SecbootfeatureW<'a, REG> = crate::FieldWriter<'a, REG, 2, Secbootfeature>;
impl<'a, REG> SecbootfeatureW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Secure boot disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Secbootfeature::Disabled)
    }
    #[doc = "Secure boot enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Secbootfeature::Enabled)
    }
    #[doc = "Error in secure boot configuration"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Secbootfeature::Error)
    }
}
#[doc = "Indicates whether the secure boot on cold reset is enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Secboot {
    #[doc = "0: Secure boot disabled"]
    Disabled = 0,
    #[doc = "1: Secure boot enabled"]
    Enabled = 1,
    #[doc = "2: Error in secure boot configuration"]
    Error = 2,
}
impl From<Secboot> for u8 {
    #[inline(always)]
    fn from(variant: Secboot) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Secboot {
    type Ux = u8;
}
impl crate::IsEnum for Secboot {}
#[doc = "Field `SECBOOT` reader - Indicates whether the secure boot on cold reset is enabled"]
pub type SecbootR = crate::FieldReader<Secboot>;
impl SecbootR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Secboot> {
        match self.bits {
            0 => Some(Secboot::Disabled),
            1 => Some(Secboot::Enabled),
            2 => Some(Secboot::Error),
            _ => None,
        }
    }
    #[doc = "Secure boot disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Secboot::Disabled
    }
    #[doc = "Secure boot enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Secboot::Enabled
    }
    #[doc = "Error in secure boot configuration"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Secboot::Error
    }
}
#[doc = "Field `SECBOOT` writer - Indicates whether the secure boot on cold reset is enabled"]
pub type SecbootW<'a, REG> = crate::FieldWriter<'a, REG, 2, Secboot>;
impl<'a, REG> SecbootW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Secure boot disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Secboot::Disabled)
    }
    #[doc = "Secure boot enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Secboot::Enabled)
    }
    #[doc = "Error in secure boot configuration"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Secboot::Error)
    }
}
#[doc = "Indicates whether the secure boot on warm reset is enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Secbootonrst {
    #[doc = "0: Secure boot disabled"]
    Disabled = 0,
    #[doc = "1: Secure boot enabled"]
    Enabled = 1,
    #[doc = "2: Error in secure boot configuration"]
    Error = 2,
}
impl From<Secbootonrst> for u8 {
    #[inline(always)]
    fn from(variant: Secbootonrst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Secbootonrst {
    type Ux = u8;
}
impl crate::IsEnum for Secbootonrst {}
#[doc = "Field `SECBOOTONRST` reader - Indicates whether the secure boot on warm reset is enabled"]
pub type SecbootonrstR = crate::FieldReader<Secbootonrst>;
impl SecbootonrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Secbootonrst> {
        match self.bits {
            0 => Some(Secbootonrst::Disabled),
            1 => Some(Secbootonrst::Enabled),
            2 => Some(Secbootonrst::Error),
            _ => None,
        }
    }
    #[doc = "Secure boot disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Secbootonrst::Disabled
    }
    #[doc = "Secure boot enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Secbootonrst::Enabled
    }
    #[doc = "Error in secure boot configuration"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Secbootonrst::Error
    }
}
#[doc = "Field `SECBOOTONRST` writer - Indicates whether the secure boot on warm reset is enabled"]
pub type SecbootonrstW<'a, REG> = crate::FieldWriter<'a, REG, 2, Secbootonrst>;
impl<'a, REG> SecbootonrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Secure boot disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Secbootonrst::Disabled)
    }
    #[doc = "Secure boot enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Secbootonrst::Enabled)
    }
    #[doc = "Error in secure boot configuration"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Secbootonrst::Error)
    }
}
impl R {
    #[doc = "Bit 0 - Determines whether the bootloader code is visible at address 0x00000000 or not. Resets to 1, write 1 to clear."]
    #[inline(always)]
    pub fn bootloaderlow(&self) -> BootloaderlowR {
        BootloaderlowR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Secure boot ROM lock. Always resets to 1, write 1 to clear. Enables system visibility to bootloader until set."]
    #[inline(always)]
    pub fn sbrlock(&self) -> SbrlockR {
        SbrlockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This field controls access to OEM keybank and restricts further application of NVM protections via the FLASHWPROTn and FLASHRPROTn registers. On read, 0 indicates locked, 1 indicates unlocked. Write 1 to clear the field (assert the lock). Default value of the hardware is 1 (unlocked), but the default value is also affected by INFOC_SECURITY->PLONEXIT. If PLONEXIT is set, PROTLOCK is asserted by SBL and the OEM application will see the value as 0 (locked)."]
    #[inline(always)]
    pub fn protlock(&self) -> ProtlockR {
        ProtlockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Secure boot loader lock. Always resets to 1, write 1 to clear. Enables system visibility to bootloader until set."]
    #[inline(always)]
    pub fn sbllock(&self) -> SbllockR {
        SbllockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 26:27 - Indicates whether the secure boot feature is enabled."]
    #[inline(always)]
    pub fn secbootfeature(&self) -> SecbootfeatureR {
        SecbootfeatureR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Indicates whether the secure boot on cold reset is enabled"]
    #[inline(always)]
    pub fn secboot(&self) -> SecbootR {
        SecbootR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Indicates whether the secure boot on warm reset is enabled"]
    #[inline(always)]
    pub fn secbootonrst(&self) -> SecbootonrstR {
        SecbootonrstR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Determines whether the bootloader code is visible at address 0x00000000 or not. Resets to 1, write 1 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn bootloaderlow(&mut self) -> BootloaderlowW<BootloaderSpec> {
        BootloaderlowW::new(self, 0)
    }
    #[doc = "Bit 1 - Secure boot ROM lock. Always resets to 1, write 1 to clear. Enables system visibility to bootloader until set."]
    #[inline(always)]
    #[must_use]
    pub fn sbrlock(&mut self) -> SbrlockW<BootloaderSpec> {
        SbrlockW::new(self, 1)
    }
    #[doc = "Bit 2 - This field controls access to OEM keybank and restricts further application of NVM protections via the FLASHWPROTn and FLASHRPROTn registers. On read, 0 indicates locked, 1 indicates unlocked. Write 1 to clear the field (assert the lock). Default value of the hardware is 1 (unlocked), but the default value is also affected by INFOC_SECURITY->PLONEXIT. If PLONEXIT is set, PROTLOCK is asserted by SBL and the OEM application will see the value as 0 (locked)."]
    #[inline(always)]
    #[must_use]
    pub fn protlock(&mut self) -> ProtlockW<BootloaderSpec> {
        ProtlockW::new(self, 2)
    }
    #[doc = "Bit 3 - Secure boot loader lock. Always resets to 1, write 1 to clear. Enables system visibility to bootloader until set."]
    #[inline(always)]
    #[must_use]
    pub fn sbllock(&mut self) -> SbllockW<BootloaderSpec> {
        SbllockW::new(self, 3)
    }
    #[doc = "Bits 26:27 - Indicates whether the secure boot feature is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn secbootfeature(&mut self) -> SecbootfeatureW<BootloaderSpec> {
        SecbootfeatureW::new(self, 26)
    }
    #[doc = "Bits 28:29 - Indicates whether the secure boot on cold reset is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn secboot(&mut self) -> SecbootW<BootloaderSpec> {
        SecbootW::new(self, 28)
    }
    #[doc = "Bits 30:31 - Indicates whether the secure boot on warm reset is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn secbootonrst(&mut self) -> SecbootonrstW<BootloaderSpec> {
        SecbootonrstW::new(self, 30)
    }
}
#[doc = "Bootloader and secure boot functions\n\nYou can [`read`](crate::Reg::read) this register and get [`bootloader::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootloader::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BootloaderSpec;
impl crate::RegisterSpec for BootloaderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootloader::R`](R) reader structure"]
impl crate::Readable for BootloaderSpec {}
#[doc = "`write(|w| ..)` method takes [`bootloader::W`](W) writer structure"]
impl crate::Writable for BootloaderSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOTLOADER to value 0x01"]
impl crate::Resettable for BootloaderSpec {
    const RESET_VALUE: u32 = 0x01;
}
