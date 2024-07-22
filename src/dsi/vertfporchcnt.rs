#[doc = "Register `VERTFPORCHCNT` reader"]
pub type R = crate::R<VertfporchcntSpec>;
#[doc = "Register `VERTFPORCHCNT` writer"]
pub type W = crate::W<VertfporchcntSpec>;
#[doc = "Field `VFPSC` reader - Shows the vertical front porch value in terms of lines. Min value - 1; Max value - any 12 bit value greater than 1 based on DPI resolution"]
pub type VfpscR = crate::FieldReader<u16>;
#[doc = "Field `VFPSC` writer - Shows the vertical front porch value in terms of lines. Min value - 1; Max value - any 12 bit value greater than 1 based on DPI resolution"]
pub type VfpscW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shows the vertical front porch value in terms of lines. Min value - 1; Max value - any 12 bit value greater than 1 based on DPI resolution"]
    #[inline(always)]
    pub fn vfpsc(&self) -> VfpscR {
        VfpscR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shows the vertical front porch value in terms of lines. Min value - 1; Max value - any 12 bit value greater than 1 based on DPI resolution"]
    #[inline(always)]
    #[must_use]
    pub fn vfpsc(&mut self) -> VfpscW<VertfporchcntSpec> {
        VfpscW::new(self, 0)
    }
}
#[doc = "Shows the vertical front porch value\n\nYou can [`read`](crate::Reg::read) this register and get [`vertfporchcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vertfporchcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VertfporchcntSpec;
impl crate::RegisterSpec for VertfporchcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vertfporchcnt::R`](R) reader structure"]
impl crate::Readable for VertfporchcntSpec {}
#[doc = "`write(|w| ..)` method takes [`vertfporchcnt::W`](W) writer structure"]
impl crate::Writable for VertfporchcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VERTFPORCHCNT to value 0"]
impl crate::Resettable for VertfporchcntSpec {
    const RESET_VALUE: u32 = 0;
}
