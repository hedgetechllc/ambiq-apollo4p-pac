#[doc = "Register `LAYER3STRIDE` reader"]
pub type R = crate::R<Layer3strideSpec>;
#[doc = "Register `LAYER3STRIDE` writer"]
pub type W = crate::W<Layer3strideSpec>;
#[doc = "Field `LAYER3STRIDEDIST` reader - Specify the stride, which is the distance from line to line in bytes for each layer 3 memory"]
pub type Layer3stridedistR = crate::FieldReader<u16>;
#[doc = "Field `LAYER3STRIDEDIST` writer - Specify the stride, which is the distance from line to line in bytes for each layer 3 memory"]
pub type Layer3stridedistW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Specify the AXI bits per burst in layer 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Layer3axiburstbits {
    #[doc = "0: 16-beats (default)"]
    Layer3BeatsDef = 0,
    #[doc = "1: 2-beats"]
    Layer3Beats2 = 1,
    #[doc = "2: 4-beats"]
    Layer3Beats4 = 2,
    #[doc = "3: 8-beats"]
    Layer3Beats8 = 3,
    #[doc = "4: 16-beats (CHECK mistake?)"]
    Layer3Beats16 = 4,
    #[doc = "5: 32-beats (AXI4 only)"]
    Layer3Beats32 = 5,
    #[doc = "6: 64-beats (AXI4 only)"]
    Layer3Beats64 = 6,
    #[doc = "7: 128-beats (AXI4 only)"]
    Layer3Beats128 = 7,
}
impl From<Layer3axiburstbits> for u8 {
    #[inline(always)]
    fn from(variant: Layer3axiburstbits) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Layer3axiburstbits {
    type Ux = u8;
}
impl crate::IsEnum for Layer3axiburstbits {}
#[doc = "Field `LAYER3AXIBURSTBITS` reader - Specify the AXI bits per burst in layer 3"]
pub type Layer3axiburstbitsR = crate::FieldReader<Layer3axiburstbits>;
impl Layer3axiburstbitsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Layer3axiburstbits {
        match self.bits {
            0 => Layer3axiburstbits::Layer3BeatsDef,
            1 => Layer3axiburstbits::Layer3Beats2,
            2 => Layer3axiburstbits::Layer3Beats4,
            3 => Layer3axiburstbits::Layer3Beats8,
            4 => Layer3axiburstbits::Layer3Beats16,
            5 => Layer3axiburstbits::Layer3Beats32,
            6 => Layer3axiburstbits::Layer3Beats64,
            7 => Layer3axiburstbits::Layer3Beats128,
            _ => unreachable!(),
        }
    }
    #[doc = "16-beats (default)"]
    #[inline(always)]
    pub fn is_layer3_beats_def(&self) -> bool {
        *self == Layer3axiburstbits::Layer3BeatsDef
    }
    #[doc = "2-beats"]
    #[inline(always)]
    pub fn is_layer3_beats_2(&self) -> bool {
        *self == Layer3axiburstbits::Layer3Beats2
    }
    #[doc = "4-beats"]
    #[inline(always)]
    pub fn is_layer3_beats_4(&self) -> bool {
        *self == Layer3axiburstbits::Layer3Beats4
    }
    #[doc = "8-beats"]
    #[inline(always)]
    pub fn is_layer3_beats_8(&self) -> bool {
        *self == Layer3axiburstbits::Layer3Beats8
    }
    #[doc = "16-beats (CHECK mistake?)"]
    #[inline(always)]
    pub fn is_layer3_beats_16(&self) -> bool {
        *self == Layer3axiburstbits::Layer3Beats16
    }
    #[doc = "32-beats (AXI4 only)"]
    #[inline(always)]
    pub fn is_layer3_beats_32(&self) -> bool {
        *self == Layer3axiburstbits::Layer3Beats32
    }
    #[doc = "64-beats (AXI4 only)"]
    #[inline(always)]
    pub fn is_layer3_beats_64(&self) -> bool {
        *self == Layer3axiburstbits::Layer3Beats64
    }
    #[doc = "128-beats (AXI4 only)"]
    #[inline(always)]
    pub fn is_layer3_beats_128(&self) -> bool {
        *self == Layer3axiburstbits::Layer3Beats128
    }
}
#[doc = "Field `LAYER3AXIBURSTBITS` writer - Specify the AXI bits per burst in layer 3"]
pub type Layer3axiburstbitsW<'a, REG> =
    crate::FieldWriter<'a, REG, 3, Layer3axiburstbits, crate::Safe>;
