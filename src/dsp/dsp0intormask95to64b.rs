#[doc = "Register `DSP0INTORMASK95TO64B` reader"]
pub type R = crate::R<Dsp0intormask95to64bSpec>;
#[doc = "Register `DSP0INTORMASK95TO64B` writer"]
pub type W = crate::W<Dsp0intormask95to64bSpec>;
#[doc = "Field `DSP0MBINTORMASKB` reader - DSP0 Mailbox Interrupt OR Mask B"]
pub type Dsp0mbintormaskbR = crate::FieldReader<u32>;
#[doc = "Field `DSP0MBINTORMASKB` writer - DSP0 Mailbox Interrupt OR Mask B"]
pub type Dsp0mbintormaskbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP0 Mailbox Interrupt OR Mask B"]
    #[inline(always)]
    pub fn dsp0mbintormaskb(&self) -> Dsp0mbintormaskbR {
        Dsp0mbintormaskbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP0 Mailbox Interrupt OR Mask B"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0mbintormaskb(&mut self) -> Dsp0mbintormaskbW<Dsp0intormask95to64bSpec> {
        Dsp0mbintormaskbW::new(self, 0)
    }
}
#[doc = "DSP0 Interrupt OR Mask B for IRQ95-64\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0intormask95to64b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0intormask95to64b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0intormask95to64bSpec;
impl crate::RegisterSpec for Dsp0intormask95to64bSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0intormask95to64b::R`](R) reader structure"]
impl crate::Readable for Dsp0intormask95to64bSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0intormask95to64b::W`](W) writer structure"]
impl crate::Writable for Dsp0intormask95to64bSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0INTORMASK95TO64B to value 0"]
impl crate::Resettable for Dsp0intormask95to64bSpec {
    const RESET_VALUE: u32 = 0;
}
