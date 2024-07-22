#[doc = "Register `DSP0MBINTSET` reader"]
pub type R = crate::R<Dsp0mbintsetSpec>;
#[doc = "Register `DSP0MBINTSET` writer"]
pub type W = crate::W<Dsp0mbintsetSpec>;
#[doc = "Field `DSP0MBINTSET` reader - DSP0 Mailbox interrupt Set. The corresponding data bit will set the interrupt."]
pub type Dsp0mbintsetR = crate::FieldReader<u32>;
#[doc = "Field `DSP0MBINTSET` writer - DSP0 Mailbox interrupt Set. The corresponding data bit will set the interrupt."]
pub type Dsp0mbintsetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP0 Mailbox interrupt Set. The corresponding data bit will set the interrupt."]
    #[inline(always)]
    pub fn dsp0mbintset(&self) -> Dsp0mbintsetR {
        Dsp0mbintsetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP0 Mailbox interrupt Set. The corresponding data bit will set the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dsp0mbintset(&mut self) -> Dsp0mbintsetW<Dsp0mbintsetSpec> {
        Dsp0mbintsetW::new(self, 0)
    }
}
#[doc = "DSP0 Mailbox Interrupt Set\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0mbintset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0mbintset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0mbintsetSpec;
impl crate::RegisterSpec for Dsp0mbintsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0mbintset::R`](R) reader structure"]
impl crate::Readable for Dsp0mbintsetSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0mbintset::W`](W) writer structure"]
impl crate::Writable for Dsp0mbintsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0MBINTSET to value 0"]
impl crate::Resettable for Dsp0mbintsetSpec {
    const RESET_VALUE: u32 = 0;
}
