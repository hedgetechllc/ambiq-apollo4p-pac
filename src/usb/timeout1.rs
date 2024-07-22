#[doc = "Register `TIMEOUT1` reader"]
pub type R = crate::R<Timeout1Spec>;
#[doc = "Register `TIMEOUT1` writer"]
pub type W = crate::W<Timeout1Spec>;
#[doc = "Field `CTUCH` reader - Configurable Chirp Timeout timer; default value of 0x4074 corresponds to a delay of 1.1ms (60Mhz clock cycles * 4 * 0x4074)."]
pub type CtuchR = crate::FieldReader<u16>;
#[doc = "Field `CTUCH` writer - Configurable Chirp Timeout timer; default value of 0x4074 corresponds to a delay of 1.1ms (60Mhz clock cycles * 4 * 0x4074)."]
pub type CtuchW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Configurable Chirp Timeout timer; default value of 0x4074 corresponds to a delay of 1.1ms (60Mhz clock cycles * 4 * 0x4074)."]
    #[inline(always)]
    pub fn ctuch(&self) -> CtuchR {
        CtuchR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configurable Chirp Timeout timer; default value of 0x4074 corresponds to a delay of 1.1ms (60Mhz clock cycles * 4 * 0x4074)."]
    #[inline(always)]
    #[must_use]
    pub fn ctuch(&mut self) -> CtuchW<Timeout1Spec> {
        CtuchW::new(self, 0)
    }
}
#[doc = "Holds the configurable chirp timeout value.\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timeout1Spec;
impl crate::RegisterSpec for Timeout1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeout1::R`](R) reader structure"]
impl crate::Readable for Timeout1Spec {}
#[doc = "`write(|w| ..)` method takes [`timeout1::W`](W) writer structure"]
impl crate::Writable for Timeout1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMEOUT1 to value 0x4074"]
impl crate::Resettable for Timeout1Spec {
    const RESET_VALUE: u32 = 0x4074;
}
