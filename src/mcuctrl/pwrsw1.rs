#[doc = "Register `PWRSW1` reader"]
pub type R = crate::R<Pwrsw1Spec>;
#[doc = "Register `PWRSW1` writer"]
pub type W = crate::W<Pwrsw1Spec>;
#[doc = "Field `PWRSWVDDRDSP0DYNSEL` reader - override value for pwrsw_vddrdsp0_dynsel"]
pub type Pwrswvddrdsp0dynselR = crate::FieldReader;
#[doc = "Field `PWRSWVDDRDSP0DYNSEL` writer - override value for pwrsw_vddrdsp0_dynsel"]
pub type Pwrswvddrdsp0dynselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWRSWVDDRDSP0PGN` reader - override value for pwrsw_vddrdsp0_pgn"]
pub type Pwrswvddrdsp0pgnR = crate::BitReader;
#[doc = "Field `PWRSWVDDRDSP0PGN` writer - override value for pwrsw_vddrdsp0_pgn"]
pub type Pwrswvddrdsp0pgnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "VDDRDSP0 power switch static select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrswvddrdsp0statsel {
    #[doc = "1: Select VDDC rail"]
    Vddc = 1,
    #[doc = "0: Select VDDFLP rail"]
    Vddflp = 0,
}
impl From<Pwrswvddrdsp0statsel> for bool {
    #[inline(always)]
    fn from(variant: Pwrswvddrdsp0statsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSWVDDRDSP0STATSEL` reader - VDDRDSP0 power switch static select"]
