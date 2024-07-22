#[doc = "Register `DSP0DSP0MBDATA` reader"]
pub type R = crate::R<Dsp0dsp0mbdataSpec>;
#[doc = "Register `DSP0DSP0MBDATA` writer"]
pub type W = crate::W<Dsp0dsp0mbdataSpec>;
#[doc = "Field `DSP0DSP0MBDATA` reader - DSP0 to DSP 0 Mailbox data"]
pub type Dsp0dsp0mbdataR = crate::FieldReader<u32>;
#[doc = "Field `DSP0DSP0MBDATA` writer - DSP0 to DSP 0 Mailbox data"]
pub type Dsp0dsp0mbdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP0 to DSP 0 Mailbox data"]
    #[inline(always)]
    pub fn dsp0dsp0mbdata(&self) -> Dsp0dsp0mbdataR {
        Dsp0dsp0mbdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP0 to DSP 0 Mailbox data"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0dsp0mbdata(&mut self) -> Dsp0dsp0mbdataW<Dsp0dsp0mbdataSpec> {
        Dsp0dsp0mbdataW::new(self, 0)
    }
}
#[doc = "DSP0 to DSP 0 Mailbox Data\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0dsp0mbdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0dsp0mbdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0dsp0mbdataSpec;
impl crate::RegisterSpec for Dsp0dsp0mbdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0dsp0mbdata::R`](R) reader structure"]
impl crate::Readable for Dsp0dsp0mbdataSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0dsp0mbdata::W`](W) writer structure"]
impl crate::Writable for Dsp0dsp0mbdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0DSP0MBDATA to value 0"]
impl crate::Resettable for Dsp0dsp0mbdataSpec {
    const RESET_VALUE: u32 = 0;
}
