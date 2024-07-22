#[doc = "Register `FIFOCTRL` reader"]
pub type R = crate::R<FifoctrlSpec>;
#[doc = "Register `FIFOCTRL` writer"]
pub type W = crate::W<FifoctrlSpec>;
#[doc = "Field `POPWR` reader - Selects the mode in which 'pop' events are done for the fifo read operations. A value of '1' will prevent a pop event on a read operation, and will require a write to the FIFOPOP register to create a pop event. A value of '0' in this register will allow a pop event to occur on the read of the FIFOPOP register, and may cause inadvertant fifo pops when used in a debugging mode."]
pub type PopwrR = crate::BitReader;
#[doc = "Field `POPWR` writer - Selects the mode in which 'pop' events are done for the fifo read operations. A value of '1' will prevent a pop event on a read operation, and will require a write to the FIFOPOP register to create a pop event. A value of '0' in this register will allow a pop event to occur on the read of the FIFOPOP register, and may cause inadvertant fifo pops when used in a debugging mode."]
pub type PopwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFORSTN` reader - Active low manual reset of the fifo. Write to 0 to reset fifo, and then write to 1 to remove the reset."]
pub type FiforstnR = crate::BitReader;
#[doc = "Field `FIFORSTN` writer - Active low manual reset of the fifo. Write to 0 to reset fifo, and then write to 1 to remove the reset."]
pub type FiforstnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Selects the mode in which 'pop' events are done for the fifo read operations. A value of '1' will prevent a pop event on a read operation, and will require a write to the FIFOPOP register to create a pop event. A value of '0' in this register will allow a pop event to occur on the read of the FIFOPOP register, and may cause inadvertant fifo pops when used in a debugging mode."]
    #[inline(always)]
    pub fn popwr(&self) -> PopwrR {
        PopwrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Active low manual reset of the fifo. Write to 0 to reset fifo, and then write to 1 to remove the reset."]
    #[inline(always)]
    pub fn fiforstn(&self) -> FiforstnR {
        FiforstnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the mode in which 'pop' events are done for the fifo read operations. A value of '1' will prevent a pop event on a read operation, and will require a write to the FIFOPOP register to create a pop event. A value of '0' in this register will allow a pop event to occur on the read of the FIFOPOP register, and may cause inadvertant fifo pops when used in a debugging mode."]
    #[inline(always)]
    #[must_use]
    pub fn popwr(&mut self) -> PopwrW<FifoctrlSpec> {
        PopwrW::new(self, 0)
    }
    #[doc = "Bit 1 - Active low manual reset of the fifo. Write to 0 to reset fifo, and then write to 1 to remove the reset."]
    #[inline(always)]
    #[must_use]
    pub fn fiforstn(&mut self) -> FiforstnW<FifoctrlSpec> {
        FiforstnW::new(self, 1)
    }
}
#[doc = "Provides controls for the operation of the internal FIFOs. Contains fields used to control the operation of the POP register, and also controls to reset the internal pointers of the FIFOs.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoctrlSpec;
impl crate::RegisterSpec for FifoctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifoctrl::R`](R) reader structure"]
impl crate::Readable for FifoctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`fifoctrl::W`](W) writer structure"]
impl crate::Writable for FifoctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOCTRL to value 0x02"]
impl crate::Resettable for FifoctrlSpec {
    const RESET_VALUE: u32 = 0x02;
}
