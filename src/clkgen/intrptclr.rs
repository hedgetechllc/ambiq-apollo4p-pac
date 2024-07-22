#[doc = "Register `INTRPTCLR` reader"]
pub type R = crate::R<IntrptclrSpec>;
#[doc = "Register `INTRPTCLR` writer"]
pub type W = crate::W<IntrptclrSpec>;
#[doc = "Field `OF` reader - XT Oscillator Fail interrupt"]
pub type OfR = crate::BitReader;
#[doc = "Field `OF` writer - XT Oscillator Fail interrupt"]
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - XT Oscillator Fail interrupt"]
    #[inline(always)]
    pub fn of(&self) -> OfR {
        OfR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XT Oscillator Fail interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OfW<IntrptclrSpec> {
        OfW::new(self, 0)
    }
}
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`intrptclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrptclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrptclrSpec;
impl crate::RegisterSpec for IntrptclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intrptclr::R`](R) reader structure"]
impl crate::Readable for IntrptclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intrptclr::W`](W) writer structure"]
impl crate::Writable for IntrptclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTRPTCLR to value 0"]
impl crate::Resettable for IntrptclrSpec {
    const RESET_VALUE: u32 = 0;
}
