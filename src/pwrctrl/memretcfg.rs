#[doc = "Register `MEMRETCFG` reader"]
pub type R = crate::R<MemretcfgSpec>;
#[doc = "Register `MEMRETCFG` writer"]
pub type W = crate::W<MemretcfgSpec>;
#[doc = "power down DTCM in deep sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dtcmpwdslp {
    #[doc = "0: All DTCM retained"]
    None = 0,
    #[doc = "1: Group0_DTCM0 powered down in deep sleep (0KB-8KB)"]
    Group0dtcm0 = 1,
    #[doc = "2: Group0_DTCM1 powered down in deep sleep (8KB-128KB)"]
    Group0dtcm1 = 2,
    #[doc = "3: Both DTCMs in group0 are powered down in deep sleep (0KB-128KB)"]
    Group0 = 3,
    #[doc = "6: Group1 and Group0_DTCM1 are powered down in deep sleep (8KB-384KB)"]
    Allbutgroup0dtcm0 = 6,
    #[doc = "4: Group1 DTCM powered down in deep sleep (128KB-384KB)"]
    Group1 = 4,
    #[doc = "7: All DTCMs powered down in deep sleep (0KB-384KB)"]
    All = 7,
}
impl From<Dtcmpwdslp> for u8 {
    #[inline(always)]
    fn from(variant: Dtcmpwdslp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dtcmpwdslp {
    type Ux = u8;
}
impl crate::IsEnum for Dtcmpwdslp {}
#[doc = "Field `DTCMPWDSLP` reader - power down DTCM in deep sleep"]
pub type DtcmpwdslpR = crate::FieldReader<Dtcmpwdslp>;
impl DtcmpwdslpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dtcmpwdslp> {
        match self.bits {
            0 => Some(Dtcmpwdslp::None),
            1 => Some(Dtcmpwdslp::Group0dtcm0),
            2 => Some(Dtcmpwdslp::Group0dtcm1),
            3 => Some(Dtcmpwdslp::Group0),
            6 => Some(Dtcmpwdslp::Allbutgroup0dtcm0),
            4 => Some(Dtcmpwdslp::Group1),
            7 => Some(Dtcmpwdslp::All),
            _ => None,
        }
    }
    #[doc = "All DTCM retained"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Dtcmpwdslp::None
    }
    #[doc = "Group0_DTCM0 powered down in deep sleep (0KB-8KB)"]
    #[inline(always)]
    pub fn is_group0dtcm0(&self) -> bool {
        *self == Dtcmpwdslp::Group0dtcm0
    }
    #[doc = "Group0_DTCM1 powered down in deep sleep (8KB-128KB)"]
    #[inline(always)]
    pub fn is_group0dtcm1(&self) -> bool {
        *self == Dtcmpwdslp::Group0dtcm1
    }
    #[doc = "Both DTCMs in group0 are powered down in deep sleep (0KB-128KB)"]
    #[inline(always)]
    pub fn is_group0(&self) -> bool {
        *self == Dtcmpwdslp::Group0
    }
    #[doc = "Group1 and Group0_DTCM1 are powered down in deep sleep (8KB-384KB)"]
    #[inline(always)]
    pub fn is_allbutgroup0dtcm0(&self) -> bool {
        *self == Dtcmpwdslp::Allbutgroup0dtcm0
    }
    #[doc = "Group1 DTCM powered down in deep sleep (128KB-384KB)"]
    #[inline(always)]
    pub fn is_group1(&self) -> bool {
        *self == Dtcmpwdslp::Group1
    }
    #[doc = "All DTCMs powered down in deep sleep (0KB-384KB)"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == Dtcmpwdslp::All
    }
}
#[doc = "Field `DTCMPWDSLP` writer - power down DTCM in deep sleep"]
pub type DtcmpwdslpW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dtcmpwdslp>;
impl<'a, REG> DtcmpwdslpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "All DTCM retained"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcmpwdslp::None)
    }
    #[doc = "Group0_DTCM0 powered down in deep sleep (0KB-8KB)"]
    #[inline(always)]
    pub fn group0dtcm0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcmpwdslp::Group0dtcm0)
    }
    #[doc = "Group0_DTCM1 powered down in deep sleep (8KB-128KB)"]
    #[inline(always)]
    pub fn group0dtcm1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcmpwdslp::Group0dtcm1)
    }
    #[doc = "Both DTCMs in group0 are powered down in deep sleep (0KB-128KB)"]
    #[inline(always)]
    pub fn group0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcmpwdslp::Group0)
    }
    #[doc = "Group1 and Group0_DTCM1 are powered down in deep sleep (8KB-384KB)"]
    #[inline(always)]
    pub fn allbutgroup0dtcm0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcmpwdslp::Allbutgroup0dtcm0)
    }
    #[doc = "Group1 DTCM powered down in deep sleep (128KB-384KB)"]
    #[inline(always)]
    pub fn group1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcmpwdslp::Group1)
    }
    #[doc = "All DTCMs powered down in deep sleep (0KB-384KB)"]
    #[inline(always)]
    pub fn all(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcmpwdslp::All)
    }
}
#[doc = "Powerdown NVM0 in deep sleep\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nvm0pwdslp {
    #[doc = "1: NVM0 is powered down during deepsleep"]
    En = 1,
    #[doc = "0: NVM0 is kept powered on during deepsleep"]
    Dis = 0,
}
impl From<Nvm0pwdslp> for bool {
    #[inline(always)]
    fn from(variant: Nvm0pwdslp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NVM0PWDSLP` reader - Powerdown NVM0 in deep sleep"]
pub type Nvm0pwdslpR = crate::BitReader<Nvm0pwdslp>;
impl Nvm0pwdslpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nvm0pwdslp {
        match self.bits {
            true => Nvm0pwdslp::En,
            false => Nvm0pwdslp::Dis,
        }
    }
    #[doc = "NVM0 is powered down during deepsleep"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Nvm0pwdslp::En
    }
    #[doc = "NVM0 is kept powered on during deepsleep"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Nvm0pwdslp::Dis
    }
}
#[doc = "Field `NVM0PWDSLP` writer - Powerdown NVM0 in deep sleep"]
pub type Nvm0pwdslpW<'a, REG> = crate::BitWriter<'a, REG, Nvm0pwdslp>;
impl<'a, REG> Nvm0pwdslpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NVM0 is powered down during deepsleep"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Nvm0pwdslp::En)
    }
    #[doc = "NVM0 is kept powered on during deepsleep"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Nvm0pwdslp::Dis)
    }
}
#[doc = "power down cache in deep sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cachepwdslp {
    #[doc = "1: Power down cache in deep sleep"]
    En = 1,
    #[doc = "0: Retain cache in deep sleep"]
    Dis = 0,
}
impl From<Cachepwdslp> for bool {
    #[inline(always)]
    fn from(variant: Cachepwdslp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CACHEPWDSLP` reader - power down cache in deep sleep"]
pub type CachepwdslpR = crate::BitReader<Cachepwdslp>;
impl CachepwdslpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cachepwdslp {
        match self.bits {
            true => Cachepwdslp::En,
            false => Cachepwdslp::Dis,
        }
    }
    #[doc = "Power down cache in deep sleep"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Cachepwdslp::En
    }
    #[doc = "Retain cache in deep sleep"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Cachepwdslp::Dis
    }
}
#[doc = "Field `CACHEPWDSLP` writer - power down cache in deep sleep"]
pub type CachepwdslpW<'a, REG> = crate::BitWriter<'a, REG, Cachepwdslp>;
impl<'a, REG> CachepwdslpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power down cache in deep sleep"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Cachepwdslp::En)
    }
    #[doc = "Retain cache in deep sleep"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Cachepwdslp::Dis)
    }
}
impl R {
    #[doc = "Bits 0:2 - power down DTCM in deep sleep"]
    #[inline(always)]
    pub fn dtcmpwdslp(&self) -> DtcmpwdslpR {
        DtcmpwdslpR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Powerdown NVM0 in deep sleep"]
    #[inline(always)]
    pub fn nvm0pwdslp(&self) -> Nvm0pwdslpR {
        Nvm0pwdslpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - power down cache in deep sleep"]
    #[inline(always)]
    pub fn cachepwdslp(&self) -> CachepwdslpR {
        CachepwdslpR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - power down DTCM in deep sleep"]
    #[inline(always)]
    #[must_use]
    pub fn dtcmpwdslp(&mut self) -> DtcmpwdslpW<MemretcfgSpec> {
        DtcmpwdslpW::new(self, 0)
    }
    #[doc = "Bit 3 - Powerdown NVM0 in deep sleep"]
    #[inline(always)]
    #[must_use]
    pub fn nvm0pwdslp(&mut self) -> Nvm0pwdslpW<MemretcfgSpec> {
        Nvm0pwdslpW::new(self, 3)
    }
    #[doc = "Bit 4 - power down cache in deep sleep"]
    #[inline(always)]
    #[must_use]
    pub fn cachepwdslp(&mut self) -> CachepwdslpW<MemretcfgSpec> {
        CachepwdslpW::new(self, 4)
    }
}
#[doc = "This controls the power down of the SRAM banks in deep sleep mode. If this is set, then the power for that SRAM bank will be gated when the core goes into deep sleep. Upon wake, the data within the SRAMs will be erased. If this is not set, retention voltage will be applied to the SRAM bank when the core goes into deep sleep. Upon wake, the data within the SRAMs are retained. Do not set this if the SRAM bank is used as the target for DMA transfer while CPU in deepsleep.\n\nYou can [`read`](crate::Reg::read) this register and get [`memretcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memretcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemretcfgSpec;
impl crate::RegisterSpec for MemretcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memretcfg::R`](R) reader structure"]
impl crate::Readable for MemretcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`memretcfg::W`](W) writer structure"]
impl crate::Writable for MemretcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMRETCFG to value 0x08"]
impl crate::Resettable for MemretcfgSpec {
    const RESET_VALUE: u32 = 0x08;
}
