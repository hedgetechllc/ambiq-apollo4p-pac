#[doc = "Register `DSP0DSP1MBDATA` reader"]
pub type R = crate::R<Dsp0dsp1mbdataSpec>;
#[doc = "Register `DSP0DSP1MBDATA` writer"]
pub type W = crate::W<Dsp0dsp1mbdataSpec>;
#[doc = "Field `DSP0DSP1MBDATA` reader - DSP0 to DSP 1 Mailbox data"]
pub type Dsp0dsp1mbdataR = crate::FieldReader<u32>;
#[doc = "Field `DSP0DSP1MBDATA` writer - DSP0 to DSP 1 Mailbox data"]
pub type Dsp0dsp1mbdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP0 to DSP 1 Mailbox data"]
    #[inline(always)]
    pub fn dsp0dsp1mbdata(&self) -> Dsp0dsp1mbdataR {
        Dsp0dsp1mbdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP0 to DSP 1 Mailbox data"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0dsp1mbdata(&mut self) -> Dsp0dsp1mbdataW<Dsp0dsp1mbdataSpec> {
        Dsp0dsp1mbdataW::new(self, 0)
    }
}
#[doc = "DSP0 to DSP 1 Mailbox Data\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0dsp1mbdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0dsp1mbdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0dsp1mbdataSpec;
impl crate::RegisterSpec for Dsp0dsp1mbdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0dsp1mbdata::R`](R) reader structure"]
impl crate::Readable for Dsp0dsp1mbdataSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0dsp1mbdata::W`](W) writer structure"]
impl crate::Writable for Dsp0dsp1mbdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0DSP1MBDATA to value 0"]
impl crate::Resettable for Dsp0dsp1mbdataSpec {
    const RESET_VALUE: u32 = 0;
}
