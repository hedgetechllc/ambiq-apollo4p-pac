#[doc = "Register `MCUPERFREQ` reader"]
pub type R = crate::R<McuperfreqSpec>;
#[doc = "Register `MCUPERFREQ` writer"]
pub type W = crate::W<McuperfreqSpec>;
#[doc = "MCU Performance mode request\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mcuperfreq {
    #[doc = "0: MCU to be run in ULP mode (freq=24MHz)"]
    Ulp = 0,
    #[doc = "1: MCU to be run in LP mode (freq=96MHz)"]
    Lp = 1,
    #[doc = "2: MCU to be run in HP mode (freq=192MHz)"]
    Hp = 2,
}
impl From<Mcuperfreq> for u8 {
    #[inline(always)]
    fn from(variant: Mcuperfreq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mcuperfreq {
    type Ux = u8;
}
impl crate::IsEnum for Mcuperfreq {}
#[doc = "Field `MCUPERFREQ` reader - MCU Performance mode request"]
pub type McuperfreqR = crate::FieldReader<Mcuperfreq>;
impl McuperfreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mcuperfreq> {
        match self.bits {
            0 => Some(Mcuperfreq::Ulp),
            1 => Some(Mcuperfreq::Lp),
            2 => Some(Mcuperfreq::Hp),
            _ => None,
        }
    }
    #[doc = "MCU to be run in ULP mode (freq=24MHz)"]
    #[inline(always)]
    pub fn is_ulp(&self) -> bool {
        *self == Mcuperfreq::Ulp
    }
    #[doc = "MCU to be run in LP mode (freq=96MHz)"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == Mcuperfreq::Lp
    }
    #[doc = "MCU to be run in HP mode (freq=192MHz)"]
    #[inline(always)]
    pub fn is_hp(&self) -> bool {
        *self == Mcuperfreq::Hp
    }
}
#[doc = "Field `MCUPERFREQ` writer - MCU Performance mode request"]
pub type McuperfreqW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mcuperfreq>;
impl<'a, REG> McuperfreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MCU to be run in ULP mode (freq=24MHz)"]
    #[inline(always)]
    pub fn ulp(self) -> &'a mut crate::W<REG> {
        self.variant(Mcuperfreq::Ulp)
    }
    #[doc = "MCU to be run in LP mode (freq=96MHz)"]
    #[inline(always)]
    pub fn lp(self) -> &'a mut crate::W<REG> {
        self.variant(Mcuperfreq::Lp)
    }
    #[doc = "MCU to be run in HP mode (freq=192MHz)"]
    #[inline(always)]
    pub fn hp(self) -> &'a mut crate::W<REG> {
        self.variant(Mcuperfreq::Hp)
    }
}
#[doc = "Field `MCUPERFACK` reader - Indicates the MCU performance status indicated in STATUS register is valid."]
pub type McuperfackR = crate::BitReader;
#[doc = "Field `MCUPERFACK` writer - Indicates the MCU performance status indicated in STATUS register is valid."]
pub type McuperfackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "MCU Performance mode request\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mcuperfstatus {
    #[doc = "0: MCU is in ULP mode (freq=24MHz)"]
    Ulp = 0,
    #[doc = "1: MCU is in LP mode (freq=96MHz)"]
    Lp = 1,
    #[doc = "2: MCU is in HP mode (freq=192MHz)"]
    Hp = 2,
}
impl From<Mcuperfstatus> for u8 {
    #[inline(always)]
    fn from(variant: Mcuperfstatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mcuperfstatus {
    type Ux = u8;
}
impl crate::IsEnum for Mcuperfstatus {}
#[doc = "Field `MCUPERFSTATUS` reader - MCU Performance mode request"]
pub type McuperfstatusR = crate::FieldReader<Mcuperfstatus>;
impl McuperfstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mcuperfstatus> {
        match self.bits {
            0 => Some(Mcuperfstatus::Ulp),
            1 => Some(Mcuperfstatus::Lp),
            2 => Some(Mcuperfstatus::Hp),
            _ => None,
        }
    }
    #[doc = "MCU is in ULP mode (freq=24MHz)"]
    #[inline(always)]
    pub fn is_ulp(&self) -> bool {
        *self == Mcuperfstatus::Ulp
    }
    #[doc = "MCU is in LP mode (freq=96MHz)"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == Mcuperfstatus::Lp
    }
    #[doc = "MCU is in HP mode (freq=192MHz)"]
    #[inline(always)]
    pub fn is_hp(&self) -> bool {
        *self == Mcuperfstatus::Hp
    }
}
#[doc = "Field `MCUPERFSTATUS` writer - MCU Performance mode request"]
pub type McuperfstatusW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mcuperfstatus>;
impl<'a, REG> McuperfstatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MCU is in ULP mode (freq=24MHz)"]
    #[inline(always)]
    pub fn ulp(self) -> &'a mut crate::W<REG> {
        self.variant(Mcuperfstatus::Ulp)
    }
    #[doc = "MCU is in LP mode (freq=96MHz)"]
    #[inline(always)]
    pub fn lp(self) -> &'a mut crate::W<REG> {
        self.variant(Mcuperfstatus::Lp)
    }
    #[doc = "MCU is in HP mode (freq=192MHz)"]
    #[inline(always)]
    pub fn hp(self) -> &'a mut crate::W<REG> {
        self.variant(Mcuperfstatus::Hp)
    }
}
impl R {
    #[doc = "Bits 0:1 - MCU Performance mode request"]
    #[inline(always)]
    pub fn mcuperfreq(&self) -> McuperfreqR {
        McuperfreqR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Indicates the MCU performance status indicated in STATUS register is valid."]
    #[inline(always)]
    pub fn mcuperfack(&self) -> McuperfackR {
        McuperfackR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - MCU Performance mode request"]
    #[inline(always)]
    pub fn mcuperfstatus(&self) -> McuperfstatusR {
        McuperfstatusR::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - MCU Performance mode request"]
    #[inline(always)]
    #[must_use]
    pub fn mcuperfreq(&mut self) -> McuperfreqW<McuperfreqSpec> {
        McuperfreqW::new(self, 0)
    }
    #[doc = "Bit 2 - Indicates the MCU performance status indicated in STATUS register is valid."]
    #[inline(always)]
    #[must_use]
    pub fn mcuperfack(&mut self) -> McuperfackW<McuperfreqSpec> {
        McuperfackW::new(self, 2)
    }
    #[doc = "Bits 3:4 - MCU Performance mode request"]
    #[inline(always)]
    #[must_use]
    pub fn mcuperfstatus(&mut self) -> McuperfstatusW<McuperfreqSpec> {
        McuperfstatusW::new(self, 3)
    }
}
#[doc = "This register provides the performance mode knobs for MCU. S/w should write the *PERFREQ field to desired mode and wait for the *PERFACK and check for the *PERFSTATUS. Some times system may not allow certain modes but *PERFACK should always follow *PERFREQ change.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcuperfreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcuperfreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McuperfreqSpec;
impl crate::RegisterSpec for McuperfreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcuperfreq::R`](R) reader structure"]
impl crate::Readable for McuperfreqSpec {}
#[doc = "`write(|w| ..)` method takes [`mcuperfreq::W`](W) writer structure"]
impl crate::Writable for McuperfreqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCUPERFREQ to value 0x09"]
impl crate::Resettable for McuperfreqSpec {
    const RESET_VALUE: u32 = 0x09;
}
