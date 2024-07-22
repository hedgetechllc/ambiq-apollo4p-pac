#[doc = "Register `MODE` reader"]
pub type R = crate::R<ModeSpec>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<ModeSpec>;
#[doc = "Field `TSTMODEN` reader - When set to 1, test mode is enabled"]
pub type TstmodenR = crate::BitReader;
#[doc = "Field `TSTMODEN` writer - When set to 1, test mode is enabled"]
pub type TstmodenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBLHORSCANEN` reader - When set to 1, double horizontal scan is enabled"]
pub type DblhorscanenR = crate::BitReader;
#[doc = "Field `DBLHORSCANEN` writer - When set to 1, double horizontal scan is enabled"]
pub type DblhorscanenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVDSINTEN` reader - When set to 1, LVDS interface is enabled"]
pub type LvdsintenR = crate::BitReader;
#[doc = "Field `LVDSINTEN` writer - When set to 1, LVDS interface is enabled"]
pub type LvdsintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `YUYVEN` reader - When set to 1, the following output color formats are enabled : Byte-3 beat Interface enabled, Byte-4 beat (RGBX) Interface enabled, Two phase serial 12-bit enabled, YUYV (16-bit mode) enabled, BT.656 enabled, JDI MIP enabled"]
pub type YuyvenR = crate::BitReader;
#[doc = "Field `YUYVEN` writer - When set to 1, the following output color formats are enabled : Byte-3 beat Interface enabled, Byte-4 beat (RGBX) Interface enabled, Two phase serial 12-bit enabled, YUYV (16-bit mode) enabled, BT.656 enabled, JDI MIP enabled"]
pub type YuyvenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBITYPEBEN` reader - When set to 1, DBI Type-B interface is enabled"]
pub type DbitypebenR = crate::BitReader;
#[doc = "Field `DBITYPEBEN` writer - When set to 1, DBI Type-B interface is enabled"]
pub type DbitypebenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Display data format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dispfmt {
    #[doc = "0: DPI Interface"]
    Dpi = 0,
    #[doc = "1: Byte-3 beat Interface"]
    Byte3 = 1,
    #[doc = "2: Byte-4 beat (RGBX) Interface"]
    Byte4 = 2,
    #[doc = "3: Two phase serial 12-bit"]
    Serial = 3,
    #[doc = "4: LVDS 24-bit unbalanced single pixel format 2"]
    Lvds2 = 4,
    #[doc = "5: LVDS 24-bit unbalanced single pixel format 1"]
    Lvds1 = 5,
    #[doc = "6: YUYV (16-bit mode)"]
    Yuyv = 6,
    #[doc = "7: BT.656"]
    Bt656 = 7,
    #[doc = "8: JDI MIP"]
    Jdi = 8,
}
impl From<Dispfmt> for u8 {
    #[inline(always)]
    fn from(variant: Dispfmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dispfmt {
    type Ux = u8;
}
impl crate::IsEnum for Dispfmt {}
#[doc = "Field `DISPFMT` reader - Display data format"]
pub type DispfmtR = crate::FieldReader<Dispfmt>;
impl DispfmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dispfmt> {
        match self.bits {
            0 => Some(Dispfmt::Dpi),
            1 => Some(Dispfmt::Byte3),
            2 => Some(Dispfmt::Byte4),
            3 => Some(Dispfmt::Serial),
            4 => Some(Dispfmt::Lvds2),
            5 => Some(Dispfmt::Lvds1),
            6 => Some(Dispfmt::Yuyv),
            7 => Some(Dispfmt::Bt656),
            8 => Some(Dispfmt::Jdi),
            _ => None,
        }
    }
    #[doc = "DPI Interface"]
    #[inline(always)]
    pub fn is_dpi(&self) -> bool {
        *self == Dispfmt::Dpi
    }
    #[doc = "Byte-3 beat Interface"]
    #[inline(always)]
    pub fn is_byte3(&self) -> bool {
        *self == Dispfmt::Byte3
    }
    #[doc = "Byte-4 beat (RGBX) Interface"]
    #[inline(always)]
    pub fn is_byte4(&self) -> bool {
        *self == Dispfmt::Byte4
    }
    #[doc = "Two phase serial 12-bit"]
    #[inline(always)]
    pub fn is_serial(&self) -> bool {
        *self == Dispfmt::Serial
    }
    #[doc = "LVDS 24-bit unbalanced single pixel format 2"]
    #[inline(always)]
    pub fn is_lvds2(&self) -> bool {
        *self == Dispfmt::Lvds2
    }
    #[doc = "LVDS 24-bit unbalanced single pixel format 1"]
    #[inline(always)]
    pub fn is_lvds1(&self) -> bool {
        *self == Dispfmt::Lvds1
    }
    #[doc = "YUYV (16-bit mode)"]
    #[inline(always)]
    pub fn is_yuyv(&self) -> bool {
        *self == Dispfmt::Yuyv
    }
    #[doc = "BT.656"]
    #[inline(always)]
    pub fn is_bt656(&self) -> bool {
        *self == Dispfmt::Bt656
    }
    #[doc = "JDI MIP"]
    #[inline(always)]
    pub fn is_jdi(&self) -> bool {
        *self == Dispfmt::Jdi
    }
}
#[doc = "Field `DISPFMT` writer - Display data format"]
pub type DispfmtW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dispfmt>;
impl<'a, REG> DispfmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DPI Interface"]
    #[inline(always)]
    pub fn dpi(self) -> &'a mut crate::W<REG> {
        self.variant(Dispfmt::Dpi)
    }
    #[doc = "Byte-3 beat Interface"]
    #[inline(always)]
    pub fn byte3(self) -> &'a mut crate::W<REG> {
        self.variant(Dispfmt::Byte3)
    }
    #[doc = "Byte-4 beat (RGBX) Interface"]
    #[inline(always)]
    pub fn byte4(self) -> &'a mut crate::W<REG> {
        self.variant(Dispfmt::Byte4)
    }
    #[doc = "Two phase serial 12-bit"]
    #[inline(always)]
    pub fn serial(self) -> &'a mut crate::W<REG> {
        self.variant(Dispfmt::Serial)
    }
    #[doc = "LVDS 24-bit unbalanced single pixel format 2"]
    #[inline(always)]
    pub fn lvds2(self) -> &'a mut crate::W<REG> {
        self.variant(Dispfmt::Lvds2)
    }
    #[doc = "LVDS 24-bit unbalanced single pixel format 1"]
    #[inline(always)]
    pub fn lvds1(self) -> &'a mut crate::W<REG> {
        self.variant(Dispfmt::Lvds1)
    }
    #[doc = "YUYV (16-bit mode)"]
    #[inline(always)]
    pub fn yuyv(self) -> &'a mut crate::W<REG> {
        self.variant(Dispfmt::Yuyv)
    }
    #[doc = "BT.656"]
    #[inline(always)]
    pub fn bt656(self) -> &'a mut crate::W<REG> {
        self.variant(Dispfmt::Bt656)
    }
    #[doc = "JDI MIP"]
    #[inline(always)]
    pub fn jdi(self) -> &'a mut crate::W<REG> {
        self.variant(Dispfmt::Jdi)
    }
}
#[doc = "Output color format:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Colfmt {
    #[doc = "1: YUV/YCbCr format is enabled"]
    YuvEn = 1,
    #[doc = "0: RGB format is enabled"]
    RgbEn = 0,
}
impl From<Colfmt> for bool {
    #[inline(always)]
    fn from(variant: Colfmt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COLFMT` reader - Output color format:"]
