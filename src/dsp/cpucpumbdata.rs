#[doc = "Register `CPUCPUMBDATA` reader"]
pub type R = crate::R<CpucpumbdataSpec>;
#[doc = "Register `CPUCPUMBDATA` writer"]
pub type W = crate::W<CpucpumbdataSpec>;
#[doc = "Field `CPUCPUMBDATA` reader - CPU CPU Mailbox data"]
pub type CpucpumbdataR = crate::FieldReader<u32>;
#[doc = "Field `CPUCPUMBDATA` writer - CPU CPU Mailbox data"]
pub type CpucpumbdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CPU CPU Mailbox data"]
    #[inline(always)]
    pub fn cpucpumbdata(&self) -> CpucpumbdataR {
        CpucpumbdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CPU CPU Mailbox data"]
    #[inline(always)]
    #[must_use]
    pub fn cpucpumbdata(&mut self) -> CpucpumbdataW<CpucpumbdataSpec> {
        CpucpumbdataW::new(self, 0)
    }
}
#[doc = "CPU CPU Mailbox Data\n\nYou can [`read`](crate::Reg::read) this register and get [`cpucpumbdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpucpumbdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpucpumbdataSpec;
impl crate::RegisterSpec for CpucpumbdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpucpumbdata::R`](R) reader structure"]
impl crate::Readable for CpucpumbdataSpec {}
#[doc = "`write(|w| ..)` method takes [`cpucpumbdata::W`](W) writer structure"]
impl crate::Writable for CpucpumbdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUCPUMBDATA to value 0"]
impl crate::Resettable for CpucpumbdataSpec {
    const RESET_VALUE: u32 = 0;
}
