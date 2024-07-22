#[doc = "Register `PWRSW0` reader"]
pub type R = crate::R<Pwrsw0Spec>;
#[doc = "Register `PWRSW0` writer"]
pub type W = crate::W<Pwrsw0Spec>;
#[doc = "Field `PWRSWVDDCPUDYNSEL` reader - override value for pwrsw_vddcpu_dynsel"]
pub type PwrswvddcpudynselR = crate::FieldReader;
#[doc = "Field `PWRSWVDDCPUDYNSEL` writer - override value for pwrsw_vddcpu_dynsel"]
pub type PwrswvddcpudynselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWRSWVDDCPUPGN` reader - override value for pwrsw_vddcpu_pgn"]
pub type PwrswvddcpupgnR = crate::BitReader;
#[doc = "Field `PWRSWVDDCPUPGN` writer - override value for pwrsw_vddcpu_pgn"]
pub type PwrswvddcpupgnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSWVDDCPUOVERRIDE` reader - override enable for pwrsw_vddcpu_dynsel and pgn"]
pub type PwrswvddcpuoverrideR = crate::BitReader;
#[doc = "Field `PWRSWVDDCPUOVERRIDE` writer - override enable for pwrsw_vddcpu_dynsel and pgn"]
pub type PwrswvddcpuoverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSWVDDCAORDYNSEL` reader - override value for pwrsw_vddcaor_dynsel"]
pub type PwrswvddcaordynselR = crate::FieldReader;
#[doc = "Field `PWRSWVDDCAORDYNSEL` writer - override value for pwrsw_vddcaor_dynsel"]
pub type PwrswvddcaordynselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWRSWVDDCAOROVERRIDE` reader - override enable for pwrsw_vddcaor_dynsel"]
pub type PwrswvddcaoroverrideR = crate::BitReader;
#[doc = "Field `PWRSWVDDCAOROVERRIDE` writer - override enable for pwrsw_vddcaor_dynsel"]
pub type PwrswvddcaoroverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSWVDDDSP0DYNSEL` reader - override value for pwrsw_vdddsp0_dynsel"]
pub type Pwrswvdddsp0dynselR = crate::FieldReader;
#[doc = "Field `PWRSWVDDDSP0DYNSEL` writer - override value for pwrsw_vdddsp0_dynsel"]
pub type Pwrswvdddsp0dynselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWRSWVDDDSP0PGN` reader - override value for pwrsw_vdddsp0_pgn"]
pub type Pwrswvdddsp0pgnR = crate::BitReader;
#[doc = "Field `PWRSWVDDDSP0PGN` writer - override value for pwrsw_vdddsp0_pgn"]
pub type Pwrswvdddsp0pgnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSWVDDDSP0OVERRIDE` reader - override enable for pwrsw_vdddsp0_dynsel and pgn"]
pub type Pwrswvdddsp0overrideR = crate::BitReader;
#[doc = "Field `PWRSWVDDDSP0OVERRIDE` writer - override enable for pwrsw_vdddsp0_dynsel and pgn"]
pub type Pwrswvdddsp0overrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSWVDDDSP1DYNSEL` reader - override value for pwrsw_vdddsp1_dynsel"]
pub type Pwrswvdddsp1dynselR = crate::FieldReader;
#[doc = "Field `PWRSWVDDDSP1DYNSEL` writer - override value for pwrsw_vdddsp1_dynsel"]
pub type Pwrswvdddsp1dynselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWRSWVDDDSP1PGN` reader - override value for pwrsw_vdddsp1_pgn"]
pub type Pwrswvdddsp1pgnR = crate::BitReader;
#[doc = "Field `PWRSWVDDDSP1PGN` writer - override value for pwrsw_vdddsp1_pgn"]
pub type Pwrswvdddsp1pgnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSWVDDDSP1OVERRIDE` reader - override enable for pwrsw_vdddsp1_dynsel and pgn"]
pub type Pwrswvdddsp1overrideR = crate::BitReader;
#[doc = "Field `PWRSWVDDDSP1OVERRIDE` writer - override enable for pwrsw_vdddsp1_dynsel and pgn"]
pub type Pwrswvdddsp1overrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSWVDDMCPUDYNSEL` reader - override value for pwrsw_vddmcpu_dynsel"]
pub type PwrswvddmcpudynselR = crate::BitReader;
#[doc = "Field `PWRSWVDDMCPUDYNSEL` writer - override value for pwrsw_vddmcpu_dynsel"]
pub type PwrswvddmcpudynselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "VDDMCPU power switch static select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrswvddmcpustatsel {
    #[doc = "0: Select VDDC rail"]
    Vddc = 0,
    #[doc = "1: Select VDDF rail"]
    Vddf = 1,
}
impl From<Pwrswvddmcpustatsel> for bool {
    #[inline(always)]
    fn from(variant: Pwrswvddmcpustatsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSWVDDMCPUSTATSEL` reader - VDDMCPU power switch static select"]
