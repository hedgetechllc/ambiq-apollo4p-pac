#[doc = "Register `LAYER1STRIDE` reader"]
pub type R = crate::R<Layer1strideSpec>;
#[doc = "Register `LAYER1STRIDE` writer"]
pub type W = crate::W<Layer1strideSpec>;
#[doc = "Field `LAYER1STRIDEDIST` reader - Specify the stride, which is the distance from line to line in bytes for each layer 1 memory"]
pub type Layer1stridedistR = crate::FieldReader<u16>;
#[doc = "Field `LAYER1STRIDEDIST` writer - Specify the stride, which is the distance from line to line in bytes for each layer 1 memory"]
pub type Layer1stridedistW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Specify the AXI bits per burst in layer 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Layer1axiburstbits {
    #[doc = "0: 16-beats (default)"]
    Layer1BeatsDef = 0,
    #[doc = "1: 2-beats"]
    Layer1Beats2 = 1,
    #[doc = "2: 4-beats"]
    Layer1Beats4 = 2,
    #[doc = "3: 8-beats"]
    Layer1Beats8 = 3,
    #[doc = "4: 16-beats (CHECK mistake?)"]
    Layer1Beats16 = 4,
    #[doc = "5: 32-beats (AXI4 only)"]
    Layer1Beats32 = 5,
    #[doc = "6: 64-beats (AXI4 only)"]
    Layer1Beats64 = 6,
    #[doc = "7: 128-beats (AXI4 only)"]
    Layer1Beats128 = 7,
}
impl From<Layer1axiburstbits> for u8 {
    #[inline(always)]
    fn from(variant: Layer1axiburstbits) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Layer1axiburstbits {
    type Ux = u8;
}
impl crate::IsEnum for Layer1axiburstbits {}
#[doc = "Field `LAYER1AXIBURSTBITS` reader - Specify the AXI bits per burst in layer 1"]
pub type Layer1axiburstbitsR = crate::FieldReader<Layer1axiburstbits>;
impl Layer1axiburstbitsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Layer1axiburstbits {
        match self.bits {
            0 => Layer1axiburstbits::Layer1BeatsDef,
            1 => Layer1axiburstbits::Layer1Beats2,
            2 => Layer1axiburstbits::Layer1Beats4,
            3 => Layer1axiburstbits::Layer1Beats8,
            4 => Layer1axiburstbits::Layer1Beats16,
            5 => Layer1axiburstbits::Layer1Beats32,
            6 => Layer1axiburstbits::Layer1Beats64,
            7 => Layer1axiburstbits::Layer1Beats128,
            _ => unreachable!(),
        }
    }
    #[doc = "16-beats (default)"]
    #[inline(always)]
    pub fn is_layer1_beats_def(&self) -> bool {
        *self == Layer1axiburstbits::Layer1BeatsDef
    }
    #[doc = "2-beats"]
    #[inline(always)]
    pub fn is_layer1_beats_2(&self) -> bool {
        *self == Layer1axiburstbits::Layer1Beats2
    }
    #[doc = "4-beats"]
    #[inline(always)]
    pub fn is_layer1_beats_4(&self) -> bool {
        *self == Layer1axiburstbits::Layer1Beats4
    }
    #[doc = "8-beats"]
    #[inline(always)]
    pub fn is_layer1_beats_8(&self) -> bool {
        *self == Layer1axiburstbits::Layer1Beats8
    }
    #[doc = "16-beats (CHECK mistake?)"]
    #[inline(always)]
    pub fn is_layer1_beats_16(&self) -> bool {
        *self == Layer1axiburstbits::Layer1Beats16
    }
    #[doc = "32-beats (AXI4 only)"]
    #[inline(always)]
    pub fn is_layer1_beats_32(&self) -> bool {
        *self == Layer1axiburstbits::Layer1Beats32
    }
    #[doc = "64-beats (AXI4 only)"]
    #[inline(always)]
    pub fn is_layer1_beats_64(&self) -> bool {
        *self == Layer1axiburstbits::Layer1Beats64
    }
    #[doc = "128-beats (AXI4 only)"]
    #[inline(always)]
    pub fn is_layer1_beats_128(&self) -> bool {
        *self == Layer1axiburstbits::Layer1Beats128
    }
}
#[doc = "Field `LAYER1AXIBURSTBITS` writer - Specify the AXI bits per burst in layer 1"]
pub type Layer1axiburstbitsW<'a, REG> =
    crate::FieldWriter<'a, REG, 3, Layer1axiburstbits, crate::Safe>;
