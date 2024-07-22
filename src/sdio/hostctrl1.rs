#[doc = "Register `HOSTCTRL1` reader"]
pub type R = crate::R<Hostctrl1Spec>;
#[doc = "Register `HOSTCTRL1` writer"]
pub type W = crate::W<Hostctrl1Spec>;
#[doc = "This bit is used to caution the user not to remove the card while the SD card is being accessed. If the software is going to issue multiple SD commands, this bit can be set during all transactions. It is not necessary to change for each transaction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ledcontrol {
    #[doc = "1: LED on"]
    On = 1,
    #[doc = "0: LED off"]
    Off = 0,
}
impl From<Ledcontrol> for bool {
    #[inline(always)]
    fn from(variant: Ledcontrol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEDCONTROL` reader - This bit is used to caution the user not to remove the card while the SD card is being accessed. If the software is going to issue multiple SD commands, this bit can be set during all transactions. It is not necessary to change for each transaction."]
pub type LedcontrolR = crate::BitReader<Ledcontrol>;
impl LedcontrolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ledcontrol {
        match self.bits {
            true => Ledcontrol::On,
            false => Ledcontrol::Off,
        }
    }
    #[doc = "LED on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Ledcontrol::On
    }
    #[doc = "LED off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Ledcontrol::Off
    }
}
#[doc = "Field `LEDCONTROL` writer - This bit is used to caution the user not to remove the card while the SD card is being accessed. If the software is going to issue multiple SD commands, this bit can be set during all transactions. It is not necessary to change for each transaction."]
pub type LedcontrolW<'a, REG> = crate::BitWriter<'a, REG, Ledcontrol>;
impl<'a, REG> LedcontrolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LED on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Ledcontrol::On)
    }
    #[doc = "LED off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Ledcontrol::Off)
    }
}
#[doc = "(SD1 or SD4) This bit selects the data width of the HC. The HD shall select it to match the data width of the SD card.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datatransferwidth {
    #[doc = "1: 4 bit mode"]
    Sd4 = 1,
    #[doc = "0: 1 bit mode"]
    Sd1 = 0,
}
impl From<Datatransferwidth> for bool {
    #[inline(always)]
    fn from(variant: Datatransferwidth) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATATRANSFERWIDTH` reader - (SD1 or SD4) This bit selects the data width of the HC. The HD shall select it to match the data width of the SD card."]
pub type DatatransferwidthR = crate::BitReader<Datatransferwidth>;
impl DatatransferwidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datatransferwidth {
        match self.bits {
            true => Datatransferwidth::Sd4,
            false => Datatransferwidth::Sd1,
        }
    }
    #[doc = "4 bit mode"]
    #[inline(always)]
    pub fn is_sd4(&self) -> bool {
        *self == Datatransferwidth::Sd4
    }
    #[doc = "1 bit mode"]
    #[inline(always)]
    pub fn is_sd1(&self) -> bool {
        *self == Datatransferwidth::Sd1
    }
}
#[doc = "Field `DATATRANSFERWIDTH` writer - (SD1 or SD4) This bit selects the data width of the HC. The HD shall select it to match the data width of the SD card."]
pub type DatatransferwidthW<'a, REG> = crate::BitWriter<'a, REG, Datatransferwidth>;
impl<'a, REG> DatatransferwidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "4 bit mode"]
    #[inline(always)]
    pub fn sd4(self) -> &'a mut crate::W<REG> {
        self.variant(Datatransferwidth::Sd4)
    }
    #[doc = "1 bit mode"]
    #[inline(always)]
    pub fn sd1(self) -> &'a mut crate::W<REG> {
        self.variant(Datatransferwidth::Sd1)
    }
}
#[doc = "This bit is optional. Before setting this bit, the HD shall check the High Speed Support in the capabilities register. If this bit is set to 0 (default), the HC outputs CMD line and DAT lines at the falling edge of the SD clock (up to 25 MHz/ 20MHz for MMC). If this bit is set to 1, the HC outputs CMD line and DAT lines at the rising edge of the SD clock (up to 50 MHz for SD/52MHz for MMC)/ 208Mhz (for SD3.0) If Preset Value Enable in the Host Control 2 register is set to 1, Host Driver needs to reset SD Clock Enable before changing this field to avoid generating clock glitches. After setting this field, the Host Driver sets SD Clock Enable again\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hispeeden {
    #[doc = "1: High Speed Mode"]
    High = 1,
    #[doc = "0: Normal Speed Mode"]
    Normal = 0,
}
impl From<Hispeeden> for bool {
    #[inline(always)]
    fn from(variant: Hispeeden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HISPEEDEN` reader - This bit is optional. Before setting this bit, the HD shall check the High Speed Support in the capabilities register. If this bit is set to 0 (default), the HC outputs CMD line and DAT lines at the falling edge of the SD clock (up to 25 MHz/ 20MHz for MMC). If this bit is set to 1, the HC outputs CMD line and DAT lines at the rising edge of the SD clock (up to 50 MHz for SD/52MHz for MMC)/ 208Mhz (for SD3.0) If Preset Value Enable in the Host Control 2 register is set to 1, Host Driver needs to reset SD Clock Enable before changing this field to avoid generating clock glitches. After setting this field, the Host Driver sets SD Clock Enable again"]
pub type HispeedenR = crate::BitReader<Hispeeden>;
impl HispeedenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hispeeden {
        match self.bits {
            true => Hispeeden::High,
            false => Hispeeden::Normal,
        }
    }
    #[doc = "High Speed Mode"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Hispeeden::High
    }
    #[doc = "Normal Speed Mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Hispeeden::Normal
    }
}
#[doc = "Field `HISPEEDEN` writer - This bit is optional. Before setting this bit, the HD shall check the High Speed Support in the capabilities register. If this bit is set to 0 (default), the HC outputs CMD line and DAT lines at the falling edge of the SD clock (up to 25 MHz/ 20MHz for MMC). If this bit is set to 1, the HC outputs CMD line and DAT lines at the rising edge of the SD clock (up to 50 MHz for SD/52MHz for MMC)/ 208Mhz (for SD3.0) If Preset Value Enable in the Host Control 2 register is set to 1, Host Driver needs to reset SD Clock Enable before changing this field to avoid generating clock glitches. After setting this field, the Host Driver sets SD Clock Enable again"]
pub type HispeedenW<'a, REG> = crate::BitWriter<'a, REG, Hispeeden>;
impl<'a, REG> HispeedenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High Speed Mode"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Hispeeden::High)
    }
    #[doc = "Normal Speed Mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Hispeeden::Normal)
    }
}
#[doc = "One of supported DMA modes can be selected. The host driver shall check support of DMA modes by referring the Capabilities register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmaselect {
    #[doc = "0: SDMA is selected"]
    Sdma = 0,
    #[doc = "1: 32-bit Address ADMA1 is selected"]
    Adma132 = 1,
    #[doc = "2: 32-bit Address ADMA2 is selected"]
    Adma232 = 2,
    #[doc = "3: 64-bit Address ADMA2 is selected"]
    Adma264 = 3,
}
impl From<Dmaselect> for u8 {
    #[inline(always)]
    fn from(variant: Dmaselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmaselect {
    type Ux = u8;
}
impl crate::IsEnum for Dmaselect {}
#[doc = "Field `DMASELECT` reader - One of supported DMA modes can be selected. The host driver shall check support of DMA modes by referring the Capabilities register."]
pub type DmaselectR = crate::FieldReader<Dmaselect>;
impl DmaselectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaselect {
        match self.bits {
            0 => Dmaselect::Sdma,
            1 => Dmaselect::Adma132,
            2 => Dmaselect::Adma232,
            3 => Dmaselect::Adma264,
            _ => unreachable!(),
        }
    }
    #[doc = "SDMA is selected"]
    #[inline(always)]
    pub fn is_sdma(&self) -> bool {
        *self == Dmaselect::Sdma
    }
    #[doc = "32-bit Address ADMA1 is selected"]
    #[inline(always)]
    pub fn is_adma132(&self) -> bool {
        *self == Dmaselect::Adma132
    }
    #[doc = "32-bit Address ADMA2 is selected"]
    #[inline(always)]
    pub fn is_adma232(&self) -> bool {
        *self == Dmaselect::Adma232
    }
    #[doc = "64-bit Address ADMA2 is selected"]
    #[inline(always)]
    pub fn is_adma264(&self) -> bool {
        *self == Dmaselect::Adma264
    }
}
#[doc = "Field `DMASELECT` writer - One of supported DMA modes can be selected. The host driver shall check support of DMA modes by referring the Capabilities register."]
pub type DmaselectW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dmaselect, crate::Safe>;
impl<'a, REG> DmaselectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SDMA is selected"]
    #[inline(always)]
    pub fn sdma(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::Sdma)
    }
    #[doc = "32-bit Address ADMA1 is selected"]
    #[inline(always)]
    pub fn adma132(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::Adma132)
    }
    #[doc = "32-bit Address ADMA2 is selected"]
    #[inline(always)]
    pub fn adma232(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::Adma232)
    }
    #[doc = "64-bit Address ADMA2 is selected"]
    #[inline(always)]
    pub fn adma264(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::Adma264)
    }
}
#[doc = "This bit controls 8-bit bus width mode for embedded device. Support of this function is indicated in 8-bit Support for Embedded Device in the Capabilities register. If a device supports 8-bit bus mode, this bit may be set to 1. If this bit is 0, bus width is controlled by Data Transfer Width in the Host Control 1 register.This bit is not effective when multiple devices are installed on a bus slot (Slot Type is set to 10b in the Capabilities register). In this case, each device bus width is controlled by Bus Width Preset field in the Shared Bus register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xferwidth {
    #[doc = "1: 8-bit Bus Width"]
    _8bit = 1,
    #[doc = "0: Bus Width is selected by Data Transfer Width"]
    Xfer = 0,
}
impl From<Xferwidth> for bool {
    #[inline(always)]
    fn from(variant: Xferwidth) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XFERWIDTH` reader - This bit controls 8-bit bus width mode for embedded device. Support of this function is indicated in 8-bit Support for Embedded Device in the Capabilities register. If a device supports 8-bit bus mode, this bit may be set to 1. If this bit is 0, bus width is controlled by Data Transfer Width in the Host Control 1 register.This bit is not effective when multiple devices are installed on a bus slot (Slot Type is set to 10b in the Capabilities register). In this case, each device bus width is controlled by Bus Width Preset field in the Shared Bus register."]
pub type XferwidthR = crate::BitReader<Xferwidth>;
impl XferwidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xferwidth {
        match self.bits {
            true => Xferwidth::_8bit,
            false => Xferwidth::Xfer,
        }
    }
    #[doc = "8-bit Bus Width"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Xferwidth::_8bit
    }
    #[doc = "Bus Width is selected by Data Transfer Width"]
    #[inline(always)]
    pub fn is_xfer(&self) -> bool {
        *self == Xferwidth::Xfer
    }
}
#[doc = "Field `XFERWIDTH` writer - This bit controls 8-bit bus width mode for embedded device. Support of this function is indicated in 8-bit Support for Embedded Device in the Capabilities register. If a device supports 8-bit bus mode, this bit may be set to 1. If this bit is 0, bus width is controlled by Data Transfer Width in the Host Control 1 register.This bit is not effective when multiple devices are installed on a bus slot (Slot Type is set to 10b in the Capabilities register). In this case, each device bus width is controlled by Bus Width Preset field in the Shared Bus register."]
pub type XferwidthW<'a, REG> = crate::BitWriter<'a, REG, Xferwidth>;
impl<'a, REG> XferwidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "8-bit Bus Width"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Xferwidth::_8bit)
    }
    #[doc = "Bus Width is selected by Data Transfer Width"]
    #[inline(always)]
    pub fn xfer(self) -> &'a mut crate::W<REG> {
        self.variant(Xferwidth::Xfer)
    }
}
#[doc = "This bit is enabled while the Card Detect Signal Selection is set to 1 and it indicates card inserted or not. Generates (card ins or card removal) interrupt when the normal int sts enable bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Testlevel {
    #[doc = "1: Card Inserted"]
    Cardinserted = 1,
    #[doc = "0: No Card"]
    Nocard = 0,
}
impl From<Testlevel> for bool {
    #[inline(always)]
    fn from(variant: Testlevel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TESTLEVEL` reader - This bit is enabled while the Card Detect Signal Selection is set to 1 and it indicates card inserted or not. Generates (card ins or card removal) interrupt when the normal int sts enable bit is set."]
pub type TestlevelR = crate::BitReader<Testlevel>;
impl TestlevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Testlevel {
        match self.bits {
            true => Testlevel::Cardinserted,
            false => Testlevel::Nocard,
        }
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn is_cardinserted(&self) -> bool {
        *self == Testlevel::Cardinserted
    }
    #[doc = "No Card"]
    #[inline(always)]
    pub fn is_nocard(&self) -> bool {
        *self == Testlevel::Nocard
    }
}
#[doc = "Field `TESTLEVEL` writer - This bit is enabled while the Card Detect Signal Selection is set to 1 and it indicates card inserted or not. Generates (card ins or card removal) interrupt when the normal int sts enable bit is set."]
pub type TestlevelW<'a, REG> = crate::BitWriter<'a, REG, Testlevel>;
impl<'a, REG> TestlevelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn cardinserted(self) -> &'a mut crate::W<REG> {
        self.variant(Testlevel::Cardinserted)
    }
    #[doc = "No Card"]
    #[inline(always)]
    pub fn nocard(self) -> &'a mut crate::W<REG> {
        self.variant(Testlevel::Nocard)
    }
}
#[doc = "This bit selects source for card detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cardsrc {
    #[doc = "1: The card detect test level is selected"]
    Test = 1,
    #[doc = "0: SDCD is selected (for normal use)"]
    Sdcd = 0,
}
impl From<Cardsrc> for bool {
    #[inline(always)]
    fn from(variant: Cardsrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDSRC` reader - This bit selects source for card detection."]
pub type CardsrcR = crate::BitReader<Cardsrc>;
impl CardsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cardsrc {
        match self.bits {
            true => Cardsrc::Test,
            false => Cardsrc::Sdcd,
        }
    }
    #[doc = "The card detect test level is selected"]
    #[inline(always)]
    pub fn is_test(&self) -> bool {
        *self == Cardsrc::Test
    }
    #[doc = "SDCD is selected (for normal use)"]
    #[inline(always)]
    pub fn is_sdcd(&self) -> bool {
        *self == Cardsrc::Sdcd
    }
}
#[doc = "Field `CARDSRC` writer - This bit selects source for card detection."]
pub type CardsrcW<'a, REG> = crate::BitWriter<'a, REG, Cardsrc>;
impl<'a, REG> CardsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The card detect test level is selected"]
    #[inline(always)]
    pub fn test(self) -> &'a mut crate::W<REG> {
        self.variant(Cardsrc::Test)
    }
    #[doc = "SDCD is selected (for normal use)"]
    #[inline(always)]
    pub fn sdcd(self) -> &'a mut crate::W<REG> {
        self.variant(Cardsrc::Sdcd)
    }
}
#[doc = "Before setting this bit, the SD host driver shall set SD Bus Voltage Select. If the HC detects the No Card State, this bit shall be cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdbuspower {
    #[doc = "1: Power on"]
    Poweron = 1,
    #[doc = "0: Power off"]
    Poweroff = 0,
}
impl From<Sdbuspower> for bool {
    #[inline(always)]
    fn from(variant: Sdbuspower) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDBUSPOWER` reader - Before setting this bit, the SD host driver shall set SD Bus Voltage Select. If the HC detects the No Card State, this bit shall be cleared."]
pub type SdbuspowerR = crate::BitReader<Sdbuspower>;
impl SdbuspowerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdbuspower {
        match self.bits {
            true => Sdbuspower::Poweron,
            false => Sdbuspower::Poweroff,
        }
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn is_poweron(&self) -> bool {
        *self == Sdbuspower::Poweron
    }
    #[doc = "Power off"]
    #[inline(always)]
    pub fn is_poweroff(&self) -> bool {
        *self == Sdbuspower::Poweroff
    }
}
#[doc = "Field `SDBUSPOWER` writer - Before setting this bit, the SD host driver shall set SD Bus Voltage Select. If the HC detects the No Card State, this bit shall be cleared."]
pub type SdbuspowerW<'a, REG> = crate::BitWriter<'a, REG, Sdbuspower>;
impl<'a, REG> SdbuspowerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power on"]
    #[inline(always)]
    pub fn poweron(self) -> &'a mut crate::W<REG> {
        self.variant(Sdbuspower::Poweron)
    }
    #[doc = "Power off"]
    #[inline(always)]
    pub fn poweroff(self) -> &'a mut crate::W<REG> {
        self.variant(Sdbuspower::Poweroff)
    }
}
#[doc = "By setting these bits, the HD selects the voltage level for the SD card. Before setting this register, the HD shall check the voltage support bits in the capabilities register. If an unsupported voltage is selected, the Host System shall not supply SD bus voltage. All voltage select values not enumerated here are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Voltselect {
    #[doc = "7: 3.3 V(Typ.)"]
    _3_3v = 7,
    #[doc = "6: 3.0 V(Typ.)"]
    _3_0v = 6,
    #[doc = "5: 1.8 V(Typ.)"]
    _1_8v = 5,
}
impl From<Voltselect> for u8 {
    #[inline(always)]
    fn from(variant: Voltselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Voltselect {
    type Ux = u8;
}
impl crate::IsEnum for Voltselect {}
#[doc = "Field `VOLTSELECT` reader - By setting these bits, the HD selects the voltage level for the SD card. Before setting this register, the HD shall check the voltage support bits in the capabilities register. If an unsupported voltage is selected, the Host System shall not supply SD bus voltage. All voltage select values not enumerated here are reserved."]
pub type VoltselectR = crate::FieldReader<Voltselect>;
impl VoltselectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Voltselect> {
        match self.bits {
            7 => Some(Voltselect::_3_3v),
            6 => Some(Voltselect::_3_0v),
            5 => Some(Voltselect::_1_8v),
            _ => None,
        }
    }
    #[doc = "3.3 V(Typ.)"]
    #[inline(always)]
    pub fn is_3_3v(&self) -> bool {
        *self == Voltselect::_3_3v
    }
    #[doc = "3.0 V(Typ.)"]
    #[inline(always)]
    pub fn is_3_0v(&self) -> bool {
        *self == Voltselect::_3_0v
    }
    #[doc = "1.8 V(Typ.)"]
    #[inline(always)]
    pub fn is_1_8v(&self) -> bool {
        *self == Voltselect::_1_8v
    }
}
#[doc = "Field `VOLTSELECT` writer - By setting these bits, the HD selects the voltage level for the SD card. Before setting this register, the HD shall check the voltage support bits in the capabilities register. If an unsupported voltage is selected, the Host System shall not supply SD bus voltage. All voltage select values not enumerated here are reserved."]
pub type VoltselectW<'a, REG> = crate::FieldWriter<'a, REG, 3, Voltselect>;
impl<'a, REG> VoltselectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "3.3 V(Typ.)"]
    #[inline(always)]
    pub fn _3_3v(self) -> &'a mut crate::W<REG> {
        self.variant(Voltselect::_3_3v)
    }
    #[doc = "3.0 V(Typ.)"]
    #[inline(always)]
    pub fn _3_0v(self) -> &'a mut crate::W<REG> {
        self.variant(Voltselect::_3_0v)
    }
    #[doc = "1.8 V(Typ.)"]
    #[inline(always)]
    pub fn _1_8v(self) -> &'a mut crate::W<REG> {
        self.variant(Voltselect::_1_8v)
    }
}
#[doc = "Hardware reset signal is generated for eMMC card when this bit is set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hwreset {
    #[doc = "1: Drives the hardware reset pin as ZERO (Active LOW to eMMC card)"]
    Assert = 1,
    #[doc = "0: Deassert the hardware reset pin"]
    Deassert = 0,
}
impl From<Hwreset> for bool {
    #[inline(always)]
    fn from(variant: Hwreset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWRESET` reader - Hardware reset signal is generated for eMMC card when this bit is set"]
pub type HwresetR = crate::BitReader<Hwreset>;
impl HwresetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hwreset {
        match self.bits {
            true => Hwreset::Assert,
            false => Hwreset::Deassert,
        }
    }
    #[doc = "Drives the hardware reset pin as ZERO (Active LOW to eMMC card)"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == Hwreset::Assert
    }
    #[doc = "Deassert the hardware reset pin"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == Hwreset::Deassert
    }
}
#[doc = "Field `HWRESET` writer - Hardware reset signal is generated for eMMC card when this bit is set"]
pub type HwresetW<'a, REG> = crate::BitWriter<'a, REG, Hwreset>;
impl<'a, REG> HwresetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Drives the hardware reset pin as ZERO (Active LOW to eMMC card)"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut crate::W<REG> {
        self.variant(Hwreset::Assert)
    }
    #[doc = "Deassert the hardware reset pin"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut crate::W<REG> {
        self.variant(Hwreset::Deassert)
    }
}
#[doc = "This bit is used to stop executing a transaction at the next block gap for non- DMA,SDMA and ADMA transfers. Until the transfer complete is set to 1, indicating a transfer completion the HD shall leave this bit set to 1. Clearing both the Stop At Block Gap Request and Continue Request shall not cause the transaction to restart. Read Wait is used to stop the read transaction at the block gap. The HC shall honour Stop At Block Gap Request for write transfers, but for read transfers it requires that the SD card support Read Wait. Therefore the HD shall not set this bit during read transfers unless the SD card supports Read Wait and has set Read Wait Control to 1. In case of write transfers in which the HD writes data to the Buffer Data Port register, the HD shall set this bit after all block data is written. If this bit is set to 1, the HD shall not write data to Buffer data port register. This bit affects Read Transfer Active, Write Transfer Active, DAT line active and Command Inhibit (DAT) in the Present State register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopatblockgaprequest {
    #[doc = "1: Stop"]
    Stop = 1,
    #[doc = "0: Transfer"]
    Transfer = 0,
}
impl From<Stopatblockgaprequest> for bool {
    #[inline(always)]
    fn from(variant: Stopatblockgaprequest) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPATBLOCKGAPREQUEST` reader - This bit is used to stop executing a transaction at the next block gap for non- DMA,SDMA and ADMA transfers. Until the transfer complete is set to 1, indicating a transfer completion the HD shall leave this bit set to 1. Clearing both the Stop At Block Gap Request and Continue Request shall not cause the transaction to restart. Read Wait is used to stop the read transaction at the block gap. The HC shall honour Stop At Block Gap Request for write transfers, but for read transfers it requires that the SD card support Read Wait. Therefore the HD shall not set this bit during read transfers unless the SD card supports Read Wait and has set Read Wait Control to 1. In case of write transfers in which the HD writes data to the Buffer Data Port register, the HD shall set this bit after all block data is written. If this bit is set to 1, the HD shall not write data to Buffer data port register. This bit affects Read Transfer Active, Write Transfer Active, DAT line active and Command Inhibit (DAT) in the Present State register."]
pub type StopatblockgaprequestR = crate::BitReader<Stopatblockgaprequest>;
impl StopatblockgaprequestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopatblockgaprequest {
        match self.bits {
            true => Stopatblockgaprequest::Stop,
            false => Stopatblockgaprequest::Transfer,
        }
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Stopatblockgaprequest::Stop
    }
    #[doc = "Transfer"]
    #[inline(always)]
    pub fn is_transfer(&self) -> bool {
        *self == Stopatblockgaprequest::Transfer
    }
}
#[doc = "Field `STOPATBLOCKGAPREQUEST` writer - This bit is used to stop executing a transaction at the next block gap for non- DMA,SDMA and ADMA transfers. Until the transfer complete is set to 1, indicating a transfer completion the HD shall leave this bit set to 1. Clearing both the Stop At Block Gap Request and Continue Request shall not cause the transaction to restart. Read Wait is used to stop the read transaction at the block gap. The HC shall honour Stop At Block Gap Request for write transfers, but for read transfers it requires that the SD card support Read Wait. Therefore the HD shall not set this bit during read transfers unless the SD card supports Read Wait and has set Read Wait Control to 1. In case of write transfers in which the HD writes data to the Buffer Data Port register, the HD shall set this bit after all block data is written. If this bit is set to 1, the HD shall not write data to Buffer data port register. This bit affects Read Transfer Active, Write Transfer Active, DAT line active and Command Inhibit (DAT) in the Present State register."]
pub type StopatblockgaprequestW<'a, REG> = crate::BitWriter<'a, REG, Stopatblockgaprequest>;
impl<'a, REG> StopatblockgaprequestW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Stopatblockgaprequest::Stop)
    }
    #[doc = "Transfer"]
    #[inline(always)]
    pub fn transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Stopatblockgaprequest::Transfer)
    }
}
#[doc = "This bit is used to restart a transaction which was stopped using the Stop At Block Gap Request. To cancel stop at the block gap, set Stop At block Gap Request to 0 and set this bit to restart the transfer. The HC automatically clears this bit in either of the following cases: 1) In the case of a read transaction, the DAT Line Active changes from 0 to 1 as a read transaction restarts. 2) In the case of a write transaction, the Write transfer active changes from 0 to 1 as the write transaction restarts. Therefore it is not necessary for Host driver to set this bit to 0. If Stop At Block Gap Request is set to 1, any write to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Contreq {
    #[doc = "1: Restart"]
    Restart = 1,
    #[doc = "0: Ignored"]
    Ignored = 0,
}
impl From<Contreq> for bool {
    #[inline(always)]
    fn from(variant: Contreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONTREQ` reader - This bit is used to restart a transaction which was stopped using the Stop At Block Gap Request. To cancel stop at the block gap, set Stop At block Gap Request to 0 and set this bit to restart the transfer. The HC automatically clears this bit in either of the following cases: 1) In the case of a read transaction, the DAT Line Active changes from 0 to 1 as a read transaction restarts. 2) In the case of a write transaction, the Write transfer active changes from 0 to 1 as the write transaction restarts. Therefore it is not necessary for Host driver to set this bit to 0. If Stop At Block Gap Request is set to 1, any write to this bit is ignored."]
