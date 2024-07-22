#[doc = "Register `SNVR0` reader"]
pub type R = crate::R<Snvr0Spec>;
#[doc = "Register `SNVR0` writer"]
pub type W = crate::W<Snvr0Spec>;
#[doc = "Field `SNVR0` reader - Value of the 32-bit counter as it ticks over."]
pub type Snvr0R = crate::FieldReader<u32>;
#[doc = "Field `SNVR0` writer - Value of the 32-bit counter as it ticks over."]
pub type Snvr0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Value of the 32-bit counter as it ticks over."]
    #[inline(always)]
    pub fn snvr0(&self) -> Snvr0R {
        Snvr0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of the 32-bit counter as it ticks over."]
    #[inline(always)]
    #[must_use]
    pub fn snvr0(&mut self) -> Snvr0W<Snvr0Spec> {
        Snvr0W::new(self, 0)
    }
}
#[doc = "The SNVR0 Register contains a portion of the stored epoch offset associated with the time in the COUNTER register. This register is only reset by POI not by HRESETn. Its contents are intended to survive all reset level except POI and full power cycles.\n\nYou can [`read`](crate::Reg::read) this register and get [`snvr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snvr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Snvr0Spec;
impl crate::RegisterSpec for Snvr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`snvr0::R`](R) reader structure"]
impl crate::Readable for Snvr0Spec {}
#[doc = "`write(|w| ..)` method takes [`snvr0::W`](W) writer structure"]
impl crate::Writable for Snvr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SNVR0 to value 0"]
impl crate::Resettable for Snvr0Spec {
    const RESET_VALUE: u32 = 0;
}
