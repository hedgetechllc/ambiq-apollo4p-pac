#[doc = "Register `MISC` reader"]
pub type R = crate::R<MiscSpec>;
#[doc = "Register `MISC` writer"]
pub type W = crate::W<MiscSpec>;
#[doc = "Force HFRC On .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frchfrc {
    #[doc = "0: HFRC stops in deep sleep mode"]
    Nofrc = 0,
    #[doc = "1: HFRC runs in deep sleep mode"]
    Frc = 1,
}
impl From<Frchfrc> for bool {
    #[inline(always)]
    fn from(variant: Frchfrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRCHFRC` reader - Force HFRC On ."]
pub type FrchfrcR = crate::BitReader<Frchfrc>;
impl FrchfrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frchfrc {
        match self.bits {
            false => Frchfrc::Nofrc,
            true => Frchfrc::Frc,
        }
    }
    #[doc = "HFRC stops in deep sleep mode"]
    #[inline(always)]
    pub fn is_nofrc(&self) -> bool {
        *self == Frchfrc::Nofrc
    }
    #[doc = "HFRC runs in deep sleep mode"]
    #[inline(always)]
    pub fn is_frc(&self) -> bool {
        *self == Frchfrc::Frc
    }
}
#[doc = "Field `FRCHFRC` writer - Force HFRC On ."]
pub type FrchfrcW<'a, REG> = crate::BitWriter<'a, REG, Frchfrc>;
impl<'a, REG> FrchfrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HFRC stops in deep sleep mode"]
    #[inline(always)]
    pub fn nofrc(self) -> &'a mut crate::W<REG> {
        self.variant(Frchfrc::Nofrc)
    }
    #[doc = "HFRC runs in deep sleep mode"]
    #[inline(always)]
    pub fn frc(self) -> &'a mut crate::W<REG> {
        self.variant(Frchfrc::Frc)
    }
}
#[doc = "Force fclk, hclk, fclk_wic and fclk_pmu to be turned off during burst transition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frcburstoff {
    #[doc = "0: fclk, hclk and fclk_wic are turned on during the burst transition"]
    Burstclkon = 0,
    #[doc = "1: fclk, hclk and fclk are turned off during burst transition"]
    Burstclkoff = 1,
}
impl From<Frcburstoff> for bool {
    #[inline(always)]
    fn from(variant: Frcburstoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRCBURSTOFF` reader - Force fclk, hclk, fclk_wic and fclk_pmu to be turned off during burst transition."]
pub type FrcburstoffR = crate::BitReader<Frcburstoff>;
impl FrcburstoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frcburstoff {
        match self.bits {
            false => Frcburstoff::Burstclkon,
            true => Frcburstoff::Burstclkoff,
        }
    }
    #[doc = "fclk, hclk and fclk_wic are turned on during the burst transition"]
    #[inline(always)]
    pub fn is_burstclkon(&self) -> bool {
        *self == Frcburstoff::Burstclkon
    }
    #[doc = "fclk, hclk and fclk are turned off during burst transition"]
    #[inline(always)]
    pub fn is_burstclkoff(&self) -> bool {
        *self == Frcburstoff::Burstclkoff
    }
}
#[doc = "Field `FRCBURSTOFF` writer - Force fclk, hclk, fclk_wic and fclk_pmu to be turned off during burst transition."]
pub type FrcburstoffW<'a, REG> = crate::BitWriter<'a, REG, Frcburstoff>;
impl<'a, REG> FrcburstoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fclk, hclk and fclk_wic are turned on during the burst transition"]
    #[inline(always)]
    pub fn burstclkon(self) -> &'a mut crate::W<REG> {
        self.variant(Frcburstoff::Burstclkon)
    }
    #[doc = "fclk, hclk and fclk are turned off during burst transition"]
    #[inline(always)]
    pub fn burstclkoff(self) -> &'a mut crate::W<REG> {
        self.variant(Frcburstoff::Burstclkoff)
    }
}
#[doc = "Use HFRC-48MHz or HFRC2-48MHz for DSP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usehfrc2fq48mhz {
    #[doc = "0: Use HFRC-48MHz"]
    Hfrcfq48mhz = 0,
    #[doc = "1: Use HFRC2-48MHz"]
    Hfrc2fq48mhz = 1,
}
impl From<Usehfrc2fq48mhz> for bool {
    #[inline(always)]
    fn from(variant: Usehfrc2fq48mhz) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USEHFRC2FQ48MHZ` reader - Use HFRC-48MHz or HFRC2-48MHz for DSP"]
pub type Usehfrc2fq48mhzR = crate::BitReader<Usehfrc2fq48mhz>;
impl Usehfrc2fq48mhzR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usehfrc2fq48mhz {
        match self.bits {
            false => Usehfrc2fq48mhz::Hfrcfq48mhz,
            true => Usehfrc2fq48mhz::Hfrc2fq48mhz,
        }
    }
    #[doc = "Use HFRC-48MHz"]
    #[inline(always)]
    pub fn is_hfrcfq48mhz(&self) -> bool {
        *self == Usehfrc2fq48mhz::Hfrcfq48mhz
    }
    #[doc = "Use HFRC2-48MHz"]
    #[inline(always)]
    pub fn is_hfrc2fq48mhz(&self) -> bool {
        *self == Usehfrc2fq48mhz::Hfrc2fq48mhz
    }
}
#[doc = "Field `USEHFRC2FQ48MHZ` writer - Use HFRC-48MHz or HFRC2-48MHz for DSP"]
pub type Usehfrc2fq48mhzW<'a, REG> = crate::BitWriter<'a, REG, Usehfrc2fq48mhz>;
impl<'a, REG> Usehfrc2fq48mhzW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use HFRC-48MHz"]
    #[inline(always)]
    pub fn hfrcfq48mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Usehfrc2fq48mhz::Hfrcfq48mhz)
    }
    #[doc = "Use HFRC2-48MHz"]
    #[inline(always)]
    pub fn hfrc2fq48mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Usehfrc2fq48mhz::Hfrc2fq48mhz)
    }
}
#[doc = "Use HFRC-96MHz or HFRC2-96MHz for DSP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usehfrc2fq96mhz {
    #[doc = "0: Use HFRC-96MHz"]
    Hfrcfq96mhz = 0,
    #[doc = "1: Use HFRC2-96MHz"]
    Hfrc2fq96mhz = 1,
}
impl From<Usehfrc2fq96mhz> for bool {
    #[inline(always)]
    fn from(variant: Usehfrc2fq96mhz) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USEHFRC2FQ96MHZ` reader - Use HFRC-96MHz or HFRC2-96MHz for DSP"]
pub type Usehfrc2fq96mhzR = crate::BitReader<Usehfrc2fq96mhz>;
impl Usehfrc2fq96mhzR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usehfrc2fq96mhz {
        match self.bits {
            false => Usehfrc2fq96mhz::Hfrcfq96mhz,
            true => Usehfrc2fq96mhz::Hfrc2fq96mhz,
        }
    }
    #[doc = "Use HFRC-96MHz"]
    #[inline(always)]
    pub fn is_hfrcfq96mhz(&self) -> bool {
        *self == Usehfrc2fq96mhz::Hfrcfq96mhz
    }
    #[doc = "Use HFRC2-96MHz"]
    #[inline(always)]
    pub fn is_hfrc2fq96mhz(&self) -> bool {
        *self == Usehfrc2fq96mhz::Hfrc2fq96mhz
    }
}
#[doc = "Field `USEHFRC2FQ96MHZ` writer - Use HFRC-96MHz or HFRC2-96MHz for DSP"]
pub type Usehfrc2fq96mhzW<'a, REG> = crate::BitWriter<'a, REG, Usehfrc2fq96mhz>;
impl<'a, REG> Usehfrc2fq96mhzW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use HFRC-96MHz"]
    #[inline(always)]
    pub fn hfrcfq96mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Usehfrc2fq96mhz::Hfrcfq96mhz)
    }
    #[doc = "Use HFRC2-96MHz"]
    #[inline(always)]
    pub fn hfrc2fq96mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Usehfrc2fq96mhz::Hfrc2fq96mhz)
    }
}
#[doc = "Use HFRC-192MHz or HFRC2-192MHz for MCU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usehfrc2fq192mhz {
    #[doc = "0: Use HFRC-192MHz"]
    Hfrcfq192mhz = 0,
    #[doc = "1: Use HFRC2-192MHz"]
    Hfrc2fq192mhz = 1,
}
impl From<Usehfrc2fq192mhz> for bool {
    #[inline(always)]
    fn from(variant: Usehfrc2fq192mhz) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USEHFRC2FQ192MHZ` reader - Use HFRC-192MHz or HFRC2-192MHz for MCU"]
pub type Usehfrc2fq192mhzR = crate::BitReader<Usehfrc2fq192mhz>;
impl Usehfrc2fq192mhzR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usehfrc2fq192mhz {
        match self.bits {
            false => Usehfrc2fq192mhz::Hfrcfq192mhz,
            true => Usehfrc2fq192mhz::Hfrc2fq192mhz,
        }
    }
    #[doc = "Use HFRC-192MHz"]
    #[inline(always)]
    pub fn is_hfrcfq192mhz(&self) -> bool {
        *self == Usehfrc2fq192mhz::Hfrcfq192mhz
    }
    #[doc = "Use HFRC2-192MHz"]
    #[inline(always)]
    pub fn is_hfrc2fq192mhz(&self) -> bool {
        *self == Usehfrc2fq192mhz::Hfrc2fq192mhz
    }
}
#[doc = "Field `USEHFRC2FQ192MHZ` writer - Use HFRC-192MHz or HFRC2-192MHz for MCU"]
pub type Usehfrc2fq192mhzW<'a, REG> = crate::BitWriter<'a, REG, Usehfrc2fq192mhz>;
impl<'a, REG> Usehfrc2fq192mhzW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use HFRC-192MHz"]
    #[inline(always)]
    pub fn hfrcfq192mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Usehfrc2fq192mhz::Hfrcfq192mhz)
    }
    #[doc = "Use HFRC2-192MHz"]
    #[inline(always)]
    pub fn hfrc2fq192mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Usehfrc2fq192mhz::Hfrc2fq192mhz)
    }
}
#[doc = "Force HFRC2 On.Setting this bit forces HFRC2 to remain on, including in deep sleep. When changing a module's clock source to HFRC2, this bit must be set and remain set when any module is using HFRC2 as its clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frchfrc2 {
    #[doc = "0: Do not force HFRC2 on; stops in deep sleep mode."]
    Nofrc = 0,
    #[doc = "1: Force HFRC2 on; runs in deep sleep mode."]
    Frc = 1,
}
impl From<Frchfrc2> for bool {
    #[inline(always)]
    fn from(variant: Frchfrc2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRCHFRC2` reader - Force HFRC2 On.Setting this bit forces HFRC2 to remain on, including in deep sleep. When changing a module's clock source to HFRC2, this bit must be set and remain set when any module is using HFRC2 as its clock."]
pub type Frchfrc2R = crate::BitReader<Frchfrc2>;
impl Frchfrc2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frchfrc2 {
        match self.bits {
            false => Frchfrc2::Nofrc,
            true => Frchfrc2::Frc,
        }
    }
    #[doc = "Do not force HFRC2 on; stops in deep sleep mode."]
    #[inline(always)]
    pub fn is_nofrc(&self) -> bool {
        *self == Frchfrc2::Nofrc
    }
    #[doc = "Force HFRC2 on; runs in deep sleep mode."]
    #[inline(always)]
    pub fn is_frc(&self) -> bool {
        *self == Frchfrc2::Frc
    }
}
#[doc = "Field `FRCHFRC2` writer - Force HFRC2 On.Setting this bit forces HFRC2 to remain on, including in deep sleep. When changing a module's clock source to HFRC2, this bit must be set and remain set when any module is using HFRC2 as its clock."]
pub type Frchfrc2W<'a, REG> = crate::BitWriter<'a, REG, Frchfrc2>;
impl<'a, REG> Frchfrc2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not force HFRC2 on; stops in deep sleep mode."]
    #[inline(always)]
    pub fn nofrc(self) -> &'a mut crate::W<REG> {
        self.variant(Frchfrc2::Nofrc)
    }
    #[doc = "Force HFRC2 on; runs in deep sleep mode."]
    #[inline(always)]
    pub fn frc(self) -> &'a mut crate::W<REG> {
        self.variant(Frchfrc2::Frc)
    }
}
#[doc = "Chicken bit to disable Rev B clock enable during reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwronclkendisp {
    #[doc = "0: Enable Display Clock to run during reset"]
    Dispclkenrst = 0,
    #[doc = "1: Chicken bit to revert to rev A"]
    Defeature = 1,
}
impl From<Pwronclkendisp> for bool {
    #[inline(always)]
    fn from(variant: Pwronclkendisp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRONCLKENDISP` reader - Chicken bit to disable Rev B clock enable during reset"]
pub type PwronclkendispR = crate::BitReader<Pwronclkendisp>;
impl PwronclkendispR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwronclkendisp {
        match self.bits {
            false => Pwronclkendisp::Dispclkenrst,
            true => Pwronclkendisp::Defeature,
        }
    }
    #[doc = "Enable Display Clock to run during reset"]
    #[inline(always)]
    pub fn is_dispclkenrst(&self) -> bool {
        *self == Pwronclkendisp::Dispclkenrst
    }
    #[doc = "Chicken bit to revert to rev A"]
    #[inline(always)]
    pub fn is_defeature(&self) -> bool {
        *self == Pwronclkendisp::Defeature
    }
}
#[doc = "Field `PWRONCLKENDISP` writer - Chicken bit to disable Rev B clock enable during reset"]
pub type PwronclkendispW<'a, REG> = crate::BitWriter<'a, REG, Pwronclkendisp>;
impl<'a, REG> PwronclkendispW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Display Clock to run during reset"]
    #[inline(always)]
    pub fn dispclkenrst(self) -> &'a mut crate::W<REG> {
        self.variant(Pwronclkendisp::Dispclkenrst)
    }
    #[doc = "Chicken bit to revert to rev A"]
    #[inline(always)]
    pub fn defeature(self) -> &'a mut crate::W<REG> {
        self.variant(Pwronclkendisp::Defeature)
    }
}
#[doc = "Chicken bit to disable Rev B clock enable during reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwronclkendispphy {
    #[doc = "0: Enable Display Phy Clock to run during reset"]
    Dispphyclkenrst = 0,
    #[doc = "1: Chicken bit to revert to rev A"]
    Defeature = 1,
}
impl From<Pwronclkendispphy> for bool {
    #[inline(always)]
    fn from(variant: Pwronclkendispphy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRONCLKENDISPPHY` reader - Chicken bit to disable Rev B clock enable during reset"]
pub type PwronclkendispphyR = crate::BitReader<Pwronclkendispphy>;
impl PwronclkendispphyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwronclkendispphy {
        match self.bits {
            false => Pwronclkendispphy::Dispphyclkenrst,
            true => Pwronclkendispphy::Defeature,
        }
    }
    #[doc = "Enable Display Phy Clock to run during reset"]
    #[inline(always)]
    pub fn is_dispphyclkenrst(&self) -> bool {
        *self == Pwronclkendispphy::Dispphyclkenrst
    }
    #[doc = "Chicken bit to revert to rev A"]
    #[inline(always)]
    pub fn is_defeature(&self) -> bool {
        *self == Pwronclkendispphy::Defeature
    }
}
#[doc = "Field `PWRONCLKENDISPPHY` writer - Chicken bit to disable Rev B clock enable during reset"]
pub type PwronclkendispphyW<'a, REG> = crate::BitWriter<'a, REG, Pwronclkendispphy>;
impl<'a, REG> PwronclkendispphyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Display Phy Clock to run during reset"]
    #[inline(always)]
    pub fn dispphyclkenrst(self) -> &'a mut crate::W<REG> {
        self.variant(Pwronclkendispphy::Dispphyclkenrst)
    }
    #[doc = "Chicken bit to revert to rev A"]
    #[inline(always)]
    pub fn defeature(self) -> &'a mut crate::W<REG> {
        self.variant(Pwronclkendispphy::Defeature)
    }
}
#[doc = "Chicken bit to disable Rev B clock enable during reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwronclkengfx {
    #[doc = "0: Enable GFX Clock to run during reset"]
    Gfxclkenrst = 0,
    #[doc = "1: Chicken bit to revert to rev A"]
    Defeature = 1,
}
impl From<Pwronclkengfx> for bool {
    #[inline(always)]
    fn from(variant: Pwronclkengfx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRONCLKENGFX` reader - Chicken bit to disable Rev B clock enable during reset"]
pub type PwronclkengfxR = crate::BitReader<Pwronclkengfx>;
impl PwronclkengfxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwronclkengfx {
        match self.bits {
            false => Pwronclkengfx::Gfxclkenrst,
            true => Pwronclkengfx::Defeature,
        }
    }
    #[doc = "Enable GFX Clock to run during reset"]
    #[inline(always)]
    pub fn is_gfxclkenrst(&self) -> bool {
        *self == Pwronclkengfx::Gfxclkenrst
    }
    #[doc = "Chicken bit to revert to rev A"]
    #[inline(always)]
    pub fn is_defeature(&self) -> bool {
        *self == Pwronclkengfx::Defeature
    }
}
#[doc = "Field `PWRONCLKENGFX` writer - Chicken bit to disable Rev B clock enable during reset"]
pub type PwronclkengfxW<'a, REG> = crate::BitWriter<'a, REG, Pwronclkengfx>;
impl<'a, REG> PwronclkengfxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable GFX Clock to run during reset"]
    #[inline(always)]
    pub fn gfxclkenrst(self) -> &'a mut crate::W<REG> {
        self.variant(Pwronclkengfx::Gfxclkenrst)
    }
    #[doc = "Chicken bit to revert to rev A"]
    #[inline(always)]
    pub fn defeature(self) -> &'a mut crate::W<REG> {
        self.variant(Pwronclkengfx::Defeature)
    }
}
#[doc = "Chicken bit to disable Rev B clock enable during reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwronclkenusb {
    #[doc = "0: Enable USB Clock to run during reset"]
    Usbclkenrst = 0,
    #[doc = "1: Chicken bit to revert to rev A"]
    Defeature = 1,
}
impl From<Pwronclkenusb> for bool {
    #[inline(always)]
    fn from(variant: Pwronclkenusb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRONCLKENUSB` reader - Chicken bit to disable Rev B clock enable during reset"]
pub type PwronclkenusbR = crate::BitReader<Pwronclkenusb>;
impl PwronclkenusbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwronclkenusb {
        match self.bits {
            false => Pwronclkenusb::Usbclkenrst,
            true => Pwronclkenusb::Defeature,
        }
    }
    #[doc = "Enable USB Clock to run during reset"]
    #[inline(always)]
    pub fn is_usbclkenrst(&self) -> bool {
        *self == Pwronclkenusb::Usbclkenrst
    }
    #[doc = "Chicken bit to revert to rev A"]
    #[inline(always)]
    pub fn is_defeature(&self) -> bool {
        *self == Pwronclkenusb::Defeature
    }
}
#[doc = "Field `PWRONCLKENUSB` writer - Chicken bit to disable Rev B clock enable during reset"]
pub type PwronclkenusbW<'a, REG> = crate::BitWriter<'a, REG, Pwronclkenusb>;
impl<'a, REG> PwronclkenusbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable USB Clock to run during reset"]
    #[inline(always)]
    pub fn usbclkenrst(self) -> &'a mut crate::W<REG> {
        self.variant(Pwronclkenusb::Usbclkenrst)
    }
    #[doc = "Chicken bit to revert to rev A"]
    #[inline(always)]
    pub fn defeature(self) -> &'a mut crate::W<REG> {
        self.variant(Pwronclkenusb::Defeature)
    }
}
#[doc = "Chicken bit to disable Rev B clock enable during reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwronclkensdio {
    #[doc = "0: Enable SDIO Clock to run during reset"]
    Sdioclkenrst = 0,
    #[doc = "1: Chicken bit to revert to rev A"]
    Defeature = 1,
}
impl From<Pwronclkensdio> for bool {
    #[inline(always)]
    fn from(variant: Pwronclkensdio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRONCLKENSDIO` reader - Chicken bit to disable Rev B clock enable during reset"]
pub type PwronclkensdioR = crate::BitReader<Pwronclkensdio>;
impl PwronclkensdioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwronclkensdio {
        match self.bits {
            false => Pwronclkensdio::Sdioclkenrst,
            true => Pwronclkensdio::Defeature,
        }
    }
    #[doc = "Enable SDIO Clock to run during reset"]
    #[inline(always)]
    pub fn is_sdioclkenrst(&self) -> bool {
        *self == Pwronclkensdio::Sdioclkenrst
    }
    #[doc = "Chicken bit to revert to rev A"]
    #[inline(always)]
    pub fn is_defeature(&self) -> bool {
        *self == Pwronclkensdio::Defeature
    }
}
#[doc = "Field `PWRONCLKENSDIO` writer - Chicken bit to disable Rev B clock enable during reset"]
pub type PwronclkensdioW<'a, REG> = crate::BitWriter<'a, REG, Pwronclkensdio>;
impl<'a, REG> PwronclkensdioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable SDIO Clock to run during reset"]
    #[inline(always)]
    pub fn sdioclkenrst(self) -> &'a mut crate::W<REG> {
        self.variant(Pwronclkensdio::Sdioclkenrst)
    }
    #[doc = "Chicken bit to revert to rev A"]
    #[inline(always)]
    pub fn defeature(self) -> &'a mut crate::W<REG> {
        self.variant(Pwronclkensdio::Defeature)
    }
}
#[doc = "Chicken bit to disable Rev B clock enable during reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwronclkencrypto {
    #[doc = "0: Enable Cryto engine Clock to run during reset"]
    Cryptoclkenrst = 0,
    #[doc = "1: Chicken bit to revert to rev A"]
    Defeature = 1,
}
impl From<Pwronclkencrypto> for bool {
    #[inline(always)]
    fn from(variant: Pwronclkencrypto) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRONCLKENCRYPTO` reader - Chicken bit to disable Rev B clock enable during reset"]
pub type PwronclkencryptoR = crate::BitReader<Pwronclkencrypto>;
impl PwronclkencryptoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwronclkencrypto {
        match self.bits {
            false => Pwronclkencrypto::Cryptoclkenrst,
            true => Pwronclkencrypto::Defeature,
        }
    }
    #[doc = "Enable Cryto engine Clock to run during reset"]
    #[inline(always)]
    pub fn is_cryptoclkenrst(&self) -> bool {
        *self == Pwronclkencrypto::Cryptoclkenrst
    }
    #[doc = "Chicken bit to revert to rev A"]
    #[inline(always)]
    pub fn is_defeature(&self) -> bool {
        *self == Pwronclkencrypto::Defeature
    }
}
#[doc = "Field `PWRONCLKENCRYPTO` writer - Chicken bit to disable Rev B clock enable during reset"]
pub type PwronclkencryptoW<'a, REG> = crate::BitWriter<'a, REG, Pwronclkencrypto>;
impl<'a, REG> PwronclkencryptoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Cryto engine Clock to run during reset"]
    #[inline(always)]
    pub fn cryptoclkenrst(self) -> &'a mut crate::W<REG> {
        self.variant(Pwronclkencrypto::Cryptoclkenrst)
    }
    #[doc = "Chicken bit to revert to rev A"]
    #[inline(always)]
    pub fn defeature(self) -> &'a mut crate::W<REG> {
        self.variant(Pwronclkencrypto::Defeature)
    }
}
#[doc = "Chicken bit to disable Rev B clock enable during reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwronclkeni2s0 {
    #[doc = "0: Enable I2S instance 0 engine Clock to run during reset"]
    I2s0clkenrst = 0,
    #[doc = "1: Chicken bit to revert to rev A"]
    Defeature = 1,
}
impl From<Pwronclkeni2s0> for bool {
    #[inline(always)]
    fn from(variant: Pwronclkeni2s0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRONCLKENI2S0` reader - Chicken bit to disable Rev B clock enable during reset"]
pub type Pwronclkeni2s0R = crate::BitReader<Pwronclkeni2s0>;
impl Pwronclkeni2s0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwronclkeni2s0 {
        match self.bits {
            false => Pwronclkeni2s0::I2s0clkenrst,
            true => Pwronclkeni2s0::Defeature,
        }
    }
    #[doc = "Enable I2S instance 0 engine Clock to run during reset"]
    #[inline(always)]
    pub fn is_i2s0clkenrst(&self) -> bool {
        *self == Pwronclkeni2s0::I2s0clkenrst
    }
    #[doc = "Chicken bit to revert to rev A"]
    #[inline(always)]
    pub fn is_defeature(&self) -> bool {
        *self == Pwronclkeni2s0::Defeature
    }
}
#[doc = "Field `PWRONCLKENI2S0` writer - Chicken bit to disable Rev B clock enable during reset"]
pub type Pwronclkeni2s0W<'a, REG> = crate::BitWriter<'a, REG, Pwronclkeni2s0>;
impl<'a, REG> Pwronclkeni2s0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable I2S instance 0 engine Clock to run during reset"]
    #[inline(always)]
    pub fn i2s0clkenrst(self) -> &'a mut crate::W<REG> {
        self.variant(Pwronclkeni2s0::I2s0clkenrst)
    }
    #[doc = "Chicken bit to revert to rev A"]
    #[inline(always)]
    pub fn defeature(self) -> &'a mut crate::W<REG> {
        self.variant(Pwronclkeni2s0::Defeature)
    }
}
#[doc = "Chicken bit to disable Rev B clock enable during reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwronclkeni2s1 {
    #[doc = "0: Enable I2S instance 1 engine Clock to run during reset"]
    I2s1clkenrst = 0,
    #[doc = "1: Chicken bit to revert to rev A"]
    Defeature = 1,
}
impl From<Pwronclkeni2s1> for bool {
    #[inline(always)]
    fn from(variant: Pwronclkeni2s1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRONCLKENI2S1` reader - Chicken bit to disable Rev B clock enable during reset"]
pub type Pwronclkeni2s1R = crate::BitReader<Pwronclkeni2s1>;
impl Pwronclkeni2s1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwronclkeni2s1 {
        match self.bits {
            false => Pwronclkeni2s1::I2s1clkenrst,
            true => Pwronclkeni2s1::Defeature,
        }
    }
    #[doc = "Enable I2S instance 1 engine Clock to run during reset"]
    #[inline(always)]
    pub fn is_i2s1clkenrst(&self) -> bool {
        *self == Pwronclkeni2s1::I2s1clkenrst
    }
    #[doc = "Chicken bit to revert to rev A"]
    #[inline(always)]
    pub fn is_defeature(&self) -> bool {
        *self == Pwronclkeni2s1::Defeature
    }
}
#[doc = "Field `PWRONCLKENI2S1` writer - Chicken bit to disable Rev B clock enable during reset"]
pub type Pwronclkeni2s1W<'a, REG> = crate::BitWriter<'a, REG, Pwronclkeni2s1>;
impl<'a, REG> Pwronclkeni2s1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable I2S instance 1 engine Clock to run during reset"]
    #[inline(always)]
    pub fn i2s1clkenrst(self) -> &'a mut crate::W<REG> {
        self.variant(Pwronclkeni2s1::I2s1clkenrst)
    }
    #[doc = "Chicken bit to revert to rev A"]
    #[inline(always)]
    pub fn defeature(self) -> &'a mut crate::W<REG> {
        self.variant(Pwronclkeni2s1::Defeature)
    }
}
#[doc = "Chicken bit to disable Rev B added clock gating\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axixaclkenovrride {
    #[doc = "0: Fine grain clock gating enabled"]
    DefeatureDisabled = 0,
    #[doc = "1: AXI Clock enabled when core is not in Sleep mode"]
    Defeature = 1,
}
impl From<Axixaclkenovrride> for bool {
    #[inline(always)]
    fn from(variant: Axixaclkenovrride) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AXIXACLKENOVRRIDE` reader - Chicken bit to disable Rev B added clock gating"]
pub type AxixaclkenovrrideR = crate::BitReader<Axixaclkenovrride>;
impl AxixaclkenovrrideR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Axixaclkenovrride {
        match self.bits {
            false => Axixaclkenovrride::DefeatureDisabled,
            true => Axixaclkenovrride::Defeature,
        }
    }
    #[doc = "Fine grain clock gating enabled"]
    #[inline(always)]
    pub fn is_defeature_disabled(&self) -> bool {
        *self == Axixaclkenovrride::DefeatureDisabled
    }
    #[doc = "AXI Clock enabled when core is not in Sleep mode"]
    #[inline(always)]
    pub fn is_defeature(&self) -> bool {
        *self == Axixaclkenovrride::Defeature
    }
}
#[doc = "Field `AXIXACLKENOVRRIDE` writer - Chicken bit to disable Rev B added clock gating"]
pub type AxixaclkenovrrideW<'a, REG> = crate::BitWriter<'a, REG, Axixaclkenovrride>;
impl<'a, REG> AxixaclkenovrrideW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fine grain clock gating enabled"]
    #[inline(always)]
    pub fn defeature_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Axixaclkenovrride::DefeatureDisabled)
    }
    #[doc = "AXI Clock enabled when core is not in Sleep mode"]
    #[inline(always)]
    pub fn defeature(self) -> &'a mut crate::W<REG> {
        self.variant(Axixaclkenovrride::Defeature)
    }
}
#[doc = "Chicken bit to disable Rev B clock enable during reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwronclkeni2s0refclk {
    #[doc = "0: Enable I2S instance 0 Clock to run during reset"]
    I2s0refclkenrst = 0,
    #[doc = "1: Chicken bit to revert to rev A"]
    Defeature = 1,
}
impl From<Pwronclkeni2s0refclk> for bool {
    #[inline(always)]
    fn from(variant: Pwronclkeni2s0refclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRONCLKENI2S0REFCLK` reader - Chicken bit to disable Rev B clock enable during reset"]
pub type Pwronclkeni2s0refclkR = crate::BitReader<Pwronclkeni2s0refclk>;
impl Pwronclkeni2s0refclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwronclkeni2s0refclk {
        match self.bits {
            false => Pwronclkeni2s0refclk::I2s0refclkenrst,
            true => Pwronclkeni2s0refclk::Defeature,
        }
    }
    #[doc = "Enable I2S instance 0 Clock to run during reset"]
    #[inline(always)]
    pub fn is_i2s0refclkenrst(&self) -> bool {
        *self == Pwronclkeni2s0refclk::I2s0refclkenrst
    }
    #[doc = "Chicken bit to revert to rev A"]
    #[inline(always)]
    pub fn is_defeature(&self) -> bool {
        *self == Pwronclkeni2s0refclk::Defeature
    }
}
#[doc = "Field `PWRONCLKENI2S0REFCLK` writer - Chicken bit to disable Rev B clock enable during reset"]
pub type Pwronclkeni2s0refclkW<'a, REG> = crate::BitWriter<'a, REG, Pwronclkeni2s0refclk>;
impl<'a, REG> Pwronclkeni2s0refclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable I2S instance 0 Clock to run during reset"]
    #[inline(always)]
    pub fn i2s0refclkenrst(self) -> &'a mut crate::W<REG> {
        self.variant(Pwronclkeni2s0refclk::I2s0refclkenrst)
    }
    #[doc = "Chicken bit to revert to rev A"]
    #[inline(always)]
    pub fn defeature(self) -> &'a mut crate::W<REG> {
        self.variant(Pwronclkeni2s0refclk::Defeature)
    }
}
#[doc = "Chicken bit to disable Rev B clock enable during reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwronclkeni2s1refclk {
    #[doc = "0: Enable I2S instance 1 Clock to run during reset"]
    I2s1refclkenrst = 0,
    #[doc = "1: Chicken bit to revert to rev A"]
    Defeature = 1,
}
impl From<Pwronclkeni2s1refclk> for bool {
    #[inline(always)]
    fn from(variant: Pwronclkeni2s1refclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRONCLKENI2S1REFCLK` reader - Chicken bit to disable Rev B clock enable during reset"]
pub type Pwronclkeni2s1refclkR = crate::BitReader<Pwronclkeni2s1refclk>;
impl Pwronclkeni2s1refclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwronclkeni2s1refclk {
        match self.bits {
            false => Pwronclkeni2s1refclk::I2s1refclkenrst,
            true => Pwronclkeni2s1refclk::Defeature,
        }
    }
    #[doc = "Enable I2S instance 1 Clock to run during reset"]
    #[inline(always)]
    pub fn is_i2s1refclkenrst(&self) -> bool {
        *self == Pwronclkeni2s1refclk::I2s1refclkenrst
    }
    #[doc = "Chicken bit to revert to rev A"]
    #[inline(always)]
    pub fn is_defeature(&self) -> bool {
        *self == Pwronclkeni2s1refclk::Defeature
    }
}
#[doc = "Field `PWRONCLKENI2S1REFCLK` writer - Chicken bit to disable Rev B clock enable during reset"]
pub type Pwronclkeni2s1refclkW<'a, REG> = crate::BitWriter<'a, REG, Pwronclkeni2s1refclk>;
impl<'a, REG> Pwronclkeni2s1refclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable I2S instance 1 Clock to run during reset"]
    #[inline(always)]
    pub fn i2s1refclkenrst(self) -> &'a mut crate::W<REG> {
        self.variant(Pwronclkeni2s1refclk::I2s1refclkenrst)
    }
    #[doc = "Chicken bit to revert to rev A"]
    #[inline(always)]
    pub fn defeature(self) -> &'a mut crate::W<REG> {
        self.variant(Pwronclkeni2s1refclk::Defeature)
    }
}
#[doc = "Chicken bit to disable Rev B clock enable during reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwronclkenusbrefclk {
    #[doc = "0: Enable USB REF Clock to run during reset"]
    Usbrefclkenrst = 0,
    #[doc = "1: Chicken bit to revert to rev A"]
    Defeature = 1,
}
impl From<Pwronclkenusbrefclk> for bool {
    #[inline(always)]
    fn from(variant: Pwronclkenusbrefclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRONCLKENUSBREFCLK` reader - Chicken bit to disable Rev B clock enable during reset"]
pub type PwronclkenusbrefclkR = crate::BitReader<Pwronclkenusbrefclk>;
impl PwronclkenusbrefclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwronclkenusbrefclk {
        match self.bits {
            false => Pwronclkenusbrefclk::Usbrefclkenrst,
            true => Pwronclkenusbrefclk::Defeature,
        }
    }
    #[doc = "Enable USB REF Clock to run during reset"]
    #[inline(always)]
    pub fn is_usbrefclkenrst(&self) -> bool {
        *self == Pwronclkenusbrefclk::Usbrefclkenrst
    }
    #[doc = "Chicken bit to revert to rev A"]
    #[inline(always)]
    pub fn is_defeature(&self) -> bool {
        *self == Pwronclkenusbrefclk::Defeature
    }
}
#[doc = "Field `PWRONCLKENUSBREFCLK` writer - Chicken bit to disable Rev B clock enable during reset"]
pub type PwronclkenusbrefclkW<'a, REG> = crate::BitWriter<'a, REG, Pwronclkenusbrefclk>;
impl<'a, REG> PwronclkenusbrefclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable USB REF Clock to run during reset"]
    #[inline(always)]
    pub fn usbrefclkenrst(self) -> &'a mut crate::W<REG> {
        self.variant(Pwronclkenusbrefclk::Usbrefclkenrst)
    }
    #[doc = "Chicken bit to revert to rev A"]
    #[inline(always)]
    pub fn defeature(self) -> &'a mut crate::W<REG> {
        self.variant(Pwronclkenusbrefclk::Defeature)
    }
}
#[doc = "Chicken bit to enable clock gating on CM4 DAXI CLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cm4daxiclkgateen {
    #[doc = "0: Disable clock gate"]
    Dis = 0,
    #[doc = "1: Enable clock gate"]
    En = 1,
}
impl From<Cm4daxiclkgateen> for bool {
    #[inline(always)]
    fn from(variant: Cm4daxiclkgateen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CM4DAXICLKGATEEN` reader - Chicken bit to enable clock gating on CM4 DAXI CLK"]
pub type Cm4daxiclkgateenR = crate::BitReader<Cm4daxiclkgateen>;
impl Cm4daxiclkgateenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cm4daxiclkgateen {
        match self.bits {
            false => Cm4daxiclkgateen::Dis,
            true => Cm4daxiclkgateen::En,
        }
    }
    #[doc = "Disable clock gate"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Cm4daxiclkgateen::Dis
    }
    #[doc = "Enable clock gate"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Cm4daxiclkgateen::En
    }
}
#[doc = "Field `CM4DAXICLKGATEEN` writer - Chicken bit to enable clock gating on CM4 DAXI CLK"]
pub type Cm4daxiclkgateenW<'a, REG> = crate::BitWriter<'a, REG, Cm4daxiclkgateen>;
impl<'a, REG> Cm4daxiclkgateenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable clock gate"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Cm4daxiclkgateen::Dis)
    }
    #[doc = "Enable clock gate"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Cm4daxiclkgateen::En)
    }
}
#[doc = "Chicken bit to enable clock gating on GFX CLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gfxclkclkgateen {
    #[doc = "0: Disable clock gate"]
    Dis = 0,
    #[doc = "1: Enable clock gate"]
    En = 1,
}
impl From<Gfxclkclkgateen> for bool {
    #[inline(always)]
    fn from(variant: Gfxclkclkgateen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GFXCLKCLKGATEEN` reader - Chicken bit to enable clock gating on GFX CLK"]
pub type GfxclkclkgateenR = crate::BitReader<Gfxclkclkgateen>;
impl GfxclkclkgateenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gfxclkclkgateen {
        match self.bits {
            false => Gfxclkclkgateen::Dis,
            true => Gfxclkclkgateen::En,
        }
    }
    #[doc = "Disable clock gate"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Gfxclkclkgateen::Dis
    }
    #[doc = "Enable clock gate"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Gfxclkclkgateen::En
    }
}
#[doc = "Field `GFXCLKCLKGATEEN` writer - Chicken bit to enable clock gating on GFX CLK"]
pub type GfxclkclkgateenW<'a, REG> = crate::BitWriter<'a, REG, Gfxclkclkgateen>;
impl<'a, REG> GfxclkclkgateenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable clock gate"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Gfxclkclkgateen::Dis)
    }
    #[doc = "Enable clock gate"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Gfxclkclkgateen::En)
    }
}
#[doc = "Chicken bit to enable clock gating on GFX AXI CLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gfxaxiclkclkgateen {
    #[doc = "0: Disable clock gate"]
    Dis = 0,
    #[doc = "1: Enable clock gate"]
    En = 1,
}
impl From<Gfxaxiclkclkgateen> for bool {
    #[inline(always)]
    fn from(variant: Gfxaxiclkclkgateen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GFXAXICLKCLKGATEEN` reader - Chicken bit to enable clock gating on GFX AXI CLK"]
pub type GfxaxiclkclkgateenR = crate::BitReader<Gfxaxiclkclkgateen>;
impl GfxaxiclkclkgateenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gfxaxiclkclkgateen {
        match self.bits {
            false => Gfxaxiclkclkgateen::Dis,
            true => Gfxaxiclkclkgateen::En,
        }
    }
    #[doc = "Disable clock gate"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Gfxaxiclkclkgateen::Dis
    }
    #[doc = "Enable clock gate"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Gfxaxiclkclkgateen::En
    }
}
#[doc = "Field `GFXAXICLKCLKGATEEN` writer - Chicken bit to enable clock gating on GFX AXI CLK"]
pub type GfxaxiclkclkgateenW<'a, REG> = crate::BitWriter<'a, REG, Gfxaxiclkclkgateen>;
impl<'a, REG> GfxaxiclkclkgateenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable clock gate"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Gfxaxiclkclkgateen::Dis)
    }
    #[doc = "Enable clock gate"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Gfxaxiclkclkgateen::En)
    }
}
#[doc = "Chicken bit to enable clock gating on APB DMA CPU CLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Apbdmacpuclkclkgateen {
    #[doc = "0: Disable clock gate"]
    Dis = 0,
    #[doc = "1: Enable clock gate"]
    En = 1,
}
impl From<Apbdmacpuclkclkgateen> for bool {
    #[inline(always)]
    fn from(variant: Apbdmacpuclkclkgateen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APBDMACPUCLKCLKGATEEN` reader - Chicken bit to enable clock gating on APB DMA CPU CLK"]
pub type ApbdmacpuclkclkgateenR = crate::BitReader<Apbdmacpuclkclkgateen>;
impl ApbdmacpuclkclkgateenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Apbdmacpuclkclkgateen {
        match self.bits {
            false => Apbdmacpuclkclkgateen::Dis,
            true => Apbdmacpuclkclkgateen::En,
        }
    }
    #[doc = "Disable clock gate"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Apbdmacpuclkclkgateen::Dis
    }
    #[doc = "Enable clock gate"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Apbdmacpuclkclkgateen::En
    }
}
#[doc = "Field `APBDMACPUCLKCLKGATEEN` writer - Chicken bit to enable clock gating on APB DMA CPU CLK"]
pub type ApbdmacpuclkclkgateenW<'a, REG> = crate::BitWriter<'a, REG, Apbdmacpuclkclkgateen>;
impl<'a, REG> ApbdmacpuclkclkgateenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable clock gate"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Apbdmacpuclkclkgateen::Dis)
    }
    #[doc = "Enable clock gate"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Apbdmacpuclkclkgateen::En)
    }
}
#[doc = "Chicken bit to enable clock gating on ETM TRACE CLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Etmtraceclkclkgateen {
    #[doc = "0: Disable clock gate"]
    Dis = 0,
    #[doc = "1: Enable clock gate"]
    En = 1,
}
impl From<Etmtraceclkclkgateen> for bool {
    #[inline(always)]
    fn from(variant: Etmtraceclkclkgateen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETMTRACECLKCLKGATEEN` reader - Chicken bit to enable clock gating on ETM TRACE CLK"]
pub type EtmtraceclkclkgateenR = crate::BitReader<Etmtraceclkclkgateen>;
impl EtmtraceclkclkgateenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Etmtraceclkclkgateen {
        match self.bits {
            false => Etmtraceclkclkgateen::Dis,
            true => Etmtraceclkclkgateen::En,
        }
    }
    #[doc = "Disable clock gate"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Etmtraceclkclkgateen::Dis
    }
    #[doc = "Enable clock gate"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Etmtraceclkclkgateen::En
    }
}
#[doc = "Field `ETMTRACECLKCLKGATEEN` writer - Chicken bit to enable clock gating on ETM TRACE CLK"]
pub type EtmtraceclkclkgateenW<'a, REG> = crate::BitWriter<'a, REG, Etmtraceclkclkgateen>;
impl<'a, REG> EtmtraceclkclkgateenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable clock gate"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Etmtraceclkclkgateen::Dis)
    }
    #[doc = "Enable clock gate"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Etmtraceclkclkgateen::En)
    }
}
#[doc = "Chicken bit to enable clock gating on HFRC_FUNC_CLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfrcfuncclkgateen {
    #[doc = "0: Disable clock gate"]
    Dis = 0,
    #[doc = "1: Enable clock gate"]
    En = 1,
}
impl From<Hfrcfuncclkgateen> for bool {
    #[inline(always)]
    fn from(variant: Hfrcfuncclkgateen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFRCFUNCCLKGATEEN` reader - Chicken bit to enable clock gating on HFRC_FUNC_CLK"]
pub type HfrcfuncclkgateenR = crate::BitReader<Hfrcfuncclkgateen>;
impl HfrcfuncclkgateenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfrcfuncclkgateen {
        match self.bits {
            false => Hfrcfuncclkgateen::Dis,
            true => Hfrcfuncclkgateen::En,
        }
    }
    #[doc = "Disable clock gate"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Hfrcfuncclkgateen::Dis
    }
    #[doc = "Enable clock gate"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Hfrcfuncclkgateen::En
    }
}
#[doc = "Field `HFRCFUNCCLKGATEEN` writer - Chicken bit to enable clock gating on HFRC_FUNC_CLK"]
pub type HfrcfuncclkgateenW<'a, REG> = crate::BitWriter<'a, REG, Hfrcfuncclkgateen>;
impl<'a, REG> HfrcfuncclkgateenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable clock gate"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Hfrcfuncclkgateen::Dis)
    }
    #[doc = "Enable clock gate"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Hfrcfuncclkgateen::En)
    }
}
#[doc = "DO NOT USE. HFRC96_TRUNK_GATE. Setting this bit when BIT23=0, will kill HFRC root clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfrc96trunkgate {
    #[doc = "0: Disable HFRC96_TRUNK_GATE"]
    Dis = 0,
    #[doc = "1: DO NOT USE. HFRC96_TRUNK_GATE. Setting this bit when BIT23=0, will kill HFRC root clock."]
    En = 1,
}
impl From<Hfrc96trunkgate> for bool {
    #[inline(always)]
    fn from(variant: Hfrc96trunkgate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFRC96TRUNKGATE` reader - DO NOT USE. HFRC96_TRUNK_GATE. Setting this bit when BIT23=0, will kill HFRC root clock."]
pub type Hfrc96trunkgateR = crate::BitReader<Hfrc96trunkgate>;
impl Hfrc96trunkgateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfrc96trunkgate {
        match self.bits {
            false => Hfrc96trunkgate::Dis,
            true => Hfrc96trunkgate::En,
        }
    }
    #[doc = "Disable HFRC96_TRUNK_GATE"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Hfrc96trunkgate::Dis
    }
    #[doc = "DO NOT USE. HFRC96_TRUNK_GATE. Setting this bit when BIT23=0, will kill HFRC root clock."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Hfrc96trunkgate::En
    }
}
#[doc = "Field `HFRC96TRUNKGATE` writer - DO NOT USE. HFRC96_TRUNK_GATE. Setting this bit when BIT23=0, will kill HFRC root clock."]
pub type Hfrc96trunkgateW<'a, REG> = crate::BitWriter<'a, REG, Hfrc96trunkgate>;
impl<'a, REG> Hfrc96trunkgateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable HFRC96_TRUNK_GATE"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Hfrc96trunkgate::Dis)
    }
    #[doc = "DO NOT USE. HFRC96_TRUNK_GATE. Setting this bit when BIT23=0, will kill HFRC root clock."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Hfrc96trunkgate::En)
    }
}
#[doc = "Field `CLKGENMISCSPARE` reader - Spare/Unused Chicken Bit"]
pub type ClkgenmiscspareR = crate::BitReader;
#[doc = "Field `CLKGENMISCSPARE` writer - Spare/Unused Chicken Bit"]
pub type ClkgenmiscspareW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Force HFRC On ."]
    #[inline(always)]
    pub fn frchfrc(&self) -> FrchfrcR {
        FrchfrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force fclk, hclk, fclk_wic and fclk_pmu to be turned off during burst transition."]
    #[inline(always)]
    pub fn frcburstoff(&self) -> FrcburstoffR {
        FrcburstoffR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Use HFRC-48MHz or HFRC2-48MHz for DSP"]
    #[inline(always)]
    pub fn usehfrc2fq48mhz(&self) -> Usehfrc2fq48mhzR {
        Usehfrc2fq48mhzR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Use HFRC-96MHz or HFRC2-96MHz for DSP"]
    #[inline(always)]
    pub fn usehfrc2fq96mhz(&self) -> Usehfrc2fq96mhzR {
        Usehfrc2fq96mhzR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Use HFRC-192MHz or HFRC2-192MHz for MCU"]
    #[inline(always)]
    pub fn usehfrc2fq192mhz(&self) -> Usehfrc2fq192mhzR {
        Usehfrc2fq192mhzR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Force HFRC2 On.Setting this bit forces HFRC2 to remain on, including in deep sleep. When changing a module's clock source to HFRC2, this bit must be set and remain set when any module is using HFRC2 as its clock."]
    #[inline(always)]
    pub fn frchfrc2(&self) -> Frchfrc2R {
        Frchfrc2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Chicken bit to disable Rev B clock enable during reset"]
    #[inline(always)]
    pub fn pwronclkendisp(&self) -> PwronclkendispR {
        PwronclkendispR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Chicken bit to disable Rev B clock enable during reset"]
    #[inline(always)]
    pub fn pwronclkendispphy(&self) -> PwronclkendispphyR {
        PwronclkendispphyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Chicken bit to disable Rev B clock enable during reset"]
    #[inline(always)]
    pub fn pwronclkengfx(&self) -> PwronclkengfxR {
        PwronclkengfxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Chicken bit to disable Rev B clock enable during reset"]
    #[inline(always)]
    pub fn pwronclkenusb(&self) -> PwronclkenusbR {
        PwronclkenusbR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Chicken bit to disable Rev B clock enable during reset"]
    #[inline(always)]
    pub fn pwronclkensdio(&self) -> PwronclkensdioR {
        PwronclkensdioR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Chicken bit to disable Rev B clock enable during reset"]
    #[inline(always)]
    pub fn pwronclkencrypto(&self) -> PwronclkencryptoR {
        PwronclkencryptoR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Chicken bit to disable Rev B clock enable during reset"]
    #[inline(always)]
    pub fn pwronclkeni2s0(&self) -> Pwronclkeni2s0R {
        Pwronclkeni2s0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Chicken bit to disable Rev B clock enable during reset"]
    #[inline(always)]
    pub fn pwronclkeni2s1(&self) -> Pwronclkeni2s1R {
        Pwronclkeni2s1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Chicken bit to disable Rev B added clock gating"]
    #[inline(always)]
    pub fn axixaclkenovrride(&self) -> AxixaclkenovrrideR {
        AxixaclkenovrrideR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Chicken bit to disable Rev B clock enable during reset"]
    #[inline(always)]
    pub fn pwronclkeni2s0refclk(&self) -> Pwronclkeni2s0refclkR {
        Pwronclkeni2s0refclkR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Chicken bit to disable Rev B clock enable during reset"]
    #[inline(always)]
    pub fn pwronclkeni2s1refclk(&self) -> Pwronclkeni2s1refclkR {
        Pwronclkeni2s1refclkR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Chicken bit to disable Rev B clock enable during reset"]
    #[inline(always)]
    pub fn pwronclkenusbrefclk(&self) -> PwronclkenusbrefclkR {
        PwronclkenusbrefclkR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Chicken bit to enable clock gating on CM4 DAXI CLK"]
    #[inline(always)]
    pub fn cm4daxiclkgateen(&self) -> Cm4daxiclkgateenR {
        Cm4daxiclkgateenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Chicken bit to enable clock gating on GFX CLK"]
    #[inline(always)]
    pub fn gfxclkclkgateen(&self) -> GfxclkclkgateenR {
        GfxclkclkgateenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Chicken bit to enable clock gating on GFX AXI CLK"]
    #[inline(always)]
    pub fn gfxaxiclkclkgateen(&self) -> GfxaxiclkclkgateenR {
        GfxaxiclkclkgateenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Chicken bit to enable clock gating on APB DMA CPU CLK"]
    #[inline(always)]
    pub fn apbdmacpuclkclkgateen(&self) -> ApbdmacpuclkclkgateenR {
        ApbdmacpuclkclkgateenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Chicken bit to enable clock gating on ETM TRACE CLK"]
    #[inline(always)]
    pub fn etmtraceclkclkgateen(&self) -> EtmtraceclkclkgateenR {
        EtmtraceclkclkgateenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Chicken bit to enable clock gating on HFRC_FUNC_CLK"]
    #[inline(always)]
    pub fn hfrcfuncclkgateen(&self) -> HfrcfuncclkgateenR {
        HfrcfuncclkgateenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DO NOT USE. HFRC96_TRUNK_GATE. Setting this bit when BIT23=0, will kill HFRC root clock."]
    #[inline(always)]
    pub fn hfrc96trunkgate(&self) -> Hfrc96trunkgateR {
        Hfrc96trunkgateR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Spare/Unused Chicken Bit"]
    #[inline(always)]
    pub fn clkgenmiscspare(&self) -> ClkgenmiscspareR {
        ClkgenmiscspareR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force HFRC On ."]
    #[inline(always)]
    #[must_use]
    pub fn frchfrc(&mut self) -> FrchfrcW<MiscSpec> {
        FrchfrcW::new(self, 0)
    }
    #[doc = "Bit 1 - Force fclk, hclk, fclk_wic and fclk_pmu to be turned off during burst transition."]
    #[inline(always)]
    #[must_use]
    pub fn frcburstoff(&mut self) -> FrcburstoffW<MiscSpec> {
        FrcburstoffW::new(self, 1)
    }
    #[doc = "Bit 2 - Use HFRC-48MHz or HFRC2-48MHz for DSP"]
    #[inline(always)]
    #[must_use]
    pub fn usehfrc2fq48mhz(&mut self) -> Usehfrc2fq48mhzW<MiscSpec> {
        Usehfrc2fq48mhzW::new(self, 2)
    }
    #[doc = "Bit 3 - Use HFRC-96MHz or HFRC2-96MHz for DSP"]
    #[inline(always)]
    #[must_use]
    pub fn usehfrc2fq96mhz(&mut self) -> Usehfrc2fq96mhzW<MiscSpec> {
        Usehfrc2fq96mhzW::new(self, 3)
    }
    #[doc = "Bit 4 - Use HFRC-192MHz or HFRC2-192MHz for MCU"]
    #[inline(always)]
    #[must_use]
    pub fn usehfrc2fq192mhz(&mut self) -> Usehfrc2fq192mhzW<MiscSpec> {
        Usehfrc2fq192mhzW::new(self, 4)
    }
    #[doc = "Bit 5 - Force HFRC2 On.Setting this bit forces HFRC2 to remain on, including in deep sleep. When changing a module's clock source to HFRC2, this bit must be set and remain set when any module is using HFRC2 as its clock."]
    #[inline(always)]
    #[must_use]
    pub fn frchfrc2(&mut self) -> Frchfrc2W<MiscSpec> {
        Frchfrc2W::new(self, 5)
    }
    #[doc = "Bit 6 - Chicken bit to disable Rev B clock enable during reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwronclkendisp(&mut self) -> PwronclkendispW<MiscSpec> {
        PwronclkendispW::new(self, 6)
    }
    #[doc = "Bit 7 - Chicken bit to disable Rev B clock enable during reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwronclkendispphy(&mut self) -> PwronclkendispphyW<MiscSpec> {
        PwronclkendispphyW::new(self, 7)
    }
    #[doc = "Bit 8 - Chicken bit to disable Rev B clock enable during reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwronclkengfx(&mut self) -> PwronclkengfxW<MiscSpec> {
        PwronclkengfxW::new(self, 8)
    }
    #[doc = "Bit 9 - Chicken bit to disable Rev B clock enable during reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwronclkenusb(&mut self) -> PwronclkenusbW<MiscSpec> {
        PwronclkenusbW::new(self, 9)
    }
    #[doc = "Bit 10 - Chicken bit to disable Rev B clock enable during reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwronclkensdio(&mut self) -> PwronclkensdioW<MiscSpec> {
        PwronclkensdioW::new(self, 10)
    }
    #[doc = "Bit 11 - Chicken bit to disable Rev B clock enable during reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwronclkencrypto(&mut self) -> PwronclkencryptoW<MiscSpec> {
        PwronclkencryptoW::new(self, 11)
    }
    #[doc = "Bit 12 - Chicken bit to disable Rev B clock enable during reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwronclkeni2s0(&mut self) -> Pwronclkeni2s0W<MiscSpec> {
        Pwronclkeni2s0W::new(self, 12)
    }
    #[doc = "Bit 13 - Chicken bit to disable Rev B clock enable during reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwronclkeni2s1(&mut self) -> Pwronclkeni2s1W<MiscSpec> {
        Pwronclkeni2s1W::new(self, 13)
    }
    #[doc = "Bit 14 - Chicken bit to disable Rev B added clock gating"]
    #[inline(always)]
    #[must_use]
    pub fn axixaclkenovrride(&mut self) -> AxixaclkenovrrideW<MiscSpec> {
        AxixaclkenovrrideW::new(self, 14)
    }
    #[doc = "Bit 15 - Chicken bit to disable Rev B clock enable during reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwronclkeni2s0refclk(&mut self) -> Pwronclkeni2s0refclkW<MiscSpec> {
        Pwronclkeni2s0refclkW::new(self, 15)
    }
    #[doc = "Bit 16 - Chicken bit to disable Rev B clock enable during reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwronclkeni2s1refclk(&mut self) -> Pwronclkeni2s1refclkW<MiscSpec> {
        Pwronclkeni2s1refclkW::new(self, 16)
    }
    #[doc = "Bit 17 - Chicken bit to disable Rev B clock enable during reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwronclkenusbrefclk(&mut self) -> PwronclkenusbrefclkW<MiscSpec> {
        PwronclkenusbrefclkW::new(self, 17)
    }
    #[doc = "Bit 18 - Chicken bit to enable clock gating on CM4 DAXI CLK"]
    #[inline(always)]
    #[must_use]
    pub fn cm4daxiclkgateen(&mut self) -> Cm4daxiclkgateenW<MiscSpec> {
        Cm4daxiclkgateenW::new(self, 18)
    }
    #[doc = "Bit 19 - Chicken bit to enable clock gating on GFX CLK"]
    #[inline(always)]
    #[must_use]
    pub fn gfxclkclkgateen(&mut self) -> GfxclkclkgateenW<MiscSpec> {
        GfxclkclkgateenW::new(self, 19)
    }
    #[doc = "Bit 20 - Chicken bit to enable clock gating on GFX AXI CLK"]
    #[inline(always)]
    #[must_use]
    pub fn gfxaxiclkclkgateen(&mut self) -> GfxaxiclkclkgateenW<MiscSpec> {
        GfxaxiclkclkgateenW::new(self, 20)
    }
    #[doc = "Bit 21 - Chicken bit to enable clock gating on APB DMA CPU CLK"]
    #[inline(always)]
    #[must_use]
    pub fn apbdmacpuclkclkgateen(&mut self) -> ApbdmacpuclkclkgateenW<MiscSpec> {
        ApbdmacpuclkclkgateenW::new(self, 21)
    }
    #[doc = "Bit 22 - Chicken bit to enable clock gating on ETM TRACE CLK"]
    #[inline(always)]
    #[must_use]
    pub fn etmtraceclkclkgateen(&mut self) -> EtmtraceclkclkgateenW<MiscSpec> {
        EtmtraceclkclkgateenW::new(self, 22)
    }
    #[doc = "Bit 23 - Chicken bit to enable clock gating on HFRC_FUNC_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcfuncclkgateen(&mut self) -> HfrcfuncclkgateenW<MiscSpec> {
        HfrcfuncclkgateenW::new(self, 23)
    }
    #[doc = "Bit 24 - DO NOT USE. HFRC96_TRUNK_GATE. Setting this bit when BIT23=0, will kill HFRC root clock."]
    #[inline(always)]
    #[must_use]
    pub fn hfrc96trunkgate(&mut self) -> Hfrc96trunkgateW<MiscSpec> {
        Hfrc96trunkgateW::new(self, 24)
    }
    #[doc = "Bit 25 - Spare/Unused Chicken Bit"]
    #[inline(always)]
    #[must_use]
    pub fn clkgenmiscspare(&mut self) -> ClkgenmiscspareW<MiscSpec> {
        ClkgenmiscspareW::new(self, 25)
    }
}
#[doc = "This register controls a 'safe' mode for burst, which disables the clock when burst transition is happening. It also includes a register to force the HFRC during deep sleep. It is mainly used for debug and testing.\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiscSpec;
impl crate::RegisterSpec for MiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc::R`](R) reader structure"]
impl crate::Readable for MiscSpec {}
#[doc = "`write(|w| ..)` method takes [`misc::W`](W) writer structure"]
impl crate::Writable for MiscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MISC to value 0"]
impl crate::Resettable for MiscSpec {
    const RESET_VALUE: u32 = 0;
}
