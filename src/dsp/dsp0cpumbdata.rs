#[doc = "Register `DSP0CPUMBDATA` reader"]
pub type R = crate::R<Dsp0cpumbdataSpec>;
#[doc = "Register `DSP0CPUMBDATA` writer"]
pub type W = crate::W<Dsp0cpumbdataSpec>;
#[doc = "Field `DSP0CPUMBDATA` reader - DSP0 to CPU Mailbox data"]
pub type Dsp0cpumbdataR = crate::FieldReader<u32>;
#[doc = "Field `DSP0CPUMBDATA` writer - DSP0 to CPU Mailbox data"]
pub type Dsp0cpumbdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP0 to CPU Mailbox data"]
    #[inline(always)]
    pub fn dsp0cpumbdata(&self) -> Dsp0cpumbdataR {
        Dsp0cpumbdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP0 to CPU Mailbox data"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0cpumbdata(&mut self) -> Dsp0cpumbdataW<Dsp0cpumbdataSpec> {
        Dsp0cpumbdataW::new(self, 0)
    }
}
#[doc = "DSP0 to CPU Mailbox Data\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0cpumbdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0cpumbdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0cpumbdataSpec;
impl crate::RegisterSpec for Dsp0cpumbdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0cpumbdata::R`](R) reader structure"]
impl crate::Readable for Dsp0cpumbdataSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0cpumbdata::W`](W) writer structure"]
impl crate::Writable for Dsp0cpumbdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0CPUMBDATA to value 0"]
impl crate::Resettable for Dsp0cpumbdataSpec {
    const RESET_VALUE: u32 = 0;
}
