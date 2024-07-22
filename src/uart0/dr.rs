#[doc = "Register `DR` reader"]
pub type R = crate::R<DrSpec>;
#[doc = "Register `DR` writer"]
pub type W = crate::W<DrSpec>;
#[doc = "Field `DATA` reader - Receive (read) data character. Transmit (write) data character."]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - Receive (read) data character. Transmit (write) data character."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). In FIFO mode, this error is associated with the character at the top of the FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fedata {
    #[doc = "0: No error on UART FEDATA, framing error indicator."]
    Noerr = 0,
    #[doc = "1: Error on UART FEDATA, framing error indicator."]
    Err = 1,
}
impl From<Fedata> for bool {
    #[inline(always)]
    fn from(variant: Fedata) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEDATA` reader - Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). In FIFO mode, this error is associated with the character at the top of the FIFO."]
pub type FedataR = crate::BitReader<Fedata>;
impl FedataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fedata {
        match self.bits {
            false => Fedata::Noerr,
            true => Fedata::Err,
        }
    }
    #[doc = "No error on UART FEDATA, framing error indicator."]
    #[inline(always)]
    pub fn is_noerr(&self) -> bool {
        *self == Fedata::Noerr
    }
    #[doc = "Error on UART FEDATA, framing error indicator."]
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        *self == Fedata::Err
    }
}
#[doc = "Field `FEDATA` writer - Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). In FIFO mode, this error is associated with the character at the top of the FIFO."]
pub type FedataW<'a, REG> = crate::BitWriter<'a, REG, Fedata>;
impl<'a, REG> FedataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error on UART FEDATA, framing error indicator."]
    #[inline(always)]
    pub fn noerr(self) -> &'a mut crate::W<REG> {
        self.variant(Fedata::Noerr)
    }
    #[doc = "Error on UART FEDATA, framing error indicator."]
    #[inline(always)]
    pub fn err(self) -> &'a mut crate::W<REG> {
        self.variant(Fedata::Err)
    }
}
#[doc = "Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCRH select. In FIFO mode, this error is associated with the character at the top of the FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pedata {
    #[doc = "0: No error on UART PEDATA, parity error indicator."]
    Noerr = 0,
    #[doc = "1: Error on UART PEDATA, parity error indicator."]
    Err = 1,
}
impl From<Pedata> for bool {
    #[inline(always)]
    fn from(variant: Pedata) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEDATA` reader - Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCRH select. In FIFO mode, this error is associated with the character at the top of the FIFO."]
pub type PedataR = crate::BitReader<Pedata>;
impl PedataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pedata {
        match self.bits {
            false => Pedata::Noerr,
            true => Pedata::Err,
        }
    }
    #[doc = "No error on UART PEDATA, parity error indicator."]
    #[inline(always)]
    pub fn is_noerr(&self) -> bool {
        *self == Pedata::Noerr
    }
    #[doc = "Error on UART PEDATA, parity error indicator."]
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        *self == Pedata::Err
    }
}
#[doc = "Field `PEDATA` writer - Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCRH select. In FIFO mode, this error is associated with the character at the top of the FIFO."]
pub type PedataW<'a, REG> = crate::BitWriter<'a, REG, Pedata>;
impl<'a, REG> PedataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error on UART PEDATA, parity error indicator."]
    #[inline(always)]
    pub fn noerr(self) -> &'a mut crate::W<REG> {
        self.variant(Pedata::Noerr)
    }
    #[doc = "Error on UART PEDATA, parity error indicator."]
    #[inline(always)]
    pub fn err(self) -> &'a mut crate::W<REG> {
        self.variant(Pedata::Err)
    }
}
#[doc = "Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits). In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state), and the next valid start bit is received.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bedata {
    #[doc = "0: No error on UART BEDATA, break error indicator."]
    Noerr = 0,
    #[doc = "1: Error on UART BEDATA, break error indicator."]
    Err = 1,
}
impl From<Bedata> for bool {
    #[inline(always)]
    fn from(variant: Bedata) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BEDATA` reader - Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits). In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state), and the next valid start bit is received."]
pub type BedataR = crate::BitReader<Bedata>;
impl BedataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bedata {
        match self.bits {
            false => Bedata::Noerr,
            true => Bedata::Err,
        }
    }
    #[doc = "No error on UART BEDATA, break error indicator."]
    #[inline(always)]
    pub fn is_noerr(&self) -> bool {
        *self == Bedata::Noerr
    }
    #[doc = "Error on UART BEDATA, break error indicator."]
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        *self == Bedata::Err
    }
}
#[doc = "Field `BEDATA` writer - Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits). In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state), and the next valid start bit is received."]
pub type BedataW<'a, REG> = crate::BitWriter<'a, REG, Bedata>;
impl<'a, REG> BedataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error on UART BEDATA, break error indicator."]
    #[inline(always)]
    pub fn noerr(self) -> &'a mut crate::W<REG> {
        self.variant(Bedata::Noerr)
    }
    #[doc = "Error on UART BEDATA, break error indicator."]
    #[inline(always)]
    pub fn err(self) -> &'a mut crate::W<REG> {
        self.variant(Bedata::Err)
    }
}
#[doc = "Overrun error. This bit is set to 1 if data is received and the receive FIFO is already full. This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oedata {
    #[doc = "0: No error on UART OEDATA, overrun error indicator."]
    Noerr = 0,
    #[doc = "1: Error on UART OEDATA, overrun error indicator."]
    Err = 1,
}
impl From<Oedata> for bool {
    #[inline(always)]
    fn from(variant: Oedata) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OEDATA` reader - Overrun error. This bit is set to 1 if data is received and the receive FIFO is already full. This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it."]
pub type OedataR = crate::BitReader<Oedata>;
impl OedataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oedata {
        match self.bits {
            false => Oedata::Noerr,
            true => Oedata::Err,
        }
    }
    #[doc = "No error on UART OEDATA, overrun error indicator."]
    #[inline(always)]
    pub fn is_noerr(&self) -> bool {
        *self == Oedata::Noerr
    }
    #[doc = "Error on UART OEDATA, overrun error indicator."]
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        *self == Oedata::Err
    }
}
#[doc = "Field `OEDATA` writer - Overrun error. This bit is set to 1 if data is received and the receive FIFO is already full. This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it."]
pub type OedataW<'a, REG> = crate::BitWriter<'a, REG, Oedata>;
impl<'a, REG> OedataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error on UART OEDATA, overrun error indicator."]
    #[inline(always)]
    pub fn noerr(self) -> &'a mut crate::W<REG> {
        self.variant(Oedata::Noerr)
    }
    #[doc = "Error on UART OEDATA, overrun error indicator."]
    #[inline(always)]
    pub fn err(self) -> &'a mut crate::W<REG> {
        self.variant(Oedata::Err)
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive (read) data character. Transmit (write) data character."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    pub fn fedata(&self) -> FedataR {
        FedataR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCRH select. In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    pub fn pedata(&self) -> PedataR {
        PedataR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits). In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state), and the next valid start bit is received."]
    #[inline(always)]
    pub fn bedata(&self) -> BedataR {
        BedataR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Overrun error. This bit is set to 1 if data is received and the receive FIFO is already full. This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it."]
    #[inline(always)]
    pub fn oedata(&self) -> OedataR {
        OedataR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive (read) data character. Transmit (write) data character."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<DrSpec> {
        DataW::new(self, 0)
    }
    #[doc = "Bit 8 - Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn fedata(&mut self) -> FedataW<DrSpec> {
        FedataW::new(self, 8)
    }
    #[doc = "Bit 9 - Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCRH select. In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn pedata(&mut self) -> PedataW<DrSpec> {
        PedataW::new(self, 9)
    }
    #[doc = "Bit 10 - Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits). In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state), and the next valid start bit is received."]
    #[inline(always)]
    #[must_use]
    pub fn bedata(&mut self) -> BedataW<DrSpec> {
        BedataW::new(self, 10)
    }
    #[doc = "Bit 11 - Overrun error. This bit is set to 1 if data is received and the receive FIFO is already full. This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it."]
    #[inline(always)]
    #[must_use]
    pub fn oedata(&mut self) -> OedataW<DrSpec> {
        OedataW::new(self, 11)
    }
}
#[doc = "UART Data\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrSpec;
impl crate::RegisterSpec for DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DrSpec {}
#[doc = "`write(|w| ..)` method takes [`dr::W`](W) writer structure"]
impl crate::Writable for DrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DrSpec {
    const RESET_VALUE: u32 = 0;
}
