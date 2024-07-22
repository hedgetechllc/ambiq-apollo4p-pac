#[doc = "Register `FIFOINC` reader"]
pub type R = crate::R<FifoincSpec>;
#[doc = "Register `FIFOINC` writer"]
pub type W = crate::W<FifoincSpec>;
#[doc = "Field `FIFOINC` reader - Increment the Overall FIFO Counter by this value on a write"]
pub type FifoincR = crate::FieldReader<u16>;
#[doc = "Field `FIFOINC` writer - Increment the Overall FIFO Counter by this value on a write"]
pub type FifoincW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Increment the Overall FIFO Counter by this value on a write"]
    #[inline(always)]
    pub fn fifoinc(&self) -> FifoincR {
        FifoincR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Increment the Overall FIFO Counter by this value on a write"]
    #[inline(always)]
    #[must_use]
    pub fn fifoinc(&mut self) -> FifoincW<FifoincSpec> {
        FifoincW::new(self, 0)
    }
}
#[doc = "Overall FIFO Counter Increment\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoinc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoinc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoincSpec;
impl crate::RegisterSpec for FifoincSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifoinc::R`](R) reader structure"]
impl crate::Readable for FifoincSpec {}
#[doc = "`write(|w| ..)` method takes [`fifoinc::W`](W) writer structure"]
impl crate::Writable for FifoincSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOINC to value 0"]
impl crate::Resettable for FifoincSpec {
    const RESET_VALUE: u32 = 0;
}
