#[doc = "Register `DSP0MBINTCLR` reader"]
pub type R = crate::R<Dsp0mbintclrSpec>;
#[doc = "Register `DSP0MBINTCLR` writer"]
pub type W = crate::W<Dsp0mbintclrSpec>;
#[doc = "Field `DSP0MBINTCLR` reader - DSP0 Mailbox interrupt Clear. The corresponding data bit will clear the interrupt."]
pub type Dsp0mbintclrR = crate::FieldReader<u32>;
#[doc = "Field `DSP0MBINTCLR` writer - DSP0 Mailbox interrupt Clear. The corresponding data bit will clear the interrupt."]
pub type Dsp0mbintclrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP0 Mailbox interrupt Clear. The corresponding data bit will clear the interrupt."]
    #[inline(always)]
    pub fn dsp0mbintclr(&self) -> Dsp0mbintclrR {
        Dsp0mbintclrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP0 Mailbox interrupt Clear. The corresponding data bit will clear the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dsp0mbintclr(&mut self) -> Dsp0mbintclrW<Dsp0mbintclrSpec> {
        Dsp0mbintclrW::new(self, 0)
    }
}
#[doc = "DSP0 Mailbox Interrupt Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0mbintclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0mbintclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0mbintclrSpec;
impl crate::RegisterSpec for Dsp0mbintclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0mbintclr::R`](R) reader structure"]
impl crate::Readable for Dsp0mbintclrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0mbintclr::W`](W) writer structure"]
impl crate::Writable for Dsp0mbintclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0MBINTCLR to value 0"]
impl crate::Resettable for Dsp0mbintclrSpec {
    const RESET_VALUE: u32 = 0;
}
