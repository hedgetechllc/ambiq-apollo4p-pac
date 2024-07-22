#[doc = "Register `DSP1PWRCTRL` reader"]
pub type R = crate::R<Dsp1pwrctrlSpec>;
#[doc = "Register `DSP1PWRCTRL` writer"]
pub type W = crate::W<Dsp1pwrctrlSpec>;
#[doc = "Field `DSP1PCMRSTDLY` reader - PCM Reset delay in number of 24MHz clocks."]
pub type Dsp1pcmrstdlyR = crate::FieldReader;
#[doc = "Field `DSP1PCMRSTDLY` writer - PCM Reset delay in number of 24MHz clocks."]
pub type Dsp1pcmrstdlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "PCM Reset override. If this is disabled, then h/w will handle the de-assertion of pcm reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsp1pcmrstor {
    #[doc = "1: Keep DSP1 PCM in Reset"]
    En = 1,
    #[doc = "0: Remove DSP1 PCM Reset override"]
    Dis = 0,
}
impl From<Dsp1pcmrstor> for bool {
    #[inline(always)]
    fn from(variant: Dsp1pcmrstor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP1PCMRSTOR` reader - PCM Reset override. If this is disabled, then h/w will handle the de-assertion of pcm reset."]
pub type Dsp1pcmrstorR = crate::BitReader<Dsp1pcmrstor>;
impl Dsp1pcmrstorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsp1pcmrstor {
        match self.bits {
            true => Dsp1pcmrstor::En,
            false => Dsp1pcmrstor::Dis,
        }
    }
    #[doc = "Keep DSP1 PCM in Reset"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dsp1pcmrstor::En
    }
    #[doc = "Remove DSP1 PCM Reset override"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dsp1pcmrstor::Dis
    }
}
#[doc = "Field `DSP1PCMRSTOR` writer - PCM Reset override. If this is disabled, then h/w will handle the de-assertion of pcm reset."]
pub type Dsp1pcmrstorW<'a, REG> = crate::BitWriter<'a, REG, Dsp1pcmrstor>;
impl<'a, REG> Dsp1pcmrstorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Keep DSP1 PCM in Reset"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp1pcmrstor::En)
    }
    #[doc = "Remove DSP1 PCM Reset override"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp1pcmrstor::Dis)
    }
}
impl R {
    #[doc = "Bits 0:3 - PCM Reset delay in number of 24MHz clocks."]
    #[inline(always)]
    pub fn dsp1pcmrstdly(&self) -> Dsp1pcmrstdlyR {
        Dsp1pcmrstdlyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - PCM Reset override. If this is disabled, then h/w will handle the de-assertion of pcm reset."]
    #[inline(always)]
    pub fn dsp1pcmrstor(&self) -> Dsp1pcmrstorR {
        Dsp1pcmrstorR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - PCM Reset delay in number of 24MHz clocks."]
    #[inline(always)]
    #[must_use]
    pub fn dsp1pcmrstdly(&mut self) -> Dsp1pcmrstdlyW<Dsp1pwrctrlSpec> {
        Dsp1pcmrstdlyW::new(self, 0)
    }
    #[doc = "Bit 4 - PCM Reset override. If this is disabled, then h/w will handle the de-assertion of pcm reset."]
    #[inline(always)]
    #[must_use]
    pub fn dsp1pcmrstor(&mut self) -> Dsp1pcmrstorW<Dsp1pwrctrlSpec> {
        Dsp1pcmrstorW::new(self, 4)
    }
}
#[doc = "Power and RST controls for DSP1\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1pwrctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1pwrctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1pwrctrlSpec;
impl crate::RegisterSpec for Dsp1pwrctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1pwrctrl::R`](R) reader structure"]
impl crate::Readable for Dsp1pwrctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1pwrctrl::W`](W) writer structure"]
impl crate::Writable for Dsp1pwrctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1PWRCTRL to value 0x08"]
impl crate::Resettable for Dsp1pwrctrlSpec {
    const RESET_VALUE: u32 = 0x08;
}
