#[doc = "Register `OEOBS1` reader"]
pub type R = crate::R<Oeobs1Spec>;
#[doc = "Register `OEOBS1` writer"]
pub type W = crate::W<Oeobs1Spec>;
#[doc = "Field `OEDATA1` reader - The signal is negative active, and a value of 0 indicates the output_en_ is active and the MCU will be driving the pad."]
pub type Oedata1R = crate::FieldReader<u32>;
#[doc = "Field `OEDATA1` writer - The signal is negative active, and a value of 0 indicates the output_en_ is active and the MCU will be driving the pad."]
pub type Oedata1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The signal is negative active, and a value of 0 indicates the output_en_ is active and the MCU will be driving the pad."]
    #[inline(always)]
    pub fn oedata1(&self) -> Oedata1R {
        Oedata1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The signal is negative active, and a value of 0 indicates the output_en_ is active and the MCU will be driving the pad."]
    #[inline(always)]
    #[must_use]
    pub fn oedata1(&mut self) -> Oedata1W<Oeobs1Spec> {
        Oedata1W::new(self, 0)
    }
}
#[doc = "Read only. Reflects the value of the output enable signals for pads 63-32 sent to the pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`oeobs1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oeobs1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Oeobs1Spec;
impl crate::RegisterSpec for Oeobs1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oeobs1::R`](R) reader structure"]
impl crate::Readable for Oeobs1Spec {}
#[doc = "`write(|w| ..)` method takes [`oeobs1::W`](W) writer structure"]
impl crate::Writable for Oeobs1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OEOBS1 to value 0"]
impl crate::Resettable for Oeobs1Spec {
    const RESET_VALUE: u32 = 0;
}
