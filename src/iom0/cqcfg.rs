#[doc = "Register `CQCFG` reader"]
pub type R = crate::R<CqcfgSpec>;
#[doc = "Register `CQCFG` writer"]
pub type W = crate::W<CqcfgSpec>;
#[doc = "Command queue enable. When set, will enable the processing of the command queue and fetches of address/data pairs will proceed from the word address within the CQADDR register. Can be disabled using a CQ executed write to this bit as well.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cqen {
    #[doc = "0: Disable CQ Function"]
    Dis = 0,
    #[doc = "1: Enable CQ Function"]
    En = 1,
}
impl From<Cqen> for bool {
    #[inline(always)]
    fn from(variant: Cqen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CQEN` reader - Command queue enable. When set, will enable the processing of the command queue and fetches of address/data pairs will proceed from the word address within the CQADDR register. Can be disabled using a CQ executed write to this bit as well."]
pub type CqenR = crate::BitReader<Cqen>;
impl CqenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cqen {
        match self.bits {
            false => Cqen::Dis,
            true => Cqen::En,
        }
    }
    #[doc = "Disable CQ Function"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Cqen::Dis
    }
    #[doc = "Enable CQ Function"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Cqen::En
    }
}
#[doc = "Field `CQEN` writer - Command queue enable. When set, will enable the processing of the command queue and fetches of address/data pairs will proceed from the word address within the CQADDR register. Can be disabled using a CQ executed write to this bit as well."]
pub type CqenW<'a, REG> = crate::BitWriter<'a, REG, Cqen>;
impl<'a, REG> CqenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable CQ Function"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Cqen::Dis)
    }
    #[doc = "Enable CQ Function"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Cqen::En)
    }
}
#[doc = "Sets the Priority of the command queue dma request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cqpri {
    #[doc = "0: Low Priority (service as best effort)"]
    Low = 0,
    #[doc = "1: High Priority (service immediately)"]
    High = 1,
}
impl From<Cqpri> for bool {
    #[inline(always)]
    fn from(variant: Cqpri) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CQPRI` reader - Sets the Priority of the command queue dma request"]
pub type CqpriR = crate::BitReader<Cqpri>;
impl CqpriR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cqpri {
        match self.bits {
            false => Cqpri::Low,
            true => Cqpri::High,
        }
    }
    #[doc = "Low Priority (service as best effort)"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Cqpri::Low
    }
    #[doc = "High Priority (service immediately)"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Cqpri::High
    }
}
#[doc = "Field `CQPRI` writer - Sets the Priority of the command queue dma request"]
pub type CqpriW<'a, REG> = crate::BitWriter<'a, REG, Cqpri>;
impl<'a, REG> CqpriW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low Priority (service as best effort)"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Cqpri::Low)
    }
    #[doc = "High Priority (service immediately)"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Cqpri::High)
    }
}
#[doc = "Selects the MPSI modules used for sourcing the CQFLAG \\[11:8\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mspiflgsel {
    #[doc = "0: Selects MPSI0 as source of signals used in CGFLAG\\[11:8\\]."]
    Mspi0flgsel = 0,
    #[doc = "1: Selects MPSI1 as source of signals used in CGFLAG\\[11:8\\]."]
    Mspi1flgsel = 1,
    #[doc = "2: Selects MPSI2 as source of signals used in CGFLAG\\[11:8\\]."]
    Mspi2flgsel = 2,
}
impl From<Mspiflgsel> for u8 {
    #[inline(always)]
    fn from(variant: Mspiflgsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mspiflgsel {
    type Ux = u8;
}
impl crate::IsEnum for Mspiflgsel {}
#[doc = "Field `MSPIFLGSEL` reader - Selects the MPSI modules used for sourcing the CQFLAG \\[11:8\\]."]
pub type MspiflgselR = crate::FieldReader<Mspiflgsel>;
impl MspiflgselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mspiflgsel> {
        match self.bits {
            0 => Some(Mspiflgsel::Mspi0flgsel),
            1 => Some(Mspiflgsel::Mspi1flgsel),
            2 => Some(Mspiflgsel::Mspi2flgsel),
            _ => None,
        }
    }
    #[doc = "Selects MPSI0 as source of signals used in CGFLAG\\[11:8\\]."]
    #[inline(always)]
    pub fn is_mspi0flgsel(&self) -> bool {
        *self == Mspiflgsel::Mspi0flgsel
    }
    #[doc = "Selects MPSI1 as source of signals used in CGFLAG\\[11:8\\]."]
    #[inline(always)]
    pub fn is_mspi1flgsel(&self) -> bool {
        *self == Mspiflgsel::Mspi1flgsel
    }
    #[doc = "Selects MPSI2 as source of signals used in CGFLAG\\[11:8\\]."]
    #[inline(always)]
    pub fn is_mspi2flgsel(&self) -> bool {
        *self == Mspiflgsel::Mspi2flgsel
    }
}
#[doc = "Field `MSPIFLGSEL` writer - Selects the MPSI modules used for sourcing the CQFLAG \\[11:8\\]."]
pub type MspiflgselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mspiflgsel>;
impl<'a, REG> MspiflgselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects MPSI0 as source of signals used in CGFLAG\\[11:8\\]."]
    #[inline(always)]
    pub fn mspi0flgsel(self) -> &'a mut crate::W<REG> {
        self.variant(Mspiflgsel::Mspi0flgsel)
    }
    #[doc = "Selects MPSI1 as source of signals used in CGFLAG\\[11:8\\]."]
    #[inline(always)]
    pub fn mspi1flgsel(self) -> &'a mut crate::W<REG> {
        self.variant(Mspiflgsel::Mspi1flgsel)
    }
    #[doc = "Selects MPSI2 as source of signals used in CGFLAG\\[11:8\\]."]
    #[inline(always)]
    pub fn mspi2flgsel(self) -> &'a mut crate::W<REG> {
        self.variant(Mspiflgsel::Mspi2flgsel)
    }
}
impl R {
    #[doc = "Bit 0 - Command queue enable. When set, will enable the processing of the command queue and fetches of address/data pairs will proceed from the word address within the CQADDR register. Can be disabled using a CQ executed write to this bit as well."]
    #[inline(always)]
    pub fn cqen(&self) -> CqenR {
        CqenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sets the Priority of the command queue dma request"]
    #[inline(always)]
    pub fn cqpri(&self) -> CqpriR {
        CqpriR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Selects the MPSI modules used for sourcing the CQFLAG \\[11:8\\]."]
    #[inline(always)]
    pub fn mspiflgsel(&self) -> MspiflgselR {
        MspiflgselR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Command queue enable. When set, will enable the processing of the command queue and fetches of address/data pairs will proceed from the word address within the CQADDR register. Can be disabled using a CQ executed write to this bit as well."]
    #[inline(always)]
    #[must_use]
    pub fn cqen(&mut self) -> CqenW<CqcfgSpec> {
        CqenW::new(self, 0)
    }
    #[doc = "Bit 1 - Sets the Priority of the command queue dma request"]
    #[inline(always)]
    #[must_use]
    pub fn cqpri(&mut self) -> CqpriW<CqcfgSpec> {
        CqpriW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Selects the MPSI modules used for sourcing the CQFLAG \\[11:8\\]."]
    #[inline(always)]
    #[must_use]
    pub fn mspiflgsel(&mut self) -> MspiflgselW<CqcfgSpec> {
        MspiflgselW::new(self, 2)
    }
}
#[doc = "Controls parameters and options for execution of the command queue operation. To enable command queue, create this in memory, set the address, and enable it with a write to CQEN\n\nYou can [`read`](crate::Reg::read) this register and get [`cqcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqcfgSpec;
impl crate::RegisterSpec for CqcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqcfg::R`](R) reader structure"]
impl crate::Readable for CqcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cqcfg::W`](W) writer structure"]
impl crate::Writable for CqcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQCFG to value 0"]
impl crate::Resettable for CqcfgSpec {
    const RESET_VALUE: u32 = 0;
}
