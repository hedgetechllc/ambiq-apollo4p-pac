#[doc = "Register `TIMERSPARES` reader"]
pub type R = crate::R<TimersparesSpec>;
#[doc = "Register `TIMERSPARES` writer"]
pub type W = crate::W<TimersparesSpec>;
#[doc = "Field `TMRSPARES` reader - Placeholer spare registes that can be used as needed for future use"]
pub type TmrsparesR = crate::FieldReader<u32>;
#[doc = "Field `TMRSPARES` writer - Placeholer spare registes that can be used as needed for future use"]
pub type TmrsparesW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Placeholer spare registes that can be used as needed for future use"]
    #[inline(always)]
    pub fn tmrspares(&self) -> TmrsparesR {
        TmrsparesR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Placeholer spare registes that can be used as needed for future use"]
    #[inline(always)]
    #[must_use]
    pub fn tmrspares(&mut self) -> TmrsparesW<TimersparesSpec> {
        TmrsparesW::new(self, 0)
    }
}
#[doc = "Timer Spare Regs\n\nYou can [`read`](crate::Reg::read) this register and get [`timerspares::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timerspares::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimersparesSpec;
impl crate::RegisterSpec for TimersparesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timerspares::R`](R) reader structure"]
impl crate::Readable for TimersparesSpec {}
#[doc = "`write(|w| ..)` method takes [`timerspares::W`](W) writer structure"]
impl crate::Writable for TimersparesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMERSPARES to value 0"]
impl crate::Resettable for TimersparesSpec {
    const RESET_VALUE: u32 = 0;
}
