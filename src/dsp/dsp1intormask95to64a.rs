#[doc = "Register `DSP1INTORMASK95TO64A` reader"]
pub type R = crate::R<Dsp1intormask95to64aSpec>;
#[doc = "Register `DSP1INTORMASK95TO64A` writer"]
pub type W = crate::W<Dsp1intormask95to64aSpec>;
#[doc = "Field `DSP1MBINTORMASKA` reader - DSP1 Mailbox Interrupt OR Mask A"]
pub type Dsp1mbintormaskaR = crate::FieldReader<u32>;
#[doc = "Field `DSP1MBINTORMASKA` writer - DSP1 Mailbox Interrupt OR Mask A"]
pub type Dsp1mbintormaskaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP1 Mailbox Interrupt OR Mask A"]
    #[inline(always)]
    pub fn dsp1mbintormaska(&self) -> Dsp1mbintormaskaR {
        Dsp1mbintormaskaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP1 Mailbox Interrupt OR Mask A"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1mbintormaska(&mut self) -> Dsp1mbintormaskaW<Dsp1intormask95to64aSpec> {
        Dsp1mbintormaskaW::new(self, 0)
    }
}
#[doc = "DSP1 Interrupt OR Mask A for IRQ95-64\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1intormask95to64a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1intormask95to64a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1intormask95to64aSpec;
impl crate::RegisterSpec for Dsp1intormask95to64aSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1intormask95to64a::R`](R) reader structure"]
impl crate::Readable for Dsp1intormask95to64aSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1intormask95to64a::W`](W) writer structure"]
impl crate::Writable for Dsp1intormask95to64aSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1INTORMASK95TO64A to value 0"]
impl crate::Resettable for Dsp1intormask95to64aSpec {
    const RESET_VALUE: u32 = 0;
}
