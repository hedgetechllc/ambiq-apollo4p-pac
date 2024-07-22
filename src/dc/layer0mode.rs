#[doc = "Register `LAYER0MODE` reader"]
pub type R = crate::R<Layer0modeSpec>;
#[doc = "Register `LAYER0MODE` writer"]
pub type W = crate::W<Layer0modeSpec>;
#[doc = "Color mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Layer0colmode {
    #[doc = "0: 8-bit color palette look-up table (LUT8)"]
    Layer0cmLutble = 0,
    #[doc = "1: 16-bit RGBX5551 color format"]
    Layer0cmRgbx5551 = 1,
    #[doc = "2: 32-bit RGBX8888 color format"]
    Layer0cmRgbx8888 = 2,
    #[doc = "4: 8-bit RGB332 color format"]
    Layer0cmRgb332 = 4,
    #[doc = "5: 16-bit RGB565 color format"]
    Layer0cmRgb565 = 5,
    #[doc = "6: 32-bit XRGB8888 color format"]
    Layer0cmXrgb8888 = 6,
    #[doc = "7: L8 Grayscale/Palette format"]
    Layer0cmL8 = 7,
    #[doc = "8: L1 Grayscale/Palette format"]
    Layer0cmL1 = 8,
    #[doc = "9: L4 Grayscale/Palette format"]
    Layer0cmL4 = 9,
    #[doc = "10: color format"]
    Layer0cmYuyv = 10,
    #[doc = "11: 24-bit RGB color format"]
    Layer0cmRbg = 11,
    #[doc = "12: YUY2 color format"]
    Layer0cmYuy2 = 12,
    #[doc = "13: 32-bit ABGR8888 color format"]
    Layer0cmAbgr8888 = 13,
    #[doc = "14: 32-bit BGRA8888 color format"]
    Layer0cmBgra8888 = 14,
    #[doc = "16: Video 420 Mode"]
    Layer0cmVideo = 16,
    #[doc = "17: Trilinea 420 Video Mode"]
    Layer0cmTrilinear = 17,
}
impl From<Layer0colmode> for u8 {
    #[inline(always)]
    fn from(variant: Layer0colmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Layer0colmode {
    type Ux = u8;
}
impl crate::IsEnum for Layer0colmode {}
#[doc = "Field `LAYER0COLMODE` reader - Color mode"]
pub type Layer0colmodeR = crate::FieldReader<Layer0colmode>;
impl Layer0colmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Layer0colmode> {
        match self.bits {
            0 => Some(Layer0colmode::Layer0cmLutble),
            1 => Some(Layer0colmode::Layer0cmRgbx5551),
            2 => Some(Layer0colmode::Layer0cmRgbx8888),
            4 => Some(Layer0colmode::Layer0cmRgb332),
            5 => Some(Layer0colmode::Layer0cmRgb565),
            6 => Some(Layer0colmode::Layer0cmXrgb8888),
            7 => Some(Layer0colmode::Layer0cmL8),
            8 => Some(Layer0colmode::Layer0cmL1),
            9 => Some(Layer0colmode::Layer0cmL4),
            10 => Some(Layer0colmode::Layer0cmYuyv),
            11 => Some(Layer0colmode::Layer0cmRbg),
            12 => Some(Layer0colmode::Layer0cmYuy2),
            13 => Some(Layer0colmode::Layer0cmAbgr8888),
            14 => Some(Layer0colmode::Layer0cmBgra8888),
            16 => Some(Layer0colmode::Layer0cmVideo),
            17 => Some(Layer0colmode::Layer0cmTrilinear),
            _ => None,
        }
    }
    #[doc = "8-bit color palette look-up table (LUT8)"]
    #[inline(always)]
    pub fn is_layer0cm_lutble(&self) -> bool {
        *self == Layer0colmode::Layer0cmLutble
    }
    #[doc = "16-bit RGBX5551 color format"]
    #[inline(always)]
    pub fn is_layer0cm_rgbx5551(&self) -> bool {
        *self == Layer0colmode::Layer0cmRgbx5551
    }
    #[doc = "32-bit RGBX8888 color format"]
    #[inline(always)]
    pub fn is_layer0cm_rgbx8888(&self) -> bool {
        *self == Layer0colmode::Layer0cmRgbx8888
    }
    #[doc = "8-bit RGB332 color format"]
    #[inline(always)]
    pub fn is_layer0cm_rgb332(&self) -> bool {
        *self == Layer0colmode::Layer0cmRgb332
    }
    #[doc = "16-bit RGB565 color format"]
    #[inline(always)]
    pub fn is_layer0cm_rgb565(&self) -> bool {
        *self == Layer0colmode::Layer0cmRgb565
    }
    #[doc = "32-bit XRGB8888 color format"]
    #[inline(always)]
    pub fn is_layer0cm_xrgb8888(&self) -> bool {
        *self == Layer0colmode::Layer0cmXrgb8888
    }
    #[doc = "L8 Grayscale/Palette format"]
    #[inline(always)]
    pub fn is_layer0cm_l8(&self) -> bool {
        *self == Layer0colmode::Layer0cmL8
    }
    #[doc = "L1 Grayscale/Palette format"]
    #[inline(always)]
    pub fn is_layer0cm_l1(&self) -> bool {
        *self == Layer0colmode::Layer0cmL1
    }
    #[doc = "L4 Grayscale/Palette format"]
    #[inline(always)]
    pub fn is_layer0cm_l4(&self) -> bool {
        *self == Layer0colmode::Layer0cmL4
    }
    #[doc = "color format"]
    #[inline(always)]
    pub fn is_layer0cm_yuyv(&self) -> bool {
        *self == Layer0colmode::Layer0cmYuyv
    }
    #[doc = "24-bit RGB color format"]
    #[inline(always)]
    pub fn is_layer0cm_rbg(&self) -> bool {
        *self == Layer0colmode::Layer0cmRbg
    }
    #[doc = "YUY2 color format"]
    #[inline(always)]
    pub fn is_layer0cm_yuy2(&self) -> bool {
        *self == Layer0colmode::Layer0cmYuy2
    }
    #[doc = "32-bit ABGR8888 color format"]
    #[inline(always)]
    pub fn is_layer0cm_abgr8888(&self) -> bool {
        *self == Layer0colmode::Layer0cmAbgr8888
    }
    #[doc = "32-bit BGRA8888 color format"]
    #[inline(always)]
    pub fn is_layer0cm_bgra8888(&self) -> bool {
        *self == Layer0colmode::Layer0cmBgra8888
    }
    #[doc = "Video 420 Mode"]
    #[inline(always)]
    pub fn is_layer0cm_video(&self) -> bool {
        *self == Layer0colmode::Layer0cmVideo
    }
    #[doc = "Trilinea 420 Video Mode"]
    #[inline(always)]
    pub fn is_layer0cm_trilinear(&self) -> bool {
        *self == Layer0colmode::Layer0cmTrilinear
    }
}
#[doc = "Field `LAYER0COLMODE` writer - Color mode"]
pub type Layer0colmodeW<'a, REG> = crate::FieldWriter<'a, REG, 5, Layer0colmode>;
impl<'a, REG> Layer0colmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-bit color palette look-up table (LUT8)"]
    #[inline(always)]
    pub fn layer0cm_lutble(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0colmode::Layer0cmLutble)
    }
    #[doc = "16-bit RGBX5551 color format"]
    #[inline(always)]
    pub fn layer0cm_rgbx5551(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0colmode::Layer0cmRgbx5551)
    }
    #[doc = "32-bit RGBX8888 color format"]
    #[inline(always)]
    pub fn layer0cm_rgbx8888(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0colmode::Layer0cmRgbx8888)
    }
    #[doc = "8-bit RGB332 color format"]
    #[inline(always)]
    pub fn layer0cm_rgb332(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0colmode::Layer0cmRgb332)
    }
    #[doc = "16-bit RGB565 color format"]
    #[inline(always)]
    pub fn layer0cm_rgb565(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0colmode::Layer0cmRgb565)
    }
    #[doc = "32-bit XRGB8888 color format"]
    #[inline(always)]
    pub fn layer0cm_xrgb8888(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0colmode::Layer0cmXrgb8888)
    }
    #[doc = "L8 Grayscale/Palette format"]
    #[inline(always)]
    pub fn layer0cm_l8(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0colmode::Layer0cmL8)
    }
    #[doc = "L1 Grayscale/Palette format"]
    #[inline(always)]
    pub fn layer0cm_l1(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0colmode::Layer0cmL1)
    }
    #[doc = "L4 Grayscale/Palette format"]
    #[inline(always)]
    pub fn layer0cm_l4(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0colmode::Layer0cmL4)
    }
    #[doc = "color format"]
    #[inline(always)]
    pub fn layer0cm_yuyv(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0colmode::Layer0cmYuyv)
    }
    #[doc = "24-bit RGB color format"]
    #[inline(always)]
    pub fn layer0cm_rbg(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0colmode::Layer0cmRbg)
    }
    #[doc = "YUY2 color format"]
    #[inline(always)]
    pub fn layer0cm_yuy2(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0colmode::Layer0cmYuy2)
    }
    #[doc = "32-bit ABGR8888 color format"]
    #[inline(always)]
    pub fn layer0cm_abgr8888(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0colmode::Layer0cmAbgr8888)
    }
    #[doc = "32-bit BGRA8888 color format"]
    #[inline(always)]
    pub fn layer0cm_bgra8888(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0colmode::Layer0cmBgra8888)
    }
    #[doc = "Video 420 Mode"]
    #[inline(always)]
    pub fn layer0cm_video(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0colmode::Layer0cmVideo)
    }
    #[doc = "Trilinea 420 Video Mode"]
    #[inline(always)]
    pub fn layer0cm_trilinear(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0colmode::Layer0cmTrilinear)
    }
}
#[doc = "Field `RSVD0` reader - This field is reserved."]
pub type Rsvd0R = crate::FieldReader;
#[doc = "Field `RSVD0` writer - This field is reserved."]
pub type Rsvd0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Source blending function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Layer0sblend {
    #[doc = "0: blend black"]
    Layer0SblackBlend = 0,
    #[doc = "1: blend white"]
    Layer0SwhiteBlend = 1,
    #[doc = "2: blend alpha source"]
    Layer0SalbhasBlend = 2,
    #[doc = "3: blend alpha global"]
    Layer0SalphagBlend = 3,
    #[doc = "4: blend alpha source and alpha global"]
    Layer0SalphasgBlend = 4,
    #[doc = "5: blend inverted source"]
    Layer0SinvertBlendSrc = 5,
    #[doc = "6: blend inverted global"]
    Layer0SinvertGlobalBlend = 6,
    #[doc = "7: blend inverted source and inverted global"]
    Layer0SinvertsgBlend = 7,
    #[doc = "10: blend alpha destination"]
    Layer0SalphaBlend = 10,
    #[doc = "13: blend inverted destination"]
    Layer0SinvertBlendDst = 13,
}
impl From<Layer0sblend> for u8 {
    #[inline(always)]
    fn from(variant: Layer0sblend) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Layer0sblend {
    type Ux = u8;
}
impl crate::IsEnum for Layer0sblend {}
#[doc = "Field `LAYER0SBLEND` reader - Source blending function"]
pub type Layer0sblendR = crate::FieldReader<Layer0sblend>;
impl Layer0sblendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Layer0sblend> {
        match self.bits {
            0 => Some(Layer0sblend::Layer0SblackBlend),
            1 => Some(Layer0sblend::Layer0SwhiteBlend),
            2 => Some(Layer0sblend::Layer0SalbhasBlend),
            3 => Some(Layer0sblend::Layer0SalphagBlend),
            4 => Some(Layer0sblend::Layer0SalphasgBlend),
            5 => Some(Layer0sblend::Layer0SinvertBlendSrc),
            6 => Some(Layer0sblend::Layer0SinvertGlobalBlend),
            7 => Some(Layer0sblend::Layer0SinvertsgBlend),
            10 => Some(Layer0sblend::Layer0SalphaBlend),
            13 => Some(Layer0sblend::Layer0SinvertBlendDst),
            _ => None,
        }
    }
    #[doc = "blend black"]
    #[inline(always)]
    pub fn is_layer0_sblack_blend(&self) -> bool {
        *self == Layer0sblend::Layer0SblackBlend
    }
    #[doc = "blend white"]
    #[inline(always)]
    pub fn is_layer0_swhite_blend(&self) -> bool {
        *self == Layer0sblend::Layer0SwhiteBlend
    }
    #[doc = "blend alpha source"]
    #[inline(always)]
    pub fn is_layer0_salbhas_blend(&self) -> bool {
        *self == Layer0sblend::Layer0SalbhasBlend
    }
    #[doc = "blend alpha global"]
    #[inline(always)]
    pub fn is_layer0_salphag_blend(&self) -> bool {
        *self == Layer0sblend::Layer0SalphagBlend
    }
    #[doc = "blend alpha source and alpha global"]
    #[inline(always)]
    pub fn is_layer0_salphasg_blend(&self) -> bool {
        *self == Layer0sblend::Layer0SalphasgBlend
    }
    #[doc = "blend inverted source"]
    #[inline(always)]
    pub fn is_layer0_sinvert_blend_src(&self) -> bool {
        *self == Layer0sblend::Layer0SinvertBlendSrc
    }
    #[doc = "blend inverted global"]
    #[inline(always)]
    pub fn is_layer0_sinvert_global_blend(&self) -> bool {
        *self == Layer0sblend::Layer0SinvertGlobalBlend
    }
    #[doc = "blend inverted source and inverted global"]
    #[inline(always)]
    pub fn is_layer0_sinvertsg_blend(&self) -> bool {
        *self == Layer0sblend::Layer0SinvertsgBlend
    }
    #[doc = "blend alpha destination"]
    #[inline(always)]
    pub fn is_layer0_salpha_blend(&self) -> bool {
        *self == Layer0sblend::Layer0SalphaBlend
    }
    #[doc = "blend inverted destination"]
    #[inline(always)]
    pub fn is_layer0_sinvert_blend_dst(&self) -> bool {
        *self == Layer0sblend::Layer0SinvertBlendDst
    }
}
#[doc = "Field `LAYER0SBLEND` writer - Source blending function"]
pub type Layer0sblendW<'a, REG> = crate::FieldWriter<'a, REG, 4, Layer0sblend>;
impl<'a, REG> Layer0sblendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "blend black"]
    #[inline(always)]
    pub fn layer0_sblack_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0sblend::Layer0SblackBlend)
    }
    #[doc = "blend white"]
    #[inline(always)]
    pub fn layer0_swhite_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0sblend::Layer0SwhiteBlend)
    }
    #[doc = "blend alpha source"]
    #[inline(always)]
    pub fn layer0_salbhas_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0sblend::Layer0SalbhasBlend)
    }
    #[doc = "blend alpha global"]
    #[inline(always)]
    pub fn layer0_salphag_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0sblend::Layer0SalphagBlend)
    }
    #[doc = "blend alpha source and alpha global"]
    #[inline(always)]
    pub fn layer0_salphasg_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0sblend::Layer0SalphasgBlend)
    }
    #[doc = "blend inverted source"]
    #[inline(always)]
    pub fn layer0_sinvert_blend_src(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0sblend::Layer0SinvertBlendSrc)
    }
    #[doc = "blend inverted global"]
    #[inline(always)]
    pub fn layer0_sinvert_global_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0sblend::Layer0SinvertGlobalBlend)
    }
    #[doc = "blend inverted source and inverted global"]
    #[inline(always)]
    pub fn layer0_sinvertsg_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0sblend::Layer0SinvertsgBlend)
    }
    #[doc = "blend alpha destination"]
    #[inline(always)]
    pub fn layer0_salpha_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0sblend::Layer0SalphaBlend)
    }
    #[doc = "blend inverted destination"]
    #[inline(always)]
    pub fn layer0_sinvert_blend_dst(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0sblend::Layer0SinvertBlendDst)
    }
}
#[doc = "Destination blending function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Layer0dblend {
    #[doc = "0: blend black"]
    Layer0DblackBlend = 0,
    #[doc = "1: blend white"]
    Layer0DwhiteBlend = 1,
    #[doc = "2: blend alpha source"]
    Layer0DalbhasBlend = 2,
    #[doc = "3: blend alpha global"]
    Layer0DalphagBlend = 3,
    #[doc = "4: blend alpha source and alpha global"]
    Layer0DalphasgBlend = 4,
    #[doc = "5: blend inverted source"]
    Layer0DinvertBlendSrc = 5,
    #[doc = "6: blend inverted global"]
    Layer0DinvertGlobalBlend = 6,
    #[doc = "7: blend inverted source and inverted global"]
    Layer0DinvertsgBlend = 7,
    #[doc = "10: blend alpha destination"]
    Layer0DalphaBlend = 10,
    #[doc = "13: blend inverted destination"]
    Layer0DinvertBlendDst = 13,
}
impl From<Layer0dblend> for u8 {
    #[inline(always)]
    fn from(variant: Layer0dblend) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Layer0dblend {
    type Ux = u8;
}
impl crate::IsEnum for Layer0dblend {}
#[doc = "Field `LAYER0DBLEND` reader - Destination blending function"]
pub type Layer0dblendR = crate::FieldReader<Layer0dblend>;
impl Layer0dblendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Layer0dblend> {
        match self.bits {
            0 => Some(Layer0dblend::Layer0DblackBlend),
            1 => Some(Layer0dblend::Layer0DwhiteBlend),
            2 => Some(Layer0dblend::Layer0DalbhasBlend),
            3 => Some(Layer0dblend::Layer0DalphagBlend),
            4 => Some(Layer0dblend::Layer0DalphasgBlend),
            5 => Some(Layer0dblend::Layer0DinvertBlendSrc),
            6 => Some(Layer0dblend::Layer0DinvertGlobalBlend),
            7 => Some(Layer0dblend::Layer0DinvertsgBlend),
            10 => Some(Layer0dblend::Layer0DalphaBlend),
            13 => Some(Layer0dblend::Layer0DinvertBlendDst),
            _ => None,
        }
    }
    #[doc = "blend black"]
    #[inline(always)]
    pub fn is_layer0_dblack_blend(&self) -> bool {
        *self == Layer0dblend::Layer0DblackBlend
    }
    #[doc = "blend white"]
    #[inline(always)]
    pub fn is_layer0_dwhite_blend(&self) -> bool {
        *self == Layer0dblend::Layer0DwhiteBlend
    }
    #[doc = "blend alpha source"]
    #[inline(always)]
    pub fn is_layer0_dalbhas_blend(&self) -> bool {
        *self == Layer0dblend::Layer0DalbhasBlend
    }
    #[doc = "blend alpha global"]
    #[inline(always)]
    pub fn is_layer0_dalphag_blend(&self) -> bool {
        *self == Layer0dblend::Layer0DalphagBlend
    }
    #[doc = "blend alpha source and alpha global"]
    #[inline(always)]
    pub fn is_layer0_dalphasg_blend(&self) -> bool {
        *self == Layer0dblend::Layer0DalphasgBlend
    }
    #[doc = "blend inverted source"]
    #[inline(always)]
    pub fn is_layer0_dinvert_blend_src(&self) -> bool {
        *self == Layer0dblend::Layer0DinvertBlendSrc
    }
    #[doc = "blend inverted global"]
    #[inline(always)]
    pub fn is_layer0_dinvert_global_blend(&self) -> bool {
        *self == Layer0dblend::Layer0DinvertGlobalBlend
    }
    #[doc = "blend inverted source and inverted global"]
    #[inline(always)]
    pub fn is_layer0_dinvertsg_blend(&self) -> bool {
        *self == Layer0dblend::Layer0DinvertsgBlend
    }
    #[doc = "blend alpha destination"]
    #[inline(always)]
    pub fn is_layer0_dalpha_blend(&self) -> bool {
        *self == Layer0dblend::Layer0DalphaBlend
    }
    #[doc = "blend inverted destination"]
    #[inline(always)]
    pub fn is_layer0_dinvert_blend_dst(&self) -> bool {
        *self == Layer0dblend::Layer0DinvertBlendDst
    }
}
#[doc = "Field `LAYER0DBLEND` writer - Destination blending function"]
pub type Layer0dblendW<'a, REG> = crate::FieldWriter<'a, REG, 4, Layer0dblend>;
impl<'a, REG> Layer0dblendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "blend black"]
    #[inline(always)]
    pub fn layer0_dblack_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0dblend::Layer0DblackBlend)
    }
    #[doc = "blend white"]
    #[inline(always)]
    pub fn layer0_dwhite_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0dblend::Layer0DwhiteBlend)
    }
    #[doc = "blend alpha source"]
    #[inline(always)]
    pub fn layer0_dalbhas_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0dblend::Layer0DalbhasBlend)
    }
    #[doc = "blend alpha global"]
    #[inline(always)]
    pub fn layer0_dalphag_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0dblend::Layer0DalphagBlend)
    }
    #[doc = "blend alpha source and alpha global"]
    #[inline(always)]
    pub fn layer0_dalphasg_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0dblend::Layer0DalphasgBlend)
    }
    #[doc = "blend inverted source"]
    #[inline(always)]
    pub fn layer0_dinvert_blend_src(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0dblend::Layer0DinvertBlendSrc)
    }
    #[doc = "blend inverted global"]
    #[inline(always)]
    pub fn layer0_dinvert_global_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0dblend::Layer0DinvertGlobalBlend)
    }
    #[doc = "blend inverted source and inverted global"]
    #[inline(always)]
    pub fn layer0_dinvertsg_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0dblend::Layer0DinvertsgBlend)
    }
    #[doc = "blend alpha destination"]
    #[inline(always)]
    pub fn layer0_dalpha_blend(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0dblend::Layer0DalphaBlend)
    }
    #[doc = "blend inverted destination"]
    #[inline(always)]
    pub fn layer0_dinvert_blend_dst(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0dblend::Layer0DinvertBlendDst)
    }
}
#[doc = "Field `LAYER0ALPHA` reader - Alpha layer global value (0x00-0xFF range)"]
pub type Layer0alphaR = crate::FieldReader;
#[doc = "Field `LAYER0ALPHA` writer - Alpha layer global value (0x00-0xFF range)"]
pub type Layer0alphaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RSVD1` reader - This field is reserved."]
pub type Rsvd1R = crate::FieldReader;
#[doc = "Field `RSVD1` writer - This field is reserved."]
pub type Rsvd1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LAYER0GAMMA` reader - When set to 1, Gamma Look Up Table is enabled"]
pub type Layer0gammaR = crate::BitReader;
#[doc = "Field `LAYER0GAMMA` writer - When set to 1, Gamma Look Up Table is enabled"]
pub type Layer0gammaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAYER0HLOCK` reader - When set to 1, HLOCK signal on AHB DMAs is asserted"]
pub type Layer0hlockR = crate::BitReader;
#[doc = "Field `LAYER0HLOCK` writer - When set to 1, HLOCK signal on AHB DMAs is asserted"]
pub type Layer0hlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAYER0PREMULT` reader - When set to 1, premultiply image alpha is enabled"]
pub type Layer0premultR = crate::BitReader;
#[doc = "Field `LAYER0PREMULT` writer - When set to 1, premultiply image alpha is enabled"]
pub type Layer0premultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAYER0BFILTER` reader - When set to 1, bilinear filtering is enabled"]
pub type Layer0bfilterR = crate::BitReader;
#[doc = "Field `LAYER0BFILTER` writer - When set to 1, bilinear filtering is enabled"]
pub type Layer0bfilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAYER0FORCE` reader - When set to 1, force alpha with global alpha is enabled"]
pub type Layer0forceR = crate::BitReader;
#[doc = "Field `LAYER0FORCE` writer - When set to 1, force alpha with global alpha is enabled"]
pub type Layer0forceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAYER0EN` reader - When set to 1, layer n is enabled"]
pub type Layer0enR = crate::BitReader;
#[doc = "Field `LAYER0EN` writer - When set to 1, layer n is enabled"]
pub type Layer0enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Color mode"]
    #[inline(always)]
    pub fn layer0colmode(&self) -> Layer0colmodeR {
        Layer0colmodeR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd0(&self) -> Rsvd0R {
        Rsvd0R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Source blending function"]
    #[inline(always)]
    pub fn layer0sblend(&self) -> Layer0sblendR {
        Layer0sblendR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Destination blending function"]
    #[inline(always)]
    pub fn layer0dblend(&self) -> Layer0dblendR {
        Layer0dblendR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Alpha layer global value (0x00-0xFF range)"]
    #[inline(always)]
    pub fn layer0alpha(&self) -> Layer0alphaR {
        Layer0alphaR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd1(&self) -> Rsvd1R {
        Rsvd1R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - When set to 1, Gamma Look Up Table is enabled"]
    #[inline(always)]
    pub fn layer0gamma(&self) -> Layer0gammaR {
        Layer0gammaR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - When set to 1, HLOCK signal on AHB DMAs is asserted"]
    #[inline(always)]
    pub fn layer0hlock(&self) -> Layer0hlockR {
        Layer0hlockR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - When set to 1, premultiply image alpha is enabled"]
    #[inline(always)]
    pub fn layer0premult(&self) -> Layer0premultR {
        Layer0premultR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - When set to 1, bilinear filtering is enabled"]
    #[inline(always)]
    pub fn layer0bfilter(&self) -> Layer0bfilterR {
        Layer0bfilterR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When set to 1, force alpha with global alpha is enabled"]
    #[inline(always)]
    pub fn layer0force(&self) -> Layer0forceR {
        Layer0forceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When set to 1, layer n is enabled"]
    #[inline(always)]
    pub fn layer0en(&self) -> Layer0enR {
        Layer0enR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Color mode"]
    #[inline(always)]
    #[must_use]
    pub fn layer0colmode(&mut self) -> Layer0colmodeW<Layer0modeSpec> {
        Layer0colmodeW::new(self, 0)
    }
    #[doc = "Bits 5:7 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd0(&mut self) -> Rsvd0W<Layer0modeSpec> {
        Rsvd0W::new(self, 5)
    }
    #[doc = "Bits 8:11 - Source blending function"]
    #[inline(always)]
    #[must_use]
    pub fn layer0sblend(&mut self) -> Layer0sblendW<Layer0modeSpec> {
        Layer0sblendW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Destination blending function"]
    #[inline(always)]
    #[must_use]
    pub fn layer0dblend(&mut self) -> Layer0dblendW<Layer0modeSpec> {
        Layer0dblendW::new(self, 12)
    }
    #[doc = "Bits 16:23 - Alpha layer global value (0x00-0xFF range)"]
    #[inline(always)]
    #[must_use]
    pub fn layer0alpha(&mut self) -> Layer0alphaW<Layer0modeSpec> {
        Layer0alphaW::new(self, 16)
    }
    #[doc = "Bits 24:25 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd1(&mut self) -> Rsvd1W<Layer0modeSpec> {
        Rsvd1W::new(self, 24)
    }
    #[doc = "Bit 26 - When set to 1, Gamma Look Up Table is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn layer0gamma(&mut self) -> Layer0gammaW<Layer0modeSpec> {
        Layer0gammaW::new(self, 26)
    }
    #[doc = "Bit 27 - When set to 1, HLOCK signal on AHB DMAs is asserted"]
    #[inline(always)]
    #[must_use]
    pub fn layer0hlock(&mut self) -> Layer0hlockW<Layer0modeSpec> {
        Layer0hlockW::new(self, 27)
    }
    #[doc = "Bit 28 - When set to 1, premultiply image alpha is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn layer0premult(&mut self) -> Layer0premultW<Layer0modeSpec> {
        Layer0premultW::new(self, 28)
    }
    #[doc = "Bit 29 - When set to 1, bilinear filtering is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn layer0bfilter(&mut self) -> Layer0bfilterW<Layer0modeSpec> {
        Layer0bfilterW::new(self, 29)
    }
    #[doc = "Bit 30 - When set to 1, force alpha with global alpha is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn layer0force(&mut self) -> Layer0forceW<Layer0modeSpec> {
        Layer0forceW::new(self, 30)
    }
    #[doc = "Bit 31 - When set to 1, layer n is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn layer0en(&mut self) -> Layer0enW<Layer0modeSpec> {
        Layer0enW::new(self, 31)
    }
}
#[doc = "LAYER0_MODE: Activate and set-up layer 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer0mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer0modeSpec;
impl crate::RegisterSpec for Layer0modeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer0mode::R`](R) reader structure"]
impl crate::Readable for Layer0modeSpec {}
#[doc = "`write(|w| ..)` method takes [`layer0mode::W`](W) writer structure"]
impl crate::Writable for Layer0modeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER0MODE to value 0"]
impl crate::Resettable for Layer0modeSpec {
    const RESET_VALUE: u32 = 0;
}
