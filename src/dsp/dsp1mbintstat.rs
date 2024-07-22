#[doc = "Register `DSP1MBINTSTAT` reader"]
pub type R = crate::R<Dsp1mbintstatSpec>;
#[doc = "Register `DSP1MBINTSTAT` writer"]
pub type W = crate::W<Dsp1mbintstatSpec>;
#[doc = "Field `DSP1MBINTSTAT` reader - DSP 1 CPU Mailbox interrupt"]
pub type Dsp1mbintstatR = crate::FieldReader<u32>;
#[doc = "Field `DSP1MBINTSTAT` writer - DSP 1 CPU Mailbox interrupt"]
pub type Dsp1mbintstatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP 1 CPU Mailbox interrupt"]
    #[inline(always)]
    pub fn dsp1mbintstat(&self) -> Dsp1mbintstatR {
        Dsp1mbintstatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP 1 CPU Mailbox interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1mbintstat(&mut self) -> Dsp1mbintstatW<Dsp1mbintstatSpec> {
        Dsp1mbintstatW::new(self, 0)
    }
}
#[doc = "DSP 1 Mailbox Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1mbintstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1mbintstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1mbintstatSpec;
impl crate::RegisterSpec for Dsp1mbintstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1mbintstat::R`](R) reader structure"]
impl crate::Readable for Dsp1mbintstatSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1mbintstat::W`](W) writer structure"]
impl crate::Writable for Dsp1mbintstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1MBINTSTAT to value 0"]
impl crate::Resettable for Dsp1mbintstatSpec {
    const RESET_VALUE: u32 = 0;
}
