#[doc = "Register `TMR5LMTVAL` reader"]
pub type R = crate::R<Tmr5lmtvalSpec>;
#[doc = "Register `TMR5LMTVAL` writer"]
pub type W = crate::W<Tmr5lmtvalSpec>;
#[doc = "Field `TMR5LMTVAL` reader - Counter/Timer 5 Limit Readback"]
pub type Tmr5lmtvalR = crate::FieldReader;
#[doc = "Field `TMR5LMTVAL` writer - Counter/Timer 5 Limit Readback"]
pub type Tmr5lmtvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Counter/Timer 5 Limit Readback"]
    #[inline(always)]
    pub fn tmr5lmtval(&self) -> Tmr5lmtvalR {
        Tmr5lmtvalR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter/Timer 5 Limit Readback"]
    #[inline(always)]
    #[must_use]
    pub fn tmr5lmtval(&mut self) -> Tmr5lmtvalW<Tmr5lmtvalSpec> {
        Tmr5lmtvalW::new(self, 0)
    }
}
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr5lmtval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr5lmtval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr5lmtvalSpec;
impl crate::RegisterSpec for Tmr5lmtvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr5lmtval::R`](R) reader structure"]
impl crate::Readable for Tmr5lmtvalSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr5lmtval::W`](W) writer structure"]
impl crate::Writable for Tmr5lmtvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR5LMTVAL to value 0"]
impl crate::Resettable for Tmr5lmtvalSpec {
    const RESET_VALUE: u32 = 0;
}
