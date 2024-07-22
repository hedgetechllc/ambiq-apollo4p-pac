#[doc = "Register `DEBUGDATA` reader"]
pub type R = crate::R<DebugdataSpec>;
#[doc = "Register `DEBUGDATA` writer"]
pub type W = crate::W<DebugdataSpec>;
#[doc = "Field `DEBUGDATA` reader - Debug Data"]
pub type DebugdataR = crate::FieldReader<u32>;
#[doc = "Field `DEBUGDATA` writer - Debug Data"]
pub type DebugdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Debug Data"]
    #[inline(always)]
    pub fn debugdata(&self) -> DebugdataR {
        DebugdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Debug Data"]
    #[inline(always)]
    #[must_use]
    pub fn debugdata(&mut self) -> DebugdataW<DebugdataSpec> {
        DebugdataW::new(self, 0)
    }
}
#[doc = "PIO Input Values\n\nYou can [`read`](crate::Reg::read) this register and get [`debugdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debugdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugdataSpec;
impl crate::RegisterSpec for DebugdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debugdata::R`](R) reader structure"]
impl crate::Readable for DebugdataSpec {}
#[doc = "`write(|w| ..)` method takes [`debugdata::W`](W) writer structure"]
impl crate::Writable for DebugdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUGDATA to value 0"]
impl crate::Resettable for DebugdataSpec {
    const RESET_VALUE: u32 = 0;
}
