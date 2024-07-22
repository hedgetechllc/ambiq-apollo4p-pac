#[doc = "Register `FIFOPUSH` reader"]
pub type R = crate::R<FifopushSpec>;
#[doc = "Register `FIFOPUSH` writer"]
pub type W = crate::W<FifopushSpec>;
#[doc = "Field `FIFODIN` reader - This register is used to write the FIFORAM in FIFO mode and will cause a push event to occur to the next open slot within the FIFORAM. Writing to this register will cause the write point to increment by 1 word(4 bytes)."]
pub type FifodinR = crate::FieldReader<u32>;
#[doc = "Field `FIFODIN` writer - This register is used to write the FIFORAM in FIFO mode and will cause a push event to occur to the next open slot within the FIFORAM. Writing to this register will cause the write point to increment by 1 word(4 bytes)."]
pub type FifodinW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register is used to write the FIFORAM in FIFO mode and will cause a push event to occur to the next open slot within the FIFORAM. Writing to this register will cause the write point to increment by 1 word(4 bytes)."]
    #[inline(always)]
    pub fn fifodin(&self) -> FifodinR {
        FifodinR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is used to write the FIFORAM in FIFO mode and will cause a push event to occur to the next open slot within the FIFORAM. Writing to this register will cause the write point to increment by 1 word(4 bytes)."]
    #[inline(always)]
    #[must_use]
    pub fn fifodin(&mut self) -> FifodinW<FifopushSpec> {
        FifodinW::new(self, 0)
    }
}
#[doc = "Will write new data into the outgoing FIFO and advance the internal write pointer.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifopush::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifopush::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifopushSpec;
impl crate::RegisterSpec for FifopushSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifopush::R`](R) reader structure"]
impl crate::Readable for FifopushSpec {}
#[doc = "`write(|w| ..)` method takes [`fifopush::W`](W) writer structure"]
impl crate::Writable for FifopushSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOPUSH to value 0"]
impl crate::Resettable for FifopushSpec {
    const RESET_VALUE: u32 = 0;
}
