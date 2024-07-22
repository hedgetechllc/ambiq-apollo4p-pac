#[doc = "Register `TMR7LMTVAL` reader"]
pub type R = crate::R<Tmr7lmtvalSpec>;
#[doc = "Register `TMR7LMTVAL` writer"]
pub type W = crate::W<Tmr7lmtvalSpec>;
#[doc = "Field `TMR7LMTVAL` reader - Counter/Timer 7 Limit Readback"]
pub type Tmr7lmtvalR = crate::FieldReader;
#[doc = "Field `TMR7LMTVAL` writer - Counter/Timer 7 Limit Readback"]
pub type Tmr7lmtvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Counter/Timer 7 Limit Readback"]
    #[inline(always)]
    pub fn tmr7lmtval(&self) -> Tmr7lmtvalR {
        Tmr7lmtvalR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter/Timer 7 Limit Readback"]
    #[inline(always)]
    #[must_use]
    pub fn tmr7lmtval(&mut self) -> Tmr7lmtvalW<Tmr7lmtvalSpec> {
        Tmr7lmtvalW::new(self, 0)
    }
}
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr7lmtval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr7lmtval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr7lmtvalSpec;
impl crate::RegisterSpec for Tmr7lmtvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr7lmtval::R`](R) reader structure"]
impl crate::Readable for Tmr7lmtvalSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr7lmtval::W`](W) writer structure"]
impl crate::Writable for Tmr7lmtvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR7LMTVAL to value 0"]
impl crate::Resettable for Tmr7lmtvalSpec {
    const RESET_VALUE: u32 = 0;
}
