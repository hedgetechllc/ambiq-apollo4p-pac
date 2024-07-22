#[doc = "Register `RNGWATCHDOGVAL` reader"]
pub type R = crate::R<RngwatchdogvalSpec>;
#[doc = "Register `RNGWATCHDOGVAL` writer"]
pub type W = crate::W<RngwatchdogvalSpec>;
#[doc = "Field `RNGWATCHDOGVAL` reader - Defines the maximum number of clock cycles per TRNG collection of 192 samples. If the number of cycles for a collection exceeds this threshold, TRNG signals an interrupt."]
pub type RngwatchdogvalR = crate::FieldReader<u32>;
#[doc = "Field `RNGWATCHDOGVAL` writer - Defines the maximum number of clock cycles per TRNG collection of 192 samples. If the number of cycles for a collection exceeds this threshold, TRNG signals an interrupt."]
pub type RngwatchdogvalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the maximum number of clock cycles per TRNG collection of 192 samples. If the number of cycles for a collection exceeds this threshold, TRNG signals an interrupt."]
    #[inline(always)]
    pub fn rngwatchdogval(&self) -> RngwatchdogvalR {
        RngwatchdogvalR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the maximum number of clock cycles per TRNG collection of 192 samples. If the number of cycles for a collection exceeds this threshold, TRNG signals an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rngwatchdogval(&mut self) -> RngwatchdogvalW<RngwatchdogvalSpec> {
        RngwatchdogvalW::new(self, 0)
    }
}
#[doc = "This register defines the number of 192-bits samples that the DMA collects per RNG configuration.bitfield 7:0 RNG_SAMPLES_NUM rw 0x0 Defines the number of 192-bits samples that the DMA collects per RNG configuration.bitfield 31:8 RESERVED rw 0x0 ReservedThis register defines the maximum number of clock cycles per TRNG collection of 192 samples. If the number of cycles for a collection exceeds this threshold, TRNG signals an interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`rngwatchdogval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngwatchdogval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngwatchdogvalSpec;
impl crate::RegisterSpec for RngwatchdogvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngwatchdogval::R`](R) reader structure"]
impl crate::Readable for RngwatchdogvalSpec {}
#[doc = "`write(|w| ..)` method takes [`rngwatchdogval::W`](W) writer structure"]
impl crate::Writable for RngwatchdogvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNGWATCHDOGVAL to value 0"]
impl crate::Resettable for RngwatchdogvalSpec {
    const RESET_VALUE: u32 = 0;
}
