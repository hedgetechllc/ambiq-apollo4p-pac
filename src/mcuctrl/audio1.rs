#[doc = "Register `AUDIO1` reader"]
pub type R = crate::R<Audio1Spec>;
#[doc = "Register `AUDIO1` writer"]
pub type W = crate::W<Audio1Spec>;
#[doc = "Field `MICBIASVOLTAGETRIM` reader - Output voltage trim"]
pub type MicbiasvoltagetrimR = crate::FieldReader;
#[doc = "Field `MICBIASVOLTAGETRIM` writer - Output voltage trim"]
pub type MicbiasvoltagetrimW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MICBIASPDNB` reader - Power down control for the block"]
pub type MicbiaspdnbR = crate::BitReader;
#[doc = "Field `MICBIASPDNB` writer - Power down control for the block"]
pub type MicbiaspdnbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 6:11 - Output voltage trim"]
    #[inline(always)]
    pub fn micbiasvoltagetrim(&self) -> MicbiasvoltagetrimR {
        MicbiasvoltagetrimR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12 - Power down control for the block"]
    #[inline(always)]
    pub fn micbiaspdnb(&self) -> MicbiaspdnbR {
        MicbiaspdnbR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 6:11 - Output voltage trim"]
    #[inline(always)]
    #[must_use]
    pub fn micbiasvoltagetrim(&mut self) -> MicbiasvoltagetrimW<Audio1Spec> {
        MicbiasvoltagetrimW::new(self, 6)
    }
    #[doc = "Bit 12 - Power down control for the block"]
    #[inline(always)]
    #[must_use]
    pub fn micbiaspdnb(&mut self) -> MicbiaspdnbW<Audio1Spec> {
        MicbiaspdnbW::new(self, 12)
    }
}
#[doc = "Audio trims 1\n\nYou can [`read`](crate::Reg::read) this register and get [`audio1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Audio1Spec;
impl crate::RegisterSpec for Audio1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio1::R`](R) reader structure"]
impl crate::Readable for Audio1Spec {}
#[doc = "`write(|w| ..)` method takes [`audio1::W`](W) writer structure"]
impl crate::Writable for Audio1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUDIO1 to value 0"]
impl crate::Resettable for Audio1Spec {
    const RESET_VALUE: u32 = 0;
}
