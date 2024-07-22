#[doc = "Register `TMR4LMTVAL` reader"]
pub type R = crate::R<Tmr4lmtvalSpec>;
#[doc = "Register `TMR4LMTVAL` writer"]
pub type W = crate::W<Tmr4lmtvalSpec>;
#[doc = "Field `TMR4LMTVAL` reader - Counter/Timer 4 Limit Readback"]
pub type Tmr4lmtvalR = crate::FieldReader;
#[doc = "Field `TMR4LMTVAL` writer - Counter/Timer 4 Limit Readback"]
pub type Tmr4lmtvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Counter/Timer 4 Limit Readback"]
    #[inline(always)]
    pub fn tmr4lmtval(&self) -> Tmr4lmtvalR {
        Tmr4lmtvalR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter/Timer 4 Limit Readback"]
    #[inline(always)]
    #[must_use]
    pub fn tmr4lmtval(&mut self) -> Tmr4lmtvalW<Tmr4lmtvalSpec> {
        Tmr4lmtvalW::new(self, 0)
    }
}
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr4lmtval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr4lmtval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr4lmtvalSpec;
impl crate::RegisterSpec for Tmr4lmtvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr4lmtval::R`](R) reader structure"]
impl crate::Readable for Tmr4lmtvalSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr4lmtval::W`](W) writer structure"]
impl crate::Writable for Tmr4lmtvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR4LMTVAL to value 0"]
impl crate::Resettable for Tmr4lmtvalSpec {
    const RESET_VALUE: u32 = 0;
}
