#[doc = "Register `LAYER2STRIDE` reader"]
pub type R = crate::R<Layer2strideSpec>;
#[doc = "Register `LAYER2STRIDE` writer"]
pub type W = crate::W<Layer2strideSpec>;
#[doc = "Field `LAYER2STRIDEDIST` reader - Specify the stride, which is the distance from line to line in bytes for each layer 2 memory"]
pub type Layer2stridedistR = crate::FieldReader<u16>;
#[doc = "Field `LAYER2STRIDEDIST` writer - Specify the stride, which is the distance from line to line in bytes for each layer 2 memory"]
pub type Layer2stridedistW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Specify the AXI bits per burst in layer 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Layer2axiburstbits {
    #[doc = "0: 16-beats (default)"]
    Layer2BeatsDef = 0,
    #[doc = "1: 2-beats"]
    Layer2Beats2 = 1,
    #[doc = "2: 4-beats"]
    Layer2Beats4 = 2,
    #[doc = "3: 8-beats"]
    Layer2Beats8 = 3,
    #[doc = "4: 16-beats (CHECK mistake?)"]
    Layer2Beats16 = 4,
    #[doc = "5: 32-beats (AXI4 only)"]
    Layer2Beats32 = 5,
    #[doc = "6: 64-beats (AXI4 only)"]
    Layer2Beats64 = 6,
    #[doc = "7: 128-beats (AXI4 only)"]
    Layer2Beats128 = 7,
}
impl From<Layer2axiburstbits> for u8 {
    #[inline(always)]
    fn from(variant: Layer2axiburstbits) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Layer2axiburstbits {
    type Ux = u8;
}
impl crate::IsEnum for Layer2axiburstbits {}
#[doc = "Field `LAYER2AXIBURSTBITS` reader - Specify the AXI bits per burst in layer 2"]
pub type Layer2axiburstbitsR = crate::FieldReader<Layer2axiburstbits>;
impl Layer2axiburstbitsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Layer2axiburstbits {
        match self.bits {
            0 => Layer2axiburstbits::Layer2BeatsDef,
            1 => Layer2axiburstbits::Layer2Beats2,
            2 => Layer2axiburstbits::Layer2Beats4,
            3 => Layer2axiburstbits::Layer2Beats8,
            4 => Layer2axiburstbits::Layer2Beats16,
            5 => Layer2axiburstbits::Layer2Beats32,
            6 => Layer2axiburstbits::Layer2Beats64,
            7 => Layer2axiburstbits::Layer2Beats128,
            _ => unreachable!(),
        }
    }
    #[doc = "16-beats (default)"]
    #[inline(always)]
    pub fn is_layer2_beats_def(&self) -> bool {
        *self == Layer2axiburstbits::Layer2BeatsDef
    }
    #[doc = "2-beats"]
    #[inline(always)]
    pub fn is_layer2_beats_2(&self) -> bool {
        *self == Layer2axiburstbits::Layer2Beats2
    }
    #[doc = "4-beats"]
    #[inline(always)]
    pub fn is_layer2_beats_4(&self) -> bool {
        *self == Layer2axiburstbits::Layer2Beats4
    }
    #[doc = "8-beats"]
    #[inline(always)]
    pub fn is_layer2_beats_8(&self) -> bool {
        *self == Layer2axiburstbits::Layer2Beats8
    }
    #[doc = "16-beats (CHECK mistake?)"]
    #[inline(always)]
    pub fn is_layer2_beats_16(&self) -> bool {
        *self == Layer2axiburstbits::Layer2Beats16
    }
    #[doc = "32-beats (AXI4 only)"]
    #[inline(always)]
    pub fn is_layer2_beats_32(&self) -> bool {
        *self == Layer2axiburstbits::Layer2Beats32
    }
    #[doc = "64-beats (AXI4 only)"]
    #[inline(always)]
    pub fn is_layer2_beats_64(&self) -> bool {
        *self == Layer2axiburstbits::Layer2Beats64
    }
    #[doc = "128-beats (AXI4 only)"]
    #[inline(always)]
    pub fn is_layer2_beats_128(&self) -> bool {
        *self == Layer2axiburstbits::Layer2Beats128
    }
}
#[doc = "Field `LAYER2AXIBURSTBITS` writer - Specify the AXI bits per burst in layer 2"]
pub type Layer2axiburstbitsW<'a, REG> =
    crate::FieldWriter<'a, REG, 3, Layer2axiburstbits, crate::Safe>;
