#[doc = "Register `TMR2LMTVAL` reader"]
pub type R = crate::R<Tmr2lmtvalSpec>;
#[doc = "Register `TMR2LMTVAL` writer"]
pub type W = crate::W<Tmr2lmtvalSpec>;
#[doc = "Field `TMR2LMTVAL` reader - Counter/Timer 2 Limit Readback"]
pub type Tmr2lmtvalR = crate::FieldReader;
#[doc = "Field `TMR2LMTVAL` writer - Counter/Timer 2 Limit Readback"]
pub type Tmr2lmtvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Counter/Timer 2 Limit Readback"]
    #[inline(always)]
    pub fn tmr2lmtval(&self) -> Tmr2lmtvalR {
        Tmr2lmtvalR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter/Timer 2 Limit Readback"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2lmtval(&mut self) -> Tmr2lmtvalW<Tmr2lmtvalSpec> {
        Tmr2lmtvalW::new(self, 0)
    }
}
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr2lmtval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr2lmtval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr2lmtvalSpec;
impl crate::RegisterSpec for Tmr2lmtvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr2lmtval::R`](R) reader structure"]
impl crate::Readable for Tmr2lmtvalSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr2lmtval::W`](W) writer structure"]
impl crate::Writable for Tmr2lmtvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR2LMTVAL to value 0"]
impl crate::Resettable for Tmr2lmtvalSpec {
    const RESET_VALUE: u32 = 0;
}