pub type PwrswvddmcpustatselR = crate::BitReader<Pwrswvddmcpustatsel>;
impl PwrswvddmcpustatselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrswvddmcpustatsel {
        match self.bits {
            false => Pwrswvddmcpustatsel::Vddc,
            true => Pwrswvddmcpustatsel::Vddf,
        }
    }
    #[doc = "Select VDDC rail"]
    #[inline(always)]
    pub fn is_vddc(&self) -> bool {
        *self == Pwrswvddmcpustatsel::Vddc
    }
    #[doc = "Select VDDF rail"]
    #[inline(always)]
    pub fn is_vddf(&self) -> bool {
        *self == Pwrswvddmcpustatsel::Vddf
    }
}
#[doc = "Field `PWRSWVDDMCPUSTATSEL` writer - VDDMCPU power switch static select"]
pub type PwrswvddmcpustatselW<'a, REG> = crate::BitWriter<'a, REG, Pwrswvddmcpustatsel>;
impl<'a, REG> PwrswvddmcpustatselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select VDDC rail"]
    #[inline(always)]
    pub fn vddc(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrswvddmcpustatsel::Vddc)
    }
    #[doc = "Select VDDF rail"]
    #[inline(always)]
    pub fn vddf(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrswvddmcpustatsel::Vddf)
    }
}
#[doc = "Field `PWRSWVDDMCPUOVERRIDE` reader - override enable for pwrsw_vddmcpu_dynsel"]
pub type PwrswvddmcpuoverrideR = crate::BitReader;
#[doc = "Field `PWRSWVDDMCPUOVERRIDE` writer - override enable for pwrsw_vddmcpu_dynsel"]
pub type PwrswvddmcpuoverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSWVDDMDSP0DYNSEL` reader - override value for pwrsw_vddmdsp0_dynsel"]
pub type Pwrswvddmdsp0dynselR = crate::BitReader;
#[doc = "Field `PWRSWVDDMDSP0DYNSEL` writer - override value for pwrsw_vddmdsp0_dynsel"]
pub type Pwrswvddmdsp0dynselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "VDDMDSP0 power switch static select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrswvddmdsp0statsel {
    #[doc = "0: Select VDDC rail"]
    Vddc = 0,
    #[doc = "1: Select VDDF rail"]
    Vddf = 1,
}
impl From<Pwrswvddmdsp0statsel> for bool {
    #[inline(always)]
    fn from(variant: Pwrswvddmdsp0statsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSWVDDMDSP0STATSEL` reader - VDDMDSP0 power switch static select"]
