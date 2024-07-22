#[doc = "Register `OEOBS2` reader"]
pub type R = crate::R<Oeobs2Spec>;
#[doc = "Register `OEOBS2` writer"]
pub type W = crate::W<Oeobs2Spec>;
#[doc = "Field `OEDATA2` reader - The signal is negative active, and a value of 0 indicates the output_en_ is active and the MCU will be driving the pad."]
pub type Oedata2R = crate::FieldReader<u32>;
#[doc = "Field `OEDATA2` writer - The signal is negative active, and a value of 0 indicates the output_en_ is active and the MCU will be driving the pad."]
pub type Oedata2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The signal is negative active, and a value of 0 indicates the output_en_ is active and the MCU will be driving the pad."]
    #[inline(always)]
    pub fn oedata2(&self) -> Oedata2R {
        Oedata2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The signal is negative active, and a value of 0 indicates the output_en_ is active and the MCU will be driving the pad."]
    #[inline(always)]
    #[must_use]
    pub fn oedata2(&mut self) -> Oedata2W<Oeobs2Spec> {
        Oedata2W::new(self, 0)
    }
}
#[doc = "Read only. Reflects the value of the output enable signals for pads 95-64 sent to the pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`oeobs2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oeobs2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Oeobs2Spec;
impl crate::RegisterSpec for Oeobs2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oeobs2::R`](R) reader structure"]
impl crate::Readable for Oeobs2Spec {}
#[doc = "`write(|w| ..)` method takes [`oeobs2::W`](W) writer structure"]
impl crate::Writable for Oeobs2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OEOBS2 to value 0"]
impl crate::Resettable for Oeobs2Spec {
    const RESET_VALUE: u32 = 0;
}
