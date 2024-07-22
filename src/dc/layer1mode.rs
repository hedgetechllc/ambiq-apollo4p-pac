#[doc = "Register `LAYER1MODE` reader"]
pub type R = crate::R<Layer1modeSpec>;
#[doc = "Register `LAYER1MODE` writer"]
pub type W = crate::W<Layer1modeSpec>;
#[doc = "Color mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Layer1colormode {
    #[doc = "0: 8-bit color palette look-up table (LUT8)"]
    Layer1Lutble = 0,
    #[doc = "1: 16-bit RGBX5551 color format"]
    Layer1Rgbx5551 = 1,
    #[doc = "2: 32-bit RGBX8888 color format"]
    Layer1Rgbx8888 = 2,
    #[doc = "4: 8-bit RGB332 color format"]
    Layer1Rgb332 = 4,
    #[doc = "5: 16-bit RGB565 color format"]
    Layer1Rgb565 = 5,
    #[doc = "6: 32-bit XRGB8888 color format"]
    Layer1Xrgb8888 = 6,
    #[doc = "7: L8 Grayscale/Palette format"]
    Layer1L8 = 7,
    #[doc = "8: L1 Grayscale/Palette format"]
    Layer1L1 = 8,
    #[doc = "9: L4 Grayscale/Palette format"]
    Layer1L4 = 9,
    #[doc = "10: YUYV color format"]
    Layer1Yuyv = 10,
    #[doc = "11: 24-bit RGB color format"]
    Layer1Rgb = 11,
    #[doc = "12: YUY2 color format"]
    Layer1Yuy2 = 12,
    #[doc = "13: 32-bit ABGR8888 color format"]
    Layer1Abgr8888 = 13,
    #[doc = "14: 32-bit BGRA8888 color format"]
    Layer1Bgra8888 = 14,
    #[doc = "16: Video 420 Mode"]
    Layer1Video420 = 16,
    #[doc = "17: Trilinear 420 Video Mode"]
    Layer1Trilin420 = 17,
}
impl From<Layer1colormode> for u8 {
    #[inline(always)]
    fn from(variant: Layer1colormode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Layer1colormode {
    type Ux = u8;
}
impl crate::IsEnum for Layer1colormode {}
#[doc = "Field `LAYER1COLORMODE` reader - Color mode"]
pub type Layer1colormodeR = crate::FieldReader<Layer1colormode>;
impl Layer1colormodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Layer1colormode> {
        match self.bits {
            0 => Some(Layer1colormode::Layer1Lutble),
            1 => Some(Layer1colormode::Layer1Rgbx5551),
            2 => Some(Layer1colormode::Layer1Rgbx8888),
            4 => Some(Layer1colormode::Layer1Rgb332),
            5 => Some(Layer1colormode::Layer1Rgb565),
            6 => Some(Layer1colormode::Layer1Xrgb8888),
            7 => Some(Layer1colormode::Layer1L8),
            8 => Some(Layer1colormode::Layer1L1),
            9 => Some(Layer1colormode::Layer1L4),
            10 => Some(Layer1colormode::Layer1Yuyv),
            11 => Some(Layer1colormode::Layer1Rgb),
            12 => Some(Layer1colormode::Layer1Yuy2),
            13 => Some(Layer1colormode::Layer1Abgr8888),
            14 => Some(Layer1colormode::Layer1Bgra8888),
            16 => Some(Layer1colormode::Layer1Video420),
            17 => Some(Layer1colormode::Layer1Trilin420),
            _ => None,
        }
    }
    #[doc = "8-bit color palette look-up table (LUT8)"]
    #[inline(always)]
    pub fn is_layer1_lutble(&self) -> bool {
        *self == Layer1colormode::Layer1Lutble
    }
    #[doc = "16-bit RGBX5551 color format"]
    #[inline(always)]
    pub fn is_layer1_rgbx5551(&self) -> bool {
        *self == Layer1colormode::Layer1Rgbx5551
    }
    #[doc = "32-bit RGBX8888 color format"]
    #[inline(always)]
    pub fn is_layer1_rgbx8888(&self) -> bool {
        *self == Layer1colormode::Layer1Rgbx8888
    }
    #[doc = "8-bit RGB332 color format"]
    #[inline(always)]
    pub fn is_layer1_rgb332(&self) -> bool {
        *self == Layer1colormode::Layer1Rgb332
    }
    #[doc = "16-bit RGB565 color format"]
    #[inline(always)]
    pub fn is_layer1_rgb565(&self) -> bool {
        *self == Layer1colormode::Layer1Rgb565
    }
    #[doc = "32-bit XRGB8888 color format"]
    #[inline(always)]
    pub fn is_layer1_xrgb8888(&self) -> bool {
        *self == Layer1colormode::Layer1Xrgb8888
    }
    #[doc = "L8 Grayscale/Palette format"]
    #[inline(always)]
    pub fn is_layer1_l8(&self) -> bool {
        *self == Layer1colormode::Layer1L8
    }
    #[doc = "L1 Grayscale/Palette format"]
    #[inline(always)]
    pub fn is_layer1_l1(&self) -> bool {
        *self == Layer1colormode::Layer1L1
    }
    #[doc = "L4 Grayscale/Palette format"]
    #[inline(always)]
    pub fn is_layer1_l4(&self) -> bool {
        *self == Layer1colormode::Layer1L4
    }
    #[doc = "YUYV color format"]
    #[inline(always)]
    pub fn is_layer1_yuyv(&self) -> bool {
        *self == Layer1colormode::Layer1Yuyv
    }
    #[doc = "24-bit RGB color format"]
    #[inline(always)]
    pub fn is_layer1_rgb(&self) -> bool {
        *self == Layer1colormode::Layer1Rgb
    }
    #[doc = "YUY2 color format"]
    #[inline(always)]
    pub fn is_layer1_yuy2(&self) -> bool {
        *self == Layer1colormode::Layer1Yuy2
    }
    #[doc = "32-bit ABGR8888 color format"]
    #[inline(always)]
    pub fn is_layer1_abgr8888(&self) -> bool {
        *self == Layer1colormode::Layer1Abgr8888
    }
    #[doc = "32-bit BGRA8888 color format"]
    #[inline(always)]
    pub fn is_layer1_bgra8888(&self) -> bool {
        *self == Layer1colormode::Layer1Bgra8888
    }
    #[doc = "Video 420 Mode"]
    #[inline(always)]
    pub fn is_layer1_video420(&self) -> bool {
        *self == Layer1colormode::Layer1Video420
    }
    #[doc = "Trilinear 420 Video Mode"]
    #[inline(always)]
    pub fn is_layer1_trilin420(&self) -> bool {
        *self == Layer1colormode::Layer1Trilin420
    }
}
#[doc = "Field `LAYER1COLORMODE` writer - Color mode"]
pub type Layer1colormodeW<'a, REG> = crate::FieldWriter<'a, REG, 5, Layer1colormode>;
impl<'a, REG> Layer1colormodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-bit color palette look-up table (LUT8)"]
    #[inline(always)]
    pub fn layer1_lutble(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1colormode::Layer1Lutble)
    }
    #[doc = "16-bit RGBX5551 color format"]
    #[inline(always)]
    pub fn layer1_rgbx5551(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1colormode::Layer1Rgbx5551)
    }
    #[doc = "32-bit RGBX8888 color format"]
    #[inline(always)]
    pub fn layer1_rgbx8888(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1colormode::Layer1Rgbx8888)
    }
    #[doc = "8-bit RGB332 color format"]
    #[inline(always)]
    pub fn layer1_rgb332(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1colormode::Layer1Rgb332)
    }
    #[doc = "16-bit RGB565 color format"]
    #[inline(always)]
    pub fn layer1_rgb565(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1colormode::Layer1Rgb565)
    }
    #[doc = "32-bit XRGB8888 color format"]
    #[inline(always)]
    pub fn layer1_xrgb8888(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1colormode::Layer1Xrgb8888)
    }
    #[doc = "L8 Grayscale/Palette format"]
    #[inline(always)]
    pub fn layer1_l8(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1colormode::Layer1L8)
    }
    #[doc = "L1 Grayscale/Palette format"]
    #[inline(always)]
    pub fn layer1_l1(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1colormode::Layer1L1)
    }
    #[doc = "L4 Grayscale/Palette format"]
    #[inline(always)]
    pub fn layer1_l4(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1colormode::Layer1L4)
    }
    #[doc = "YUYV color format"]
    #[inline(always)]
    pub fn layer1_yuyv(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1colormode::Layer1Yuyv)
    }
    #[doc = "24-bit RGB color format"]
    #[inline(always)]
    pub fn layer1_rgb(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1colormode::Layer1Rgb)
    }
    #[doc = "YUY2 color format"]
    #[inline(always)]
    pub fn layer1_yuy2(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1colormode::Layer1Yuy2)
    }
    #[doc = "32-bit ABGR8888 color format"]
    #[inline(always)]
    pub fn layer1_abgr8888(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1colormode::Layer1Abgr8888)
    }
    #[doc = "32-bit BGRA8888 color format"]
    #[inline(always)]
    pub fn layer1_bgra8888(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1colormode::Layer1Bgra8888)
    }
    #[doc = "Video 420 Mode"]
    #[inline(always)]
    pub fn layer1_video420(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1colormode::Layer1Video420)
    }
    #[doc = "Trilinear 420 Video Mode"]
    #[inline(always)]
    pub fn layer1_trilin420(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1colormode::Layer1Trilin420)
    }
}
#[doc = "Field `RSVD0` reader - This field is reserved."]
pub type Rsvd0R = crate::FieldReader;
#[doc = "Field `RSVD0` writer - This field is reserved."]
pub type Rsvd0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Source blending function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Layer1sblend {
    #[doc = "0: blend black"]
    Layer1SblackBlend = 0,
    #[doc = "1: blend white"]
    Layer1SwhiteBlend = 1,
    #[doc = "2: blend alpha source"]
    Layer1SalbhasBlend = 2,
    #[doc = "3: blend alpha global"]
    Layer1SalphagBlend = 3,
    #[doc = "4: blend alpha source and alpha global"]
    Layer1SalphasgBlend = 4,
    #[doc = "5: blend inverted source"]
    Layer1SinvertBlendSrc = 5,
    #[doc = "6: blend inverted global"]
    Layer1SinvertGlobalBlend = 6,
    #[doc = "7: blend inverted source and inverted global"]
    Layer1SinvertsgBlend = 7,
    #[doc = "10: blend alpha destination"]
    Layer1SalphaBlend = 10,
    #[doc = "13: blend inverted destination"]
    Layer1SinvertBlendDst = 13,
}
impl From<Layer1sblend> for u8 {
    #[inline(always)]
    fn from(variant: Layer1sblend) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Layer1sblend {
    type Ux = u8;
}
impl crate::IsEnum for Layer1sblend {}
#[doc = "Field `LAYER1SBLEND` reader - Source blending function"]
pub type Layer1sblendR = crate::FieldReader<Layer1sblend>;
impl Layer1sblendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Layer1sblend> {
        match self.bits {
            0 => Some(Layer1sblend::Layer1SblackBlend),
            1 => Some(Layer1sblend::Layer1SwhiteBlend),
            2 => Some(Layer1sblend::Layer1SalbhasBlend),
            3 => Some(Layer1sblend::Layer1SalphagBlend),
            4 => Some(Layer1sblend::Layer1SalphasgBlend),
            5 => Some(Layer1sblend::Layer1SinvertBlendSrc),
            6 => Some(Layer1sblend::Layer1SinvertGlobalBlend),
            7 => Some(Layer1sblend::Layer1SinvertsgBlend),
            10 => Some(Layer1sblend::Layer1SalphaBlend),
            13 => Some(Layer1sblend::Layer1SinvertBlendDst),
            _ => None,
        }
    }
    #[doc = "blend black"]
    #[inline(always)]
    pub fn is_layer1_sblack_blend(&self) -> bool {
        *self == Layer1sblend::Layer1SblackBlend
    }
    #[doc = "blend white"]
    #[inline(always)]
    pub fn is_layer1_swhite_blend(&self) -> bool {
        *self == Layer1sblend::Layer1SwhiteBlend
    }
    #[doc = "blend alpha source"]
    #[inline(always)]
    pub fn is_layer1_salbhas_blend(&self) -> bool {
        *self == Layer1sblend::Layer1SalbhasBlend
    }
    #[doc = "blend alpha global"]
    #[inline(always)]
    pub fn is_layer1_salphag_blend(&self) -> bool {
        *self == Layer1sblend::Layer1SalphagBlend
    }
    #[doc = "blend alpha source and alpha global"]
    #[inline(always)]
    pub fn is_layer1_salphasg_blend(&self) -> bool {
        *self == Layer1sblend::Layer1SalphasgBlend
    }
    #[doc = "blend inverted source"]
    #[inline(always)]
    pub fn is_layer1_sinvert_blend_src(&self) -> bool {
        *self == Layer1sblend::Layer1SinvertBlendSrc
    }
    #[doc = "blend inverted global"]
    #[inline(always)]
    pub fn is_layer1_sinvert_global_blend(&self) -> bool {
        *self == Layer1sblend::Layer1SinvertGlobalBlend
    }
    #[doc = "blend inverted source and inverted global"]
    #[inline(always)]
    pub fn is_layer1_sinvertsg_blend(&self) -> bool {
        *self == Layer1sblend::Layer1SinvertsgBlend
    }
    #[doc = "blend alpha destination"]
    #[inline(always)]
    pub fn is_layer1_salpha_blend(&self) -> bool {
        *self == Layer1sblend::Layer1SalphaBlend
    }
    #[doc = "blend inverted destination"]
    #[inline(always)]
    pub fn is_layer1_sinvert_blend_dst(&self) -> bool {
        *self == Layer1sblend::Layer1SinvertBlendDst
    }
}
#[doc = "Field `LAYER1SBLEND` writer - Source blending function"]
pub type Layer1sblendW<'a, REG> = crate::FieldWriter<'a, REG, 4, Layer1sblend>;
impl<'a, REG> Layer1sblendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "blend black"]
    #[inline(always)]
    pub fn layer1_sblack_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1sblend::Layer1SblackBlend)
    }
    #[doc = "blend white"]
    #[inline(always)]
    pub fn layer1_swhite_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1sblend::Layer1SwhiteBlend)
    }
    #[doc = "blend alpha source"]
    #[inline(always)]
    pub fn layer1_salbhas_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1sblend::Layer1SalbhasBlend)
    }
    #[doc = "blend alpha global"]
    #[inline(always)]
    pub fn layer1_salphag_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1sblend::Layer1SalphagBlend)
    }
    #[doc = "blend alpha source and alpha global"]
    #[inline(always)]
    pub fn layer1_salphasg_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1sblend::Layer1SalphasgBlend)
    }
    #[doc = "blend inverted source"]
    #[inline(always)]
    pub fn layer1_sinvert_blend_src(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1sblend::Layer1SinvertBlendSrc)
    }
    #[doc = "blend inverted global"]
    #[inline(always)]
    pub fn layer1_sinvert_global_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1sblend::Layer1SinvertGlobalBlend)
    }
    #[doc = "blend inverted source and inverted global"]
    #[inline(always)]
    pub fn layer1_sinvertsg_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1sblend::Layer1SinvertsgBlend)
    }
    #[doc = "blend alpha destination"]
    #[inline(always)]
    pub fn layer1_salpha_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1sblend::Layer1SalphaBlend)
    }
    #[doc = "blend inverted destination"]
    #[inline(always)]
    pub fn layer1_sinvert_blend_dst(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1sblend::Layer1SinvertBlendDst)
    }
}
#[doc = "Destination blending function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Layer1dblend {
    #[doc = "0: blend black"]
    Layer1DblackBlend = 0,
    #[doc = "1: blend white"]
    Layer1DwhiteBlend = 1,
    #[doc = "2: blend alpha source"]
    Layer1DalbhasBlend = 2,
    #[doc = "3: blend alpha global"]
    Layer1DalphagBlend = 3,
    #[doc = "4: blend alpha source and alpha global"]
    Layer1DalphasgBlend = 4,
    #[doc = "5: blend inverted source"]
    Layer1DinvertBlendSrc = 5,
    #[doc = "6: blend inverted global"]
    Layer1DinvertGlobalBlend = 6,
    #[doc = "7: blend inverted source and inverted global"]
    Layer1DinvertsgBlend = 7,
    #[doc = "10: blend alpha destination"]
    Layer1DalphaBlend = 10,
    #[doc = "13: blend inverted destination"]
    Layer1DinvertBlendDst = 13,
}
impl From<Layer1dblend> for u8 {
    #[inline(always)]
    fn from(variant: Layer1dblend) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Layer1dblend {
    type Ux = u8;
}
impl crate::IsEnum for Layer1dblend {}
#[doc = "Field `LAYER1DBLEND` reader - Destination blending function"]
pub type Layer1dblendR = crate::FieldReader<Layer1dblend>;
impl Layer1dblendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Layer1dblend> {
        match self.bits {
            0 => Some(Layer1dblend::Layer1DblackBlend),
            1 => Some(Layer1dblend::Layer1DwhiteBlend),
            2 => Some(Layer1dblend::Layer1DalbhasBlend),
            3 => Some(Layer1dblend::Layer1DalphagBlend),
            4 => Some(Layer1dblend::Layer1DalphasgBlend),
            5 => Some(Layer1dblend::Layer1DinvertBlendSrc),
            6 => Some(Layer1dblend::Layer1DinvertGlobalBlend),
            7 => Some(Layer1dblend::Layer1DinvertsgBlend),
            10 => Some(Layer1dblend::Layer1DalphaBlend),
            13 => Some(Layer1dblend::Layer1DinvertBlendDst),
            _ => None,
        }
    }
    #[doc = "blend black"]
    #[inline(always)]
    pub fn is_layer1_dblack_blend(&self) -> bool {
        *self == Layer1dblend::Layer1DblackBlend
    }
    #[doc = "blend white"]
    #[inline(always)]
    pub fn is_layer1_dwhite_blend(&self) -> bool {
        *self == Layer1dblend::Layer1DwhiteBlend
    }
    #[doc = "blend alpha source"]
    #[inline(always)]
    pub fn is_layer1_dalbhas_blend(&self) -> bool {
        *self == Layer1dblend::Layer1DalbhasBlend
    }
    #[doc = "blend alpha global"]
    #[inline(always)]
    pub fn is_layer1_dalphag_blend(&self) -> bool {
        *self == Layer1dblend::Layer1DalphagBlend
    }
    #[doc = "blend alpha source and alpha global"]
    #[inline(always)]
    pub fn is_layer1_dalphasg_blend(&self) -> bool {
        *self == Layer1dblend::Layer1DalphasgBlend
    }
    #[doc = "blend inverted source"]
    #[inline(always)]
    pub fn is_layer1_dinvert_blend_src(&self) -> bool {
        *self == Layer1dblend::Layer1DinvertBlendSrc
    }
    #[doc = "blend inverted global"]
    #[inline(always)]
    pub fn is_layer1_dinvert_global_blend(&self) -> bool {
        *self == Layer1dblend::Layer1DinvertGlobalBlend
    }
    #[doc = "blend inverted source and inverted global"]
    #[inline(always)]
    pub fn is_layer1_dinvertsg_blend(&self) -> bool {
        *self == Layer1dblend::Layer1DinvertsgBlend
    }
    #[doc = "blend alpha destination"]
    #[inline(always)]
    pub fn is_layer1_dalpha_blend(&self) -> bool {
        *self == Layer1dblend::Layer1DalphaBlend
    }
    #[doc = "blend inverted destination"]
    #[inline(always)]
    pub fn is_layer1_dinvert_blend_dst(&self) -> bool {
        *self == Layer1dblend::Layer1DinvertBlendDst
    }
}
#[doc = "Field `LAYER1DBLEND` writer - Destination blending function"]
pub type Layer1dblendW<'a, REG> = crate::FieldWriter<'a, REG, 4, Layer1dblend>;
impl<'a, REG> Layer1dblendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "blend black"]
    #[inline(always)]
    pub fn layer1_dblack_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1dblend::Layer1DblackBlend)
    }
    #[doc = "blend white"]
    #[inline(always)]
    pub fn layer1_dwhite_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1dblend::Layer1DwhiteBlend)
    }
    #[doc = "blend alpha source"]
    #[inline(always)]
    pub fn layer1_dalbhas_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1dblend::Layer1DalbhasBlend)
    }
    #[doc = "blend alpha global"]
    #[inline(always)]
    pub fn layer1_dalphag_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1dblend::Layer1DalphagBlend)
    }
    #[doc = "blend alpha source and alpha global"]
    #[inline(always)]
    pub fn layer1_dalphasg_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1dblend::Layer1DalphasgBlend)
    }
    #[doc = "blend inverted source"]
    #[inline(always)]
    pub fn layer1_dinvert_blend_src(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1dblend::Layer1DinvertBlendSrc)
    }
    #[doc = "blend inverted global"]
    #[inline(always)]
    pub fn layer1_dinvert_global_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1dblend::Layer1DinvertGlobalBlend)
    }
    #[doc = "blend inverted source and inverted global"]
    #[inline(always)]
    pub fn layer1_dinvertsg_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1dblend::Layer1DinvertsgBlend)
    }
    #[doc = "blend alpha destination"]
    #[inline(always)]
    pub fn layer1_dalpha_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1dblend::Layer1DalphaBlend)
    }
    #[doc = "blend inverted destination"]
    #[inline(always)]
    pub fn layer1_dinvert_blend_dst(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1dblend::Layer1DinvertBlendDst)
    }
}
#[doc = "Field `LAYER1ALPHA` reader - Alpha layer global value (0x00-0xFF range)"]
pub type Layer1alphaR = crate::FieldReader;
#[doc = "Field `LAYER1ALPHA` writer - Alpha layer global value (0x00-0xFF range)"]
pub type Layer1alphaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RSVD1` reader - This field is reserved."]
pub type Rsvd1R = crate::FieldReader;
#[doc = "Field `RSVD1` writer - This field is reserved."]
pub type Rsvd1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LAYER1GAMMA` reader - When set to 1, Gamma Look Up Table is enabled"]
pub type Layer1gammaR = crate::BitReader;
#[doc = "Field `LAYER1GAMMA` writer - When set to 1, Gamma Look Up Table is enabled"]
pub type Layer1gammaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAYER1HLOCK` reader - When set to 1, HLOCK signal on AHB DMAs is asserted"]
pub type Layer1hlockR = crate::BitReader;
#[doc = "Field `LAYER1HLOCK` writer - When set to 1, HLOCK signal on AHB DMAs is asserted"]
pub type Layer1hlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAYER1PREMULT` reader - When set to 1, premultiply image alpha is enabled"]
pub type Layer1premultR = crate::BitReader;
#[doc = "Field `LAYER1PREMULT` writer - When set to 1, premultiply image alpha is enabled"]
pub type Layer1premultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAYER1BFILTER` reader - When set to 1, bilinear filtering is enabled"]
pub type Layer1bfilterR = crate::BitReader;
#[doc = "Field `LAYER1BFILTER` writer - When set to 1, bilinear filtering is enabled"]
pub type Layer1bfilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAYER1FORCE` reader - When set to 1, force alpha with global alpha is enabled"]
pub type Layer1forceR = crate::BitReader;
#[doc = "Field `LAYER1FORCE` writer - When set to 1, force alpha with global alpha is enabled"]
pub type Layer1forceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAYER1EN` reader - When set to 1, layer n is enabled"]
pub type Layer1enR = crate::BitReader;
#[doc = "Field `LAYER1EN` writer - When set to 1, layer n is enabled"]
pub type Layer1enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Color mode"]
    #[inline(always)]
    pub fn layer1colormode(&self) -> Layer1colormodeR {
        Layer1colormodeR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd0(&self) -> Rsvd0R {
        Rsvd0R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Source blending function"]
    #[inline(always)]
    pub fn layer1sblend(&self) -> Layer1sblendR {
        Layer1sblendR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Destination blending function"]
    #[inline(always)]
    pub fn layer1dblend(&self) -> Layer1dblendR {
        Layer1dblendR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Alpha layer global value (0x00-0xFF range)"]
    #[inline(always)]
    pub fn layer1alpha(&self) -> Layer1alphaR {
        Layer1alphaR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd1(&self) -> Rsvd1R {
        Rsvd1R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - When set to 1, Gamma Look Up Table is enabled"]
    #[inline(always)]
    pub fn layer1gamma(&self) -> Layer1gammaR {
        Layer1gammaR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - When set to 1, HLOCK signal on AHB DMAs is asserted"]
    #[inline(always)]
    pub fn layer1hlock(&self) -> Layer1hlockR {
        Layer1hlockR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - When set to 1, premultiply image alpha is enabled"]
    #[inline(always)]
    pub fn layer1premult(&self) -> Layer1premultR {
        Layer1premultR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - When set to 1, bilinear filtering is enabled"]
    #[inline(always)]
    pub fn layer1bfilter(&self) -> Layer1bfilterR {
        Layer1bfilterR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When set to 1, force alpha with global alpha is enabled"]
    #[inline(always)]
    pub fn layer1force(&self) -> Layer1forceR {
        Layer1forceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When set to 1, layer n is enabled"]
    #[inline(always)]
    pub fn layer1en(&self) -> Layer1enR {
        Layer1enR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Color mode"]
    #[inline(always)]
    #[must_use]
    pub fn layer1colormode(&mut self) -> Layer1colormodeW<Layer1modeSpec> {
        Layer1colormodeW::new(self, 0)
    }
    #[doc = "Bits 5:7 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd0(&mut self) -> Rsvd0W<Layer1modeSpec> {
        Rsvd0W::new(self, 5)
    }
    #[doc = "Bits 8:11 - Source blending function"]
    #[inline(always)]
    #[must_use]
    pub fn layer1sblend(&mut self) -> Layer1sblendW<Layer1modeSpec> {
        Layer1sblendW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Destination blending function"]
    #[inline(always)]
    #[must_use]
    pub fn layer1dblend(&mut self) -> Layer1dblendW<Layer1modeSpec> {
        Layer1dblendW::new(self, 12)
    }
    #[doc = "Bits 16:23 - Alpha layer global value (0x00-0xFF range)"]
    #[inline(always)]
    #[must_use]
    pub fn layer1alpha(&mut self) -> Layer1alphaW<Layer1modeSpec> {
        Layer1alphaW::new(self, 16)
    }
    #[doc = "Bits 24:25 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd1(&mut self) -> Rsvd1W<Layer1modeSpec> {
        Rsvd1W::new(self, 24)
    }
    #[doc = "Bit 26 - When set to 1, Gamma Look Up Table is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn layer1gamma(&mut self) -> Layer1gammaW<Layer1modeSpec> {
        Layer1gammaW::new(self, 26)
    }
    #[doc = "Bit 27 - When set to 1, HLOCK signal on AHB DMAs is asserted"]
    #[inline(always)]
    #[must_use]
    pub fn layer1hlock(&mut self) -> Layer1hlockW<Layer1modeSpec> {
        Layer1hlockW::new(self, 27)
    }
    #[doc = "Bit 28 - When set to 1, premultiply image alpha is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn layer1premult(&mut self) -> Layer1premultW<Layer1modeSpec> {
        Layer1premultW::new(self, 28)
    }
    #[doc = "Bit 29 - When set to 1, bilinear filtering is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn layer1bfilter(&mut self) -> Layer1bfilterW<Layer1modeSpec> {
        Layer1bfilterW::new(self, 29)
    }
    #[doc = "Bit 30 - When set to 1, force alpha with global alpha is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn layer1force(&mut self) -> Layer1forceW<Layer1modeSpec> {
        Layer1forceW::new(self, 30)
    }
    #[doc = "Bit 31 - When set to 1, layer n is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn layer1en(&mut self) -> Layer1enW<Layer1modeSpec> {
        Layer1enW::new(self, 31)
    }
}
#[doc = "Activate and set-up layer 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer1mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer1mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer1modeSpec;
impl crate::RegisterSpec for Layer1modeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer1mode::R`](R) reader structure"]
impl crate::Readable for Layer1modeSpec {}
#[doc = "`write(|w| ..)` method takes [`layer1mode::W`](W) writer structure"]
impl crate::Writable for Layer1modeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER1MODE to value 0"]
impl crate::Resettable for Layer1modeSpec {
    const RESET_VALUE: u32 = 0;
}
