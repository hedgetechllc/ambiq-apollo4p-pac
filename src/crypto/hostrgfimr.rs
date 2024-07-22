#[doc = "Register `HOSTRGFIMR` reader"]
pub type R = crate::R<HostrgfimrSpec>;
#[doc = "Register `HOSTRGFIMR` writer"]
pub type W = crate::W<HostrgfimrSpec>;
#[doc = "Field `SRAMTODINMASK` reader - The SRAM to DIN DMA done interrupt mask."]
pub type SramtodinmaskR = crate::BitReader;
#[doc = "Field `SRAMTODINMASK` writer - The SRAM to DIN DMA done interrupt mask."]
pub type SramtodinmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTTOSRAMMASK` reader - The DOUT to SRAM DMA done interrupt mask."]
pub type DouttosrammaskR = crate::BitReader;
#[doc = "Field `DOUTTOSRAMMASK` writer - The DOUT to SRAM DMA done interrupt mask."]
pub type DouttosrammaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMTODINMASK` reader - The memory to DIN DMA done interrupt mask."]
pub type MemtodinmaskR = crate::BitReader;
#[doc = "Field `MEMTODINMASK` writer - The memory to DIN DMA done interrupt mask."]
pub type MemtodinmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTTOMEMMASK` reader - The DOUT to memory DMA done interrupt mask."]
pub type DouttomemmaskR = crate::BitReader;
#[doc = "Field `DOUTTOMEMMASK` writer - The DOUT to memory DMA done interrupt mask."]
pub type DouttomemmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXIERRMASK` reader - The AXI error interrupt mask."]
pub type AxierrmaskR = crate::BitReader;
#[doc = "Field `AXIERRMASK` writer - The AXI error interrupt mask."]
pub type AxierrmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKAEXPMASK` reader - The PKA end of operation interrupt mask."]
pub type PkaexpmaskR = crate::BitReader;
#[doc = "Field `PKAEXPMASK` writer - The PKA end of operation interrupt mask."]
pub type PkaexpmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGINTMASK` reader - The RNG interrupt mask."]
pub type RngintmaskR = crate::BitReader;
#[doc = "Field `RNGINTMASK` writer - The RNG interrupt mask."]
pub type RngintmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYMDMACOMPLETEDMASK` reader - The GPR interrupt mask."]
pub type SymdmacompletedmaskR = crate::BitReader;
#[doc = "Field `SYMDMACOMPLETEDMASK` writer - The GPR interrupt mask."]
pub type SymdmacompletedmaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - The SRAM to DIN DMA done interrupt mask."]
    #[inline(always)]
    pub fn sramtodinmask(&self) -> SramtodinmaskR {
        SramtodinmaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The DOUT to SRAM DMA done interrupt mask."]
    #[inline(always)]
    pub fn douttosrammask(&self) -> DouttosrammaskR {
        DouttosrammaskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The memory to DIN DMA done interrupt mask."]
    #[inline(always)]
    pub fn memtodinmask(&self) -> MemtodinmaskR {
        MemtodinmaskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The DOUT to memory DMA done interrupt mask."]
    #[inline(always)]
    pub fn douttomemmask(&self) -> DouttomemmaskR {
        DouttomemmaskR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The AXI error interrupt mask."]
    #[inline(always)]
    pub fn axierrmask(&self) -> AxierrmaskR {
        AxierrmaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The PKA end of operation interrupt mask."]
    #[inline(always)]
    pub fn pkaexpmask(&self) -> PkaexpmaskR {
        PkaexpmaskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The RNG interrupt mask."]
    #[inline(always)]
    pub fn rngintmask(&self) -> RngintmaskR {
        RngintmaskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The GPR interrupt mask."]
    #[inline(always)]
    pub fn symdmacompletedmask(&self) -> SymdmacompletedmaskR {
        SymdmacompletedmaskR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - The SRAM to DIN DMA done interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn sramtodinmask(&mut self) -> SramtodinmaskW<HostrgfimrSpec> {
        SramtodinmaskW::new(self, 4)
    }
    #[doc = "Bit 5 - The DOUT to SRAM DMA done interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn douttosrammask(&mut self) -> DouttosrammaskW<HostrgfimrSpec> {
        DouttosrammaskW::new(self, 5)
    }
    #[doc = "Bit 6 - The memory to DIN DMA done interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memtodinmask(&mut self) -> MemtodinmaskW<HostrgfimrSpec> {
        MemtodinmaskW::new(self, 6)
    }
    #[doc = "Bit 7 - The DOUT to memory DMA done interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn douttomemmask(&mut self) -> DouttomemmaskW<HostrgfimrSpec> {
        DouttomemmaskW::new(self, 7)
    }
    #[doc = "Bit 8 - The AXI error interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn axierrmask(&mut self) -> AxierrmaskW<HostrgfimrSpec> {
        AxierrmaskW::new(self, 8)
    }
    #[doc = "Bit 9 - The PKA end of operation interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn pkaexpmask(&mut self) -> PkaexpmaskW<HostrgfimrSpec> {
        PkaexpmaskW::new(self, 9)
    }
    #[doc = "Bit 10 - The RNG interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn rngintmask(&mut self) -> RngintmaskW<HostrgfimrSpec> {
        RngintmaskW::new(self, 10)
    }
    #[doc = "Bit 11 - The GPR interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn symdmacompletedmask(&mut self) -> SymdmacompletedmaskW<HostrgfimrSpec> {
        SymdmacompletedmaskW::new(self, 11)
    }
}
#[doc = "The Interrupt Mask register. Each bit of this register holds the mask of a single interrupt source.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostrgfimr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostrgfimr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostrgfimrSpec;
impl crate::RegisterSpec for HostrgfimrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostrgfimr::R`](R) reader structure"]
impl crate::Readable for HostrgfimrSpec {}
#[doc = "`write(|w| ..)` method takes [`hostrgfimr::W`](W) writer structure"]
impl crate::Writable for HostrgfimrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTRGFIMR to value 0x0ff0"]
impl crate::Resettable for HostrgfimrSpec {
    const RESET_VALUE: u32 = 0x0ff0;
}
