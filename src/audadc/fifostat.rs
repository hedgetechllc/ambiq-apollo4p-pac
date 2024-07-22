#[doc = "Register `FIFOSTAT` reader"]
pub type R = crate::R<FifostatSpec>;
#[doc = "Register `FIFOSTAT` writer"]
pub type W = crate::W<FifostatSpec>;
#[doc = "Field `FIFOCNT` reader - Number of valid entries in the AUDADC FIFO."]
pub type FifocntR = crate::FieldReader;
#[doc = "Field `FIFOCNT` writer - Number of valid entries in the AUDADC FIFO."]
pub type FifocntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Number of valid entries in the AUDADC FIFO."]
    #[inline(always)]
    pub fn fifocnt(&self) -> FifocntR {
        FifocntR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of valid entries in the AUDADC FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn fifocnt(&mut self) -> FifocntW<FifostatSpec> {
        FifocntW::new(self, 0)
    }
}
#[doc = "This register contains status of the data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifostat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifostat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifostatSpec;
impl crate::RegisterSpec for FifostatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifostat::R`](R) reader structure"]
impl crate::Readable for FifostatSpec {}
#[doc = "`write(|w| ..)` method takes [`fifostat::W`](W) writer structure"]
impl crate::Writable for FifostatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOSTAT to value 0"]
impl crate::Resettable for FifostatSpec {
    const RESET_VALUE: u32 = 0;
}
