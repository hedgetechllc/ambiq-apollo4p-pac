#[doc = "Register `OEOBS0` reader"]
pub type R = crate::R<Oeobs0Spec>;
#[doc = "Register `OEOBS0` writer"]
pub type W = crate::W<Oeobs0Spec>;
#[doc = "Field `OEDATA0` reader - The signal is negative active, and a value of 0 indicates the output_en_ is active and the MCU will be driving the pad."]
pub type Oedata0R = crate::FieldReader<u32>;
#[doc = "Field `OEDATA0` writer - The signal is negative active, and a value of 0 indicates the output_en_ is active and the MCU will be driving the pad."]
pub type Oedata0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The signal is negative active, and a value of 0 indicates the output_en_ is active and the MCU will be driving the pad."]
    #[inline(always)]
    pub fn oedata0(&self) -> Oedata0R {
        Oedata0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The signal is negative active, and a value of 0 indicates the output_en_ is active and the MCU will be driving the pad."]
    #[inline(always)]
    #[must_use]
    pub fn oedata0(&mut self) -> Oedata0W<Oeobs0Spec> {
        Oedata0W::new(self, 0)
    }
}
#[doc = "Read only. Reflects the value of the output enable signals for pads 31-0 sent to the pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`oeobs0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oeobs0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Oeobs0Spec;
impl crate::RegisterSpec for Oeobs0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oeobs0::R`](R) reader structure"]
impl crate::Readable for Oeobs0Spec {}
#[doc = "`write(|w| ..)` method takes [`oeobs0::W`](W) writer structure"]
impl crate::Writable for Oeobs0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OEOBS0 to value 0"]
impl crate::Resettable for Oeobs0Spec {
    const RESET_VALUE: u32 = 0;
}
