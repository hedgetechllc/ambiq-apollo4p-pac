#[doc = "Register `TMR9LMTVAL` reader"]
pub type R = crate::R<Tmr9lmtvalSpec>;
#[doc = "Register `TMR9LMTVAL` writer"]
pub type W = crate::W<Tmr9lmtvalSpec>;
#[doc = "Field `TMR9LMTVAL` reader - Counter/Timer 9 Limit Readback"]
pub type Tmr9lmtvalR = crate::FieldReader;
#[doc = "Field `TMR9LMTVAL` writer - Counter/Timer 9 Limit Readback"]
pub type Tmr9lmtvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Counter/Timer 9 Limit Readback"]
    #[inline(always)]
    pub fn tmr9lmtval(&self) -> Tmr9lmtvalR {
        Tmr9lmtvalR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter/Timer 9 Limit Readback"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9lmtval(&mut self) -> Tmr9lmtvalW<Tmr9lmtvalSpec> {
        Tmr9lmtvalW::new(self, 0)
    }
}
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr9lmtval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr9lmtval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr9lmtvalSpec;
impl crate::RegisterSpec for Tmr9lmtvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr9lmtval::R`](R) reader structure"]
impl crate::Readable for Tmr9lmtvalSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr9lmtval::W`](W) writer structure"]
impl crate::Writable for Tmr9lmtvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR9LMTVAL to value 0"]
impl crate::Resettable for Tmr9lmtvalSpec {
    const RESET_VALUE: u32 = 0;
}
