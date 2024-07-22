#[doc = "Register `CQCURIDX` reader"]
pub type R = crate::R<CqcuridxSpec>;
#[doc = "Register `CQCURIDX` writer"]
pub type W = crate::W<CqcuridxSpec>;
#[doc = "Field `CQCURIDX` reader - Can be used to indicate the current position of the command queue by having CQ operations write this field. A CQ hardware status flag indicates when CURIDX and ENDIDX are not equal, allowing SW to pause the CQ processing until the end index is updated."]
pub type CqcuridxR = crate::FieldReader;
#[doc = "Field `CQCURIDX` writer - Can be used to indicate the current position of the command queue by having CQ operations write this field. A CQ hardware status flag indicates when CURIDX and ENDIDX are not equal, allowing SW to pause the CQ processing until the end index is updated."]
pub type CqcuridxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Can be used to indicate the current position of the command queue by having CQ operations write this field. A CQ hardware status flag indicates when CURIDX and ENDIDX are not equal, allowing SW to pause the CQ processing until the end index is updated."]
    #[inline(always)]
    pub fn cqcuridx(&self) -> CqcuridxR {
        CqcuridxR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Can be used to indicate the current position of the command queue by having CQ operations write this field. A CQ hardware status flag indicates when CURIDX and ENDIDX are not equal, allowing SW to pause the CQ processing until the end index is updated."]
    #[inline(always)]
    #[must_use]
    pub fn cqcuridx(&mut self) -> CqcuridxW<CqcuridxSpec> {
        CqcuridxW::new(self, 0)
    }
}
#[doc = "This register can be used in conjunction with the CQENDIDX register to manage the command queue. Typically software will initialize the CQCURIDX and CQENDIDX to the same value, which will cause the CQ to be paused when enabled. Software may then add entries to the command queue (in SRAM) and update CQENDIDX. The command queue operations will then increment CQCURIDX as it processes operations. Once CQCURIDX==CQENDIDX, the command queue hardware will automatically pause since no additional operations have been appended to the queue.\n\nYou can [`read`](crate::Reg::read) this register and get [`cqcuridx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqcuridx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqcuridxSpec;
impl crate::RegisterSpec for CqcuridxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqcuridx::R`](R) reader structure"]
impl crate::Readable for CqcuridxSpec {}
#[doc = "`write(|w| ..)` method takes [`cqcuridx::W`](W) writer structure"]
impl crate::Writable for CqcuridxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQCURIDX to value 0"]
impl crate::Resettable for CqcuridxSpec {
    const RESET_VALUE: u32 = 0;
}
