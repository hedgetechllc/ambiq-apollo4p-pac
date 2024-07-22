#[doc = "Register `IEOBS2` reader"]
pub type R = crate::R<Ieobs2Spec>;
#[doc = "Register `IEOBS2` writer"]
pub type W = crate::W<Ieobs2Spec>;
#[doc = "Field `IEDATA2` reader - 1 indicates the input_en is active and the value of the pad will be trasmitted to the internal logic within the device."]
pub type Iedata2R = crate::FieldReader<u32>;
#[doc = "Field `IEDATA2` writer - 1 indicates the input_en is active and the value of the pad will be trasmitted to the internal logic within the device."]
pub type Iedata2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1 indicates the input_en is active and the value of the pad will be trasmitted to the internal logic within the device."]
    #[inline(always)]
    pub fn iedata2(&self) -> Iedata2R {
        Iedata2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1 indicates the input_en is active and the value of the pad will be trasmitted to the internal logic within the device."]
    #[inline(always)]
    #[must_use]
    pub fn iedata2(&mut self) -> Iedata2W<Ieobs2Spec> {
        Iedata2W::new(self, 0)
    }
}
#[doc = "Read only. Reflects the value of the input enable signals for pads 95-64 sent to the pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`ieobs2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ieobs2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ieobs2Spec;
impl crate::RegisterSpec for Ieobs2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ieobs2::R`](R) reader structure"]
impl crate::Readable for Ieobs2Spec {}
#[doc = "`write(|w| ..)` method takes [`ieobs2::W`](W) writer structure"]
impl crate::Writable for Ieobs2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEOBS2 to value 0"]
impl crate::Resettable for Ieobs2Spec {
    const RESET_VALUE: u32 = 0;
}
