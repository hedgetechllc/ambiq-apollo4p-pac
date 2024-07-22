#[doc = "Register `TMR15LMTVAL` reader"]
pub type R = crate::R<Tmr15lmtvalSpec>;
#[doc = "Register `TMR15LMTVAL` writer"]
pub type W = crate::W<Tmr15lmtvalSpec>;
#[doc = "Field `TMR15LMTVAL` reader - Counter/Timer 15 Limit Readback"]
pub type Tmr15lmtvalR = crate::FieldReader;
#[doc = "Field `TMR15LMTVAL` writer - Counter/Timer 15 Limit Readback"]
pub type Tmr15lmtvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Counter/Timer 15 Limit Readback"]
    #[inline(always)]
    pub fn tmr15lmtval(&self) -> Tmr15lmtvalR {
        Tmr15lmtvalR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter/Timer 15 Limit Readback"]
    #[inline(always)]
    #[must_use]
    pub fn tmr15lmtval(&mut self) -> Tmr15lmtvalW<Tmr15lmtvalSpec> {
        Tmr15lmtvalW::new(self, 0)
    }
}
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr15lmtval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr15lmtval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr15lmtvalSpec;
impl crate::RegisterSpec for Tmr15lmtvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr15lmtval::R`](R) reader structure"]
impl crate::Readable for Tmr15lmtvalSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr15lmtval::W`](W) writer structure"]
impl crate::Writable for Tmr15lmtvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR15LMTVAL to value 0"]
impl crate::Resettable for Tmr15lmtvalSpec {
    const RESET_VALUE: u32 = 0;
}
