#[doc = "Register `LAYER3MODE` reader"]
pub type R = crate::R<Layer3modeSpec>;
#[doc = "Register `LAYER3MODE` writer"]
pub type W = crate::W<Layer3modeSpec>;
#[doc = "Color mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Layer3colormode {
    #[doc = "0: 8-bit color palette look-up table (LUT8)"]
    Layer3Lutble = 0,
    #[doc = "1: 16-bit RGBX5551 color format"]
    Layer3Rgbx5551 = 1,
    #[doc = "2: 32-bit RGBX8888 color format"]
    Layer3Rgbx8888 = 2,
    #[doc = "4: 8-bit RGB332 color format"]
    Layer3Rgb332 = 4,
    #[doc = "5: 16-bit RGB565 color format"]
    Layer3Rgb565 = 5,
    #[doc = "6: 32-bit XRGB8888 color format"]
    Layer3Xrgb8888 = 6,
    #[doc = "7: L8 Grayscale/Palette format"]
    Layer3L8 = 7,
    #[doc = "8: L1 Grayscale/Palette format"]
    Layer3L1 = 8,
    #[doc = "9: L4 Grayscale/Palette format"]
    Layer3L4 = 9,
    #[doc = "10: YUYV color format"]
    Layer3Yuyv = 10,
    #[doc = "11: 24-bit RGB color format"]
    Layer3Rgb = 11,
    #[doc = "12: YUY2 color format"]
    Layer3Yuy2 = 12,
    #[doc = "13: 32-bit ABGR8888 color format"]
    Layer3Abgr8888 = 13,
    #[doc = "14: 32-bit BGRA8888 color format"]
    Layer3Bgra8888 = 14,
    #[doc = "16: Video 420 Mode"]
    Layer3Video420 = 16,
    #[doc = "17: Trilinear 420 Video Mode"]
    Layer3Trilin42 = 17,
}
impl From<Layer3colormode> for u8 {
    #[inline(always)]
    fn from(variant: Layer3colormode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Layer3colormode {
    type Ux = u8;
}
impl crate::IsEnum for Layer3colormode {}
#[doc = "Field `LAYER3COLORMODE` reader - Color mode"]
pub type Layer3colormodeR = crate::FieldReader<Layer3colormode>;
impl Layer3colormodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Layer3colormode> {
        match self.bits {
            0 => Some(Layer3colormode::Layer3Lutble),
            1 => Some(Layer3colormode::Layer3Rgbx5551),
            2 => Some(Layer3colormode::Layer3Rgbx8888),
            4 => Some(Layer3colormode::Layer3Rgb332),
            5 => Some(Layer3colormode::Layer3Rgb565),
            6 => Some(Layer3colormode::Layer3Xrgb8888),
            7 => Some(Layer3colormode::Layer3L8),
            8 => Some(Layer3colormode::Layer3L1),
            9 => Some(Layer3colormode::Layer3L4),
            10 => Some(Layer3colormode::Layer3Yuyv),
            11 => Some(Layer3colormode::Layer3Rgb),
            12 => Some(Layer3colormode::Layer3Yuy2),
            13 => Some(Layer3colormode::Layer3Abgr8888),
            14 => Some(Layer3colormode::Layer3Bgra8888),
            16 => Some(Layer3colormode::Layer3Video420),
            17 => Some(Layer3colormode::Layer3Trilin42),
            _ => None,
        }
    }
    #[doc = "8-bit color palette look-up table (LUT8)"]
    #[inline(always)]
    pub fn is_layer3_lutble(&self) -> bool {
        *self == Layer3colormode::Layer3Lutble
    }
    #[doc = "16-bit RGBX5551 color format"]
    #[inline(always)]
    pub fn is_layer3_rgbx5551(&self) -> bool {
        *self == Layer3colormode::Layer3Rgbx5551
    }
    #[doc = "32-bit RGBX8888 color format"]
    #[inline(always)]
    pub fn is_layer3_rgbx8888(&self) -> bool {
        *self == Layer3colormode::Layer3Rgbx8888
    }
    #[doc = "8-bit RGB332 color format"]
    #[inline(always)]
    pub fn is_layer3_rgb332(&self) -> bool {
        *self == Layer3colormode::Layer3Rgb332
    }
    #[doc = "16-bit RGB565 color format"]
    #[inline(always)]
    pub fn is_layer3_rgb565(&self) -> bool {
        *self == Layer3colormode::Layer3Rgb565
    }
    #[doc = "32-bit XRGB8888 color format"]
    #[inline(always)]
    pub fn is_layer3_xrgb8888(&self) -> bool {
        *self == Layer3colormode::Layer3Xrgb8888
    }
    #[doc = "L8 Grayscale/Palette format"]
    #[inline(always)]
    pub fn is_layer3_l8(&self) -> bool {
        *self == Layer3colormode::Layer3L8
    }
    #[doc = "L1 Grayscale/Palette format"]
    #[inline(always)]
    pub fn is_layer3_l1(&self) -> bool {
        *self == Layer3colormode::Layer3L1
    }
    #[doc = "L4 Grayscale/Palette format"]
    #[inline(always)]
    pub fn is_layer3_l4(&self) -> bool {
        *self == Layer3colormode::Layer3L4
    }
    #[doc = "YUYV color format"]
    #[inline(always)]
    pub fn is_layer3_yuyv(&self) -> bool {
        *self == Layer3colormode::Layer3Yuyv
    }
    #[doc = "24-bit RGB color format"]
    #[inline(always)]
    pub fn is_layer3_rgb(&self) -> bool {
        *self == Layer3colormode::Layer3Rgb
    }
    #[doc = "YUY2 color format"]
    #[inline(always)]
    pub fn is_layer3_yuy2(&self) -> bool {
        *self == Layer3colormode::Layer3Yuy2
    }
    #[doc = "32-bit ABGR8888 color format"]
    #[inline(always)]
    pub fn is_layer3_abgr8888(&self) -> bool {
        *self == Layer3colormode::Layer3Abgr8888
    }
    #[doc = "32-bit BGRA8888 color format"]
    #[inline(always)]
    pub fn is_layer3_bgra8888(&self) -> bool {
        *self == Layer3colormode::Layer3Bgra8888
    }
    #[doc = "Video 420 Mode"]
    #[inline(always)]
    pub fn is_layer3_video420(&self) -> bool {
        *self == Layer3colormode::Layer3Video420
    }
    #[doc = "Trilinear 420 Video Mode"]
    #[inline(always)]
    pub fn is_layer3_trilin42(&self) -> bool {
        *self == Layer3colormode::Layer3Trilin42
    }
}
#[doc = "Field `LAYER3COLORMODE` writer - Color mode"]
pub type Layer3colormodeW<'a, REG> = crate::FieldWriter<'a, REG, 5, Layer3colormode>;
impl<'a, REG> Layer3colormodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-bit color palette look-up table (LUT8)"]
    #[inline(always)]
    pub fn layer3_lutble(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3colormode::Layer3Lutble)
    }
    #[doc = "16-bit RGBX5551 color format"]
    #[inline(always)]
    pub fn layer3_rgbx5551(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3colormode::Layer3Rgbx5551)
    }
    #[doc = "32-bit RGBX8888 color format"]
    #[inline(always)]
    pub fn layer3_rgbx8888(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3colormode::Layer3Rgbx8888)
    }
    #[doc = "8-bit RGB332 color format"]
    #[inline(always)]
    pub fn layer3_rgb332(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3colormode::Layer3Rgb332)
    }
    #[doc = "16-bit RGB565 color format"]
    #[inline(always)]
    pub fn layer3_rgb565(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3colormode::Layer3Rgb565)
    }
    #[doc = "32-bit XRGB8888 color format"]
    #[inline(always)]
    pub fn layer3_xrgb8888(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3colormode::Layer3Xrgb8888)
    }
    #[doc = "L8 Grayscale/Palette format"]
    #[inline(always)]
    pub fn layer3_l8(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3colormode::Layer3L8)
    }
    #[doc = "L1 Grayscale/Palette format"]
    #[inline(always)]
    pub fn layer3_l1(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3colormode::Layer3L1)
    }
    #[doc = "L4 Grayscale/Palette format"]
    #[inline(always)]
    pub fn layer3_l4(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3colormode::Layer3L4)
    }
    #[doc = "YUYV color format"]
    #[inline(always)]
    pub fn layer3_yuyv(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3colormode::Layer3Yuyv)
    }
    #[doc = "24-bit RGB color format"]
    #[inline(always)]
    pub fn layer3_rgb(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3colormode::Layer3Rgb)
    }
    #[doc = "YUY2 color format"]
    #[inline(always)]
    pub fn layer3_yuy2(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3colormode::Layer3Yuy2)
    }
    #[doc = "32-bit ABGR8888 color format"]
    #[inline(always)]
    pub fn layer3_abgr8888(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3colormode::Layer3Abgr8888)
    }
    #[doc = "32-bit BGRA8888 color format"]
    #[inline(always)]
    pub fn layer3_bgra8888(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3colormode::Layer3Bgra8888)
    }
    #[doc = "Video 420 Mode"]
    #[inline(always)]
    pub fn layer3_video420(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3colormode::Layer3Video420)
    }
    #[doc = "Trilinear 420 Video Mode"]
    #[inline(always)]
    pub fn layer3_trilin42(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3colormode::Layer3Trilin42)
    }
}
#[doc = "Field `RSVD0` reader - This field is reserved."]
pub type Rsvd0R = crate::FieldReader;
#[doc = "Field `RSVD0` writer - This field is reserved."]
pub type Rsvd0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Source blending function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Layer3sblend {
    #[doc = "0: layer 3 black blend register. blend black"]
    Layer3sblackblend = 0,
    #[doc = "1: blend white"]
    Layer3swhiteblend = 1,
    #[doc = "2: blend alpha source"]
    Layer3salbhasblend = 2,
    #[doc = "3: blend alpha global"]
    Layer3salphagblend = 3,
    #[doc = "4: blend alpha source and alpha global"]
    Layer3salphasgblend = 4,
    #[doc = "5: blend inverted source"]
    Layer3SinvertBlendSrc = 5,
    #[doc = "6: blend inverted global"]
    Layer3sinvertglobalblend = 6,
    #[doc = "7: blend inverted source and inverted global"]
    Layer3sinvertsgblend = 7,
    #[doc = "10: blend alpha destination"]
    Layer3salphablend = 10,
    #[doc = "13: blend inverted destination"]
    Layer3SinvertBlendDst = 13,
}
impl From<Layer3sblend> for u8 {
    #[inline(always)]
    fn from(variant: Layer3sblend) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Layer3sblend {
    type Ux = u8;
}
impl crate::IsEnum for Layer3sblend {}
#[doc = "Field `LAYER3SBLEND` reader - Source blending function"]
pub type Layer3sblendR = crate::FieldReader<Layer3sblend>;
impl Layer3sblendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Layer3sblend> {
        match self.bits {
            0 => Some(Layer3sblend::Layer3sblackblend),
            1 => Some(Layer3sblend::Layer3swhiteblend),
            2 => Some(Layer3sblend::Layer3salbhasblend),
            3 => Some(Layer3sblend::Layer3salphagblend),
            4 => Some(Layer3sblend::Layer3salphasgblend),
            5 => Some(Layer3sblend::Layer3SinvertBlendSrc),
            6 => Some(Layer3sblend::Layer3sinvertglobalblend),
            7 => Some(Layer3sblend::Layer3sinvertsgblend),
            10 => Some(Layer3sblend::Layer3salphablend),
            13 => Some(Layer3sblend::Layer3SinvertBlendDst),
            _ => None,
        }
    }
    #[doc = "layer 3 black blend register. blend black"]
    #[inline(always)]
    pub fn is_layer3sblackblend(&self) -> bool {
        *self == Layer3sblend::Layer3sblackblend
    }
    #[doc = "blend white"]
    #[inline(always)]
    pub fn is_layer3swhiteblend(&self) -> bool {
        *self == Layer3sblend::Layer3swhiteblend
    }
    #[doc = "blend alpha source"]
    #[inline(always)]
    pub fn is_layer3salbhasblend(&self) -> bool {
        *self == Layer3sblend::Layer3salbhasblend
    }
    #[doc = "blend alpha global"]
    #[inline(always)]
    pub fn is_layer3salphagblend(&self) -> bool {
        *self == Layer3sblend::Layer3salphagblend
    }
    #[doc = "blend alpha source and alpha global"]
    #[inline(always)]
    pub fn is_layer3salphasgblend(&self) -> bool {
        *self == Layer3sblend::Layer3salphasgblend
    }
    #[doc = "blend inverted source"]
    #[inline(always)]
    pub fn is_layer3_sinvert_blend_src(&self) -> bool {
        *self == Layer3sblend::Layer3SinvertBlendSrc
    }
    #[doc = "blend inverted global"]
    #[inline(always)]
    pub fn is_layer3sinvertglobalblend(&self) -> bool {
        *self == Layer3sblend::Layer3sinvertglobalblend
    }
    #[doc = "blend inverted source and inverted global"]
    #[inline(always)]
    pub fn is_layer3sinvertsgblend(&self) -> bool {
        *self == Layer3sblend::Layer3sinvertsgblend
    }
    #[doc = "blend alpha destination"]
    #[inline(always)]
    pub fn is_layer3salphablend(&self) -> bool {
        *self == Layer3sblend::Layer3salphablend
    }
    #[doc = "blend inverted destination"]
    #[inline(always)]
    pub fn is_layer3_sinvert_blend_dst(&self) -> bool {
        *self == Layer3sblend::Layer3SinvertBlendDst
    }
}
#[doc = "Field `LAYER3SBLEND` writer - Source blending function"]
pub type Layer3sblendW<'a, REG> = crate::FieldWriter<'a, REG, 4, Layer3sblend>;
impl<'a, REG> Layer3sblendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "layer 3 black blend register. blend black"]
    #[inline(always)]
    pub fn layer3sblackblend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3sblend::Layer3sblackblend)
    }
    #[doc = "blend white"]
    #[inline(always)]
    pub fn layer3swhiteblend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3sblend::Layer3swhiteblend)
    }
    #[doc = "blend alpha source"]
    #[inline(always)]
    pub fn layer3salbhasblend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3sblend::Layer3salbhasblend)
    }
    #[doc = "blend alpha global"]
    #[inline(always)]
    pub fn layer3salphagblend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3sblend::Layer3salphagblend)
    }
    #[doc = "blend alpha source and alpha global"]
    #[inline(always)]
    pub fn layer3salphasgblend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3sblend::Layer3salphasgblend)
    }
    #[doc = "blend inverted source"]
    #[inline(always)]
    pub fn layer3_sinvert_blend_src(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3sblend::Layer3SinvertBlendSrc)
    }
    #[doc = "blend inverted global"]
    #[inline(always)]
    pub fn layer3sinvertglobalblend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3sblend::Layer3sinvertglobalblend)
    }
    #[doc = "blend inverted source and inverted global"]
    #[inline(always)]
    pub fn layer3sinvertsgblend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3sblend::Layer3sinvertsgblend)
    }
    #[doc = "blend alpha destination"]
    #[inline(always)]
    pub fn layer3salphablend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3sblend::Layer3salphablend)
    }
    #[doc = "blend inverted destination"]
    #[inline(always)]
    pub fn layer3_sinvert_blend_dst(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3sblend::Layer3SinvertBlendDst)
    }
}
#[doc = "Destination blending function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Layer3dblend {
    #[doc = "0: blend black"]
    Layer3DblackBlend = 0,
    #[doc = "1: blend white"]
    Layer3DwhiteBlend = 1,
    #[doc = "2: blend alpha source"]
    Layer3DalbhasBlend = 2,
    #[doc = "3: blend alpha global"]
    Layer3DalphagBlend = 3,
    #[doc = "4: blend alpha source and alpha global"]
    Layer3DalphasgBlend = 4,
    #[doc = "5: blend inverted source"]
    Layer3DinvertBlendSrc = 5,
    #[doc = "6: blend inverted global"]
    Layer3DinvertGlobalBlend = 6,
    #[doc = "7: blend inverted source and inverted global"]
    Layer3DinvertsgBlend = 7,
    #[doc = "10: blend alpha destination"]
    Layer3DalphaBlend = 10,
    #[doc = "13: blend inverted destination"]
    Layer3DinvertBlendDst = 13,
}
impl From<Layer3dblend> for u8 {
    #[inline(always)]
    fn from(variant: Layer3dblend) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Layer3dblend {
    type Ux = u8;
}
impl crate::IsEnum for Layer3dblend {}
#[doc = "Field `LAYER3DBLEND` reader - Destination blending function"]
pub type Layer3dblendR = crate::FieldReader<Layer3dblend>;
impl Layer3dblendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Layer3dblend> {
        match self.bits {
            0 => Some(Layer3dblend::Layer3DblackBlend),
            1 => Some(Layer3dblend::Layer3DwhiteBlend),
            2 => Some(Layer3dblend::Layer3DalbhasBlend),
            3 => Some(Layer3dblend::Layer3DalphagBlend),
            4 => Some(Layer3dblend::Layer3DalphasgBlend),
            5 => Some(Layer3dblend::Layer3DinvertBlendSrc),
            6 => Some(Layer3dblend::Layer3DinvertGlobalBlend),
            7 => Some(Layer3dblend::Layer3DinvertsgBlend),
            10 => Some(Layer3dblend::Layer3DalphaBlend),
            13 => Some(Layer3dblend::Layer3DinvertBlendDst),
            _ => None,
        }
    }
    #[doc = "blend black"]
    #[inline(always)]
    pub fn is_layer3_dblack_blend(&self) -> bool {
        *self == Layer3dblend::Layer3DblackBlend
    }
    #[doc = "blend white"]
    #[inline(always)]
    pub fn is_layer3_dwhite_blend(&self) -> bool {
        *self == Layer3dblend::Layer3DwhiteBlend
    }
    #[doc = "blend alpha source"]
    #[inline(always)]
    pub fn is_layer3_dalbhas_blend(&self) -> bool {
        *self == Layer3dblend::Layer3DalbhasBlend
    }
    #[doc = "blend alpha global"]
    #[inline(always)]
    pub fn is_layer3_dalphag_blend(&self) -> bool {
        *self == Layer3dblend::Layer3DalphagBlend
    }
    #[doc = "blend alpha source and alpha global"]
    #[inline(always)]
    pub fn is_layer3_dalphasg_blend(&self) -> bool {
        *self == Layer3dblend::Layer3DalphasgBlend
    }
    #[doc = "blend inverted source"]
    #[inline(always)]
    pub fn is_layer3_dinvert_blend_src(&self) -> bool {
        *self == Layer3dblend::Layer3DinvertBlendSrc
    }
    #[doc = "blend inverted global"]
    #[inline(always)]
    pub fn is_layer3_dinvert_global_blend(&self) -> bool {
        *self == Layer3dblend::Layer3DinvertGlobalBlend
    }
    #[doc = "blend inverted source and inverted global"]
    #[inline(always)]
    pub fn is_layer3_dinvertsg_blend(&self) -> bool {
        *self == Layer3dblend::Layer3DinvertsgBlend
    }
    #[doc = "blend alpha destination"]
    #[inline(always)]
    pub fn is_layer3_dalpha_blend(&self) -> bool {
        *self == Layer3dblend::Layer3DalphaBlend
    }
    #[doc = "blend inverted destination"]
    #[inline(always)]
    pub fn is_layer3_dinvert_blend_dst(&self) -> bool {
        *self == Layer3dblend::Layer3DinvertBlendDst
    }
}
#[doc = "Field `LAYER3DBLEND` writer - Destination blending function"]
pub type Layer3dblendW<'a, REG> = crate::FieldWriter<'a, REG, 4, Layer3dblend>;
impl<'a, REG> Layer3dblendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "blend black"]
    #[inline(always)]
    pub fn layer3_dblack_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3dblend::Layer3DblackBlend)
    }
    #[doc = "blend white"]
    #[inline(always)]
    pub fn layer3_dwhite_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3dblend::Layer3DwhiteBlend)
    }
    #[doc = "blend alpha source"]
    #[inline(always)]
    pub fn layer3_dalbhas_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3dblend::Layer3DalbhasBlend)
    }
    #[doc = "blend alpha global"]
    #[inline(always)]
    pub fn layer3_dalphag_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3dblend::Layer3DalphagBlend)
    }
    #[doc = "blend alpha source and alpha global"]
    #[inline(always)]
    pub fn layer3_dalphasg_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3dblend::Layer3DalphasgBlend)
    }
    #[doc = "blend inverted source"]
    #[inline(always)]
    pub fn layer3_dinvert_blend_src(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3dblend::Layer3DinvertBlendSrc)
    }
    #[doc = "blend inverted global"]
    #[inline(always)]
    pub fn layer3_dinvert_global_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3dblend::Layer3DinvertGlobalBlend)
    }
    #[doc = "blend inverted source and inverted global"]
    #[inline(always)]
    pub fn layer3_dinvertsg_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3dblend::Layer3DinvertsgBlend)
    }
    #[doc = "blend alpha destination"]
    #[inline(always)]
    pub fn layer3_dalpha_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3dblend::Layer3DalphaBlend)
    }
    #[doc = "blend inverted destination"]
    #[inline(always)]
    pub fn layer3_dinvert_blend_dst(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3dblend::Layer3DinvertBlendDst)
    }
}
#[doc = "Field `LAYER3ALPHA` reader - Alpha layer global value (0x00-0xFF range)"]
pub type Layer3alphaR = crate::FieldReader;
#[doc = "Field `LAYER3ALPHA` writer - Alpha layer global value (0x00-0xFF range)"]
pub type Layer3alphaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RSVD1` reader - This field is reserved."]
pub type Rsvd1R = crate::FieldReader;
#[doc = "Field `RSVD1` writer - This field is reserved."]
pub type Rsvd1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LAYER3GAMMA` reader - When set to 1, Gamma Look Up Table is enabled"]
pub type Layer3gammaR = crate::BitReader;
#[doc = "Field `LAYER3GAMMA` writer - When set to 1, Gamma Look Up Table is enabled"]
pub type Layer3gammaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAYER3HLOCK` reader - When set to 1, HLOCK signal on AHB DMAs is asserted"]
pub type Layer3hlockR = crate::BitReader;
#[doc = "Field `LAYER3HLOCK` writer - When set to 1, HLOCK signal on AHB DMAs is asserted"]
pub type Layer3hlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAYER3PREMULT` reader - When set to 1, premultiply image alpha is enabled"]
pub type Layer3premultR = crate::BitReader;
#[doc = "Field `LAYER3PREMULT` writer - When set to 1, premultiply image alpha is enabled"]
pub type Layer3premultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAYER3BFILTER` reader - When set to 1, bilinear filtering is enabled"]
pub type Layer3bfilterR = crate::BitReader;
#[doc = "Field `LAYER3BFILTER` writer - When set to 1, bilinear filtering is enabled"]
pub type Layer3bfilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAYER3FORCE` reader - When set to 1, force alpha with global alpha is enabled"]
pub type Layer3forceR = crate::BitReader;
#[doc = "Field `LAYER3FORCE` writer - When set to 1, force alpha with global alpha is enabled"]
pub type Layer3forceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAYER3EN` reader - When set to 1, layer n is enabled"]
pub type Layer3enR = crate::BitReader;
#[doc = "Field `LAYER3EN` writer - When set to 1, layer n is enabled"]
pub type Layer3enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Color mode"]
    #[inline(always)]
    pub fn layer3colormode(&self) -> Layer3colormodeR {
        Layer3colormodeR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd0(&self) -> Rsvd0R {
        Rsvd0R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Source blending function"]
    #[inline(always)]
    pub fn layer3sblend(&self) -> Layer3sblendR {
        Layer3sblendR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Destination blending function"]
    #[inline(always)]
    pub fn layer3dblend(&self) -> Layer3dblendR {
        Layer3dblendR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Alpha layer global value (0x00-0xFF range)"]
    #[inline(always)]
    pub fn layer3alpha(&self) -> Layer3alphaR {
        Layer3alphaR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd1(&self) -> Rsvd1R {
        Rsvd1R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - When set to 1, Gamma Look Up Table is enabled"]
    #[inline(always)]
    pub fn layer3gamma(&self) -> Layer3gammaR {
        Layer3gammaR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - When set to 1, HLOCK signal on AHB DMAs is asserted"]
    #[inline(always)]
    pub fn layer3hlock(&self) -> Layer3hlockR {
        Layer3hlockR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - When set to 1, premultiply image alpha is enabled"]
    #[inline(always)]
    pub fn layer3premult(&self) -> Layer3premultR {
        Layer3premultR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - When set to 1, bilinear filtering is enabled"]
    #[inline(always)]
    pub fn layer3bfilter(&self) -> Layer3bfilterR {
        Layer3bfilterR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When set to 1, force alpha with global alpha is enabled"]
    #[inline(always)]
    pub fn layer3force(&self) -> Layer3forceR {
        Layer3forceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When set to 1, layer n is enabled"]
    #[inline(always)]
    pub fn layer3en(&self) -> Layer3enR {
        Layer3enR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Color mode"]
    #[inline(always)]
    #[must_use]
    pub fn layer3colormode(&mut self) -> Layer3colormodeW<Layer3modeSpec> {
        Layer3colormodeW::new(self, 0)
    }
    #[doc = "Bits 5:7 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd0(&mut self) -> Rsvd0W<Layer3modeSpec> {
        Rsvd0W::new(self, 5)
    }
    #[doc = "Bits 8:11 - Source blending function"]
    #[inline(always)]
    #[must_use]
    pub fn layer3sblend(&mut self) -> Layer3sblendW<Layer3modeSpec> {
        Layer3sblendW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Destination blending function"]
    #[inline(always)]
    #[must_use]
    pub fn layer3dblend(&mut self) -> Layer3dblendW<Layer3modeSpec> {
        Layer3dblendW::new(self, 12)
    }
    #[doc = "Bits 16:23 - Alpha layer global value (0x00-0xFF range)"]
    #[inline(always)]
    #[must_use]
    pub fn layer3alpha(&mut self) -> Layer3alphaW<Layer3modeSpec> {
        Layer3alphaW::new(self, 16)
    }
    #[doc = "Bits 24:25 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd1(&mut self) -> Rsvd1W<Layer3modeSpec> {
        Rsvd1W::new(self, 24)
    }
    #[doc = "Bit 26 - When set to 1, Gamma Look Up Table is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn layer3gamma(&mut self) -> Layer3gammaW<Layer3modeSpec> {
        Layer3gammaW::new(self, 26)
    }
    #[doc = "Bit 27 - When set to 1, HLOCK signal on AHB DMAs is asserted"]
    #[inline(always)]
    #[must_use]
    pub fn layer3hlock(&mut self) -> Layer3hlockW<Layer3modeSpec> {
        Layer3hlockW::new(self, 27)
    }
    #[doc = "Bit 28 - When set to 1, premultiply image alpha is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn layer3premult(&mut self) -> Layer3premultW<Layer3modeSpec> {
        Layer3premultW::new(self, 28)
    }
    #[doc = "Bit 29 - When set to 1, bilinear filtering is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn layer3bfilter(&mut self) -> Layer3bfilterW<Layer3modeSpec> {
        Layer3bfilterW::new(self, 29)
    }
    #[doc = "Bit 30 - When set to 1, force alpha with global alpha is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn layer3force(&mut self) -> Layer3forceW<Layer3modeSpec> {
        Layer3forceW::new(self, 30)
    }
    #[doc = "Bit 31 - When set to 1, layer n is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn layer3en(&mut self) -> Layer3enW<Layer3modeSpec> {
        Layer3enW::new(self, 31)
    }
}
#[doc = "Activate and set-up layer 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer3mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer3mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer3modeSpec;
impl crate::RegisterSpec for Layer3modeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer3mode::R`](R) reader structure"]
impl crate::Readable for Layer3modeSpec {}
#[doc = "`write(|w| ..)` method takes [`layer3mode::W`](W) writer structure"]
impl crate::Writable for Layer3modeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER3MODE to value 0"]
impl crate::Resettable for Layer3modeSpec {
    const RESET_VALUE: u32 = 0;
}
