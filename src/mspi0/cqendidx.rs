#[doc = "Register `CQENDIDX` reader"]
pub type R = crate::R<CqendidxSpec>;
#[doc = "Register `CQENDIDX` writer"]
pub type W = crate::W<CqendidxSpec>;
#[doc = "Field `CQENDIDX` reader - Can be used to indicate the end position of the command queue. A CQ hardware status bit indices when CURIDX != ENDIDX so that the CQ can be paused when it reaches the end pointer."]
pub type CqendidxR = crate::FieldReader;
#[doc = "Field `CQENDIDX` writer - Can be used to indicate the end position of the command queue. A CQ hardware status bit indices when CURIDX != ENDIDX so that the CQ can be paused when it reaches the end pointer."]
pub type CqendidxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Can be used to indicate the end position of the command queue. A CQ hardware status bit indices when CURIDX != ENDIDX so that the CQ can be paused when it reaches the end pointer."]
    #[inline(always)]
    pub fn cqendidx(&self) -> CqendidxR {
        CqendidxR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Can be used to indicate the end position of the command queue. A CQ hardware status bit indices when CURIDX != ENDIDX so that the CQ can be paused when it reaches the end pointer."]
    #[inline(always)]
    #[must_use]
    pub fn cqendidx(&mut self) -> CqendidxW<CqendidxSpec> {
        CqendidxW::new(self, 0)
    }
}
#[doc = "Command Queue End Index\n\nYou can [`read`](crate::Reg::read) this register and get [`cqendidx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqendidx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqendidxSpec;
impl crate::RegisterSpec for CqendidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqendidx::R`](R) reader structure"]
impl crate::Readable for CqendidxSpec {}
#[doc = "`write(|w| ..)` method takes [`cqendidx::W`](W) writer structure"]
impl crate::Writable for CqendidxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQENDIDX to value 0"]
impl crate::Resettable for CqendidxSpec {
    const RESET_VALUE: u32 = 0;
}
