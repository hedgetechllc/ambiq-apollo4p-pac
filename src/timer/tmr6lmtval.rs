#[doc = "Register `TMR6LMTVAL` reader"]
pub type R = crate::R<Tmr6lmtvalSpec>;
#[doc = "Register `TMR6LMTVAL` writer"]
pub type W = crate::W<Tmr6lmtvalSpec>;
#[doc = "Field `TMR6LMTVAL` reader - Counter/Timer 6 Limit Readback"]
pub type Tmr6lmtvalR = crate::FieldReader;
#[doc = "Field `TMR6LMTVAL` writer - Counter/Timer 6 Limit Readback"]
pub type Tmr6lmtvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Counter/Timer 6 Limit Readback"]
    #[inline(always)]
    pub fn tmr6lmtval(&self) -> Tmr6lmtvalR {
        Tmr6lmtvalR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter/Timer 6 Limit Readback"]
    #[inline(always)]
    #[must_use]
    pub fn tmr6lmtval(&mut self) -> Tmr6lmtvalW<Tmr6lmtvalSpec> {
        Tmr6lmtvalW::new(self, 0)
    }
}
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr6lmtval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr6lmtval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr6lmtvalSpec;
impl crate::RegisterSpec for Tmr6lmtvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr6lmtval::R`](R) reader structure"]
impl crate::Readable for Tmr6lmtvalSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr6lmtval::W`](W) writer structure"]
impl crate::Writable for Tmr6lmtvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR6LMTVAL to value 0"]
impl crate::Resettable for Tmr6lmtvalSpec {
    const RESET_VALUE: u32 = 0;
}
