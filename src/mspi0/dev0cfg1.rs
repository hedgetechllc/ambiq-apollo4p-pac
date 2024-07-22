#[doc = "Register `DEV0CFG1` reader"]
pub type R = crate::R<Dev0cfg1Spec>;
#[doc = "Register `DEV0CFG1` writer"]
pub type W = crate::W<Dev0cfg1Spec>;
#[doc = "Field `SFTURN0` reader - Subtract from internal counter of write latency and turnaround"]
pub type Sfturn0R = crate::FieldReader;
#[doc = "Field `SFTURN0` writer - Subtract from internal counter of write latency and turnaround"]
pub type Sfturn0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RXCAPEXT0` reader - Specify the number of apb_clk of RX capture phse {RXCAPEXT, RXCAP}"]
pub type Rxcapext0R = crate::BitReader;
#[doc = "Field `RXCAPEXT0` writer - Specify the number of apb_clk of RX capture phse {RXCAPEXT, RXCAP}"]
pub type Rxcapext0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLKRXHALT0` reader - Halt sclk based on xfer_count"]
pub type Sclkrxhalt0R = crate::BitReader;
#[doc = "Field `SCLKRXHALT0` writer - Halt sclk based on xfer_count"]
pub type Sclkrxhalt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WBX0` reader - Enable the support of WBX ( page boundary crossing on write )"]
pub type Wbx0R = crate::BitReader;
#[doc = "Field `WBX0` writer - Enable the support of WBX ( page boundary crossing on write )"]
pub type Wbx0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBX0` reader - Enable the support of RBX ( page boundary crossing on read )"]
pub type Rbx0R = crate::BitReader;
#[doc = "Field `RBX0` writer - Enable the support of RBX ( page boundary crossing on read )"]
pub type Rbx0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSMP0` reader - Sampling edge based on sclk edge. No effect when div1"]
pub type Rxsmp0R = crate::FieldReader;
#[doc = "Field `RXSMP0` writer - Sampling edge based on sclk edge. No effect when div1"]
pub type Rxsmp0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HYPERIO0` reader - When using Windbond, set this bit to 1 to generate CA\\[47:0\\]
in hardware."]
pub type Hyperio0R = crate::BitReader;
#[doc = "Field `HYPERIO0` writer - When using Windbond, set this bit to 1 to generate CA\\[47:0\\]
in hardware."]
pub type Hyperio0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAFOURTH0` reader - Add 1/4th mspi_clk in DDR to turnaround. Recommended set this to 1 when EMULATEDDR is set in non-DQS mode (ENABLEDQS = 0)."]
pub type Tafourth0R = crate::BitReader;
#[doc = "Field `TAFOURTH0` writer - Add 1/4th mspi_clk in DDR to turnaround. Recommended set this to 1 when EMULATEDDR is set in non-DQS mode (ENABLEDQS = 0)."]
pub type Tafourth0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXHI0` reader - Force st_rx to start at clock high of mspi_clk"]
pub type Rxhi0R = crate::BitReader;
#[doc = "Field `RXHI0` writer - Force st_rx to start at clock high of mspi_clk"]
pub type Rxhi0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQSTURN0` reader - In DQS mode, the internal cycle count to enable DQS path."]
pub type Dqsturn0R = crate::FieldReader;
#[doc = "Field `DQSTURN0` writer - In DQS mode, the internal cycle count to enable DQS path."]
pub type Dqsturn0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - Subtract from internal counter of write latency and turnaround"]
    #[inline(always)]
    pub fn sfturn0(&self) -> Sfturn0R {
        Sfturn0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Specify the number of apb_clk of RX capture phse {RXCAPEXT, RXCAP}"]
    #[inline(always)]
    pub fn rxcapext0(&self) -> Rxcapext0R {
        Rxcapext0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Halt sclk based on xfer_count"]
    #[inline(always)]
    pub fn sclkrxhalt0(&self) -> Sclkrxhalt0R {
        Sclkrxhalt0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable the support of WBX ( page boundary crossing on write )"]
    #[inline(always)]
    pub fn wbx0(&self) -> Wbx0R {
        Wbx0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable the support of RBX ( page boundary crossing on read )"]
    #[inline(always)]
    pub fn rbx0(&self) -> Rbx0R {
        Rbx0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Sampling edge based on sclk edge. No effect when div1"]
    #[inline(always)]
    pub fn rxsmp0(&self) -> Rxsmp0R {
        Rxsmp0R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - When using Windbond, set this bit to 1 to generate CA\\[47:0\\]
in hardware."]
    #[inline(always)]
    pub fn hyperio0(&self) -> Hyperio0R {
        Hyperio0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Add 1/4th mspi_clk in DDR to turnaround. Recommended set this to 1 when EMULATEDDR is set in non-DQS mode (ENABLEDQS = 0)."]
    #[inline(always)]
    pub fn tafourth0(&self) -> Tafourth0R {
        Tafourth0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Force st_rx to start at clock high of mspi_clk"]
    #[inline(always)]
    pub fn rxhi0(&self) -> Rxhi0R {
        Rxhi0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16 - In DQS mode, the internal cycle count to enable DQS path."]
    #[inline(always)]
    pub fn dqsturn0(&self) -> Dqsturn0R {
        Dqsturn0R::new(((self.bits >> 14) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Subtract from internal counter of write latency and turnaround"]
    #[inline(always)]
    #[must_use]
    pub fn sfturn0(&mut self) -> Sfturn0W<Dev0cfg1Spec> {
        Sfturn0W::new(self, 0)
    }
    #[doc = "Bit 4 - Specify the number of apb_clk of RX capture phse {RXCAPEXT, RXCAP}"]
    #[inline(always)]
    #[must_use]
    pub fn rxcapext0(&mut self) -> Rxcapext0W<Dev0cfg1Spec> {
        Rxcapext0W::new(self, 4)
    }
    #[doc = "Bit 5 - Halt sclk based on xfer_count"]
    #[inline(always)]
    #[must_use]
    pub fn sclkrxhalt0(&mut self) -> Sclkrxhalt0W<Dev0cfg1Spec> {
        Sclkrxhalt0W::new(self, 5)
    }
    #[doc = "Bit 7 - Enable the support of WBX ( page boundary crossing on write )"]
    #[inline(always)]
    #[must_use]
    pub fn wbx0(&mut self) -> Wbx0W<Dev0cfg1Spec> {
        Wbx0W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable the support of RBX ( page boundary crossing on read )"]
    #[inline(always)]
    #[must_use]
    pub fn rbx0(&mut self) -> Rbx0W<Dev0cfg1Spec> {
        Rbx0W::new(self, 8)
    }
    #[doc = "Bits 9:10 - Sampling edge based on sclk edge. No effect when div1"]
    #[inline(always)]
    #[must_use]
    pub fn rxsmp0(&mut self) -> Rxsmp0W<Dev0cfg1Spec> {
        Rxsmp0W::new(self, 9)
    }
    #[doc = "Bit 11 - When using Windbond, set this bit to 1 to generate CA\\[47:0\\]
in hardware."]
    #[inline(always)]
    #[must_use]
    pub fn hyperio0(&mut self) -> Hyperio0W<Dev0cfg1Spec> {
        Hyperio0W::new(self, 11)
    }
    #[doc = "Bit 12 - Add 1/4th mspi_clk in DDR to turnaround. Recommended set this to 1 when EMULATEDDR is set in non-DQS mode (ENABLEDQS = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn tafourth0(&mut self) -> Tafourth0W<Dev0cfg1Spec> {
        Tafourth0W::new(self, 12)
    }
    #[doc = "Bit 13 - Force st_rx to start at clock high of mspi_clk"]
    #[inline(always)]
    #[must_use]
    pub fn rxhi0(&mut self) -> Rxhi0W<Dev0cfg1Spec> {
        Rxhi0W::new(self, 13)
    }
    #[doc = "Bits 14:16 - In DQS mode, the internal cycle count to enable DQS path."]
    #[inline(always)]
    #[must_use]
    pub fn dqsturn0(&mut self) -> Dqsturn0W<Dev0cfg1Spec> {
        Dqsturn0W::new(self, 14)
    }
}
#[doc = "Timing and mode configuration bits for the MSPI module.\n\nYou can [`read`](crate::Reg::read) this register and get [`dev0cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev0cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dev0cfg1Spec;
impl crate::RegisterSpec for Dev0cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev0cfg1::R`](R) reader structure"]
impl crate::Readable for Dev0cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`dev0cfg1::W`](W) writer structure"]
impl crate::Writable for Dev0cfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEV0CFG1 to value 0x8200"]
impl crate::Resettable for Dev0cfg1Spec {
    const RESET_VALUE: u32 = 0x8200;
}
