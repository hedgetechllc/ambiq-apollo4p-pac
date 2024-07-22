#[doc = "Register `INTCLR` reader"]
pub type R = crate::R<IntclrSpec>;
#[doc = "Register `INTCLR` writer"]
pub type W = crate::W<IntclrSpec>;
#[doc = "Field `ALM` reader - RTC Alarm interrupt"]
pub type AlmR = crate::BitReader;
#[doc = "Field `ALM` writer - RTC Alarm interrupt"]
pub type AlmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RTC Alarm interrupt"]
    #[inline(always)]
    pub fn alm(&self) -> AlmR {
        AlmR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Alarm interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn alm(&mut self) -> AlmW<IntclrSpec> {
        AlmW::new(self, 0)
    }
}
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`intclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntclrSpec;
impl crate::RegisterSpec for IntclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intclr::R`](R) reader structure"]
impl crate::Readable for IntclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intclr::W`](W) writer structure"]
impl crate::Writable for IntclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTCLR to value 0"]
impl crate::Resettable for IntclrSpec {
    const RESET_VALUE: u32 = 0;
}
