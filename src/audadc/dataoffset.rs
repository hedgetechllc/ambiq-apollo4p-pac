#[doc = "Register `DATAOFFSET` reader"]
pub type R = crate::R<DataoffsetSpec>;
#[doc = "Register `DATAOFFSET` writer"]
pub type W = crate::W<DataoffsetSpec>;
#[doc = "Field `OFFSET` reader - Add this signed offset to data before being written to the FIFO. This enables the user to convert unsigned samples to signed or remove a DC offset on the samples. Note that this does NOT affect the comparator limits, which still operate on original unsigned samples."]
pub type OffsetR = crate::FieldReader<u16>;
#[doc = "Field `OFFSET` writer - Add this signed offset to data before being written to the FIFO. This enables the user to convert unsigned samples to signed or remove a DC offset on the samples. Note that this does NOT affect the comparator limits, which still operate on original unsigned samples."]
pub type OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Add this signed offset to data before being written to the FIFO. This enables the user to convert unsigned samples to signed or remove a DC offset on the samples. Note that this does NOT affect the comparator limits, which still operate on original unsigned samples."]
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Add this signed offset to data before being written to the FIFO. This enables the user to convert unsigned samples to signed or remove a DC offset on the samples. Note that this does NOT affect the comparator limits, which still operate on original unsigned samples."]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OffsetW<DataoffsetSpec> {
        OffsetW::new(self, 0)
    }
}
#[doc = "ERROR: reg_brief VALUE MISSING\n\nYou can [`read`](crate::Reg::read) this register and get [`dataoffset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataoffset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataoffsetSpec;
impl crate::RegisterSpec for DataoffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dataoffset::R`](R) reader structure"]
impl crate::Readable for DataoffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`dataoffset::W`](W) writer structure"]
impl crate::Writable for DataoffsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATAOFFSET to value 0"]
impl crate::Resettable for DataoffsetSpec {
    const RESET_VALUE: u32 = 0;
}
