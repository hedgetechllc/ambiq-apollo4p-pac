#[doc = "Register `TMR12LMTVAL` reader"]
pub type R = crate::R<Tmr12lmtvalSpec>;
#[doc = "Register `TMR12LMTVAL` writer"]
pub type W = crate::W<Tmr12lmtvalSpec>;
#[doc = "Field `TMR12LMTVAL` reader - Counter/Timer 12 Limit Readback"]
pub type Tmr12lmtvalR = crate::FieldReader;
#[doc = "Field `TMR12LMTVAL` writer - Counter/Timer 12 Limit Readback"]
pub type Tmr12lmtvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Counter/Timer 12 Limit Readback"]
    #[inline(always)]
    pub fn tmr12lmtval(&self) -> Tmr12lmtvalR {
        Tmr12lmtvalR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter/Timer 12 Limit Readback"]
    #[inline(always)]
    #[must_use]
    pub fn tmr12lmtval(&mut self) -> Tmr12lmtvalW<Tmr12lmtvalSpec> {
        Tmr12lmtvalW::new(self, 0)
    }
}
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr12lmtval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr12lmtval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr12lmtvalSpec;
impl crate::RegisterSpec for Tmr12lmtvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr12lmtval::R`](R) reader structure"]
impl crate::Readable for Tmr12lmtvalSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr12lmtval::W`](W) writer structure"]
impl crate::Writable for Tmr12lmtvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR12LMTVAL to value 0"]
impl crate::Resettable for Tmr12lmtvalSpec {
    const RESET_VALUE: u32 = 0;
}