pub type ContreqR = crate::BitReader<Contreq>;
impl ContreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Contreq {
        match self.bits {
            true => Contreq::Restart,
            false => Contreq::Ignored,
        }
    }
    #[doc = "Restart"]
    #[inline(always)]
    pub fn is_restart(&self) -> bool {
        *self == Contreq::Restart
    }
    #[doc = "Ignored"]
    #[inline(always)]
    pub fn is_ignored(&self) -> bool {
        *self == Contreq::Ignored
    }
}
#[doc = "Field `CONTREQ` writer - This bit is used to restart a transaction which was stopped using the Stop At Block Gap Request. To cancel stop at the block gap, set Stop At block Gap Request to 0 and set this bit to restart the transfer. The HC automatically clears this bit in either of the following cases: 1) In the case of a read transaction, the DAT Line Active changes from 0 to 1 as a read transaction restarts. 2) In the case of a write transaction, the Write transfer active changes from 0 to 1 as the write transaction restarts. Therefore it is not necessary for Host driver to set this bit to 0. If Stop At Block Gap Request is set to 1, any write to this bit is ignored."]
pub type ContreqW<'a, REG> = crate::BitWriter<'a, REG, Contreq>;
impl<'a, REG> ContreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Restart"]
    #[inline(always)]
    pub fn restart(self) -> &'a mut crate::W<REG> {
        self.variant(Contreq::Restart)
    }
    #[doc = "Ignored"]
    #[inline(always)]
    pub fn ignored(self) -> &'a mut crate::W<REG> {
        self.variant(Contreq::Ignored)
    }
}
#[doc = "The read wait function is optional for SDIO cards. If the card supports read wait, set this bit to enable use of the read wait protocol to stop read data using DAT\\[2\\]
line. Otherwise the HC has to stop the SD clock to hold read data, which restricts commands generation. When the HD detects an SD card insertion, it shall set this bit according to the CCCR of the SDIO card. If the card does not support read wait, this bit shall never be set to 1 otherwise DAT line conflict may occur. If this bit is set to 0, Suspend / Resume cannot be supported\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Readwaitctrl {
    #[doc = "1: Enable Read Wait Control"]
    Enable = 1,
    #[doc = "0: Disable Read Wait Control"]
    Disable = 0,
}
impl From<Readwaitctrl> for bool {
    #[inline(always)]
    fn from(variant: Readwaitctrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READWAITCTRL` reader - The read wait function is optional for SDIO cards. If the card supports read wait, set this bit to enable use of the read wait protocol to stop read data using DAT\\[2\\]
line. Otherwise the HC has to stop the SD clock to hold read data, which restricts commands generation. When the HD detects an SD card insertion, it shall set this bit according to the CCCR of the SDIO card. If the card does not support read wait, this bit shall never be set to 1 otherwise DAT line conflict may occur. If this bit is set to 0, Suspend / Resume cannot be supported"]
pub type ReadwaitctrlR = crate::BitReader<Readwaitctrl>;
impl ReadwaitctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Readwaitctrl {
        match self.bits {
            true => Readwaitctrl::Enable,
            false => Readwaitctrl::Disable,
        }
    }
    #[doc = "Enable Read Wait Control"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Readwaitctrl::Enable
    }
    #[doc = "Disable Read Wait Control"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Readwaitctrl::Disable
    }
}
#[doc = "Field `READWAITCTRL` writer - The read wait function is optional for SDIO cards. If the card supports read wait, set this bit to enable use of the read wait protocol to stop read data using DAT\\[2\\]
line. Otherwise the HC has to stop the SD clock to hold read data, which restricts commands generation. When the HD detects an SD card insertion, it shall set this bit according to the CCCR of the SDIO card. If the card does not support read wait, this bit shall never be set to 1 otherwise DAT line conflict may occur. If this bit is set to 0, Suspend / Resume cannot be supported"]
pub type ReadwaitctrlW<'a, REG> = crate::BitWriter<'a, REG, Readwaitctrl>;
impl<'a, REG> ReadwaitctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Read Wait Control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Readwaitctrl::Enable)
    }
    #[doc = "Disable Read Wait Control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Readwaitctrl::Disable)
    }
}
#[doc = "Field `GAP` reader - This bit is valid only in 4-bit mode of the SDIO card and selects a sample point in the interrupt cycle. Setting to 1 enables interrupt detection at the block gap for a multiple block transfer. If the SD card cannot signal an interrupt during a multiple block transfer, this bit should be set to 0. When the HD detects an SD card insertion, it shall set this bit according to the CCCR of the SDIO card."]
pub type GapR = crate::BitReader;
#[doc = "Field `GAP` writer - This bit is valid only in 4-bit mode of the SDIO card and selects a sample point in the interrupt cycle. Setting to 1 enables interrupt detection at the block gap for a multiple block transfer. If the SD card cannot signal an interrupt during a multiple block transfer, this bit should be set to 0. When the HD detects an SD card insertion, it shall set this bit according to the CCCR of the SDIO card."]
pub type GapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "SPI mode enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spimode {
    #[doc = "1: SPI mode"]
    Spi = 1,
    #[doc = "0: SD mode"]
    Sd = 0,
}
impl From<Spimode> for bool {
    #[inline(always)]
    fn from(variant: Spimode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIMODE` reader - SPI mode enable bit."]
pub type SpimodeR = crate::BitReader<Spimode>;
impl SpimodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spimode {
        match self.bits {
            true => Spimode::Spi,
            false => Spimode::Sd,
        }
    }
    #[doc = "SPI mode"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == Spimode::Spi
    }
    #[doc = "SD mode"]
    #[inline(always)]
    pub fn is_sd(&self) -> bool {
        *self == Spimode::Sd
    }
}
#[doc = "Field `SPIMODE` writer - SPI mode enable bit."]
pub type SpimodeW<'a, REG> = crate::BitWriter<'a, REG, Spimode>;
impl<'a, REG> SpimodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI mode"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut crate::W<REG> {
        self.variant(Spimode::Spi)
    }
    #[doc = "SD mode"]
    #[inline(always)]
    pub fn sd(self) -> &'a mut crate::W<REG> {
        self.variant(Spimode::Sd)
    }
}
#[doc = "To start boot code access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Booten {
    #[doc = "1: To start boot code access"]
    Start = 1,
    #[doc = "0: To stop boot code access"]
    Stop = 0,
}
impl From<Booten> for bool {
    #[inline(always)]
    fn from(variant: Booten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOTEN` reader - To start boot code access"]
pub type BootenR = crate::BitReader<Booten>;
impl BootenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Booten {
        match self.bits {
            true => Booten::Start,
            false => Booten::Stop,
        }
    }
    #[doc = "To start boot code access"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Booten::Start
    }
    #[doc = "To stop boot code access"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Booten::Stop
    }
}
#[doc = "Field `BOOTEN` writer - To start boot code access"]
pub type BootenW<'a, REG> = crate::BitWriter<'a, REG, Booten>;
impl<'a, REG> BootenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "To start boot code access"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Booten::Start)
    }
    #[doc = "To stop boot code access"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Booten::Stop)
    }
}
#[doc = "To start boot code access in alternative mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Altbooten {
    #[doc = "1: To start alternate boot mode access"]
    Start = 1,
    #[doc = "0: To stop alternate boot mode access"]
    Stop = 0,
}
impl From<Altbooten> for bool {
    #[inline(always)]
    fn from(variant: Altbooten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALTBOOTEN` reader - To start boot code access in alternative mode."]
pub type AltbootenR = crate::BitReader<Altbooten>;
impl AltbootenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Altbooten {
        match self.bits {
            true => Altbooten::Start,
            false => Altbooten::Stop,
        }
    }
    #[doc = "To start alternate boot mode access"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Altbooten::Start
    }
    #[doc = "To stop alternate boot mode access"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Altbooten::Stop
    }
}
#[doc = "Field `ALTBOOTEN` writer - To start boot code access in alternative mode."]
pub type AltbootenW<'a, REG> = crate::BitWriter<'a, REG, Altbooten>;
impl<'a, REG> AltbootenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "To start alternate boot mode access"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Altbooten::Start)
    }
    #[doc = "To stop alternate boot mode access"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Altbooten::Stop)
    }
}
#[doc = "To check for the boot acknowledge in boot operation.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bootackchk {
    #[doc = "1: wait for boot ack from eMMC card"]
    Wait = 1,
    #[doc = "0: Will not wait for boot ack from eMMC card"]
    Nowait = 0,
}
impl From<Bootackchk> for bool {
    #[inline(always)]
    fn from(variant: Bootackchk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOTACKCHK` reader - To check for the boot acknowledge in boot operation."]
pub type BootackchkR = crate::BitReader<Bootackchk>;
impl BootackchkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bootackchk {
        match self.bits {
            true => Bootackchk::Wait,
            false => Bootackchk::Nowait,
        }
    }
    #[doc = "wait for boot ack from eMMC card"]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == Bootackchk::Wait
    }
    #[doc = "Will not wait for boot ack from eMMC card"]
    #[inline(always)]
    pub fn is_nowait(&self) -> bool {
        *self == Bootackchk::Nowait
    }
}
#[doc = "Field `BOOTACKCHK` writer - To check for the boot acknowledge in boot operation."]
pub type BootackchkW<'a, REG> = crate::BitWriter<'a, REG, Bootackchk>;
impl<'a, REG> BootackchkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wait for boot ack from eMMC card"]
    #[inline(always)]
    pub fn wait(self) -> &'a mut crate::W<REG> {
        self.variant(Bootackchk::Wait)
    }
    #[doc = "Will not wait for boot ack from eMMC card"]
    #[inline(always)]
    pub fn nowait(self) -> &'a mut crate::W<REG> {
        self.variant(Bootackchk::Nowait)
    }
}
#[doc = "This bit enables wakeup event via Card Interrupt assertion in the Normal Interrupt Status register. This bit can be set to 1 if FN_WUS (Wake Up Support) in CIS is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuencardint {
    #[doc = "1: Enable"]
    Enable = 1,
    #[doc = "0: Disable"]
    Disable = 0,
}
impl From<Wuencardint> for bool {
    #[inline(always)]
    fn from(variant: Wuencardint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUENCARDINT` reader - This bit enables wakeup event via Card Interrupt assertion in the Normal Interrupt Status register. This bit can be set to 1 if FN_WUS (Wake Up Support) in CIS is set to 1."]
pub type WuencardintR = crate::BitReader<Wuencardint>;
impl WuencardintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuencardint {
        match self.bits {
            true => Wuencardint::Enable,
            false => Wuencardint::Disable,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wuencardint::Enable
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wuencardint::Disable
    }
}
#[doc = "Field `WUENCARDINT` writer - This bit enables wakeup event via Card Interrupt assertion in the Normal Interrupt Status register. This bit can be set to 1 if FN_WUS (Wake Up Support) in CIS is set to 1."]
pub type WuencardintW<'a, REG> = crate::BitWriter<'a, REG, Wuencardint>;
impl<'a, REG> WuencardintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wuencardint::Enable)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wuencardint::Disable)
    }
}
#[doc = "This bit enables wakeup event via Card Insertion assertion in the Normal Interrupt Status register. FN_WUS (Wake up Support) in CIS does not affect this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuencardinsert {
    #[doc = "1: Enable"]
    Enable = 1,
    #[doc = "0: Disable"]
    Disable = 0,
}
impl From<Wuencardinsert> for bool {
    #[inline(always)]
    fn from(variant: Wuencardinsert) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUENCARDINSERT` reader - This bit enables wakeup event via Card Insertion assertion in the Normal Interrupt Status register. FN_WUS (Wake up Support) in CIS does not affect this bit."]
pub type WuencardinsertR = crate::BitReader<Wuencardinsert>;
impl WuencardinsertR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuencardinsert {
        match self.bits {
            true => Wuencardinsert::Enable,
            false => Wuencardinsert::Disable,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wuencardinsert::Enable
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wuencardinsert::Disable
    }
}
#[doc = "Field `WUENCARDINSERT` writer - This bit enables wakeup event via Card Insertion assertion in the Normal Interrupt Status register. FN_WUS (Wake up Support) in CIS does not affect this bit."]
pub type WuencardinsertW<'a, REG> = crate::BitWriter<'a, REG, Wuencardinsert>;
impl<'a, REG> WuencardinsertW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wuencardinsert::Enable)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wuencardinsert::Disable)
    }
}
#[doc = "This bit enables wakeup event via Card Removal assertion in the Normal Interrupt Status register. FN_WUS (Wake up Support) in CIS does not affect this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuencardremovl {
    #[doc = "1: Enable"]
    Enable = 1,
    #[doc = "0: Disable"]
    Disable = 0,
}
impl From<Wuencardremovl> for bool {
    #[inline(always)]
    fn from(variant: Wuencardremovl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUENCARDREMOVL` reader - This bit enables wakeup event via Card Removal assertion in the Normal Interrupt Status register. FN_WUS (Wake up Support) in CIS does not affect this bit."]
pub type WuencardremovlR = crate::BitReader<Wuencardremovl>;
impl WuencardremovlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuencardremovl {
        match self.bits {
            true => Wuencardremovl::Enable,
            false => Wuencardremovl::Disable,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wuencardremovl::Enable
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wuencardremovl::Disable
    }
}
#[doc = "Field `WUENCARDREMOVL` writer - This bit enables wakeup event via Card Removal assertion in the Normal Interrupt Status register. FN_WUS (Wake up Support) in CIS does not affect this bit."]
pub type WuencardremovlW<'a, REG> = crate::BitWriter<'a, REG, Wuencardremovl>;
impl<'a, REG> WuencardremovlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wuencardremovl::Enable)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wuencardremovl::Disable)
    }
}
impl R {
    #[doc = "Bit 0 - This bit is used to caution the user not to remove the card while the SD card is being accessed. If the software is going to issue multiple SD commands, this bit can be set during all transactions. It is not necessary to change for each transaction."]
    #[inline(always)]
    pub fn ledcontrol(&self) -> LedcontrolR {
        LedcontrolR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - (SD1 or SD4) This bit selects the data width of the HC. The HD shall select it to match the data width of the SD card."]
    #[inline(always)]
    pub fn datatransferwidth(&self) -> DatatransferwidthR {
        DatatransferwidthR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is optional. Before setting this bit, the HD shall check the High Speed Support in the capabilities register. If this bit is set to 0 (default), the HC outputs CMD line and DAT lines at the falling edge of the SD clock (up to 25 MHz/ 20MHz for MMC). If this bit is set to 1, the HC outputs CMD line and DAT lines at the rising edge of the SD clock (up to 50 MHz for SD/52MHz for MMC)/ 208Mhz (for SD3.0) If Preset Value Enable in the Host Control 2 register is set to 1, Host Driver needs to reset SD Clock Enable before changing this field to avoid generating clock glitches. After setting this field, the Host Driver sets SD Clock Enable again"]
    #[inline(always)]
    pub fn hispeeden(&self) -> HispeedenR {
        HispeedenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - One of supported DMA modes can be selected. The host driver shall check support of DMA modes by referring the Capabilities register."]
    #[inline(always)]
    pub fn dmaselect(&self) -> DmaselectR {
        DmaselectR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - This bit controls 8-bit bus width mode for embedded device. Support of this function is indicated in 8-bit Support for Embedded Device in the Capabilities register. If a device supports 8-bit bus mode, this bit may be set to 1. If this bit is 0, bus width is controlled by Data Transfer Width in the Host Control 1 register.This bit is not effective when multiple devices are installed on a bus slot (Slot Type is set to 10b in the Capabilities register). In this case, each device bus width is controlled by Bus Width Preset field in the Shared Bus register."]
    #[inline(always)]
    pub fn xferwidth(&self) -> XferwidthR {
        XferwidthR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is enabled while the Card Detect Signal Selection is set to 1 and it indicates card inserted or not. Generates (card ins or card removal) interrupt when the normal int sts enable bit is set."]
    #[inline(always)]
    pub fn testlevel(&self) -> TestlevelR {
        TestlevelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit selects source for card detection."]
    #[inline(always)]
    pub fn cardsrc(&self) -> CardsrcR {
        CardsrcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Before setting this bit, the SD host driver shall set SD Bus Voltage Select. If the HC detects the No Card State, this bit shall be cleared."]
    #[inline(always)]
    pub fn sdbuspower(&self) -> SdbuspowerR {
        SdbuspowerR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - By setting these bits, the HD selects the voltage level for the SD card. Before setting this register, the HD shall check the voltage support bits in the capabilities register. If an unsupported voltage is selected, the Host System shall not supply SD bus voltage. All voltage select values not enumerated here are reserved."]
    #[inline(always)]
    pub fn voltselect(&self) -> VoltselectR {
        VoltselectR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Hardware reset signal is generated for eMMC card when this bit is set"]
    #[inline(always)]
    pub fn hwreset(&self) -> HwresetR {
        HwresetR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit is used to stop executing a transaction at the next block gap for non- DMA,SDMA and ADMA transfers. Until the transfer complete is set to 1, indicating a transfer completion the HD shall leave this bit set to 1. Clearing both the Stop At Block Gap Request and Continue Request shall not cause the transaction to restart. Read Wait is used to stop the read transaction at the block gap. The HC shall honour Stop At Block Gap Request for write transfers, but for read transfers it requires that the SD card support Read Wait. Therefore the HD shall not set this bit during read transfers unless the SD card supports Read Wait and has set Read Wait Control to 1. In case of write transfers in which the HD writes data to the Buffer Data Port register, the HD shall set this bit after all block data is written. If this bit is set to 1, the HD shall not write data to Buffer data port register. This bit affects Read Transfer Active, Write Transfer Active, DAT line active and Command Inhibit (DAT) in the Present State register."]
    #[inline(always)]
    pub fn stopatblockgaprequest(&self) -> StopatblockgaprequestR {
        StopatblockgaprequestR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This bit is used to restart a transaction which was stopped using the Stop At Block Gap Request. To cancel stop at the block gap, set Stop At block Gap Request to 0 and set this bit to restart the transfer. The HC automatically clears this bit in either of the following cases: 1) In the case of a read transaction, the DAT Line Active changes from 0 to 1 as a read transaction restarts. 2) In the case of a write transaction, the Write transfer active changes from 0 to 1 as the write transaction restarts. Therefore it is not necessary for Host driver to set this bit to 0. If Stop At Block Gap Request is set to 1, any write to this bit is ignored."]
    #[inline(always)]
    pub fn contreq(&self) -> ContreqR {
        ContreqR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The read wait function is optional for SDIO cards. If the card supports read wait, set this bit to enable use of the read wait protocol to stop read data using DAT\\[2\\]
line. Otherwise the HC has to stop the SD clock to hold read data, which restricts commands generation. When the HD detects an SD card insertion, it shall set this bit according to the CCCR of the SDIO card. If the card does not support read wait, this bit shall never be set to 1 otherwise DAT line conflict may occur. If this bit is set to 0, Suspend / Resume cannot be supported"]
    #[inline(always)]
    pub fn readwaitctrl(&self) -> ReadwaitctrlR {
        ReadwaitctrlR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This bit is valid only in 4-bit mode of the SDIO card and selects a sample point in the interrupt cycle. Setting to 1 enables interrupt detection at the block gap for a multiple block transfer. If the SD card cannot signal an interrupt during a multiple block transfer, this bit should be set to 0. When the HD detects an SD card insertion, it shall set this bit according to the CCCR of the SDIO card."]
    #[inline(always)]
    pub fn gap(&self) -> GapR {
        GapR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI mode enable bit."]
    #[inline(always)]
    pub fn spimode(&self) -> SpimodeR {
        SpimodeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - To start boot code access"]
    #[inline(always)]
    pub fn booten(&self) -> BootenR {
        BootenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - To start boot code access in alternative mode."]
    #[inline(always)]
    pub fn altbooten(&self) -> AltbootenR {
        AltbootenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - To check for the boot acknowledge in boot operation."]
    #[inline(always)]
    pub fn bootackchk(&self) -> BootackchkR {
        BootackchkR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit enables wakeup event via Card Interrupt assertion in the Normal Interrupt Status register. This bit can be set to 1 if FN_WUS (Wake Up Support) in CIS is set to 1."]
    #[inline(always)]
    pub fn wuencardint(&self) -> WuencardintR {
        WuencardintR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - This bit enables wakeup event via Card Insertion assertion in the Normal Interrupt Status register. FN_WUS (Wake up Support) in CIS does not affect this bit."]
    #[inline(always)]
    pub fn wuencardinsert(&self) -> WuencardinsertR {
        WuencardinsertR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - This bit enables wakeup event via Card Removal assertion in the Normal Interrupt Status register. FN_WUS (Wake up Support) in CIS does not affect this bit."]
    #[inline(always)]
    pub fn wuencardremovl(&self) -> WuencardremovlR {
        WuencardremovlR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to caution the user not to remove the card while the SD card is being accessed. If the software is going to issue multiple SD commands, this bit can be set during all transactions. It is not necessary to change for each transaction."]
    #[inline(always)]
    #[must_use]
    pub fn ledcontrol(&mut self) -> LedcontrolW<Hostctrl1Spec> {
        LedcontrolW::new(self, 0)
    }
    #[doc = "Bit 1 - (SD1 or SD4) This bit selects the data width of the HC. The HD shall select it to match the data width of the SD card."]
    #[inline(always)]
    #[must_use]
    pub fn datatransferwidth(&mut self) -> DatatransferwidthW<Hostctrl1Spec> {
        DatatransferwidthW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit is optional. Before setting this bit, the HD shall check the High Speed Support in the capabilities register. If this bit is set to 0 (default), the HC outputs CMD line and DAT lines at the falling edge of the SD clock (up to 25 MHz/ 20MHz for MMC). If this bit is set to 1, the HC outputs CMD line and DAT lines at the rising edge of the SD clock (up to 50 MHz for SD/52MHz for MMC)/ 208Mhz (for SD3.0) If Preset Value Enable in the Host Control 2 register is set to 1, Host Driver needs to reset SD Clock Enable before changing this field to avoid generating clock glitches. After setting this field, the Host Driver sets SD Clock Enable again"]
    #[inline(always)]
    #[must_use]
    pub fn hispeeden(&mut self) -> HispeedenW<Hostctrl1Spec> {
        HispeedenW::new(self, 2)
    }
    #[doc = "Bits 3:4 - One of supported DMA modes can be selected. The host driver shall check support of DMA modes by referring the Capabilities register."]
    #[inline(always)]
    #[must_use]
    pub fn dmaselect(&mut self) -> DmaselectW<Hostctrl1Spec> {
        DmaselectW::new(self, 3)
    }
    #[doc = "Bit 5 - This bit controls 8-bit bus width mode for embedded device. Support of this function is indicated in 8-bit Support for Embedded Device in the Capabilities register. If a device supports 8-bit bus mode, this bit may be set to 1. If this bit is 0, bus width is controlled by Data Transfer Width in the Host Control 1 register.This bit is not effective when multiple devices are installed on a bus slot (Slot Type is set to 10b in the Capabilities register). In this case, each device bus width is controlled by Bus Width Preset field in the Shared Bus register."]
    #[inline(always)]
    #[must_use]
    pub fn xferwidth(&mut self) -> XferwidthW<Hostctrl1Spec> {
        XferwidthW::new(self, 5)
    }
    #[doc = "Bit 6 - This bit is enabled while the Card Detect Signal Selection is set to 1 and it indicates card inserted or not. Generates (card ins or card removal) interrupt when the normal int sts enable bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn testlevel(&mut self) -> TestlevelW<Hostctrl1Spec> {
        TestlevelW::new(self, 6)
    }
    #[doc = "Bit 7 - This bit selects source for card detection."]
    #[inline(always)]
    #[must_use]
    pub fn cardsrc(&mut self) -> CardsrcW<Hostctrl1Spec> {
        CardsrcW::new(self, 7)
    }
    #[doc = "Bit 8 - Before setting this bit, the SD host driver shall set SD Bus Voltage Select. If the HC detects the No Card State, this bit shall be cleared."]
    #[inline(always)]
    #[must_use]
    pub fn sdbuspower(&mut self) -> SdbuspowerW<Hostctrl1Spec> {
        SdbuspowerW::new(self, 8)
    }
    #[doc = "Bits 9:11 - By setting these bits, the HD selects the voltage level for the SD card. Before setting this register, the HD shall check the voltage support bits in the capabilities register. If an unsupported voltage is selected, the Host System shall not supply SD bus voltage. All voltage select values not enumerated here are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn voltselect(&mut self) -> VoltselectW<Hostctrl1Spec> {
        VoltselectW::new(self, 9)
    }
    #[doc = "Bit 12 - Hardware reset signal is generated for eMMC card when this bit is set"]
    #[inline(always)]
    #[must_use]
    pub fn hwreset(&mut self) -> HwresetW<Hostctrl1Spec> {
        HwresetW::new(self, 12)
    }
    #[doc = "Bit 16 - This bit is used to stop executing a transaction at the next block gap for non- DMA,SDMA and ADMA transfers. Until the transfer complete is set to 1, indicating a transfer completion the HD shall leave this bit set to 1. Clearing both the Stop At Block Gap Request and Continue Request shall not cause the transaction to restart. Read Wait is used to stop the read transaction at the block gap. The HC shall honour Stop At Block Gap Request for write transfers, but for read transfers it requires that the SD card support Read Wait. Therefore the HD shall not set this bit during read transfers unless the SD card supports Read Wait and has set Read Wait Control to 1. In case of write transfers in which the HD writes data to the Buffer Data Port register, the HD shall set this bit after all block data is written. If this bit is set to 1, the HD shall not write data to Buffer data port register. This bit affects Read Transfer Active, Write Transfer Active, DAT line active and Command Inhibit (DAT) in the Present State register."]
    #[inline(always)]
    #[must_use]
    pub fn stopatblockgaprequest(&mut self) -> StopatblockgaprequestW<Hostctrl1Spec> {
        StopatblockgaprequestW::new(self, 16)
    }
    #[doc = "Bit 17 - This bit is used to restart a transaction which was stopped using the Stop At Block Gap Request. To cancel stop at the block gap, set Stop At block Gap Request to 0 and set this bit to restart the transfer. The HC automatically clears this bit in either of the following cases: 1) In the case of a read transaction, the DAT Line Active changes from 0 to 1 as a read transaction restarts. 2) In the case of a write transaction, the Write transfer active changes from 0 to 1 as the write transaction restarts. Therefore it is not necessary for Host driver to set this bit to 0. If Stop At Block Gap Request is set to 1, any write to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn contreq(&mut self) -> ContreqW<Hostctrl1Spec> {
        ContreqW::new(self, 17)
    }
    #[doc = "Bit 18 - The read wait function is optional for SDIO cards. If the card supports read wait, set this bit to enable use of the read wait protocol to stop read data using DAT\\[2\\]
line. Otherwise the HC has to stop the SD clock to hold read data, which restricts commands generation. When the HD detects an SD card insertion, it shall set this bit according to the CCCR of the SDIO card. If the card does not support read wait, this bit shall never be set to 1 otherwise DAT line conflict may occur. If this bit is set to 0, Suspend / Resume cannot be supported"]
    #[inline(always)]
    #[must_use]
    pub fn readwaitctrl(&mut self) -> ReadwaitctrlW<Hostctrl1Spec> {
        ReadwaitctrlW::new(self, 18)
    }
    #[doc = "Bit 19 - This bit is valid only in 4-bit mode of the SDIO card and selects a sample point in the interrupt cycle. Setting to 1 enables interrupt detection at the block gap for a multiple block transfer. If the SD card cannot signal an interrupt during a multiple block transfer, this bit should be set to 0. When the HD detects an SD card insertion, it shall set this bit according to the CCCR of the SDIO card."]
    #[inline(always)]
    #[must_use]
    pub fn gap(&mut self) -> GapW<Hostctrl1Spec> {
        GapW::new(self, 19)
    }
    #[doc = "Bit 20 - SPI mode enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn spimode(&mut self) -> SpimodeW<Hostctrl1Spec> {
        SpimodeW::new(self, 20)
    }
    #[doc = "Bit 21 - To start boot code access"]
    #[inline(always)]
    #[must_use]
    pub fn booten(&mut self) -> BootenW<Hostctrl1Spec> {
        BootenW::new(self, 21)
    }
    #[doc = "Bit 22 - To start boot code access in alternative mode."]
    #[inline(always)]
    #[must_use]
    pub fn altbooten(&mut self) -> AltbootenW<Hostctrl1Spec> {
        AltbootenW::new(self, 22)
    }
    #[doc = "Bit 23 - To check for the boot acknowledge in boot operation."]
    #[inline(always)]
    #[must_use]
    pub fn bootackchk(&mut self) -> BootackchkW<Hostctrl1Spec> {
        BootackchkW::new(self, 23)
    }
    #[doc = "Bit 24 - This bit enables wakeup event via Card Interrupt assertion in the Normal Interrupt Status register. This bit can be set to 1 if FN_WUS (Wake Up Support) in CIS is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn wuencardint(&mut self) -> WuencardintW<Hostctrl1Spec> {
        WuencardintW::new(self, 24)
    }
    #[doc = "Bit 25 - This bit enables wakeup event via Card Insertion assertion in the Normal Interrupt Status register. FN_WUS (Wake up Support) in CIS does not affect this bit."]
    #[inline(always)]
    #[must_use]
    pub fn wuencardinsert(&mut self) -> WuencardinsertW<Hostctrl1Spec> {
        WuencardinsertW::new(self, 25)
    }
    #[doc = "Bit 26 - This bit enables wakeup event via Card Removal assertion in the Normal Interrupt Status register. FN_WUS (Wake up Support) in CIS does not affect this bit."]
    #[inline(always)]
    #[must_use]
    pub fn wuencardremovl(&mut self) -> WuencardremovlW<Hostctrl1Spec> {
        WuencardremovlW::new(self, 26)
    }
}
#[doc = "Host control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hostctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hostctrl1Spec;
impl crate::RegisterSpec for Hostctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostctrl1::R`](R) reader structure"]
impl crate::Readable for Hostctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`hostctrl1::W`](W) writer structure"]
impl crate::Writable for Hostctrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTCTRL1 to value 0x0080_0000"]
impl crate::Resettable for Hostctrl1Spec {
    const RESET_VALUE: u32 = 0x0080_0000;
}
