#[doc = "Register `LAYER0STRIDE` reader"]
pub type R = crate::R<Layer0strideSpec>;
#[doc = "Register `LAYER0STRIDE` writer"]
pub type W = crate::W<Layer0strideSpec>;
#[doc = "Field `LAYER0STRIDEDIST` reader - Specify the stride, which is the distance from line to line in bytes for each layer 0 memory"]
pub type Layer0stridedistR = crate::FieldReader<u16>;
#[doc = "Field `LAYER0STRIDEDIST` writer - Specify the stride, which is the distance from line to line in bytes for each layer 0 memory"]
pub type Layer0stridedistW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Specify the AXI bits per burst in layer 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Layer0axiburstbits {
    #[doc = "0: 16-beats (default)"]
    Layer0BeatsDef = 0,
    #[doc = "1: 2-beats"]
    Layer0Beats2 = 1,
    #[doc = "2: 4-beats"]
    Layer0Beats4 = 2,
    #[doc = "3: 8-beats"]
    Layer0Beats8 = 3,
    #[doc = "4: 16-beats (CHECK mistake?)"]
    Layer0Beats16 = 4,
    #[doc = "5: 32-beats (AXI4 only)"]
    Layer0Beats32 = 5,
    #[doc = "6: 64-beats (AXI4 only)"]
    Layer0Beats64 = 6,
    #[doc = "7: 128-beats (AXI4 only)"]
    Layer0Beats128 = 7,
}
impl From<Layer0axiburstbits> for u8 {
    #[inline(always)]
    fn from(variant: Layer0axiburstbits) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Layer0axiburstbits {
    type Ux = u8;
}
impl crate::IsEnum for Layer0axiburstbits {}
#[doc = "Field `LAYER0AXIBURSTBITS` reader - Specify the AXI bits per burst in layer 0"]
pub type Layer0axiburstbitsR = crate::FieldReader<Layer0axiburstbits>;
impl Layer0axiburstbitsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Layer0axiburstbits {
        match self.bits {
            0 => Layer0axiburstbits::Layer0BeatsDef,
            1 => Layer0axiburstbits::Layer0Beats2,
            2 => Layer0axiburstbits::Layer0Beats4,
            3 => Layer0axiburstbits::Layer0Beats8,
            4 => Layer0axiburstbits::Layer0Beats16,
            5 => Layer0axiburstbits::Layer0Beats32,
            6 => Layer0axiburstbits::Layer0Beats64,
            7 => Layer0axiburstbits::Layer0Beats128,
            _ => unreachable!(),
        }
    }
    #[doc = "16-beats (default)"]
    #[inline(always)]
    pub fn is_layer0_beats_def(&self) -> bool {
        *self == Layer0axiburstbits::Layer0BeatsDef
    }
    #[doc = "2-beats"]
    #[inline(always)]
    pub fn is_layer0_beats_2(&self) -> bool {
        *self == Layer0axiburstbits::Layer0Beats2
    }
    #[doc = "4-beats"]
    #[inline(always)]
    pub fn is_layer0_beats_4(&self) -> bool {
        *self == Layer0axiburstbits::Layer0Beats4
    }
    #[doc = "8-beats"]
    #[inline(always)]
    pub fn is_layer0_beats_8(&self) -> bool {
        *self == Layer0axiburstbits::Layer0Beats8
    }
    #[doc = "16-beats (CHECK mistake?)"]
    #[inline(always)]
    pub fn is_layer0_beats_16(&self) -> bool {
        *self == Layer0axiburstbits::Layer0Beats16
    }
    #[doc = "32-beats (AXI4 only)"]
    #[inline(always)]
    pub fn is_layer0_beats_32(&self) -> bool {
        *self == Layer0axiburstbits::Layer0Beats32
    }
    #[doc = "64-beats (AXI4 only)"]
    #[inline(always)]
    pub fn is_layer0_beats_64(&self) -> bool {
        *self == Layer0axiburstbits::Layer0Beats64
    }
    #[doc = "128-beats (AXI4 only)"]
    #[inline(always)]
    pub fn is_layer0_beats_128(&self) -> bool {
        *self == Layer0axiburstbits::Layer0Beats128
    }
}
#[doc = "Field `LAYER0AXIBURSTBITS` writer - Specify the AXI bits per burst in layer 0"]
pub type Layer0axiburstbitsW<'a, REG> =
    crate::FieldWriter<'a, REG, 3, Layer0axiburstbits, crate::Safe>;