pub type ColfmtR = crate::BitReader<Colfmt>;
impl ColfmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Colfmt {
        match self.bits {
            true => Colfmt::YuvEn,
            false => Colfmt::RgbEn,
        }
    }
    #[doc = "YUV/YCbCr format is enabled"]
    #[inline(always)]
    pub fn is_yuv_en(&self) -> bool {
        *self == Colfmt::YuvEn
    }
    #[doc = "RGB format is enabled"]
    #[inline(always)]
    pub fn is_rgb_en(&self) -> bool {
        *self == Colfmt::RgbEn
    }
}
#[doc = "Field `COLFMT` writer - Output color format:"]
pub type ColfmtW<'a, REG> = crate::BitWriter<'a, REG, Colfmt>;
impl<'a, REG> ColfmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "YUV/YCbCr format is enabled"]
    #[inline(always)]
    pub fn yuv_en(self) -> &'a mut crate::W<REG> {
        self.variant(Colfmt::YuvEn)
    }
    #[doc = "RGB format is enabled"]
    #[inline(always)]
    pub fn rgb_en(self) -> &'a mut crate::W<REG> {
        self.variant(Colfmt::RgbEn)
    }
}
#[doc = "Field `LVDSPADSEN` reader - When set to 1, LVDS output pads are enabled"]
pub type LvdspadsenR = crate::BitReader;
#[doc = "Field `LVDSPADSEN` writer - When set to 1, LVDS output pads are enabled"]
pub type LvdspadsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLCLKNDIV` reader - When set to 1, PLL_CLK is not divided"]
pub type PllclkndivR = crate::BitReader;
#[doc = "Field `PLLCLKNDIV` writer - When set to 1, PLL_CLK is not divided"]
pub type PllclkndivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD0` reader - This field is reserved."]
pub type Rsvd0R = crate::FieldReader;
#[doc = "Field `RSVD0` writer - This field is reserved."]
pub type Rsvd0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FRAMEUPDTEN` reader - When set to 1, single frame update is enabled"]
pub type FrameupdtenR = crate::BitReader;
#[doc = "Field `FRAMEUPDTEN` writer - When set to 1, single frame update is enabled"]
pub type FrameupdtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD1` reader - This field is reserved."]
pub type Rsvd1R = crate::BitReader;
#[doc = "Field `RSVD1` writer - This field is reserved."]
pub type Rsvd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLANKFRC` reader - When set to 1, forces output to blank"]
pub type BlankfrcR = crate::BitReader;
#[doc = "Field `BLANKFRC` writer - When set to 1, forces output to blank"]
pub type BlankfrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMARAMPEN` reader - When set to 1, gamma ramp is enabled"]
pub type GamarampenR = crate::BitReader;
#[doc = "Field `GAMARAMPEN` writer - When set to 1, gamma ramp is enabled"]
pub type GamarampenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD2` reader - This field is reserved."]
pub type Rsvd2R = crate::BitReader;
#[doc = "Field `RSVD2` writer - This field is reserved."]
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Defines Pixel Clock out polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pixclkpol {
    #[doc = "1: Pixel Clock out polarity is negative"]
    PolNeg = 1,
    #[doc = "0: Pixel Clock out polarity is positive"]
    PolPos = 0,
}
impl From<Pixclkpol> for bool {
    #[inline(always)]
    fn from(variant: Pixclkpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIXCLKPOL` reader - Defines Pixel Clock out polarity"]
