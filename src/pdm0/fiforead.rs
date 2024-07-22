#[doc = "Register `FIFOREAD` reader"]
pub type R = crate::R<FiforeadSpec>;
#[doc = "Register `FIFOREAD` writer"]
pub type W = crate::W<FiforeadSpec>;
#[doc = "Field `FIFOREAD` reader - FIFO read data."]
pub type FiforeadR = crate::FieldReader<u32>;
#[doc = "Field `FIFOREAD` writer - FIFO read data."]
pub type FiforeadW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - FIFO read data."]
    #[inline(always)]
    pub fn fiforead(&self) -> FiforeadR {
        FiforeadR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FIFO read data."]
    #[inline(always)]
    #[must_use]
    pub fn fiforead(&mut self) -> FiforeadW<FiforeadSpec> {
        FiforeadW::new(self, 0)
    }
}
#[doc = "FIFO Read\n\nYou can [`read`](crate::Reg::read) this register and get [`fiforead::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiforead::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FiforeadSpec;
impl crate::RegisterSpec for FiforeadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiforead::R`](R) reader structure"]
impl crate::Readable for FiforeadSpec {}
#[doc = "`write(|w| ..)` method takes [`fiforead::W`](W) writer structure"]
impl crate::Writable for FiforeadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOREAD to value 0"]
impl crate::Resettable for FiforeadSpec {
    const RESET_VALUE: u32 = 0;
}
