#[doc = "Register `TEX3STRIDE` reader"]
pub type R = crate::R<Tex3strideSpec>;
#[doc = "Register `TEX3STRIDE` writer"]
pub type W = crate::W<Tex3strideSpec>;
#[doc = "Field `IMGSTRD` reader - image stride (signed) distance in bytes from one scanline to another"]
pub type ImgstrdR = crate::FieldReader<u16>;
#[doc = "Field `IMGSTRD` writer - image stride (signed) distance in bytes from one scanline to another"]
pub type ImgstrdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "image mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Imgmode {
    #[doc = "0: Texture mapping: rotate, resize and distort a bitmap image."]
    Pointsample = 0,
    #[doc = "1: Texture mapping: rotate, resize and distort a bitmap image. A method used to smooth textures when displayed larger or smaller than they actually are."]
    Bilinearfiltering = 1,
}
impl From<Imgmode> for u8 {
    #[inline(always)]
    fn from(variant: Imgmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Imgmode {
    type Ux = u8;
}
impl crate::IsEnum for Imgmode {}
#[doc = "Field `IMGMODE` reader - image mode"]
pub type ImgmodeR = crate::FieldReader<Imgmode>;
impl ImgmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Imgmode> {
        match self.bits {
            0 => Some(Imgmode::Pointsample),
            1 => Some(Imgmode::Bilinearfiltering),
            _ => None,
        }
    }
    #[doc = "Texture mapping: rotate, resize and distort a bitmap image."]
    #[inline(always)]
    pub fn is_pointsample(&self) -> bool {
        *self == Imgmode::Pointsample
    }
    #[doc = "Texture mapping: rotate, resize and distort a bitmap image. A method used to smooth textures when displayed larger or smaller than they actually are."]
    #[inline(always)]
    pub fn is_bilinearfiltering(&self) -> bool {
        *self == Imgmode::Bilinearfiltering
    }
}
#[doc = "Field `IMGMODE` writer - image mode"]
pub type ImgmodeW<'a, REG> = crate::FieldWriter<'a, REG, 8, Imgmode>;
impl<'a, REG> ImgmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Texture mapping: rotate, resize and distort a bitmap image."]
    #[inline(always)]
    pub fn pointsample(self) -> &'a mut crate::W<REG> {
        self.variant(Imgmode::Pointsample)
    }
    #[doc = "Texture mapping: rotate, resize and distort a bitmap image. A method used to smooth textures when displayed larger or smaller than they actually are."]
    #[inline(always)]
    pub fn bilinearfiltering(self) -> &'a mut crate::W<REG> {
        self.variant(Imgmode::Bilinearfiltering)
    }
}
#[doc = "image format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Imgfmt {
    #[doc = "0: Color Space RGBX means, that the pixel format still has an alpha channel, but it is ignored, and is always set to 255. The RGBX 32 bit RGB format is stored in memory as 8 red bits, 8 green bits, 8 blue bits, and 8 ignored bits."]
    Rgbx8888 = 0,
    #[doc = "1: Color Space RED GREEN BLUE ALPHA (internal format is always on: 32-bit)"]
    Rgba8888 = 1,
    #[doc = "2: Color Space"]
    Xrgb8888 = 2,
    #[doc = "3: Color Space In the ARGB (word-order) encoding the intensity of each channel sample is defined by 8 bits, and are arranged in memory in such manner that a single 32-bit unsigned integer has the alpha sample in the highest 8 bits, followed by the red sample, green sample and finally the blue sample in the lowest 8 bits"]
    Argb8888 = 3,
    #[doc = "4: Color Space RED(5-bits) GREEN(6-bits) BLUE (5-bits)"]
    Rgba565 = 4,
    #[doc = "5: Color Space Red,green,blue,alpha (transparency)."]
    Rgba5551 = 5,
    #[doc = "9: Color Space Lum8 grayscale"]
    L8 = 9,
    #[doc = "18: Color Space Proprietary texture compression"]
    Tsc4 = 18,
    #[doc = "22: Color Space Optional proprietary texture compression"]
    Tsc6 = 22,
    #[doc = "23: Color Space Optional proprietary texture compression"]
    Tsc6a = 23,
}
impl From<Imgfmt> for u8 {
    #[inline(always)]
    fn from(variant: Imgfmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Imgfmt {
    type Ux = u8;
}
impl crate::IsEnum for Imgfmt {}
#[doc = "Field `IMGFMT` reader - image format"]
pub type ImgfmtR = crate::FieldReader<Imgfmt>;
impl ImgfmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Imgfmt> {
        match self.bits {
            0 => Some(Imgfmt::Rgbx8888),
            1 => Some(Imgfmt::Rgba8888),
            2 => Some(Imgfmt::Xrgb8888),
            3 => Some(Imgfmt::Argb8888),
            4 => Some(Imgfmt::Rgba565),
            5 => Some(Imgfmt::Rgba5551),
            9 => Some(Imgfmt::L8),
            18 => Some(Imgfmt::Tsc4),
            22 => Some(Imgfmt::Tsc6),
            23 => Some(Imgfmt::Tsc6a),
            _ => None,
        }
    }
    #[doc = "Color Space RGBX means, that the pixel format still has an alpha channel, but it is ignored, and is always set to 255. The RGBX 32 bit RGB format is stored in memory as 8 red bits, 8 green bits, 8 blue bits, and 8 ignored bits."]
    #[inline(always)]
    pub fn is_rgbx8888(&self) -> bool {
        *self == Imgfmt::Rgbx8888
    }
    #[doc = "Color Space RED GREEN BLUE ALPHA (internal format is always on: 32-bit)"]
    #[inline(always)]
    pub fn is_rgba8888(&self) -> bool {
        *self == Imgfmt::Rgba8888
    }
    #[doc = "Color Space"]
    #[inline(always)]
    pub fn is_xrgb8888(&self) -> bool {
        *self == Imgfmt::Xrgb8888
    }
    #[doc = "Color Space In the ARGB (word-order) encoding the intensity of each channel sample is defined by 8 bits, and are arranged in memory in such manner that a single 32-bit unsigned integer has the alpha sample in the highest 8 bits, followed by the red sample, green sample and finally the blue sample in the lowest 8 bits"]
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == Imgfmt::Argb8888
    }
    #[doc = "Color Space RED(5-bits) GREEN(6-bits) BLUE (5-bits)"]
    #[inline(always)]
    pub fn is_rgba565(&self) -> bool {
        *self == Imgfmt::Rgba565
    }
    #[doc = "Color Space Red,green,blue,alpha (transparency)."]
    #[inline(always)]
    pub fn is_rgba5551(&self) -> bool {
        *self == Imgfmt::Rgba5551
    }
    #[doc = "Color Space Lum8 grayscale"]
    #[inline(always)]
    pub fn is_l8(&self) -> bool {
        *self == Imgfmt::L8
    }
    #[doc = "Color Space Proprietary texture compression"]
    #[inline(always)]
    pub fn is_tsc4(&self) -> bool {
        *self == Imgfmt::Tsc4
    }
    #[doc = "Color Space Optional proprietary texture compression"]
    #[inline(always)]
    pub fn is_tsc6(&self) -> bool {
        *self == Imgfmt::Tsc6
    }
    #[doc = "Color Space Optional proprietary texture compression"]
    #[inline(always)]
    pub fn is_tsc6a(&self) -> bool {
        *self == Imgfmt::Tsc6a
    }
}
#[doc = "Field `IMGFMT` writer - image format"]
pub type ImgfmtW<'a, REG> = crate::FieldWriter<'a, REG, 8, Imgfmt>;
impl<'a, REG> ImgfmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Color Space RGBX means, that the pixel format still has an alpha channel, but it is ignored, and is always set to 255. The RGBX 32 bit RGB format is stored in memory as 8 red bits, 8 green bits, 8 blue bits, and 8 ignored bits."]
    #[inline(always)]
    pub fn rgbx8888(self) -> &'a mut crate::W<REG> {
        self.variant(Imgfmt::Rgbx8888)
    }
    #[doc = "Color Space RED GREEN BLUE ALPHA (internal format is always on: 32-bit)"]
    #[inline(always)]
    pub fn rgba8888(self) -> &'a mut crate::W<REG> {
        self.variant(Imgfmt::Rgba8888)
    }
    #[doc = "Color Space"]
    #[inline(always)]
    pub fn xrgb8888(self) -> &'a mut crate::W<REG> {
        self.variant(Imgfmt::Xrgb8888)
    }
    #[doc = "Color Space In the ARGB (word-order) encoding the intensity of each channel sample is defined by 8 bits, and are arranged in memory in such manner that a single 32-bit unsigned integer has the alpha sample in the highest 8 bits, followed by the red sample, green sample and finally the blue sample in the lowest 8 bits"]
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut crate::W<REG> {
        self.variant(Imgfmt::Argb8888)
    }
    #[doc = "Color Space RED(5-bits) GREEN(6-bits) BLUE (5-bits)"]
    #[inline(always)]
    pub fn rgba565(self) -> &'a mut crate::W<REG> {
        self.variant(Imgfmt::Rgba565)
    }
    #[doc = "Color Space Red,green,blue,alpha (transparency)."]
    #[inline(always)]
    pub fn rgba5551(self) -> &'a mut crate::W<REG> {
        self.variant(Imgfmt::Rgba5551)
    }
    #[doc = "Color Space Lum8 grayscale"]
    #[inline(always)]
    pub fn l8(self) -> &'a mut crate::W<REG> {
        self.variant(Imgfmt::L8)
    }
    #[doc = "Color Space Proprietary texture compression"]
    #[inline(always)]
    pub fn tsc4(self) -> &'a mut crate::W<REG> {
        self.variant(Imgfmt::Tsc4)
    }
    #[doc = "Color Space Optional proprietary texture compression"]
    #[inline(always)]
    pub fn tsc6(self) -> &'a mut crate::W<REG> {
        self.variant(Imgfmt::Tsc6)
    }
    #[doc = "Color Space Optional proprietary texture compression"]
    #[inline(always)]
    pub fn tsc6a(self) -> &'a mut crate::W<REG> {
        self.variant(Imgfmt::Tsc6a)
    }
}
impl R {
    #[doc = "Bits 0:15 - image stride (signed) distance in bytes from one scanline to another"]
    #[inline(always)]
    pub fn imgstrd(&self) -> ImgstrdR {
        ImgstrdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - image mode"]
    #[inline(always)]
    pub fn imgmode(&self) -> ImgmodeR {
        ImgmodeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - image format"]
    #[inline(always)]
    pub fn imgfmt(&self) -> ImgfmtR {
        ImgfmtR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - image stride (signed) distance in bytes from one scanline to another"]
    #[inline(always)]
    #[must_use]
    pub fn imgstrd(&mut self) -> ImgstrdW<Tex3strideSpec> {
        ImgstrdW::new(self, 0)
    }
    #[doc = "Bits 16:23 - image mode"]
    #[inline(always)]
    #[must_use]
    pub fn imgmode(&mut self) -> ImgmodeW<Tex3strideSpec> {
        ImgmodeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - image format"]
    #[inline(always)]
    #[must_use]
    pub fn imgfmt(&mut self) -> ImgfmtW<Tex3strideSpec> {
        ImgfmtW::new(self, 24)
    }
}
#[doc = "mode and stride.\n\nYou can [`read`](crate::Reg::read) this register and get [`tex3stride::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tex3stride::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tex3strideSpec;
impl crate::RegisterSpec for Tex3strideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tex3stride::R`](R) reader structure"]
impl crate::Readable for Tex3strideSpec {}
#[doc = "`write(|w| ..)` method takes [`tex3stride::W`](W) writer structure"]
impl crate::Writable for Tex3strideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEX3STRIDE to value 0"]
impl crate::Resettable for Tex3strideSpec {
    const RESET_VALUE: u32 = 0;
}
