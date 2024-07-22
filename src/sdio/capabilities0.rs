#[doc = "Register `CAPABILITIES0` reader"]
pub type R = crate::R<Capabilities0Spec>;
#[doc = "Register `CAPABILITIES0` writer"]
pub type W = crate::W<Capabilities0Spec>;
#[doc = "This bit shows the base clock frequency used to detect Data Timeout Error. Not 0 - 1Khz to 63Khz or 1Mhz to 63Mhz Note: The Host System shall support at least one of these voltages above. The HD sets the SD Bus Voltage Select in Power Control register according to these support bits. If multiple voltages are supported, select the usable lower voltage by comparing the OCR value from the card. These registers indicate maximum current capability for each voltage. The value is meaningful if Voltage Support is set in the Capabilities register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Toclkfreq {
    #[doc = "1: 1KHZ or 1MHZ"]
    _1 = 1,
    #[doc = "2: 2KHZ or 2MHZ"]
    _2 = 2,
    #[doc = "63: 63KHZ or 63MHZ"]
    _63 = 63,
    #[doc = "0: Get Information via another method."]
    Other = 0,
}
impl From<Toclkfreq> for u8 {
    #[inline(always)]
    fn from(variant: Toclkfreq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Toclkfreq {
    type Ux = u8;
}
impl crate::IsEnum for Toclkfreq {}
#[doc = "Field `TOCLKFREQ` reader - This bit shows the base clock frequency used to detect Data Timeout Error. Not 0 - 1Khz to 63Khz or 1Mhz to 63Mhz Note: The Host System shall support at least one of these voltages above. The HD sets the SD Bus Voltage Select in Power Control register according to these support bits. If multiple voltages are supported, select the usable lower voltage by comparing the OCR value from the card. These registers indicate maximum current capability for each voltage. The value is meaningful if Voltage Support is set in the Capabilities register."]
pub type ToclkfreqR = crate::FieldReader<Toclkfreq>;
impl ToclkfreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Toclkfreq> {
        match self.bits {
            1 => Some(Toclkfreq::_1),
            2 => Some(Toclkfreq::_2),
            63 => Some(Toclkfreq::_63),
            0 => Some(Toclkfreq::Other),
            _ => None,
        }
    }
    #[doc = "1KHZ or 1MHZ"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Toclkfreq::_1
    }
    #[doc = "2KHZ or 2MHZ"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Toclkfreq::_2
    }
    #[doc = "63KHZ or 63MHZ"]
    #[inline(always)]
    pub fn is_63(&self) -> bool {
        *self == Toclkfreq::_63
    }
    #[doc = "Get Information via another method."]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == Toclkfreq::Other
    }
}
#[doc = "Field `TOCLKFREQ` writer - This bit shows the base clock frequency used to detect Data Timeout Error. Not 0 - 1Khz to 63Khz or 1Mhz to 63Mhz Note: The Host System shall support at least one of these voltages above. The HD sets the SD Bus Voltage Select in Power Control register according to these support bits. If multiple voltages are supported, select the usable lower voltage by comparing the OCR value from the card. These registers indicate maximum current capability for each voltage. The value is meaningful if Voltage Support is set in the Capabilities register."]
pub type ToclkfreqW<'a, REG> = crate::FieldWriter<'a, REG, 6, Toclkfreq>;
impl<'a, REG> ToclkfreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1KHZ or 1MHZ"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Toclkfreq::_1)
    }
    #[doc = "2KHZ or 2MHZ"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Toclkfreq::_2)
    }
    #[doc = "63KHZ or 63MHZ"]
    #[inline(always)]
    pub fn _63(self) -> &'a mut crate::W<REG> {
        self.variant(Toclkfreq::_63)
    }
    #[doc = "Get Information via another method."]
    #[inline(always)]
    pub fn other(self) -> &'a mut crate::W<REG> {
        self.variant(Toclkfreq::Other)
    }
}
#[doc = "This bit shows the unit of base clock frequency used to detect Data Timeout Error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Toclkunit {
    #[doc = "0: Khz"]
    Khz = 0,
    #[doc = "1: Mhz"]
    Mhz = 1,
}
impl From<Toclkunit> for bool {
    #[inline(always)]
    fn from(variant: Toclkunit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOCLKUNIT` reader - This bit shows the unit of base clock frequency used to detect Data Timeout Error."]
pub type ToclkunitR = crate::BitReader<Toclkunit>;
impl ToclkunitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Toclkunit {
        match self.bits {
            false => Toclkunit::Khz,
            true => Toclkunit::Mhz,
        }
    }
    #[doc = "Khz"]
    #[inline(always)]
    pub fn is_khz(&self) -> bool {
        *self == Toclkunit::Khz
    }
    #[doc = "Mhz"]
    #[inline(always)]
    pub fn is_mhz(&self) -> bool {
        *self == Toclkunit::Mhz
    }
}
#[doc = "Field `TOCLKUNIT` writer - This bit shows the unit of base clock frequency used to detect Data Timeout Error."]
pub type ToclkunitW<'a, REG> = crate::BitWriter<'a, REG, Toclkunit>;
impl<'a, REG> ToclkunitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Khz"]
    #[inline(always)]
    pub fn khz(self) -> &'a mut crate::W<REG> {
        self.variant(Toclkunit::Khz)
    }
    #[doc = "Mhz"]
    #[inline(always)]
    pub fn mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Toclkunit::Mhz)
    }
}
#[doc = "6-bit Base Clock Frequency This mode is supported by the Host Controller Version 1.00 and 2.00. Upper 2-bit is not effective and always 0. Unit values are 1MHz. The supported clock range is 10MHz to 63MHz. 11xx xxxxb Not supported 0011 1111b 63MHz 0000 0010b 2MHz 0000 0001b 1MHz 0000 0000b Get information via another method (2) 8-bit Base Clock Frequency This mode is supported by the Host Controller Version 3.00.Unit values are 1MHz. The supported clock range is 10MHz to 255MHz. FFh 255MHz 02h 2MHz 01h 1MHz 00h Get information via another method If the real frequency is 16.5MHz, the lager value shall be set 0001 0001b (17MHz) because the Host Driver use this value to calculate the clock divider value (Refer to the SDCLK Frequency Select in the Clock Control register.) and it shall not exceed upper limit of the SD Clock frequency. If these bits are all 0, the Host System has to get information via another method. Some example frequency values are enumerated below.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdclkfreq {
    #[doc = "255: 2) 8-bit base clock frequency supports frequencies 10MHz-255MHz."]
    _255mhz = 255,
    #[doc = "63: 1) 6-bit base clock frequency supports frequencies 10MHz-63MHz. 2) 8-bit base clock frequency supports frequencies 10MHz-255MHz."]
    _63mhz = 63,
    #[doc = "2: 1) 6-bit base clock frequency supports frequencies 10MHz-63MHz. 2) 8-bit base clock frequency supports frequencies 10MHz-255MHz."]
    _2mhz = 2,
    #[doc = "1: 1) 6-bit base clock frequency supports frequencies 10MHz-63MHz. 2) 8-bit base clock frequency supports frequencies 10MHz-255MHz."]
    _1mhz = 1,
    #[doc = "0: Get information via another method"]
    Other = 0,
}
impl From<Sdclkfreq> for u8 {
    #[inline(always)]
    fn from(variant: Sdclkfreq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdclkfreq {
    type Ux = u8;
}
impl crate::IsEnum for Sdclkfreq {}
#[doc = "Field `SDCLKFREQ` reader - 6-bit Base Clock Frequency This mode is supported by the Host Controller Version 1.00 and 2.00. Upper 2-bit is not effective and always 0. Unit values are 1MHz. The supported clock range is 10MHz to 63MHz. 11xx xxxxb Not supported 0011 1111b 63MHz 0000 0010b 2MHz 0000 0001b 1MHz 0000 0000b Get information via another method (2) 8-bit Base Clock Frequency This mode is supported by the Host Controller Version 3.00.Unit values are 1MHz. The supported clock range is 10MHz to 255MHz. FFh 255MHz 02h 2MHz 01h 1MHz 00h Get information via another method If the real frequency is 16.5MHz, the lager value shall be set 0001 0001b (17MHz) because the Host Driver use this value to calculate the clock divider value (Refer to the SDCLK Frequency Select in the Clock Control register.) and it shall not exceed upper limit of the SD Clock frequency. If these bits are all 0, the Host System has to get information via another method. Some example frequency values are enumerated below."]
pub type SdclkfreqR = crate::FieldReader<Sdclkfreq>;
impl SdclkfreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sdclkfreq> {
        match self.bits {
            255 => Some(Sdclkfreq::_255mhz),
            63 => Some(Sdclkfreq::_63mhz),
            2 => Some(Sdclkfreq::_2mhz),
            1 => Some(Sdclkfreq::_1mhz),
            0 => Some(Sdclkfreq::Other),
            _ => None,
        }
    }
    #[doc = "2) 8-bit base clock frequency supports frequencies 10MHz-255MHz."]
    #[inline(always)]
    pub fn is_255mhz(&self) -> bool {
        *self == Sdclkfreq::_255mhz
    }
    #[doc = "1) 6-bit base clock frequency supports frequencies 10MHz-63MHz. 2) 8-bit base clock frequency supports frequencies 10MHz-255MHz."]
    #[inline(always)]
    pub fn is_63mhz(&self) -> bool {
        *self == Sdclkfreq::_63mhz
    }
    #[doc = "1) 6-bit base clock frequency supports frequencies 10MHz-63MHz. 2) 8-bit base clock frequency supports frequencies 10MHz-255MHz."]
    #[inline(always)]
    pub fn is_2mhz(&self) -> bool {
        *self == Sdclkfreq::_2mhz
    }
    #[doc = "1) 6-bit base clock frequency supports frequencies 10MHz-63MHz. 2) 8-bit base clock frequency supports frequencies 10MHz-255MHz."]
    #[inline(always)]
    pub fn is_1mhz(&self) -> bool {
        *self == Sdclkfreq::_1mhz
    }
    #[doc = "Get information via another method"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == Sdclkfreq::Other
    }
}
#[doc = "Field `SDCLKFREQ` writer - 6-bit Base Clock Frequency This mode is supported by the Host Controller Version 1.00 and 2.00. Upper 2-bit is not effective and always 0. Unit values are 1MHz. The supported clock range is 10MHz to 63MHz. 11xx xxxxb Not supported 0011 1111b 63MHz 0000 0010b 2MHz 0000 0001b 1MHz 0000 0000b Get information via another method (2) 8-bit Base Clock Frequency This mode is supported by the Host Controller Version 3.00.Unit values are 1MHz. The supported clock range is 10MHz to 255MHz. FFh 255MHz 02h 2MHz 01h 1MHz 00h Get information via another method If the real frequency is 16.5MHz, the lager value shall be set 0001 0001b (17MHz) because the Host Driver use this value to calculate the clock divider value (Refer to the SDCLK Frequency Select in the Clock Control register.) and it shall not exceed upper limit of the SD Clock frequency. If these bits are all 0, the Host System has to get information via another method. Some example frequency values are enumerated below."]
pub type SdclkfreqW<'a, REG> = crate::FieldWriter<'a, REG, 8, Sdclkfreq>;
impl<'a, REG> SdclkfreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2) 8-bit base clock frequency supports frequencies 10MHz-255MHz."]
    #[inline(always)]
    pub fn _255mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Sdclkfreq::_255mhz)
    }
    #[doc = "1) 6-bit base clock frequency supports frequencies 10MHz-63MHz. 2) 8-bit base clock frequency supports frequencies 10MHz-255MHz."]
    #[inline(always)]
    pub fn _63mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Sdclkfreq::_63mhz)
    }
    #[doc = "1) 6-bit base clock frequency supports frequencies 10MHz-63MHz. 2) 8-bit base clock frequency supports frequencies 10MHz-255MHz."]
    #[inline(always)]
    pub fn _2mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Sdclkfreq::_2mhz)
    }
    #[doc = "1) 6-bit base clock frequency supports frequencies 10MHz-63MHz. 2) 8-bit base clock frequency supports frequencies 10MHz-255MHz."]
    #[inline(always)]
    pub fn _1mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Sdclkfreq::_1mhz)
    }
    #[doc = "Get information via another method"]
    #[inline(always)]
    pub fn other(self) -> &'a mut crate::W<REG> {
        self.variant(Sdclkfreq::Other)
    }
}
#[doc = "This value indicates the maximum block size that the HD can read and write to the buffer in the HC. The buffer shall transfer this block size without wait cycles. Three sizes can be defined as indicated below.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Maxblklen {
    #[doc = "0: 512 byte"]
    _512 = 0,
    #[doc = "1: 1024 byte"]
    _1024 = 1,
    #[doc = "2: 2048 byte"]
    _2048 = 2,
    #[doc = "3: 4096 byte"]
    _4096 = 3,
}
impl From<Maxblklen> for u8 {
    #[inline(always)]
    fn from(variant: Maxblklen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Maxblklen {
    type Ux = u8;
}
impl crate::IsEnum for Maxblklen {}
#[doc = "Field `MAXBLKLEN` reader - This value indicates the maximum block size that the HD can read and write to the buffer in the HC. The buffer shall transfer this block size without wait cycles. Three sizes can be defined as indicated below."]
pub type MaxblklenR = crate::FieldReader<Maxblklen>;
impl MaxblklenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Maxblklen {
        match self.bits {
            0 => Maxblklen::_512,
            1 => Maxblklen::_1024,
            2 => Maxblklen::_2048,
            3 => Maxblklen::_4096,
            _ => unreachable!(),
        }
    }
    #[doc = "512 byte"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == Maxblklen::_512
    }
    #[doc = "1024 byte"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == Maxblklen::_1024
    }
    #[doc = "2048 byte"]
    #[inline(always)]
    pub fn is_2048(&self) -> bool {
        *self == Maxblklen::_2048
    }
    #[doc = "4096 byte"]
    #[inline(always)]
    pub fn is_4096(&self) -> bool {
        *self == Maxblklen::_4096
    }
}
#[doc = "Field `MAXBLKLEN` writer - This value indicates the maximum block size that the HD can read and write to the buffer in the HC. The buffer shall transfer this block size without wait cycles. Three sizes can be defined as indicated below."]
pub type MaxblklenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Maxblklen, crate::Safe>;
impl<'a, REG> MaxblklenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "512 byte"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut crate::W<REG> {
        self.variant(Maxblklen::_512)
    }
    #[doc = "1024 byte"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut crate::W<REG> {
        self.variant(Maxblklen::_1024)
    }
    #[doc = "2048 byte"]
    #[inline(always)]
    pub fn _2048(self) -> &'a mut crate::W<REG> {
        self.variant(Maxblklen::_2048)
    }
    #[doc = "4096 byte"]
    #[inline(always)]
    pub fn _4096(self) -> &'a mut crate::W<REG> {
        self.variant(Maxblklen::_4096)
    }
}
#[doc = "This bit indicates whether the Host Controller is capable of using 8-bit bus width mode. This bit is not effective when Slot Type is set to 10b. In this case, refer to Bus Width Preset in the Shared Bus resister. Supported\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extmedia {
    #[doc = "1: Extended Media Bus Supported"]
    Supported = 1,
    #[doc = "0: Extended Media Bus not supported"]
    Notsupported = 0,
}
impl From<Extmedia> for bool {
    #[inline(always)]
    fn from(variant: Extmedia) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTMEDIA` reader - This bit indicates whether the Host Controller is capable of using 8-bit bus width mode. This bit is not effective when Slot Type is set to 10b. In this case, refer to Bus Width Preset in the Shared Bus resister. Supported"]
pub type ExtmediaR = crate::BitReader<Extmedia>;
impl ExtmediaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extmedia {
        match self.bits {
            true => Extmedia::Supported,
            false => Extmedia::Notsupported,
        }
    }
    #[doc = "Extended Media Bus Supported"]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Extmedia::Supported
    }
    #[doc = "Extended Media Bus not supported"]
    #[inline(always)]
    pub fn is_notsupported(&self) -> bool {
        *self == Extmedia::Notsupported
    }
}
#[doc = "Field `EXTMEDIA` writer - This bit indicates whether the Host Controller is capable of using 8-bit bus width mode. This bit is not effective when Slot Type is set to 10b. In this case, refer to Bus Width Preset in the Shared Bus resister. Supported"]
pub type ExtmediaW<'a, REG> = crate::BitWriter<'a, REG, Extmedia>;
impl<'a, REG> ExtmediaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Extended Media Bus Supported"]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Extmedia::Supported)
    }
    #[doc = "Extended Media Bus not supported"]
    #[inline(always)]
    pub fn notsupported(self) -> &'a mut crate::W<REG> {
        self.variant(Extmedia::Notsupported)
    }
}
#[doc = "Desc\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adma2 {
    #[doc = "1: ADMA2 support."]
    Supported = 1,
    #[doc = "0: ADMA2 not support"]
    Notsupported = 0,
}
impl From<Adma2> for bool {
    #[inline(always)]
    fn from(variant: Adma2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADMA2` reader - Desc"]
pub type Adma2R = crate::BitReader<Adma2>;
impl Adma2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adma2 {
        match self.bits {
            true => Adma2::Supported,
            false => Adma2::Notsupported,
        }
    }
    #[doc = "ADMA2 support."]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Adma2::Supported
    }
    #[doc = "ADMA2 not support"]
    #[inline(always)]
    pub fn is_notsupported(&self) -> bool {
        *self == Adma2::Notsupported
    }
}
#[doc = "Field `ADMA2` writer - Desc"]
pub type Adma2W<'a, REG> = crate::BitWriter<'a, REG, Adma2>;
impl<'a, REG> Adma2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADMA2 support."]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Adma2::Supported)
    }
    #[doc = "ADMA2 not support"]
    #[inline(always)]
    pub fn notsupported(self) -> &'a mut crate::W<REG> {
        self.variant(Adma2::Notsupported)
    }
}
#[doc = "This bit indicates whether the HC and the Host System support High Speed mode and they can supply SD Clock frequency from 25Mhz to 50 Mhz (for SD)/ 20MHz to 52MHz (for MMC).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Highspeed {
    #[doc = "0: High Speed Not Supported"]
    Notsupported = 0,
    #[doc = "1: High Speed Supported"]
    Supported = 1,
}
impl From<Highspeed> for bool {
    #[inline(always)]
    fn from(variant: Highspeed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIGHSPEED` reader - This bit indicates whether the HC and the Host System support High Speed mode and they can supply SD Clock frequency from 25Mhz to 50 Mhz (for SD)/ 20MHz to 52MHz (for MMC)."]
pub type HighspeedR = crate::BitReader<Highspeed>;
impl HighspeedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Highspeed {
        match self.bits {
            false => Highspeed::Notsupported,
            true => Highspeed::Supported,
        }
    }
    #[doc = "High Speed Not Supported"]
    #[inline(always)]
    pub fn is_notsupported(&self) -> bool {
        *self == Highspeed::Notsupported
    }
    #[doc = "High Speed Supported"]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Highspeed::Supported
    }
}
#[doc = "Field `HIGHSPEED` writer - This bit indicates whether the HC and the Host System support High Speed mode and they can supply SD Clock frequency from 25Mhz to 50 Mhz (for SD)/ 20MHz to 52MHz (for MMC)."]
pub type HighspeedW<'a, REG> = crate::BitWriter<'a, REG, Highspeed>;
impl<'a, REG> HighspeedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High Speed Not Supported"]
    #[inline(always)]
    pub fn notsupported(self) -> &'a mut crate::W<REG> {
        self.variant(Highspeed::Notsupported)
    }
    #[doc = "High Speed Supported"]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Highspeed::Supported)
    }
}
#[doc = "This bit indicates whether the HC is capable of using DMA to transfer data between system memory and the HC directly.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdma {
    #[doc = "0: SDMA Not Supported"]
    Notsupported = 0,
    #[doc = "1: SDMA Supported."]
    Supported = 1,
}
impl From<Sdma> for bool {
    #[inline(always)]
    fn from(variant: Sdma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMA` reader - This bit indicates whether the HC is capable of using DMA to transfer data between system memory and the HC directly."]
pub type SdmaR = crate::BitReader<Sdma>;
impl SdmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdma {
        match self.bits {
            false => Sdma::Notsupported,
            true => Sdma::Supported,
        }
    }
    #[doc = "SDMA Not Supported"]
    #[inline(always)]
    pub fn is_notsupported(&self) -> bool {
        *self == Sdma::Notsupported
    }
    #[doc = "SDMA Supported."]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Sdma::Supported
    }
}
#[doc = "Field `SDMA` writer - This bit indicates whether the HC is capable of using DMA to transfer data between system memory and the HC directly."]
pub type SdmaW<'a, REG> = crate::BitWriter<'a, REG, Sdma>;
impl<'a, REG> SdmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDMA Not Supported"]
    #[inline(always)]
    pub fn notsupported(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma::Notsupported)
    }
    #[doc = "SDMA Supported."]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma::Supported)
    }
}
#[doc = "This bit indicates whether the HC supports Suspend / Resume functionality. If this bit is 0, the Suspend and Resume mechanism are not supported and the HD shall not issue either Suspend / Resume commands.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suspres {
    #[doc = "0: Suspend / Resume Not Supported"]
    Notsupported = 0,
    #[doc = "1: Suspend / Resume Supported"]
    Supported = 1,
}
impl From<Suspres> for bool {
    #[inline(always)]
    fn from(variant: Suspres) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPRES` reader - This bit indicates whether the HC supports Suspend / Resume functionality. If this bit is 0, the Suspend and Resume mechanism are not supported and the HD shall not issue either Suspend / Resume commands."]
pub type SuspresR = crate::BitReader<Suspres>;
impl SuspresR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Suspres {
        match self.bits {
            false => Suspres::Notsupported,
            true => Suspres::Supported,
        }
    }
    #[doc = "Suspend / Resume Not Supported"]
    #[inline(always)]
    pub fn is_notsupported(&self) -> bool {
        *self == Suspres::Notsupported
    }
    #[doc = "Suspend / Resume Supported"]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Suspres::Supported
    }
}
#[doc = "Field `SUSPRES` writer - This bit indicates whether the HC supports Suspend / Resume functionality. If this bit is 0, the Suspend and Resume mechanism are not supported and the HD shall not issue either Suspend / Resume commands."]
pub type SuspresW<'a, REG> = crate::BitWriter<'a, REG, Suspres>;
impl<'a, REG> SuspresW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Suspend / Resume Not Supported"]
    #[inline(always)]
    pub fn notsupported(self) -> &'a mut crate::W<REG> {
        self.variant(Suspres::Notsupported)
    }
    #[doc = "Suspend / Resume Supported"]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Suspres::Supported)
    }
}
#[doc = "Desc\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Volt33v {
    #[doc = "0: 3.3 V Not Supported"]
    Notsupported = 0,
    #[doc = "1: 3.3 V Supported"]
    Supported = 1,
}
impl From<Volt33v> for bool {
    #[inline(always)]
    fn from(variant: Volt33v) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOLT33V` reader - Desc"]
pub type Volt33vR = crate::BitReader<Volt33v>;
impl Volt33vR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Volt33v {
        match self.bits {
            false => Volt33v::Notsupported,
            true => Volt33v::Supported,
        }
    }
    #[doc = "3.3 V Not Supported"]
    #[inline(always)]
    pub fn is_notsupported(&self) -> bool {
        *self == Volt33v::Notsupported
    }
    #[doc = "3.3 V Supported"]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Volt33v::Supported
    }
}
#[doc = "Field `VOLT33V` writer - Desc"]
pub type Volt33vW<'a, REG> = crate::BitWriter<'a, REG, Volt33v>;
impl<'a, REG> Volt33vW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "3.3 V Not Supported"]
    #[inline(always)]
    pub fn notsupported(self) -> &'a mut crate::W<REG> {
        self.variant(Volt33v::Notsupported)
    }
    #[doc = "3.3 V Supported"]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Volt33v::Supported)
    }
}
#[doc = "Voltage support 3.0v\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Volt30v {
    #[doc = "0: 3.0 V Not Supported"]
    Notsupported = 0,
    #[doc = "1: 3.0 V Supported"]
    Supported = 1,
}
impl From<Volt30v> for bool {
    #[inline(always)]
    fn from(variant: Volt30v) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOLT30V` reader - Voltage support 3.0v"]
pub type Volt30vR = crate::BitReader<Volt30v>;
impl Volt30vR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Volt30v {
        match self.bits {
            false => Volt30v::Notsupported,
            true => Volt30v::Supported,
        }
    }
    #[doc = "3.0 V Not Supported"]
    #[inline(always)]
    pub fn is_notsupported(&self) -> bool {
        *self == Volt30v::Notsupported
    }
    #[doc = "3.0 V Supported"]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Volt30v::Supported
    }
}
#[doc = "Field `VOLT30V` writer - Voltage support 3.0v"]
pub type Volt30vW<'a, REG> = crate::BitWriter<'a, REG, Volt30v>;
impl<'a, REG> Volt30vW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "3.0 V Not Supported"]
    #[inline(always)]
    pub fn notsupported(self) -> &'a mut crate::W<REG> {
        self.variant(Volt30v::Notsupported)
    }
    #[doc = "3.0 V Supported"]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Volt30v::Supported)
    }
}
#[doc = "Voltage support 1.8v\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Volt18v {
    #[doc = "0: 1.8 V Not Supported"]
    Notsupported = 0,
    #[doc = "1: 1.8 V Supported"]
    Supported = 1,
}
impl From<Volt18v> for bool {
    #[inline(always)]
    fn from(variant: Volt18v) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOLT18V` reader - Voltage support 1.8v"]
pub type Volt18vR = crate::BitReader<Volt18v>;
impl Volt18vR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Volt18v {
        match self.bits {
            false => Volt18v::Notsupported,
            true => Volt18v::Supported,
        }
    }
    #[doc = "1.8 V Not Supported"]
    #[inline(always)]
    pub fn is_notsupported(&self) -> bool {
        *self == Volt18v::Notsupported
    }
    #[doc = "1.8 V Supported"]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Volt18v::Supported
    }
}
#[doc = "Field `VOLT18V` writer - Voltage support 1.8v"]
pub type Volt18vW<'a, REG> = crate::BitWriter<'a, REG, Volt18v>;
impl<'a, REG> Volt18vW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1.8 V Not Supported"]
    #[inline(always)]
    pub fn notsupported(self) -> &'a mut crate::W<REG> {
        self.variant(Volt18v::Notsupported)
    }
    #[doc = "1.8 V Supported"]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Volt18v::Supported)
    }
}
#[doc = "Desc\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sysbus64 {
    #[doc = "1: Supports 64 bit system address"]
    Supported = 1,
    #[doc = "0: Does not support 64 bit system address"]
    Notsupported = 0,
}
impl From<Sysbus64> for bool {
    #[inline(always)]
    fn from(variant: Sysbus64) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSBUS64` reader - Desc"]
pub type Sysbus64R = crate::BitReader<Sysbus64>;
impl Sysbus64R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sysbus64 {
        match self.bits {
            true => Sysbus64::Supported,
            false => Sysbus64::Notsupported,
        }
    }
    #[doc = "Supports 64 bit system address"]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Sysbus64::Supported
    }
    #[doc = "Does not support 64 bit system address"]
    #[inline(always)]
    pub fn is_notsupported(&self) -> bool {
        *self == Sysbus64::Notsupported
    }
}
#[doc = "Field `SYSBUS64` writer - Desc"]
pub type Sysbus64W<'a, REG> = crate::BitWriter<'a, REG, Sysbus64>;
impl<'a, REG> Sysbus64W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Supports 64 bit system address"]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Sysbus64::Supported)
    }
    #[doc = "Does not support 64 bit system address"]
    #[inline(always)]
    pub fn notsupported(self) -> &'a mut crate::W<REG> {
        self.variant(Sysbus64::Notsupported)
    }
}
#[doc = "Refer to SDIO Specification Version 3.00 about asynchronous interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asyncint {
    #[doc = "1: Asynchronous Interrupt Supported"]
    Supported = 1,
    #[doc = "0: Asynchronous Interrupt Not Supported"]
    Notsupported = 0,
}
impl From<Asyncint> for bool {
    #[inline(always)]
    fn from(variant: Asyncint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASYNCINT` reader - Refer to SDIO Specification Version 3.00 about asynchronous interrupt."]
pub type AsyncintR = crate::BitReader<Asyncint>;
impl AsyncintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Asyncint {
        match self.bits {
            true => Asyncint::Supported,
            false => Asyncint::Notsupported,
        }
    }
    #[doc = "Asynchronous Interrupt Supported"]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Asyncint::Supported
    }
    #[doc = "Asynchronous Interrupt Not Supported"]
    #[inline(always)]
    pub fn is_notsupported(&self) -> bool {
        *self == Asyncint::Notsupported
    }
}
#[doc = "Field `ASYNCINT` writer - Refer to SDIO Specification Version 3.00 about asynchronous interrupt."]
pub type AsyncintW<'a, REG> = crate::BitWriter<'a, REG, Asyncint>;
impl<'a, REG> AsyncintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Asynchronous Interrupt Supported"]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Asyncint::Supported)
    }
    #[doc = "Asynchronous Interrupt Not Supported"]
    #[inline(always)]
    pub fn notsupported(self) -> &'a mut crate::W<REG> {
        self.variant(Asyncint::Notsupported)
    }
}
#[doc = "This field indicates usage of a slot by a specific Host System. (A host controller register set is defined per slot.) Embedded slot for one device (01b) means that only one non-removable device is connected to a SD bus slot. Shared Bus Slot (10b) can be set if Host Controller supports Shared Bus Control register. The Standard Host Driver controls only a removable card or one embedded device is connected to a SD bus slot. If a slot is configured for shared bus (10b), the Standard Host Driver does not control embedded devices connected to a shared bus. Shared bus slot is controlled by a specific host driver developed by a Host System.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Slottype {
    #[doc = "0: Removable card slot"]
    Removable = 0,
    #[doc = "1: Embedded Slot for One Device"]
    Embedded = 1,
    #[doc = "2: Shared Bus Slot"]
    Shared = 2,
}
impl From<Slottype> for u8 {
    #[inline(always)]
    fn from(variant: Slottype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Slottype {
    type Ux = u8;
}
impl crate::IsEnum for Slottype {}
#[doc = "Field `SLOTTYPE` reader - This field indicates usage of a slot by a specific Host System. (A host controller register set is defined per slot.) Embedded slot for one device (01b) means that only one non-removable device is connected to a SD bus slot. Shared Bus Slot (10b) can be set if Host Controller supports Shared Bus Control register. The Standard Host Driver controls only a removable card or one embedded device is connected to a SD bus slot. If a slot is configured for shared bus (10b), the Standard Host Driver does not control embedded devices connected to a shared bus. Shared bus slot is controlled by a specific host driver developed by a Host System."]
pub type SlottypeR = crate::FieldReader<Slottype>;
impl SlottypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Slottype> {
        match self.bits {
            0 => Some(Slottype::Removable),
            1 => Some(Slottype::Embedded),
            2 => Some(Slottype::Shared),
            _ => None,
        }
    }
    #[doc = "Removable card slot"]
    #[inline(always)]
    pub fn is_removable(&self) -> bool {
        *self == Slottype::Removable
    }
    #[doc = "Embedded Slot for One Device"]
    #[inline(always)]
    pub fn is_embedded(&self) -> bool {
        *self == Slottype::Embedded
    }
    #[doc = "Shared Bus Slot"]
    #[inline(always)]
    pub fn is_shared(&self) -> bool {
        *self == Slottype::Shared
    }
}
#[doc = "Field `SLOTTYPE` writer - This field indicates usage of a slot by a specific Host System. (A host controller register set is defined per slot.) Embedded slot for one device (01b) means that only one non-removable device is connected to a SD bus slot. Shared Bus Slot (10b) can be set if Host Controller supports Shared Bus Control register. The Standard Host Driver controls only a removable card or one embedded device is connected to a SD bus slot. If a slot is configured for shared bus (10b), the Standard Host Driver does not control embedded devices connected to a shared bus. Shared bus slot is controlled by a specific host driver developed by a Host System."]
pub type SlottypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Slottype>;
impl<'a, REG> SlottypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Removable card slot"]
    #[inline(always)]
    pub fn removable(self) -> &'a mut crate::W<REG> {
        self.variant(Slottype::Removable)
    }
    #[doc = "Embedded Slot for One Device"]
    #[inline(always)]
    pub fn embedded(self) -> &'a mut crate::W<REG> {
        self.variant(Slottype::Embedded)
    }
    #[doc = "Shared Bus Slot"]
    #[inline(always)]
    pub fn shared(self) -> &'a mut crate::W<REG> {
        self.variant(Slottype::Shared)
    }
}
impl R {
    #[doc = "Bits 0:5 - This bit shows the base clock frequency used to detect Data Timeout Error. Not 0 - 1Khz to 63Khz or 1Mhz to 63Mhz Note: The Host System shall support at least one of these voltages above. The HD sets the SD Bus Voltage Select in Power Control register according to these support bits. If multiple voltages are supported, select the usable lower voltage by comparing the OCR value from the card. These registers indicate maximum current capability for each voltage. The value is meaningful if Voltage Support is set in the Capabilities register."]
    #[inline(always)]
    pub fn toclkfreq(&self) -> ToclkfreqR {
        ToclkfreqR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - This bit shows the unit of base clock frequency used to detect Data Timeout Error."]
    #[inline(always)]
    pub fn toclkunit(&self) -> ToclkunitR {
        ToclkunitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - 6-bit Base Clock Frequency This mode is supported by the Host Controller Version 1.00 and 2.00. Upper 2-bit is not effective and always 0. Unit values are 1MHz. The supported clock range is 10MHz to 63MHz. 11xx xxxxb Not supported 0011 1111b 63MHz 0000 0010b 2MHz 0000 0001b 1MHz 0000 0000b Get information via another method (2) 8-bit Base Clock Frequency This mode is supported by the Host Controller Version 3.00.Unit values are 1MHz. The supported clock range is 10MHz to 255MHz. FFh 255MHz 02h 2MHz 01h 1MHz 00h Get information via another method If the real frequency is 16.5MHz, the lager value shall be set 0001 0001b (17MHz) because the Host Driver use this value to calculate the clock divider value (Refer to the SDCLK Frequency Select in the Clock Control register.) and it shall not exceed upper limit of the SD Clock frequency. If these bits are all 0, the Host System has to get information via another method. Some example frequency values are enumerated below."]
    #[inline(always)]
    pub fn sdclkfreq(&self) -> SdclkfreqR {
        SdclkfreqR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - This value indicates the maximum block size that the HD can read and write to the buffer in the HC. The buffer shall transfer this block size without wait cycles. Three sizes can be defined as indicated below."]
    #[inline(always)]
    pub fn maxblklen(&self) -> MaxblklenR {
        MaxblklenR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - This bit indicates whether the Host Controller is capable of using 8-bit bus width mode. This bit is not effective when Slot Type is set to 10b. In this case, refer to Bus Width Preset in the Shared Bus resister. Supported"]
    #[inline(always)]
    pub fn extmedia(&self) -> ExtmediaR {
        ExtmediaR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Desc"]
    #[inline(always)]
    pub fn adma2(&self) -> Adma2R {
        Adma2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - This bit indicates whether the HC and the Host System support High Speed mode and they can supply SD Clock frequency from 25Mhz to 50 Mhz (for SD)/ 20MHz to 52MHz (for MMC)."]
    #[inline(always)]
    pub fn highspeed(&self) -> HighspeedR {
        HighspeedR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - This bit indicates whether the HC is capable of using DMA to transfer data between system memory and the HC directly."]
    #[inline(always)]
    pub fn sdma(&self) -> SdmaR {
        SdmaR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - This bit indicates whether the HC supports Suspend / Resume functionality. If this bit is 0, the Suspend and Resume mechanism are not supported and the HD shall not issue either Suspend / Resume commands."]
    #[inline(always)]
    pub fn suspres(&self) -> SuspresR {
        SuspresR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Desc"]
    #[inline(always)]
    pub fn volt33v(&self) -> Volt33vR {
        Volt33vR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Voltage support 3.0v"]
    #[inline(always)]
    pub fn volt30v(&self) -> Volt30vR {
        Volt30vR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Voltage support 1.8v"]
    #[inline(always)]
    pub fn volt18v(&self) -> Volt18vR {
        Volt18vR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Desc"]
    #[inline(always)]
    pub fn sysbus64(&self) -> Sysbus64R {
        Sysbus64R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Refer to SDIO Specification Version 3.00 about asynchronous interrupt."]
    #[inline(always)]
    pub fn asyncint(&self) -> AsyncintR {
        AsyncintR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - This field indicates usage of a slot by a specific Host System. (A host controller register set is defined per slot.) Embedded slot for one device (01b) means that only one non-removable device is connected to a SD bus slot. Shared Bus Slot (10b) can be set if Host Controller supports Shared Bus Control register. The Standard Host Driver controls only a removable card or one embedded device is connected to a SD bus slot. If a slot is configured for shared bus (10b), the Standard Host Driver does not control embedded devices connected to a shared bus. Shared bus slot is controlled by a specific host driver developed by a Host System."]
    #[inline(always)]
    pub fn slottype(&self) -> SlottypeR {
        SlottypeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - This bit shows the base clock frequency used to detect Data Timeout Error. Not 0 - 1Khz to 63Khz or 1Mhz to 63Mhz Note: The Host System shall support at least one of these voltages above. The HD sets the SD Bus Voltage Select in Power Control register according to these support bits. If multiple voltages are supported, select the usable lower voltage by comparing the OCR value from the card. These registers indicate maximum current capability for each voltage. The value is meaningful if Voltage Support is set in the Capabilities register."]
    #[inline(always)]
    #[must_use]
    pub fn toclkfreq(&mut self) -> ToclkfreqW<Capabilities0Spec> {
        ToclkfreqW::new(self, 0)
    }
    #[doc = "Bit 7 - This bit shows the unit of base clock frequency used to detect Data Timeout Error."]
    #[inline(always)]
    #[must_use]
    pub fn toclkunit(&mut self) -> ToclkunitW<Capabilities0Spec> {
        ToclkunitW::new(self, 7)
    }
    #[doc = "Bits 8:15 - 6-bit Base Clock Frequency This mode is supported by the Host Controller Version 1.00 and 2.00. Upper 2-bit is not effective and always 0. Unit values are 1MHz. The supported clock range is 10MHz to 63MHz. 11xx xxxxb Not supported 0011 1111b 63MHz 0000 0010b 2MHz 0000 0001b 1MHz 0000 0000b Get information via another method (2) 8-bit Base Clock Frequency This mode is supported by the Host Controller Version 3.00.Unit values are 1MHz. The supported clock range is 10MHz to 255MHz. FFh 255MHz 02h 2MHz 01h 1MHz 00h Get information via another method If the real frequency is 16.5MHz, the lager value shall be set 0001 0001b (17MHz) because the Host Driver use this value to calculate the clock divider value (Refer to the SDCLK Frequency Select in the Clock Control register.) and it shall not exceed upper limit of the SD Clock frequency. If these bits are all 0, the Host System has to get information via another method. Some example frequency values are enumerated below."]
    #[inline(always)]
    #[must_use]
    pub fn sdclkfreq(&mut self) -> SdclkfreqW<Capabilities0Spec> {
        SdclkfreqW::new(self, 8)
    }
    #[doc = "Bits 16:17 - This value indicates the maximum block size that the HD can read and write to the buffer in the HC. The buffer shall transfer this block size without wait cycles. Three sizes can be defined as indicated below."]
    #[inline(always)]
    #[must_use]
    pub fn maxblklen(&mut self) -> MaxblklenW<Capabilities0Spec> {
        MaxblklenW::new(self, 16)
    }
    #[doc = "Bit 18 - This bit indicates whether the Host Controller is capable of using 8-bit bus width mode. This bit is not effective when Slot Type is set to 10b. In this case, refer to Bus Width Preset in the Shared Bus resister. Supported"]
    #[inline(always)]
    #[must_use]
    pub fn extmedia(&mut self) -> ExtmediaW<Capabilities0Spec> {
        ExtmediaW::new(self, 18)
    }
    #[doc = "Bit 19 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn adma2(&mut self) -> Adma2W<Capabilities0Spec> {
        Adma2W::new(self, 19)
    }
    #[doc = "Bit 21 - This bit indicates whether the HC and the Host System support High Speed mode and they can supply SD Clock frequency from 25Mhz to 50 Mhz (for SD)/ 20MHz to 52MHz (for MMC)."]
    #[inline(always)]
    #[must_use]
    pub fn highspeed(&mut self) -> HighspeedW<Capabilities0Spec> {
        HighspeedW::new(self, 21)
    }
    #[doc = "Bit 22 - This bit indicates whether the HC is capable of using DMA to transfer data between system memory and the HC directly."]
    #[inline(always)]
    #[must_use]
    pub fn sdma(&mut self) -> SdmaW<Capabilities0Spec> {
        SdmaW::new(self, 22)
    }
    #[doc = "Bit 23 - This bit indicates whether the HC supports Suspend / Resume functionality. If this bit is 0, the Suspend and Resume mechanism are not supported and the HD shall not issue either Suspend / Resume commands."]
    #[inline(always)]
    #[must_use]
    pub fn suspres(&mut self) -> SuspresW<Capabilities0Spec> {
        SuspresW::new(self, 23)
    }
    #[doc = "Bit 24 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn volt33v(&mut self) -> Volt33vW<Capabilities0Spec> {
        Volt33vW::new(self, 24)
    }
    #[doc = "Bit 25 - Voltage support 3.0v"]
    #[inline(always)]
    #[must_use]
    pub fn volt30v(&mut self) -> Volt30vW<Capabilities0Spec> {
        Volt30vW::new(self, 25)
    }
    #[doc = "Bit 26 - Voltage support 1.8v"]
    #[inline(always)]
    #[must_use]
    pub fn volt18v(&mut self) -> Volt18vW<Capabilities0Spec> {
        Volt18vW::new(self, 26)
    }
    #[doc = "Bit 28 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn sysbus64(&mut self) -> Sysbus64W<Capabilities0Spec> {
        Sysbus64W::new(self, 28)
    }
    #[doc = "Bit 29 - Refer to SDIO Specification Version 3.00 about asynchronous interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn asyncint(&mut self) -> AsyncintW<Capabilities0Spec> {
        AsyncintW::new(self, 29)
    }
    #[doc = "Bits 30:31 - This field indicates usage of a slot by a specific Host System. (A host controller register set is defined per slot.) Embedded slot for one device (01b) means that only one non-removable device is connected to a SD bus slot. Shared Bus Slot (10b) can be set if Host Controller supports Shared Bus Control register. The Standard Host Driver controls only a removable card or one embedded device is connected to a SD bus slot. If a slot is configured for shared bus (10b), the Standard Host Driver does not control embedded devices connected to a shared bus. Shared bus slot is controlled by a specific host driver developed by a Host System."]
    #[inline(always)]
    #[must_use]
    pub fn slottype(&mut self) -> SlottypeW<Capabilities0Spec> {
        SlottypeW::new(self, 30)
    }
}
#[doc = "Capabilities\n\nYou can [`read`](crate::Reg::read) this register and get [`capabilities0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capabilities0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Capabilities0Spec;
impl crate::RegisterSpec for Capabilities0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capabilities0::R`](R) reader structure"]
impl crate::Readable for Capabilities0Spec {}
#[doc = "`write(|w| ..)` method takes [`capabilities0::W`](W) writer structure"]
impl crate::Writable for Capabilities0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAPABILITIES0 to value 0"]
impl crate::Resettable for Capabilities0Spec {
    const RESET_VALUE: u32 = 0;
}