impl<'a, REG> Layer1axiburstbitsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16-beats (default)"]
    #[inline(always)]
    pub fn layer1_beats_def(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1axiburstbits::Layer1BeatsDef)
    }
    #[doc = "2-beats"]
    #[inline(always)]
    pub fn layer1_beats_2(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1axiburstbits::Layer1Beats2)
    }
    #[doc = "4-beats"]
    #[inline(always)]
    pub fn layer1_beats_4(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1axiburstbits::Layer1Beats4)
    }
    #[doc = "8-beats"]
    #[inline(always)]
    pub fn layer1_beats_8(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1axiburstbits::Layer1Beats8)
    }
    #[doc = "16-beats (CHECK mistake?)"]
    #[inline(always)]
    pub fn layer1_beats_16(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1axiburstbits::Layer1Beats16)
    }
    #[doc = "32-beats (AXI4 only)"]
    #[inline(always)]
    pub fn layer1_beats_32(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1axiburstbits::Layer1Beats32)
    }
    #[doc = "64-beats (AXI4 only)"]
    #[inline(always)]
    pub fn layer1_beats_64(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1axiburstbits::Layer1Beats64)
    }
    #[doc = "128-beats (AXI4 only)"]
    #[inline(always)]
    pub fn layer1_beats_128(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1axiburstbits::Layer1Beats128)
    }
}
#[doc = "Specify the AXI fifo threshold burst start in layer 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Layer1axififothld {
    #[doc = "0: half fifo (default)"]
    Layer1BurstHalfSz = 0,
    #[doc = "1: 2 burst-size"]
    Layer1Burst2 = 1,
    #[doc = "2: 4 burst-size"]
    Layer1Burst4 = 2,
    #[doc = "3: 8 burst-size"]
    Layer1Burst8 = 3,
}
impl From<Layer1axififothld> for u8 {
    #[inline(always)]
    fn from(variant: Layer1axififothld) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Layer1axififothld {
    type Ux = u8;
}
impl crate::IsEnum for Layer1axififothld {}
#[doc = "Field `LAYER1AXIFIFOTHLD` reader - Specify the AXI fifo threshold burst start in layer 1"]
pub type Layer1axififothldR = crate::FieldReader<Layer1axififothld>;
impl Layer1axififothldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Layer1axififothld {
        match self.bits {
            0 => Layer1axififothld::Layer1BurstHalfSz,
            1 => Layer1axififothld::Layer1Burst2,
            2 => Layer1axififothld::Layer1Burst4,
            3 => Layer1axififothld::Layer1Burst8,
            _ => unreachable!(),
        }
    }
    #[doc = "half fifo (default)"]
    #[inline(always)]
    pub fn is_layer1_burst_half_sz(&self) -> bool {
        *self == Layer1axififothld::Layer1BurstHalfSz
    }
    #[doc = "2 burst-size"]
    #[inline(always)]
    pub fn is_layer1_burst_2(&self) -> bool {
        *self == Layer1axififothld::Layer1Burst2
    }
    #[doc = "4 burst-size"]
    #[inline(always)]
    pub fn is_layer1_burst_4(&self) -> bool {
        *self == Layer1axififothld::Layer1Burst4
    }
    #[doc = "8 burst-size"]
    #[inline(always)]
    pub fn is_layer1_burst_8(&self) -> bool {
        *self == Layer1axififothld::Layer1Burst8
    }
}
#[doc = "Field `LAYER1AXIFIFOTHLD` writer - Specify the AXI fifo threshold burst start in layer 1"]
pub type Layer1axififothldW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Layer1axififothld, crate::Safe>;
impl<'a, REG> Layer1axififothldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "half fifo (default)"]
    #[inline(always)]
    pub fn layer1_burst_half_sz(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1axififothld::Layer1BurstHalfSz)
    }
    #[doc = "2 burst-size"]
    #[inline(always)]
    pub fn layer1_burst_2(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1axififothld::Layer1Burst2)
    }
    #[doc = "4 burst-size"]
    #[inline(always)]
    pub fn layer1_burst_4(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1axififothld::Layer1Burst4)
    }
    #[doc = "8 burst-size"]
    #[inline(always)]
    pub fn layer1_burst_8(self) -> &'a mut crate::W<REG> {
        self.variant(Layer1axififothld::Layer1Burst8)
    }
}
#[doc = "Field `RSVD` reader - This field is reserved."]
pub type RsvdR = crate::FieldReader<u16>;
#[doc = "Field `RSVD` writer - This field is reserved."]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:15 - Specify the stride, which is the distance from line to line in bytes for each layer 1 memory"]
    #[inline(always)]
    pub fn layer1stridedist(&self) -> Layer1stridedistR {
        Layer1stridedistR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Specify the AXI bits per burst in layer 1"]
    #[inline(always)]
    pub fn layer1axiburstbits(&self) -> Layer1axiburstbitsR {
        Layer1axiburstbitsR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:20 - Specify the AXI fifo threshold burst start in layer 1"]
    #[inline(always)]
    pub fn layer1axififothld(&self) -> Layer1axififothldR {
        Layer1axififothldR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:31 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specify the stride, which is the distance from line to line in bytes for each layer 1 memory"]
    #[inline(always)]
    #[must_use]
    pub fn layer1stridedist(&mut self) -> Layer1stridedistW<Layer1strideSpec> {
        Layer1stridedistW::new(self, 0)
    }
    #[doc = "Bits 16:18 - Specify the AXI bits per burst in layer 1"]
    #[inline(always)]
    #[must_use]
    pub fn layer1axiburstbits(&mut self) -> Layer1axiburstbitsW<Layer1strideSpec> {
        Layer1axiburstbitsW::new(self, 16)
    }
    #[doc = "Bits 19:20 - Specify the AXI fifo threshold burst start in layer 1"]
    #[inline(always)]
    #[must_use]
    pub fn layer1axififothld(&mut self) -> Layer1axififothldW<Layer1strideSpec> {
        Layer1axififothldW::new(self, 19)
    }
    #[doc = "Bits 21:31 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd(&mut self) -> RsvdW<Layer1strideSpec> {
        RsvdW::new(self, 21)
    }
}
#[doc = "Specify the stride and the AXI bus burst of layer 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer1stride::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer1stride::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer1strideSpec;
impl crate::RegisterSpec for Layer1strideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer1stride::R`](R) reader structure"]
impl crate::Readable for Layer1strideSpec {}
#[doc = "`write(|w| ..)` method takes [`layer1stride::W`](W) writer structure"]
impl crate::Writable for Layer1strideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER1STRIDE to value 0"]
impl crate::Resettable for Layer1strideSpec {
    const RESET_VALUE: u32 = 0;
}
