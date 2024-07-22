#[doc = "Register `DSP1DSP1MBDATA` reader"]
pub type R = crate::R<Dsp1dsp1mbdataSpec>;
#[doc = "Register `DSP1DSP1MBDATA` writer"]
pub type W = crate::W<Dsp1dsp1mbdataSpec>;
#[doc = "Field `DSP1DSP1MBDATA` reader - DSP1 to DSP 1 Mailbox data"]
pub type Dsp1dsp1mbdataR = crate::FieldReader<u32>;
#[doc = "Field `DSP1DSP1MBDATA` writer - DSP1 to DSP 1 Mailbox data"]
pub type Dsp1dsp1mbdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP1 to DSP 1 Mailbox data"]
    #[inline(always)]
    pub fn dsp1dsp1mbdata(&self) -> Dsp1dsp1mbdataR {
        Dsp1dsp1mbdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP1 to DSP 1 Mailbox data"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1dsp1mbdata(&mut self) -> Dsp1dsp1mbdataW<Dsp1dsp1mbdataSpec> {
        Dsp1dsp1mbdataW::new(self, 0)
    }
}
#[doc = "DSP1 to DSP 1 Mailbox Data\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1dsp1mbdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1dsp1mbdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1dsp1mbdataSpec;
impl crate::RegisterSpec for Dsp1dsp1mbdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1dsp1mbdata::R`](R) reader structure"]
impl crate::Readable for Dsp1dsp1mbdataSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1dsp1mbdata::W`](W) writer structure"]
impl crate::Writable for Dsp1dsp1mbdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1DSP1MBDATA to value 0"]
impl crate::Resettable for Dsp1dsp1mbdataSpec {
    const RESET_VALUE: u32 = 0;
}
