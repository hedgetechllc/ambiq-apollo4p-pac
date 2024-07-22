#[doc = "Register `DSP0PWRCTRL` reader"]
pub type R = crate::R<Dsp0pwrctrlSpec>;
#[doc = "Register `DSP0PWRCTRL` writer"]
pub type W = crate::W<Dsp0pwrctrlSpec>;
#[doc = "Field `DSP0PCMRSTDLY` reader - PCM Reset delay in number of 24MHz clocks."]
pub type Dsp0pcmrstdlyR = crate::FieldReader;
#[doc = "Field `DSP0PCMRSTDLY` writer - PCM Reset delay in number of 24MHz clocks."]
pub type Dsp0pcmrstdlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "PCM Reset override. If this is disabled, then h/w will handle the de-assertion of pcm reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsp0pcmrstor {
    #[doc = "1: Keep DSP0 PCM in Reset"]
    En = 1,
    #[doc = "0: Remove DSP0 PCM Reset override"]
    Dis = 0,
}
impl From<Dsp0pcmrstor> for bool {
    #[inline(always)]
    fn from(variant: Dsp0pcmrstor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP0PCMRSTOR` reader - PCM Reset override. If this is disabled, then h/w will handle the de-assertion of pcm reset."]
pub type Dsp0pcmrstorR = crate::BitReader<Dsp0pcmrstor>;
impl Dsp0pcmrstorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsp0pcmrstor {
        match self.bits {
            true => Dsp0pcmrstor::En,
            false => Dsp0pcmrstor::Dis,
        }
    }
    #[doc = "Keep DSP0 PCM in Reset"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dsp0pcmrstor::En
    }
    #[doc = "Remove DSP0 PCM Reset override"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dsp0pcmrstor::Dis
    }
}
#[doc = "Field `DSP0PCMRSTOR` writer - PCM Reset override. If this is disabled, then h/w will handle the de-assertion of pcm reset."]
pub type Dsp0pcmrstorW<'a, REG> = crate::BitWriter<'a, REG, Dsp0pcmrstor>;
impl<'a, REG> Dsp0pcmrstorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Keep DSP0 PCM in Reset"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp0pcmrstor::En)
    }
    #[doc = "Remove DSP0 PCM Reset override"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp0pcmrstor::Dis)
    }
}
impl R {
    #[doc = "Bits 0:3 - PCM Reset delay in number of 24MHz clocks."]
    #[inline(always)]
    pub fn dsp0pcmrstdly(&self) -> Dsp0pcmrstdlyR {
        Dsp0pcmrstdlyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - PCM Reset override. If this is disabled, then h/w will handle the de-assertion of pcm reset."]
    #[inline(always)]
    pub fn dsp0pcmrstor(&self) -> Dsp0pcmrstorR {
        Dsp0pcmrstorR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - PCM Reset delay in number of 24MHz clocks."]
    #[inline(always)]
    #[must_use]
    pub fn dsp0pcmrstdly(&mut self) -> Dsp0pcmrstdlyW<Dsp0pwrctrlSpec> {
        Dsp0pcmrstdlyW::new(self, 0)
    }
    #[doc = "Bit 4 - PCM Reset override. If this is disabled, then h/w will handle the de-assertion of pcm reset."]
    #[inline(always)]
    #[must_use]
    pub fn dsp0pcmrstor(&mut self) -> Dsp0pcmrstorW<Dsp0pwrctrlSpec> {
        Dsp0pcmrstorW::new(self, 4)
    }
}
#[doc = "Power and RST controls for DSP0\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0pwrctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0pwrctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0pwrctrlSpec;
impl crate::RegisterSpec for Dsp0pwrctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0pwrctrl::R`](R) reader structure"]
impl crate::Readable for Dsp0pwrctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0pwrctrl::W`](W) writer structure"]
impl crate::Writable for Dsp0pwrctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0PWRCTRL to value 0x08"]
impl crate::Resettable for Dsp0pwrctrlSpec {
    const RESET_VALUE: u32 = 0x08;
}
