#[doc = "Register `DMON0` reader"]
pub type R = crate::R<Dmon0Spec>;
#[doc = "Register `DMON0` writer"]
pub type W = crate::W<Dmon0Spec>;
#[doc = "Field `DACCESS` reader - Total accesses to data cache. All performance metrics should be relative to the number of accesses performed."]
pub type DaccessR = crate::FieldReader<u32>;
#[doc = "Field `DACCESS` writer - Total accesses to data cache. All performance metrics should be relative to the number of accesses performed."]
pub type DaccessW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Total accesses to data cache. All performance metrics should be relative to the number of accesses performed."]
    #[inline(always)]
    pub fn daccess(&self) -> DaccessR {
        DaccessR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Total accesses to data cache. All performance metrics should be relative to the number of accesses performed."]
    #[inline(always)]
    #[must_use]
    pub fn daccess(&mut self) -> DaccessW<Dmon0Spec> {
        DaccessW::new(self, 0)
    }
}
#[doc = "Data Cache Total Accesses\n\nYou can [`read`](crate::Reg::read) this register and get [`dmon0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmon0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmon0Spec;
impl crate::RegisterSpec for Dmon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmon0::R`](R) reader structure"]
impl crate::Readable for Dmon0Spec {}
#[doc = "`write(|w| ..)` method takes [`dmon0::W`](W) writer structure"]
impl crate::Writable for Dmon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMON0 to value 0"]
impl crate::Resettable for Dmon0Spec {
    const RESET_VALUE: u32 = 0;
}