pub type Pwrswvddrdsp0statselR = crate::BitReader<Pwrswvddrdsp0statsel>;
impl Pwrswvddrdsp0statselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrswvddrdsp0statsel {
        match self.bits {
            true => Pwrswvddrdsp0statsel::Vddc,
            false => Pwrswvddrdsp0statsel::Vddflp,
        }
    }
    #[doc = "Select VDDC rail"]
    #[inline(always)]
    pub fn is_vddc(&self) -> bool {
        *self == Pwrswvddrdsp0statsel::Vddc
    }
    #[doc = "Select VDDFLP rail"]
    #[inline(always)]
    pub fn is_vddflp(&self) -> bool {
        *self == Pwrswvddrdsp0statsel::Vddflp
    }
}
#[doc = "Field `PWRSWVDDRDSP0STATSEL` writer - VDDRDSP0 power switch static select"]
pub type Pwrswvddrdsp0statselW<'a, REG> = crate::BitWriter<'a, REG, Pwrswvddrdsp0statsel>;
impl<'a, REG> Pwrswvddrdsp0statselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select VDDC rail"]
    #[inline(always)]
    pub fn vddc(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrswvddrdsp0statsel::Vddc)
    }
    #[doc = "Select VDDFLP rail"]
    #[inline(always)]
    pub fn vddflp(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrswvddrdsp0statsel::Vddflp)
    }
}
#[doc = "Field `PWRSWVDDRDSP0OVERRIDE` reader - override enable for pwrsw_vddrdsp0_dynsel and pgn"]
pub type Pwrswvddrdsp0overrideR = crate::BitReader;
#[doc = "Field `PWRSWVDDRDSP0OVERRIDE` writer - override enable for pwrsw_vddrdsp0_dynsel and pgn"]
pub type Pwrswvddrdsp0overrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSWVDDRDSP1DYNSEL` reader - override value for pwrsw_vddrdsp1_dynsel"]
pub type Pwrswvddrdsp1dynselR = crate::FieldReader;
#[doc = "Field `PWRSWVDDRDSP1DYNSEL` writer - override value for pwrsw_vddrdsp1_dynsel"]
pub type Pwrswvddrdsp1dynselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWRSWVDDRDSP1PGN` reader - override value for pwrsw_vddrdsp1_pgn"]
pub type Pwrswvddrdsp1pgnR = crate::BitReader;
#[doc = "Field `PWRSWVDDRDSP1PGN` writer - override value for pwrsw_vddrdsp1_pgn"]
pub type Pwrswvddrdsp1pgnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "VDDRDSP1 power switch static select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrswvddrdsp1statsel {
    #[doc = "1: Select VDDC rail"]
    Vddc = 1,
    #[doc = "0: Select VDDFLP rail"]
    Vddflp = 0,
}
impl From<Pwrswvddrdsp1statsel> for bool {
    #[inline(always)]
    fn from(variant: Pwrswvddrdsp1statsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSWVDDRDSP1STATSEL` reader - VDDRDSP1 power switch static select"]
pub type Pwrswvddrdsp1statselR = crate::BitReader<Pwrswvddrdsp1statsel>;
impl Pwrswvddrdsp1statselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrswvddrdsp1statsel {
        match self.bits {
            true => Pwrswvddrdsp1statsel::Vddc,
            false => Pwrswvddrdsp1statsel::Vddflp,
        }
    }
    #[doc = "Select VDDC rail"]
    #[inline(always)]
    pub fn is_vddc(&self) -> bool {
        *self == Pwrswvddrdsp1statsel::Vddc
    }
    #[doc = "Select VDDFLP rail"]
    #[inline(always)]
    pub fn is_vddflp(&self) -> bool {
        *self == Pwrswvddrdsp1statsel::Vddflp
    }
}
#[doc = "Field `PWRSWVDDRDSP1STATSEL` writer - VDDRDSP1 power switch static select"]
pub type Pwrswvddrdsp1statselW<'a, REG> = crate::BitWriter<'a, REG, Pwrswvddrdsp1statsel>;
impl<'a, REG> Pwrswvddrdsp1statselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select VDDC rail"]
    #[inline(always)]
    pub fn vddc(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrswvddrdsp1statsel::Vddc)
    }
    #[doc = "Select VDDFLP rail"]
    #[inline(always)]
    pub fn vddflp(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrswvddrdsp1statsel::Vddflp)
    }
}
#[doc = "Field `PWRSWVDDRDSP1OVERRIDE` reader - override enable for pwrsw_vddrdsp1_dynsel and pgn"]
pub type Pwrswvddrdsp1overrideR = crate::BitReader;
#[doc = "Field `PWRSWVDDRDSP1OVERRIDE` writer - override enable for pwrsw_vddrdsp1_dynsel and pgn"]
pub type Pwrswvddrdsp1overrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSWVDDRLDYNSEL` reader - override value for pwrsw_vddrl_dynsel"]
pub type PwrswvddrldynselR = crate::BitReader;
#[doc = "Field `PWRSWVDDRLDYNSEL` writer - override value for pwrsw_vddrl_dynsel"]
pub type PwrswvddrldynselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSWVDDRLPGN` reader - override value for pwrsw_vddrl_pgn"]
pub type PwrswvddrlpgnR = crate::BitReader;
#[doc = "Field `PWRSWVDDRLPGN` writer - override value for pwrsw_vddrl_pgn"]
pub type PwrswvddrlpgnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "VDDRL power switch static select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrswvddrlstatsel {
    #[doc = "1: Select VDDC rail"]
    Vddc = 1,
    #[doc = "0: Select VDDFLP rail"]
    Vddflp = 0,
}
impl From<Pwrswvddrlstatsel> for bool {
    #[inline(always)]
    fn from(variant: Pwrswvddrlstatsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSWVDDRLSTATSEL` reader - VDDRL power switch static select"]
pub type PwrswvddrlstatselR = crate::BitReader<Pwrswvddrlstatsel>;
impl PwrswvddrlstatselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrswvddrlstatsel {
        match self.bits {
            true => Pwrswvddrlstatsel::Vddc,
            false => Pwrswvddrlstatsel::Vddflp,
        }
    }
    #[doc = "Select VDDC rail"]
    #[inline(always)]
    pub fn is_vddc(&self) -> bool {
        *self == Pwrswvddrlstatsel::Vddc
    }
    #[doc = "Select VDDFLP rail"]
    #[inline(always)]
    pub fn is_vddflp(&self) -> bool {
        *self == Pwrswvddrlstatsel::Vddflp
    }
}
#[doc = "Field `PWRSWVDDRLSTATSEL` writer - VDDRL power switch static select"]
pub type PwrswvddrlstatselW<'a, REG> = crate::BitWriter<'a, REG, Pwrswvddrlstatsel>;
impl<'a, REG> PwrswvddrlstatselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select VDDC rail"]
    #[inline(always)]
    pub fn vddc(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrswvddrlstatsel::Vddc)
    }
    #[doc = "Select VDDFLP rail"]
    #[inline(always)]
    pub fn vddflp(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrswvddrlstatsel::Vddflp)
    }
}
#[doc = "Field `PWRSWVDDRLOVERRIDE` reader - override enable for pwrsw_vddrl_dynsel and pgn"]
pub type PwrswvddrloverrideR = crate::BitReader;
#[doc = "Field `PWRSWVDDRLOVERRIDE` writer - override enable for pwrsw_vddrl_dynsel and pgn"]
pub type PwrswvddrloverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSWVDDRMDYNSEL` reader - override value for pwrsw_vddrm_dynsel"]
pub type PwrswvddrmdynselR = crate::BitReader;
#[doc = "Field `PWRSWVDDRMDYNSEL` writer - override value for pwrsw_vddrm_dynsel"]
pub type PwrswvddrmdynselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSWVDDRMPGN` reader - override value for pwrsw_vddrm_pgn"]
pub type PwrswvddrmpgnR = crate::BitReader;
#[doc = "Field `PWRSWVDDRMPGN` writer - override value for pwrsw_vddrm_pgn"]
pub type PwrswvddrmpgnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "VDDRM power switch static select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrswvddrmstatsel {
    #[doc = "1: Select VDDC rail"]
    Vddc = 1,
    #[doc = "0: Select VDDFLP rail"]
    Vddflp = 0,
}
impl From<Pwrswvddrmstatsel> for bool {
    #[inline(always)]
    fn from(variant: Pwrswvddrmstatsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSWVDDRMSTATSEL` reader - VDDRM power switch static select"]
pub type PwrswvddrmstatselR = crate::BitReader<Pwrswvddrmstatsel>;
impl PwrswvddrmstatselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrswvddrmstatsel {
        match self.bits {
            true => Pwrswvddrmstatsel::Vddc,
            false => Pwrswvddrmstatsel::Vddflp,
        }
    }
    #[doc = "Select VDDC rail"]
    #[inline(always)]
    pub fn is_vddc(&self) -> bool {
        *self == Pwrswvddrmstatsel::Vddc
    }
    #[doc = "Select VDDFLP rail"]
    #[inline(always)]
    pub fn is_vddflp(&self) -> bool {
        *self == Pwrswvddrmstatsel::Vddflp
    }
}
#[doc = "Field `PWRSWVDDRMSTATSEL` writer - VDDRM power switch static select"]
pub type PwrswvddrmstatselW<'a, REG> = crate::BitWriter<'a, REG, Pwrswvddrmstatsel>;
impl<'a, REG> PwrswvddrmstatselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select VDDC rail"]
    #[inline(always)]
    pub fn vddc(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrswvddrmstatsel::Vddc)
    }
    #[doc = "Select VDDFLP rail"]
    #[inline(always)]
    pub fn vddflp(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrswvddrmstatsel::Vddflp)
    }
}
#[doc = "Field `PWRSWVDDRMOVERRIDE` reader - override enable for pwrsw_vddrm_dynsel and pgn"]
pub type PwrswvddrmoverrideR = crate::BitReader;
#[doc = "Field `PWRSWVDDRMOVERRIDE` writer - override enable for pwrsw_vddrm_dynsel and pgn"]
pub type PwrswvddrmoverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSWVDDLPGN` reader - override value for pwrsw_vddl_pgn"]
pub type PwrswvddlpgnR = crate::BitReader;
#[doc = "Field `PWRSWVDDLPGN` writer - override value for pwrsw_vddl_pgn"]
pub type PwrswvddlpgnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSWVDDLOVERRIDE` reader - override enable for pwrsw_vddl_pgn"]
pub type PwrswvddloverrideR = crate::BitReader;
#[doc = "Field `PWRSWVDDLOVERRIDE` writer - override enable for pwrsw_vddl_pgn"]
pub type PwrswvddloverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSWCOMPPDNB` reader - pwrsw_comp_pdnb"]
pub type PwrswcomppdnbR = crate::BitReader;
#[doc = "Field `PWRSWCOMPPDNB` writer - pwrsw_comp_pdnb"]
pub type PwrswcomppdnbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSWOVRDRVEN` reader - pwrsw_ovrdrv_en"]
pub type PwrswovrdrvenR = crate::BitReader;
#[doc = "Field `PWRSWOVRDRVEN` writer - pwrsw_ovrdrv_en"]
pub type PwrswovrdrvenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIGPWRSWOVRDRVEN` reader - digpwrsw_ovrdrv_en"]
pub type DigpwrswovrdrvenR = crate::BitReader;
#[doc = "Field `DIGPWRSWOVRDRVEN` writer - digpwrsw_ovrdrv_en"]
pub type DigpwrswovrdrvenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIGPWRSWOVRDRVSEL` reader - digpwrsw_ovrdrv_sel"]
pub type DigpwrswovrdrvselR = crate::FieldReader;
#[doc = "Field `DIGPWRSWOVRDRVSEL` writer - digpwrsw_ovrdrv_sel"]
pub type DigpwrswovrdrvselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USEVDDF4VDDRCPUINHP` reader - Setting this bit selects VDDF for VDDRCPU in when MCU is in HP mode. This is valid for only normal operational mode (i.e without overrides)."]
pub type Usevddf4vddrcpuinhpR = crate::BitReader;
#[doc = "Field `USEVDDF4VDDRCPUINHP` writer - Setting this bit selects VDDF for VDDRCPU in when MCU is in HP mode. This is valid for only normal operational mode (i.e without overrides)."]
pub type Usevddf4vddrcpuinhpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCEVDDRMVDDS` reader - Setting this bit selects VDDS for VDDRM when Flash is off. This is valid for only normal operational mode (i.e. without overrides)."]
pub type ForcevddrmvddsR = crate::BitReader;
#[doc = "Field `FORCEVDDRMVDDS` writer - Setting this bit selects VDDS for VDDRM when Flash is off. This is valid for only normal operational mode (i.e. without overrides)."]
pub type ForcevddrmvddsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCEVDDRMOFF` reader - Setting this bit forces VDDRM to be open when Flash is off. This is valid for only normal operational mode (i.e. without overrides)."]
pub type ForcevddrmoffR = crate::BitReader;
#[doc = "Field `FORCEVDDRMOFF` writer - Setting this bit forces VDDRM to be open when Flash is off. This is valid for only normal operational mode (i.e. without overrides)."]
pub type ForcevddrmoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTVDDCVDDCLVOREN` reader - pwrsw short override select for vddc/vddclv"]
pub type ShortvddcvddclvorenR = crate::BitReader;
#[doc = "Field `SHORTVDDCVDDCLVOREN` writer - pwrsw short override select for vddc/vddclv"]
pub type ShortvddcvddclvorenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTVDDCVDDCLVORVAL` reader - pwrsw short override value for vddc/vddclv"]
pub type ShortvddcvddclvorvalR = crate::BitReader;
#[doc = "Field `SHORTVDDCVDDCLVORVAL` writer - pwrsw short override value for vddc/vddclv"]
pub type ShortvddcvddclvorvalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTVDDFVDDSOREN` reader - pwrsw short override select for vddf/vdds"]
pub type ShortvddfvddsorenR = crate::BitReader;
#[doc = "Field `SHORTVDDFVDDSOREN` writer - pwrsw short override select for vddf/vdds"]
pub type ShortvddfvddsorenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTVDDFVDDSORVAL` reader - pwrsw short override value for vddf/vdds"]
pub type ShortvddfvddsorvalR = crate::BitReader;
#[doc = "Field `SHORTVDDFVDDSORVAL` writer - pwrsw short override value for vddf/vdds"]
pub type ShortvddfvddsorvalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - override value for pwrsw_vddrdsp0_dynsel"]
    #[inline(always)]
    pub fn pwrswvddrdsp0dynsel(&self) -> Pwrswvddrdsp0dynselR {
        Pwrswvddrdsp0dynselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - override value for pwrsw_vddrdsp0_pgn"]
    #[inline(always)]
    pub fn pwrswvddrdsp0pgn(&self) -> Pwrswvddrdsp0pgnR {
        Pwrswvddrdsp0pgnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VDDRDSP0 power switch static select"]
    #[inline(always)]
    pub fn pwrswvddrdsp0statsel(&self) -> Pwrswvddrdsp0statselR {
        Pwrswvddrdsp0statselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - override enable for pwrsw_vddrdsp0_dynsel and pgn"]
    #[inline(always)]
    pub fn pwrswvddrdsp0override(&self) -> Pwrswvddrdsp0overrideR {
        Pwrswvddrdsp0overrideR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - override value for pwrsw_vddrdsp1_dynsel"]
    #[inline(always)]
    pub fn pwrswvddrdsp1dynsel(&self) -> Pwrswvddrdsp1dynselR {
        Pwrswvddrdsp1dynselR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - override value for pwrsw_vddrdsp1_pgn"]
    #[inline(always)]
    pub fn pwrswvddrdsp1pgn(&self) -> Pwrswvddrdsp1pgnR {
        Pwrswvddrdsp1pgnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - VDDRDSP1 power switch static select"]
    #[inline(always)]
    pub fn pwrswvddrdsp1statsel(&self) -> Pwrswvddrdsp1statselR {
        Pwrswvddrdsp1statselR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - override enable for pwrsw_vddrdsp1_dynsel and pgn"]
    #[inline(always)]
    pub fn pwrswvddrdsp1override(&self) -> Pwrswvddrdsp1overrideR {
        Pwrswvddrdsp1overrideR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - override value for pwrsw_vddrl_dynsel"]
    #[inline(always)]
    pub fn pwrswvddrldynsel(&self) -> PwrswvddrldynselR {
        PwrswvddrldynselR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - override value for pwrsw_vddrl_pgn"]
    #[inline(always)]
    pub fn pwrswvddrlpgn(&self) -> PwrswvddrlpgnR {
        PwrswvddrlpgnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - VDDRL power switch static select"]
    #[inline(always)]
    pub fn pwrswvddrlstatsel(&self) -> PwrswvddrlstatselR {
        PwrswvddrlstatselR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - override enable for pwrsw_vddrl_dynsel and pgn"]
    #[inline(always)]
    pub fn pwrswvddrloverride(&self) -> PwrswvddrloverrideR {
        PwrswvddrloverrideR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - override value for pwrsw_vddrm_dynsel"]
    #[inline(always)]
    pub fn pwrswvddrmdynsel(&self) -> PwrswvddrmdynselR {
        PwrswvddrmdynselR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - override value for pwrsw_vddrm_pgn"]
    #[inline(always)]
    pub fn pwrswvddrmpgn(&self) -> PwrswvddrmpgnR {
        PwrswvddrmpgnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - VDDRM power switch static select"]
    #[inline(always)]
    pub fn pwrswvddrmstatsel(&self) -> PwrswvddrmstatselR {
        PwrswvddrmstatselR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - override enable for pwrsw_vddrm_dynsel and pgn"]
    #[inline(always)]
    pub fn pwrswvddrmoverride(&self) -> PwrswvddrmoverrideR {
        PwrswvddrmoverrideR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - override value for pwrsw_vddl_pgn"]
    #[inline(always)]
    pub fn pwrswvddlpgn(&self) -> PwrswvddlpgnR {
        PwrswvddlpgnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - override enable for pwrsw_vddl_pgn"]
    #[inline(always)]
    pub fn pwrswvddloverride(&self) -> PwrswvddloverrideR {
        PwrswvddloverrideR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - pwrsw_comp_pdnb"]
    #[inline(always)]
    pub fn pwrswcomppdnb(&self) -> PwrswcomppdnbR {
        PwrswcomppdnbR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - pwrsw_ovrdrv_en"]
    #[inline(always)]
    pub fn pwrswovrdrven(&self) -> PwrswovrdrvenR {
        PwrswovrdrvenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - digpwrsw_ovrdrv_en"]
    #[inline(always)]
    pub fn digpwrswovrdrven(&self) -> DigpwrswovrdrvenR {
        DigpwrswovrdrvenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:24 - digpwrsw_ovrdrv_sel"]
    #[inline(always)]
    pub fn digpwrswovrdrvsel(&self) -> DigpwrswovrdrvselR {
        DigpwrswovrdrvselR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - Setting this bit selects VDDF for VDDRCPU in when MCU is in HP mode. This is valid for only normal operational mode (i.e without overrides)."]
    #[inline(always)]
    pub fn usevddf4vddrcpuinhp(&self) -> Usevddf4vddrcpuinhpR {
        Usevddf4vddrcpuinhpR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Setting this bit selects VDDS for VDDRM when Flash is off. This is valid for only normal operational mode (i.e. without overrides)."]
    #[inline(always)]
    pub fn forcevddrmvdds(&self) -> ForcevddrmvddsR {
        ForcevddrmvddsR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Setting this bit forces VDDRM to be open when Flash is off. This is valid for only normal operational mode (i.e. without overrides)."]
    #[inline(always)]
    pub fn forcevddrmoff(&self) -> ForcevddrmoffR {
        ForcevddrmoffR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - pwrsw short override select for vddc/vddclv"]
    #[inline(always)]
    pub fn shortvddcvddclvoren(&self) -> ShortvddcvddclvorenR {
        ShortvddcvddclvorenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - pwrsw short override value for vddc/vddclv"]
    #[inline(always)]
    pub fn shortvddcvddclvorval(&self) -> ShortvddcvddclvorvalR {
        ShortvddcvddclvorvalR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - pwrsw short override select for vddf/vdds"]
    #[inline(always)]
    pub fn shortvddfvddsoren(&self) -> ShortvddfvddsorenR {
        ShortvddfvddsorenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - pwrsw short override value for vddf/vdds"]
    #[inline(always)]
    pub fn shortvddfvddsorval(&self) -> ShortvddfvddsorvalR {
        ShortvddfvddsorvalR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - override value for pwrsw_vddrdsp0_dynsel"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddrdsp0dynsel(&mut self) -> Pwrswvddrdsp0dynselW<Pwrsw1Spec> {
        Pwrswvddrdsp0dynselW::new(self, 0)
    }
    #[doc = "Bit 2 - override value for pwrsw_vddrdsp0_pgn"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddrdsp0pgn(&mut self) -> Pwrswvddrdsp0pgnW<Pwrsw1Spec> {
        Pwrswvddrdsp0pgnW::new(self, 2)
    }
    #[doc = "Bit 3 - VDDRDSP0 power switch static select"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddrdsp0statsel(&mut self) -> Pwrswvddrdsp0statselW<Pwrsw1Spec> {
        Pwrswvddrdsp0statselW::new(self, 3)
    }
    #[doc = "Bit 4 - override enable for pwrsw_vddrdsp0_dynsel and pgn"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddrdsp0override(&mut self) -> Pwrswvddrdsp0overrideW<Pwrsw1Spec> {
        Pwrswvddrdsp0overrideW::new(self, 4)
    }
    #[doc = "Bits 5:6 - override value for pwrsw_vddrdsp1_dynsel"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddrdsp1dynsel(&mut self) -> Pwrswvddrdsp1dynselW<Pwrsw1Spec> {
        Pwrswvddrdsp1dynselW::new(self, 5)
    }
    #[doc = "Bit 7 - override value for pwrsw_vddrdsp1_pgn"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddrdsp1pgn(&mut self) -> Pwrswvddrdsp1pgnW<Pwrsw1Spec> {
        Pwrswvddrdsp1pgnW::new(self, 7)
    }
    #[doc = "Bit 8 - VDDRDSP1 power switch static select"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddrdsp1statsel(&mut self) -> Pwrswvddrdsp1statselW<Pwrsw1Spec> {
        Pwrswvddrdsp1statselW::new(self, 8)
    }
    #[doc = "Bit 9 - override enable for pwrsw_vddrdsp1_dynsel and pgn"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddrdsp1override(&mut self) -> Pwrswvddrdsp1overrideW<Pwrsw1Spec> {
        Pwrswvddrdsp1overrideW::new(self, 9)
    }
    #[doc = "Bit 10 - override value for pwrsw_vddrl_dynsel"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddrldynsel(&mut self) -> PwrswvddrldynselW<Pwrsw1Spec> {
        PwrswvddrldynselW::new(self, 10)
    }
    #[doc = "Bit 11 - override value for pwrsw_vddrl_pgn"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddrlpgn(&mut self) -> PwrswvddrlpgnW<Pwrsw1Spec> {
        PwrswvddrlpgnW::new(self, 11)
    }
    #[doc = "Bit 12 - VDDRL power switch static select"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddrlstatsel(&mut self) -> PwrswvddrlstatselW<Pwrsw1Spec> {
        PwrswvddrlstatselW::new(self, 12)
    }
    #[doc = "Bit 13 - override enable for pwrsw_vddrl_dynsel and pgn"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddrloverride(&mut self) -> PwrswvddrloverrideW<Pwrsw1Spec> {
        PwrswvddrloverrideW::new(self, 13)
    }
    #[doc = "Bit 14 - override value for pwrsw_vddrm_dynsel"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddrmdynsel(&mut self) -> PwrswvddrmdynselW<Pwrsw1Spec> {
        PwrswvddrmdynselW::new(self, 14)
    }
    #[doc = "Bit 15 - override value for pwrsw_vddrm_pgn"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddrmpgn(&mut self) -> PwrswvddrmpgnW<Pwrsw1Spec> {
        PwrswvddrmpgnW::new(self, 15)
    }
    #[doc = "Bit 16 - VDDRM power switch static select"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddrmstatsel(&mut self) -> PwrswvddrmstatselW<Pwrsw1Spec> {
        PwrswvddrmstatselW::new(self, 16)
    }
    #[doc = "Bit 17 - override enable for pwrsw_vddrm_dynsel and pgn"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddrmoverride(&mut self) -> PwrswvddrmoverrideW<Pwrsw1Spec> {
        PwrswvddrmoverrideW::new(self, 17)
    }
    #[doc = "Bit 18 - override value for pwrsw_vddl_pgn"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddlpgn(&mut self) -> PwrswvddlpgnW<Pwrsw1Spec> {
        PwrswvddlpgnW::new(self, 18)
    }
    #[doc = "Bit 19 - override enable for pwrsw_vddl_pgn"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswvddloverride(&mut self) -> PwrswvddloverrideW<Pwrsw1Spec> {
        PwrswvddloverrideW::new(self, 19)
    }
    #[doc = "Bit 20 - pwrsw_comp_pdnb"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswcomppdnb(&mut self) -> PwrswcomppdnbW<Pwrsw1Spec> {
        PwrswcomppdnbW::new(self, 20)
    }
    #[doc = "Bit 21 - pwrsw_ovrdrv_en"]
    #[inline(always)]
    #[must_use]
    pub fn pwrswovrdrven(&mut self) -> PwrswovrdrvenW<Pwrsw1Spec> {
        PwrswovrdrvenW::new(self, 21)
    }
    #[doc = "Bit 22 - digpwrsw_ovrdrv_en"]
    #[inline(always)]
    #[must_use]
    pub fn digpwrswovrdrven(&mut self) -> DigpwrswovrdrvenW<Pwrsw1Spec> {
        DigpwrswovrdrvenW::new(self, 22)
    }
    #[doc = "Bits 23:24 - digpwrsw_ovrdrv_sel"]
    #[inline(always)]
    #[must_use]
    pub fn digpwrswovrdrvsel(&mut self) -> DigpwrswovrdrvselW<Pwrsw1Spec> {
        DigpwrswovrdrvselW::new(self, 23)
    }
    #[doc = "Bit 25 - Setting this bit selects VDDF for VDDRCPU in when MCU is in HP mode. This is valid for only normal operational mode (i.e without overrides)."]
    #[inline(always)]
    #[must_use]
    pub fn usevddf4vddrcpuinhp(&mut self) -> Usevddf4vddrcpuinhpW<Pwrsw1Spec> {
        Usevddf4vddrcpuinhpW::new(self, 25)
    }
    #[doc = "Bit 26 - Setting this bit selects VDDS for VDDRM when Flash is off. This is valid for only normal operational mode (i.e. without overrides)."]
    #[inline(always)]
    #[must_use]
    pub fn forcevddrmvdds(&mut self) -> ForcevddrmvddsW<Pwrsw1Spec> {
        ForcevddrmvddsW::new(self, 26)
    }
    #[doc = "Bit 27 - Setting this bit forces VDDRM to be open when Flash is off. This is valid for only normal operational mode (i.e. without overrides)."]
    #[inline(always)]
    #[must_use]
    pub fn forcevddrmoff(&mut self) -> ForcevddrmoffW<Pwrsw1Spec> {
        ForcevddrmoffW::new(self, 27)
    }
    #[doc = "Bit 28 - pwrsw short override select for vddc/vddclv"]
    #[inline(always)]
    #[must_use]
    pub fn shortvddcvddclvoren(&mut self) -> ShortvddcvddclvorenW<Pwrsw1Spec> {
        ShortvddcvddclvorenW::new(self, 28)
    }
    #[doc = "Bit 29 - pwrsw short override value for vddc/vddclv"]
    #[inline(always)]
    #[must_use]
    pub fn shortvddcvddclvorval(&mut self) -> ShortvddcvddclvorvalW<Pwrsw1Spec> {
        ShortvddcvddclvorvalW::new(self, 29)
    }
    #[doc = "Bit 30 - pwrsw short override select for vddf/vdds"]
    #[inline(always)]
    #[must_use]
    pub fn shortvddfvddsoren(&mut self) -> ShortvddfvddsorenW<Pwrsw1Spec> {
        ShortvddfvddsorenW::new(self, 30)
    }
    #[doc = "Bit 31 - pwrsw short override value for vddf/vdds"]
    #[inline(always)]
    #[must_use]
    pub fn shortvddfvddsorval(&mut self) -> ShortvddfvddsorvalW<Pwrsw1Spec> {
        ShortvddfvddsorvalW::new(self, 31)
    }
}
#[doc = "PWRSW Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrsw1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrsw1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrsw1Spec;
impl crate::RegisterSpec for Pwrsw1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrsw1::R`](R) reader structure"]
impl crate::Readable for Pwrsw1Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrsw1::W`](W) writer structure"]
impl crate::Writable for Pwrsw1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRSW1 to value 0x0010_0000"]
impl crate::Resettable for Pwrsw1Spec {
    const RESET_VALUE: u32 = 0x0010_0000;
}
