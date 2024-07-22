#[doc = "Register `FIFO` reader"]
pub type R = crate::R<FifoSpec>;
#[doc = "Register `FIFO` writer"]
pub type W = crate::W<FifoSpec>;
#[doc = "Field `FIFO` reader - FIFO direct access. Only locations 0 - 3F will return valid information."]
pub type FifoR = crate::FieldReader<u32>;
#[doc = "Field `FIFO` writer - FIFO direct access. Only locations 0 - 3F will return valid information."]
pub type FifoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - FIFO direct access. Only locations 0 - 3F will return valid information."]
    #[inline(always)]
    pub fn fifo(&self) -> FifoR {
        FifoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FIFO direct access. Only locations 0 - 3F will return valid information."]
    #[inline(always)]
    #[must_use]
    pub fn fifo(&mut self) -> FifoW<FifoSpec> {
        FifoW::new(self, 0)
    }
}
#[doc = "Provides direct random access to both input and output fifos. The state of the FIFO is not distured by reading these locations (ie no POP will be done). FIFO0 is accessible from addresses 0x0 - 0x1C, and is used for data outuput from the IOM to external devices. These FIFO locations can be read and written directly. FIFO locations 0x20 - 0x3C provide read only access to the input fifo. These FIFO locations cannot be directly written by the MCU, and are updated only by the internal hardware. Writes to the FIFO0 will take effect immediately. The currently FIFO pointers in register FIFOLOC indicate the current offset into each FIFO0 for the read and write operations. Access to the FIFOs can only be done in word increments; Byte access and writes are not supported. For push and pop style access to FIFO0 can be done using the FIFOPOP and FIFOPUSH registers below.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
