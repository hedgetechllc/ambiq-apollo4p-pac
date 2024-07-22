#[doc = "Register `INFO` reader"]
pub type R = crate::R<InfoSpec>;
#[doc = "Register `INFO` writer"]
pub type W = crate::W<InfoSpec>;
#[doc = "Field `InEndPoints` reader - Provides the number of implemented IN Endpoints."]
pub type InEndPointsR = crate::FieldReader;
#[doc = "Field `InEndPoints` writer - Provides the number of implemented IN Endpoints."]
pub type InEndPointsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OutEndPoints` reader - Provides the number of implemented OUT Endpoints."]
pub type OutEndPointsR = crate::FieldReader;
#[doc = "Field `OutEndPoints` writer - Provides the number of implemented OUT Endpoints."]
pub type OutEndPointsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RamBits` reader - Provides the width of the RAM address bus."]
pub type RamBitsR = crate::FieldReader;
#[doc = "Field `RamBits` writer - Provides the width of the RAM address bus."]
pub type RamBitsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RSTS` reader - Soft reset for the CLK domain. cause the output signal NRSTO to be asserted low. This bit is self-clearing. For reset to actually occur, the output NRSTO must be connected to the input NRST."]
pub type RstsR = crate::BitReader;
#[doc = "Field `RSTS` writer - Soft reset for the CLK domain. cause the output signal NRSTO to be asserted low. This bit is self-clearing. For reset to actually occur, the output NRSTO must be connected to the input NRST."]
pub type RstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTXS` reader - Soft reset for the XCLK domain. will cause the output signal NRSTXO to be asserted low. This bit is self-clearing. For reset to actually occur, the output NRSTXO must be connected to the input NRSTX."]
pub type RstxsR = crate::BitReader;
#[doc = "Field `RSTXS` writer - Soft reset for the XCLK domain. will cause the output signal NRSTXO to be asserted low. This bit is self-clearing. For reset to actually occur, the output NRSTXO must be connected to the input NRSTX."]
pub type RstxsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Provides the number of implemented IN Endpoints."]
    #[inline(always)]
    pub fn in_end_points(&self) -> InEndPointsR {
        InEndPointsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Provides the number of implemented OUT Endpoints."]
    #[inline(always)]
    pub fn out_end_points(&self) -> OutEndPointsR {
        OutEndPointsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Provides the width of the RAM address bus."]
    #[inline(always)]
    pub fn ram_bits(&self) -> RamBitsR {
        RamBitsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Soft reset for the CLK domain. cause the output signal NRSTO to be asserted low. This bit is self-clearing. For reset to actually occur, the output NRSTO must be connected to the input NRST."]
    #[inline(always)]
    pub fn rsts(&self) -> RstsR {
        RstsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Soft reset for the XCLK domain. will cause the output signal NRSTXO to be asserted low. This bit is self-clearing. For reset to actually occur, the output NRSTXO must be connected to the input NRSTX."]
    #[inline(always)]
    pub fn rstxs(&self) -> RstxsR {
        RstxsR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Provides the number of implemented IN Endpoints."]
    #[inline(always)]
    #[must_use]
    pub fn in_end_points(&mut self) -> InEndPointsW<InfoSpec> {
        InEndPointsW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Provides the number of implemented OUT Endpoints."]
    #[inline(always)]
    #[must_use]
    pub fn out_end_points(&mut self) -> OutEndPointsW<InfoSpec> {
        OutEndPointsW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Provides the width of the RAM address bus."]
    #[inline(always)]
    #[must_use]
    pub fn ram_bits(&mut self) -> RamBitsW<InfoSpec> {
        RamBitsW::new(self, 8)
    }
    #[doc = "Bit 16 - Soft reset for the CLK domain. cause the output signal NRSTO to be asserted low. This bit is self-clearing. For reset to actually occur, the output NRSTO must be connected to the input NRST."]
    #[inline(always)]
    #[must_use]
    pub fn rsts(&mut self) -> RstsW<InfoSpec> {
        RstsW::new(self, 16)
    }
    #[doc = "Bit 17 - Soft reset for the XCLK domain. will cause the output signal NRSTXO to be asserted low. This bit is self-clearing. For reset to actually occur, the output NRSTXO must be connected to the input NRSTX."]
    #[inline(always)]
    #[must_use]
    pub fn rstxs(&mut self) -> RstxsW<InfoSpec> {
        RstxsW::new(self, 17)
    }
}
#[doc = "Contains read-only info of the number of IN and OUT endpoints included in the design, width of the RAM, the ability to reset the USB Controller via software, a soft reset bit for the CLK clock domain and a soft reset bit for the XCLK clock domain.\n\nYou can [`read`](crate::Reg::read) this register and get [`info::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`info::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InfoSpec;
impl crate::RegisterSpec for InfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`info::R`](R) reader structure"]
impl crate::Readable for InfoSpec {}
#[doc = "`write(|w| ..)` method takes [`info::W`](W) writer structure"]
impl crate::Writable for InfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INFO to value 0"]
impl crate::Resettable for InfoSpec {
    const RESET_VALUE: u32 = 0;
}
