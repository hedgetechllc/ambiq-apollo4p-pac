#[doc = "Register `SATCLR` reader"]
pub type R = crate::R<SatclrSpec>;
#[doc = "Register `SATCLR` writer"]
pub type W = crate::W<SatclrSpec>;
#[doc = "Field `SATCACLR` reader - Clear saturation event counter register for channel A (slots 0 or 1, depending on SATCHANSEL)"]
pub type SatcaclrR = crate::BitReader;
#[doc = "Field `SATCACLR` writer - Clear saturation event counter register for channel A (slots 0 or 1, depending on SATCHANSEL)"]
pub type SatcaclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SATCBCLR` reader - Clear saturation event counter register for channel B (slots 2 or 3, depending on SATCHANSEL)"]
pub type SatcbclrR = crate::BitReader;
#[doc = "Field `SATCBCLR` writer - Clear saturation event counter register for channel B (slots 2 or 3, depending on SATCHANSEL)"]
pub type SatcbclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear saturation event counter register for channel A (slots 0 or 1, depending on SATCHANSEL)"]
    #[inline(always)]
    pub fn satcaclr(&self) -> SatcaclrR {
        SatcaclrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear saturation event counter register for channel B (slots 2 or 3, depending on SATCHANSEL)"]
    #[inline(always)]
    pub fn satcbclr(&self) -> SatcbclrR {
        SatcbclrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear saturation event counter register for channel A (slots 0 or 1, depending on SATCHANSEL)"]
    #[inline(always)]
    #[must_use]
    pub fn satcaclr(&mut self) -> SatcaclrW<SatclrSpec> {
        SatcaclrW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear saturation event counter register for channel B (slots 2 or 3, depending on SATCHANSEL)"]
    #[inline(always)]
    #[must_use]
    pub fn satcbclr(&mut self) -> SatcbclrW<SatclrSpec> {
        SatcbclrW::new(self, 1)
    }
}
#[doc = "Clears the saturation event counter registers\n\nYou can [`read`](crate::Reg::read) this register and get [`satclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`satclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SatclrSpec;
impl crate::RegisterSpec for SatclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`satclr::R`](R) reader structure"]
impl crate::Readable for SatclrSpec {}
#[doc = "`write(|w| ..)` method takes [`satclr::W`](W) writer structure"]
impl crate::Writable for SatclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SATCLR to value 0"]
impl crate::Resettable for SatclrSpec {
    const RESET_VALUE: u32 = 0;
}
