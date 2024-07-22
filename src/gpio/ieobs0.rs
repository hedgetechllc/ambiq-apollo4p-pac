#[doc = "Register `IEOBS0` reader"]
pub type R = crate::R<Ieobs0Spec>;
#[doc = "Register `IEOBS0` writer"]
pub type W = crate::W<Ieobs0Spec>;
#[doc = "Field `IEDATA0` reader - 1 indicates the input_en is active and the value of the pad will be trasmitted to the internal logic within the device."]
pub type Iedata0R = crate::FieldReader<u32>;
#[doc = "Field `IEDATA0` writer - 1 indicates the input_en is active and the value of the pad will be trasmitted to the internal logic within the device."]
pub type Iedata0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1 indicates the input_en is active and the value of the pad will be trasmitted to the internal logic within the device."]
    #[inline(always)]
    pub fn iedata0(&self) -> Iedata0R {
        Iedata0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1 indicates the input_en is active and the value of the pad will be trasmitted to the internal logic within the device."]
    #[inline(always)]
    #[must_use]
    pub fn iedata0(&mut self) -> Iedata0W<Ieobs0Spec> {
        Iedata0W::new(self, 0)
    }
}
#[doc = "Read only. Reflects the value of the input enable signals for pads 31-0 sent to the pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`ieobs0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ieobs0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ieobs0Spec;
impl crate::RegisterSpec for Ieobs0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ieobs0::R`](R) reader structure"]
impl crate::Readable for Ieobs0Spec {}
#[doc = "`write(|w| ..)` method takes [`ieobs0::W`](W) writer structure"]
impl crate::Writable for Ieobs0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEOBS0 to value 0"]
impl crate::Resettable for Ieobs0Spec {
    const RESET_VALUE: u32 = 0;
}
