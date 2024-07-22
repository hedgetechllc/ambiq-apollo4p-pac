#[doc = "Register `VRSTATUS` reader"]
pub type R = crate::R<VrstatusSpec>;
#[doc = "Register `VRSTATUS` writer"]
pub type W = crate::W<VrstatusSpec>;
#[doc = "Indicates CORELDO status. bit\\[1\\]
indicates ON/OFF and bit\\[0\\]
indicates ACT/LP.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Coreldost {
    #[doc = "0: Indicates the the CORELDO is OFF."]
    Off = 0,
    #[doc = "2: Indicates the the CORELDO is ON and in LP mode."]
    Lp = 2,
    #[doc = "3: Indicates the the CORELDO is ON and in ACT mode."]
    Act = 3,
}
impl From<Coreldost> for u8 {
    #[inline(always)]
    fn from(variant: Coreldost) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Coreldost {
    type Ux = u8;
}
impl crate::IsEnum for Coreldost {}
#[doc = "Field `CORELDOST` reader - Indicates CORELDO status. bit\\[1\\]
indicates ON/OFF and bit\\[0\\]
indicates ACT/LP."]
pub type CoreldostR = crate::FieldReader<Coreldost>;
impl CoreldostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Coreldost> {
        match self.bits {
            0 => Some(Coreldost::Off),
            2 => Some(Coreldost::Lp),
            3 => Some(Coreldost::Act),
            _ => None,
        }
    }
    #[doc = "Indicates the the CORELDO is OFF."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Coreldost::Off
    }
    #[doc = "Indicates the the CORELDO is ON and in LP mode."]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == Coreldost::Lp
    }
    #[doc = "Indicates the the CORELDO is ON and in ACT mode."]
    #[inline(always)]
    pub fn is_act(&self) -> bool {
        *self == Coreldost::Act
    }
}
#[doc = "Field `CORELDOST` writer - Indicates CORELDO status. bit\\[1\\]
indicates ON/OFF and bit\\[0\\]
indicates ACT/LP."]
pub type CoreldostW<'a, REG> = crate::FieldWriter<'a, REG, 2, Coreldost>;
impl<'a, REG> CoreldostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Indicates the the CORELDO is OFF."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Coreldost::Off)
    }
    #[doc = "Indicates the the CORELDO is ON and in LP mode."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut crate::W<REG> {
        self.variant(Coreldost::Lp)
    }
    #[doc = "Indicates the the CORELDO is ON and in ACT mode."]
    #[inline(always)]
    pub fn act(self) -> &'a mut crate::W<REG> {
        self.variant(Coreldost::Act)
    }
}
#[doc = "Indicates MEMLDO status. bit\\[1\\]
indicates ON/OFF and bit\\[0\\]
indicates ACT/LP.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Memldost {
    #[doc = "0: Indicates the the MEMLDO is OFF."]
    Off = 0,
    #[doc = "2: Indicates the the MEMLDO is ON and in LP mode."]
    Lp = 2,
    #[doc = "3: Indicates the the MEMLDO is ON and in ACT mode."]
    Act = 3,
}
impl From<Memldost> for u8 {
    #[inline(always)]
    fn from(variant: Memldost) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Memldost {
    type Ux = u8;
}
impl crate::IsEnum for Memldost {}
#[doc = "Field `MEMLDOST` reader - Indicates MEMLDO status. bit\\[1\\]
indicates ON/OFF and bit\\[0\\]
indicates ACT/LP."]
pub type MemldostR = crate::FieldReader<Memldost>;
impl MemldostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Memldost> {
        match self.bits {
            0 => Some(Memldost::Off),
            2 => Some(Memldost::Lp),
            3 => Some(Memldost::Act),
            _ => None,
        }
    }
    #[doc = "Indicates the the MEMLDO is OFF."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Memldost::Off
    }
    #[doc = "Indicates the the MEMLDO is ON and in LP mode."]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == Memldost::Lp
    }
    #[doc = "Indicates the the MEMLDO is ON and in ACT mode."]
    #[inline(always)]
    pub fn is_act(&self) -> bool {
        *self == Memldost::Act
    }
}
#[doc = "Field `MEMLDOST` writer - Indicates MEMLDO status. bit\\[1\\]
indicates ON/OFF and bit\\[0\\]
indicates ACT/LP."]
pub type MemldostW<'a, REG> = crate::FieldWriter<'a, REG, 2, Memldost>;
impl<'a, REG> MemldostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Indicates the the MEMLDO is OFF."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Memldost::Off)
    }
    #[doc = "Indicates the the MEMLDO is ON and in LP mode."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut crate::W<REG> {
        self.variant(Memldost::Lp)
    }
    #[doc = "Indicates the the MEMLDO is ON and in ACT mode."]
    #[inline(always)]
    pub fn act(self) -> &'a mut crate::W<REG> {
        self.variant(Memldost::Act)
    }
}
#[doc = "Indicates SIMO BUCK status. bit\\[1\\]
indicates ON/OFF and bit\\[0\\]
indicates ACT/LP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Simobuckst {
    #[doc = "0: Indicates the the SIMO BUCK is OFF."]
    Off = 0,
    #[doc = "2: Indicates the the SIMO BUCK is ON and in LP mode."]
    Lp = 2,
    #[doc = "3: Indicates the the SIMO BUCK is ON and in ACT mode."]
    Act = 3,
}
impl From<Simobuckst> for u8 {
    #[inline(always)]
    fn from(variant: Simobuckst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Simobuckst {
    type Ux = u8;
}
impl crate::IsEnum for Simobuckst {}
#[doc = "Field `SIMOBUCKST` reader - Indicates SIMO BUCK status. bit\\[1\\]
indicates ON/OFF and bit\\[0\\]
indicates ACT/LP"]
pub type SimobuckstR = crate::FieldReader<Simobuckst>;
impl SimobuckstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Simobuckst> {
        match self.bits {
            0 => Some(Simobuckst::Off),
            2 => Some(Simobuckst::Lp),
            3 => Some(Simobuckst::Act),
            _ => None,
        }
    }
    #[doc = "Indicates the the SIMO BUCK is OFF."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Simobuckst::Off
    }
    #[doc = "Indicates the the SIMO BUCK is ON and in LP mode."]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == Simobuckst::Lp
    }
    #[doc = "Indicates the the SIMO BUCK is ON and in ACT mode."]
    #[inline(always)]
    pub fn is_act(&self) -> bool {
        *self == Simobuckst::Act
    }
}
#[doc = "Field `SIMOBUCKST` writer - Indicates SIMO BUCK status. bit\\[1\\]
indicates ON/OFF and bit\\[0\\]
indicates ACT/LP"]
pub type SimobuckstW<'a, REG> = crate::FieldWriter<'a, REG, 2, Simobuckst>;
impl<'a, REG> SimobuckstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Indicates the the SIMO BUCK is OFF."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Simobuckst::Off)
    }
    #[doc = "Indicates the the SIMO BUCK is ON and in LP mode."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut crate::W<REG> {
        self.variant(Simobuckst::Lp)
    }
    #[doc = "Indicates the the SIMO BUCK is ON and in ACT mode."]
    #[inline(always)]
    pub fn act(self) -> &'a mut crate::W<REG> {
        self.variant(Simobuckst::Act)
    }
}
impl R {
    #[doc = "Bits 0:1 - Indicates CORELDO status. bit\\[1\\]
indicates ON/OFF and bit\\[0\\]
indicates ACT/LP."]
    #[inline(always)]
    pub fn coreldost(&self) -> CoreldostR {
        CoreldostR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Indicates MEMLDO status. bit\\[1\\]
indicates ON/OFF and bit\\[0\\]
indicates ACT/LP."]
    #[inline(always)]
    pub fn memldost(&self) -> MemldostR {
        MemldostR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Indicates SIMO BUCK status. bit\\[1\\]
indicates ON/OFF and bit\\[0\\]
indicates ACT/LP"]
    #[inline(always)]
    pub fn simobuckst(&self) -> SimobuckstR {
        SimobuckstR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Indicates CORELDO status. bit\\[1\\]
indicates ON/OFF and bit\\[0\\]
indicates ACT/LP."]
    #[inline(always)]
    #[must_use]
    pub fn coreldost(&mut self) -> CoreldostW<VrstatusSpec> {
        CoreldostW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Indicates MEMLDO status. bit\\[1\\]
indicates ON/OFF and bit\\[0\\]
indicates ACT/LP."]
    #[inline(always)]
    #[must_use]
    pub fn memldost(&mut self) -> MemldostW<VrstatusSpec> {
        MemldostW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Indicates SIMO BUCK status. bit\\[1\\]
indicates ON/OFF and bit\\[0\\]
indicates ACT/LP"]
    #[inline(always)]
    #[must_use]
    pub fn simobuckst(&mut self) -> SimobuckstW<VrstatusSpec> {
        SimobuckstW::new(self, 4)
    }
}
#[doc = "Provides BUCK and LDOs status.\n\nYou can [`read`](crate::Reg::read) this register and get [`vrstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VrstatusSpec;
impl crate::RegisterSpec for VrstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vrstatus::R`](R) reader structure"]
impl crate::Readable for VrstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`vrstatus::W`](W) writer structure"]
impl crate::Writable for VrstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VRSTATUS to value 0"]
impl crate::Resettable for VrstatusSpec {
    const RESET_VALUE: u32 = 0;
}
