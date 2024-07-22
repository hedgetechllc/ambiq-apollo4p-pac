#[doc = "Register `INSTR` reader"]
pub type R = crate::R<InstrSpec>;
#[doc = "Register `INSTR` writer"]
pub type W = crate::W<InstrSpec>;
#[doc = "Field `INSTR` reader - Optional Instruction field to send (1st byte) - qualified by ISEND/ISIZE"]
pub type InstrR = crate::FieldReader<u16>;
#[doc = "Field `INSTR` writer - Optional Instruction field to send (1st byte) - qualified by ISEND/ISIZE"]
pub type InstrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Optional Instruction field to send (1st byte) - qualified by ISEND/ISIZE"]
    #[inline(always)]
    pub fn instr(&self) -> InstrR {
        InstrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Optional Instruction field to send (1st byte) - qualified by ISEND/ISIZE"]
    #[inline(always)]
    #[must_use]
    pub fn instr(&mut self) -> InstrW<InstrSpec> {
        InstrW::new(self, 0)
    }
}
#[doc = "Optional Instruction field to send for PIO transfers\n\nYou can [`read`](crate::Reg::read) this register and get [`instr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`instr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InstrSpec;
impl crate::RegisterSpec for InstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`instr::R`](R) reader structure"]
impl crate::Readable for InstrSpec {}
#[doc = "`write(|w| ..)` method takes [`instr::W`](W) writer structure"]
impl crate::Writable for InstrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INSTR to value 0"]
impl crate::Resettable for InstrSpec {
    const RESET_VALUE: u32 = 0;
}
