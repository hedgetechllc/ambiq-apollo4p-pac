#[doc = "Register `DSP0PERFREQ` reader"]
pub type R = crate::R<Dsp0perfreqSpec>;
#[doc = "Register `DSP0PERFREQ` writer"]
pub type W = crate::W<Dsp0perfreqSpec>;
#[doc = "DSP0 Performance mode request\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dsp0perfreq {
    #[doc = "0: DSP0 to be run in ULP mode (freq=48MHz)"]
    Ulp = 0,
    #[doc = "1: DSP0 to be run in LP mode (freq=192MHz)"]
    Lp = 1,
    #[doc = "2: DSP0 to be run in HP mode (freq=384MHz)"]
    Hp = 2,
}
impl From<Dsp0perfreq> for u8 {
    #[inline(always)]
    fn from(variant: Dsp0perfreq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dsp0perfreq {
    type Ux = u8;
}
impl crate::IsEnum for Dsp0perfreq {}
#[doc = "Field `DSP0PERFREQ` reader - DSP0 Performance mode request"]
pub type Dsp0perfreqR = crate::FieldReader<Dsp0perfreq>;
impl Dsp0perfreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dsp0perfreq> {
        match self.bits {
            0 => Some(Dsp0perfreq::Ulp),
            1 => Some(Dsp0perfreq::Lp),
            2 => Some(Dsp0perfreq::Hp),
            _ => None,
        }
    }
    #[doc = "DSP0 to be run in ULP mode (freq=48MHz)"]
    #[inline(always)]
    pub fn is_ulp(&self) -> bool {
        *self == Dsp0perfreq::Ulp
    }
    #[doc = "DSP0 to be run in LP mode (freq=192MHz)"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == Dsp0perfreq::Lp
    }
    #[doc = "DSP0 to be run in HP mode (freq=384MHz)"]
    #[inline(always)]
    pub fn is_hp(&self) -> bool {
        *self == Dsp0perfreq::Hp
    }
}
#[doc = "Field `DSP0PERFREQ` writer - DSP0 Performance mode request"]
pub type Dsp0perfreqW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dsp0perfreq>;
impl<'a, REG> Dsp0perfreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DSP0 to be run in ULP mode (freq=48MHz)"]
    #[inline(always)]
    pub fn ulp(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp0perfreq::Ulp)
    }
    #[doc = "DSP0 to be run in LP mode (freq=192MHz)"]
    #[inline(always)]
    pub fn lp(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp0perfreq::Lp)
    }
    #[doc = "DSP0 to be run in HP mode (freq=384MHz)"]
    #[inline(always)]
    pub fn hp(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp0perfreq::Hp)
    }
}
#[doc = "Field `DSP0PERFACK` reader - Indicates the DSP0 performance status indicated in STATUS register is valid."]
pub type Dsp0perfackR = crate::BitReader;
#[doc = "Field `DSP0PERFACK` writer - Indicates the DSP0 performance status indicated in STATUS register is valid."]
pub type Dsp0perfackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DSP0 Performance mode request\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dsp0perfstatus {
    #[doc = "0: DSP0 is in ULP mode (freq=48MHz)"]
    Ulp = 0,
    #[doc = "1: DSP0 is in LP mode (freq=192MHz)"]
    Lp = 1,
    #[doc = "2: DSP0 is in HP mode (freq=384MHz)"]
    Hp = 2,
}
impl From<Dsp0perfstatus> for u8 {
    #[inline(always)]
    fn from(variant: Dsp0perfstatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dsp0perfstatus {
    type Ux = u8;
}
impl crate::IsEnum for Dsp0perfstatus {}
#[doc = "Field `DSP0PERFSTATUS` reader - DSP0 Performance mode request"]
pub type Dsp0perfstatusR = crate::FieldReader<Dsp0perfstatus>;
impl Dsp0perfstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dsp0perfstatus> {
        match self.bits {
            0 => Some(Dsp0perfstatus::Ulp),
            1 => Some(Dsp0perfstatus::Lp),
            2 => Some(Dsp0perfstatus::Hp),
            _ => None,
        }
    }
    #[doc = "DSP0 is in ULP mode (freq=48MHz)"]
    #[inline(always)]
    pub fn is_ulp(&self) -> bool {
        *self == Dsp0perfstatus::Ulp
    }
    #[doc = "DSP0 is in LP mode (freq=192MHz)"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == Dsp0perfstatus::Lp
    }
    #[doc = "DSP0 is in HP mode (freq=384MHz)"]
    #[inline(always)]
    pub fn is_hp(&self) -> bool {
        *self == Dsp0perfstatus::Hp
    }
}
#[doc = "Field `DSP0PERFSTATUS` writer - DSP0 Performance mode request"]
pub type Dsp0perfstatusW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dsp0perfstatus>;
impl<'a, REG> Dsp0perfstatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DSP0 is in ULP mode (freq=48MHz)"]
    #[inline(always)]
    pub fn ulp(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp0perfstatus::Ulp)
    }
    #[doc = "DSP0 is in LP mode (freq=192MHz)"]
    #[inline(always)]
    pub fn lp(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp0perfstatus::Lp)
    }
    #[doc = "DSP0 is in HP mode (freq=384MHz)"]
    #[inline(always)]
    pub fn hp(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp0perfstatus::Hp)
    }
}
impl R {
    #[doc = "Bits 0:1 - DSP0 Performance mode request"]
    #[inline(always)]
    pub fn dsp0perfreq(&self) -> Dsp0perfreqR {
        Dsp0perfreqR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Indicates the DSP0 performance status indicated in STATUS register is valid."]
    #[inline(always)]
    pub fn dsp0perfack(&self) -> Dsp0perfackR {
        Dsp0perfackR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - DSP0 Performance mode request"]
    #[inline(always)]
    pub fn dsp0perfstatus(&self) -> Dsp0perfstatusR {
        Dsp0perfstatusR::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DSP0 Performance mode request"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0perfreq(&mut self) -> Dsp0perfreqW<Dsp0perfreqSpec> {
        Dsp0perfreqW::new(self, 0)
    }
    #[doc = "Bit 2 - Indicates the DSP0 performance status indicated in STATUS register is valid."]
    #[inline(always)]
    #[must_use]
    pub fn dsp0perfack(&mut self) -> Dsp0perfackW<Dsp0perfreqSpec> {
        Dsp0perfackW::new(self, 2)
    }
    #[doc = "Bits 3:4 - DSP0 Performance mode request"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0perfstatus(&mut self) -> Dsp0perfstatusW<Dsp0perfreqSpec> {
        Dsp0perfstatusW::new(self, 3)
    }
}
#[doc = "This register provides the performance mode knobs for DSP0. S/w should write the *PERFREQ field to desired mode and wait for the *PERFACK and check for the *PERFSTATUS. Some times system may not allow certain modes but *PERFACK should always follow *PERFREQ change.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0perfreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0perfreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0perfreqSpec;
impl crate::RegisterSpec for Dsp0perfreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0perfreq::R`](R) reader structure"]
impl crate::Readable for Dsp0perfreqSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0perfreq::W`](W) writer structure"]
impl crate::Writable for Dsp0perfreqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0PERFREQ to value 0x09"]
impl crate::Resettable for Dsp0perfreqSpec {
    const RESET_VALUE: u32 = 0x09;
}
