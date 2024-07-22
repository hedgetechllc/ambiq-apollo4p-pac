#[doc = "Register `DSP0INTORMASK95TO64A` reader"]
pub type R = crate::R<Dsp0intormask95to64aSpec>;
#[doc = "Register `DSP0INTORMASK95TO64A` writer"]
pub type W = crate::W<Dsp0intormask95to64aSpec>;
#[doc = "Field `DSP0MBINTORMASKA` reader - DSP0 Mailbox Interrupt OR Mask A"]
pub type Dsp0mbintormaskaR = crate::FieldReader<u32>;
#[doc = "Field `DSP0MBINTORMASKA` writer - DSP0 Mailbox Interrupt OR Mask A"]
pub type Dsp0mbintormaskaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DSP0 Mailbox Interrupt OR Mask A"]
    #[inline(always)]
    pub fn dsp0mbintormaska(&self) -> Dsp0mbintormaskaR {
        Dsp0mbintormaskaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSP0 Mailbox Interrupt OR Mask A"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0mbintormaska(&mut self) -> Dsp0mbintormaskaW<Dsp0intormask95to64aSpec> {
        Dsp0mbintormaskaW::new(self, 0)
    }
}
#[doc = "DSP0 Interrupt OR Mask A for IRQ95-64\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0intormask95to64a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0intormask95to64a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0intormask95to64aSpec;
impl crate::RegisterSpec for Dsp0intormask95to64aSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0intormask95to64a::R`](R) reader structure"]
impl crate::Readable for Dsp0intormask95to64aSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0intormask95to64a::W`](W) writer structure"]
impl crate::Writable for Dsp0intormask95to64aSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0INTORMASK95TO64A to value 0"]
impl crate::Resettable for Dsp0intormask95to64aSpec {
    const RESET_VALUE: u32 = 0;
}
