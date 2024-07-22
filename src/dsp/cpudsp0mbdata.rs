#[doc = "Register `CPUDSP0MBDATA` reader"]
pub type R = crate::R<Cpudsp0mbdataSpec>;
#[doc = "Register `CPUDSP0MBDATA` writer"]
pub type W = crate::W<Cpudsp0mbdataSpec>;
#[doc = "Field `CPUDSP0MBDATA` reader - DSP 0 CPU Mailbox data"]
pub type Cpudsp0mbdataR = crate::FieldReader<u32>;
#[doc = "Field `CPUDSP0MBDATA` writer - DSP 0 CPU Mailbox data"]
pub type Cpudsp0mbdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP 0 CPU Mailbox data"]
    #[inline(always)]
    pub fn cpudsp0mbdata(&self) -> Cpudsp0mbdataR {
        Cpudsp0mbdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP 0 CPU Mailbox data"]
    #[inline(always)]
    #[must_use]
    pub fn cpudsp0mbdata(&mut self) -> Cpudsp0mbdataW<Cpudsp0mbdataSpec> {
        Cpudsp0mbdataW::new(self, 0)
    }
}
#[doc = "CPU to DSP 0 Mailbox Data\n\nYou can [`read`](crate::Reg::read) this register and get [`cpudsp0mbdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpudsp0mbdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpudsp0mbdataSpec;
impl crate::RegisterSpec for Cpudsp0mbdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpudsp0mbdata::R`](R) reader structure"]
impl crate::Readable for Cpudsp0mbdataSpec {}
#[doc = "`write(|w| ..)` method takes [`cpudsp0mbdata::W`](W) writer structure"]
impl crate::Writable for Cpudsp0mbdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUDSP0MBDATA to value 0"]
impl crate::Resettable for Cpudsp0mbdataSpec {
    const RESET_VALUE: u32 = 0;
}
