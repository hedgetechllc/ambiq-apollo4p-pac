#[doc = "Register `CQADDR` reader"]
pub type R = crate::R<CqaddrSpec>;
#[doc = "Register `CQADDR` writer"]
pub type W = crate::W<CqaddrSpec>;
#[doc = "Field `CQADDR` reader - Bits 28:2 of target byte address for source of CQ . The buffer must be aligned on a word boundary"]
pub type CqaddrR = crate::FieldReader<u32>;
#[doc = "Field `CQADDR` writer - Bits 28:2 of target byte address for source of CQ . The buffer must be aligned on a word boundary"]
pub type CqaddrW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 2:28 - Bits 28:2 of target byte address for source of CQ . The buffer must be aligned on a word boundary"]
    #[inline(always)]
    pub fn cqaddr(&self) -> CqaddrR {
        CqaddrR::new((self.bits >> 2) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:28 - Bits 28:2 of target byte address for source of CQ . The buffer must be aligned on a word boundary"]
    #[inline(always)]
    #[must_use]
    pub fn cqaddr(&mut self) -> CqaddrW<CqaddrSpec> {
        CqaddrW::new(self, 2)
    }
}
#[doc = "The SRAM address which will be fetched next execution of the CQ operation. This register is updated as the CQ operation progresses, and is the live version of the register. The register can also be written by the Command Queue operation itself, allowing the relocation of successive CQ fetches. In this case, the new CQ address will be used for the next CQ address/data fetch\n\nYou can [`read`](crate::Reg::read) this register and get [`cqaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
