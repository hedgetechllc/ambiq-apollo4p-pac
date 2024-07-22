#[doc = "Register `TMR0LMTVAL` reader"]
pub type R = crate::R<Tmr0lmtvalSpec>;
#[doc = "Register `TMR0LMTVAL` writer"]
pub type W = crate::W<Tmr0lmtvalSpec>;
#[doc = "Field `TMR0LMTVAL` reader - Counter/Timer 0 Limit Readback"]
pub type Tmr0lmtvalR = crate::FieldReader;
#[doc = "Field `TMR0LMTVAL` writer - Counter/Timer 0 Limit Readback"]
pub type Tmr0lmtvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Counter/Timer 0 Limit Readback"]
    #[inline(always)]
    pub fn tmr0lmtval(&self) -> Tmr0lmtvalR {
        Tmr0lmtvalR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter/Timer 0 Limit Readback"]
    #[inline(always)]
    #[must_use]
    pub fn tmr0lmtval(&mut self) -> Tmr0lmtvalW<Tmr0lmtvalSpec> {
        Tmr0lmtvalW::new(self, 0)
    }
}
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr0lmtval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr0lmtval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr0lmtvalSpec;
impl crate::RegisterSpec for Tmr0lmtvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr0lmtval::R`](R) reader structure"]
impl crate::Readable for Tmr0lmtvalSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr0lmtval::W`](W) writer structure"]
impl crate::Writable for Tmr0lmtvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR0LMTVAL to value 0"]
impl crate::Resettable for Tmr0lmtvalSpec {
    const RESET_VALUE: u32 = 0;
}
