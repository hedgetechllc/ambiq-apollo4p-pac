#[doc = "Register `INTRPTSTAT` reader"]
pub type R = crate::R<IntrptstatSpec>;
#[doc = "Register `INTRPTSTAT` writer"]
pub type W = crate::W<IntrptstatSpec>;
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
    pub fn of(&mut self) -> OfW<IntrptstatSpec> {
        OfW::new(self, 0)
    }
}
#[doc = "Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`intrptstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrptstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrptstatSpec;
impl crate::RegisterSpec for IntrptstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intrptstat::R`](R) reader structure"]
impl crate::Readable for IntrptstatSpec {}
#[doc = "`write(|w| ..)` method takes [`intrptstat::W`](W) writer structure"]
impl crate::Writable for IntrptstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTRPTSTAT to value 0"]
impl crate::Resettable for IntrptstatSpec {
    const RESET_VALUE: u32 = 0;
}