pub type Pwrswvddmdsp0statselR = crate::BitReader<Pwrswvddmdsp0statsel>;
impl Pwrswvddmdsp0statselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrswvddmdsp0statsel {
        match self.bits {
            false => Pwrswvddmdsp0statsel::Vddc,
            true => Pwrswvddmdsp0statsel::Vddf,
        }
    }
    #[doc = "Select VDDC rail"]
    #[inline(always)]
    pub fn is_vddc(&self) -> bool {
        *self == Pwrswvddmdsp0statsel::Vddc
    }
    #[doc = "Select VDDF rail"]
    #[inline(always)]
    pub fn is_vddf(&self) -> bool {
        *self == Pwrswvddmdsp0statsel::Vddf
    }
}
#[doc = "Field `PWRSWVDDMDSP0STATSEL` writer - VDDMDSP0 power switch static select"]
pub type Pwrswvddmdsp0statselW<'a, REG> = crate::BitWriter<'a, REG, Pwrswvddmdsp0statsel>;
impl<'a, REG> Pwrswvddmdsp0statselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select VDDC rail"]
    #[inline(always)]
    pub fn vddc(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrswvddmdsp0statsel::Vddc)
    }
    #[doc = "Select VDDF rail"]
    #[inline(always)]
    pub fn vddf(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrswvddmdsp0statsel::Vddf)
    }
}
#[doc = "Field `PWRSWVDDMDSP0OVERRIDE` reader - override enable for pwrsw_vddmdsp0_dynsel"]
pub type Pwrswvddmdsp0overrideR = crate::BitReader;
#[doc = "Field `PWRSWVDDMDSP0OVERRIDE` writer - override enable for pwrsw_vddmdsp0_dynsel"]
pub type Pwrswvddmdsp0overrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSWVDDMDSP1DYNSEL` reader - override value for pwrsw_vddmdsp1_dynsel"]
pub type Pwrswvddmdsp1dynselR = crate::BitReader;
#[doc = "Field `PWRSWVDDMDSP1DYNSEL` writer - override value for pwrsw_vddmdsp1_dynsel"]
pub type Pwrswvddmdsp1dynselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "VDDMDSP1 power switch static select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrswvddmdsp1statsel {
    #[doc = "0: Select VDDC rail"]
    Vddc = 0,
    #[doc = "1: Select VDDF rail"]
    Vddf = 1,
}
impl From<Pwrswvddmdsp1statsel> for bool {
    #[inline(always)]
    fn from(variant: Pwrswvddmdsp1statsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSWVDDMDSP1STATSEL` reader - VDDMDSP1 power switch static select"]
pub type Pwrswvddmdsp1statselR = crate::BitReader<Pwrswvddmdsp1statsel>;
impl Pwrswvddmdsp1statselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrswvddmdsp1statsel {
        match self.bits {
            false => Pwrswvddmdsp1statsel::Vddc,
            true => Pwrswvddmdsp1statsel::Vddf,
        }
    }
    #[doc = "Select VDDC rail"]
    #[inline(always)]
    pub fn is_vddc(&self) -> bool {
        *self == Pwrswvddmdsp1statsel::Vddc
    }
    #[doc = "Select VDDF rail"]
    #[inline(always)]
    pub fn is_vddf(&self) -> bool {
        *self == Pwrswvddmdsp1statsel::Vddf
    }
}
#[doc = "Field `PWRSWVDDMDSP1STATSEL` writer - VDDMDSP1 power switch static select"]
pub type Pwrswvddmdsp1statselW<'a, REG> = crate::BitWriter<'a, REG, Pwrswvddmdsp1statsel>;
impl<'a, REG> Pwrswvddmdsp1statselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select VDDC rail"]
    #[inline(always)]
    pub fn vddc(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrswvddmdsp1statsel::Vddc)
    }
    #[doc = "Select VDDF rail"]
    #[inline(always)]
    pub fn vddf(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrswvddmdsp1statsel::Vddf)
    }
}
#[doc = "Field `PWRSWVDDMDSP1OVERRIDE` reader - override enable for pwrsw_vddmdsp1_dynsel"]
pub type Pwrswvddmdsp1overrideR = crate::BitReader;
#[doc = "Field `PWRSWVDDMDSP1OVERRIDE` writer - override enable for pwrsw_vddmdsp1_dynsel"]
pub type Pwrswvddmdsp1overrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSWVDDMLDYNSEL` reader - override value for pwrsw_vddml_dynsel"]
pub type PwrswvddmldynselR = crate::BitReader;
#[doc = "Field `PWRSWVDDMLDYNSEL` writer - override value for pwrsw_vddml_dynsel"]
pub type PwrswvddmldynselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "VDDML power switch static select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrswvddmlstatsel {
    #[doc = "0: Select VDDC rail"]
    Vddc = 0,
    #[doc = "1: Select VDDF rail"]
    Vddf = 1,
}
impl From<Pwrswvddmlstatsel> for bool {
    #[inline(always)]
    fn from(variant: Pwrswvddmlstatsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSWVDDMLSTATSEL` reader - VDDML power switch static select"]
