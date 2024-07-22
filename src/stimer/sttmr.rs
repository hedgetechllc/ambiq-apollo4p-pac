#[doc = "Register `STTMR` reader"]
pub type R = crate::R<SttmrSpec>;
#[doc = "Register `STTMR` writer"]
pub type W = crate::W<SttmrSpec>;
#[doc = "Field `STTMR` reader - Value of the 32-bit counter as it ticks over."]
pub type SttmrR = crate::FieldReader<u32>;
#[doc = "Field `STTMR` writer - Value of the 32-bit counter as it ticks over."]
pub type SttmrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Value of the 32-bit counter as it ticks over."]
    #[inline(always)]
    pub fn sttmr(&self) -> SttmrR {
        SttmrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of the 32-bit counter as it ticks over."]
    #[inline(always)]
    #[must_use]
    pub fn sttmr(&mut self) -> SttmrW<SttmrSpec> {
        SttmrW::new(self, 0)
    }
}
#[doc = "The COUNTER Register contains the running count of time as maintained by incrementing for every rising clock edge of the clock source selected in the configuration register. It is this counter value that captured in the capture registers and it is this counter value that is compared against the various compare registers. This register cannot be written, but can be cleared to 0 for a deterministic value. Use the FREEZE bit will stop this counter from incrementing.\n\nYou can [`read`](crate::Reg::read) this register and get [`sttmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sttmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SttmrSpec;
impl crate::RegisterSpec for SttmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sttmr::R`](R) reader structure"]
impl crate::Readable for SttmrSpec {}
#[doc = "`write(|w| ..)` method takes [`sttmr::W`](W) writer structure"]
impl crate::Writable for SttmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STTMR to value 0"]
impl crate::Resettable for SttmrSpec {
    const RESET_VALUE: u32 = 0;
}
