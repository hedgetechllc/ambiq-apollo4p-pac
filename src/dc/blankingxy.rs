#[doc = "Register `BLANKINGXY` reader"]
pub type R = crate::R<BlankingxySpec>;
#[doc = "Register `BLANKINGXY` writer"]
pub type W = crate::W<BlankingxySpec>;
#[doc = "Field `VSYNCLINES` reader - Specify the VSYNC lines for the Y dimension blanking period"]
pub type VsynclinesR = crate::FieldReader<u16>;
#[doc = "Field `VSYNCLINES` writer - Specify the VSYNC lines for the Y dimension blanking period"]
pub type VsynclinesW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `HSYNCPULSE` reader - Specify the HSYNC pulse length for the X dimension blanking period"]
pub type HsyncpulseR = crate::FieldReader<u16>;
#[doc = "Field `HSYNCPULSE` writer - Specify the HSYNC pulse length for the X dimension blanking period"]
pub type HsyncpulseW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Specify the VSYNC lines for the Y dimension blanking period"]
    #[inline(always)]
    pub fn vsynclines(&self) -> VsynclinesR {
        VsynclinesR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specify the HSYNC pulse length for the X dimension blanking period"]
    #[inline(always)]
    pub fn hsyncpulse(&self) -> HsyncpulseR {
        HsyncpulseR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specify the VSYNC lines for the Y dimension blanking period"]
    #[inline(always)]
    #[must_use]
    pub fn vsynclines(&mut self) -> VsynclinesW<BlankingxySpec> {
        VsynclinesW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Specify the HSYNC pulse length for the X dimension blanking period"]
    #[inline(always)]
    #[must_use]
    pub fn hsyncpulse(&mut self) -> HsyncpulseW<BlankingxySpec> {
        HsyncpulseW::new(self, 16)
    }
}
#[doc = "Specifies the X and Y dimensions for the Blanking Period.\n\nYou can [`read`](crate::Reg::read) this register and get [`blankingxy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blankingxy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlankingxySpec;
impl crate::RegisterSpec for BlankingxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blankingxy::R`](R) reader structure"]
impl crate::Readable for BlankingxySpec {}
#[doc = "`write(|w| ..)` method takes [`blankingxy::W`](W) writer structure"]
impl crate::Writable for BlankingxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLANKINGXY to value 0"]
impl crate::Resettable for BlankingxySpec {
    const RESET_VALUE: u32 = 0;
}