pub type PwrswvddmlstatselR = crate::BitReader<Pwrswvddmlstatsel>;
impl PwrswvddmlstatselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrswvddmlstatsel {
        match self.bits {
            false => Pwrswvddmlstatsel::Vddc,
            true => Pwrswvddmlstatsel::Vddf,
        }
    }
    #[doc = "Select VDDC rail"]
    #[inline(always)]
    pub fn is_vddc(&self) -> bool {
        *self == Pwrswvddmlstatsel::Vddc
    }
    #[doc = "Select VDDF rail"]
    #[inline(always)]
    pub fn is_vddf(&self) -> bool {
        *self == Pwrswvddmlstatsel::Vddf
    }
}
#[doc = "Field `PWRSWVDDMLSTATSEL` writer - VDDML power switch static select"]
pub type PwrswvddmlstatselW<'a, REG> = crate::BitWriter<'a, REG, Pwrswvddmlstatsel>;
impl<'a, REG> PwrswvddmlstatselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select VDDC rail"]
    #[inline(always)]
    pub fn vddc(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrswvddmlstatsel::Vddc)
    }
    #[doc = "Select VDDF rail"]
    #[inline(always)]
    pub fn vddf(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrswvddmlstatsel::Vddf)
    }
}
#[doc = "Field `PWRSWVDDMLOVERRIDE` reader - override enable for pwrsw_vddml_dynsel"]
pub type PwrswvddmloverrideR = crate::BitReader;
#[doc = "Field `PWRSWVDDMLOVERRIDE` writer - override enable for pwrsw_vddml_dynsel"]
pub type PwrswvddmloverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSWVDDRCPUDYNSEL` reader - override value for pwrsw_vddrcpu_dynsel"]
pub type PwrswvddrcpudynselR = crate::FieldReader;
#[doc = "Field `PWRSWVDDRCPUDYNSEL` writer - override value for pwrsw_vddrcpu_dynsel"]
pub type PwrswvddrcpudynselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWRSWVDDRCPUPGN` reader - override value for pwrsw_vddrcpu_pgn"]
pub type PwrswvddrcpupgnR = crate::BitReader;
#[doc = "Field `PWRSWVDDRCPUPGN` writer - override value for pwrsw_vddrcpu_pgn"]
pub type PwrswvddrcpupgnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "VDDRCPU power switch static select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrswvddrcpustatsel {
    #[doc = "1: Select VDDC rail"]
    Vddc = 1,
    #[doc = "0: Select VDDFLP rail"]
    Vddflp = 0,
}
impl From<Pwrswvddrcpustatsel> for bool {
    #[inline(always)]
    fn from(variant: Pwrswvddrcpustatsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSWVDDRCPUSTATSEL` reader - VDDRCPU power switch static select"]
pub type PwrswvddrcpustatselR = crate::BitReader<Pwrswvddrcpustatsel>;
impl PwrswvddrcpustatselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrswvddrcpustatsel {
        match self.bits {
            true => Pwrswvddrcpustatsel::Vddc,
            false => Pwrswvddrcpustatsel::Vddflp,
        }
    }
    #[doc = "Select VDDC rail"]
    #[inline(always)]
    pub fn is_vddc(&self) -> bool {
        *self == Pwrswvddrcpustatsel::Vddc
    }
    #[doc = "Select VDDFLP rail"]
    #[inline(always)]
    pub fn is_vddflp(&self) -> bool {
        *self == Pwrswvddrcpustatsel::Vddflp
    }
}
#[doc = "Field `PWRSWVDDRCPUSTATSEL` writer - VDDRCPU power switch static select"]
pub type PwrswvddrcpustatselW<'a, REG> = crate::BitWriter<'a, REG, Pwrswvddrcpustatsel>;
impl<'a, REG> PwrswvddrcpustatselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select VDDC rail"]
    #[inline(always)]
    pub fn vddc(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrswvddrcpustatsel::Vddc)
    }
    #[doc = "Select VDDFLP rail"]
    #[inline(always)]
    pub fn vddflp(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrswvddrcpustatsel::Vddflp)
    }
}
#[doc = "Field `PWRSWVDDRCPUOVERRIDE` reader - override enable for pwrsw_vddrcpu_dynsel and pgn"]
pub type PwrswvddrcpuoverrideR = crate::BitReader;
#[doc = "Field `PWRSWVDDRCPUOVERRIDE` writer - override enable for pwrsw_vddrcpu_dynsel and pgn"]
pub type PwrswvddrcpuoverrideW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - override value for pwrsw_vddcpu_dynsel"]
    #[inline(always)]
    pub fn pwrswvddcpudynsel(&self) -> PwrswvddcpudynselR {
        PwrswvddcpudynselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - override value for pwrsw_vddcpu_pgn"]
    #[inline(always)]
    pub fn pwrswvddcpupgn(&self) -> PwrswvddcpupgnR {
        PwrswvddcpupgnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - override enable for pwrsw_vddcpu_dynsel and pgn"]
    #[inline(always)]
    pub fn pwrswvddcpuoverride(&self) -> PwrswvddcpuoverrideR {
        PwrswvddcpuoverrideR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - override value for pwrsw_vddcaor_dynsel"]
    #[inline(always)]
    pub fn pwrswvddcaordynsel(&self) -> PwrswvddcaordynselR {
        PwrswvddcaordynselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - override enable for pwrsw_vddcaor_dynsel"]
    #[inline(always)]
    pub fn pwrswvddcaoroverride(&self) -> PwrswvddcaoroverrideR {
        PwrswvddcaoroverrideR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - override value for pwrsw_vdddsp0_dynsel"]
    #[inline(always)]
    pub fn pwrswvdddsp0dynsel(&self) -> Pwrswvdddsp0dynselR {
        Pwrswvdddsp0dynselR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - override value for pwrsw_vdddsp0_pgn"]
    #[inline(always)]
    pub fn pwrswvdddsp0pgn(&self) -> Pwrswvdddsp0pgnR {
        Pwrswvdddsp0pgnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - override enable for pwrsw_vdddsp0_dynsel and pgn"]
    #[inline(always)]
    pub fn pwrswvdddsp0override(&self) -> Pwrswvdddsp0overrideR {
        Pwrswvdddsp0overrideR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - override value for pwrsw_vdddsp1_dynsel"]
    #[inline(always)]
    pub fn pwrswvdddsp1dynsel(&self) -> Pwrswvdddsp1dynselR {
        Pwrswvdddsp1dynselR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - override value for pwrsw_vdddsp1_pgn"]
    #[inline(always)]
    pub fn pwrswvdddsp1pgn(&self) -> Pwrswvdddsp1pgnR {
        Pwrswvdddsp1pgnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - override enable for pwrsw_vdddsp1_dynsel and pgn"]
    #[inline(always)]
    pub fn pwrswvdddsp1override(&self) -> Pwrswvdddsp1overrideR {
        Pwrswvdddsp1overrideR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - override value for pwrsw_vddmcpu_dynsel"]
    #[inline(always)]
    pub fn pwrswvddmcpudynsel(&self) -> PwrswvddmcpudynselR {
        PwrswvddmcpudynselR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - VDDMCPU power switch static select"]
    #[inline(always)]
    pub fn pwrswvddmcpustatsel(&self) -> PwrswvddmcpustatselR {
        PwrswvddmcpustatselR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - override enable for pwrsw_vddmcpu_dynsel"]
    #[inline(always)]
    pub fn pwrswvddmcpuoverride(&self) -> PwrswvddmcpuoverrideR {
        PwrswvddmcpuoverrideR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - override value for pwrsw_vddmdsp0_dynsel"]
    #[inline(always)]
    pub fn pwrswvddmdsp0dynsel(&self) -> Pwrswvddmdsp0dynselR {
        Pwrswvddmdsp0dynselR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - VDDMDSP0 power switch static select"]
    #[inline(always)]
    pub fn pwrswvddmdsp0statsel(&self) -> Pwrswvddmdsp0statselR {
        Pwrswvddmdsp0statselR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - override enable for pwrsw_vddmdsp0_dynsel"]
    #[inline(always)]
    pub fn pwrswvddmdsp0override(&self) -> Pwrswvddmdsp0overrideR {
        Pwrswvddmdsp0overrideR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - override value for pwrsw_vddmdsp1_dynsel"]
    #[inline(always)]
    pub fn pwrswvddmdsp1dynsel(&self) -> Pwrswvddmdsp1dynselR {
        Pwrswvddmdsp1dynselR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - VDDMDSP1 power switch static select"]
    #[inline(always)]
    pub fn pwrswvddmdsp1statsel(&self) -> Pwrswvddmdsp1statselR {
        Pwrswvddmdsp1statselR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - override enable for pwrsw_vddmdsp1_dynsel"]
    #[inline(always)]
    pub fn pwrswvddmdsp1override(&self) -> Pwrswvddmdsp1overrideR {
        Pwrswvddmdsp1overrideR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - override value for pwrsw_vddml_dynsel"]
    #[inline(always)]
    pub fn pwrswvddmldynsel(&self) -> PwrswvddmldynselR {
        PwrswvddmldynselR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - VDDML power switch static select"]
    #[inline(always)]
    pub fn pwrswvddmlstatsel(&self) -> PwrswvddmlstatselR {
        PwrswvddmlstatselR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - override enable for pwrsw_vddml_dynsel"]
    #[inline(always)]
    pub fn pwrswvddmloverride(&self) -> PwrswvddmloverrideR {
        PwrswvddmloverrideR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - override value for pwrsw_vddrcpu_dynsel"]
    #[inline(always)]
    pub fn pwrswvddrcpudynsel(&self) -> PwrswvddrcpudynselR {
        PwrswvddrcpudynselR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - override value for pwrsw_vddrcpu_pgn"]
    #[inline(always)]
    pub fn pwrswvddrcpupgn(&self) -> PwrswvddrcpupgnR {
        PwrswvddrcpupgnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - VDDRCPU power switch static select"]
    #[inline(always)]
    pub fn pwrswvddrcpustatsel(&self) -> PwrswvddrcpustatselR {
        PwrswvddrcpustatselR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - override enable for pwrsw_vddrcpu_dynsel and pgn"]
    #[inline(always)]
    pub fn pwrswvddrcpuoverride(&self) -> PwrswvddrcpuoverrideR {
        PwrswvddrcpuoverrideR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - override value for pwrsw_vddcpu_dynsel"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddcpudynsel(&mut self) -> PwrswvddcpudynselW<Pwrsw0Spec> {
        PwrswvddcpudynselW::new(self, 0)
    }
    #[doc = "Bit 2 - override value for pwrsw_vddcpu_pgn"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddcpupgn(&mut self) -> PwrswvddcpupgnW<Pwrsw0Spec> {
        PwrswvddcpupgnW::new(self, 2)
    }
    #[doc = "Bit 3 - override enable for pwrsw_vddcpu_dynsel and pgn"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddcpuoverride(&mut self) -> PwrswvddcpuoverrideW<Pwrsw0Spec> {
        PwrswvddcpuoverrideW::new(self, 3)
    }
    #[doc = "Bits 4:5 - override value for pwrsw_vddcaor_dynsel"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddcaordynsel(&mut self) -> PwrswvddcaordynselW<Pwrsw0Spec> {
        PwrswvddcaordynselW::new(self, 4)
    }
    #[doc = "Bit 6 - override enable for pwrsw_vddcaor_dynsel"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddcaoroverride(&mut self) -> PwrswvddcaoroverrideW<Pwrsw0Spec> {
        PwrswvddcaoroverrideW::new(self, 6)
    }
    #[doc = "Bits 7:8 - override value for pwrsw_vdddsp0_dynsel"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvdddsp0dynsel(&mut self) -> Pwrswvdddsp0dynselW<Pwrsw0Spec> {
        Pwrswvdddsp0dynselW::new(self, 7)
    }
    #[doc = "Bit 9 - override value for pwrsw_vdddsp0_pgn"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvdddsp0pgn(&mut self) -> Pwrswvdddsp0pgnW<Pwrsw0Spec> {
        Pwrswvdddsp0pgnW::new(self, 9)
    }
    #[doc = "Bit 10 - override enable for pwrsw_vdddsp0_dynsel and pgn"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvdddsp0override(&mut self) -> Pwrswvdddsp0overrideW<Pwrsw0Spec> {
        Pwrswvdddsp0overrideW::new(self, 10)
    }
    #[doc = "Bits 11:12 - override value for pwrsw_vdddsp1_dynsel"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvdddsp1dynsel(&mut self) -> Pwrswvdddsp1dynselW<Pwrsw0Spec> {
        Pwrswvdddsp1dynselW::new(self, 11)
    }
    #[doc = "Bit 13 - override value for pwrsw_vdddsp1_pgn"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvdddsp1pgn(&mut self) -> Pwrswvdddsp1pgnW<Pwrsw0Spec> {
        Pwrswvdddsp1pgnW::new(self, 13)
    }
    #[doc = "Bit 14 - override enable for pwrsw_vdddsp1_dynsel and pgn"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvdddsp1override(&mut self) -> Pwrswvdddsp1overrideW<Pwrsw0Spec> {
        Pwrswvdddsp1overrideW::new(self, 14)
    }
    #[doc = "Bit 15 - override value for pwrsw_vddmcpu_dynsel"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddmcpudynsel(&mut self) -> PwrswvddmcpudynselW<Pwrsw0Spec> {
        PwrswvddmcpudynselW::new(self, 15)
    }
    #[doc = "Bit 16 - VDDMCPU power switch static select"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddmcpustatsel(&mut self) -> PwrswvddmcpustatselW<Pwrsw0Spec> {
        PwrswvddmcpustatselW::new(self, 16)
    }
    #[doc = "Bit 17 - override enable for pwrsw_vddmcpu_dynsel"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddmcpuoverride(&mut self) -> PwrswvddmcpuoverrideW<Pwrsw0Spec> {
        PwrswvddmcpuoverrideW::new(self, 17)
    }
    #[doc = "Bit 18 - override value for pwrsw_vddmdsp0_dynsel"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddmdsp0dynsel(&mut self) -> Pwrswvddmdsp0dynselW<Pwrsw0Spec> {
        Pwrswvddmdsp0dynselW::new(self, 18)
    }
    #[doc = "Bit 19 - VDDMDSP0 power switch static select"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddmdsp0statsel(&mut self) -> Pwrswvddmdsp0statselW<Pwrsw0Spec> {
        Pwrswvddmdsp0statselW::new(self, 19)
    }
    #[doc = "Bit 20 - override enable for pwrsw_vddmdsp0_dynsel"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddmdsp0override(&mut self) -> Pwrswvddmdsp0overrideW<Pwrsw0Spec> {
        Pwrswvddmdsp0overrideW::new(self, 20)
    }
    #[doc = "Bit 21 - override value for pwrsw_vddmdsp1_dynsel"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddmdsp1dynsel(&mut self) -> Pwrswvddmdsp1dynselW<Pwrsw0Spec> {
        Pwrswvddmdsp1dynselW::new(self, 21)
    }
    #[doc = "Bit 22 - VDDMDSP1 power switch static select"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddmdsp1statsel(&mut self) -> Pwrswvddmdsp1statselW<Pwrsw0Spec> {
        Pwrswvddmdsp1statselW::new(self, 22)
    }
    #[doc = "Bit 23 - override enable for pwrsw_vddmdsp1_dynsel"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddmdsp1override(&mut self) -> Pwrswvddmdsp1overrideW<Pwrsw0Spec> {
        Pwrswvddmdsp1overrideW::new(self, 23)
    }
    #[doc = "Bit 24 - override value for pwrsw_vddml_dynsel"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddmldynsel(&mut self) -> PwrswvddmldynselW<Pwrsw0Spec> {
        PwrswvddmldynselW::new(self, 24)
    }
    #[doc = "Bit 25 - VDDML power switch static select"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddmlstatsel(&mut self) -> PwrswvddmlstatselW<Pwrsw0Spec> {
        PwrswvddmlstatselW::new(self, 25)
    }
    #[doc = "Bit 26 - override enable for pwrsw_vddml_dynsel"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddmloverride(&mut self) -> PwrswvddmloverrideW<Pwrsw0Spec> {
        PwrswvddmloverrideW::new(self, 26)
    }
    #[doc = "Bits 27:28 - override value for pwrsw_vddrcpu_dynsel"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddrcpudynsel(&mut self) -> PwrswvddrcpudynselW<Pwrsw0Spec> {
        PwrswvddrcpudynselW::new(self, 27)
    }
    #[doc = "Bit 29 - override value for pwrsw_vddrcpu_pgn"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddrcpupgn(&mut self) -> PwrswvddrcpupgnW<Pwrsw0Spec> {
        PwrswvddrcpupgnW::new(self, 29)
    }
    #[doc = "Bit 30 - VDDRCPU power switch static select"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddrcpustatsel(&mut self) -> PwrswvddrcpustatselW<Pwrsw0Spec> {
        PwrswvddrcpustatselW::new(self, 30)
    }
    #[doc = "Bit 31 - override enable for pwrsw_vddrcpu_dynsel and pgn"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddrcpuoverride(&mut self) -> PwrswvddrcpuoverrideW<Pwrsw0Spec> {
        PwrswvddrcpuoverrideW::new(self, 31)
    }
}
#[doc = "PWRSW Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrsw0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrsw0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrsw0Spec;
impl crate::RegisterSpec for Pwrsw0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrsw0::R`](R) reader structure"]
impl crate::Readable for Pwrsw0Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrsw0::W`](W) writer structure"]
impl crate::Writable for Pwrsw0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRSW0 to value 0"]
impl crate::Resettable for Pwrsw0Spec {
    const RESET_VALUE: u32 = 0;
}