impl<'a, REG> Layer2axiburstbitsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16-beats (default)"]
    #[inline(always)]
    pub fn layer2_beats_def(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2axiburstbits::Layer2BeatsDef)
    }
    #[doc = "2-beats"]
    #[inline(always)]
    pub fn layer2_beats_2(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2axiburstbits::Layer2Beats2)
    }
    #[doc = "4-beats"]
    #[inline(always)]
    pub fn layer2_beats_4(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2axiburstbits::Layer2Beats4)
    }
    #[doc = "8-beats"]
    #[inline(always)]
    pub fn layer2_beats_8(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2axiburstbits::Layer2Beats8)
    }
    #[doc = "16-beats (CHECK mistake?)"]
    #[inline(always)]
    pub fn layer2_beats_16(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2axiburstbits::Layer2Beats16)
    }
    #[doc = "32-beats (AXI4 only)"]
    #[inline(always)]
    pub fn layer2_beats_32(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2axiburstbits::Layer2Beats32)
    }
    #[doc = "64-beats (AXI4 only)"]
    #[inline(always)]
    pub fn layer2_beats_64(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2axiburstbits::Layer2Beats64)
    }
    #[doc = "128-beats (AXI4 only)"]
    #[inline(always)]
    pub fn layer2_beats_128(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2axiburstbits::Layer2Beats128)
    }
}
#[doc = "Specify the AXI fifo threshold burst start in layer 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Layer2axififothld {
    #[doc = "0: half fifo (default)"]
    Layer2BurstHalfSz = 0,
    #[doc = "1: 2 burst-size"]
    Layer2Burst2 = 1,
    #[doc = "2: 4 burst-size"]
    Layer2Burst4 = 2,
    #[doc = "3: 8 burst-size"]
    Layer2Burst8 = 3,
}
impl From<Layer2axififothld> for u8 {
    #[inline(always)]
    fn from(variant: Layer2axififothld) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Layer2axififothld {
    type Ux = u8;
}
impl crate::IsEnum for Layer2axififothld {}
#[doc = "Field `LAYER2AXIFIFOTHLD` reader - Specify the AXI fifo threshold burst start in layer 2"]
pub type Layer2axififothldR = crate::FieldReader<Layer2axififothld>;
impl Layer2axififothldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Layer2axififothld {
        match self.bits {
            0 => Layer2axififothld::Layer2BurstHalfSz,
            1 => Layer2axififothld::Layer2Burst2,
            2 => Layer2axififothld::Layer2Burst4,
            3 => Layer2axififothld::Layer2Burst8,
            _ => unreachable!(),
        }
    }
    #[doc = "half fifo (default)"]
    #[inline(always)]
    pub fn is_layer2_burst_half_sz(&self) -> bool {
        *self == Layer2axififothld::Layer2BurstHalfSz
    }
    #[doc = "2 burst-size"]
    #[inline(always)]
    pub fn is_layer2_burst_2(&self) -> bool {
        *self == Layer2axififothld::Layer2Burst2
    }
    #[doc = "4 burst-size"]
    #[inline(always)]
    pub fn is_layer2_burst_4(&self) -> bool {
        *self == Layer2axififothld::Layer2Burst4
    }
    #[doc = "8 burst-size"]
    #[inline(always)]
    pub fn is_layer2_burst_8(&self) -> bool {
        *self == Layer2axififothld::Layer2Burst8
    }
}
#[doc = "Field `LAYER2AXIFIFOTHLD` writer - Specify the AXI fifo threshold burst start in layer 2"]
pub type Layer2axififothldW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Layer2axififothld, crate::Safe>;
impl<'a, REG> Layer2axififothldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "half fifo (default)"]
    #[inline(always)]
    pub fn layer2_burst_half_sz(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2axififothld::Layer2BurstHalfSz)
    }
    #[doc = "2 burst-size"]
    #[inline(always)]
    pub fn layer2_burst_2(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2axififothld::Layer2Burst2)
    }
    #[doc = "4 burst-size"]
    #[inline(always)]
    pub fn layer2_burst_4(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2axififothld::Layer2Burst4)
    }
    #[doc = "8 burst-size"]
    #[inline(always)]
    pub fn layer2_burst_8(self) -> &'a mut crate::W<REG> {
        self.variant(Layer2axififothld::Layer2Burst8)
    }
}
#[doc = "Field `RSVD` reader - This field is reserved."]
pub type RsvdR = crate::FieldReader<u16>;
#[doc = "Field `RSVD` writer - This field is reserved."]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:15 - Specify the stride, which is the distance from line to line in bytes for each layer 2 memory"]
    #[inline(always)]
    pub fn layer2stridedist(&self) -> Layer2stridedistR {
        Layer2stridedistR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Specify the AXI bits per burst in layer 2"]
    #[inline(always)]
    pub fn layer2axiburstbits(&self) -> Layer2axiburstbitsR {
        Layer2axiburstbitsR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:20 - Specify the AXI fifo threshold burst start in layer 2"]
    #[inline(always)]
    pub fn layer2axififothld(&self) -> Layer2axififothldR {
        Layer2axififothldR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:31 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specify the stride, which is the distance from line to line in bytes for each layer 2 memory"]
    #[inline(always)]
    #[must_use]
    pub fn layer2stridedist(&mut self) -> Layer2stridedistW<Layer2strideSpec> {
        Layer2stridedistW::new(self, 0)
    }
    #[doc = "Bits 16:18 - Specify the AXI bits per burst in layer 2"]
    #[inline(always)]
    #[must_use]
    pub fn layer2axiburstbits(&mut self) -> Layer2axiburstbitsW<Layer2strideSpec> {
        Layer2axiburstbitsW::new(self, 16)
    }
    #[doc = "Bits 19:20 - Specify the AXI fifo threshold burst start in layer 2"]
    #[inline(always)]
    #[must_use]
    pub fn layer2axififothld(&mut self) -> Layer2axififothldW<Layer2strideSpec> {
        Layer2axififothldW::new(self, 19)
    }
    #[doc = "Bits 21:31 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd(&mut self) -> RsvdW<Layer2strideSpec> {
        RsvdW::new(self, 21)
    }
}
#[doc = "Specify the stride and the AXI bus burst of layer 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer2stride::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer2stride::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer2strideSpec;
impl crate::RegisterSpec for Layer2strideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer2stride::R`](R) reader structure"]
impl crate::Readable for Layer2strideSpec {}
#[doc = "`write(|w| ..)` method takes [`layer2stride::W`](W) writer structure"]
impl crate::Writable for Layer2strideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER2STRIDE to value 0"]
impl crate::Resettable for Layer2strideSpec {
    const RESET_VALUE: u32 = 0;
}
