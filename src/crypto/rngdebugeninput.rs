#[doc = "Register `RNGDEBUGENINPUT` reader"]
pub type R = crate::R<RngdebugeninputSpec>;
#[doc = "Register `RNGDEBUGENINPUT` writer"]
pub type W = crate::W<RngdebugeninputSpec>;
#[doc = "Field `RNGDEBUGEN` reader - Reflects the rng_debug_enable input port"]
pub type RngdebugenR = crate::BitReader;
#[doc = "Field `RNGDEBUGEN` writer - Reflects the rng_debug_enable input port"]
pub type RngdebugenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reflects the rng_debug_enable input port"]
    #[inline(always)]
    pub fn rngdebugen(&self) -> RngdebugenR {
        RngdebugenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reflects the rng_debug_enable input port"]
    #[inline(always)]
    #[must_use]
    pub fn rngdebugen(&mut self) -> RngdebugenW<RngdebugeninputSpec> {
        RngdebugenW::new(self, 0)
    }
}
#[doc = "Defines the RNG in debug mode\n\nYou can [`read`](crate::Reg::read) this register and get [`rngdebugeninput::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngdebugeninput::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngdebugeninputSpec;
impl crate::RegisterSpec for RngdebugeninputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngdebugeninput::R`](R) reader structure"]
impl crate::Readable for RngdebugeninputSpec {}
#[doc = "`write(|w| ..)` method takes [`rngdebugeninput::W`](W) writer structure"]
impl crate::Writable for RngdebugeninputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNGDEBUGENINPUT to value 0"]
impl crate::Resettable for RngdebugeninputSpec {
    const RESET_VALUE: u32 = 0;
}
