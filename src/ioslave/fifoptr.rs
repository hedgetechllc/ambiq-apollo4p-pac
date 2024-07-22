#[doc = "Register `FIFOPTR` reader"]
pub type R = crate::R<FifoptrSpec>;
#[doc = "Register `FIFOPTR` writer"]
pub type W = crate::W<FifoptrSpec>;
#[doc = "Field `FIFOPTR` reader - Current FIFO pointer."]
pub type FifoptrR = crate::FieldReader;
#[doc = "Field `FIFOPTR` writer - Current FIFO pointer."]
pub type FifoptrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FIFOSIZ` reader - The number of bytes currently in the hardware FIFO."]
pub type FifosizR = crate::FieldReader;
#[doc = "Field `FIFOSIZ` writer - The number of bytes currently in the hardware FIFO."]
pub type FifosizW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Current FIFO pointer."]
    #[inline(always)]
    pub fn fifoptr(&self) -> FifoptrR {
        FifoptrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The number of bytes currently in the hardware FIFO."]
    #[inline(always)]
    pub fn fifosiz(&self) -> FifosizR {
        FifosizR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Current FIFO pointer."]
    #[inline(always)]
    #[must_use]
    pub fn fifoptr(&mut self) -> FifoptrW<FifoptrSpec> {
        FifoptrW::new(self, 0)
    }
    #[doc = "Bits 8:15 - The number of bytes currently in the hardware FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn fifosiz(&mut self) -> FifosizW<FifoptrSpec> {
        FifosizW::new(self, 8)
    }
}
#[doc = "Current FIFO Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoptrSpec;
impl crate::RegisterSpec for FifoptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifoptr::R`](R) reader structure"]
impl crate::Readable for FifoptrSpec {}
#[doc = "`write(|w| ..)` method takes [`fifoptr::W`](W) writer structure"]
impl crate::Writable for FifoptrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOPTR to value 0"]
impl crate::Resettable for FifoptrSpec {
    const RESET_VALUE: u32 = 0;
}
