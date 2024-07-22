#[doc = "Register `HOSTRGFIRR` reader"]
pub type R = crate::R<HostrgfirrSpec>;
#[doc = "Register `HOSTRGFIRR` writer"]
pub type W = crate::W<HostrgfirrSpec>;
#[doc = "Field `SRAMTODININT` reader - The SRAM to DIN DMA done interrupt status. This interrupt is asserted when all data was delivered to DIN buffer from SRAM."]
pub type SramtodinintR = crate::BitReader;
#[doc = "Field `SRAMTODININT` writer - The SRAM to DIN DMA done interrupt status. This interrupt is asserted when all data was delivered to DIN buffer from SRAM."]
pub type SramtodinintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTTOSRAMINT` reader - The DOUT to SRAM DMA done interrupt status. This interrupt is asserted when all data was delivered to SRAM buffer from DOUT."]
pub type DouttosramintR = crate::BitReader;
#[doc = "Field `DOUTTOSRAMINT` writer - The DOUT to SRAM DMA done interrupt status. This interrupt is asserted when all data was delivered to SRAM buffer from DOUT."]
pub type DouttosramintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMTODININT` reader - The memory to DIN DMA done interrupt status. This interrupt is asserted when all data was delivered to DIN buffer from memory."]
pub type MemtodinintR = crate::BitReader;
#[doc = "Field `MEMTODININT` writer - The memory to DIN DMA done interrupt status. This interrupt is asserted when all data was delivered to DIN buffer from memory."]
pub type MemtodinintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTTOMEMINT` reader - The DOUT to memory DMA done interrupt status. This interrupt is asserted when all data was delivered to memory buffer from DOUT."]
pub type DouttomemintR = crate::BitReader;
#[doc = "Field `DOUTTOMEMINT` writer - The DOUT to memory DMA done interrupt status. This interrupt is asserted when all data was delivered to memory buffer from DOUT."]
pub type DouttomemintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERRINT` reader - The AXI error interrupt status."]
pub type AhberrintR = crate::BitReader;
#[doc = "Field `AHBERRINT` writer - The AXI error interrupt status."]
pub type AhberrintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKAEXPINT` reader - The PKA end of operation interrupt status."]
pub type PkaexpintR = crate::BitReader;
#[doc = "Field `PKAEXPINT` writer - The PKA end of operation interrupt status."]
pub type PkaexpintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGINT` reader - The RNG interrupt status."]
pub type RngintR = crate::BitReader;
#[doc = "Field `RNGINT` writer - The RNG interrupt status."]
pub type RngintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYMDMACOMPLETED` reader - The GPR interrupt status."]
pub type SymdmacompletedR = crate::BitReader;
#[doc = "Field `SYMDMACOMPLETED` writer - The GPR interrupt status."]
pub type SymdmacompletedW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - The SRAM to DIN DMA done interrupt status. This interrupt is asserted when all data was delivered to DIN buffer from SRAM."]
    #[inline(always)]
    pub fn sramtodinint(&self) -> SramtodinintR {
        SramtodinintR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The DOUT to SRAM DMA done interrupt status. This interrupt is asserted when all data was delivered to SRAM buffer from DOUT."]
    #[inline(always)]
    pub fn douttosramint(&self) -> DouttosramintR {
        DouttosramintR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The memory to DIN DMA done interrupt status. This interrupt is asserted when all data was delivered to DIN buffer from memory."]
    #[inline(always)]
    pub fn memtodinint(&self) -> MemtodinintR {
        MemtodinintR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The DOUT to memory DMA done interrupt status. This interrupt is asserted when all data was delivered to memory buffer from DOUT."]
    #[inline(always)]
    pub fn douttomemint(&self) -> DouttomemintR {
        DouttomemintR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The AXI error interrupt status."]
    #[inline(always)]
    pub fn ahberrint(&self) -> AhberrintR {
        AhberrintR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The PKA end of operation interrupt status."]
    #[inline(always)]
    pub fn pkaexpint(&self) -> PkaexpintR {
        PkaexpintR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The RNG interrupt status."]
    #[inline(always)]
    pub fn rngint(&self) -> RngintR {
        RngintR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The GPR interrupt status."]
    #[inline(always)]
    pub fn symdmacompleted(&self) -> SymdmacompletedR {
        SymdmacompletedR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - The SRAM to DIN DMA done interrupt status. This interrupt is asserted when all data was delivered to DIN buffer from SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn sramtodinint(&mut self) -> SramtodinintW<HostrgfirrSpec> {
        SramtodinintW::new(self, 4)
    }
    #[doc = "Bit 5 - The DOUT to SRAM DMA done interrupt status. This interrupt is asserted when all data was delivered to SRAM buffer from DOUT."]
    #[inline(always)]
    #[must_use]
    pub fn douttosramint(&mut self) -> DouttosramintW<HostrgfirrSpec> {
        DouttosramintW::new(self, 5)
    }
    #[doc = "Bit 6 - The memory to DIN DMA done interrupt status. This interrupt is asserted when all data was delivered to DIN buffer from memory."]
    #[inline(always)]
    #[must_use]
    pub fn memtodinint(&mut self) -> MemtodinintW<HostrgfirrSpec> {
        MemtodinintW::new(self, 6)
    }
    #[doc = "Bit 7 - The DOUT to memory DMA done interrupt status. This interrupt is asserted when all data was delivered to memory buffer from DOUT."]
    #[inline(always)]
    #[must_use]
    pub fn douttomemint(&mut self) -> DouttomemintW<HostrgfirrSpec> {
        DouttomemintW::new(self, 7)
    }
    #[doc = "Bit 8 - The AXI error interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn ahberrint(&mut self) -> AhberrintW<HostrgfirrSpec> {
        AhberrintW::new(self, 8)
    }
    #[doc = "Bit 9 - The PKA end of operation interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn pkaexpint(&mut self) -> PkaexpintW<HostrgfirrSpec> {
        PkaexpintW::new(self, 9)
    }
    #[doc = "Bit 10 - The RNG interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn rngint(&mut self) -> RngintW<HostrgfirrSpec> {
        RngintW::new(self, 10)
    }
    #[doc = "Bit 11 - The GPR interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn symdmacompleted(&mut self) -> SymdmacompletedW<HostrgfirrSpec> {
        SymdmacompletedW::new(self, 11)
    }
}
#[doc = "The Interrupt Request register. Each bit of this register holds the interrupt status of a single interrupt source.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostrgfirr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostrgfirr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostrgfirrSpec;
impl crate::RegisterSpec for HostrgfirrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostrgfirr::R`](R) reader structure"]
impl crate::Readable for HostrgfirrSpec {}
#[doc = "`write(|w| ..)` method takes [`hostrgfirr::W`](W) writer structure"]
impl crate::Writable for HostrgfirrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTRGFIRR to value 0"]
impl crate::Resettable for HostrgfirrSpec {
    const RESET_VALUE: u32 = 0;
}
