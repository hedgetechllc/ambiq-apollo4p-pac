#[doc = "Register `DMACFG` reader"]
pub type R = crate::R<DmacfgSpec>;
#[doc = "Register `DMACFG` writer"]
pub type W = crate::W<DmacfgSpec>;
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaen {
    #[doc = "0: Disable DMA Function"]
    Dis = 0,
    #[doc = "1: Enable DMA Function"]
    En = 1,
}
impl From<Dmaen> for bool {
    #[inline(always)]
    fn from(variant: Dmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub type DmaenR = crate::BitReader<Dmaen>;
impl DmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaen {
        match self.bits {
            false => Dmaen::Dis,
            true => Dmaen::En,
        }
    }
    #[doc = "Disable DMA Function"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dmaen::Dis
    }
    #[doc = "Enable DMA Function"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dmaen::En
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG, Dmaen>;
impl<'a, REG> DmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable DMA Function"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::Dis)
    }
    #[doc = "Enable DMA Function"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::En)
    }
}
#[doc = "Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmadir {
    #[doc = "0: Peripheral to Memory (SRAM) transaction"]
    P2m = 0,
    #[doc = "1: Memory to Peripheral transaction"]
    M2p = 1,
}
impl From<Dmadir> for bool {
    #[inline(always)]
    fn from(variant: Dmadir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMADIR` reader - Direction"]
pub type DmadirR = crate::BitReader<Dmadir>;
impl DmadirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmadir {
        match self.bits {
            false => Dmadir::P2m,
            true => Dmadir::M2p,
        }
    }
    #[doc = "Peripheral to Memory (SRAM) transaction"]
    #[inline(always)]
    pub fn is_p2m(&self) -> bool {
        *self == Dmadir::P2m
    }
    #[doc = "Memory to Peripheral transaction"]
    #[inline(always)]
    pub fn is_m2p(&self) -> bool {
        *self == Dmadir::M2p
    }
}
#[doc = "Field `DMADIR` writer - Direction"]
pub type DmadirW<'a, REG> = crate::BitWriter<'a, REG, Dmadir>;
impl<'a, REG> DmadirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral to Memory (SRAM) transaction"]
    #[inline(always)]
    pub fn p2m(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadir::P2m)
    }
    #[doc = "Memory to Peripheral transaction"]
    #[inline(always)]
    pub fn m2p(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadir::M2p)
    }
}
#[doc = "Sets the Priority of the DMA request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmapri {
    #[doc = "0: Low Priority (service as best effort)"]
    Low = 0,
    #[doc = "1: High Priority (service immediately)"]
    High = 1,
}
impl From<Dmapri> for bool {
    #[inline(always)]
    fn from(variant: Dmapri) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAPRI` reader - Sets the Priority of the DMA request"]
pub type DmapriR = crate::BitReader<Dmapri>;
impl DmapriR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmapri {
        match self.bits {
            false => Dmapri::Low,
            true => Dmapri::High,
        }
    }
    #[doc = "Low Priority (service as best effort)"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Dmapri::Low
    }
    #[doc = "High Priority (service immediately)"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Dmapri::High
    }
}
#[doc = "Field `DMAPRI` writer - Sets the Priority of the DMA request"]
pub type DmapriW<'a, REG> = crate::BitWriter<'a, REG, Dmapri>;
impl<'a, REG> DmapriW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low Priority (service as best effort)"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Dmapri::Low)
    }
    #[doc = "High Priority (service immediately)"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Dmapri::High)
    }
}
#[doc = "Enables dynamic priority based on FIFO fullness. When FIFO is full, priority is automatically set to HIGH. Otherwise, DMAPRI is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmadynpri {
    #[doc = "0: Disable dynamic priority (use DMAPRI setting only)"]
    Dis = 0,
    #[doc = "1: Enable dynamic priority"]
    En = 1,
}
impl From<Dmadynpri> for bool {
    #[inline(always)]
    fn from(variant: Dmadynpri) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMADYNPRI` reader - Enables dynamic priority based on FIFO fullness. When FIFO is full, priority is automatically set to HIGH. Otherwise, DMAPRI is used."]
pub type DmadynpriR = crate::BitReader<Dmadynpri>;
impl DmadynpriR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmadynpri {
        match self.bits {
            false => Dmadynpri::Dis,
            true => Dmadynpri::En,
        }
    }
    #[doc = "Disable dynamic priority (use DMAPRI setting only)"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dmadynpri::Dis
    }
    #[doc = "Enable dynamic priority"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dmadynpri::En
    }
}
#[doc = "Field `DMADYNPRI` writer - Enables dynamic priority based on FIFO fullness. When FIFO is full, priority is automatically set to HIGH. Otherwise, DMAPRI is used."]
pub type DmadynpriW<'a, REG> = crate::BitWriter<'a, REG, Dmadynpri>;
impl<'a, REG> DmadynpriW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable dynamic priority (use DMAPRI setting only)"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadynpri::Dis)
    }
    #[doc = "Enable dynamic priority"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadynpri::En)
    }
}
#[doc = "Mask the FIFOCNT and SLOTNUM when transferring FIFO contents to memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmamsk {
    #[doc = "0: FIFO Contents are copied directly to memory without modification."]
    Dis = 0,
    #[doc = "1: Only the FIFODATA contents are copied to memory on DMA transfers. The SLOTNUM and FIFOCNT contents are cleared to zero."]
    En = 1,
}
impl From<Dmamsk> for bool {
    #[inline(always)]
    fn from(variant: Dmamsk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMSK` reader - Mask the FIFOCNT and SLOTNUM when transferring FIFO contents to memory"]
pub type DmamskR = crate::BitReader<Dmamsk>;
impl DmamskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmamsk {
        match self.bits {
            false => Dmamsk::Dis,
            true => Dmamsk::En,
        }
    }
    #[doc = "FIFO Contents are copied directly to memory without modification."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dmamsk::Dis
    }
    #[doc = "Only the FIFODATA contents are copied to memory on DMA transfers. The SLOTNUM and FIFOCNT contents are cleared to zero."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dmamsk::En
    }
}
#[doc = "Field `DMAMSK` writer - Mask the FIFOCNT and SLOTNUM when transferring FIFO contents to memory"]
pub type DmamskW<'a, REG> = crate::BitWriter<'a, REG, Dmamsk>;
impl<'a, REG> DmamskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FIFO Contents are copied directly to memory without modification."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Dmamsk::Dis)
    }
    #[doc = "Only the FIFODATA contents are copied to memory on DMA transfers. The SLOTNUM and FIFOCNT contents are cleared to zero."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Dmamsk::En)
    }
}
#[doc = "Field `DPWROFF` reader - Power Off the ADC System upon DMACPL."]
pub type DpwroffR = crate::BitReader;
#[doc = "Field `DPWROFF` writer - Power Off the ADC System upon DMACPL."]
pub type DpwroffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Direction"]
    #[inline(always)]
    pub fn dmadir(&self) -> DmadirR {
        DmadirR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Sets the Priority of the DMA request"]
    #[inline(always)]
    pub fn dmapri(&self) -> DmapriR {
        DmapriR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enables dynamic priority based on FIFO fullness. When FIFO is full, priority is automatically set to HIGH. Otherwise, DMAPRI is used."]
    #[inline(always)]
    pub fn dmadynpri(&self) -> DmadynpriR {
        DmadynpriR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17 - Mask the FIFOCNT and SLOTNUM when transferring FIFO contents to memory"]
    #[inline(always)]
    pub fn dmamsk(&self) -> DmamskR {
        DmamskR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Power Off the ADC System upon DMACPL."]
    #[inline(always)]
    pub fn dpwroff(&self) -> DpwroffR {
        DpwroffR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DmaenW<DmacfgSpec> {
        DmaenW::new(self, 0)
    }
    #[doc = "Bit 2 - Direction"]
    #[inline(always)]
    #[must_use]
    pub fn dmadir(&mut self) -> DmadirW<DmacfgSpec> {
        DmadirW::new(self, 2)
    }
    #[doc = "Bit 8 - Sets the Priority of the DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn dmapri(&mut self) -> DmapriW<DmacfgSpec> {
        DmapriW::new(self, 8)
    }
    #[doc = "Bit 9 - Enables dynamic priority based on FIFO fullness. When FIFO is full, priority is automatically set to HIGH. Otherwise, DMAPRI is used."]
    #[inline(always)]
    #[must_use]
    pub fn dmadynpri(&mut self) -> DmadynpriW<DmacfgSpec> {
        DmadynpriW::new(self, 9)
    }
    #[doc = "Bit 17 - Mask the FIFOCNT and SLOTNUM when transferring FIFO contents to memory"]
    #[inline(always)]
    #[must_use]
    pub fn dmamsk(&mut self) -> DmamskW<DmacfgSpec> {
        DmamskW::new(self, 17)
    }
    #[doc = "Bit 18 - Power Off the ADC System upon DMACPL."]
    #[inline(always)]
    #[must_use]
    pub fn dpwroff(&mut self) -> DpwroffW<DmacfgSpec> {
        DpwroffW::new(self, 18)
    }
}
#[doc = "DMA Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacfgSpec;
impl crate::RegisterSpec for DmacfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacfg::R`](R) reader structure"]
impl crate::Readable for DmacfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacfg::W`](W) writer structure"]
impl crate::Writable for DmacfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACFG to value 0"]
impl crate::Resettable for DmacfgSpec {
    const RESET_VALUE: u32 = 0;
}
