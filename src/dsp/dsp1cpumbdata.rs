#[doc = "Register `DSP1CPUMBDATA` reader"]
pub type R = crate::R<Dsp1cpumbdataSpec>;
#[doc = "Register `DSP1CPUMBDATA` writer"]
pub type W = crate::W<Dsp1cpumbdataSpec>;
#[doc = "Field `DSP1CPUMBDATA` reader - DSP1 to CPU Mailbox data"]
pub type Dsp1cpumbdataR = crate::FieldReader<u32>;
#[doc = "Field `DSP1CPUMBDATA` writer - DSP1 to CPU Mailbox data"]
pub type Dsp1cpumbdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP1 to CPU Mailbox data"]
    #[inline(always)]
    pub fn dsp1cpumbdata(&self) -> Dsp1cpumbdataR {
        Dsp1cpumbdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP1 to CPU Mailbox data"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1cpumbdata(&mut self) -> Dsp1cpumbdataW<Dsp1cpumbdataSpec> {
        Dsp1cpumbdataW::new(self, 0)
    }
}
#[doc = "DSP1 to CPU Mailbox Data\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1cpumbdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1cpumbdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1cpumbdataSpec;
impl crate::RegisterSpec for Dsp1cpumbdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1cpumbdata::R`](R) reader structure"]
impl crate::Readable for Dsp1cpumbdataSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1cpumbdata::W`](W) writer structure"]
impl crate::Writable for Dsp1cpumbdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1CPUMBDATA to value 0"]
impl crate::Resettable for Dsp1cpumbdataSpec {
    const RESET_VALUE: u32 = 0;
}
