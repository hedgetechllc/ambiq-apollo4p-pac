#[doc = "Register `IOMDBG` reader"]
pub type R = crate::R<IomdbgSpec>;
#[doc = "Register `IOMDBG` writer"]
pub type W = crate::W<IomdbgSpec>;
#[doc = "Field `DBGEN` reader - Debug Enable. Setting bit will enable the update of data within this register, otherwise it is clock gated for power savings"]
pub type DbgenR = crate::BitReader;
#[doc = "Field `DBGEN` writer - Debug Enable. Setting bit will enable the update of data within this register, otherwise it is clock gated for power savings"]
pub type DbgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCLKON` reader - IOCLK debug clock control. Enable IO_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
pub type IoclkonR = crate::BitReader;
#[doc = "Field `IOCLKON` writer - IOCLK debug clock control. Enable IO_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
pub type IoclkonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APBCLKON` reader - APBCLK debug clock control. Enable APB_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
pub type ApbclkonR = crate::BitReader;
#[doc = "Field `APBCLKON` writer - APBCLK debug clock control. Enable APB_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
pub type ApbclkonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGDATA` reader - Debug control for various options. DBGDATA\\[1:0\\]
is used to select between different debug data available in the DBG0 and DBG1 registers."]
pub type DbgdataR = crate::FieldReader<u32>;
#[doc = "Field `DBGDATA` writer - Debug control for various options. DBGDATA\\[1:0\\]
is used to select between different debug data available in the DBG0 and DBG1 registers."]
pub type DbgdataW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - Debug Enable. Setting bit will enable the update of data within this register, otherwise it is clock gated for power savings"]
    #[inline(always)]
    pub fn dbgen(&self) -> DbgenR {
        DbgenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IOCLK debug clock control. Enable IO_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
    #[inline(always)]
    pub fn ioclkon(&self) -> IoclkonR {
        IoclkonR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - APBCLK debug clock control. Enable APB_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
    #[inline(always)]
    pub fn apbclkon(&self) -> ApbclkonR {
        ApbclkonR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - Debug control for various options. DBGDATA\\[1:0\\]
is used to select between different debug data available in the DBG0 and DBG1 registers."]
    #[inline(always)]
    pub fn dbgdata(&self) -> DbgdataR {
        DbgdataR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Enable. Setting bit will enable the update of data within this register, otherwise it is clock gated for power savings"]
    #[inline(always)]
    #[must_use]
    pub fn dbgen(&mut self) -> DbgenW<IomdbgSpec> {
        DbgenW::new(self, 0)
    }
    #[doc = "Bit 1 - IOCLK debug clock control. Enable IO_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
    #[inline(always)]
    #[must_use]
    pub fn ioclkon(&mut self) -> IoclkonW<IomdbgSpec> {
        IoclkonW::new(self, 1)
    }
    #[doc = "Bit 2 - APBCLK debug clock control. Enable APB_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
    #[inline(always)]
    #[must_use]
    pub fn apbclkon(&mut self) -> ApbclkonW<IomdbgSpec> {
        ApbclkonW::new(self, 2)
    }
    #[doc = "Bits 3:31 - Debug control for various options. DBGDATA\\[1:0\\]
is used to select between different debug data available in the DBG0 and DBG1 registers."]
    #[inline(always)]
    #[must_use]
    pub fn dbgdata(&mut self) -> DbgdataW<IomdbgSpec> {
        DbgdataW::new(self, 3)
    }
}
#[doc = "Debug control\n\nYou can [`read`](crate::Reg::read) this register and get [`iomdbg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomdbg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IomdbgSpec;
impl crate::RegisterSpec for IomdbgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iomdbg::R`](R) reader structure"]
impl crate::Readable for IomdbgSpec {}
#[doc = "`write(|w| ..)` method takes [`iomdbg::W`](W) writer structure"]
impl crate::Writable for IomdbgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOMDBG to value 0"]
impl crate::Resettable for IomdbgSpec {
    const RESET_VALUE: u32 = 0;
}
