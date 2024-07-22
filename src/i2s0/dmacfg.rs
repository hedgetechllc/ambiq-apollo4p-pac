#[doc = "Register `DMACFG` reader"]
pub type R = crate::R<DmacfgSpec>;
#[doc = "Register `DMACFG` writer"]
pub type W = crate::W<DmacfgSpec>;
#[doc = "DMA Enable for RX channel. Setting this bit to EN will start the DMA operation. This should be the last DMA related register set prior to issuing the command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdmaen {
    #[doc = "0: Disable RXDMA Function"]
    Dis = 0,
    #[doc = "1: Enable RXDMA Function"]
    En = 1,
}
impl From<Rxdmaen> for bool {
    #[inline(always)]
    fn from(variant: Rxdmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMAEN` reader - DMA Enable for RX channel. Setting this bit to EN will start the DMA operation. This should be the last DMA related register set prior to issuing the command"]
pub type RxdmaenR = crate::BitReader<Rxdmaen>;
impl RxdmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxdmaen {
        match self.bits {
            false => Rxdmaen::Dis,
            true => Rxdmaen::En,
        }
    }
    #[doc = "Disable RXDMA Function"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rxdmaen::Dis
    }
    #[doc = "Enable RXDMA Function"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Rxdmaen::En
    }
}
#[doc = "Field `RXDMAEN` writer - DMA Enable for RX channel. Setting this bit to EN will start the DMA operation. This should be the last DMA related register set prior to issuing the command"]
pub type RxdmaenW<'a, REG> = crate::BitWriter<'a, REG, Rxdmaen>;
impl<'a, REG> RxdmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable RXDMA Function"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdmaen::Dis)
    }
    #[doc = "Enable RXDMA Function"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdmaen::En)
    }
}
#[doc = "Sets the Priority of the RXDMA request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdmapri {
    #[doc = "0: Low Priority (service as best effort)"]
    Low = 0,
    #[doc = "1: High Priority (service immediately)"]
    High = 1,
}
impl From<Rxdmapri> for bool {
    #[inline(always)]
    fn from(variant: Rxdmapri) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMAPRI` reader - Sets the Priority of the RXDMA request"]
pub type RxdmapriR = crate::BitReader<Rxdmapri>;
impl RxdmapriR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxdmapri {
        match self.bits {
            false => Rxdmapri::Low,
            true => Rxdmapri::High,
        }
    }
    #[doc = "Low Priority (service as best effort)"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Rxdmapri::Low
    }
    #[doc = "High Priority (service immediately)"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Rxdmapri::High
    }
}
#[doc = "Field `RXDMAPRI` writer - Sets the Priority of the RXDMA request"]
pub type RxdmapriW<'a, REG> = crate::BitWriter<'a, REG, Rxdmapri>;
impl<'a, REG> RxdmapriW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low Priority (service as best effort)"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdmapri::Low)
    }
    #[doc = "High Priority (service immediately)"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdmapri::High)
    }
}
#[doc = "DMA Enable for TX channel. Setting this bit to EN will start the DMA operation. This should be the last DMA related register set prior to issuing the command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdmaen {
    #[doc = "0: Disable TXDMA Function"]
    Dis = 0,
    #[doc = "1: Enable TXDMA Function"]
    En = 1,
}
impl From<Txdmaen> for bool {
    #[inline(always)]
    fn from(variant: Txdmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDMAEN` reader - DMA Enable for TX channel. Setting this bit to EN will start the DMA operation. This should be the last DMA related register set prior to issuing the command"]
pub type TxdmaenR = crate::BitReader<Txdmaen>;
impl TxdmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txdmaen {
        match self.bits {
            false => Txdmaen::Dis,
            true => Txdmaen::En,
        }
    }
    #[doc = "Disable TXDMA Function"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Txdmaen::Dis
    }
    #[doc = "Enable TXDMA Function"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Txdmaen::En
    }
}
#[doc = "Field `TXDMAEN` writer - DMA Enable for TX channel. Setting this bit to EN will start the DMA operation. This should be the last DMA related register set prior to issuing the command"]
pub type TxdmaenW<'a, REG> = crate::BitWriter<'a, REG, Txdmaen>;
impl<'a, REG> TxdmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable TXDMA Function"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Txdmaen::Dis)
    }
    #[doc = "Enable TXDMA Function"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Txdmaen::En)
    }
}
#[doc = "Sets the Priority of the TXDMA request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdmapri {
    #[doc = "0: Low Priority (service as best effort)"]
    Low = 0,
    #[doc = "1: High Priority (service immediately)"]
    High = 1,
}
impl From<Txdmapri> for bool {
    #[inline(always)]
    fn from(variant: Txdmapri) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDMAPRI` reader - Sets the Priority of the TXDMA request"]
pub type TxdmapriR = crate::BitReader<Txdmapri>;
impl TxdmapriR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txdmapri {
        match self.bits {
            false => Txdmapri::Low,
            true => Txdmapri::High,
        }
    }
    #[doc = "Low Priority (service as best effort)"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Txdmapri::Low
    }
    #[doc = "High Priority (service immediately)"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Txdmapri::High
    }
}
#[doc = "Field `TXDMAPRI` writer - Sets the Priority of the TXDMA request"]
pub type TxdmapriW<'a, REG> = crate::BitWriter<'a, REG, Txdmapri>;
impl<'a, REG> TxdmapriW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low Priority (service as best effort)"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Txdmapri::Low)
    }
    #[doc = "High Priority (service immediately)"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Txdmapri::High)
    }
}
#[doc = "Field `TXREQCNT` reader - Number of blocks of samples transferred before asserting the TXREQCNT interrupt signal. A block is 8 samples. The interrupt will assert if enabled and after TXREQCNT blocks of data has been transferred to the I2S module from the device. A value of 0 will cause the assertion of the interrupt for every block of transfer done."]
pub type TxreqcntR = crate::FieldReader;
#[doc = "Field `TXREQCNT` writer - Number of blocks of samples transferred before asserting the TXREQCNT interrupt signal. A block is 8 samples. The interrupt will assert if enabled and after TXREQCNT blocks of data has been transferred to the I2S module from the device. A value of 0 will cause the assertion of the interrupt for every block of transfer done."]
pub type TxreqcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RXREQCNT` reader - Number of blocks of samples transferred before asserting the RXREQCNT interrupt signal. A block is 8 samples. The interrupt will assert if enabled and after RXREQCNT blocks of data has been transferred from the I2S into the device. A value of 0 will cause the assertion of the interrupt for every block of transfer done."]
pub type RxreqcntR = crate::FieldReader;
#[doc = "Field `RXREQCNT` writer - Number of blocks of samples transferred before asserting the RXREQCNT interrupt signal. A block is 8 samples. The interrupt will assert if enabled and after RXREQCNT blocks of data has been transferred from the I2S into the device. A value of 0 will cause the assertion of the interrupt for every block of transfer done."]
pub type RxreqcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - DMA Enable for RX channel. Setting this bit to EN will start the DMA operation. This should be the last DMA related register set prior to issuing the command"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RxdmaenR {
        RxdmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sets the Priority of the RXDMA request"]
    #[inline(always)]
    pub fn rxdmapri(&self) -> RxdmapriR {
        RxdmapriR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Enable for TX channel. Setting this bit to EN will start the DMA operation. This should be the last DMA related register set prior to issuing the command"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TxdmaenR {
        TxdmaenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sets the Priority of the TXDMA request"]
    #[inline(always)]
    pub fn txdmapri(&self) -> TxdmapriR {
        TxdmapriR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Number of blocks of samples transferred before asserting the TXREQCNT interrupt signal. A block is 8 samples. The interrupt will assert if enabled and after TXREQCNT blocks of data has been transferred to the I2S module from the device. A value of 0 will cause the assertion of the interrupt for every block of transfer done."]
    #[inline(always)]
    pub fn txreqcnt(&self) -> TxreqcntR {
        TxreqcntR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of blocks of samples transferred before asserting the RXREQCNT interrupt signal. A block is 8 samples. The interrupt will assert if enabled and after RXREQCNT blocks of data has been transferred from the I2S into the device. A value of 0 will cause the assertion of the interrupt for every block of transfer done."]
    #[inline(always)]
    pub fn rxreqcnt(&self) -> RxreqcntR {
        RxreqcntR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable for RX channel. Setting this bit to EN will start the DMA operation. This should be the last DMA related register set prior to issuing the command"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RxdmaenW<DmacfgSpec> {
        RxdmaenW::new(self, 0)
    }
    #[doc = "Bit 1 - Sets the Priority of the RXDMA request"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmapri(&mut self) -> RxdmapriW<DmacfgSpec> {
        RxdmapriW::new(self, 1)
    }
    #[doc = "Bit 4 - DMA Enable for TX channel. Setting this bit to EN will start the DMA operation. This should be the last DMA related register set prior to issuing the command"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TxdmaenW<DmacfgSpec> {
        TxdmaenW::new(self, 4)
    }
    #[doc = "Bit 5 - Sets the Priority of the TXDMA request"]
    #[inline(always)]
    #[must_use]
    pub fn txdmapri(&mut self) -> TxdmapriW<DmacfgSpec> {
        TxdmapriW::new(self, 5)
    }
    #[doc = "Bits 8:15 - Number of blocks of samples transferred before asserting the TXREQCNT interrupt signal. A block is 8 samples. The interrupt will assert if enabled and after TXREQCNT blocks of data has been transferred to the I2S module from the device. A value of 0 will cause the assertion of the interrupt for every block of transfer done."]
    #[inline(always)]
    #[must_use]
    pub fn txreqcnt(&mut self) -> TxreqcntW<DmacfgSpec> {
        TxreqcntW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Number of blocks of samples transferred before asserting the RXREQCNT interrupt signal. A block is 8 samples. The interrupt will assert if enabled and after RXREQCNT blocks of data has been transferred from the I2S into the device. A value of 0 will cause the assertion of the interrupt for every block of transfer done."]
    #[inline(always)]
    #[must_use]
    pub fn rxreqcnt(&mut self) -> RxreqcntW<DmacfgSpec> {
        RxreqcntW::new(self, 16)
    }
}
#[doc = "Configuration control of the DMA process, including the direction of DMA, and enablement of DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
