#[doc = "Register `FIFOPOP` reader"]
pub type R = crate::R<FifopopSpec>;
#[doc = "Register `FIFOPOP` writer"]
pub type W = crate::W<FifopopSpec>;
#[doc = "Field `FIFODOUT` reader - This register will return the read data indicated by the current read pointer on reads. If the POPWR control bit in the FIFOCTRL register is reset (0), the fifo read pointer will be advanced by one word as a result of the read. If the POPWR bit is set (1), the fifo read pointer will only be advanced after a write operation to this register. The write data is ignored for this register. If less than a even word multiple is available, and the command is completed, the module will return the word containing these bytes and undetermined data in the unused fields of the word."]
pub type FifodoutR = crate::FieldReader<u32>;
#[doc = "Field `FIFODOUT` writer - This register will return the read data indicated by the current read pointer on reads. If the POPWR control bit in the FIFOCTRL register is reset (0), the fifo read pointer will be advanced by one word as a result of the read. If the POPWR bit is set (1), the fifo read pointer will only be advanced after a write operation to this register. The write data is ignored for this register. If less than a even word multiple is available, and the command is completed, the module will return the word containing these bytes and undetermined data in the unused fields of the word."]
pub type FifodoutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register will return the read data indicated by the current read pointer on reads. If the POPWR control bit in the FIFOCTRL register is reset (0), the fifo read pointer will be advanced by one word as a result of the read. If the POPWR bit is set (1), the fifo read pointer will only be advanced after a write operation to this register. The write data is ignored for this register. If less than a even word multiple is available, and the command is completed, the module will return the word containing these bytes and undetermined data in the unused fields of the word."]
    #[inline(always)]
    pub fn fifodout(&self) -> FifodoutR {
        FifodoutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register will return the read data indicated by the current read pointer on reads. If the POPWR control bit in the FIFOCTRL register is reset (0), the fifo read pointer will be advanced by one word as a result of the read. If the POPWR bit is set (1), the fifo read pointer will only be advanced after a write operation to this register. The write data is ignored for this register. If less than a even word multiple is available, and the command is completed, the module will return the word containing these bytes and undetermined data in the unused fields of the word."]
    #[inline(always)]
    #[must_use]
    pub fn fifodout(&mut self) -> FifodoutW<FifopopSpec> {
        FifodoutW::new(self, 0)
    }
}
#[doc = "Will advance the internal read pointer of the incoming FIFO (FIFO1) when read, if POPWR is not active. If POPWR is active, a write to this register is needed to advance the internal FIFO pointer.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifopop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifopop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifopopSpec;
impl crate::RegisterSpec for FifopopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifopop::R`](R) reader structure"]
impl crate::Readable for FifopopSpec {}
#[doc = "`write(|w| ..)` method takes [`fifopop::W`](W) writer structure"]
impl crate::Writable for FifopopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOPOP to value 0"]
impl crate::Resettable for FifopopSpec {
    const RESET_VALUE: u32 = 0;
}
