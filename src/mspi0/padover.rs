#[doc = "Register `PADOVER` reader"]
pub type R = crate::R<PadoverSpec>;
#[doc = "Register `PADOVER` writer"]
pub type W = crate::W<PadoverSpec>;
#[doc = "Field `OVERRIDE` reader - Output pad override value. \\[7:0\\]=data \\[8\\]=clock \\[9\\]=DM"]
pub type OverrideR = crate::FieldReader<u32>;
#[doc = "Field `OVERRIDE` writer - Output pad override value. \\[7:0\\]=data \\[8\\]=clock \\[9\\]=DM"]
pub type OverrideW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Output pad override value. \\[7:0\\]=data \\[8\\]=clock \\[9\\]=DM"]
    #[inline(always)]
    pub fn override_(&self) -> OverrideR {
        OverrideR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Output pad override value. \\[7:0\\]=data \\[8\\]=clock \\[9\\]=DM"]
    #[inline(always)]
    #[must_use]
    pub fn override_(&mut self) -> OverrideW<PadoverSpec> {
        OverrideW::new(self, 0)
    }
}
#[doc = "Override data value\n\nYou can [`read`](crate::Reg::read) this register and get [`padover::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padover::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadoverSpec;
impl crate::RegisterSpec for PadoverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padover::R`](R) reader structure"]
impl crate::Readable for PadoverSpec {}
#[doc = "`write(|w| ..)` method takes [`padover::W`](W) writer structure"]
impl crate::Writable for PadoverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PADOVER to value 0"]
impl crate::Resettable for PadoverSpec {
    const RESET_VALUE: u32 = 0;
}
