#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `COREBSY` reader - processing core busy (Cores 3-0)"]
pub type CorebsyR = crate::FieldReader;
#[doc = "Field `COREBSY` writer - processing core busy (Cores 3-0)"]
pub type CorebsyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PIPEBSY` reader - pipeline busy (Cores 3-0)"]
pub type PipebsyR = crate::FieldReader;
#[doc = "Field `PIPEBSY` writer - pipeline busy (Cores 3-0)"]
pub type PipebsyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TEXTMAPBSY` reader - texture map busy (Cores 3-0)"]
pub type TextmapbsyR = crate::FieldReader;
#[doc = "Field `TEXTMAPBSY` writer - texture map busy (Cores 3-0)"]
pub type TextmapbsyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RENDERBSY` reader - render output unit busy (Cores 3-0)"]
pub type RenderbsyR = crate::FieldReader;
#[doc = "Field `RENDERBSY` writer - render output unit busy (Cores 3-0)"]
pub type RenderbsyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DEPTHFIFOBSY` reader - depth buffer busy (Cores 3-0)"]
pub type DepthfifobsyR = crate::FieldReader;
#[doc = "Field `DEPTHFIFOBSY` writer - depth buffer busy (Cores 3-0)"]
pub type DepthfifobsyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RASTBSY` reader - rasterizer busy"]
pub type RastbsyR = crate::FieldReader;
#[doc = "Field `RASTBSY` writer - rasterizer busy"]
pub type RastbsyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLPBSY` reader - command list processor busy"]
pub type ClpbsyR = crate::BitReader;
#[doc = "Field `CLPBSY` writer - command list processor busy"]
pub type ClpbsyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLBSY` reader - command list bus busy"]
pub type ClbsyR = crate::BitReader;
#[doc = "Field `CLBSY` writer - command list bus busy"]
pub type ClbsyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMBSY` reader - memory system busy"]
pub type MembsyR = crate::BitReader;
#[doc = "Field `MEMBSY` writer - memory system busy"]
pub type MembsyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSBSY` reader - system busy"]
pub type SysbsyR = crate::BitReader;
#[doc = "Field `SYSBSY` writer - system busy"]
pub type SysbsyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - processing core busy (Cores 3-0)"]
    #[inline(always)]
    pub fn corebsy(&self) -> CorebsyR {
        CorebsyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - pipeline busy (Cores 3-0)"]
    #[inline(always)]
    pub fn pipebsy(&self) -> PipebsyR {
        PipebsyR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - texture map busy (Cores 3-0)"]
    #[inline(always)]
    pub fn textmapbsy(&self) -> TextmapbsyR {
        TextmapbsyR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - render output unit busy (Cores 3-0)"]
    #[inline(always)]
    pub fn renderbsy(&self) -> RenderbsyR {
        RenderbsyR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - depth buffer busy (Cores 3-0)"]
    #[inline(always)]
    pub fn depthfifobsy(&self) -> DepthfifobsyR {
        DepthfifobsyR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - rasterizer busy"]
    #[inline(always)]
    pub fn rastbsy(&self) -> RastbsyR {
        RastbsyR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - command list processor busy"]
    #[inline(always)]
    pub fn clpbsy(&self) -> ClpbsyR {
        ClpbsyR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - command list bus busy"]
    #[inline(always)]
    pub fn clbsy(&self) -> ClbsyR {
        ClbsyR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - memory system busy"]
    #[inline(always)]
    pub fn membsy(&self) -> MembsyR {
        MembsyR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - system busy"]
    #[inline(always)]
    pub fn sysbsy(&self) -> SysbsyR {
        SysbsyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - processing core busy (Cores 3-0)"]
    #[inline(always)]
    #[must_use]
    pub fn corebsy(&mut self) -> CorebsyW<StatusSpec> {
        CorebsyW::new(self, 0)
    }
    #[doc = "Bits 4:7 - pipeline busy (Cores 3-0)"]
    #[inline(always)]
    #[must_use]
    pub fn pipebsy(&mut self) -> PipebsyW<StatusSpec> {
        PipebsyW::new(self, 4)
    }
    #[doc = "Bits 8:11 - texture map busy (Cores 3-0)"]
    #[inline(always)]
    #[must_use]
    pub fn textmapbsy(&mut self) -> TextmapbsyW<StatusSpec> {
        TextmapbsyW::new(self, 8)
    }
    #[doc = "Bits 12:15 - render output unit busy (Cores 3-0)"]
    #[inline(always)]
    #[must_use]
    pub fn renderbsy(&mut self) -> RenderbsyW<StatusSpec> {
        RenderbsyW::new(self, 12)
    }
    #[doc = "Bits 16:19 - depth buffer busy (Cores 3-0)"]
    #[inline(always)]
    #[must_use]
    pub fn depthfifobsy(&mut self) -> DepthfifobsyW<StatusSpec> {
        DepthfifobsyW::new(self, 16)
    }
    #[doc = "Bits 24:27 - rasterizer busy"]
    #[inline(always)]
    #[must_use]
    pub fn rastbsy(&mut self) -> RastbsyW<StatusSpec> {
        RastbsyW::new(self, 24)
    }
    #[doc = "Bit 28 - command list processor busy"]
    #[inline(always)]
    #[must_use]
    pub fn clpbsy(&mut self) -> ClpbsyW<StatusSpec> {
        ClpbsyW::new(self, 28)
    }
    #[doc = "Bit 29 - command list bus busy"]
    #[inline(always)]
    #[must_use]
    pub fn clbsy(&mut self) -> ClbsyW<StatusSpec> {
        ClbsyW::new(self, 29)
    }
    #[doc = "Bit 30 - memory system busy"]
    #[inline(always)]
    #[must_use]
    pub fn membsy(&mut self) -> MembsyW<StatusSpec> {
        MembsyW::new(self, 30)
    }
    #[doc = "Bit 31 - system busy"]
    #[inline(always)]
    #[must_use]
    pub fn sysbsy(&mut self) -> SysbsyW<StatusSpec> {
        SysbsyW::new(self, 31)
    }
}
#[doc = "On read, returns GPU status (CHECK address!!).\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
