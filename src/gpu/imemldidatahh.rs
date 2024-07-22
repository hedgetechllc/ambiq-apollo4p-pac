#[doc = "Register `IMEMLDIDATAHH` reader"]
pub type R = crate::R<ImemldidatahhSpec>;
#[doc = "Register `IMEMLDIDATAHH` writer"]
pub type W = crate::W<ImemldidatahhSpec>;
#[doc = "Field `IMEM` reader - DATA Load shader. Load shader instruction Memory data (63:32)."]
pub type ImemR = crate::FieldReader<u32>;
#[doc = "Field `IMEM` writer - DATA Load shader. Load shader instruction Memory data (63:32)."]
pub type ImemW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DATA Load shader. Load shader instruction Memory data (63:32)."]
    #[inline(always)]
    pub fn imem(&self) -> ImemR {
        ImemR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DATA Load shader. Load shader instruction Memory data (63:32)."]
    #[inline(always)]
    #[must_use]
    pub fn imem(&mut self) -> ImemW<ImemldidatahhSpec> {
        ImemW::new(self, 0)
    }
}
#[doc = "Load shader instruction Memory data (63:32).\n\nYou can [`read`](crate::Reg::read) this register and get [`imemldidatahh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imemldidatahh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImemldidatahhSpec;
impl crate::RegisterSpec for ImemldidatahhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imemldidatahh::R`](R) reader structure"]
impl crate::Readable for ImemldidatahhSpec {}
#[doc = "`write(|w| ..)` method takes [`imemldidatahh::W`](W) writer structure"]
impl crate::Writable for ImemldidatahhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMEMLDIDATAHH to value 0"]
impl crate::Resettable for ImemldidatahhSpec {
    const RESET_VALUE: u32 = 0;
}
