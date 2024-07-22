#[doc = "Register `LAYER2MODE` reader"]
pub type R = crate::R<Layer2modeSpec>;
#[doc = "Register `LAYER2MODE` writer"]
pub type W = crate::W<Layer2modeSpec>;
#[doc = "Color mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Layer2colormode {
    #[doc = "0: 8-bit color palette look-up table (LUT8)"]
    Layer2Lutble = 0,
    #[doc = "1: 16-bit RGBX5551 color format"]
    Layer2Rgbx5551 = 1,
    #[doc = "2: 32-bit RGBX8888 color format"]
    Layer2Rgbx8888 = 2,
    #[doc = "4: 8-bit RGB332 color format"]
    Layer2Rgb332 = 4,
    #[doc = "5: 16-bit RGB565 color format"]
    Layer2Rgb565 = 5,
    #[doc = "6: 32-bit XRGB8888 color format"]
    Layer2Xrgb8888 = 6,
    #[doc = "7: L8 Grayscale/Palette format"]
    Layer2L8 = 7,
    #[doc = "8: L1 Grayscale/Palette format"]
    Layer2L1 = 8,
    #[doc = "9: L4 Grayscale/Palette format"]
    Layer2L4 = 9,
    #[doc = "10: YUYV color format"]
    Layer2Yuyv = 10,
    #[doc = "11: 24-bit RGB color format"]
    Layer2Rgb = 11,
    #[doc = "12: YUY2 color format"]
    Layer2Yuy2 = 12,
    #[doc = "13: 32-bit ABGR8888 color format"]
    Layer2Abgr8888 = 13,
    #[doc = "14: 32-bit BGRA8888 color format"]
    Layer2Bgra8888 = 14,
    #[doc = "16: Video 420 Mode"]
    Layer2Video420 = 16,
    #[doc = "17: Trilinear 420 Video Mode"]
    Layer2Trilin420 = 17,
}
impl From<Layer2colormode> for u8 {
    #[inline(always)]
    fn from(variant: Layer2colormode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Layer2colormode {
    type Ux = u8;
}
impl crate::IsEnum for Layer2colormode {}
#[doc = "Field `LAYER2COLORMODE` reader - Color mode"]
pub type Layer2colormodeR = crate::FieldReader<Layer2colormode>;
impl Layer2colormodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Layer2colormode> {
        match self.bits {
            0 => Some(Layer2colormode::Layer2Lutble),
            1 => Some(Layer2colormode::Layer2Rgbx5551),
            2 => Some(Layer2colormode::Layer2Rgbx8888),
            4 => Some(Layer2colormode::Layer2Rgb332),
            5 => Some(Layer2colormode::Layer2Rgb565),
            6 => Some(Layer2colormode::Layer2Xrgb8888),
            7 => Some(Layer2colormode::Layer2L8),
            8 => Some(Layer2colormode::Layer2L1),
            9 => Some(Layer2colormode::Layer2L4),
            10 => Some(Layer2colormode::Layer2Yuyv),
            11 => Some(Layer2colormode::Layer2Rgb),
            12 => Some(Layer2colormode::Layer2Yuy2),
            13 => Some(Layer2colormode::Layer2Abgr8888),
            14 => Some(Layer2colormode::Layer2Bgra8888),
            16 => Some(Layer2colormode::Layer2Video420),
            17 => Some(Layer2colormode::Layer2Trilin420),
            _ => None,
        }
    }
    #[doc = "8-bit color palette look-up table (LUT8)"]
    #[inline(always)]
    pub fn is_layer2_lutble(&self) -> bool {
        *self == Layer2colormode::Layer2Lutble
    }
    #[doc = "16-bit RGBX5551 color format"]
    #[inline(always)]
    pub fn is_layer2_rgbx5551(&self) -> bool {
        *self == Layer2colormode::Layer2Rgbx5551
    }
    #[doc = "32-bit RGBX8888 color format"]
    #[inline(always)]
    pub fn is_layer2_rgbx8888(&self) -> bool {
        *self == Layer2colormode::Layer2Rgbx8888
    }
    #[doc = "8-bit RGB332 color format"]
    #[inline(always)]
    pub fn is_layer2_rgb332(&self) -> bool {
        *self == Layer2colormode::Layer2Rgb332
    }
    #[doc = "16-bit RGB565 color format"]
    #[inline(always)]
    pub fn is_layer2_rgb565(&self) -> bool {
        *self == Layer2colormode::Layer2Rgb565
    }
    #[doc = "32-bit XRGB8888 color format"]
    #[inline(always)]
    pub fn is_layer2_xrgb8888(&self) -> bool {
        *self == Layer2colormode::Layer2Xrgb8888
    }
    #[doc = "L8 Grayscale/Palette format"]
    #[inline(always)]
    pub fn is_layer2_l8(&self) -> bool {
        *self == Layer2colormode::Layer2L8
    }
    #[doc = "L1 Grayscale/Palette format"]
    #[inline(always)]
    pub fn is_layer2_l1(&self) -> bool {
        *self == Layer2colormode::Layer2L1
    }
    #[doc = "L4 Grayscale/Palette format"]
    #[inline(always)]
    pub fn is_layer2_l4(&self) -> bool {
        *self == Layer2colormode::Layer2L4
    }
    #[doc = "YUYV color format"]
    #[inline(always)]
    pub fn is_layer2_yuyv(&self) -> bool {
        *self == Layer2colormode::Layer2Yuyv
    }
    #[doc = "24-bit RGB color format"]
    #[inline(always)]
    pub fn is_layer2_rgb(&self) -> bool {
        *self == Layer2colormode::Layer2Rgb
    }
    #[doc = "YUY2 color format"]
    #[inline(always)]
    pub fn is_layer2_yuy2(&self) -> bool {
        *self == Layer2colormode::Layer2Yuy2
    }
    #[doc = "32-bit ABGR8888 color format"]
    #[inline(always)]
    pub fn is_layer2_abgr8888(&self) -> bool {
        *self == Layer2colormode::Layer2Abgr8888
    }
    #[doc = "32-bit BGRA8888 color format"]
    #[inline(always)]
    pub fn is_layer2_bgra8888(&self) -> bool {
        *self == Layer2colormode::Layer2Bgra8888
    }
    #[doc = "Video 420 Mode"]
    #[inline(always)]
    pub fn is_layer2_video420(&self) -> bool {
        *self == Layer2colormode::Layer2Video420
    }
    #[doc = "Trilinear 420 Video Mode"]
    #[inline(always)]
    pub fn is_layer2_trilin420(&self) -> bool {
        *self == Layer2colormode::Layer2Trilin420
    }
}
#[doc = "Field `LAYER2COLORMODE` writer - Color mode"]
pub type Layer2colormodeW<'a, REG> = crate::FieldWriter<'a, REG, 5, Layer2colormode>;
impl<'a, REG> Layer2colormodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-bit color palette look-up table (LUT8)"]
    #[inline(always)]
    pub fn layer2_lutble(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2colormode::Layer2Lutble)
    }
    #[doc = "16-bit RGBX5551 color format"]
    #[inline(always)]
    pub fn layer2_rgbx5551(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2colormode::Layer2Rgbx5551)
    }
    #[doc = "32-bit RGBX8888 color format"]
    #[inline(always)]
    pub fn layer2_rgbx8888(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2colormode::Layer2Rgbx8888)
    }
    #[doc = "8-bit RGB332 color format"]
    #[inline(always)]
    pub fn layer2_rgb332(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2colormode::Layer2Rgb332)
    }
    #[doc = "16-bit RGB565 color format"]
    #[inline(always)]
    pub fn layer2_rgb565(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2colormode::Layer2Rgb565)
    }
    #[doc = "32-bit XRGB8888 color format"]
    #[inline(always)]
    pub fn layer2_xrgb8888(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2colormode::Layer2Xrgb8888)
    }
    #[doc = "L8 Grayscale/Palette format"]
    #[inline(always)]
    pub fn layer2_l8(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2colormode::Layer2L8)
    }
    #[doc = "L1 Grayscale/Palette format"]
    #[inline(always)]
    pub fn layer2_l1(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2colormode::Layer2L1)
    }
    #[doc = "L4 Grayscale/Palette format"]
    #[inline(always)]
    pub fn layer2_l4(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2colormode::Layer2L4)
    }
    #[doc = "YUYV color format"]
    #[inline(always)]
    pub fn layer2_yuyv(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2colormode::Layer2Yuyv)
    }
    #[doc = "24-bit RGB color format"]
    #[inline(always)]
    pub fn layer2_rgb(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2colormode::Layer2Rgb)
    }
    #[doc = "YUY2 color format"]
    #[inline(always)]
    pub fn layer2_yuy2(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2colormode::Layer2Yuy2)
    }
    #[doc = "32-bit ABGR8888 color format"]
    #[inline(always)]
    pub fn layer2_abgr8888(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2colormode::Layer2Abgr8888)
    }
    #[doc = "32-bit BGRA8888 color format"]
    #[inline(always)]
    pub fn layer2_bgra8888(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2colormode::Layer2Bgra8888)
    }
    #[doc = "Video 420 Mode"]
    #[inline(always)]
    pub fn layer2_video420(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2colormode::Layer2Video420)
    }
    #[doc = "Trilinear 420 Video Mode"]
    #[inline(always)]
    pub fn layer2_trilin420(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2colormode::Layer2Trilin420)
    }
}
#[doc = "Field `RSVD0` reader - This field is reserved."]
pub type Rsvd0R = crate::FieldReader;
#[doc = "Field `RSVD0` writer - This field is reserved."]
pub type Rsvd0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Source blending function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Layer2sblend {
    #[doc = "0: blend black"]
    Layer2SblackBlend = 0,
    #[doc = "1: blend white"]
    Layer2SwhiteBlend = 1,
    #[doc = "2: blend alpha source"]
    Layer2SalbhasBlend = 2,
    #[doc = "3: blend alpha global"]
    Layer2SalphagBlend = 3,
    #[doc = "4: blend alpha source and alpha global"]
    Layer2SalphasgBlend = 4,
    #[doc = "5: blend inverted source"]
    Layer2SinvertBlendSrc = 5,
    #[doc = "6: blend inverted global"]
    Layer2SinvertGlobalBlend = 6,
    #[doc = "7: blend inverted source and inverted global"]
    Layer2SinvertsgBlend = 7,
    #[doc = "10: blend alpha destination"]
    Layer2SalphaBlend = 10,
    #[doc = "13: blend inverted destination"]
    Layer2SinvertBlendDst = 13,
}
impl From<Layer2sblend> for u8 {
    #[inline(always)]
    fn from(variant: Layer2sblend) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Layer2sblend {
    type Ux = u8;
}
impl crate::IsEnum for Layer2sblend {}
#[doc = "Field `LAYER2SBLEND` reader - Source blending function"]
pub type Layer2sblendR = crate::FieldReader<Layer2sblend>;
impl Layer2sblendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Layer2sblend> {
        match self.bits {
            0 => Some(Layer2sblend::Layer2SblackBlend),
            1 => Some(Layer2sblend::Layer2SwhiteBlend),
            2 => Some(Layer2sblend::Layer2SalbhasBlend),
            3 => Some(Layer2sblend::Layer2SalphagBlend),
            4 => Some(Layer2sblend::Layer2SalphasgBlend),
            5 => Some(Layer2sblend::Layer2SinvertBlendSrc),
            6 => Some(Layer2sblend::Layer2SinvertGlobalBlend),
            7 => Some(Layer2sblend::Layer2SinvertsgBlend),
            10 => Some(Layer2sblend::Layer2SalphaBlend),
            13 => Some(Layer2sblend::Layer2SinvertBlendDst),
            _ => None,
        }
    }
    #[doc = "blend black"]
    #[inline(always)]
    pub fn is_layer2_sblack_blend(&self) -> bool {
        *self == Layer2sblend::Layer2SblackBlend
    }
    #[doc = "blend white"]
    #[inline(always)]
    pub fn is_layer2_swhite_blend(&self) -> bool {
        *self == Layer2sblend::Layer2SwhiteBlend
    }
    #[doc = "blend alpha source"]
    #[inline(always)]
    pub fn is_layer2_salbhas_blend(&self) -> bool {
        *self == Layer2sblend::Layer2SalbhasBlend
    }
    #[doc = "blend alpha global"]
    #[inline(always)]
    pub fn is_layer2_salphag_blend(&self) -> bool {
        *self == Layer2sblend::Layer2SalphagBlend
    }
    #[doc = "blend alpha source and alpha global"]
    #[inline(always)]
    pub fn is_layer2_salphasg_blend(&self) -> bool {
        *self == Layer2sblend::Layer2SalphasgBlend
    }
    #[doc = "blend inverted source"]
    #[inline(always)]
    pub fn is_layer2_sinvert_blend_src(&self) -> bool {
        *self == Layer2sblend::Layer2SinvertBlendSrc
    }
    #[doc = "blend inverted global"]
    #[inline(always)]
    pub fn is_layer2_sinvert_global_blend(&self) -> bool {
        *self == Layer2sblend::Layer2SinvertGlobalBlend
    }
    #[doc = "blend inverted source and inverted global"]
    #[inline(always)]
    pub fn is_layer2_sinvertsg_blend(&self) -> bool {
        *self == Layer2sblend::Layer2SinvertsgBlend
    }
    #[doc = "blend alpha destination"]
    #[inline(always)]
    pub fn is_layer2_salpha_blend(&self) -> bool {
        *self == Layer2sblend::Layer2SalphaBlend
    }
    #[doc = "blend inverted destination"]
    #[inline(always)]
    pub fn is_layer2_sinvert_blend_dst(&self) -> bool {
        *self == Layer2sblend::Layer2SinvertBlendDst
    }
}
#[doc = "Field `LAYER2SBLEND` writer - Source blending function"]
pub type Layer2sblendW<'a, REG> = crate::FieldWriter<'a, REG, 4, Layer2sblend>;
impl<'a, REG> Layer2sblendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "blend black"]
    #[inline(always)]
    pub fn layer2_sblack_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2sblend::Layer2SblackBlend)
    }
    #[doc = "blend white"]
    #[inline(always)]
    pub fn layer2_swhite_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2sblend::Layer2SwhiteBlend)
    }
    #[doc = "blend alpha source"]
    #[inline(always)]
    pub fn layer2_salbhas_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2sblend::Layer2SalbhasBlend)
    }
    #[doc = "blend alpha global"]
    #[inline(always)]
    pub fn layer2_salphag_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2sblend::Layer2SalphagBlend)
    }
    #[doc = "blend alpha source and alpha global"]
    #[inline(always)]
    pub fn layer2_salphasg_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2sblend::Layer2SalphasgBlend)
    }
    #[doc = "blend inverted source"]
    #[inline(always)]
    pub fn layer2_sinvert_blend_src(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2sblend::Layer2SinvertBlendSrc)
    }
    #[doc = "blend inverted global"]
    #[inline(always)]
    pub fn layer2_sinvert_global_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2sblend::Layer2SinvertGlobalBlend)
    }
    #[doc = "blend inverted source and inverted global"]
    #[inline(always)]
    pub fn layer2_sinvertsg_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2sblend::Layer2SinvertsgBlend)
    }
    #[doc = "blend alpha destination"]
    #[inline(always)]
    pub fn layer2_salpha_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2sblend::Layer2SalphaBlend)
    }
    #[doc = "blend inverted destination"]
    #[inline(always)]
    pub fn layer2_sinvert_blend_dst(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2sblend::Layer2SinvertBlendDst)
    }
}
#[doc = "Destination blending function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Layer2dblend {
    #[doc = "0: blend black"]
    Layer2DblackBlend = 0,
    #[doc = "1: blend white"]
    Layer2DwhiteBlend = 1,
    #[doc = "2: blend alpha source"]
    Layer2DalbhasBlend = 2,
    #[doc = "3: blend alpha global"]
    Layer2DalphagBlend = 3,
    #[doc = "4: blend alpha source and alpha global"]
    Layer2DalphasgBlend = 4,
    #[doc = "5: blend inverted source"]
    Layer2DinvertBlendSrc = 5,
    #[doc = "6: blend inverted global"]
    Layer2DinvertGlobalBlend = 6,
    #[doc = "7: blend inverted source and inverted global"]
    Layer2DinvertsgBlend = 7,
    #[doc = "10: blend alpha destination"]
    Layer2DalphaBlend = 10,
    #[doc = "13: blend inverted destination"]
    Layer2DinvertBlendDst = 13,
}
impl From<Layer2dblend> for u8 {
    #[inline(always)]
    fn from(variant: Layer2dblend) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Layer2dblend {
    type Ux = u8;
}
impl crate::IsEnum for Layer2dblend {}
#[doc = "Field `LAYER2DBLEND` reader - Destination blending function"]
pub type Layer2dblendR = crate::FieldReader<Layer2dblend>;
impl Layer2dblendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Layer2dblend> {
        match self.bits {
            0 => Some(Layer2dblend::Layer2DblackBlend),
            1 => Some(Layer2dblend::Layer2DwhiteBlend),
            2 => Some(Layer2dblend::Layer2DalbhasBlend),
            3 => Some(Layer2dblend::Layer2DalphagBlend),
            4 => Some(Layer2dblend::Layer2DalphasgBlend),
            5 => Some(Layer2dblend::Layer2DinvertBlendSrc),
            6 => Some(Layer2dblend::Layer2DinvertGlobalBlend),
            7 => Some(Layer2dblend::Layer2DinvertsgBlend),
            10 => Some(Layer2dblend::Layer2DalphaBlend),
            13 => Some(Layer2dblend::Layer2DinvertBlendDst),
            _ => None,
        }
    }
    #[doc = "blend black"]
    #[inline(always)]
    pub fn is_layer2_dblack_blend(&self) -> bool {
        *self == Layer2dblend::Layer2DblackBlend
    }
    #[doc = "blend white"]
    #[inline(always)]
    pub fn is_layer2_dwhite_blend(&self) -> bool {
        *self == Layer2dblend::Layer2DwhiteBlend
    }
    #[doc = "blend alpha source"]
    #[inline(always)]
    pub fn is_layer2_dalbhas_blend(&self) -> bool {
        *self == Layer2dblend::Layer2DalbhasBlend
    }
    #[doc = "blend alpha global"]
    #[inline(always)]
    pub fn is_layer2_dalphag_blend(&self) -> bool {
        *self == Layer2dblend::Layer2DalphagBlend
    }
    #[doc = "blend alpha source and alpha global"]
    #[inline(always)]
    pub fn is_layer2_dalphasg_blend(&self) -> bool {
        *self == Layer2dblend::Layer2DalphasgBlend
    }
    #[doc = "blend inverted source"]
    #[inline(always)]
    pub fn is_layer2_dinvert_blend_src(&self) -> bool {
        *self == Layer2dblend::Layer2DinvertBlendSrc
    }
    #[doc = "blend inverted global"]
    #[inline(always)]
    pub fn is_layer2_dinvert_global_blend(&self) -> bool {
        *self == Layer2dblend::Layer2DinvertGlobalBlend
    }
    #[doc = "blend inverted source and inverted global"]
    #[inline(always)]
    pub fn is_layer2_dinvertsg_blend(&self) -> bool {
        *self == Layer2dblend::Layer2DinvertsgBlend
    }
    #[doc = "blend alpha destination"]
    #[inline(always)]
    pub fn is_layer2_dalpha_blend(&self) -> bool {
        *self == Layer2dblend::Layer2DalphaBlend
    }
    #[doc = "blend inverted destination"]
    #[inline(always)]
    pub fn is_layer2_dinvert_blend_dst(&self) -> bool {
        *self == Layer2dblend::Layer2DinvertBlendDst
    }
}
#[doc = "Field `LAYER2DBLEND` writer - Destination blending function"]
pub type Layer2dblendW<'a, REG> = crate::FieldWriter<'a, REG, 4, Layer2dblend>;
impl<'a, REG> Layer2dblendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "blend black"]
    #[inline(always)]
    pub fn layer2_dblack_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2dblend::Layer2DblackBlend)
    }
    #[doc = "blend white"]
    #[inline(always)]
    pub fn layer2_dwhite_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2dblend::Layer2DwhiteBlend)
    }
    #[doc = "blend alpha source"]
    #[inline(always)]
    pub fn layer2_dalbhas_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2dblend::Layer2DalbhasBlend)
    }
    #[doc = "blend alpha global"]
    #[inline(always)]
    pub fn layer2_dalphag_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2dblend::Layer2DalphagBlend)
    }
    #[doc = "blend alpha source and alpha global"]
    #[inline(always)]
    pub fn layer2_dalphasg_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2dblend::Layer2DalphasgBlend)
    }
    #[doc = "blend inverted source"]
    #[inline(always)]
    pub fn layer2_dinvert_blend_src(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2dblend::Layer2DinvertBlendSrc)
    }
    #[doc = "blend inverted global"]
    #[inline(always)]
    pub fn layer2_dinvert_global_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2dblend::Layer2DinvertGlobalBlend)
    }
    #[doc = "blend inverted source and inverted global"]
    #[inline(always)]
    pub fn layer2_dinvertsg_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2dblend::Layer2DinvertsgBlend)
    }
    #[doc = "blend alpha destination"]
    #[inline(always)]
    pub fn layer2_dalpha_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2dblend::Layer2DalphaBlend)
    }
    #[doc = "blend inverted destination"]
    #[inline(always)]
    pub fn layer2_dinvert_blend_dst(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2dblend::Layer2DinvertBlendDst)
    }
}
#[doc = "Field `LAYER2ALPHA` reader - Alpha layer global value (0x00-0xFF range)"]
pub type Layer2alphaR = crate::FieldReader;
#[doc = "Field `LAYER2ALPHA` writer - Alpha layer global value (0x00-0xFF range)"]
pub type Layer2alphaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RSVD1` reader - This field is reserved."]
pub type Rsvd1R = crate::FieldReader;
#[doc = "Field `RSVD1` writer - This field is reserved."]
pub type Rsvd1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LAYER2GAMMA` reader - When set to 1, Gamma Look Up Table is enabled"]
pub type Layer2gammaR = crate::BitReader;
#[doc = "Field `LAYER2GAMMA` writer - When set to 1, Gamma Look Up Table is enabled"]
pub type Layer2gammaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAYER2HLOCK` reader - When set to 1, HLOCK signal on AHB DMAs is asserted"]
pub type Layer2hlockR = crate::BitReader;
#[doc = "Field `LAYER2HLOCK` writer - When set to 1, HLOCK signal on AHB DMAs is asserted"]
pub type Layer2hlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAYER2PREMULT` reader - When set to 1, premultiply image alpha is enabled"]
pub type Layer2premultR = crate::BitReader;
#[doc = "Field `LAYER2PREMULT` writer - When set to 1, premultiply image alpha is enabled"]
pub type Layer2premultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAYER2BFILTER` reader - When set to 1, bilinear filtering is enabled"]
pub type Layer2bfilterR = crate::BitReader;
#[doc = "Field `LAYER2BFILTER` writer - When set to 1, bilinear filtering is enabled"]
pub type Layer2bfilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAYER2FORCE` reader - When set to 1, force alpha with global alpha is enabled"]
pub type Layer2forceR = crate::BitReader;
#[doc = "Field `LAYER2FORCE` writer - When set to 1, force alpha with global alpha is enabled"]
pub type Layer2forceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAYER2EN` reader - When set to 1, layer n is enabled"]
pub type Layer2enR = crate::BitReader;
#[doc = "Field `LAYER2EN` writer - When set to 1, layer n is enabled"]
pub type Layer2enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Color mode"]
    #[inline(always)]
    pub fn layer2colormode(&self) -> Layer2colormodeR {
        Layer2colormodeR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd0(&self) -> Rsvd0R {
        Rsvd0R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Source blending function"]
    #[inline(always)]
    pub fn layer2sblend(&self) -> Layer2sblendR {
        Layer2sblendR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Destination blending function"]
    #[inline(always)]
    pub fn layer2dblend(&self) -> Layer2dblendR {
        Layer2dblendR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Alpha layer global value (0x00-0xFF range)"]
    #[inline(always)]
    pub fn layer2alpha(&self) -> Layer2alphaR {
        Layer2alphaR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd1(&self) -> Rsvd1R {
        Rsvd1R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - When set to 1, Gamma Look Up Table is enabled"]
    #[inline(always)]
    pub fn layer2gamma(&self) -> Layer2gammaR {
        Layer2gammaR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - When set to 1, HLOCK signal on AHB DMAs is asserted"]
    #[inline(always)]
    pub fn layer2hlock(&self) -> Layer2hlockR {
        Layer2hlockR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - When set to 1, premultiply image alpha is enabled"]
    #[inline(always)]
    pub fn layer2premult(&self) -> Layer2premultR {
        Layer2premultR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - When set to 1, bilinear filtering is enabled"]
    #[inline(always)]
    pub fn layer2bfilter(&self) -> Layer2bfilterR {
        Layer2bfilterR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When set to 1, force alpha with global alpha is enabled"]
    #[inline(always)]
    pub fn layer2force(&self) -> Layer2forceR {
        Layer2forceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When set to 1, layer n is enabled"]
    #[inline(always)]
    pub fn layer2en(&self) -> Layer2enR {
        Layer2enR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Color mode"]
    #[inline(always)]
    #[must_use]
    pub fn layer2colormode(&mut self) -> Layer2colormodeW<Layer2modeSpec> {
        Layer2colormodeW::new(self, 0)
    }
    #[doc = "Bits 5:7 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd0(&mut self) -> Rsvd0W<Layer2modeSpec> {
        Rsvd0W::new(self, 5)
    }
    #[doc = "Bits 8:11 - Source blending function"]
    #[inline(always)]
    #[must_use]
    pub fn layer2sblend(&mut self) -> Layer2sblendW<Layer2modeSpec> {
        Layer2sblendW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Destination blending function"]
    #[inline(always)]
    #[must_use]
    pub fn layer2dblend(&mut self) -> Layer2dblendW<Layer2modeSpec> {
        Layer2dblendW::new(self, 12)
    }
    #[doc = "Bits 16:23 - Alpha layer global value (0x00-0xFF range)"]
    #[inline(always)]
    #[must_use]
    pub fn layer2alpha(&mut self) -> Layer2alphaW<Layer2modeSpec> {
        Layer2alphaW::new(self, 16)
    }
    #[doc = "Bits 24:25 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd1(&mut self) -> Rsvd1W<Layer2modeSpec> {
        Rsvd1W::new(self, 24)
    }
    #[doc = "Bit 26 - When set to 1, Gamma Look Up Table is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn layer2gamma(&mut self) -> Layer2gammaW<Layer2modeSpec> {
        Layer2gammaW::new(self, 26)
    }
    #[doc = "Bit 27 - When set to 1, HLOCK signal on AHB DMAs is asserted"]
    #[inline(always)]
    #[must_use]
    pub fn layer2hlock(&mut self) -> Layer2hlockW<Layer2modeSpec> {
        Layer2hlockW::new(self, 27)
    }
    #[doc = "Bit 28 - When set to 1, premultiply image alpha is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn layer2premult(&mut self) -> Layer2premultW<Layer2modeSpec> {
        Layer2premultW::new(self, 28)
    }
    #[doc = "Bit 29 - When set to 1, bilinear filtering is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn layer2bfilter(&mut self) -> Layer2bfilterW<Layer2modeSpec> {
        Layer2bfilterW::new(self, 29)
    }
    #[doc = "Bit 30 - When set to 1, force alpha with global alpha is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn layer2force(&mut self) -> Layer2forceW<Layer2modeSpec> {
        Layer2forceW::new(self, 30)
    }
    #[doc = "Bit 31 - When set to 1, layer n is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn layer2en(&mut self) -> Layer2enW<Layer2modeSpec> {
        Layer2enW::new(self, 31)
    }
}
#[doc = "Activate and set-up layer 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer2mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer2mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer2modeSpec;
impl crate::RegisterSpec for Layer2modeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer2mode::R`](R) reader structure"]
impl crate::Readable for Layer2modeSpec {}
#[doc = "`write(|w| ..)` method takes [`layer2mode::W`](W) writer structure"]
impl crate::Writable for Layer2modeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER2MODE to value 0"]
impl crate::Resettable for Layer2modeSpec {
    const RESET_VALUE: u32 = 0;
}
