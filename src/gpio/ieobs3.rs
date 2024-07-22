#[doc = "Register `IEOBS3` reader"]
pub type R = crate::R<Ieobs3Spec>;
#[doc = "Register `IEOBS3` writer"]
pub type W = crate::W<Ieobs3Spec>;
#[doc = "Field `IEDATA3` reader - 1 indicates the input_en is active and the value of the pad will be trasmitted to the internal logic within the device."]
pub type Iedata3R = crate::FieldReader<u32>;
#[doc = "Field `IEDATA3` writer - 1 indicates the input_en is active and the value of the pad will be trasmitted to the internal logic within the device."]
pub type Iedata3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1 indicates the input_en is active and the value of the pad will be trasmitted to the internal logic within the device."]
    #[inline(always)]
    pub fn iedata3(&self) -> Iedata3R {
        Iedata3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1 indicates the input_en is active and the value of the pad will be trasmitted to the internal logic within the device."]
    #[inline(always)]
    #[must_use]
    pub fn iedata3(&mut self) -> Iedata3W<Ieobs3Spec> {
        Iedata3W::new(self, 0)
    }
}
#[doc = "Read only. Reflects the value of the input enable signals for pads 127-96 sent to the pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`ieobs3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ieobs3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ieobs3Spec;
impl crate::RegisterSpec for Ieobs3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ieobs3::R`](R) reader structure"]
impl crate::Readable for Ieobs3Spec {}
#[doc = "`write(|w| ..)` method takes [`ieobs3::W`](W) writer structure"]
impl crate::Writable for Ieobs3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEOBS3 to value 0"]
impl crate::Resettable for Ieobs3Spec {
    const RESET_VALUE: u32 = 0;
}
