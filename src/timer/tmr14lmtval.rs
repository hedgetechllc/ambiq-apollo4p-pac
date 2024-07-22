#[doc = "Register `TMR14LMTVAL` reader"]
pub type R = crate::R<Tmr14lmtvalSpec>;
#[doc = "Register `TMR14LMTVAL` writer"]
pub type W = crate::W<Tmr14lmtvalSpec>;
#[doc = "Field `TMR14LMTVAL` reader - Counter/Timer 14 Limit Readback"]
pub type Tmr14lmtvalR = crate::FieldReader;
#[doc = "Field `TMR14LMTVAL` writer - Counter/Timer 14 Limit Readback"]
pub type Tmr14lmtvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Counter/Timer 14 Limit Readback"]
    #[inline(always)]
    pub fn tmr14lmtval(&self) -> Tmr14lmtvalR {
        Tmr14lmtvalR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter/Timer 14 Limit Readback"]
    #[inline(always)]
    #[must_use]
    pub fn tmr14lmtval(&mut self) -> Tmr14lmtvalW<Tmr14lmtvalSpec> {
        Tmr14lmtvalW::new(self, 0)
    }
}
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr14lmtval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr14lmtval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr14lmtvalSpec;
impl crate::RegisterSpec for Tmr14lmtvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr14lmtval::R`](R) reader structure"]
impl crate::Readable for Tmr14lmtvalSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr14lmtval::W`](W) writer structure"]
impl crate::Writable for Tmr14lmtvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR14LMTVAL to value 0"]
impl crate::Resettable for Tmr14lmtvalSpec {
    const RESET_VALUE: u32 = 0;
}
