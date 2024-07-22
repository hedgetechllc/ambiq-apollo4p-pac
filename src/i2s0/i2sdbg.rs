#[doc = "Register `I2SDBG` reader"]
pub type R = crate::R<I2sdbgSpec>;
#[doc = "Register `I2SDBG` writer"]
pub type W = crate::W<I2sdbgSpec>;
#[doc = "Field `DBGEN` reader - Debug Enable. Setting bit will enable the update of data within this register, otherwise it is clock gated for power savings"]
pub type DbgenR = crate::BitReader;
#[doc = "Field `DBGEN` writer - Debug Enable. Setting bit will enable the update of data within this register, otherwise it is clock gated for power savings"]
pub type DbgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCLKON` reader - MCLK debug clock control. Enable MCLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
pub type MclkonR = crate::BitReader;
#[doc = "Field `MCLKON` writer - MCLK debug clock control. Enable MCLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
pub type MclkonW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 1 - MCLK debug clock control. Enable MCLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
    #[inline(always)]
    pub fn mclkon(&self) -> MclkonR {
        MclkonR::new(((self.bits >> 1) & 1) != 0)
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
    pub fn dbgen(&mut self) -> DbgenW<I2sdbgSpec> {
        DbgenW::new(self, 0)
    }
    #[doc = "Bit 1 - MCLK debug clock control. Enable MCLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
    #[inline(always)]
    #[must_use]
    pub fn mclkon(&mut self) -> MclkonW<I2sdbgSpec> {
        MclkonW::new(self, 1)
    }
    #[doc = "Bit 2 - APBCLK debug clock control. Enable APB_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
    #[inline(always)]
    #[must_use]
    pub fn apbclkon(&mut self) -> ApbclkonW<I2sdbgSpec> {
        ApbclkonW::new(self, 2)
    }
    #[doc = "Bits 3:31 - Debug control for various options. DBGDATA\\[1:0\\]
is used to select between different debug data available in the DBG0 and DBG1 registers."]
    #[inline(always)]
    #[must_use]
    pub fn dbgdata(&mut self) -> DbgdataW<I2sdbgSpec> {
        DbgdataW::new(self, 3)
    }
}
#[doc = "Debug control\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sdbg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sdbg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sdbgSpec;
impl crate::RegisterSpec for I2sdbgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2sdbg::R`](R) reader structure"]
impl crate::Readable for I2sdbgSpec {}
#[doc = "`write(|w| ..)` method takes [`i2sdbg::W`](W) writer structure"]
impl crate::Writable for I2sdbgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2SDBG to value 0"]
impl crate::Resettable for I2sdbgSpec {
    const RESET_VALUE: u32 = 0;
}
