#[doc = "Register `FIFO` reader"]
pub type R = crate::R<FifoSpec>;
#[doc = "Register `FIFO` writer"]
pub type W = crate::W<FifoSpec>;
#[doc = "Field `DATA` reader - Oldest data in the FIFO."]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Oldest data in the FIFO."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `COUNT` reader - Number of valid entries in the ADC FIFO."]
pub type CountR = crate::FieldReader;
#[doc = "Field `COUNT` writer - Number of valid entries in the ADC FIFO."]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SLOTNUM` reader - Slot number associated with this FIFO data."]
pub type SlotnumR = crate::FieldReader;
#[doc = "Field `SLOTNUM` writer - Slot number associated with this FIFO data."]
pub type SlotnumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RSVD` reader - RESERVED."]
pub type RsvdR = crate::BitReader;
#[doc = "Field `RSVD` writer - RESERVED."]
pub type RsvdW<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn slotnum(&self) -> SlotnumR {
        SlotnumR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - RESERVED."]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - Oldest data in the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<FifoSpec> {
        DataW::new(self, 0)
    }
    #[doc = "Bits 20:27 - Number of valid entries in the ADC FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> CountW<FifoSpec> {
        CountW::new(self, 20)
    }
    #[doc = "Bits 28:30 - Slot number associated with this FIFO data."]
    #[inline(always)]
    #[must_use]
    pub fn slotnum(&mut self) -> SlotnumW<FifoSpec> {
        SlotnumW::new(self, 28)
    }
    #[doc = "Bit 31 - RESERVED."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd(&mut self) -> RsvdW<FifoSpec> {
        RsvdW::new(self, 31)
    }
}
#[doc = "The ADC FIFO Register contains the slot number and fifo data for the oldest conversion data in the FIFO. The COUNT field indicates the total number of valid entries in the FIFO. A write to this register will pop one of the FIFO entries off the FIFO and decrease the COUNT by 1 if the COUNT is greater than zero.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoSpec;
impl crate::RegisterSpec for FifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo::R`](R) reader structure"]
impl crate::Readable for FifoSpec {}
#[doc = "`write(|w| ..)` method takes [`fifo::W`](W) writer structure"]
impl crate::Writable for FifoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFO to value 0"]
impl crate::Resettable for FifoSpec {
    const RESET_VALUE: u32 = 0;
}
