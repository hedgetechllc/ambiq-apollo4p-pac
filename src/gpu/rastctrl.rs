#[doc = "Register `RASTCTRL` reader"]
pub type R = crate::R<RastctrlSpec>;
#[doc = "Register `RASTCTRL` writer"]
pub type W = crate::W<RastctrlSpec>;
#[doc = "Field `RSVD` reader - This bitfield is reserved."]
pub type RsvdR = crate::FieldReader<u32>;
#[doc = "Field `RSVD` writer - This bitfield is reserved."]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `BYPASS` reader - tells module to bypass calculations"]
pub type BypassR = crate::BitReader;
#[doc = "Field `BYPASS` writer - tells module to bypass calculations"]
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD` reader - adds 0.5 to X and Y"]
pub type AddR = crate::BitReader;
#[doc = "Field `ADD` writer - adds 0.5 to X and Y"]
pub type AddW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERSP` reader - when set to 0 is in perspective mode (MISTAKE IN DOC?)"]
pub type PerspR = crate::FieldReader;
#[doc = "Field `PERSP` writer - when set to 0 is in perspective mode (MISTAKE IN DOC?)"]
pub type PerspW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:27 - This bitfield is reserved."]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 28 - tells module to bypass calculations"]
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - adds 0.5 to X and Y"]
    #[inline(always)]
    pub fn add(&self) -> AddR {
        AddR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - when set to 0 is in perspective mode (MISTAKE IN DOC?)"]
    #[inline(always)]
    pub fn persp(&self) -> PerspR {
        PerspR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:27 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd(&mut self) -> RsvdW<RastctrlSpec> {
        RsvdW::new(self, 0)
    }
    #[doc = "Bit 28 - tells module to bypass calculations"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BypassW<RastctrlSpec> {
        BypassW::new(self, 28)
    }
    #[doc = "Bit 29 - adds 0.5 to X and Y"]
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> AddW<RastctrlSpec> {
        AddW::new(self, 29)
    }
    #[doc = "Bits 30:31 - when set to 0 is in perspective mode (MISTAKE IN DOC?)"]
    #[inline(always)]
    #[must_use]
    pub fn persp(&mut self) -> PerspW<RastctrlSpec> {
        PerspW::new(self, 30)
    }
}
#[doc = "Rasterizer matrix multiplication control\n\nYou can [`read`](crate::Reg::read) this register and get [`rastctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rastctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RastctrlSpec;
impl crate::RegisterSpec for RastctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rastctrl::R`](R) reader structure"]
impl crate::Readable for RastctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rastctrl::W`](W) writer structure"]
impl crate::Writable for RastctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RASTCTRL to value 0"]
impl crate::Resettable for RastctrlSpec {
    const RESET_VALUE: u32 = 0;
}
