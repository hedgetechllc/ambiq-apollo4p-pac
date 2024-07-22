#[doc = "Register `TMR8LMTVAL` reader"]
pub type R = crate::R<Tmr8lmtvalSpec>;
#[doc = "Register `TMR8LMTVAL` writer"]
pub type W = crate::W<Tmr8lmtvalSpec>;
#[doc = "Field `TMR8LMTVAL` reader - Counter/Timer 8 Limit Readback"]
pub type Tmr8lmtvalR = crate::FieldReader;
#[doc = "Field `TMR8LMTVAL` writer - Counter/Timer 8 Limit Readback"]
pub type Tmr8lmtvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Counter/Timer 8 Limit Readback"]
    #[inline(always)]
    pub fn tmr8lmtval(&self) -> Tmr8lmtvalR {
        Tmr8lmtvalR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter/Timer 8 Limit Readback"]
    #[inline(always)]
    #[must_use]
    pub fn tmr8lmtval(&mut self) -> Tmr8lmtvalW<Tmr8lmtvalSpec> {
        Tmr8lmtvalW::new(self, 0)
    }
}
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr8lmtval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr8lmtval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr8lmtvalSpec;
impl crate::RegisterSpec for Tmr8lmtvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr8lmtval::R`](R) reader structure"]
impl crate::Readable for Tmr8lmtvalSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr8lmtval::W`](W) writer structure"]
impl crate::Writable for Tmr8lmtvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR8LMTVAL to value 0"]
impl crate::Resettable for Tmr8lmtvalSpec {
    const RESET_VALUE: u32 = 0;
}
