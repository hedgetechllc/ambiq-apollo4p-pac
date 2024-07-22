#[doc = "Register `LCRH` reader"]
pub type R = crate::R<LcrhSpec>;
#[doc = "Register `LCRH` writer"]
pub type W = crate::W<LcrhSpec>;
#[doc = "Field `BRK` reader - This bit holds the break set. If this bit is set to 1, a low-level is continually output on the UARTTXD output, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
pub type BrkR = crate::BitReader;
#[doc = "Field `BRK` writer - This bit holds the break set. If this bit is set to 1, a low-level is continually output on the UARTTXD output, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
pub type BrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN` reader - This bit holds the parity enable. 0 = parity is disabled and no parity bit added to the data frame. 1 = parity checking and generation is enabled."]
pub type PenR = crate::BitReader;
#[doc = "Field `PEN` writer - This bit holds the parity enable. 0 = parity is disabled and no parity bit added to the data frame. 1 = parity checking and generation is enabled."]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPS` reader - This bit holds the even parity select. Controls the type of parity the UART uses during transmission and reception: 0 = odd parity. The UART generates or checks for an odd number of 1s in the data and parity bits. 1 = even parity. The UART generates or checks for an even number of 1s in the data and parity bits. This bit has no effect when the PEN bit disables parity checking and generation."]
pub type EpsR = crate::BitReader;
#[doc = "Field `EPS` writer - This bit holds the even parity select. Controls the type of parity the UART uses during transmission and reception: 0 = odd parity. The UART generates or checks for an odd number of 1s in the data and parity bits. 1 = even parity. The UART generates or checks for an even number of 1s in the data and parity bits. This bit has no effect when the PEN bit disables parity checking and generation."]
pub type EpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STP2` reader - This bit holds the two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
pub type Stp2R = crate::BitReader;
#[doc = "Field `STP2` writer - This bit holds the two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
pub type Stp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN` reader - This bit holds the FIFO enable. 0 = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers. 1 = transmit and receive FIFO buffers are enabled (FIFO mode)."]
pub type FenR = crate::BitReader;
#[doc = "Field `FEN` writer - This bit holds the FIFO enable. 0 = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers. 1 = transmit and receive FIFO buffers are enabled (FIFO mode)."]
pub type FenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WLEN` reader - These bits hold the write length. These bits indicate the number of data bits transmitted or received in a frame as follows: b11 = 8 bits, b10 = 7 bits, b01 = 6 bits, b00 = 5 bits."]
pub type WlenR = crate::FieldReader;
#[doc = "Field `WLEN` writer - These bits hold the write length. These bits indicate the number of data bits transmitted or received in a frame as follows: b11 = 8 bits, b10 = 7 bits, b01 = 6 bits, b00 = 5 bits."]
pub type WlenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPS` reader - This bit holds the stick parity select. If the EPS bit is 0 then the parity bit is transmitted and checked as a 1. If the EPS bit is 1 then the parity bit is transmitted and checked as a 0. This bit has no effect when the PEN bit disables parity checking and generation."]
pub type SpsR = crate::BitReader;
#[doc = "Field `SPS` writer - This bit holds the stick parity select. If the EPS bit is 0 then the parity bit is transmitted and checked as a 1. If the EPS bit is 1 then the parity bit is transmitted and checked as a 0. This bit has no effect when the PEN bit disables parity checking and generation."]
pub type SpsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit holds the break set. If this bit is set to 1, a low-level is continually output on the UARTTXD output, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
    #[inline(always)]
    pub fn brk(&self) -> BrkR {
        BrkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit holds the parity enable. 0 = parity is disabled and no parity bit added to the data frame. 1 = parity checking and generation is enabled."]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit holds the even parity select. Controls the type of parity the UART uses during transmission and reception: 0 = odd parity. The UART generates or checks for an odd number of 1s in the data and parity bits. 1 = even parity. The UART generates or checks for an even number of 1s in the data and parity bits. This bit has no effect when the PEN bit disables parity checking and generation."]
    #[inline(always)]
    pub fn eps(&self) -> EpsR {
        EpsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit holds the two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
    #[inline(always)]
    pub fn stp2(&self) -> Stp2R {
        Stp2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit holds the FIFO enable. 0 = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers. 1 = transmit and receive FIFO buffers are enabled (FIFO mode)."]
    #[inline(always)]
    pub fn fen(&self) -> FenR {
        FenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - These bits hold the write length. These bits indicate the number of data bits transmitted or received in a frame as follows: b11 = 8 bits, b10 = 7 bits, b01 = 6 bits, b00 = 5 bits."]
    #[inline(always)]
    pub fn wlen(&self) -> WlenR {
        WlenR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - This bit holds the stick parity select. If the EPS bit is 0 then the parity bit is transmitted and checked as a 1. If the EPS bit is 1 then the parity bit is transmitted and checked as a 0. This bit has no effect when the PEN bit disables parity checking and generation."]
    #[inline(always)]
    pub fn sps(&self) -> SpsR {
        SpsR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit holds the break set. If this bit is set to 1, a low-level is continually output on the UARTTXD output, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn brk(&mut self) -> BrkW<LcrhSpec> {
        BrkW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit holds the parity enable. 0 = parity is disabled and no parity bit added to the data frame. 1 = parity checking and generation is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PenW<LcrhSpec> {
        PenW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit holds the even parity select. Controls the type of parity the UART uses during transmission and reception: 0 = odd parity. The UART generates or checks for an odd number of 1s in the data and parity bits. 1 = even parity. The UART generates or checks for an even number of 1s in the data and parity bits. This bit has no effect when the PEN bit disables parity checking and generation."]
    #[inline(always)]
    #[must_use]
    pub fn eps(&mut self) -> EpsW<LcrhSpec> {
        EpsW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit holds the two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
    #[inline(always)]
    #[must_use]
    pub fn stp2(&mut self) -> Stp2W<LcrhSpec> {
        Stp2W::new(self, 3)
    }
    #[doc = "Bit 4 - This bit holds the FIFO enable. 0 = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers. 1 = transmit and receive FIFO buffers are enabled (FIFO mode)."]
    #[inline(always)]
    #[must_use]
    pub fn fen(&mut self) -> FenW<LcrhSpec> {
        FenW::new(self, 4)
    }
    #[doc = "Bits 5:6 - These bits hold the write length. These bits indicate the number of data bits transmitted or received in a frame as follows: b11 = 8 bits, b10 = 7 bits, b01 = 6 bits, b00 = 5 bits."]
    #[inline(always)]
    #[must_use]
    pub fn wlen(&mut self) -> WlenW<LcrhSpec> {
        WlenW::new(self, 5)
    }
    #[doc = "Bit 7 - This bit holds the stick parity select. If the EPS bit is 0 then the parity bit is transmitted and checked as a 1. If the EPS bit is 1 then the parity bit is transmitted and checked as a 0. This bit has no effect when the PEN bit disables parity checking and generation."]
    #[inline(always)]
    #[must_use]
    pub fn sps(&mut self) -> SpsW<LcrhSpec> {
        SpsW::new(self, 7)
    }
}
#[doc = "Line Control High\n\nYou can [`read`](crate::Reg::read) this register and get [`lcrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcrhSpec;
impl crate::RegisterSpec for LcrhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcrh::R`](R) reader structure"]
impl crate::Readable for LcrhSpec {}
#[doc = "`write(|w| ..)` method takes [`lcrh::W`](W) writer structure"]
impl crate::Writable for LcrhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCRH to value 0"]
impl crate::Resettable for LcrhSpec {
    const RESET_VALUE: u32 = 0;
}
