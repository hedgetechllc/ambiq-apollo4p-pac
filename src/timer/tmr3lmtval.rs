#[doc = "Register `TMR3LMTVAL` reader"]
pub type R = crate::R<Tmr3lmtvalSpec>;
#[doc = "Register `TMR3LMTVAL` writer"]
pub type W = crate::W<Tmr3lmtvalSpec>;
#[doc = "Field `TMR3LMTVAL` reader - Counter/Timer 3 Limit Readback"]
pub type Tmr3lmtvalR = crate::FieldReader;
#[doc = "Field `TMR3LMTVAL` writer - Counter/Timer 3 Limit Readback"]
pub type Tmr3lmtvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Counter/Timer 3 Limit Readback"]
    #[inline(always)]
    pub fn tmr3lmtval(&self) -> Tmr3lmtvalR {
        Tmr3lmtvalR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter/Timer 3 Limit Readback"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3lmtval(&mut self) -> Tmr3lmtvalW<Tmr3lmtvalSpec> {
        Tmr3lmtvalW::new(self, 0)
    }
}
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr3lmtval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr3lmtval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr3lmtvalSpec;
impl crate::RegisterSpec for Tmr3lmtvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr3lmtval::R`](R) reader structure"]
impl crate::Readable for Tmr3lmtvalSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr3lmtval::W`](W) writer structure"]
impl crate::Writable for Tmr3lmtvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR3LMTVAL to value 0"]
impl crate::Resettable for Tmr3lmtvalSpec {
    const RESET_VALUE: u32 = 0;
}
