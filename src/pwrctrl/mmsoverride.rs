#[doc = "Register `MMSOVERRIDE` reader"]
pub type R = crate::R<MmsoverrideSpec>;
#[doc = "Register `MMSOVERRIDE` writer"]
pub type W = crate::W<MmsoverrideSpec>;
#[doc = "MMS override for MCUL on by PD_DISP setting.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mmsovrmculdisp {
    #[doc = "0: When PD_DISP is on, MCUL is on."]
    Mculon = 0,
    #[doc = "1: When PD_DISP is on, MCUL is still off."]
    Mculoff = 1,
}
impl From<Mmsovrmculdisp> for bool {
    #[inline(always)]
    fn from(variant: Mmsovrmculdisp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMSOVRMCULDISP` reader - MMS override for MCUL on by PD_DISP setting."]
pub type MmsovrmculdispR = crate::BitReader<Mmsovrmculdisp>;
impl MmsovrmculdispR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mmsovrmculdisp {
        match self.bits {
            false => Mmsovrmculdisp::Mculon,
            true => Mmsovrmculdisp::Mculoff,
        }
    }
    #[doc = "When PD_DISP is on, MCUL is on."]
    #[inline(always)]
    pub fn is_mculon(&self) -> bool {
        *self == Mmsovrmculdisp::Mculon
    }
    #[doc = "When PD_DISP is on, MCUL is still off."]
    #[inline(always)]
    pub fn is_mculoff(&self) -> bool {
        *self == Mmsovrmculdisp::Mculoff
    }
}
#[doc = "Field `MMSOVRMCULDISP` writer - MMS override for MCUL on by PD_DISP setting."]
pub type MmsovrmculdispW<'a, REG> = crate::BitWriter<'a, REG, Mmsovrmculdisp>;
impl<'a, REG> MmsovrmculdispW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When PD_DISP is on, MCUL is on."]
    #[inline(always)]
    pub fn mculon(self) -> &'a mut crate::W<REG> {
        self.variant(Mmsovrmculdisp::Mculon)
    }
    #[doc = "When PD_DISP is on, MCUL is still off."]
    #[inline(always)]
    pub fn mculoff(self) -> &'a mut crate::W<REG> {
        self.variant(Mmsovrmculdisp::Mculoff)
    }
}
#[doc = "MMS override for MCUL on by PD_GFX setting.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mmsovrmculgfx {
    #[doc = "0: When PD_GFX is on, MCUL is on."]
    Mculon = 0,
    #[doc = "1: When PD_GFX is on, MCUL is still off."]
    Mculoff = 1,
}
impl From<Mmsovrmculgfx> for bool {
    #[inline(always)]
    fn from(variant: Mmsovrmculgfx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMSOVRMCULGFX` reader - MMS override for MCUL on by PD_GFX setting."]
pub type MmsovrmculgfxR = crate::BitReader<Mmsovrmculgfx>;
impl MmsovrmculgfxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mmsovrmculgfx {
        match self.bits {
            false => Mmsovrmculgfx::Mculon,
            true => Mmsovrmculgfx::Mculoff,
        }
    }
    #[doc = "When PD_GFX is on, MCUL is on."]
    #[inline(always)]
    pub fn is_mculon(&self) -> bool {
        *self == Mmsovrmculgfx::Mculon
    }
    #[doc = "When PD_GFX is on, MCUL is still off."]
    #[inline(always)]
    pub fn is_mculoff(&self) -> bool {
        *self == Mmsovrmculgfx::Mculoff
    }
}
#[doc = "Field `MMSOVRMCULGFX` writer - MMS override for MCUL on by PD_GFX setting."]
pub type MmsovrmculgfxW<'a, REG> = crate::BitWriter<'a, REG, Mmsovrmculgfx>;
impl<'a, REG> MmsovrmculgfxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When PD_GFX is on, MCUL is on."]
    #[inline(always)]
    pub fn mculon(self) -> &'a mut crate::W<REG> {
        self.variant(Mmsovrmculgfx::Mculon)
    }
    #[doc = "When PD_GFX is on, MCUL is still off."]
    #[inline(always)]
    pub fn mculoff(self) -> &'a mut crate::W<REG> {
        self.variant(Mmsovrmculgfx::Mculoff)
    }
}
#[doc = "MMS override for SSRAM power state by PD_DISP power setting.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mmsovrssramdisp {
    #[doc = "1: SSRAM power state is not affected by PD_DISP setting."]
    Nodisp = 1,
    #[doc = "0: SSRAM power state set by SSRAMPWREN_PWRENSSRAM is overridden by PD_DISP setting."]
    PdDisp = 0,
}
impl From<Mmsovrssramdisp> for bool {
    #[inline(always)]
    fn from(variant: Mmsovrssramdisp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMSOVRSSRAMDISP` reader - MMS override for SSRAM power state by PD_DISP power setting."]
pub type MmsovrssramdispR = crate::BitReader<Mmsovrssramdisp>;
impl MmsovrssramdispR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mmsovrssramdisp {
        match self.bits {
            true => Mmsovrssramdisp::Nodisp,
            false => Mmsovrssramdisp::PdDisp,
        }
    }
    #[doc = "SSRAM power state is not affected by PD_DISP setting."]
    #[inline(always)]
    pub fn is_nodisp(&self) -> bool {
        *self == Mmsovrssramdisp::Nodisp
    }
    #[doc = "SSRAM power state set by SSRAMPWREN_PWRENSSRAM is overridden by PD_DISP setting."]
    #[inline(always)]
    pub fn is_pd_disp(&self) -> bool {
        *self == Mmsovrssramdisp::PdDisp
    }
}
#[doc = "Field `MMSOVRSSRAMDISP` writer - MMS override for SSRAM power state by PD_DISP power setting."]
pub type MmsovrssramdispW<'a, REG> = crate::BitWriter<'a, REG, Mmsovrssramdisp>;
impl<'a, REG> MmsovrssramdispW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SSRAM power state is not affected by PD_DISP setting."]
    #[inline(always)]
    pub fn nodisp(self) -> &'a mut crate::W<REG> {
        self.variant(Mmsovrssramdisp::Nodisp)
    }
    #[doc = "SSRAM power state set by SSRAMPWREN_PWRENSSRAM is overridden by PD_DISP setting."]
    #[inline(always)]
    pub fn pd_disp(self) -> &'a mut crate::W<REG> {
        self.variant(Mmsovrssramdisp::PdDisp)
    }
}
#[doc = "MMS override for SSRAM power state by PD_GFX power setting.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mmsovrssramgfx {
    #[doc = "1: SSRAM power state is not affected by PD_GFX setting."]
    Nogfx = 1,
    #[doc = "0: SSRAM power state set by SSRAMPWREN_PWRENSSRAM is overridden by PD_GFX setting."]
    PdGfx = 0,
}
impl From<Mmsovrssramgfx> for bool {
    #[inline(always)]
    fn from(variant: Mmsovrssramgfx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMSOVRSSRAMGFX` reader - MMS override for SSRAM power state by PD_GFX power setting."]
pub type MmsovrssramgfxR = crate::BitReader<Mmsovrssramgfx>;
impl MmsovrssramgfxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mmsovrssramgfx {
        match self.bits {
            true => Mmsovrssramgfx::Nogfx,
            false => Mmsovrssramgfx::PdGfx,
        }
    }
    #[doc = "SSRAM power state is not affected by PD_GFX setting."]
    #[inline(always)]
    pub fn is_nogfx(&self) -> bool {
        *self == Mmsovrssramgfx::Nogfx
    }
    #[doc = "SSRAM power state set by SSRAMPWREN_PWRENSSRAM is overridden by PD_GFX setting."]
    #[inline(always)]
    pub fn is_pd_gfx(&self) -> bool {
        *self == Mmsovrssramgfx::PdGfx
    }
}
#[doc = "Field `MMSOVRSSRAMGFX` writer - MMS override for SSRAM power state by PD_GFX power setting."]
pub type MmsovrssramgfxW<'a, REG> = crate::BitWriter<'a, REG, Mmsovrssramgfx>;
impl<'a, REG> MmsovrssramgfxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SSRAM power state is not affected by PD_GFX setting."]
    #[inline(always)]
    pub fn nogfx(self) -> &'a mut crate::W<REG> {
        self.variant(Mmsovrssramgfx::Nogfx)
    }
    #[doc = "SSRAM power state set by SSRAMPWREN_PWRENSSRAM is overridden by PD_GFX setting."]
    #[inline(always)]
    pub fn pd_gfx(self) -> &'a mut crate::W<REG> {
        self.variant(Mmsovrssramgfx::PdGfx)
    }
}
#[doc = "If set, retention equation doesn't consider DISP. Each bit corresponds to a domain.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mmsovrdspramretdisp {
    #[doc = "1: When PD_DISP is off, retention is always okay for PD_DISP domain."]
    Alwayson = 1,
    #[doc = "0: When PD_DISP is off, retention is okay based on the state of PD_DISP domain and DSP\\[1|0\\]MEMRETCFG."]
    Pgstate = 0,
}
impl From<Mmsovrdspramretdisp> for u8 {
    #[inline(always)]
    fn from(variant: Mmsovrdspramretdisp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mmsovrdspramretdisp {
    type Ux = u8;
}
impl crate::IsEnum for Mmsovrdspramretdisp {}
#[doc = "Field `MMSOVRDSPRAMRETDISP` reader - If set, retention equation doesn't consider DISP. Each bit corresponds to a domain."]
pub type MmsovrdspramretdispR = crate::FieldReader<Mmsovrdspramretdisp>;
impl MmsovrdspramretdispR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mmsovrdspramretdisp> {
        match self.bits {
            1 => Some(Mmsovrdspramretdisp::Alwayson),
            0 => Some(Mmsovrdspramretdisp::Pgstate),
            _ => None,
        }
    }
    #[doc = "When PD_DISP is off, retention is always okay for PD_DISP domain."]
    #[inline(always)]
    pub fn is_alwayson(&self) -> bool {
        *self == Mmsovrdspramretdisp::Alwayson
    }
    #[doc = "When PD_DISP is off, retention is okay based on the state of PD_DISP domain and DSP\\[1|0\\]MEMRETCFG."]
    #[inline(always)]
    pub fn is_pgstate(&self) -> bool {
        *self == Mmsovrdspramretdisp::Pgstate
    }
}
#[doc = "Field `MMSOVRDSPRAMRETDISP` writer - If set, retention equation doesn't consider DISP. Each bit corresponds to a domain."]
pub type MmsovrdspramretdispW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mmsovrdspramretdisp>;
impl<'a, REG> MmsovrdspramretdispW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "When PD_DISP is off, retention is always okay for PD_DISP domain."]
    #[inline(always)]
    pub fn alwayson(self) -> &'a mut crate::W<REG> {
        self.variant(Mmsovrdspramretdisp::Alwayson)
    }
    #[doc = "When PD_DISP is off, retention is okay based on the state of PD_DISP domain and DSP\\[1|0\\]MEMRETCFG."]
    #[inline(always)]
    pub fn pgstate(self) -> &'a mut crate::W<REG> {
        self.variant(Mmsovrdspramretdisp::Pgstate)
    }
}
#[doc = "If set, retention equation doesn't consider GFX. Each bit corresponds to a domain.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mmsovrdspramretgfx {
    #[doc = "1: When PD_GFX is off, retention is always okay for PD_GFX domain."]
    Alwayson = 1,
    #[doc = "0: When PD_GFX is off, retention is okay based on the state of PD_GFX domain and DSP\\[1|0\\]MEMRETCFG."]
    Pgstate = 0,
}
impl From<Mmsovrdspramretgfx> for u8 {
    #[inline(always)]
    fn from(variant: Mmsovrdspramretgfx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mmsovrdspramretgfx {
    type Ux = u8;
}
impl crate::IsEnum for Mmsovrdspramretgfx {}
#[doc = "Field `MMSOVRDSPRAMRETGFX` reader - If set, retention equation doesn't consider GFX. Each bit corresponds to a domain."]
pub type MmsovrdspramretgfxR = crate::FieldReader<Mmsovrdspramretgfx>;
impl MmsovrdspramretgfxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mmsovrdspramretgfx> {
        match self.bits {
            1 => Some(Mmsovrdspramretgfx::Alwayson),
            0 => Some(Mmsovrdspramretgfx::Pgstate),
            _ => None,
        }
    }
    #[doc = "When PD_GFX is off, retention is always okay for PD_GFX domain."]
    #[inline(always)]
    pub fn is_alwayson(&self) -> bool {
        *self == Mmsovrdspramretgfx::Alwayson
    }
    #[doc = "When PD_GFX is off, retention is okay based on the state of PD_GFX domain and DSP\\[1|0\\]MEMRETCFG."]
    #[inline(always)]
    pub fn is_pgstate(&self) -> bool {
        *self == Mmsovrdspramretgfx::Pgstate
    }
}
#[doc = "Field `MMSOVRDSPRAMRETGFX` writer - If set, retention equation doesn't consider GFX. Each bit corresponds to a domain."]
pub type MmsovrdspramretgfxW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mmsovrdspramretgfx>;
impl<'a, REG> MmsovrdspramretgfxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "When PD_GFX is off, retention is always okay for PD_GFX domain."]
    #[inline(always)]
    pub fn alwayson(self) -> &'a mut crate::W<REG> {
        self.variant(Mmsovrdspramretgfx::Alwayson)
    }
    #[doc = "When PD_GFX is off, retention is okay based on the state of PD_GFX domain and DSP\\[1|0\\]MEMRETCFG."]
    #[inline(always)]
    pub fn pgstate(self) -> &'a mut crate::W<REG> {
        self.variant(Mmsovrdspramretgfx::Pgstate)
    }
}
#[doc = "If set, retention equation doesn't consider DISP. Each bit corresponds to a domain.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mmsovrssramretdisp {
    #[doc = "1: When PD_DISP is off, retention is always okay for PD_DISP domain."]
    Alwayson = 1,
    #[doc = "0: When PD_DISP is off, retention is okay based on the state of PD_DISP domain and SSRAMRETCFG."]
    Pgstate = 0,
}
impl From<Mmsovrssramretdisp> for u8 {
    #[inline(always)]
    fn from(variant: Mmsovrssramretdisp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mmsovrssramretdisp {
    type Ux = u8;
}
impl crate::IsEnum for Mmsovrssramretdisp {}
#[doc = "Field `MMSOVRSSRAMRETDISP` reader - If set, retention equation doesn't consider DISP. Each bit corresponds to a domain."]
pub type MmsovrssramretdispR = crate::FieldReader<Mmsovrssramretdisp>;
impl MmsovrssramretdispR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mmsovrssramretdisp> {
        match self.bits {
            1 => Some(Mmsovrssramretdisp::Alwayson),
            0 => Some(Mmsovrssramretdisp::Pgstate),
            _ => None,
        }
    }
    #[doc = "When PD_DISP is off, retention is always okay for PD_DISP domain."]
    #[inline(always)]
    pub fn is_alwayson(&self) -> bool {
        *self == Mmsovrssramretdisp::Alwayson
    }
    #[doc = "When PD_DISP is off, retention is okay based on the state of PD_DISP domain and SSRAMRETCFG."]
    #[inline(always)]
    pub fn is_pgstate(&self) -> bool {
        *self == Mmsovrssramretdisp::Pgstate
    }
}
#[doc = "Field `MMSOVRSSRAMRETDISP` writer - If set, retention equation doesn't consider DISP. Each bit corresponds to a domain."]
pub type MmsovrssramretdispW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mmsovrssramretdisp>;
impl<'a, REG> MmsovrssramretdispW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "When PD_DISP is off, retention is always okay for PD_DISP domain."]
    #[inline(always)]
    pub fn alwayson(self) -> &'a mut crate::W<REG> {
        self.variant(Mmsovrssramretdisp::Alwayson)
    }
    #[doc = "When PD_DISP is off, retention is okay based on the state of PD_DISP domain and SSRAMRETCFG."]
    #[inline(always)]
    pub fn pgstate(self) -> &'a mut crate::W<REG> {
        self.variant(Mmsovrssramretdisp::Pgstate)
    }
}
#[doc = "If set, retention equation doesn't consider GFX. Each bit corresponds to a domain.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mmsovrssramretgfx {
    #[doc = "1: When PD_GFX is off, retention is always okay for PD_GFX domain."]
    Alwayson = 1,
    #[doc = "0: When PD_GFX is off, retention is okay based on the state of PD_GFX domain and SSRAMRETCFG."]
    Pgstate = 0,
}
impl From<Mmsovrssramretgfx> for u8 {
    #[inline(always)]
    fn from(variant: Mmsovrssramretgfx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mmsovrssramretgfx {
    type Ux = u8;
}
impl crate::IsEnum for Mmsovrssramretgfx {}
#[doc = "Field `MMSOVRSSRAMRETGFX` reader - If set, retention equation doesn't consider GFX. Each bit corresponds to a domain."]
pub type MmsovrssramretgfxR = crate::FieldReader<Mmsovrssramretgfx>;
impl MmsovrssramretgfxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mmsovrssramretgfx> {
        match self.bits {
            1 => Some(Mmsovrssramretgfx::Alwayson),
            0 => Some(Mmsovrssramretgfx::Pgstate),
            _ => None,
        }
    }
    #[doc = "When PD_GFX is off, retention is always okay for PD_GFX domain."]
    #[inline(always)]
    pub fn is_alwayson(&self) -> bool {
        *self == Mmsovrssramretgfx::Alwayson
    }
    #[doc = "When PD_GFX is off, retention is okay based on the state of PD_GFX domain and SSRAMRETCFG."]
    #[inline(always)]
    pub fn is_pgstate(&self) -> bool {
        *self == Mmsovrssramretgfx::Pgstate
    }
}
#[doc = "Field `MMSOVRSSRAMRETGFX` writer - If set, retention equation doesn't consider GFX. Each bit corresponds to a domain."]
pub type MmsovrssramretgfxW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mmsovrssramretgfx>;
impl<'a, REG> MmsovrssramretgfxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "When PD_GFX is off, retention is always okay for PD_GFX domain."]
    #[inline(always)]
    pub fn alwayson(self) -> &'a mut crate::W<REG> {
        self.variant(Mmsovrssramretgfx::Alwayson)
    }
    #[doc = "When PD_GFX is off, retention is okay based on the state of PD_GFX domain and SSRAMRETCFG."]
    #[inline(always)]
    pub fn pgstate(self) -> &'a mut crate::W<REG> {
        self.variant(Mmsovrssramretgfx::Pgstate)
    }
}
impl R {
    #[doc = "Bit 0 - MMS override for MCUL on by PD_DISP setting."]
    #[inline(always)]
    pub fn mmsovrmculdisp(&self) -> MmsovrmculdispR {
        MmsovrmculdispR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMS override for MCUL on by PD_GFX setting."]
    #[inline(always)]
    pub fn mmsovrmculgfx(&self) -> MmsovrmculgfxR {
        MmsovrmculgfxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MMS override for SSRAM power state by PD_DISP power setting."]
    #[inline(always)]
    pub fn mmsovrssramdisp(&self) -> MmsovrssramdispR {
        MmsovrssramdispR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMS override for SSRAM power state by PD_GFX power setting."]
    #[inline(always)]
    pub fn mmsovrssramgfx(&self) -> MmsovrssramgfxR {
        MmsovrssramgfxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - If set, retention equation doesn't consider DISP. Each bit corresponds to a domain."]
    #[inline(always)]
    pub fn mmsovrdspramretdisp(&self) -> MmsovrdspramretdispR {
        MmsovrdspramretdispR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - If set, retention equation doesn't consider GFX. Each bit corresponds to a domain."]
    #[inline(always)]
    pub fn mmsovrdspramretgfx(&self) -> MmsovrdspramretgfxR {
        MmsovrdspramretgfxR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - If set, retention equation doesn't consider DISP. Each bit corresponds to a domain."]
    #[inline(always)]
    pub fn mmsovrssramretdisp(&self) -> MmsovrssramretdispR {
        MmsovrssramretdispR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - If set, retention equation doesn't consider GFX. Each bit corresponds to a domain."]
    #[inline(always)]
    pub fn mmsovrssramretgfx(&self) -> MmsovrssramretgfxR {
        MmsovrssramretgfxR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MMS override for MCUL on by PD_DISP setting."]
    #[inline(always)]
    #[must_use]
    pub fn mmsovrmculdisp(&mut self) -> MmsovrmculdispW<MmsoverrideSpec> {
        MmsovrmculdispW::new(self, 0)
    }
    #[doc = "Bit 1 - MMS override for MCUL on by PD_GFX setting."]
    #[inline(always)]
    #[must_use]
    pub fn mmsovrmculgfx(&mut self) -> MmsovrmculgfxW<MmsoverrideSpec> {
        MmsovrmculgfxW::new(self, 1)
    }
    #[doc = "Bit 2 - MMS override for SSRAM power state by PD_DISP power setting."]
    #[inline(always)]
    #[must_use]
    pub fn mmsovrssramdisp(&mut self) -> MmsovrssramdispW<MmsoverrideSpec> {
        MmsovrssramdispW::new(self, 2)
    }
    #[doc = "Bit 3 - MMS override for SSRAM power state by PD_GFX power setting."]
    #[inline(always)]
    #[must_use]
    pub fn mmsovrssramgfx(&mut self) -> MmsovrssramgfxW<MmsoverrideSpec> {
        MmsovrssramgfxW::new(self, 3)
    }
    #[doc = "Bits 4:5 - If set, retention equation doesn't consider DISP. Each bit corresponds to a domain."]
    #[inline(always)]
    #[must_use]
    pub fn mmsovrdspramretdisp(&mut self) -> MmsovrdspramretdispW<MmsoverrideSpec> {
        MmsovrdspramretdispW::new(self, 4)
    }
    #[doc = "Bits 6:7 - If set, retention equation doesn't consider GFX. Each bit corresponds to a domain."]
    #[inline(always)]
    #[must_use]
    pub fn mmsovrdspramretgfx(&mut self) -> MmsovrdspramretgfxW<MmsoverrideSpec> {
        MmsovrdspramretgfxW::new(self, 6)
    }
    #[doc = "Bits 8:9 - If set, retention equation doesn't consider DISP. Each bit corresponds to a domain."]
    #[inline(always)]
    #[must_use]
    pub fn mmsovrssramretdisp(&mut self) -> MmsovrssramretdispW<MmsoverrideSpec> {
        MmsovrssramretdispW::new(self, 8)
    }
    #[doc = "Bits 10:11 - If set, retention equation doesn't consider GFX. Each bit corresponds to a domain."]
    #[inline(always)]
    #[must_use]
    pub fn mmsovrssramretgfx(&mut self) -> MmsovrssramretgfxW<MmsoverrideSpec> {
        MmsovrssramretgfxW::new(self, 10)
    }
}
#[doc = "Power domain behavior overrides related to MMS ( Multimedia System ).\n\nYou can [`read`](crate::Reg::read) this register and get [`mmsoverride::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmsoverride::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmsoverrideSpec;
impl crate::RegisterSpec for MmsoverrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmsoverride::R`](R) reader structure"]
impl crate::Readable for MmsoverrideSpec {}
#[doc = "`write(|w| ..)` method takes [`mmsoverride::W`](W) writer structure"]
impl crate::Writable for MmsoverrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMSOVERRIDE to value 0x0ffc"]
impl crate::Resettable for MmsoverrideSpec {
    const RESET_VALUE: u32 = 0x0ffc;
}
