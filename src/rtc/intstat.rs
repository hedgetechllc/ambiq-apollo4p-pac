#[doc = "Register `INTSTAT` reader"]
pub type R = crate::R<IntstatSpec>;
#[doc = "Register `INTSTAT` writer"]
pub type W = crate::W<IntstatSpec>;
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
    pub fn alm(&mut self) -> AlmW<IntstatSpec> {
        AlmW::new(self, 0)
    }
}
#[doc = "Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatSpec;
impl crate::RegisterSpec for IntstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat::R`](R) reader structure"]
impl crate::Readable for IntstatSpec {}
#[doc = "`write(|w| ..)` method takes [`intstat::W`](W) writer structure"]
impl crate::Writable for IntstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for IntstatSpec {
    const RESET_VALUE: u32 = 0;
}