pub type PixclkpolR = crate::BitReader<Pixclkpol>;
impl PixclkpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pixclkpol {
        match self.bits {
            true => Pixclkpol::PolNeg,
            false => Pixclkpol::PolPos,
        }
    }
    #[doc = "Pixel Clock out polarity is negative"]
    #[inline(always)]
    pub fn is_pol_neg(&self) -> bool {
        *self == Pixclkpol::PolNeg
    }
    #[doc = "Pixel Clock out polarity is positive"]
    #[inline(always)]
    pub fn is_pol_pos(&self) -> bool {
        *self == Pixclkpol::PolPos
    }
}
#[doc = "Field `PIXCLKPOL` writer - Defines Pixel Clock out polarity"]
pub type PixclkpolW<'a, REG> = crate::BitWriter<'a, REG, Pixclkpol>;
impl<'a, REG> PixclkpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pixel Clock out polarity is negative"]
    #[inline(always)]
    pub fn pol_neg(self) -> &'a mut crate::W<REG> {
        self.variant(Pixclkpol::PolNeg)
    }
    #[doc = "Pixel Clock out polarity is positive"]
    #[inline(always)]
    pub fn pol_pos(self) -> &'a mut crate::W<REG> {
        self.variant(Pixclkpol::PolPos)
    }
}
#[doc = "Field `VSYNCEN` reader - When set to 1, VSYNC for a single cycle per line is enabled"]
pub type VsyncenR = crate::BitReader;
#[doc = "Field `VSYNCEN` writer - When set to 1, VSYNC for a single cycle per line is enabled"]
pub type VsyncenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DITHEREN` reader - When set to 1, dithering is enabled"]
pub type DitherenR = crate::BitReader;
#[doc = "Field `DITHEREN` writer - When set to 1, dithering is enabled"]
pub type DitherenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD3` reader - This field is reserved."]
pub type Rsvd3R = crate::BitReader;
#[doc = "Field `RSVD3` writer - This field is reserved."]
pub type Rsvd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Defines DE polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Depol {
    #[doc = "1: DE polarity is negative"]
    DeNeg = 1,
    #[doc = "0: DE polarity is positive"]
    DePos = 0,
}
impl From<Depol> for bool {
    #[inline(always)]
    fn from(variant: Depol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEPOL` reader - Defines DE polarity"]
pub type DepolR = crate::BitReader<Depol>;
impl DepolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Depol {
        match self.bits {
            true => Depol::DeNeg,
            false => Depol::DePos,
        }
    }
    #[doc = "DE polarity is negative"]
    #[inline(always)]
    pub fn is_de_neg(&self) -> bool {
        *self == Depol::DeNeg
    }
    #[doc = "DE polarity is positive"]
    #[inline(always)]
    pub fn is_de_pos(&self) -> bool {
        *self == Depol::DePos
    }
}
#[doc = "Field `DEPOL` writer - Defines DE polarity"]
pub type DepolW<'a, REG> = crate::BitWriter<'a, REG, Depol>;
impl<'a, REG> DepolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DE polarity is negative"]
    #[inline(always)]
    pub fn de_neg(self) -> &'a mut crate::W<REG> {
        self.variant(Depol::DeNeg)
    }
    #[doc = "DE polarity is positive"]
    #[inline(always)]
    pub fn de_pos(self) -> &'a mut crate::W<REG> {
        self.variant(Depol::DePos)
    }
}
#[doc = "Defines HSYNC polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsyncpol {
    #[doc = "1: HSYNC polarity is negative"]
    HsyncNeg = 1,
    #[doc = "0: HSYNC polarity is positive"]
    HsyncPos = 0,
}
impl From<Hsyncpol> for bool {
    #[inline(always)]
    fn from(variant: Hsyncpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSYNCPOL` reader - Defines HSYNC polarity"]
pub type HsyncpolR = crate::BitReader<Hsyncpol>;
impl HsyncpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsyncpol {
        match self.bits {
            true => Hsyncpol::HsyncNeg,
            false => Hsyncpol::HsyncPos,
        }
    }
    #[doc = "HSYNC polarity is negative"]
    #[inline(always)]
    pub fn is_hsync_neg(&self) -> bool {
        *self == Hsyncpol::HsyncNeg
    }
    #[doc = "HSYNC polarity is positive"]
    #[inline(always)]
    pub fn is_hsync_pos(&self) -> bool {
        *self == Hsyncpol::HsyncPos
    }
}
#[doc = "Field `HSYNCPOL` writer - Defines HSYNC polarity"]
pub type HsyncpolW<'a, REG> = crate::BitWriter<'a, REG, Hsyncpol>;
impl<'a, REG> HsyncpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSYNC polarity is negative"]
    #[inline(always)]
    pub fn hsync_neg(self) -> &'a mut crate::W<REG> {
        self.variant(Hsyncpol::HsyncNeg)
    }
    #[doc = "HSYNC polarity is positive"]
    #[inline(always)]
    pub fn hsync_pos(self) -> &'a mut crate::W<REG> {
        self.variant(Hsyncpol::HsyncPos)
    }
}
#[doc = "Defines VSYNC polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vsyncpol {
    #[doc = "1: VSYNC polarity is negative"]
    VsyncNeg = 1,
    #[doc = "0: VSYNC polarity is positive"]
    VsyncPos = 0,
}
impl From<Vsyncpol> for bool {
    #[inline(always)]
    fn from(variant: Vsyncpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSYNCPOL` reader - Defines VSYNC polarity"]
pub type VsyncpolR = crate::BitReader<Vsyncpol>;
impl VsyncpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vsyncpol {
        match self.bits {
            true => Vsyncpol::VsyncNeg,
            false => Vsyncpol::VsyncPos,
        }
    }
    #[doc = "VSYNC polarity is negative"]
    #[inline(always)]
    pub fn is_vsync_neg(&self) -> bool {
        *self == Vsyncpol::VsyncNeg
    }
    #[doc = "VSYNC polarity is positive"]
    #[inline(always)]
    pub fn is_vsync_pos(&self) -> bool {
        *self == Vsyncpol::VsyncPos
    }
}
#[doc = "Field `VSYNCPOL` writer - Defines VSYNC polarity"]
pub type VsyncpolW<'a, REG> = crate::BitWriter<'a, REG, Vsyncpol>;
impl<'a, REG> VsyncpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VSYNC polarity is negative"]
    #[inline(always)]
    pub fn vsync_neg(self) -> &'a mut crate::W<REG> {
        self.variant(Vsyncpol::VsyncNeg)
    }
    #[doc = "VSYNC polarity is positive"]
    #[inline(always)]
    pub fn vsync_pos(self) -> &'a mut crate::W<REG> {
        self.variant(Vsyncpol::VsyncPos)
    }
}
#[doc = "Field `RSVD4` reader - This field is reserved."]
pub type Rsvd4R = crate::BitReader;
#[doc = "Field `RSVD4` writer - This field is reserved."]
pub type Rsvd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUSOREN` reader - When set to 1, programmable cursor is enabled"]
pub type CusorenR = crate::BitReader;
#[doc = "Field `CUSOREN` writer - When set to 1, programmable cursor is enabled"]
pub type CusorenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC400ACT` reader - When set to 1, the dc400 controller is activated"]
pub type Dc400actR = crate::BitReader;
#[doc = "Field `DC400ACT` writer - When set to 1, the dc400 controller is activated"]
pub type Dc400actW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set to 1, test mode is enabled"]
    #[inline(always)]
    pub fn tstmoden(&self) -> TstmodenR {
        TstmodenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set to 1, double horizontal scan is enabled"]
    #[inline(always)]
    pub fn dblhorscanen(&self) -> DblhorscanenR {
        DblhorscanenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set to 1, LVDS interface is enabled"]
    #[inline(always)]
    pub fn lvdsinten(&self) -> LvdsintenR {
        LvdsintenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When set to 1, the following output color formats are enabled : Byte-3 beat Interface enabled, Byte-4 beat (RGBX) Interface enabled, Two phase serial 12-bit enabled, YUYV (16-bit mode) enabled, BT.656 enabled, JDI MIP enabled"]
    #[inline(always)]
    pub fn yuyven(&self) -> YuyvenR {
        YuyvenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When set to 1, DBI Type-B interface is enabled"]
    #[inline(always)]
    pub fn dbitypeben(&self) -> DbitypebenR {
        DbitypebenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - Display data format"]
    #[inline(always)]
    pub fn dispfmt(&self) -> DispfmtR {
        DispfmtR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Output color format:"]
    #[inline(always)]
    pub fn colfmt(&self) -> ColfmtR {
        ColfmtR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When set to 1, LVDS output pads are enabled"]
    #[inline(always)]
    pub fn lvdspadsen(&self) -> LvdspadsenR {
        LvdspadsenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When set to 1, PLL_CLK is not divided"]
    #[inline(always)]
    pub fn pllclkndiv(&self) -> PllclkndivR {
        PllclkndivR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:16 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd0(&self) -> Rsvd0R {
        Rsvd0R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bit 17 - When set to 1, single frame update is enabled"]
    #[inline(always)]
    pub fn frameupdten(&self) -> FrameupdtenR {
        FrameupdtenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd1(&self) -> Rsvd1R {
        Rsvd1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - When set to 1, forces output to blank"]
    #[inline(always)]
    pub fn blankfrc(&self) -> BlankfrcR {
        BlankfrcR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - When set to 1, gamma ramp is enabled"]
    #[inline(always)]
    pub fn gamarampen(&self) -> GamarampenR {
        GamarampenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Defines Pixel Clock out polarity"]
    #[inline(always)]
    pub fn pixclkpol(&self) -> PixclkpolR {
        PixclkpolR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - When set to 1, VSYNC for a single cycle per line is enabled"]
    #[inline(always)]
    pub fn vsyncen(&self) -> VsyncenR {
        VsyncenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - When set to 1, dithering is enabled"]
    #[inline(always)]
    pub fn ditheren(&self) -> DitherenR {
        DitherenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Defines DE polarity"]
    #[inline(always)]
    pub fn depol(&self) -> DepolR {
        DepolR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Defines HSYNC polarity"]
    #[inline(always)]
    pub fn hsyncpol(&self) -> HsyncpolR {
        HsyncpolR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Defines VSYNC polarity"]
    #[inline(always)]
    pub fn vsyncpol(&self) -> VsyncpolR {
        VsyncpolR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd4(&self) -> Rsvd4R {
        Rsvd4R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When set to 1, programmable cursor is enabled"]
    #[inline(always)]
    pub fn cusoren(&self) -> CusorenR {
        CusorenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When set to 1, the dc400 controller is activated"]
    #[inline(always)]
    pub fn dc400act(&self) -> Dc400actR {
        Dc400actR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set to 1, test mode is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn tstmoden(&mut self) -> TstmodenW<ModeSpec> {
        TstmodenW::new(self, 0)
    }
    #[doc = "Bit 1 - When set to 1, double horizontal scan is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dblhorscanen(&mut self) -> DblhorscanenW<ModeSpec> {
        DblhorscanenW::new(self, 1)
    }
    #[doc = "Bit 2 - When set to 1, LVDS interface is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn lvdsinten(&mut self) -> LvdsintenW<ModeSpec> {
        LvdsintenW::new(self, 2)
    }
    #[doc = "Bit 3 - When set to 1, the following output color formats are enabled : Byte-3 beat Interface enabled, Byte-4 beat (RGBX) Interface enabled, Two phase serial 12-bit enabled, YUYV (16-bit mode) enabled, BT.656 enabled, JDI MIP enabled"]
    #[inline(always)]
    #[must_use]
    pub fn yuyven(&mut self) -> YuyvenW<ModeSpec> {
        YuyvenW::new(self, 3)
    }
    #[doc = "Bit 4 - When set to 1, DBI Type-B interface is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dbitypeben(&mut self) -> DbitypebenW<ModeSpec> {
        DbitypebenW::new(self, 4)
    }
    #[doc = "Bits 5:8 - Display data format"]
    #[inline(always)]
    #[must_use]
    pub fn dispfmt(&mut self) -> DispfmtW<ModeSpec> {
        DispfmtW::new(self, 5)
    }
    #[doc = "Bit 9 - Output color format:"]
    #[inline(always)]
    #[must_use]
    pub fn colfmt(&mut self) -> ColfmtW<ModeSpec> {
        ColfmtW::new(self, 9)
    }
    #[doc = "Bit 10 - When set to 1, LVDS output pads are enabled"]
    #[inline(always)]
    #[must_use]
    pub fn lvdspadsen(&mut self) -> LvdspadsenW<ModeSpec> {
        LvdspadsenW::new(self, 10)
    }
    #[doc = "Bit 11 - When set to 1, PLL_CLK is not divided"]
    #[inline(always)]
    #[must_use]
    pub fn pllclkndiv(&mut self) -> PllclkndivW<ModeSpec> {
        PllclkndivW::new(self, 11)
    }
    #[doc = "Bits 12:16 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd0(&mut self) -> Rsvd0W<ModeSpec> {
        Rsvd0W::new(self, 12)
    }
    #[doc = "Bit 17 - When set to 1, single frame update is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn frameupdten(&mut self) -> FrameupdtenW<ModeSpec> {
        FrameupdtenW::new(self, 17)
    }
    #[doc = "Bit 18 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd1(&mut self) -> Rsvd1W<ModeSpec> {
        Rsvd1W::new(self, 18)
    }
    #[doc = "Bit 19 - When set to 1, forces output to blank"]
    #[inline(always)]
    #[must_use]
    pub fn blankfrc(&mut self) -> BlankfrcW<ModeSpec> {
        BlankfrcW::new(self, 19)
    }
    #[doc = "Bit 20 - When set to 1, gamma ramp is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn gamarampen(&mut self) -> GamarampenW<ModeSpec> {
        GamarampenW::new(self, 20)
    }
    #[doc = "Bit 21 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd2(&mut self) -> Rsvd2W<ModeSpec> {
        Rsvd2W::new(self, 21)
    }
    #[doc = "Bit 22 - Defines Pixel Clock out polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pixclkpol(&mut self) -> PixclkpolW<ModeSpec> {
        PixclkpolW::new(self, 22)
    }
    #[doc = "Bit 23 - When set to 1, VSYNC for a single cycle per line is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn vsyncen(&mut self) -> VsyncenW<ModeSpec> {
        VsyncenW::new(self, 23)
    }
    #[doc = "Bit 24 - When set to 1, dithering is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ditheren(&mut self) -> DitherenW<ModeSpec> {
        DitherenW::new(self, 24)
    }
    #[doc = "Bit 25 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd3(&mut self) -> Rsvd3W<ModeSpec> {
        Rsvd3W::new(self, 25)
    }
    #[doc = "Bit 26 - Defines DE polarity"]
    #[inline(always)]
    #[must_use]
    pub fn depol(&mut self) -> DepolW<ModeSpec> {
        DepolW::new(self, 26)
    }
    #[doc = "Bit 27 - Defines HSYNC polarity"]
    #[inline(always)]
    #[must_use]
    pub fn hsyncpol(&mut self) -> HsyncpolW<ModeSpec> {
        HsyncpolW::new(self, 27)
    }
    #[doc = "Bit 28 - Defines VSYNC polarity"]
    #[inline(always)]
    #[must_use]
    pub fn vsyncpol(&mut self) -> VsyncpolW<ModeSpec> {
        VsyncpolW::new(self, 28)
    }
    #[doc = "Bit 29 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd4(&mut self) -> Rsvd4W<ModeSpec> {
        Rsvd4W::new(self, 29)
    }
    #[doc = "Bit 30 - When set to 1, programmable cursor is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cusoren(&mut self) -> CusorenW<ModeSpec> {
        CusorenW::new(self, 30)
    }
    #[doc = "Bit 31 - When set to 1, the dc400 controller is activated"]
    #[inline(always)]
    #[must_use]
    pub fn dc400act(&mut self) -> Dc400actW<ModeSpec> {
        Dc400actW::new(self, 31)
    }
}
#[doc = "General control register that activates the NEMAp|dc400 controller and various parameters, sets the timing signals' polarity, activates the global look-up table for gamma correction and chooses the output display formats to meet LCD color specifications.\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeSpec;
impl crate::RegisterSpec for ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for ModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for ModeSpec {
    const RESET_VALUE: u32 = 0;
}
