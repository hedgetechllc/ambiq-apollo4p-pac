#[doc = "Register `TMR10LMTVAL` reader"]
pub type R = crate::R<Tmr10lmtvalSpec>;
#[doc = "Register `TMR10LMTVAL` writer"]
pub type W = crate::W<Tmr10lmtvalSpec>;
#[doc = "Field `TMR10LMTVAL` reader - Counter/Timer 10 Limit Readback"]
pub type Tmr10lmtvalR = crate::FieldReader;
#[doc = "Field `TMR10LMTVAL` writer - Counter/Timer 10 Limit Readback"]
pub type Tmr10lmtvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Counter/Timer 10 Limit Readback"]
    #[inline(always)]
    pub fn tmr10lmtval(&self) -> Tmr10lmtvalR {
        Tmr10lmtvalR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter/Timer 10 Limit Readback"]
    #[inline(always)]
    #[must_use]
    pub fn tmr10lmtval(&mut self) -> Tmr10lmtvalW<Tmr10lmtvalSpec> {
        Tmr10lmtvalW::new(self, 0)
    }
}
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr10lmtval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr10lmtval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr10lmtvalSpec;
impl crate::RegisterSpec for Tmr10lmtvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr10lmtval::R`](R) reader structure"]
impl crate::Readable for Tmr10lmtvalSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr10lmtval::W`](W) writer structure"]
impl crate::Writable for Tmr10lmtvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR10LMTVAL to value 0"]
impl crate::Resettable for Tmr10lmtvalSpec {
    const RESET_VALUE: u32 = 0;
}
