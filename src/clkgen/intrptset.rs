#[doc = "Register `INTRPTSET` reader"]
pub type R = crate::R<IntrptsetSpec>;
#[doc = "Register `INTRPTSET` writer"]
pub type W = crate::W<IntrptsetSpec>;
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
    pub fn of(&mut self) -> OfW<IntrptsetSpec> {
        OfW::new(self, 0)
    }
}
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`intrptset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrptset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrptsetSpec;
impl crate::RegisterSpec for IntrptsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intrptset::R`](R) reader structure"]
impl crate::Readable for IntrptsetSpec {}
#[doc = "`write(|w| ..)` method takes [`intrptset::W`](W) writer structure"]
impl crate::Writable for IntrptsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTRPTSET to value 0"]
impl crate::Resettable for IntrptsetSpec {
    const RESET_VALUE: u32 = 0;
}
