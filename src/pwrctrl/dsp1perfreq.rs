#[doc = "Register `DSP1PERFREQ` reader"]
pub type R = crate::R<Dsp1perfreqSpec>;
#[doc = "Register `DSP1PERFREQ` writer"]
pub type W = crate::W<Dsp1perfreqSpec>;
#[doc = "DSP1 Performance mode request\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dsp1perfreq {
    #[doc = "0: DSP1 to be run in ULP mode (freq=48MHz)"]
    Ulp = 0,
    #[doc = "1: DSP1 to be run in LP mode (freq=192MHz)"]
    Lp = 1,
    #[doc = "2: DSP1 to be run in HP mode (freq=384MHz)"]
    Hp = 2,
}
impl From<Dsp1perfreq> for u8 {
    #[inline(always)]
    fn from(variant: Dsp1perfreq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dsp1perfreq {
    type Ux = u8;
}
impl crate::IsEnum for Dsp1perfreq {}
#[doc = "Field `DSP1PERFREQ` reader - DSP1 Performance mode request"]
pub type Dsp1perfreqR = crate::FieldReader<Dsp1perfreq>;
impl Dsp1perfreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dsp1perfreq> {
        match self.bits {
            0 => Some(Dsp1perfreq::Ulp),
            1 => Some(Dsp1perfreq::Lp),
            2 => Some(Dsp1perfreq::Hp),
            _ => None,
        }
    }
    #[doc = "DSP1 to be run in ULP mode (freq=48MHz)"]
    #[inline(always)]
    pub fn is_ulp(&self) -> bool {
        *self == Dsp1perfreq::Ulp
    }
    #[doc = "DSP1 to be run in LP mode (freq=192MHz)"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == Dsp1perfreq::Lp
    }
    #[doc = "DSP1 to be run in HP mode (freq=384MHz)"]
    #[inline(always)]
    pub fn is_hp(&self) -> bool {
        *self == Dsp1perfreq::Hp
    }
}
#[doc = "Field `DSP1PERFREQ` writer - DSP1 Performance mode request"]
pub type Dsp1perfreqW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dsp1perfreq>;
impl<'a, REG> Dsp1perfreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DSP1 to be run in ULP mode (freq=48MHz)"]
    #[inline(always)]
    pub fn ulp(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp1perfreq::Ulp)
    }
    #[doc = "DSP1 to be run in LP mode (freq=192MHz)"]
    #[inline(always)]
    pub fn lp(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp1perfreq::Lp)
    }
    #[doc = "DSP1 to be run in HP mode (freq=384MHz)"]
    #[inline(always)]
    pub fn hp(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp1perfreq::Hp)
    }
}
#[doc = "Field `DSP1PERFACK` reader - Indicates the DSP1 performance status indicated in STATUS register is valid."]
pub type Dsp1perfackR = crate::BitReader;
#[doc = "Field `DSP1PERFACK` writer - Indicates the DSP1 performance status indicated in STATUS register is valid."]
pub type Dsp1perfackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DSP1 Performance mode request\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dsp1perfstatus {
    #[doc = "0: DSP1 is in ULP mode (freq=48MHz)"]
    Ulp = 0,
    #[doc = "1: DSP1 is in LP mode (freq=192MHz)"]
    Lp = 1,
    #[doc = "2: DSP1 is in HP mode (freq=384MHz)"]
    Hp = 2,
}
impl From<Dsp1perfstatus> for u8 {
    #[inline(always)]
    fn from(variant: Dsp1perfstatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dsp1perfstatus {
    type Ux = u8;
}
impl crate::IsEnum for Dsp1perfstatus {}
#[doc = "Field `DSP1PERFSTATUS` reader - DSP1 Performance mode request"]
pub type Dsp1perfstatusR = crate::FieldReader<Dsp1perfstatus>;
impl Dsp1perfstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dsp1perfstatus> {
        match self.bits {
            0 => Some(Dsp1perfstatus::Ulp),
            1 => Some(Dsp1perfstatus::Lp),
            2 => Some(Dsp1perfstatus::Hp),
            _ => None,
        }
    }
    #[doc = "DSP1 is in ULP mode (freq=48MHz)"]
    #[inline(always)]
    pub fn is_ulp(&self) -> bool {
        *self == Dsp1perfstatus::Ulp
    }
    #[doc = "DSP1 is in LP mode (freq=192MHz)"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == Dsp1perfstatus::Lp
    }
    #[doc = "DSP1 is in HP mode (freq=384MHz)"]
    #[inline(always)]
    pub fn is_hp(&self) -> bool {
        *self == Dsp1perfstatus::Hp
    }
}
#[doc = "Field `DSP1PERFSTATUS` writer - DSP1 Performance mode request"]
pub type Dsp1perfstatusW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dsp1perfstatus>;
impl<'a, REG> Dsp1perfstatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DSP1 is in ULP mode (freq=48MHz)"]
    #[inline(always)]
    pub fn ulp(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp1perfstatus::Ulp)
    }
    #[doc = "DSP1 is in LP mode (freq=192MHz)"]
    #[inline(always)]
    pub fn lp(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp1perfstatus::Lp)
    }
    #[doc = "DSP1 is in HP mode (freq=384MHz)"]
    #[inline(always)]
    pub fn hp(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp1perfstatus::Hp)
    }
}
impl R {
    #[doc = "Bits 0:1 - DSP1 Performance mode request"]
    #[inline(always)]
    pub fn dsp1perfreq(&self) -> Dsp1perfreqR {
        Dsp1perfreqR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Indicates the DSP1 performance status indicated in STATUS register is valid."]
    #[inline(always)]
    pub fn dsp1perfack(&self) -> Dsp1perfackR {
        Dsp1perfackR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - DSP1 Performance mode request"]
    #[inline(always)]
    pub fn dsp1perfstatus(&self) -> Dsp1perfstatusR {
        Dsp1perfstatusR::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DSP1 Performance mode request"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1perfreq(&mut self) -> Dsp1perfreqW<Dsp1perfreqSpec> {
        Dsp1perfreqW::new(self, 0)
    }
    #[doc = "Bit 2 - Indicates the DSP1 performance status indicated in STATUS register is valid."]
    #[inline(always)]
    #[must_use]
    pub fn dsp1perfack(&mut self) -> Dsp1perfackW<Dsp1perfreqSpec> {
        Dsp1perfackW::new(self, 2)
    }
    #[doc = "Bits 3:4 - DSP1 Performance mode request"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1perfstatus(&mut self) -> Dsp1perfstatusW<Dsp1perfreqSpec> {
        Dsp1perfstatusW::new(self, 3)
    }
}
#[doc = "This register provides the performance mode knobs for DSP1. S/w should write the *PERFREQ field to desired mode and wait for the *PERFACK and check for the *PERFSTATUS. Some times system may not allow certain modes but *PERFACK should always follow *PERFREQ change.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1perfreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1perfreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1perfreqSpec;
impl crate::RegisterSpec for Dsp1perfreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1perfreq::R`](R) reader structure"]
impl crate::Readable for Dsp1perfreqSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1perfreq::W`](W) writer structure"]
impl crate::Writable for Dsp1perfreqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1PERFREQ to value 0x09"]
impl crate::Resettable for Dsp1perfreqSpec {
    const RESET_VALUE: u32 = 0x09;
}
