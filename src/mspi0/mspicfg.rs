#[doc = "Register `MSPICFG` reader"]
pub type R = crate::R<MspicfgSpec>;
#[doc = "Register `MSPICFG` writer"]
pub type W = crate::W<MspicfgSpec>;
#[doc = "Enable continuous APB clock. For power-efficient operation, APBCLK should be set to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Apbclk {
    #[doc = "0: Disable continuous clock."]
    Dis = 0,
    #[doc = "1: Enable continuous clock."]
    En = 1,
}
impl From<Apbclk> for bool {
    #[inline(always)]
    fn from(variant: Apbclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APBCLK` reader - Enable continuous APB clock. For power-efficient operation, APBCLK should be set to 0."]
pub type ApbclkR = crate::BitReader<Apbclk>;
impl ApbclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Apbclk {
        match self.bits {
            false => Apbclk::Dis,
            true => Apbclk::En,
        }
    }
    #[doc = "Disable continuous clock."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Apbclk::Dis
    }
    #[doc = "Enable continuous clock."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Apbclk::En
    }
}
#[doc = "Field `APBCLK` writer - Enable continuous APB clock. For power-efficient operation, APBCLK should be set to 0."]
pub type ApbclkW<'a, REG> = crate::BitWriter<'a, REG, Apbclk>;
impl<'a, REG> ApbclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable continuous clock."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Apbclk::Dis)
    }
    #[doc = "Enable continuous clock."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Apbclk::En)
    }
}
#[doc = "Selects which IOM is selected for CQ handshake status.\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iomsel {
    #[doc = "0: ERROR: desc VALUE MISSING"]
    Iom0 = 0,
    #[doc = "1: ERROR: desc VALUE MISSING"]
    Iom1 = 1,
    #[doc = "2: ERROR: desc VALUE MISSING"]
    Iom2 = 2,
    #[doc = "3: ERROR: desc VALUE MISSING"]
    Iom3 = 3,
    #[doc = "4: ERROR: desc VALUE MISSING"]
    Iom4 = 4,
    #[doc = "5: ERROR: desc VALUE MISSING"]
    Iom5 = 5,
    #[doc = "6: ERROR: desc VALUE MISSING"]
    Iom6 = 6,
    #[doc = "7: ERROR: desc VALUE MISSING"]
    Iom7 = 7,
    #[doc = "8: ERROR: desc VALUE MISSING"]
    Mspi0 = 8,
    #[doc = "9: ERROR: desc VALUE MISSING"]
    Mspi1 = 9,
    #[doc = "10: ERROR: desc VALUE MISSING"]
    Mspi2 = 10,
}
impl From<Iomsel> for u8 {
    #[inline(always)]
    fn from(variant: Iomsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iomsel {
    type Ux = u8;
}
impl crate::IsEnum for Iomsel {}
#[doc = "Field `IOMSEL` reader - Selects which IOM is selected for CQ handshake status."]
pub type IomselR = crate::FieldReader<Iomsel>;
impl IomselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Iomsel> {
        match self.bits {
            0 => Some(Iomsel::Iom0),
            1 => Some(Iomsel::Iom1),
            2 => Some(Iomsel::Iom2),
            3 => Some(Iomsel::Iom3),
            4 => Some(Iomsel::Iom4),
            5 => Some(Iomsel::Iom5),
            6 => Some(Iomsel::Iom6),
            7 => Some(Iomsel::Iom7),
            8 => Some(Iomsel::Mspi0),
            9 => Some(Iomsel::Mspi1),
            10 => Some(Iomsel::Mspi2),
            _ => None,
        }
    }
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn is_iom0(&self) -> bool {
        *self == Iomsel::Iom0
    }
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn is_iom1(&self) -> bool {
        *self == Iomsel::Iom1
    }
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn is_iom2(&self) -> bool {
        *self == Iomsel::Iom2
    }
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn is_iom3(&self) -> bool {
        *self == Iomsel::Iom3
    }
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn is_iom4(&self) -> bool {
        *self == Iomsel::Iom4
    }
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn is_iom5(&self) -> bool {
        *self == Iomsel::Iom5
    }
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn is_iom6(&self) -> bool {
        *self == Iomsel::Iom6
    }
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn is_iom7(&self) -> bool {
        *self == Iomsel::Iom7
    }
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn is_mspi0(&self) -> bool {
        *self == Iomsel::Mspi0
    }
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn is_mspi1(&self) -> bool {
        *self == Iomsel::Mspi1
    }
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn is_mspi2(&self) -> bool {
        *self == Iomsel::Mspi2
    }
}
#[doc = "Field `IOMSEL` writer - Selects which IOM is selected for CQ handshake status."]
pub type IomselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Iomsel>;
impl<'a, REG> IomselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn iom0(self) -> &'a mut crate::W<REG> {
        self.variant(Iomsel::Iom0)
    }
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn iom1(self) -> &'a mut crate::W<REG> {
        self.variant(Iomsel::Iom1)
    }
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn iom2(self) -> &'a mut crate::W<REG> {
        self.variant(Iomsel::Iom2)
    }
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn iom3(self) -> &'a mut crate::W<REG> {
        self.variant(Iomsel::Iom3)
    }
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn iom4(self) -> &'a mut crate::W<REG> {
        self.variant(Iomsel::Iom4)
    }
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn iom5(self) -> &'a mut crate::W<REG> {
        self.variant(Iomsel::Iom5)
    }
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn iom6(self) -> &'a mut crate::W<REG> {
        self.variant(Iomsel::Iom6)
    }
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn iom7(self) -> &'a mut crate::W<REG> {
        self.variant(Iomsel::Iom7)
    }
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn mspi0(self) -> &'a mut crate::W<REG> {
        self.variant(Iomsel::Mspi0)
    }
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn mspi1(self) -> &'a mut crate::W<REG> {
        self.variant(Iomsel::Mspi1)
    }
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn mspi2(self) -> &'a mut crate::W<REG> {
        self.variant(Iomsel::Mspi2)
    }
}
#[doc = "Field `FIFORESET` reader - Reset MSPI FIFO (active high). 1=reset FIFO, 0=normal operation. May be used to manually flush the FIFO in error handling."]
pub type FiforesetR = crate::BitReader;
#[doc = "Field `FIFORESET` writer - Reset MSPI FIFO (active high). 1=reset FIFO, 0=normal operation. May be used to manually flush the FIFO in error handling."]
pub type FiforesetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPRSTN` reader - IP block reset. Write to 0 to put the transfer module in reset or 1 for normal operation. This may be required after error conditions to clear the transfer on the bus."]
pub type IprstnR = crate::BitReader;
#[doc = "Field `IPRSTN` writer - IP block reset. Write to 0 to put the transfer module in reset or 1 for normal operation. This may be required after error conditions to clear the transfer on the bus."]
pub type IprstnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSTN` reader - Peripheral reset. Master reset to the entire MSPI module (DMA, XIP, and transfer state machines). 1=normal operation, 0=in reset."]
pub type PrstnR = crate::BitReader;
#[doc = "Field `PRSTN` writer - Peripheral reset. Master reset to the entire MSPI module (DMA, XIP, and transfer state machines). 1=normal operation, 0=in reset."]
pub type PrstnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable continuous APB clock. For power-efficient operation, APBCLK should be set to 0."]
    #[inline(always)]
    pub fn apbclk(&self) -> ApbclkR {
        ApbclkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Selects which IOM is selected for CQ handshake status."]
    #[inline(always)]
    pub fn iomsel(&self) -> IomselR {
        IomselR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Reset MSPI FIFO (active high). 1=reset FIFO, 0=normal operation. May be used to manually flush the FIFO in error handling."]
    #[inline(always)]
    pub fn fiforeset(&self) -> FiforesetR {
        FiforesetR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - IP block reset. Write to 0 to put the transfer module in reset or 1 for normal operation. This may be required after error conditions to clear the transfer on the bus."]
    #[inline(always)]
    pub fn iprstn(&self) -> IprstnR {
        IprstnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Peripheral reset. Master reset to the entire MSPI module (DMA, XIP, and transfer state machines). 1=normal operation, 0=in reset."]
    #[inline(always)]
    pub fn prstn(&self) -> PrstnR {
        PrstnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable continuous APB clock. For power-efficient operation, APBCLK should be set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn apbclk(&mut self) -> ApbclkW<MspicfgSpec> {
        ApbclkW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Selects which IOM is selected for CQ handshake status."]
    #[inline(always)]
    #[must_use]
    pub fn iomsel(&mut self) -> IomselW<MspicfgSpec> {
        IomselW::new(self, 4)
    }
    #[doc = "Bit 29 - Reset MSPI FIFO (active high). 1=reset FIFO, 0=normal operation. May be used to manually flush the FIFO in error handling."]
    #[inline(always)]
    #[must_use]
    pub fn fiforeset(&mut self) -> FiforesetW<MspicfgSpec> {
        FiforesetW::new(self, 29)
    }
    #[doc = "Bit 30 - IP block reset. Write to 0 to put the transfer module in reset or 1 for normal operation. This may be required after error conditions to clear the transfer on the bus."]
    #[inline(always)]
    #[must_use]
    pub fn iprstn(&mut self) -> IprstnW<MspicfgSpec> {
        IprstnW::new(self, 30)
    }
    #[doc = "Bit 31 - Peripheral reset. Master reset to the entire MSPI module (DMA, XIP, and transfer state machines). 1=normal operation, 0=in reset."]
    #[inline(always)]
    #[must_use]
    pub fn prstn(&mut self) -> PrstnW<MspicfgSpec> {
        PrstnW::new(self, 31)
    }
}
#[doc = "Timing configuration bits for the MSPI module. PRSTN, IPRSTN, and FIFORESET can be used to reset portions of the MSPI interface in order to clear error conditions. The remaining bits control clock frequency and TX/RX capture timings.\n\nYou can [`read`](crate::Reg::read) this register and get [`mspicfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspicfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MspicfgSpec;
impl crate::RegisterSpec for MspicfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mspicfg::R`](R) reader structure"]
impl crate::Readable for MspicfgSpec {}
#[doc = "`write(|w| ..)` method takes [`mspicfg::W`](W) writer structure"]
impl crate::Writable for MspicfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSPICFG to value 0xc000_00f0"]
impl crate::Resettable for MspicfgSpec {
    const RESET_VALUE: u32 = 0xc000_00f0;
}
