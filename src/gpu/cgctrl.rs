#[doc = "Register `CGCTRL` reader"]
pub type R = crate::R<CgctrlSpec>;
#[doc = "Register `CGCTRL` writer"]
pub type W = crate::W<CgctrlSpec>;
#[doc = "Field `DISCLKPROC` reader - disable clock gating for command list processor"]
pub type DisclkprocR = crate::BitReader;
#[doc = "Field `DISCLKPROC` writer - disable clock gating for command list processor"]
pub type DisclkprocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCLKCFG` reader - disable clock gating for configuration file"]
pub type DisclkcfgR = crate::BitReader;
#[doc = "Field `DISCLKCFG` writer - disable clock gating for configuration file"]
pub type DisclkcfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCLKFRAME` reader - disable clock gating for framebuffer 0 (MISTAKE ?)"]
pub type DisclkframeR = crate::FieldReader;
#[doc = "Field `DISCLKFRAME` writer - disable clock gating for framebuffer 0 (MISTAKE ?)"]
pub type DisclkframeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RSVD0` reader - This bitfield is reserved."]
pub type Rsvd0R = crate::FieldReader<u32>;
#[doc = "Field `RSVD0` writer - This bitfield is reserved."]
pub type Rsvd0W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `DISCLKCORE` reader - disable clock gating for core 0"]
pub type DisclkcoreR = crate::BitReader;
#[doc = "Field `DISCLKCORE` writer - disable clock gating for core 0"]
pub type DisclkcoreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD1` reader - This bitfield is reserved."]
pub type Rsvd1R = crate::FieldReader;
#[doc = "Field `RSVD1` writer - This bitfield is reserved."]
pub type Rsvd1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DISCLKMOD` reader - disable clock gating for all modules (MISTAKE ?)"]
pub type DisclkmodR = crate::FieldReader;
#[doc = "Field `DISCLKMOD` writer - disable clock gating for all modules (MISTAKE ?)"]
pub type DisclkmodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - disable clock gating for command list processor"]
    #[inline(always)]
    pub fn disclkproc(&self) -> DisclkprocR {
        DisclkprocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - disable clock gating for configuration file"]
    #[inline(always)]
    pub fn disclkcfg(&self) -> DisclkcfgR {
        DisclkcfgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - disable clock gating for framebuffer 0 (MISTAKE ?)"]
    #[inline(always)]
    pub fn disclkframe(&self) -> DisclkframeR {
        DisclkframeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:22 - This bitfield is reserved."]
    #[inline(always)]
    pub fn rsvd0(&self) -> Rsvd0R {
        Rsvd0R::new((self.bits >> 4) & 0x0007_ffff)
    }
    #[doc = "Bit 23 - disable clock gating for core 0"]
    #[inline(always)]
    pub fn disclkcore(&self) -> DisclkcoreR {
        DisclkcoreR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:29 - This bitfield is reserved."]
    #[inline(always)]
    pub fn rsvd1(&self) -> Rsvd1R {
        Rsvd1R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 30:31 - disable clock gating for all modules (MISTAKE ?)"]
    #[inline(always)]
    pub fn disclkmod(&self) -> DisclkmodR {
        DisclkmodR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - disable clock gating for command list processor"]
    #[inline(always)]
    #[must_use]
    pub fn disclkproc(&mut self) -> DisclkprocW<CgctrlSpec> {
        DisclkprocW::new(self, 0)
    }
    #[doc = "Bit 1 - disable clock gating for configuration file"]
    #[inline(always)]
    #[must_use]
    pub fn disclkcfg(&mut self) -> DisclkcfgW<CgctrlSpec> {
        DisclkcfgW::new(self, 1)
    }
    #[doc = "Bits 2:3 - disable clock gating for framebuffer 0 (MISTAKE ?)"]
    #[inline(always)]
    #[must_use]
    pub fn disclkframe(&mut self) -> DisclkframeW<CgctrlSpec> {
        DisclkframeW::new(self, 2)
    }
    #[doc = "Bits 4:22 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd0(&mut self) -> Rsvd0W<CgctrlSpec> {
        Rsvd0W::new(self, 4)
    }
    #[doc = "Bit 23 - disable clock gating for core 0"]
    #[inline(always)]
    #[must_use]
    pub fn disclkcore(&mut self) -> DisclkcoreW<CgctrlSpec> {
        DisclkcoreW::new(self, 23)
    }
    #[doc = "Bits 24:29 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd1(&mut self) -> Rsvd1W<CgctrlSpec> {
        Rsvd1W::new(self, 24)
    }
    #[doc = "Bits 30:31 - disable clock gating for all modules (MISTAKE ?)"]
    #[inline(always)]
    #[must_use]
    pub fn disclkmod(&mut self) -> DisclkmodW<CgctrlSpec> {
        DisclkmodW::new(self, 30)
    }
}
#[doc = "CGCTRL register description needed here.\n\nYou can [`read`](crate::Reg::read) this register and get [`cgctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CgctrlSpec;
impl crate::RegisterSpec for CgctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgctrl::R`](R) reader structure"]
impl crate::Readable for CgctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cgctrl::W`](W) writer structure"]
impl crate::Writable for CgctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CGCTRL to value 0"]
impl crate::Resettable for CgctrlSpec {
    const RESET_VALUE: u32 = 0;
}
