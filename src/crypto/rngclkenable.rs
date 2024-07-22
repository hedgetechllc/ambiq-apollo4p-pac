#[doc = "Register `RNGCLKENABLE` reader"]
pub type R = crate::R<RngclkenableSpec>;
#[doc = "Register `RNGCLKENABLE` writer"]
pub type W = crate::W<RngclkenableSpec>;
#[doc = "Field `EN` reader - Writing value 0x1 enables RNG clock."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Writing value 0x1 enables RNG clock."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing value 0x1 enables RNG clock."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing value 0x1 enables RNG clock."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<RngclkenableSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "Writing to this register enables_disables the RNG clock.\n\nYou can [`read`](crate::Reg::read) this register and get [`rngclkenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngclkenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngclkenableSpec;
impl crate::RegisterSpec for RngclkenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngclkenable::R`](R) reader structure"]
impl crate::Readable for RngclkenableSpec {}
#[doc = "`write(|w| ..)` method takes [`rngclkenable::W`](W) writer structure"]
impl crate::Writable for RngclkenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNGCLKENABLE to value 0"]
impl crate::Resettable for RngclkenableSpec {
    const RESET_VALUE: u32 = 0;
}
