#[doc = "Register `PADOVEREN` reader"]
pub type R = crate::R<PadoverenSpec>;
#[doc = "Register `PADOVEREN` writer"]
pub type W = crate::W<PadoverenSpec>;
#[doc = "Field `OVERRIDEEN` reader - Output pad override enable. Bit mask for pad outputs. When set to 1, the values in the OVERRIDE field are driven on the pad (output enable is implicitly set in this mode). \\[7:0\\]=data \\[8\\]=clock \\[9\\]=DM"]
pub type OverrideenR = crate::FieldReader<u32>;
#[doc = "Field `OVERRIDEEN` writer - Output pad override enable. Bit mask for pad outputs. When set to 1, the values in the OVERRIDE field are driven on the pad (output enable is implicitly set in this mode). \\[7:0\\]=data \\[8\\]=clock \\[9\\]=DM"]
pub type OverrideenW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Output pad override enable. Bit mask for pad outputs. When set to 1, the values in the OVERRIDE field are driven on the pad (output enable is implicitly set in this mode). \\[7:0\\]=data \\[8\\]=clock \\[9\\]=DM"]
    #[inline(always)]
    pub fn overrideen(&self) -> OverrideenR {
        OverrideenR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Output pad override enable. Bit mask for pad outputs. When set to 1, the values in the OVERRIDE field are driven on the pad (output enable is implicitly set in this mode). \\[7:0\\]=data \\[8\\]=clock \\[9\\]=DM"]
    #[inline(always)]
    #[must_use]
    pub fn overrideen(&mut self) -> OverrideenW<PadoverenSpec> {
        OverrideenW::new(self, 0)
    }
}
#[doc = "Enables PIO-like pad override control\n\nYou can [`read`](crate::Reg::read) this register and get [`padoveren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padoveren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadoverenSpec;
impl crate::RegisterSpec for PadoverenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padoveren::R`](R) reader structure"]
impl crate::Readable for PadoverenSpec {}
#[doc = "`write(|w| ..)` method takes [`padoveren::W`](W) writer structure"]
impl crate::Writable for PadoverenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PADOVEREN to value 0"]
impl crate::Resettable for PadoverenSpec {
    const RESET_VALUE: u32 = 0;
}
