#[doc = "Register `DMACFG` reader"]
pub type R = crate::R<DmacfgSpec>;
#[doc = "Register `DMACFG` writer"]
pub type W = crate::W<DmacfgSpec>;
#[doc = "DMA Enable. Setting this bit to EN will start the DMA operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmaen {
    #[doc = "0: Disable DMA Function"]
    Dis = 0,
    #[doc = "3: Enable HW controlled DMA Function to manage DMA to flash devices. HW will automatically handle issuance of instruction/address bytes based on settings in the FLASH register."]
    En = 3,
}
impl From<Dmaen> for u8 {
    #[inline(always)]
    fn from(variant: Dmaen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmaen {
    type Ux = u8;
}
impl crate::IsEnum for Dmaen {}
#[doc = "Field `DMAEN` reader - DMA Enable. Setting this bit to EN will start the DMA operation"]
pub type DmaenR = crate::FieldReader<Dmaen>;
impl DmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dmaen> {
        match self.bits {
            0 => Some(Dmaen::Dis),
            3 => Some(Dmaen::En),
            _ => None,
        }
    }
    #[doc = "Disable DMA Function"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dmaen::Dis
    }
    #[doc = "Enable HW controlled DMA Function to manage DMA to flash devices. HW will automatically handle issuance of instruction/address bytes based on settings in the FLASH register."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dmaen::En
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable. Setting this bit to EN will start the DMA operation"]
pub type DmaenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dmaen>;
impl<'a, REG> DmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable DMA Function"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::Dis)
    }
    #[doc = "Enable HW controlled DMA Function to manage DMA to flash devices. HW will automatically handle issuance of instruction/address bytes based on settings in the FLASH register."]
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
#[doc = "DMA Device Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmadev {
    #[doc = "0: Select Device 0 for DMA"]
    Dev0 = 0,
}
impl From<Dmadev> for bool {
    #[inline(always)]
    fn from(variant: Dmadev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMADEV` reader - DMA Device Select"]
pub type DmadevR = crate::BitReader<Dmadev>;
impl DmadevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dmadev> {
        match self.bits {
            false => Some(Dmadev::Dev0),
            _ => None,
        }
    }
    #[doc = "Select Device 0 for DMA"]
    #[inline(always)]
    pub fn is_dev0(&self) -> bool {
        *self == Dmadev::Dev0
    }
}
#[doc = "Field `DMADEV` writer - DMA Device Select"]
pub type DmadevW<'a, REG> = crate::BitWriter<'a, REG, Dmadev>;
impl<'a, REG> DmadevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select Device 0 for DMA"]
    #[inline(always)]
    pub fn dev0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadev::Dev0)
    }
}
#[doc = "Sets the Priority of the DMA request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmapri {
    #[doc = "0: Low Priority (service as best effort)"]
    Low = 0,
    #[doc = "1: High Priority (service immediately)"]
    High = 1,
    #[doc = "2: Auto Priority (priority raised once TX FIFO empties or RX FIFO fills)"]
    Auto = 2,
}
impl From<Dmapri> for u8 {
    #[inline(always)]
    fn from(variant: Dmapri) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmapri {
    type Ux = u8;
}
impl crate::IsEnum for Dmapri {}
#[doc = "Field `DMAPRI` reader - Sets the Priority of the DMA request"]
pub type DmapriR = crate::FieldReader<Dmapri>;
impl DmapriR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dmapri> {
        match self.bits {
            0 => Some(Dmapri::Low),
            1 => Some(Dmapri::High),
            2 => Some(Dmapri::Auto),
            _ => None,
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
    #[doc = "Auto Priority (priority raised once TX FIFO empties or RX FIFO fills)"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == Dmapri::Auto
    }
}
#[doc = "Field `DMAPRI` writer - Sets the Priority of the DMA request"]
pub type DmapriW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dmapri>;
impl<'a, REG> DmapriW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
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
    #[doc = "Auto Priority (priority raised once TX FIFO empties or RX FIFO fills)"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(Dmapri::Auto)
    }
}
#[doc = "Field `DMATXEMPT` reader - For DMA_M2P, only start when DMA fifo is not empty."]
pub type DmatxemptR = crate::BitReader;
#[doc = "Field `DMATXEMPT` writer - For DMA_M2P, only start when DMA fifo is not empty."]
pub type DmatxemptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAPWROFF` reader - Power off MSPI domain upon completion of DMA operation."]
pub type DmapwroffR = crate::BitReader;
#[doc = "Field `DMAPWROFF` writer - Power off MSPI domain upon completion of DMA operation."]
pub type DmapwroffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - DMA Enable. Setting this bit to EN will start the DMA operation"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Direction"]
    #[inline(always)]
    pub fn dmadir(&self) -> DmadirR {
        DmadirR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Device Select"]
    #[inline(always)]
    pub fn dmadev(&self) -> DmadevR {
        DmadevR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Sets the Priority of the DMA request"]
    #[inline(always)]
    pub fn dmapri(&self) -> DmapriR {
        DmapriR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 17 - For DMA_M2P, only start when DMA fifo is not empty."]
    #[inline(always)]
    pub fn dmatxempt(&self) -> DmatxemptR {
        DmatxemptR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Power off MSPI domain upon completion of DMA operation."]
    #[inline(always)]
    pub fn dmapwroff(&self) -> DmapwroffR {
        DmapwroffR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DMA Enable. Setting this bit to EN will start the DMA operation"]
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
    #[doc = "Bit 3 - DMA Device Select"]
    #[inline(always)]
    #[must_use]
    pub fn dmadev(&mut self) -> DmadevW<DmacfgSpec> {
        DmadevW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Sets the Priority of the DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn dmapri(&mut self) -> DmapriW<DmacfgSpec> {
        DmapriW::new(self, 4)
    }
    #[doc = "Bit 17 - For DMA_M2P, only start when DMA fifo is not empty."]
    #[inline(always)]
    #[must_use]
    pub fn dmatxempt(&mut self) -> DmatxemptW<DmacfgSpec> {
        DmatxemptW::new(self, 17)
    }
    #[doc = "Bit 18 - Power off MSPI domain upon completion of DMA operation."]
    #[inline(always)]
    #[must_use]
    pub fn dmapwroff(&mut self) -> DmapwroffW<DmacfgSpec> {
        DmapwroffW::new(self, 18)
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
