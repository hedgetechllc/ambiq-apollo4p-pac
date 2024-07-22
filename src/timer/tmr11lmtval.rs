#[doc = "Register `TMR11LMTVAL` reader"]
pub type R = crate::R<Tmr11lmtvalSpec>;
#[doc = "Register `TMR11LMTVAL` writer"]
pub type W = crate::W<Tmr11lmtvalSpec>;
#[doc = "Field `TMR11LMTVAL` reader - Counter/Timer 11 Limit Readback"]
pub type Tmr11lmtvalR = crate::FieldReader;
#[doc = "Field `TMR11LMTVAL` writer - Counter/Timer 11 Limit Readback"]
pub type Tmr11lmtvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Counter/Timer 11 Limit Readback"]
    #[inline(always)]
    pub fn tmr11lmtval(&self) -> Tmr11lmtvalR {
        Tmr11lmtvalR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter/Timer 11 Limit Readback"]
    #[inline(always)]
    #[must_use]
    pub fn tmr11lmtval(&mut self) -> Tmr11lmtvalW<Tmr11lmtvalSpec> {
        Tmr11lmtvalW::new(self, 0)
    }
}
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr11lmtval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr11lmtval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr11lmtvalSpec;
impl crate::RegisterSpec for Tmr11lmtvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr11lmtval::R`](R) reader structure"]
impl crate::Readable for Tmr11lmtvalSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr11lmtval::W`](W) writer structure"]
impl crate::Writable for Tmr11lmtvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR11LMTVAL to value 0"]
impl crate::Resettable for Tmr11lmtvalSpec {
    const RESET_VALUE: u32 = 0;
}
