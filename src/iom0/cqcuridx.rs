#[doc = "Register `CQCURIDX` reader"]
pub type R = crate::R<CqcuridxSpec>;
#[doc = "Register `CQCURIDX` writer"]
pub type W = crate::W<CqcuridxSpec>;
#[doc = "Field `CQCURIDX` reader - Holds 8 bits of data that will be compared with the CQENDIX register field. If the values match, the IDXEQ pause event will be activated, which will cause the pausing of command quue operation if the IDXEQ bit is enabled in CQPAUSEEN."]
pub type CqcuridxR = crate::FieldReader;
#[doc = "Field `CQCURIDX` writer - Holds 8 bits of data that will be compared with the CQENDIX register field. If the values match, the IDXEQ pause event will be activated, which will cause the pausing of command quue operation if the IDXEQ bit is enabled in CQPAUSEEN."]
pub type CqcuridxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Holds 8 bits of data that will be compared with the CQENDIX register field. If the values match, the IDXEQ pause event will be activated, which will cause the pausing of command quue operation if the IDXEQ bit is enabled in CQPAUSEEN."]
    #[inline(always)]
    pub fn cqcuridx(&self) -> CqcuridxR {
        CqcuridxR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Holds 8 bits of data that will be compared with the CQENDIX register field. If the values match, the IDXEQ pause event will be activated, which will cause the pausing of command quue operation if the IDXEQ bit is enabled in CQPAUSEEN."]
    #[inline(always)]
    #[must_use]
    pub fn cqcuridx(&mut self) -> CqcuridxW<CqcuridxSpec> {
        CqcuridxW::new(self, 0)
    }
}
#[doc = "Current index value, targeted to be written by register write operations within the command queue. This is compared to the CQENDIDX and will stop the CQ operation if bit 15 of the CQPAUSEEN is '1' and this current index equals the CQENDIDX register value. This will only pause when the values are equal.\n\nYou can [`read`](crate::Reg::read) this register and get [`cqcuridx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqcuridx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
