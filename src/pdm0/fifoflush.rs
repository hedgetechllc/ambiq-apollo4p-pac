#[doc = "Register `FIFOFLUSH` reader"]
pub type R = crate::R<FifoflushSpec>;
#[doc = "Register `FIFOFLUSH` writer"]
pub type W = crate::W<FifoflushSpec>;
#[doc = "Field `FIFOFLUSH` reader - FIFO FLUSH."]
pub type FifoflushR = crate::BitReader;
#[doc = "Field `FIFOFLUSH` writer - FIFO FLUSH."]
pub type FifoflushW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FIFO FLUSH."]
    #[inline(always)]
    pub fn fifoflush(&self) -> FifoflushR {
        FifoflushR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO FLUSH."]
    #[inline(always)]
    #[must_use]
    pub fn fifoflush(&mut self) -> FifoflushW<FifoflushSpec> {
        FifoflushW::new(self, 0)
    }
}
#[doc = "FIFO Flush\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoflush::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoflush::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoflushSpec;
impl crate::RegisterSpec for FifoflushSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifoflush::R`](R) reader structure"]
impl crate::Readable for FifoflushSpec {}
#[doc = "`write(|w| ..)` method takes [`fifoflush::W`](W) writer structure"]
impl crate::Writable for FifoflushSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOFLUSH to value 0"]
impl crate::Resettable for FifoflushSpec {
    const RESET_VALUE: u32 = 0;
}
