#[doc = "Register `CPUDSP1MBDATA` reader"]
pub type R = crate::R<Cpudsp1mbdataSpec>;
#[doc = "Register `CPUDSP1MBDATA` writer"]
pub type W = crate::W<Cpudsp1mbdataSpec>;
#[doc = "Field `CPUDSP1MBDATA` reader - DSP 1 CPU Mailbox data"]
pub type Cpudsp1mbdataR = crate::FieldReader<u32>;
#[doc = "Field `CPUDSP1MBDATA` writer - DSP 1 CPU Mailbox data"]
pub type Cpudsp1mbdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP 1 CPU Mailbox data"]
    #[inline(always)]
    pub fn cpudsp1mbdata(&self) -> Cpudsp1mbdataR {
        Cpudsp1mbdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP 1 CPU Mailbox data"]
    #[inline(always)]
    #[must_use]
    pub fn cpudsp1mbdata(&mut self) -> Cpudsp1mbdataW<Cpudsp1mbdataSpec> {
        Cpudsp1mbdataW::new(self, 0)
    }
}
#[doc = "CPU to DSP 1 Mailbox Data\n\nYou can [`read`](crate::Reg::read) this register and get [`cpudsp1mbdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpudsp1mbdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpudsp1mbdataSpec;
impl crate::RegisterSpec for Cpudsp1mbdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpudsp1mbdata::R`](R) reader structure"]
impl crate::Readable for Cpudsp1mbdataSpec {}
#[doc = "`write(|w| ..)` method takes [`cpudsp1mbdata::W`](W) writer structure"]
impl crate::Writable for Cpudsp1mbdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUDSP1MBDATA to value 0"]
impl crate::Resettable for Cpudsp1mbdataSpec {
    const RESET_VALUE: u32 = 0;
}