impl<'a, REG> Layer0axiburstbitsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16-beats (default)"]
    #[inline(always)]
    pub fn layer0_beats_def(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0axiburstbits::Layer0BeatsDef)
    }
    #[doc = "2-beats"]
    #[inline(always)]
    pub fn layer0_beats_2(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0axiburstbits::Layer0Beats2)
    }
    #[doc = "4-beats"]
    #[inline(always)]
    pub fn layer0_beats_4(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0axiburstbits::Layer0Beats4)
    }
    #[doc = "8-beats"]
    #[inline(always)]
    pub fn layer0_beats_8(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0axiburstbits::Layer0Beats8)
    }
    #[doc = "16-beats (CHECK mistake?)"]
    #[inline(always)]
    pub fn layer0_beats_16(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0axiburstbits::Layer0Beats16)
    }
    #[doc = "32-beats (AXI4 only)"]
    #[inline(always)]
    pub fn layer0_beats_32(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0axiburstbits::Layer0Beats32)
    }
    #[doc = "64-beats (AXI4 only)"]
    #[inline(always)]
    pub fn layer0_beats_64(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0axiburstbits::Layer0Beats64)
    }
    #[doc = "128-beats (AXI4 only)"]
    #[inline(always)]
    pub fn layer0_beats_128(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0axiburstbits::Layer0Beats128)
    }
}
#[doc = "Specify the AXI fifo threshold burst start in layer 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Layer0axififothld {
    #[doc = "0: half fifo (default)"]
    Layer0BurstHalfSz = 0,
    #[doc = "1: 2 burst-size"]
    Layer0Burst2 = 1,
    #[doc = "2: 4 burst-size"]
    Layer0Burst4 = 2,
    #[doc = "3: 8 burst-size"]
    Layer0Burst8 = 3,
}
impl From<Layer0axififothld> for u8 {
    #[inline(always)]
    fn from(variant: Layer0axififothld) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Layer0axififothld {
    type Ux = u8;
}
impl crate::IsEnum for Layer0axififothld {}
#[doc = "Field `LAYER0AXIFIFOTHLD` reader - Specify the AXI fifo threshold burst start in layer 0"]
pub type Layer0axififothldR = crate::FieldReader<Layer0axififothld>;
impl Layer0axififothldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Layer0axififothld {
        match self.bits {
            0 => Layer0axififothld::Layer0BurstHalfSz,
            1 => Layer0axififothld::Layer0Burst2,
            2 => Layer0axififothld::Layer0Burst4,
            3 => Layer0axififothld::Layer0Burst8,
            _ => unreachable!(),
        }
    }
    #[doc = "half fifo (default)"]
    #[inline(always)]
    pub fn is_layer0_burst_half_sz(&self) -> bool {
        *self == Layer0axififothld::Layer0BurstHalfSz
    }
    #[doc = "2 burst-size"]
    #[inline(always)]
    pub fn is_layer0_burst_2(&self) -> bool {
        *self == Layer0axififothld::Layer0Burst2
    }
    #[doc = "4 burst-size"]
    #[inline(always)]
    pub fn is_layer0_burst_4(&self) -> bool {
        *self == Layer0axififothld::Layer0Burst4
    }
    #[doc = "8 burst-size"]
    #[inline(always)]
    pub fn is_layer0_burst_8(&self) -> bool {
        *self == Layer0axififothld::Layer0Burst8
    }
}
#[doc = "Field `LAYER0AXIFIFOTHLD` writer - Specify the AXI fifo threshold burst start in layer 0"]
pub type Layer0axififothldW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Layer0axififothld, crate::Safe>;
impl<'a, REG> Layer0axififothldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "half fifo (default)"]
    #[inline(always)]
    pub fn layer0_burst_half_sz(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0axififothld::Layer0BurstHalfSz)
    }
    #[doc = "2 burst-size"]
    #[inline(always)]
    pub fn layer0_burst_2(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0axififothld::Layer0Burst2)
    }
    #[doc = "4 burst-size"]
    #[inline(always)]
    pub fn layer0_burst_4(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0axififothld::Layer0Burst4)
    }
    #[doc = "8 burst-size"]
    #[inline(always)]
    pub fn layer0_burst_8(self) -> &'a mut crate::W<REG> {
        self.variant(Layer0axififothld::Layer0Burst8)
    }
}
#[doc = "Field `RSVD` reader - This field is reserved."]
pub type RsvdR = crate::FieldReader<u16>;
#[doc = "Field `RSVD` writer - This field is reserved."]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:15 - Specify the stride, which is the distance from line to line in bytes for each layer 0 memory"]
    #[inline(always)]
    pub fn layer0stridedist(&self) -> Layer0stridedistR {
        Layer0stridedistR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Specify the AXI bits per burst in layer 0"]
    #[inline(always)]
    pub fn layer0axiburstbits(&self) -> Layer0axiburstbitsR {
        Layer0axiburstbitsR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:20 - Specify the AXI fifo threshold burst start in layer 0"]
    #[inline(always)]
    pub fn layer0axififothld(&self) -> Layer0axififothldR {
        Layer0axififothldR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:31 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specify the stride, which is the distance from line to line in bytes for each layer 0 memory"]
    #[inline(always)]
    #[must_use]
    pub fn layer0stridedist(&mut self) -> Layer0stridedistW<Layer0strideSpec> {
        Layer0stridedistW::new(self, 0)
    }
    #[doc = "Bits 16:18 - Specify the AXI bits per burst in layer 0"]
    #[inline(always)]
    #[must_use]
    pub fn layer0axiburstbits(&mut self) -> Layer0axiburstbitsW<Layer0strideSpec> {
        Layer0axiburstbitsW::new(self, 16)
    }
    #[doc = "Bits 19:20 - Specify the AXI fifo threshold burst start in layer 0"]
    #[inline(always)]
    #[must_use]
    pub fn layer0axififothld(&mut self) -> Layer0axififothldW<Layer0strideSpec> {
        Layer0axififothldW::new(self, 19)
    }
    #[doc = "Bits 21:31 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd(&mut self) -> RsvdW<Layer0strideSpec> {
        RsvdW::new(self, 21)
    }
}
#[doc = "Specify the stride and the AXI bus burst of layer 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer0stride::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0stride::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer0strideSpec;
impl crate::RegisterSpec for Layer0strideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer0stride::R`](R) reader structure"]
impl crate::Readable for Layer0strideSpec {}
#[doc = "`write(|w| ..)` method takes [`layer0stride::W`](W) writer structure"]
impl crate::Writable for Layer0strideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER0STRIDE to value 0"]
impl crate::Resettable for Layer0strideSpec {
    const RESET_VALUE: u32 = 0;
}
