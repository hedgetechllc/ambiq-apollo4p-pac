#[doc = "Register `VERTBKPORCHCNT` reader"]
pub type R = crate::R<VertbkporchcntSpec>;
#[doc = "Register `VERTBKPORCHCNT` writer"]
pub type W = crate::W<VertbkporchcntSpec>;
#[doc = "Field `VBPSC` reader - Shows the vertical back porch value in terms of lines. Min value - 1; Max value - any 12 bit value greater than 1 based on DPI resolution"]
pub type VbpscR = crate::FieldReader<u16>;
#[doc = "Field `VBPSC` writer - Shows the vertical back porch value in terms of lines. Min value - 1; Max value - any 12 bit value greater than 1 based on DPI resolution"]
pub type VbpscW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shows the vertical back porch value in terms of lines. Min value - 1; Max value - any 12 bit value greater than 1 based on DPI resolution"]
    #[inline(always)]
    pub fn vbpsc(&self) -> VbpscR {
        VbpscR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shows the vertical back porch value in terms of lines. Min value - 1; Max value - any 12 bit value greater than 1 based on DPI resolution"]
    #[inline(always)]
    #[must_use]
    pub fn vbpsc(&mut self) -> VbpscW<VertbkporchcntSpec> {
        VbpscW::new(self, 0)
    }
}
#[doc = "Shows the vertical back porch value\n\nYou can [`read`](crate::Reg::read) this register and get [`vertbkporchcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vertbkporchcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VertbkporchcntSpec;
impl crate::RegisterSpec for VertbkporchcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vertbkporchcnt::R`](R) reader structure"]
impl crate::Readable for VertbkporchcntSpec {}
#[doc = "`write(|w| ..)` method takes [`vertbkporchcnt::W`](W) writer structure"]
impl crate::Writable for VertbkporchcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VERTBKPORCHCNT to value 0"]
impl crate::Resettable for VertbkporchcntSpec {
    const RESET_VALUE: u32 = 0;
}
