#[doc = "Register `RNGBUSY` reader"]
pub type R = crate::R<RngbusySpec>;
#[doc = "Register `RNGBUSY` writer"]
pub type W = crate::W<RngbusySpec>;
#[doc = "Field `RNGBUSY` reader - Reflects rng_busy output port which Consists of trng_busy and prng_busy."]
pub type RngbusyR = crate::BitReader;
#[doc = "Field `RNGBUSY` writer - Reflects rng_busy output port which Consists of trng_busy and prng_busy."]
pub type RngbusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRNGBUSY` reader - Reflects trng_busy."]
pub type TrngbusyR = crate::BitReader;
#[doc = "Field `TRNGBUSY` writer - Reflects trng_busy."]
pub type TrngbusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRNGBUSY` reader - Reflects prng_busy."]
pub type PrngbusyR = crate::BitReader;
#[doc = "Field `PRNGBUSY` writer - Reflects prng_busy."]
pub type PrngbusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reflects rng_busy output port which Consists of trng_busy and prng_busy."]
    #[inline(always)]
    pub fn rngbusy(&self) -> RngbusyR {
        RngbusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reflects trng_busy."]
    #[inline(always)]
    pub fn trngbusy(&self) -> TrngbusyR {
        TrngbusyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reflects prng_busy."]
    #[inline(always)]
    pub fn prngbusy(&self) -> PrngbusyR {
        PrngbusyR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reflects rng_busy output port which Consists of trng_busy and prng_busy."]
    #[inline(always)]
    #[must_use]
    pub fn rngbusy(&mut self) -> RngbusyW<RngbusySpec> {
        RngbusyW::new(self, 0)
    }
    #[doc = "Bit 1 - Reflects trng_busy."]
    #[inline(always)]
    #[must_use]
    pub fn trngbusy(&mut self) -> TrngbusyW<RngbusySpec> {
        TrngbusyW::new(self, 1)
    }
    #[doc = "Bit 2 - Reflects prng_busy."]
    #[inline(always)]
    #[must_use]
    pub fn prngbusy(&mut self) -> PrngbusyW<RngbusySpec> {
        PrngbusyW::new(self, 2)
    }
}
#[doc = "RNG busy indication\n\nYou can [`read`](crate::Reg::read) this register and get [`rngbusy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngbusy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngbusySpec;
impl crate::RegisterSpec for RngbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngbusy::R`](R) reader structure"]
impl crate::Readable for RngbusySpec {}
#[doc = "`write(|w| ..)` method takes [`rngbusy::W`](W) writer structure"]
impl crate::Writable for RngbusySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNGBUSY to value 0"]
impl crate::Resettable for RngbusySpec {
    const RESET_VALUE: u32 = 0;
}
