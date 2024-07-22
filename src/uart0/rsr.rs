#[doc = "Register `RSR` reader"]
pub type R = crate::R<RsrSpec>;
#[doc = "Register `RSR` writer"]
pub type W = crate::W<RsrSpec>;
#[doc = "Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Festat {
    #[doc = "0: No error on UART FESTAT, framing error indicator."]
    Noerr = 0,
    #[doc = "1: Error on UART FESTAT, framing error indicator."]
    Err = 1,
}
impl From<Festat> for bool {
    #[inline(always)]
    fn from(variant: Festat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FESTAT` reader - Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO."]
pub type FestatR = crate::BitReader<Festat>;
impl FestatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Festat {
        match self.bits {
            false => Festat::Noerr,
            true => Festat::Err,
        }
    }
    #[doc = "No error on UART FESTAT, framing error indicator."]
    #[inline(always)]
    pub fn is_noerr(&self) -> bool {
        *self == Festat::Noerr
    }
    #[doc = "Error on UART FESTAT, framing error indicator."]
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        *self == Festat::Err
    }
}
#[doc = "Field `FESTAT` writer - Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO."]
pub type FestatW<'a, REG> = crate::BitWriter<'a, REG, Festat>;
impl<'a, REG> FestatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error on UART FESTAT, framing error indicator."]
    #[inline(always)]
    pub fn noerr(self) -> &'a mut crate::W<REG> {
        self.variant(Festat::Noerr)
    }
    #[doc = "Error on UART FESTAT, framing error indicator."]
    #[inline(always)]
    pub fn err(self) -> &'a mut crate::W<REG> {
        self.variant(Festat::Err)
    }
}
#[doc = "Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCRH select. This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pestat {
    #[doc = "0: No error on UART PESTAT, parity error indicator."]
    Noerr = 0,
    #[doc = "1: Error on UART PESTAT, parity error indicator."]
    Err = 1,
}
impl From<Pestat> for bool {
    #[inline(always)]
    fn from(variant: Pestat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PESTAT` reader - Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCRH select. This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO."]
pub type PestatR = crate::BitReader<Pestat>;
impl PestatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pestat {
        match self.bits {
            false => Pestat::Noerr,
            true => Pestat::Err,
        }
    }
    #[doc = "No error on UART PESTAT, parity error indicator."]
    #[inline(always)]
    pub fn is_noerr(&self) -> bool {
        *self == Pestat::Noerr
    }
    #[doc = "Error on UART PESTAT, parity error indicator."]
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        *self == Pestat::Err
    }
}
#[doc = "Field `PESTAT` writer - Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCRH select. This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO."]
pub type PestatW<'a, REG> = crate::BitWriter<'a, REG, Pestat>;
impl<'a, REG> PestatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error on UART PESTAT, parity error indicator."]
    #[inline(always)]
    pub fn noerr(self) -> &'a mut crate::W<REG> {
        self.variant(Pestat::Noerr)
    }
    #[doc = "Error on UART PESTAT, parity error indicator."]
    #[inline(always)]
    pub fn err(self) -> &'a mut crate::W<REG> {
        self.variant(Pestat::Err)
    }
}
#[doc = "Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity, and stop bits). This bit is cleared to 0 after a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bestat {
    #[doc = "0: No error on UART BESTAT, break error indicator."]
    Noerr = 0,
    #[doc = "1: Error on UART BESTAT, break error indicator."]
    Err = 1,
}
impl From<Bestat> for bool {
    #[inline(always)]
    fn from(variant: Bestat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BESTAT` reader - Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity, and stop bits). This bit is cleared to 0 after a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received."]
pub type BestatR = crate::BitReader<Bestat>;
impl BestatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bestat {
        match self.bits {
            false => Bestat::Noerr,
            true => Bestat::Err,
        }
    }
    #[doc = "No error on UART BESTAT, break error indicator."]
    #[inline(always)]
    pub fn is_noerr(&self) -> bool {
        *self == Bestat::Noerr
    }
    #[doc = "Error on UART BESTAT, break error indicator."]
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        *self == Bestat::Err
    }
}
#[doc = "Field `BESTAT` writer - Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity, and stop bits). This bit is cleared to 0 after a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received."]
pub type BestatW<'a, REG> = crate::BitWriter<'a, REG, Bestat>;
impl<'a, REG> BestatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error on UART BESTAT, break error indicator."]
    #[inline(always)]
    pub fn noerr(self) -> &'a mut crate::W<REG> {
        self.variant(Bestat::Noerr)
    }
    #[doc = "Error on UART BESTAT, break error indicator."]
    #[inline(always)]
    pub fn err(self) -> &'a mut crate::W<REG> {
        self.variant(Bestat::Err)
    }
}
#[doc = "Overrun error. This bit is set to 1 if data is received and the FIFO is already full. This bit is cleared to 0 by a write to UARTECR. The FIFO contents remain valid because no more data is written when the FIFO is full, only the contents of the shift register are overwritten. The CPU must now read the data, to empty the FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oestat {
    #[doc = "0: No error on UART OESTAT, overrun error indicator."]
    Noerr = 0,
    #[doc = "1: Error on UART OESTAT, overrun error indicator."]
    Err = 1,
}
impl From<Oestat> for bool {
    #[inline(always)]
    fn from(variant: Oestat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OESTAT` reader - Overrun error. This bit is set to 1 if data is received and the FIFO is already full. This bit is cleared to 0 by a write to UARTECR. The FIFO contents remain valid because no more data is written when the FIFO is full, only the contents of the shift register are overwritten. The CPU must now read the data, to empty the FIFO."]
pub type OestatR = crate::BitReader<Oestat>;
impl OestatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oestat {
        match self.bits {
            false => Oestat::Noerr,
            true => Oestat::Err,
        }
    }
    #[doc = "No error on UART OESTAT, overrun error indicator."]
    #[inline(always)]
    pub fn is_noerr(&self) -> bool {
        *self == Oestat::Noerr
    }
    #[doc = "Error on UART OESTAT, overrun error indicator."]
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        *self == Oestat::Err
    }
}
#[doc = "Field `OESTAT` writer - Overrun error. This bit is set to 1 if data is received and the FIFO is already full. This bit is cleared to 0 by a write to UARTECR. The FIFO contents remain valid because no more data is written when the FIFO is full, only the contents of the shift register are overwritten. The CPU must now read the data, to empty the FIFO."]
pub type OestatW<'a, REG> = crate::BitWriter<'a, REG, Oestat>;
impl<'a, REG> OestatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error on UART OESTAT, overrun error indicator."]
    #[inline(always)]
    pub fn noerr(self) -> &'a mut crate::W<REG> {
        self.variant(Oestat::Noerr)
    }
    #[doc = "Error on UART OESTAT, overrun error indicator."]
    #[inline(always)]
    pub fn err(self) -> &'a mut crate::W<REG> {
        self.variant(Oestat::Err)
    }
}
impl R {
    #[doc = "Bit 0 - Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    pub fn festat(&self) -> FestatR {
        FestatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCRH select. This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    pub fn pestat(&self) -> PestatR {
        PestatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity, and stop bits). This bit is cleared to 0 after a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received."]
    #[inline(always)]
    pub fn bestat(&self) -> BestatR {
        BestatR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error. This bit is set to 1 if data is received and the FIFO is already full. This bit is cleared to 0 by a write to UARTECR. The FIFO contents remain valid because no more data is written when the FIFO is full, only the contents of the shift register are overwritten. The CPU must now read the data, to empty the FIFO."]
    #[inline(always)]
    pub fn oestat(&self) -> OestatR {
        OestatR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn festat(&mut self) -> FestatW<RsrSpec> {
        FestatW::new(self, 0)
    }
    #[doc = "Bit 1 - Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCRH select. This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn pestat(&mut self) -> PestatW<RsrSpec> {
        PestatW::new(self, 1)
    }
    #[doc = "Bit 2 - Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity, and stop bits). This bit is cleared to 0 after a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received."]
    #[inline(always)]
    #[must_use]
    pub fn bestat(&mut self) -> BestatW<RsrSpec> {
        BestatW::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun error. This bit is set to 1 if data is received and the FIFO is already full. This bit is cleared to 0 by a write to UARTECR. The FIFO contents remain valid because no more data is written when the FIFO is full, only the contents of the shift register are overwritten. The CPU must now read the data, to empty the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn oestat(&mut self) -> OestatW<RsrSpec> {
        OestatW::new(self, 3)
    }
}
#[doc = "UART Status\n\nYou can [`read`](crate::Reg::read) this register and get [`rsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RsrSpec;
impl crate::RegisterSpec for RsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsr::R`](R) reader structure"]
impl crate::Readable for RsrSpec {}
#[doc = "`write(|w| ..)` method takes [`rsr::W`](W) writer structure"]
impl crate::Writable for RsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSR to value 0"]
impl crate::Resettable for RsrSpec {
    const RESET_VALUE: u32 = 0;
}
