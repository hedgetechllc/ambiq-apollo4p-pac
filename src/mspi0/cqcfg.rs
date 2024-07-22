#[doc = "Register `CQCFG` reader"]
pub type R = crate::R<CqcfgSpec>;
#[doc = "Register `CQCFG` writer"]
pub type W = crate::W<CqcfgSpec>;
#[doc = "Command queue enable. When set, will enable the processing of the command queue\n\nValue on reset: 0"]
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
#[doc = "Field `CQEN` reader - Command queue enable. When set, will enable the processing of the command queue"]
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
#[doc = "Field `CQEN` writer - Command queue enable. When set, will enable the processing of the command queue"]
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
#[doc = "Sets the Priority of the command queue DMA request\n\nValue on reset: 0"]
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
#[doc = "Field `CQPRI` reader - Sets the Priority of the command queue DMA request"]
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
#[doc = "Field `CQPRI` writer - Sets the Priority of the command queue DMA request"]
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
#[doc = "Field `CQPWROFF` reader - Power off MSPI domain upon completion of DMA operation."]
pub type CqpwroffR = crate::BitReader;
#[doc = "Field `CQPWROFF` writer - Power off MSPI domain upon completion of DMA operation."]
pub type CqpwroffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CQAUTOCLEARMASK` reader - Enable clear of CQMASK after each pause operation. This may be useful when using software flags to pause CQ."]
pub type CqautoclearmaskR = crate::BitReader;
#[doc = "Field `CQAUTOCLEARMASK` writer - Enable clear of CQMASK after each pause operation. This may be useful when using software flags to pause CQ."]
pub type CqautoclearmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CQPAUSEOP` reader - CQPAUSEOP register description needed."]
pub type CqpauseopR = crate::BitReader;
#[doc = "Field `CQPAUSEOP` writer - CQPAUSEOP register description needed."]
pub type CqpauseopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command queue enable. When set, will enable the processing of the command queue"]
    #[inline(always)]
    pub fn cqen(&self) -> CqenR {
        CqenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sets the Priority of the command queue DMA request"]
    #[inline(always)]
    pub fn cqpri(&self) -> CqpriR {
        CqpriR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power off MSPI domain upon completion of DMA operation."]
    #[inline(always)]
    pub fn cqpwroff(&self) -> CqpwroffR {
        CqpwroffR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable clear of CQMASK after each pause operation. This may be useful when using software flags to pause CQ."]
    #[inline(always)]
    pub fn cqautoclearmask(&self) -> CqautoclearmaskR {
        CqautoclearmaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CQPAUSEOP register description needed."]
    #[inline(always)]
    pub fn cqpauseop(&self) -> CqpauseopR {
        CqpauseopR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command queue enable. When set, will enable the processing of the command queue"]
    #[inline(always)]
    #[must_use]
    pub fn cqen(&mut self) -> CqenW<CqcfgSpec> {
        CqenW::new(self, 0)
    }
    #[doc = "Bit 1 - Sets the Priority of the command queue DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn cqpri(&mut self) -> CqpriW<CqcfgSpec> {
        CqpriW::new(self, 1)
    }
    #[doc = "Bit 2 - Power off MSPI domain upon completion of DMA operation."]
    #[inline(always)]
    #[must_use]
    pub fn cqpwroff(&mut self) -> CqpwroffW<CqcfgSpec> {
        CqpwroffW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable clear of CQMASK after each pause operation. This may be useful when using software flags to pause CQ."]
    #[inline(always)]
    #[must_use]
    pub fn cqautoclearmask(&mut self) -> CqautoclearmaskW<CqcfgSpec> {
        CqautoclearmaskW::new(self, 3)
    }
    #[doc = "Bit 4 - CQPAUSEOP register description needed."]
    #[inline(always)]
    #[must_use]
    pub fn cqpauseop(&mut self) -> CqpauseopW<CqcfgSpec> {
        CqpauseopW::new(self, 4)
    }
}
#[doc = "This register controls Command Queuing (CQ) operations in a manner similar to the DMACFG register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cqcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
