#[doc = "Register `DSP0MBINTSTAT` reader"]
pub type R = crate::R<Dsp0mbintstatSpec>;
#[doc = "Register `DSP0MBINTSTAT` writer"]
pub type W = crate::W<Dsp0mbintstatSpec>;
#[doc = "Field `DSP0MBINTSTAT` reader - DSP 0 CPU Mailbox interrupt"]
pub type Dsp0mbintstatR = crate::FieldReader<u32>;
#[doc = "Field `DSP0MBINTSTAT` writer - DSP 0 CPU Mailbox interrupt"]
pub type Dsp0mbintstatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP 0 CPU Mailbox interrupt"]
    #[inline(always)]
    pub fn dsp0mbintstat(&self) -> Dsp0mbintstatR {
        Dsp0mbintstatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP 0 CPU Mailbox interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0mbintstat(&mut self) -> Dsp0mbintstatW<Dsp0mbintstatSpec> {
        Dsp0mbintstatW::new(self, 0)
    }
}
#[doc = "DSP 0 Mailbox Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0mbintstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0mbintstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0mbintstatSpec;
impl crate::RegisterSpec for Dsp0mbintstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0mbintstat::R`](R) reader structure"]
impl crate::Readable for Dsp0mbintstatSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0mbintstat::W`](W) writer structure"]
impl crate::Writable for Dsp0mbintstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0MBINTSTAT to value 0"]
impl crate::Resettable for Dsp0mbintstatSpec {
    const RESET_VALUE: u32 = 0;
}
