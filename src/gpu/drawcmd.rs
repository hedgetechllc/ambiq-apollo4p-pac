#[doc = "Register `DRAWCMD` reader"]
pub type R = crate::R<DrawcmdSpec>;
#[doc = "Register `DRAWCMD` writer"]
pub type W = crate::W<DrawcmdSpec>;
#[doc = "Start the draw command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Start {
    #[doc = "0: draw pixel using STARTXY"]
    Pixel = 0,
    #[doc = "1: draw line from STARTXY to ENDXY"]
    Line = 1,
    #[doc = "2: fill rectangle from STARTXY to ENDXY"]
    Rect = 2,
    #[doc = "3: draw triangle (if enabled)"]
    Tri = 3,
    #[doc = "4: draw quadrilateral (if enabled)"]
    Quad = 4,
}
impl From<Start> for u8 {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Start {
    type Ux = u8;
}
impl crate::IsEnum for Start {}
#[doc = "Field `START` reader - Start the draw command"]
pub type StartR = crate::FieldReader<Start>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Start> {
        match self.bits {
            0 => Some(Start::Pixel),
            1 => Some(Start::Line),
            2 => Some(Start::Rect),
            3 => Some(Start::Tri),
            4 => Some(Start::Quad),
            _ => None,
        }
    }
    #[doc = "draw pixel using STARTXY"]
    #[inline(always)]
    pub fn is_pixel(&self) -> bool {
        *self == Start::Pixel
    }
    #[doc = "draw line from STARTXY to ENDXY"]
    #[inline(always)]
    pub fn is_line(&self) -> bool {
        *self == Start::Line
    }
    #[doc = "fill rectangle from STARTXY to ENDXY"]
    #[inline(always)]
    pub fn is_rect(&self) -> bool {
        *self == Start::Rect
    }
    #[doc = "draw triangle (if enabled)"]
    #[inline(always)]
    pub fn is_tri(&self) -> bool {
        *self == Start::Tri
    }
    #[doc = "draw quadrilateral (if enabled)"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == Start::Quad
    }
}
#[doc = "Field `START` writer - Start the draw command"]
pub type StartW<'a, REG> = crate::FieldWriter<'a, REG, 3, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "draw pixel using STARTXY"]
    #[inline(always)]
    pub fn pixel(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Pixel)
    }
    #[doc = "draw line from STARTXY to ENDXY"]
    #[inline(always)]
    pub fn line(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Line)
    }
    #[doc = "fill rectangle from STARTXY to ENDXY"]
    #[inline(always)]
    pub fn rect(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Rect)
    }
    #[doc = "draw triangle (if enabled)"]
    #[inline(always)]
    pub fn tri(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Tri)
    }
    #[doc = "draw quadrilateral (if enabled)"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Quad)
    }
}
#[doc = "Field `RSVD` reader - This bitfield is reserved."]
pub type RsvdR = crate::FieldReader<u32>;
#[doc = "Field `RSVD` writer - This bitfield is reserved."]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:2 - Start the draw command"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - This bitfield is reserved."]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - Start the draw command"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<DrawcmdSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bits 3:31 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd(&mut self) -> RsvdW<DrawcmdSpec> {
        RsvdW::new(self, 3)
    }
}
#[doc = "Rasterizer drawing command.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrawcmdSpec;
impl crate::RegisterSpec for DrawcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`drawcmd::R`](R) reader structure"]
impl crate::Readable for DrawcmdSpec {}
#[doc = "`write(|w| ..)` method takes [`drawcmd::W`](W) writer structure"]
impl crate::Writable for DrawcmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DRAWCMD to value 0"]
impl crate::Resettable for DrawcmdSpec {
    const RESET_VALUE: u32 = 0;
}
