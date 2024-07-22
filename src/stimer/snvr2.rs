#[doc = "Register `SNVR2` reader"]
pub type R = crate::R<Snvr2Spec>;
#[doc = "Register `SNVR2` writer"]
pub type W = crate::W<Snvr2Spec>;
#[doc = "Field `SNVR2` reader - Value of the 32-bit counter as it ticks over."]
pub type Snvr2R = crate::FieldReader<u32>;
#[doc = "Field `SNVR2` writer - Value of the 32-bit counter as it ticks over."]
pub type Snvr2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Value of the 32-bit counter as it ticks over."]
    #[inline(always)]
    pub fn snvr2(&self) -> Snvr2R {
        Snvr2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of the 32-bit counter as it ticks over."]
    #[inline(always)]
    #[must_use]
    pub fn snvr2(&mut self) -> Snvr2W<Snvr2Spec> {
        Snvr2W::new(self, 0)
    }
}
#[doc = "The SNVR2 Register contains a portion of the stored epoch offset associated with the time in the COUNTER register. This register is only reset by POI not by HRESETn. Its contents are intended to survive all reset level except POI and full power cycles.\n\nYou can [`read`](crate::Reg::read) this register and get [`snvr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snvr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Snvr2Spec;
impl crate::RegisterSpec for Snvr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`snvr2::R`](R) reader structure"]
impl crate::Readable for Snvr2Spec {}
#[doc = "`write(|w| ..)` method takes [`snvr2::W`](W) writer structure"]
impl crate::Writable for Snvr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SNVR2 to value 0"]
impl crate::Resettable for Snvr2Spec {
    const RESET_VALUE: u32 = 0;
}
