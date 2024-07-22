#[doc = "Register `CORECFG1` reader"]
pub type R = crate::R<Corecfg1Spec>;
#[doc = "Register `CORECFG1` writer"]
pub type W = crate::W<Corecfg1Spec>;
#[doc = "PCM output chanel 0xsetting\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pcmchset {
    #[doc = "0: Channel Disabled"]
    Chandis = 0,
    #[doc = "1: MONO Left"]
    Monol = 1,
    #[doc = "2: MONO right"]
    Monor = 2,
    #[doc = "3: Stereo"]
    Stereo = 3,
}
impl From<Pcmchset> for u8 {
    #[inline(always)]
    fn from(variant: Pcmchset) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pcmchset {
    type Ux = u8;
}
impl crate::IsEnum for Pcmchset {}
#[doc = "Field `PCMCHSET` reader - PCM output chanel 0xsetting"]
pub type PcmchsetR = crate::FieldReader<Pcmchset>;
impl PcmchsetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcmchset {
        match self.bits {
            0 => Pcmchset::Chandis,
            1 => Pcmchset::Monol,
            2 => Pcmchset::Monor,
            3 => Pcmchset::Stereo,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel Disabled"]
    #[inline(always)]
    pub fn is_chandis(&self) -> bool {
        *self == Pcmchset::Chandis
    }
    #[doc = "MONO Left"]
    #[inline(always)]
    pub fn is_monol(&self) -> bool {
        *self == Pcmchset::Monol
    }
    #[doc = "MONO right"]
    #[inline(always)]
    pub fn is_monor(&self) -> bool {
        *self == Pcmchset::Monor
    }
    #[doc = "Stereo"]
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        *self == Pcmchset::Stereo
    }
}
#[doc = "Field `PCMCHSET` writer - PCM output chanel 0xsetting"]
pub type PcmchsetW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pcmchset, crate::Safe>;
impl<'a, REG> PcmchsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel Disabled"]
    #[inline(always)]
    pub fn chandis(self) -> &'a mut crate::W<REG> {
        self.variant(Pcmchset::Chandis)
    }
    #[doc = "MONO Left"]
    #[inline(always)]
    pub fn monol(self) -> &'a mut crate::W<REG> {
        self.variant(Pcmchset::Monol)
    }
    #[doc = "MONO right"]
    #[inline(always)]
    pub fn monor(self) -> &'a mut crate::W<REG> {
        self.variant(Pcmchset::Monor)
    }
    #[doc = "Stereo"]
    #[inline(always)]
    pub fn stereo(self) -> &'a mut crate::W<REG> {
        self.variant(Pcmchset::Stereo)
    }
}
#[doc = "Field `DIVMCLKQ` reader - Divide down ratio for generating internal master MCLKQ. DIVMCLKQ > 0. DIVMCLKQ = 0 PROHIBITED. Recommend value of 1. Fmclkq = Fpdmclk/(DIVMCLKQ+1)."]
pub type DivmclkqR = crate::FieldReader;
#[doc = "Field `DIVMCLKQ` writer - Divide down ratio for generating internal master MCLKQ. DIVMCLKQ > 0. DIVMCLKQ = 0 PROHIBITED. Recommend value of 1. Fmclkq = Fpdmclk/(DIVMCLKQ+1)."]
pub type DivmclkqW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "PDMA_CKO clock phase delay in terms of PDMCLK period to internal sampler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckodly {
    #[doc = "0: No extra PDMCLK cycle delays."]
    _0cycles = 0,
    #[doc = "1: One xtra PDMCLK cycle delay."]
    _1cycles = 1,
    #[doc = "2: Two extra PDMCLK cycle delays."]
    _2cycles = 2,
    #[doc = "3: Three extra PDMCLK cycle delays."]
    _3cycles = 3,
    #[doc = "4: Four extra PDMCLK cycle delays."]
    _4cycles = 4,
    #[doc = "5: Five extra PDMCLK cycle delays."]
    _5cycles = 5,
    #[doc = "6: Six extra PDMCLK cycle delays."]
    _6cycles = 6,
    #[doc = "7: Seven extra PDMCLK cycle delays."]
    _7cycles = 7,
}
impl From<Ckodly> for u8 {
    #[inline(always)]
    fn from(variant: Ckodly) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckodly {
    type Ux = u8;
}
impl crate::IsEnum for Ckodly {}
#[doc = "Field `CKODLY` reader - PDMA_CKO clock phase delay in terms of PDMCLK period to internal sampler"]
pub type CkodlyR = crate::FieldReader<Ckodly>;
impl CkodlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckodly {
        match self.bits {
            0 => Ckodly::_0cycles,
            1 => Ckodly::_1cycles,
            2 => Ckodly::_2cycles,
            3 => Ckodly::_3cycles,
            4 => Ckodly::_4cycles,
            5 => Ckodly::_5cycles,
            6 => Ckodly::_6cycles,
            7 => Ckodly::_7cycles,
            _ => unreachable!(),
        }
    }
    #[doc = "No extra PDMCLK cycle delays."]
    #[inline(always)]
    pub fn is_0cycles(&self) -> bool {
        *self == Ckodly::_0cycles
    }
    #[doc = "One xtra PDMCLK cycle delay."]
    #[inline(always)]
    pub fn is_1cycles(&self) -> bool {
        *self == Ckodly::_1cycles
    }
    #[doc = "Two extra PDMCLK cycle delays."]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == Ckodly::_2cycles
    }
    #[doc = "Three extra PDMCLK cycle delays."]
    #[inline(always)]
    pub fn is_3cycles(&self) -> bool {
        *self == Ckodly::_3cycles
    }
    #[doc = "Four extra PDMCLK cycle delays."]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == Ckodly::_4cycles
    }
    #[doc = "Five extra PDMCLK cycle delays."]
    #[inline(always)]
    pub fn is_5cycles(&self) -> bool {
        *self == Ckodly::_5cycles
    }
    #[doc = "Six extra PDMCLK cycle delays."]
    #[inline(always)]
    pub fn is_6cycles(&self) -> bool {
        *self == Ckodly::_6cycles
    }
    #[doc = "Seven extra PDMCLK cycle delays."]
    #[inline(always)]
    pub fn is_7cycles(&self) -> bool {
        *self == Ckodly::_7cycles
    }
}
#[doc = "Field `CKODLY` writer - PDMA_CKO clock phase delay in terms of PDMCLK period to internal sampler"]
pub type CkodlyW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ckodly, crate::Safe>;
impl<'a, REG> CkodlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No extra PDMCLK cycle delays."]
    #[inline(always)]
    pub fn _0cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Ckodly::_0cycles)
    }
    #[doc = "One xtra PDMCLK cycle delay."]
    #[inline(always)]
    pub fn _1cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Ckodly::_1cycles)
    }
    #[doc = "Two extra PDMCLK cycle delays."]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Ckodly::_2cycles)
    }
    #[doc = "Three extra PDMCLK cycle delays."]
    #[inline(always)]
    pub fn _3cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Ckodly::_3cycles)
    }
    #[doc = "Four extra PDMCLK cycle delays."]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Ckodly::_4cycles)
    }
    #[doc = "Five extra PDMCLK cycle delays."]
    #[inline(always)]
    pub fn _5cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Ckodly::_5cycles)
    }
    #[doc = "Six extra PDMCLK cycle delays."]
    #[inline(always)]
    pub fn _6cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Ckodly::_6cycles)
    }
    #[doc = "Seven extra PDMCLK cycle delays."]
    #[inline(always)]
    pub fn _7cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Ckodly::_7cycles)
    }
}
#[doc = "Fine grain step size for smooth PGA or Softmute attenuation transition 0: 0.13dB 1: 0.26dB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selstep {
    #[doc = "0: 0.13dB fine grain step size."]
    _0_13db = 0,
    #[doc = "1: 0.26dB fine grain step size."]
    _0_26db = 1,
}
impl From<Selstep> for bool {
    #[inline(always)]
    fn from(variant: Selstep) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELSTEP` reader - Fine grain step size for smooth PGA or Softmute attenuation transition 0: 0.13dB 1: 0.26dB"]
pub type SelstepR = crate::BitReader<Selstep>;
impl SelstepR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selstep {
        match self.bits {
            false => Selstep::_0_13db,
            true => Selstep::_0_26db,
        }
    }
    #[doc = "0.13dB fine grain step size."]
    #[inline(always)]
    pub fn is_0_13db(&self) -> bool {
        *self == Selstep::_0_13db
    }
    #[doc = "0.26dB fine grain step size."]
    #[inline(always)]
    pub fn is_0_26db(&self) -> bool {
        *self == Selstep::_0_26db
    }
}
#[doc = "Field `SELSTEP` writer - Fine grain step size for smooth PGA or Softmute attenuation transition 0: 0.13dB 1: 0.26dB"]
pub type SelstepW<'a, REG> = crate::BitWriter<'a, REG, Selstep>;
impl<'a, REG> SelstepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "0.13dB fine grain step size."]
    #[inline(always)]
    pub fn _0_13db(self) -> &'a mut crate::W<REG> {
        self.variant(Selstep::_0_13db)
    }
    #[doc = "0.26dB fine grain step size."]
    #[inline(always)]
    pub fn _0_26db(self) -> &'a mut crate::W<REG> {
        self.variant(Selstep::_0_26db)
    }
}
impl R {
    #[doc = "Bits 0:1 - PCM output chanel 0xsetting"]
    #[inline(always)]
    pub fn pcmchset(&self) -> PcmchsetR {
        PcmchsetR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Divide down ratio for generating internal master MCLKQ. DIVMCLKQ > 0. DIVMCLKQ = 0 PROHIBITED. Recommend value of 1. Fmclkq = Fpdmclk/(DIVMCLKQ+1)."]
    #[inline(always)]
    pub fn divmclkq(&self) -> DivmclkqR {
        DivmclkqR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - PDMA_CKO clock phase delay in terms of PDMCLK period to internal sampler"]
    #[inline(always)]
    pub fn ckodly(&self) -> CkodlyR {
        CkodlyR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Fine grain step size for smooth PGA or Softmute attenuation transition 0: 0.13dB 1: 0.26dB"]
    #[inline(always)]
    pub fn selstep(&self) -> SelstepR {
        SelstepR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PCM output chanel 0xsetting"]
    #[inline(always)]
    #[must_use]
    pub fn pcmchset(&mut self) -> PcmchsetW<Corecfg1Spec> {
        PcmchsetW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Divide down ratio for generating internal master MCLKQ. DIVMCLKQ > 0. DIVMCLKQ = 0 PROHIBITED. Recommend value of 1. Fmclkq = Fpdmclk/(DIVMCLKQ+1)."]
    #[inline(always)]
    #[must_use]
    pub fn divmclkq(&mut self) -> DivmclkqW<Corecfg1Spec> {
        DivmclkqW::new(self, 2)
    }
    #[doc = "Bits 4:6 - PDMA_CKO clock phase delay in terms of PDMCLK period to internal sampler"]
    #[inline(always)]
    #[must_use]
    pub fn ckodly(&mut self) -> CkodlyW<Corecfg1Spec> {
        CkodlyW::new(self, 4)
    }
    #[doc = "Bit 7 - Fine grain step size for smooth PGA or Softmute attenuation transition 0: 0.13dB 1: 0.26dB"]
    #[inline(always)]
    #[must_use]
    pub fn selstep(&mut self) -> SelstepW<Corecfg1Spec> {
        SelstepW::new(self, 7)
    }
}
#[doc = "PDM to PCM Extra Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`corecfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`corecfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Corecfg1Spec;
impl crate::RegisterSpec for Corecfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`corecfg1::R`](R) reader structure"]
impl crate::Readable for Corecfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`corecfg1::W`](W) writer structure"]
impl crate::Writable for Corecfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORECFG1 to value 0x07"]
impl crate::Resettable for Corecfg1Spec {
    const RESET_VALUE: u32 = 0x07;
}
