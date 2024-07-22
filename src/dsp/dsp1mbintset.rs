#[doc = "Register `DSP1MBINTSET` reader"]
pub type R = crate::R<Dsp1mbintsetSpec>;
#[doc = "Register `DSP1MBINTSET` writer"]
pub type W = crate::W<Dsp1mbintsetSpec>;
#[doc = "Field `DSP1MBINTSET` reader - DSP1 Mailbox interrupt Set. The corresponding data bit will set the interrupt."]
pub type Dsp1mbintsetR = crate::FieldReader<u32>;
#[doc = "Field `DSP1MBINTSET` writer - DSP1 Mailbox interrupt Set. The corresponding data bit will set the interrupt."]
pub type Dsp1mbintsetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP1 Mailbox interrupt Set. The corresponding data bit will set the interrupt."]
    #[inline(always)]
    pub fn dsp1mbintset(&self) -> Dsp1mbintsetR {
        Dsp1mbintsetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP1 Mailbox interrupt Set. The corresponding data bit will set the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dsp1mbintset(&mut self) -> Dsp1mbintsetW<Dsp1mbintsetSpec> {
        Dsp1mbintsetW::new(self, 0)
    }
}
#[doc = "DSP1 Mailbox Interrupt Set\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1mbintset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1mbintset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1mbintsetSpec;
impl crate::RegisterSpec for Dsp1mbintsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1mbintset::R`](R) reader structure"]
impl crate::Readable for Dsp1mbintsetSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1mbintset::W`](W) writer structure"]
impl crate::Writable for Dsp1mbintsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1MBINTSET to value 0"]
impl crate::Resettable for Dsp1mbintsetSpec {
    const RESET_VALUE: u32 = 0;
}
