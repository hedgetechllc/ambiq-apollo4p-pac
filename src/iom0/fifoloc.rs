#[doc = "Register `FIFOLOC` reader"]
pub type R = crate::R<FifolocSpec>;
#[doc = "Register `FIFOLOC` writer"]
pub type W = crate::W<FifolocSpec>;
#[doc = "Field `FIFOWPTR` reader - Current FIFO write pointer. Value is the index into the outgoing FIFO (FIFO0), which is used during write operations to external devices."]
pub type FifowptrR = crate::FieldReader;
#[doc = "Field `FIFOWPTR` writer - Current FIFO write pointer. Value is the index into the outgoing FIFO (FIFO0), which is used during write operations to external devices."]
pub type FifowptrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FIFORPTR` reader - Current FIFO read pointer. Used to index into the incoming FIFO (FIFO1), which is used to store read data returned from external devices during a read operation."]
pub type FiforptrR = crate::FieldReader;
#[doc = "Field `FIFORPTR` writer - Current FIFO read pointer. Used to index into the incoming FIFO (FIFO1), which is used to store read data returned from external devices during a read operation."]
pub type FiforptrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Current FIFO write pointer. Value is the index into the outgoing FIFO (FIFO0), which is used during write operations to external devices."]
    #[inline(always)]
    pub fn fifowptr(&self) -> FifowptrR {
        FifowptrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Current FIFO read pointer. Used to index into the incoming FIFO (FIFO1), which is used to store read data returned from external devices during a read operation."]
    #[inline(always)]
    pub fn fiforptr(&self) -> FiforptrR {
        FiforptrR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Current FIFO write pointer. Value is the index into the outgoing FIFO (FIFO0), which is used during write operations to external devices."]
    #[inline(always)]
    #[must_use]
    pub fn fifowptr(&mut self) -> FifowptrW<FifolocSpec> {
        FifowptrW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Current FIFO read pointer. Used to index into the incoming FIFO (FIFO1), which is used to store read data returned from external devices during a read operation."]
    #[inline(always)]
    #[must_use]
    pub fn fiforptr(&mut self) -> FiforptrW<FifolocSpec> {
        FiforptrW::new(self, 8)
    }
}
#[doc = "Provides a read only value of the current read and write pointers. This register is read only and can be used alogn with the FIFO direct access method to determine the next data to be used for input and output functions.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoloc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoloc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifolocSpec;
impl crate::RegisterSpec for FifolocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifoloc::R`](R) reader structure"]
impl crate::Readable for FifolocSpec {}
#[doc = "`write(|w| ..)` method takes [`fifoloc::W`](W) writer structure"]
impl crate::Writable for FifolocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOLOC to value 0"]
impl crate::Resettable for FifolocSpec {
    const RESET_VALUE: u32 = 0;
}
