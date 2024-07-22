#[doc = "Register `TMR1LMTVAL` reader"]
pub type R = crate::R<Tmr1lmtvalSpec>;
#[doc = "Register `TMR1LMTVAL` writer"]
pub type W = crate::W<Tmr1lmtvalSpec>;
#[doc = "Field `TMR1LMTVAL` reader - Counter/Timer 1 Limit Readback"]
pub type Tmr1lmtvalR = crate::FieldReader;
#[doc = "Field `TMR1LMTVAL` writer - Counter/Timer 1 Limit Readback"]
pub type Tmr1lmtvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Counter/Timer 1 Limit Readback"]
    #[inline(always)]
    pub fn tmr1lmtval(&self) -> Tmr1lmtvalR {
        Tmr1lmtvalR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter/Timer 1 Limit Readback"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1lmtval(&mut self) -> Tmr1lmtvalW<Tmr1lmtvalSpec> {
        Tmr1lmtvalW::new(self, 0)
    }
}
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr1lmtval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr1lmtval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr1lmtvalSpec;
impl crate::RegisterSpec for Tmr1lmtvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr1lmtval::R`](R) reader structure"]
impl crate::Readable for Tmr1lmtvalSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr1lmtval::W`](W) writer structure"]
impl crate::Writable for Tmr1lmtvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR1LMTVAL to value 0"]
impl crate::Resettable for Tmr1lmtvalSpec {
    const RESET_VALUE: u32 = 0;
}
