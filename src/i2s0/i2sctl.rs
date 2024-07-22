#[doc = "Register `I2SCTL` reader"]
pub type R = crate::R<I2sctlSpec>;
#[doc = "Register `I2SCTL` writer"]
pub type W = crate::W<I2sctlSpec>;
#[doc = "Field `TXEN` reader - Transmit enable signal. 1 will enable the transmission of serial audio. For Full duplex operation, RXEN and TXEN MUST be set in a single register write access, or the Slave FSM may ignore one of the bit-field read-modify-write accesses. TXRST and RXRST must be cleared in advance."]
pub type TxenR = crate::BitReader;
#[doc = "Field `TXEN` writer - Transmit enable signal. 1 will enable the transmission of serial audio. For Full duplex operation, RXEN and TXEN MUST be set in a single register write access, or the Slave FSM may ignore one of the bit-field read-modify-write accesses. TXRST and RXRST must be cleared in advance."]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRST` reader - Transmit reset signal. 1 will reset the TX side registers and flush the TX FIFO."]
pub type TxrstR = crate::BitReader;
#[doc = "Field `TXRST` writer - Transmit reset signal. 1 will reset the TX side registers and flush the TX FIFO."]
pub type TxrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN` reader - Receive enable control. 1: Enables capture of serial audio, starting with first channel. 0: No receive data captured. For Full duplex operation, RXEN and TXEN MUST be set in a single register write access, or the Slave FSM may ignore one of the bit-field read-modify-write accesses. TXRST and RXRST must be cleared in advance."]
pub type RxenR = crate::BitReader;
#[doc = "Field `RXEN` writer - Receive enable control. 1: Enables capture of serial audio, starting with first channel. 0: No receive data captured. For Full duplex operation, RXEN and TXEN MUST be set in a single register write access, or the Slave FSM may ignore one of the bit-field read-modify-write accesses. TXRST and RXRST must be cleared in advance."]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRST` reader - Active high receiver reset signal. 1: Flush the RX FIFO"]
pub type RxrstR = crate::BitReader;
#[doc = "Field `RXRST` writer - Active high receiver reset signal. 1: Flush the RX FIFO"]
pub type RxrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SVAL` reader - I2S validity bit mode. 1: RX data stored only when validity mask condition is asserted. 0: No validity mask conditions checking is done."]
pub type I2svalR = crate::BitReader;
#[doc = "Field `I2SVAL` writer - I2S validity bit mode. 1: RX data stored only when validity mask condition is asserted. 0: No validity mask conditions checking is done."]
pub type I2svalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit enable signal. 1 will enable the transmission of serial audio. For Full duplex operation, RXEN and TXEN MUST be set in a single register write access, or the Slave FSM may ignore one of the bit-field read-modify-write accesses. TXRST and RXRST must be cleared in advance."]
    #[inline(always)]
    pub fn txen(&self) -> TxenR {
        TxenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit reset signal. 1 will reset the TX side registers and flush the TX FIFO."]
    #[inline(always)]
    pub fn txrst(&self) -> TxrstR {
        TxrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive enable control. 1: Enables capture of serial audio, starting with first channel. 0: No receive data captured. For Full duplex operation, RXEN and TXEN MUST be set in a single register write access, or the Slave FSM may ignore one of the bit-field read-modify-write accesses. TXRST and RXRST must be cleared in advance."]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Active high receiver reset signal. 1: Flush the RX FIFO"]
    #[inline(always)]
    pub fn rxrst(&self) -> RxrstR {
        RxrstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 31 - I2S validity bit mode. 1: RX data stored only when validity mask condition is asserted. 0: No validity mask conditions checking is done."]
    #[inline(always)]
    pub fn i2sval(&self) -> I2svalR {
        I2svalR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit enable signal. 1 will enable the transmission of serial audio. For Full duplex operation, RXEN and TXEN MUST be set in a single register write access, or the Slave FSM may ignore one of the bit-field read-modify-write accesses. TXRST and RXRST must be cleared in advance."]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TxenW<I2sctlSpec> {
        TxenW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit reset signal. 1 will reset the TX side registers and flush the TX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn txrst(&mut self) -> TxrstW<I2sctlSpec> {
        TxrstW::new(self, 1)
    }
    #[doc = "Bit 4 - Receive enable control. 1: Enables capture of serial audio, starting with first channel. 0: No receive data captured. For Full duplex operation, RXEN and TXEN MUST be set in a single register write access, or the Slave FSM may ignore one of the bit-field read-modify-write accesses. TXRST and RXRST must be cleared in advance."]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RxenW<I2sctlSpec> {
        RxenW::new(self, 4)
    }
    #[doc = "Bit 5 - Active high receiver reset signal. 1: Flush the RX FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn rxrst(&mut self) -> RxrstW<I2sctlSpec> {
        RxrstW::new(self, 5)
    }
    #[doc = "Bit 31 - I2S validity bit mode. 1: RX data stored only when validity mask condition is asserted. 0: No validity mask conditions checking is done."]
    #[inline(always)]
    #[must_use]
    pub fn i2sval(&mut self) -> I2svalW<I2sctlSpec> {
        I2svalW::new(self, 31)
    }
}
#[doc = "Specified polarity and clock configuration of the I2S IPB clocks and IO signals\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sctlSpec;
impl crate::RegisterSpec for I2sctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2sctl::R`](R) reader structure"]
impl crate::Readable for I2sctlSpec {}
#[doc = "`write(|w| ..)` method takes [`i2sctl::W`](W) writer structure"]
impl crate::Writable for I2sctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2SCTL to value 0"]
impl crate::Resettable for I2sctlSpec {
    const RESET_VALUE: u32 = 0;
}
