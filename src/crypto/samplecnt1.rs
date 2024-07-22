#[doc = "Register `SAMPLECNT1` reader"]
pub type R = crate::R<Samplecnt1Spec>;
#[doc = "Register `SAMPLECNT1` writer"]
pub type W = crate::W<Samplecnt1Spec>;
#[doc = "Field `SAMPLECNTR1` reader - Sets the number of rng_clk cycles between two consecutive ring oscillator samples. Note: If the Von-Neumann is bypassed, the minimum value for sample counter must not be less than decimal seventeen."]
pub type Samplecntr1R = crate::FieldReader<u32>;
#[doc = "Field `SAMPLECNTR1` writer - Sets the number of rng_clk cycles between two consecutive ring oscillator samples. Note: If the Von-Neumann is bypassed, the minimum value for sample counter must not be less than decimal seventeen."]
pub type Samplecntr1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Sets the number of rng_clk cycles between two consecutive ring oscillator samples. Note: If the Von-Neumann is bypassed, the minimum value for sample counter must not be less than decimal seventeen."]
    #[inline(always)]
    pub fn samplecntr1(&self) -> Samplecntr1R {
        Samplecntr1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Sets the number of rng_clk cycles between two consecutive ring oscillator samples. Note: If the Von-Neumann is bypassed, the minimum value for sample counter must not be less than decimal seventeen."]
    #[inline(always)]
    #[must_use]
    pub fn samplecntr1(&mut self) -> Samplecntr1W<Samplecnt1Spec> {
        Samplecntr1W::new(self, 0)
    }
}
#[doc = "Counts clocks between sampling of random bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`samplecnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`samplecnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Samplecnt1Spec;
impl crate::RegisterSpec for Samplecnt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`samplecnt1::R`](R) reader structure"]
impl crate::Readable for Samplecnt1Spec {}
#[doc = "`write(|w| ..)` method takes [`samplecnt1::W`](W) writer structure"]
impl crate::Writable for Samplecnt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAMPLECNT1 to value 0xffff"]
impl crate::Resettable for Samplecnt1Spec {
    const RESET_VALUE: u32 = 0xffff;
}
