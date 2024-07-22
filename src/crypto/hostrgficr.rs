#[doc = "Register `HOSTRGFICR` reader"]
pub type R = crate::R<HostrgficrSpec>;
#[doc = "Register `HOSTRGFICR` writer"]
pub type W = crate::W<HostrgficrSpec>;
#[doc = "Field `SRAMTODINCLEAR` reader - The SRAM to DIN DMA done interrupt clear."]
pub type SramtodinclearR = crate::BitReader;
#[doc = "Field `SRAMTODINCLEAR` writer - The SRAM to DIN DMA done interrupt clear."]
pub type SramtodinclearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTTOSRAMCLEAR` reader - The DOUT to SRAM DMA done interrupt clear."]
pub type DouttosramclearR = crate::BitReader;
#[doc = "Field `DOUTTOSRAMCLEAR` writer - The DOUT to SRAM DMA done interrupt clear."]
pub type DouttosramclearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMTODINCLEAR` reader - The memory to DIN DMA done interrupt clear."]
pub type MemtodinclearR = crate::BitReader;
#[doc = "Field `MEMTODINCLEAR` writer - The memory to DIN DMA done interrupt clear."]
pub type MemtodinclearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTTOMEMCLEAR` reader - The DOUT to memory DMA done interrupt clear."]
pub type DouttomemclearR = crate::BitReader;
#[doc = "Field `DOUTTOMEMCLEAR` writer - The DOUT to memory DMA done interrupt clear."]
pub type DouttomemclearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXIERRCLEAR` reader - The AXI error interrupt clear."]
pub type AxierrclearR = crate::BitReader;
#[doc = "Field `AXIERRCLEAR` writer - The AXI error interrupt clear."]
pub type AxierrclearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKAEXPCLEAR` reader - The PKA end of operation interrupt clear."]
pub type PkaexpclearR = crate::BitReader;
#[doc = "Field `PKAEXPCLEAR` writer - The PKA end of operation interrupt clear."]
pub type PkaexpclearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGINTCLEAR` reader - The RNG interrupt clear."]
pub type RngintclearR = crate::BitReader;
#[doc = "Field `RNGINTCLEAR` writer - The RNG interrupt clear."]
pub type RngintclearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYMDMACOMPLETEDCLEAR` reader - The GPR interrupt clear."]
pub type SymdmacompletedclearR = crate::BitReader;
#[doc = "Field `SYMDMACOMPLETEDCLEAR` writer - The GPR interrupt clear."]
pub type SymdmacompletedclearW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - The SRAM to DIN DMA done interrupt clear."]
    #[inline(always)]
    pub fn sramtodinclear(&self) -> SramtodinclearR {
        SramtodinclearR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The DOUT to SRAM DMA done interrupt clear."]
    #[inline(always)]
    pub fn douttosramclear(&self) -> DouttosramclearR {
        DouttosramclearR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The memory to DIN DMA done interrupt clear."]
    #[inline(always)]
    pub fn memtodinclear(&self) -> MemtodinclearR {
        MemtodinclearR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The DOUT to memory DMA done interrupt clear."]
    #[inline(always)]
    pub fn douttomemclear(&self) -> DouttomemclearR {
        DouttomemclearR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The AXI error interrupt clear."]
    #[inline(always)]
    pub fn axierrclear(&self) -> AxierrclearR {
        AxierrclearR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The PKA end of operation interrupt clear."]
    #[inline(always)]
    pub fn pkaexpclear(&self) -> PkaexpclearR {
        PkaexpclearR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The RNG interrupt clear."]
    #[inline(always)]
    pub fn rngintclear(&self) -> RngintclearR {
        RngintclearR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The GPR interrupt clear."]
    #[inline(always)]
    pub fn symdmacompletedclear(&self) -> SymdmacompletedclearR {
        SymdmacompletedclearR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - The SRAM to DIN DMA done interrupt clear."]
    #[inline(always)]
    #[must_use]
    pub fn sramtodinclear(&mut self) -> SramtodinclearW<HostrgficrSpec> {
        SramtodinclearW::new(self, 4)
    }
    #[doc = "Bit 5 - The DOUT to SRAM DMA done interrupt clear."]
    #[inline(always)]
    #[must_use]
    pub fn douttosramclear(&mut self) -> DouttosramclearW<HostrgficrSpec> {
        DouttosramclearW::new(self, 5)
    }
    #[doc = "Bit 6 - The memory to DIN DMA done interrupt clear."]
    #[inline(always)]
    #[must_use]
    pub fn memtodinclear(&mut self) -> MemtodinclearW<HostrgficrSpec> {
        MemtodinclearW::new(self, 6)
    }
    #[doc = "Bit 7 - The DOUT to memory DMA done interrupt clear."]
    #[inline(always)]
    #[must_use]
    pub fn douttomemclear(&mut self) -> DouttomemclearW<HostrgficrSpec> {
        DouttomemclearW::new(self, 7)
    }
    #[doc = "Bit 8 - The AXI error interrupt clear."]
    #[inline(always)]
    #[must_use]
    pub fn axierrclear(&mut self) -> AxierrclearW<HostrgficrSpec> {
        AxierrclearW::new(self, 8)
    }
    #[doc = "Bit 9 - The PKA end of operation interrupt clear."]
    #[inline(always)]
    #[must_use]
    pub fn pkaexpclear(&mut self) -> PkaexpclearW<HostrgficrSpec> {
        PkaexpclearW::new(self, 9)
    }
    #[doc = "Bit 10 - The RNG interrupt clear."]
    #[inline(always)]
    #[must_use]
    pub fn rngintclear(&mut self) -> RngintclearW<HostrgficrSpec> {
        RngintclearW::new(self, 10)
    }
    #[doc = "Bit 11 - The GPR interrupt clear."]
    #[inline(always)]
    #[must_use]
    pub fn symdmacompletedclear(&mut self) -> SymdmacompletedclearW<HostrgficrSpec> {
        SymdmacompletedclearW::new(self, 11)
    }
}
#[doc = "Interrupt Clear Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostrgficr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostrgficr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostrgficrSpec;
impl crate::RegisterSpec for HostrgficrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostrgficr::R`](R) reader structure"]
impl crate::Readable for HostrgficrSpec {}
#[doc = "`write(|w| ..)` method takes [`hostrgficr::W`](W) writer structure"]
impl crate::Writable for HostrgficrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTRGFICR to value 0"]
impl crate::Resettable for HostrgficrSpec {
    const RESET_VALUE: u32 = 0;
}
