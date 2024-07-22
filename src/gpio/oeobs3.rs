#[doc = "Register `OEOBS3` reader"]
pub type R = crate::R<Oeobs3Spec>;
#[doc = "Register `OEOBS3` writer"]
pub type W = crate::W<Oeobs3Spec>;
#[doc = "Field `OEDATA3` reader - The signal is negative active, and a value of 0 indicates the output_en_ is active and the MCU will be driving the pad."]
pub type Oedata3R = crate::FieldReader<u32>;
#[doc = "Field `OEDATA3` writer - The signal is negative active, and a value of 0 indicates the output_en_ is active and the MCU will be driving the pad."]
pub type Oedata3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The signal is negative active, and a value of 0 indicates the output_en_ is active and the MCU will be driving the pad."]
    #[inline(always)]
    pub fn oedata3(&self) -> Oedata3R {
        Oedata3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The signal is negative active, and a value of 0 indicates the output_en_ is active and the MCU will be driving the pad."]
    #[inline(always)]
    #[must_use]
    pub fn oedata3(&mut self) -> Oedata3W<Oeobs3Spec> {
        Oedata3W::new(self, 0)
    }
}
#[doc = "Read only. Reflects the value of the output enable signals for pads 127-96 sent to the pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`oeobs3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oeobs3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Oeobs3Spec;
impl crate::RegisterSpec for Oeobs3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oeobs3::R`](R) reader structure"]
impl crate::Readable for Oeobs3Spec {}
#[doc = "`write(|w| ..)` method takes [`oeobs3::W`](W) writer structure"]
impl crate::Writable for Oeobs3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OEOBS3 to value 0"]
impl crate::Resettable for Oeobs3Spec {
    const RESET_VALUE: u32 = 0;
}
