#[doc = "Register `SKU` reader"]
pub type R = crate::R<SkuSpec>;
#[doc = "Register `SKU` writer"]
pub type W = crate::W<SkuSpec>;
#[doc = "Field `SKUSRAMSIZE` reader - SRAM SKU dictates the available memory for MCU. All of the MCU TCM (384KB) and System SRAM (2MB) available by default. Memory SU disabled"]
pub type SkusramsizeR = crate::FieldReader;
#[doc = "Field `SKUSRAMSIZE` writer - SRAM SKU dictates the available memory for MCU. All of the MCU TCM (384KB) and System SRAM (2MB) available by default. Memory SU disabled"]
pub type SkusramsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SKUMRAMSIZE` reader - MRAM size SKU. All of 2MB Available by default. Memory SU disabled"]
pub type SkumramsizeR = crate::FieldReader;
#[doc = "Field `SKUMRAMSIZE` writer - MRAM size SKU. All of 2MB Available by default. Memory SU disabled"]
pub type SkumramsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SKUDSP` reader - DSP availability SKU setting. 384K available by default. Memory SU disabled"]
pub type SkudspR = crate::FieldReader;
#[doc = "Field `SKUDSP` writer - DSP availability SKU setting. 384K available by default. Memory SU disabled"]
pub type SkudspW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SKUTURBOSPOT` reader - High performance mode for MCU and DSPs."]
pub type SkuturbospotR = crate::BitReader;
#[doc = "Field `SKUTURBOSPOT` writer - High performance mode for MCU and DSPs."]
pub type SkuturbospotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SKUMIPIDSI` reader - MIPI DSI available"]
pub type SkumipidsiR = crate::BitReader;
#[doc = "Field `SKUMIPIDSI` writer - MIPI DSI available"]
pub type SkumipidsiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SKUGFX` reader - GFX available"]
pub type SkugfxR = crate::BitReader;
#[doc = "Field `SKUGFX` writer - GFX available"]
pub type SkugfxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SKUUSB` reader - USB available"]
pub type SkuusbR = crate::BitReader;
#[doc = "Field `SKUUSB` writer - USB available"]
pub type SkuusbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SKUSECURESPOT` reader - Secure boot feature"]
pub type SkusecurespotR = crate::BitReader;
#[doc = "Field `SKUSECURESPOT` writer - Secure boot feature"]
pub type SkusecurespotW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - SRAM SKU dictates the available memory for MCU. All of the MCU TCM (384KB) and System SRAM (2MB) available by default. Memory SU disabled"]
    #[inline(always)]
    pub fn skusramsize(&self) -> SkusramsizeR {
        SkusramsizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - MRAM size SKU. All of 2MB Available by default. Memory SU disabled"]
    #[inline(always)]
    pub fn skumramsize(&self) -> SkumramsizeR {
        SkumramsizeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - DSP availability SKU setting. 384K available by default. Memory SU disabled"]
    #[inline(always)]
    pub fn skudsp(&self) -> SkudspR {
        SkudspR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - High performance mode for MCU and DSPs."]
    #[inline(always)]
    pub fn skuturbospot(&self) -> SkuturbospotR {
        SkuturbospotR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MIPI DSI available"]
    #[inline(always)]
    pub fn skumipidsi(&self) -> SkumipidsiR {
        SkumipidsiR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GFX available"]
    #[inline(always)]
    pub fn skugfx(&self) -> SkugfxR {
        SkugfxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USB available"]
    #[inline(always)]
    pub fn skuusb(&self) -> SkuusbR {
        SkuusbR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Secure boot feature"]
    #[inline(always)]
    pub fn skusecurespot(&self) -> SkusecurespotR {
        SkusecurespotR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SRAM SKU dictates the available memory for MCU. All of the MCU TCM (384KB) and System SRAM (2MB) available by default. Memory SU disabled"]
    #[inline(always)]
    #[must_use]
    pub fn skusramsize(&mut self) -> SkusramsizeW<SkuSpec> {
        SkusramsizeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - MRAM size SKU. All of 2MB Available by default. Memory SU disabled"]
    #[inline(always)]
    #[must_use]
    pub fn skumramsize(&mut self) -> SkumramsizeW<SkuSpec> {
        SkumramsizeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - DSP availability SKU setting. 384K available by default. Memory SU disabled"]
    #[inline(always)]
    #[must_use]
    pub fn skudsp(&mut self) -> SkudspW<SkuSpec> {
        SkudspW::new(self, 4)
    }
    #[doc = "Bit 6 - High performance mode for MCU and DSPs."]
    #[inline(always)]
    #[must_use]
    pub fn skuturbospot(&mut self) -> SkuturbospotW<SkuSpec> {
        SkuturbospotW::new(self, 6)
    }
    #[doc = "Bit 7 - MIPI DSI available"]
    #[inline(always)]
    #[must_use]
    pub fn skumipidsi(&mut self) -> SkumipidsiW<SkuSpec> {
        SkumipidsiW::new(self, 7)
    }
    #[doc = "Bit 8 - GFX available"]
    #[inline(always)]
    #[must_use]
    pub fn skugfx(&mut self) -> SkugfxW<SkuSpec> {
        SkugfxW::new(self, 8)
    }
    #[doc = "Bit 9 - USB available"]
    #[inline(always)]
    #[must_use]
    pub fn skuusb(&mut self) -> SkuusbW<SkuSpec> {
        SkuusbW::new(self, 9)
    }
    #[doc = "Bit 10 - Secure boot feature"]
    #[inline(always)]
    #[must_use]
    pub fn skusecurespot(&mut self) -> SkusecurespotW<SkuSpec> {
        SkusecurespotW::new(self, 10)
    }
}
#[doc = "Unique Chip SKU\n\nYou can [`read`](crate::Reg::read) this register and get [`sku::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sku::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SkuSpec;
impl crate::RegisterSpec for SkuSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sku::R`](R) reader structure"]
impl crate::Readable for SkuSpec {}
#[doc = "`write(|w| ..)` method takes [`sku::W`](W) writer structure"]
impl crate::Writable for SkuSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SKU to value 0x3f"]
impl crate::Resettable for SkuSpec {
    const RESET_VALUE: u32 = 0x3f;
}
