#[doc = "Register `TMR13LMTVAL` reader"]
pub type R = crate::R<Tmr13lmtvalSpec>;
#[doc = "Register `TMR13LMTVAL` writer"]
pub type W = crate::W<Tmr13lmtvalSpec>;
#[doc = "Field `TMR13LMTVAL` reader - Counter/Timer 13 Limit Readback"]
pub type Tmr13lmtvalR = crate::FieldReader;
#[doc = "Field `TMR13LMTVAL` writer - Counter/Timer 13 Limit Readback"]
pub type Tmr13lmtvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Counter/Timer 13 Limit Readback"]
    #[inline(always)]
    pub fn tmr13lmtval(&self) -> Tmr13lmtvalR {
        Tmr13lmtvalR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter/Timer 13 Limit Readback"]
    #[inline(always)]
    #[must_use]
    pub fn tmr13lmtval(&mut self) -> Tmr13lmtvalW<Tmr13lmtvalSpec> {
        Tmr13lmtvalW::new(self, 0)
    }
}
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr13lmtval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr13lmtval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr13lmtvalSpec;
impl crate::RegisterSpec for Tmr13lmtvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr13lmtval::R`](R) reader structure"]
impl crate::Readable for Tmr13lmtvalSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr13lmtval::W`](W) writer structure"]
impl crate::Writable for Tmr13lmtvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR13LMTVAL to value 0"]
impl crate::Resettable for Tmr13lmtvalSpec {
    const RESET_VALUE: u32 = 0;
}
