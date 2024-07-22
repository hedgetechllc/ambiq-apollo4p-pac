#[doc = "Register `DSP1MBINTCLR` reader"]
pub type R = crate::R<Dsp1mbintclrSpec>;
#[doc = "Register `DSP1MBINTCLR` writer"]
pub type W = crate::W<Dsp1mbintclrSpec>;
#[doc = "Field `DSP1MBINTCLR` reader - DSP1 Mailbox interrupt Clear. The corresponding data bit will clear the interrupt."]
pub type Dsp1mbintclrR = crate::FieldReader<u32>;
#[doc = "Field `DSP1MBINTCLR` writer - DSP1 Mailbox interrupt Clear. The corresponding data bit will clear the interrupt."]
pub type Dsp1mbintclrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP1 Mailbox interrupt Clear. The corresponding data bit will clear the interrupt."]
    #[inline(always)]
    pub fn dsp1mbintclr(&self) -> Dsp1mbintclrR {
        Dsp1mbintclrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP1 Mailbox interrupt Clear. The corresponding data bit will clear the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dsp1mbintclr(&mut self) -> Dsp1mbintclrW<Dsp1mbintclrSpec> {
        Dsp1mbintclrW::new(self, 0)
    }
}
#[doc = "DSP1 Mailbox Interrupt Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1mbintclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1mbintclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1mbintclrSpec;
impl crate::RegisterSpec for Dsp1mbintclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1mbintclr::R`](R) reader structure"]
impl crate::Readable for Dsp1mbintclrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1mbintclr::W`](W) writer structure"]
impl crate::Writable for Dsp1mbintclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1MBINTCLR to value 0"]
impl crate::Resettable for Dsp1mbintclrSpec {
    const RESET_VALUE: u32 = 0;
}
