#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
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
    pub fn alm(&mut self) -> AlmW<IntenSpec> {
        AlmW::new(self, 0)
    }
}
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
