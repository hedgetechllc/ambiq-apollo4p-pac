#[doc = "Register `CQADDR` reader"]
pub type R = crate::R<CqaddrSpec>;
#[doc = "Register `CQADDR` writer"]
pub type W = crate::W<CqaddrSpec>;
#[doc = "Field `CQADDR` reader - Address of command queue buffer in SRAM or flash. The buffer address must be aligned to a word boundary."]
pub type CqaddrR = crate::FieldReader<u32>;
#[doc = "Field `CQADDR` writer - Address of command queue buffer in SRAM or flash. The buffer address must be aligned to a word boundary."]
pub type CqaddrW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - Address of command queue buffer in SRAM or flash. The buffer address must be aligned to a word boundary."]
    #[inline(always)]
    pub fn cqaddr(&self) -> CqaddrR {
        CqaddrR::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - Address of command queue buffer in SRAM or flash. The buffer address must be aligned to a word boundary."]
    #[inline(always)]
    #[must_use]
    pub fn cqaddr(&mut self) -> CqaddrW<CqaddrSpec> {
        CqaddrW::new(self, 0)
    }
}
#[doc = "Location of the command queue in SRAM or flash memory. This register will increment as CQ operations commence. Software should only write CQADDR when CQEN is disabled, however the command queue script itself may update CQADDR in order to perform queue management functions (like resetting the pointers)\n\nYou can [`read`](crate::Reg::read) this register and get [`cqaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqaddrSpec;
impl crate::RegisterSpec for CqaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqaddr::R`](R) reader structure"]
impl crate::Readable for CqaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`cqaddr::W`](W) writer structure"]
impl crate::Writable for CqaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQADDR to value 0"]
impl crate::Resettable for CqaddrSpec {
    const RESET_VALUE: u32 = 0;
}
