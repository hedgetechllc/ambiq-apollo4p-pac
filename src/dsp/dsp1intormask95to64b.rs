#[doc = "Register `DSP1INTORMASK95TO64B` reader"]
pub type R = crate::R<Dsp1intormask95to64bSpec>;
#[doc = "Register `DSP1INTORMASK95TO64B` writer"]
pub type W = crate::W<Dsp1intormask95to64bSpec>;
#[doc = "Field `DSP1MBINTORMASKB` reader - DSP1 Mailbox Interrupt OR Mask B"]
pub type Dsp1mbintormaskbR = crate::FieldReader<u32>;
#[doc = "Field `DSP1MBINTORMASKB` writer - DSP1 Mailbox Interrupt OR Mask B"]
pub type Dsp1mbintormaskbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP1 Mailbox Interrupt OR Mask B"]
    #[inline(always)]
    pub fn dsp1mbintormaskb(&self) -> Dsp1mbintormaskbR {
        Dsp1mbintormaskbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP1 Mailbox Interrupt OR Mask B"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1mbintormaskb(&mut self) -> Dsp1mbintormaskbW<Dsp1intormask95to64bSpec> {
        Dsp1mbintormaskbW::new(self, 0)
    }
}
#[doc = "DSP1 Interrupt OR Mask B for IRQ95-64\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1intormask95to64b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1intormask95to64b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1intormask95to64bSpec;
impl crate::RegisterSpec for Dsp1intormask95to64bSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1intormask95to64b::R`](R) reader structure"]
impl crate::Readable for Dsp1intormask95to64bSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1intormask95to64b::W`](W) writer structure"]
impl crate::Writable for Dsp1intormask95to64bSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1INTORMASK95TO64B to value 0"]
impl crate::Resettable for Dsp1intormask95to64bSpec {
    const RESET_VALUE: u32 = 0;
}