impl<'a, REG> Layer3axiburstbitsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16-beats (default)"]
    #[inline(always)]
    pub fn layer3_beats_def(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3axiburstbits::Layer3BeatsDef)
    }
    #[doc = "2-beats"]
    #[inline(always)]
    pub fn layer3_beats_2(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3axiburstbits::Layer3Beats2)
    }
    #[doc = "4-beats"]
    #[inline(always)]
    pub fn layer3_beats_4(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3axiburstbits::Layer3Beats4)
    }
    #[doc = "8-beats"]
    #[inline(always)]
    pub fn layer3_beats_8(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3axiburstbits::Layer3Beats8)
    }
    #[doc = "16-beats (CHECK mistake?)"]
    #[inline(always)]
    pub fn layer3_beats_16(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3axiburstbits::Layer3Beats16)
    }
    #[doc = "32-beats (AXI4 only)"]
    #[inline(always)]
    pub fn layer3_beats_32(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3axiburstbits::Layer3Beats32)
    }
    #[doc = "64-beats (AXI4 only)"]
    #[inline(always)]
    pub fn layer3_beats_64(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3axiburstbits::Layer3Beats64)
    }
    #[doc = "128-beats (AXI4 only)"]
    #[inline(always)]
    pub fn layer3_beats_128(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3axiburstbits::Layer3Beats128)
    }
}
#[doc = "Specify the AXI fifo threshold burst start in layer 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Layer3axififothld {
    #[doc = "0: half fifo (default)"]
    Layer3BurstHalfSz = 0,
    #[doc = "1: 2 burst-size"]
    Layer3Burst2 = 1,
    #[doc = "2: 4 burst-size"]
    Layer3Burst4 = 2,
    #[doc = "3: 8 burst-size"]
    Layer3Burst8 = 3,
}
impl From<Layer3axififothld> for u8 {
    #[inline(always)]
    fn from(variant: Layer3axififothld) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Layer3axififothld {
    type Ux = u8;
}
impl crate::IsEnum for Layer3axififothld {}
#[doc = "Field `LAYER3AXIFIFOTHLD` reader - Specify the AXI fifo threshold burst start in layer 3"]
pub type Layer3axififothldR = crate::FieldReader<Layer3axififothld>;
impl Layer3axififothldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Layer3axififothld {
        match self.bits {
            0 => Layer3axififothld::Layer3BurstHalfSz,
            1 => Layer3axififothld::Layer3Burst2,
            2 => Layer3axififothld::Layer3Burst4,
            3 => Layer3axififothld::Layer3Burst8,
            _ => unreachable!(),
        }
    }
    #[doc = "half fifo (default)"]
    #[inline(always)]
    pub fn is_layer3_burst_half_sz(&self) -> bool {
        *self == Layer3axififothld::Layer3BurstHalfSz
    }
    #[doc = "2 burst-size"]
    #[inline(always)]
    pub fn is_layer3_burst_2(&self) -> bool {
        *self == Layer3axififothld::Layer3Burst2
    }
    #[doc = "4 burst-size"]
    #[inline(always)]
    pub fn is_layer3_burst_4(&self) -> bool {
        *self == Layer3axififothld::Layer3Burst4
    }
    #[doc = "8 burst-size"]
    #[inline(always)]
    pub fn is_layer3_burst_8(&self) -> bool {
        *self == Layer3axififothld::Layer3Burst8
    }
}
#[doc = "Field `LAYER3AXIFIFOTHLD` writer - Specify the AXI fifo threshold burst start in layer 3"]
pub type Layer3axififothldW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Layer3axififothld, crate::Safe>;
impl<'a, REG> Layer3axififothldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "half fifo (default)"]
    #[inline(always)]
    pub fn layer3_burst_half_sz(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3axififothld::Layer3BurstHalfSz)
    }
    #[doc = "2 burst-size"]
    #[inline(always)]
    pub fn layer3_burst_2(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3axififothld::Layer3Burst2)
    }
    #[doc = "4 burst-size"]
    #[inline(always)]
    pub fn layer3_burst_4(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3axififothld::Layer3Burst4)
    }
    #[doc = "8 burst-size"]
    #[inline(always)]
    pub fn layer3_burst_8(self) -> &'a mut crate::W<REG> {
        self.variant(Layer3axififothld::Layer3Burst8)
    }
}
#[doc = "Field `RSVD` reader - This field is reserved."]
pub type RsvdR = crate::FieldReader<u16>;
#[doc = "Field `RSVD` writer - This field is reserved."]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:15 - Specify the stride, which is the distance from line to line in bytes for each layer 3 memory"]
    #[inline(always)]
    pub fn layer3stridedist(&self) -> Layer3stridedistR {
        Layer3stridedistR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Specify the AXI bits per burst in layer 3"]
    #[inline(always)]
    pub fn layer3axiburstbits(&self) -> Layer3axiburstbitsR {
        Layer3axiburstbitsR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:20 - Specify the AXI fifo threshold burst start in layer 3"]
    #[inline(always)]
    pub fn layer3axififothld(&self) -> Layer3axififothldR {
        Layer3axififothldR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:31 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specify the stride, which is the distance from line to line in bytes for each layer 3 memory"]
    #[inline(always)]
    #[must_use]
    pub fn layer3stridedist(&mut self) -> Layer3stridedistW<Layer3strideSpec> {
        Layer3stridedistW::new(self, 0)
    }
    #[doc = "Bits 16:18 - Specify the AXI bits per burst in layer 3"]
    #[inline(always)]
    #[must_use]
    pub fn layer3axiburstbits(&mut self) -> Layer3axiburstbitsW<Layer3strideSpec> {
        Layer3axiburstbitsW::new(self, 16)
    }
    #[doc = "Bits 19:20 - Specify the AXI fifo threshold burst start in layer 3"]
    #[inline(always)]
    #[must_use]
    pub fn layer3axififothld(&mut self) -> Layer3axififothldW<Layer3strideSpec> {
        Layer3axififothldW::new(self, 19)
    }
    #[doc = "Bits 21:31 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd(&mut self) -> RsvdW<Layer3strideSpec> {
        RsvdW::new(self, 21)
    }
}
#[doc = "Specify the stride and the AXI bus burst of layer 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer3stride::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer3stride::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer3strideSpec;
impl crate::RegisterSpec for Layer3strideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer3stride::R`](R) reader structure"]
impl crate::Readable for Layer3strideSpec {}
#[doc = "`write(|w| ..)` method takes [`layer3stride::W`](W) writer structure"]
impl crate::Writable for Layer3strideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER3STRIDE to value 0"]
impl crate::Resettable for Layer3strideSpec {
    const RESET_VALUE: u32 = 0;
}
