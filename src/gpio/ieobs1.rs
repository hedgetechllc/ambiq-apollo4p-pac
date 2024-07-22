#[doc = "Register `IEOBS1` reader"]
pub type R = crate::R<Ieobs1Spec>;
#[doc = "Register `IEOBS1` writer"]
pub type W = crate::W<Ieobs1Spec>;
#[doc = "Field `IEDATA1` reader - 1 indicates the input_en is active and the value of the pad will be trasmitted to the internal logic within the device."]
pub type Iedata1R = crate::FieldReader<u32>;
#[doc = "Field `IEDATA1` writer - 1 indicates the input_en is active and the value of the pad will be trasmitted to the internal logic within the device."]
pub type Iedata1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1 indicates the input_en is active and the value of the pad will be trasmitted to the internal logic within the device."]
    #[inline(always)]
    pub fn iedata1(&self) -> Iedata1R {
        Iedata1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1 indicates the input_en is active and the value of the pad will be trasmitted to the internal logic within the device."]
    #[inline(always)]
    #[must_use]
    pub fn iedata1(&mut self) -> Iedata1W<Ieobs1Spec> {
        Iedata1W::new(self, 0)
    }
}
#[doc = "Read only. Reflects the value of the input enable signals for pads 63-32 sent to the pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`ieobs1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ieobs1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ieobs1Spec;
impl crate::RegisterSpec for Ieobs1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ieobs1::R`](R) reader structure"]
impl crate::Readable for Ieobs1Spec {}
#[doc = "`write(|w| ..)` method takes [`ieobs1::W`](W) writer structure"]
impl crate::Writable for Ieobs1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEOBS1 to value 0"]
impl crate::Resettable for Ieobs1Spec {
    const RESET_VALUE: u32 = 0;
}
