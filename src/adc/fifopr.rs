#[doc = "Register `FIFOPR` reader"]
pub type R = crate::R<FifoprSpec>;
#[doc = "Register `FIFOPR` writer"]
pub type W = crate::W<FifoprSpec>;
#[doc = "Field `DATA` reader - Oldest data in the FIFO."]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Oldest data in the FIFO."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `COUNT` reader - Number of valid entries in the ADC FIFO."]
pub type CountR = crate::FieldReader;
#[doc = "Field `COUNT` writer - Number of valid entries in the ADC FIFO."]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SLOTNUMPR` reader - Slot number associated with this FIFO data."]
pub type SlotnumprR = crate::FieldReader;
#[doc = "Field `SLOTNUMPR` writer - Slot number associated with this FIFO data."]
pub type SlotnumprW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RSVDPR` reader - RESERVED."]
pub type RsvdprR = crate::BitReader;
#[doc = "Field `RSVDPR` writer - RESERVED."]
pub type RsvdprW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:19 - Oldest data in the FIFO."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:27 - Number of valid entries in the ADC FIFO."]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:30 - Slot number associated with this FIFO data."]
    #[inline(always)]
    pub fn slotnumpr(&self) -> SlotnumprR {
        SlotnumprR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - RESERVED."]
    #[inline(always)]
    pub fn rsvdpr(&self) -> RsvdprR {
        RsvdprR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - Oldest data in the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<FifoprSpec> {
        DataW::new(self, 0)
    }
    #[doc = "Bits 20:27 - Number of valid entries in the ADC FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> CountW<FifoprSpec> {
        CountW::new(self, 20)
    }
    #[doc = "Bits 28:30 - Slot number associated with this FIFO data."]
    #[inline(always)]
    #[must_use]
    pub fn slotnumpr(&mut self) -> SlotnumprW<FifoprSpec> {
        SlotnumprW::new(self, 28)
    }
    #[doc = "Bit 31 - RESERVED."]
    #[inline(always)]
    #[must_use]
    pub fn rsvdpr(&mut self) -> RsvdprW<FifoprSpec> {
        RsvdprW::new(self, 31)
    }
}
#[doc = "This is a Pop Read mirrored copy of the ADCFIFO register with the only difference being that reading this register will result in a simultaneous FIFO POP which is also achieved by writing to the ADCFIFO Register. Note: The DFIFORDEN bit must be set in the CFG register for the the destructive read to be enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifopr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifopr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoprSpec;
impl crate::RegisterSpec for FifoprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifopr::R`](R) reader structure"]
impl crate::Readable for FifoprSpec {}
#[doc = "`write(|w| ..)` method takes [`fifopr::W`](W) writer structure"]
impl crate::Writable for FifoprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOPR to value 0"]
impl crate::Resettable for FifoprSpec {
    const RESET_VALUE: u32 = 0;
}
