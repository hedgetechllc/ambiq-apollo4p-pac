#[doc = "Register `DEV0CFG` reader"]
pub type R = crate::R<Dev0cfgSpec>;
#[doc = "Register `DEV0CFG` writer"]
pub type W = crate::W<Dev0cfgSpec>;
#[doc = "Flash configuration for XIP and AUTO DMA operations. Controls value for SER (Slave Enable) for XIP operations and address generation for DMA/XIP modes. Also used to configure SPIFRF (frame format).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Devcfg0 {
    #[doc = "1: Single bit SPI flash on chip select 0"]
    Serial0 = 1,
    #[doc = "2: Single bit SPI flash on chip select 1"]
    Serial1 = 2,
    #[doc = "5: Dual SPI flash on chip select 0"]
    Dual0 = 5,
    #[doc = "6: Dual bit SPI flash on chip select 1"]
    Dual1 = 6,
    #[doc = "9: Quad SPI flash on chip select 0"]
    Quad0 = 9,
    #[doc = "10: Quad SPI flash on chip select 1"]
    Quad1 = 10,
    #[doc = "13: Octal SPI flash on chip select 0"]
    Octal0 = 13,
    #[doc = "14: Octal SPI flash on chip select 1"]
    Octal1 = 14,
    #[doc = "15: Dual Quad SPI flash on chip selects 0/1."]
    Quadpaired = 15,
    #[doc = "3: Dual Quad SPI flash on chip selects 0/1, but transmit in serial mode for initialization operations"]
    QuadpairedSerial = 3,
    #[doc = "17: Hex SPI flash on chip selects 0."]
    Hex0 = 17,
    #[doc = "18: Hex SPI flash on chip selects 1."]
    Hex1 = 18,
}
impl From<Devcfg0> for u8 {
    #[inline(always)]
    fn from(variant: Devcfg0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Devcfg0 {
    type Ux = u8;
}
impl crate::IsEnum for Devcfg0 {}
#[doc = "Field `DEVCFG0` reader - Flash configuration for XIP and AUTO DMA operations. Controls value for SER (Slave Enable) for XIP operations and address generation for DMA/XIP modes. Also used to configure SPIFRF (frame format)."]
pub type Devcfg0R = crate::FieldReader<Devcfg0>;
impl Devcfg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Devcfg0> {
        match self.bits {
            1 => Some(Devcfg0::Serial0),
            2 => Some(Devcfg0::Serial1),
            5 => Some(Devcfg0::Dual0),
            6 => Some(Devcfg0::Dual1),
            9 => Some(Devcfg0::Quad0),
            10 => Some(Devcfg0::Quad1),
            13 => Some(Devcfg0::Octal0),
            14 => Some(Devcfg0::Octal1),
            15 => Some(Devcfg0::Quadpaired),
            3 => Some(Devcfg0::QuadpairedSerial),
            17 => Some(Devcfg0::Hex0),
            18 => Some(Devcfg0::Hex1),
            _ => None,
        }
    }
    #[doc = "Single bit SPI flash on chip select 0"]
    #[inline(always)]
    pub fn is_serial0(&self) -> bool {
        *self == Devcfg0::Serial0
    }
    #[doc = "Single bit SPI flash on chip select 1"]
    #[inline(always)]
    pub fn is_serial1(&self) -> bool {
        *self == Devcfg0::Serial1
    }
    #[doc = "Dual SPI flash on chip select 0"]
    #[inline(always)]
    pub fn is_dual0(&self) -> bool {
        *self == Devcfg0::Dual0
    }
    #[doc = "Dual bit SPI flash on chip select 1"]
    #[inline(always)]
    pub fn is_dual1(&self) -> bool {
        *self == Devcfg0::Dual1
    }
    #[doc = "Quad SPI flash on chip select 0"]
    #[inline(always)]
    pub fn is_quad0(&self) -> bool {
        *self == Devcfg0::Quad0
    }
    #[doc = "Quad SPI flash on chip select 1"]
    #[inline(always)]
    pub fn is_quad1(&self) -> bool {
        *self == Devcfg0::Quad1
    }
    #[doc = "Octal SPI flash on chip select 0"]
    #[inline(always)]
    pub fn is_octal0(&self) -> bool {
        *self == Devcfg0::Octal0
    }
    #[doc = "Octal SPI flash on chip select 1"]
    #[inline(always)]
    pub fn is_octal1(&self) -> bool {
        *self == Devcfg0::Octal1
    }
    #[doc = "Dual Quad SPI flash on chip selects 0/1."]
    #[inline(always)]
    pub fn is_quadpaired(&self) -> bool {
        *self == Devcfg0::Quadpaired
    }
    #[doc = "Dual Quad SPI flash on chip selects 0/1, but transmit in serial mode for initialization operations"]
    #[inline(always)]
    pub fn is_quadpaired_serial(&self) -> bool {
        *self == Devcfg0::QuadpairedSerial
    }
    #[doc = "Hex SPI flash on chip selects 0."]
    #[inline(always)]
    pub fn is_hex0(&self) -> bool {
        *self == Devcfg0::Hex0
    }
    #[doc = "Hex SPI flash on chip selects 1."]
    #[inline(always)]
    pub fn is_hex1(&self) -> bool {
        *self == Devcfg0::Hex1
    }
}
#[doc = "Field `DEVCFG0` writer - Flash configuration for XIP and AUTO DMA operations. Controls value for SER (Slave Enable) for XIP operations and address generation for DMA/XIP modes. Also used to configure SPIFRF (frame format)."]
pub type Devcfg0W<'a, REG> = crate::FieldWriter<'a, REG, 5, Devcfg0>;
impl<'a, REG> Devcfg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single bit SPI flash on chip select 0"]
    #[inline(always)]
    pub fn serial0(self) -> &'a mut crate::W<REG> {
        self.variant(Devcfg0::Serial0)
    }
    #[doc = "Single bit SPI flash on chip select 1"]
    #[inline(always)]
    pub fn serial1(self) -> &'a mut crate::W<REG> {
        self.variant(Devcfg0::Serial1)
    }
    #[doc = "Dual SPI flash on chip select 0"]
    #[inline(always)]
    pub fn dual0(self) -> &'a mut crate::W<REG> {
        self.variant(Devcfg0::Dual0)
    }
    #[doc = "Dual bit SPI flash on chip select 1"]
    #[inline(always)]
    pub fn dual1(self) -> &'a mut crate::W<REG> {
        self.variant(Devcfg0::Dual1)
    }
    #[doc = "Quad SPI flash on chip select 0"]
    #[inline(always)]
    pub fn quad0(self) -> &'a mut crate::W<REG> {
        self.variant(Devcfg0::Quad0)
    }
    #[doc = "Quad SPI flash on chip select 1"]
    #[inline(always)]
    pub fn quad1(self) -> &'a mut crate::W<REG> {
        self.variant(Devcfg0::Quad1)
    }
    #[doc = "Octal SPI flash on chip select 0"]
    #[inline(always)]
    pub fn octal0(self) -> &'a mut crate::W<REG> {
        self.variant(Devcfg0::Octal0)
    }
    #[doc = "Octal SPI flash on chip select 1"]
    #[inline(always)]
    pub fn octal1(self) -> &'a mut crate::W<REG> {
        self.variant(Devcfg0::Octal1)
    }
    #[doc = "Dual Quad SPI flash on chip selects 0/1."]
    #[inline(always)]
    pub fn quadpaired(self) -> &'a mut crate::W<REG> {
        self.variant(Devcfg0::Quadpaired)
    }
    #[doc = "Dual Quad SPI flash on chip selects 0/1, but transmit in serial mode for initialization operations"]
    #[inline(always)]
    pub fn quadpaired_serial(self) -> &'a mut crate::W<REG> {
        self.variant(Devcfg0::QuadpairedSerial)
    }
    #[doc = "Hex SPI flash on chip selects 0."]
    #[inline(always)]
    pub fn hex0(self) -> &'a mut crate::W<REG> {
        self.variant(Devcfg0::Hex0)
    }
    #[doc = "Hex SPI flash on chip selects 1."]
    #[inline(always)]
    pub fn hex1(self) -> &'a mut crate::W<REG> {
        self.variant(Devcfg0::Hex1)
    }
}
#[doc = "Address Size. Address bytes to send from ADDR register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Asize0 {
    #[doc = "0: Send one address byte"]
    A1 = 0,
    #[doc = "1: Send two address bytes"]
    A2 = 1,
    #[doc = "2: Send three address bytes"]
    A3 = 2,
    #[doc = "3: Send four address bytes"]
    A4 = 3,
}
impl From<Asize0> for u8 {
    #[inline(always)]
    fn from(variant: Asize0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Asize0 {
    type Ux = u8;
}
impl crate::IsEnum for Asize0 {}
#[doc = "Field `ASIZE0` reader - Address Size. Address bytes to send from ADDR register"]
pub type Asize0R = crate::FieldReader<Asize0>;
impl Asize0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Asize0 {
        match self.bits {
            0 => Asize0::A1,
            1 => Asize0::A2,
            2 => Asize0::A3,
            3 => Asize0::A4,
            _ => unreachable!(),
        }
    }
    #[doc = "Send one address byte"]
    #[inline(always)]
    pub fn is_a1(&self) -> bool {
        *self == Asize0::A1
    }
    #[doc = "Send two address bytes"]
    #[inline(always)]
    pub fn is_a2(&self) -> bool {
        *self == Asize0::A2
    }
    #[doc = "Send three address bytes"]
    #[inline(always)]
    pub fn is_a3(&self) -> bool {
        *self == Asize0::A3
    }
    #[doc = "Send four address bytes"]
    #[inline(always)]
    pub fn is_a4(&self) -> bool {
        *self == Asize0::A4
    }
}
#[doc = "Field `ASIZE0` writer - Address Size. Address bytes to send from ADDR register"]
pub type Asize0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Asize0, crate::Safe>;
impl<'a, REG> Asize0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Send one address byte"]
    #[inline(always)]
    pub fn a1(self) -> &'a mut crate::W<REG> {
        self.variant(Asize0::A1)
    }
    #[doc = "Send two address bytes"]
    #[inline(always)]
    pub fn a2(self) -> &'a mut crate::W<REG> {
        self.variant(Asize0::A2)
    }
    #[doc = "Send three address bytes"]
    #[inline(always)]
    pub fn a3(self) -> &'a mut crate::W<REG> {
        self.variant(Asize0::A3)
    }
    #[doc = "Send four address bytes"]
    #[inline(always)]
    pub fn a4(self) -> &'a mut crate::W<REG> {
        self.variant(Asize0::A4)
    }
}
#[doc = "Instruction Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isize0 {
    #[doc = "0: Instruction is 1 byte"]
    I8 = 0,
    #[doc = "1: Instruction is 2 bytes"]
    I16 = 1,
}
impl From<Isize0> for bool {
    #[inline(always)]
    fn from(variant: Isize0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISIZE0` reader - Instruction Size"]
pub type Isize0R = crate::BitReader<Isize0>;
impl Isize0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isize0 {
        match self.bits {
            false => Isize0::I8,
            true => Isize0::I16,
        }
    }
    #[doc = "Instruction is 1 byte"]
    #[inline(always)]
    pub fn is_i8(&self) -> bool {
        *self == Isize0::I8
    }
    #[doc = "Instruction is 2 bytes"]
    #[inline(always)]
    pub fn is_i16(&self) -> bool {
        *self == Isize0::I16
    }
}
#[doc = "Field `ISIZE0` writer - Instruction Size"]
pub type Isize0W<'a, REG> = crate::BitWriter<'a, REG, Isize0>;
impl<'a, REG> Isize0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Instruction is 1 byte"]
    #[inline(always)]
    pub fn i8(self) -> &'a mut crate::W<REG> {
        self.variant(Isize0::I8)
    }
    #[doc = "Instruction is 2 bytes"]
    #[inline(always)]
    pub fn i16(self) -> &'a mut crate::W<REG> {
        self.variant(Isize0::I16)
    }
}
#[doc = "Field `TURNAROUND0` reader - Number of turnaound cycles (for TX->RX transitions). Qualified by ENTURN bit field."]
pub type Turnaround0R = crate::FieldReader;
#[doc = "Field `TURNAROUND0` writer - Number of turnaound cycles (for TX->RX transitions). Qualified by ENTURN bit field."]
pub type Turnaround0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Serial clock phase.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpha0 {
    #[doc = "0: Clock toggles in middle of data bit."]
    Middle = 0,
    #[doc = "1: Clock toggles at start of data bit."]
    Start = 1,
}
impl From<Cpha0> for bool {
    #[inline(always)]
    fn from(variant: Cpha0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA0` reader - Serial clock phase."]
pub type Cpha0R = crate::BitReader<Cpha0>;
impl Cpha0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpha0 {
        match self.bits {
            false => Cpha0::Middle,
            true => Cpha0::Start,
        }
    }
    #[doc = "Clock toggles in middle of data bit."]
    #[inline(always)]
    pub fn is_middle(&self) -> bool {
        *self == Cpha0::Middle
    }
    #[doc = "Clock toggles at start of data bit."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Cpha0::Start
    }
}
#[doc = "Field `CPHA0` writer - Serial clock phase."]
pub type Cpha0W<'a, REG> = crate::BitWriter<'a, REG, Cpha0>;
impl<'a, REG> Cpha0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock toggles in middle of data bit."]
    #[inline(always)]
    pub fn middle(self) -> &'a mut crate::W<REG> {
        self.variant(Cpha0::Middle)
    }
    #[doc = "Clock toggles at start of data bit."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Cpha0::Start)
    }
}
#[doc = "Serial clock polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpol0 {
    #[doc = "0: Clock inactive state is low."]
    Low = 0,
    #[doc = "1: Clock inactive state is high."]
    High = 1,
}
impl From<Cpol0> for bool {
    #[inline(always)]
    fn from(variant: Cpol0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL0` reader - Serial clock polarity."]
pub type Cpol0R = crate::BitReader<Cpol0>;
impl Cpol0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpol0 {
        match self.bits {
            false => Cpol0::Low,
            true => Cpol0::High,
        }
    }
    #[doc = "Clock inactive state is low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Cpol0::Low
    }
    #[doc = "Clock inactive state is high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Cpol0::High
    }
}
#[doc = "Field `CPOL0` writer - Serial clock polarity."]
pub type Cpol0W<'a, REG> = crate::BitWriter<'a, REG, Cpol0>;
impl<'a, REG> Cpol0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock inactive state is low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Cpol0::Low)
    }
    #[doc = "Clock inactive state is high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Cpol0::High)
    }
}
#[doc = "Clock Divider. Allows dividing 96 MHz base clock by integer multiples. Enumerations are provided for common frequency, but any integer divide from 96 MHz is allowed. Odd divide ratios will result in a 33/66 percent duty cycle with a long low clock pulse (to allow longer round-trip for read data).\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkdiv0 {
    #[doc = "1: 96 MHz MSPI clock"]
    Clk96 = 1,
    #[doc = "2: 48 MHz MSPI clock"]
    Clk48 = 2,
    #[doc = "3: 32 MHz MSPI clock"]
    Clk32 = 3,
    #[doc = "4: 24 MHz MSPI clock"]
    Clk24 = 4,
    #[doc = "6: 16 MHz MSPI clock"]
    Clk16 = 6,
    #[doc = "8: 12 MHz MSPI clock"]
    Clk12 = 8,
    #[doc = "12: 8 MHz MSPI clock"]
    Clk8 = 12,
    #[doc = "16: 6 MHz MSPI clock"]
    Clk6 = 16,
    #[doc = "24: 4 MHz MSPI clock"]
    Clk4 = 24,
    #[doc = "32: 3 MHz MSPI clock"]
    Clk3 = 32,
}
impl From<Clkdiv0> for u8 {
    #[inline(always)]
    fn from(variant: Clkdiv0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkdiv0 {
    type Ux = u8;
}
impl crate::IsEnum for Clkdiv0 {}
#[doc = "Field `CLKDIV0` reader - Clock Divider. Allows dividing 96 MHz base clock by integer multiples. Enumerations are provided for common frequency, but any integer divide from 96 MHz is allowed. Odd divide ratios will result in a 33/66 percent duty cycle with a long low clock pulse (to allow longer round-trip for read data)."]
pub type Clkdiv0R = crate::FieldReader<Clkdiv0>;
impl Clkdiv0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkdiv0> {
        match self.bits {
            1 => Some(Clkdiv0::Clk96),
            2 => Some(Clkdiv0::Clk48),
            3 => Some(Clkdiv0::Clk32),
            4 => Some(Clkdiv0::Clk24),
            6 => Some(Clkdiv0::Clk16),
            8 => Some(Clkdiv0::Clk12),
            12 => Some(Clkdiv0::Clk8),
            16 => Some(Clkdiv0::Clk6),
            24 => Some(Clkdiv0::Clk4),
            32 => Some(Clkdiv0::Clk3),
            _ => None,
        }
    }
    #[doc = "96 MHz MSPI clock"]
    #[inline(always)]
    pub fn is_clk96(&self) -> bool {
        *self == Clkdiv0::Clk96
    }
    #[doc = "48 MHz MSPI clock"]
    #[inline(always)]
    pub fn is_clk48(&self) -> bool {
        *self == Clkdiv0::Clk48
    }
    #[doc = "32 MHz MSPI clock"]
    #[inline(always)]
    pub fn is_clk32(&self) -> bool {
        *self == Clkdiv0::Clk32
    }
    #[doc = "24 MHz MSPI clock"]
    #[inline(always)]
    pub fn is_clk24(&self) -> bool {
        *self == Clkdiv0::Clk24
    }
    #[doc = "16 MHz MSPI clock"]
    #[inline(always)]
    pub fn is_clk16(&self) -> bool {
        *self == Clkdiv0::Clk16
    }
    #[doc = "12 MHz MSPI clock"]
    #[inline(always)]
    pub fn is_clk12(&self) -> bool {
        *self == Clkdiv0::Clk12
    }
    #[doc = "8 MHz MSPI clock"]
    #[inline(always)]
    pub fn is_clk8(&self) -> bool {
        *self == Clkdiv0::Clk8
    }
    #[doc = "6 MHz MSPI clock"]
    #[inline(always)]
    pub fn is_clk6(&self) -> bool {
        *self == Clkdiv0::Clk6
    }
    #[doc = "4 MHz MSPI clock"]
    #[inline(always)]
    pub fn is_clk4(&self) -> bool {
        *self == Clkdiv0::Clk4
    }
    #[doc = "3 MHz MSPI clock"]
    #[inline(always)]
    pub fn is_clk3(&self) -> bool {
        *self == Clkdiv0::Clk3
    }
}
#[doc = "Field `CLKDIV0` writer - Clock Divider. Allows dividing 96 MHz base clock by integer multiples. Enumerations are provided for common frequency, but any integer divide from 96 MHz is allowed. Odd divide ratios will result in a 33/66 percent duty cycle with a long low clock pulse (to allow longer round-trip for read data)."]
pub type Clkdiv0W<'a, REG> = crate::FieldWriter<'a, REG, 6, Clkdiv0>;
impl<'a, REG> Clkdiv0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "96 MHz MSPI clock"]
    #[inline(always)]
    pub fn clk96(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv0::Clk96)
    }
    #[doc = "48 MHz MSPI clock"]
    #[inline(always)]
    pub fn clk48(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv0::Clk48)
    }
    #[doc = "32 MHz MSPI clock"]
    #[inline(always)]
    pub fn clk32(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv0::Clk32)
    }
    #[doc = "24 MHz MSPI clock"]
    #[inline(always)]
    pub fn clk24(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv0::Clk24)
    }
    #[doc = "16 MHz MSPI clock"]
    #[inline(always)]
    pub fn clk16(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv0::Clk16)
    }
    #[doc = "12 MHz MSPI clock"]
    #[inline(always)]
    pub fn clk12(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv0::Clk12)
    }
    #[doc = "8 MHz MSPI clock"]
    #[inline(always)]
    pub fn clk8(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv0::Clk8)
    }
    #[doc = "6 MHz MSPI clock"]
    #[inline(always)]
    pub fn clk6(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv0::Clk6)
    }
    #[doc = "4 MHz MSPI clock"]
    #[inline(always)]
    pub fn clk4(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv0::Clk4)
    }
    #[doc = "3 MHz MSPI clock"]
    #[inline(always)]
    pub fn clk3(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv0::Clk3)
    }
}
#[doc = "Controls RX data capture phase. A setting of 0 (NORMAL) captures read data at the normal capture point relative to the internal clock launch point. However, to accomodate chip/pad/board delays, a setting of RXCAP of 1 is expected to be used to align the capture point with the return data window. This bit is used in conjunction with RXNEG to provide 4 unique capture points, all about 10ns apart.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxcap0 {
    #[doc = "0: RX Capture phase aligns with CPHA setting"]
    Normal = 0,
    #[doc = "1: RX Capture phase is delayed from CPHA setting by one clock edge"]
    Delay = 1,
}
impl From<Rxcap0> for bool {
    #[inline(always)]
    fn from(variant: Rxcap0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXCAP0` reader - Controls RX data capture phase. A setting of 0 (NORMAL) captures read data at the normal capture point relative to the internal clock launch point. However, to accomodate chip/pad/board delays, a setting of RXCAP of 1 is expected to be used to align the capture point with the return data window. This bit is used in conjunction with RXNEG to provide 4 unique capture points, all about 10ns apart."]
pub type Rxcap0R = crate::BitReader<Rxcap0>;
impl Rxcap0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxcap0 {
        match self.bits {
            false => Rxcap0::Normal,
            true => Rxcap0::Delay,
        }
    }
    #[doc = "RX Capture phase aligns with CPHA setting"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Rxcap0::Normal
    }
    #[doc = "RX Capture phase is delayed from CPHA setting by one clock edge"]
    #[inline(always)]
    pub fn is_delay(&self) -> bool {
        *self == Rxcap0::Delay
    }
}
#[doc = "Field `RXCAP0` writer - Controls RX data capture phase. A setting of 0 (NORMAL) captures read data at the normal capture point relative to the internal clock launch point. However, to accomodate chip/pad/board delays, a setting of RXCAP of 1 is expected to be used to align the capture point with the return data window. This bit is used in conjunction with RXNEG to provide 4 unique capture points, all about 10ns apart."]
pub type Rxcap0W<'a, REG> = crate::BitWriter<'a, REG, Rxcap0>;
impl<'a, REG> Rxcap0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX Capture phase aligns with CPHA setting"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Rxcap0::Normal)
    }
    #[doc = "RX Capture phase is delayed from CPHA setting by one clock edge"]
    #[inline(always)]
    pub fn delay(self) -> &'a mut crate::W<REG> {
        self.variant(Rxcap0::Delay)
    }
}
#[doc = "Adjusts the RX capture phase to the negedge of the 48MHz internal clock (~10ns early). For normal operation, it is expected that RXNEG will be set to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxneg0 {
    #[doc = "0: RX data sampled on posedge of internal clock"]
    Normal = 0,
    #[doc = "1: RX data sampled on negedge of internal clock"]
    Negedge = 1,
}
impl From<Rxneg0> for bool {
    #[inline(always)]
    fn from(variant: Rxneg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXNEG0` reader - Adjusts the RX capture phase to the negedge of the 48MHz internal clock (~10ns early). For normal operation, it is expected that RXNEG will be set to 0."]
pub type Rxneg0R = crate::BitReader<Rxneg0>;
impl Rxneg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxneg0 {
        match self.bits {
            false => Rxneg0::Normal,
            true => Rxneg0::Negedge,
        }
    }
    #[doc = "RX data sampled on posedge of internal clock"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Rxneg0::Normal
    }
    #[doc = "RX data sampled on negedge of internal clock"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == Rxneg0::Negedge
    }
}
#[doc = "Field `RXNEG0` writer - Adjusts the RX capture phase to the negedge of the 48MHz internal clock (~10ns early). For normal operation, it is expected that RXNEG will be set to 0."]
pub type Rxneg0W<'a, REG> = crate::BitWriter<'a, REG, Rxneg0>;
impl<'a, REG> Rxneg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX data sampled on posedge of internal clock"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Rxneg0::Normal)
    }
    #[doc = "RX data sampled on negedge of internal clock"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut crate::W<REG> {
        self.variant(Rxneg0::Negedge)
    }
}
#[doc = "Launches TX data a half clock cycle (~10ns) early. This should normally be programmed to zero (NORMAL).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txneg0 {
    #[doc = "0: TX launched from posedge internal clock"]
    Normal = 0,
    #[doc = "1: TX data launched from negedge of internal clock"]
    Negedge = 1,
}
impl From<Txneg0> for bool {
    #[inline(always)]
    fn from(variant: Txneg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXNEG0` reader - Launches TX data a half clock cycle (~10ns) early. This should normally be programmed to zero (NORMAL)."]
pub type Txneg0R = crate::BitReader<Txneg0>;
impl Txneg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txneg0 {
        match self.bits {
            false => Txneg0::Normal,
            true => Txneg0::Negedge,
        }
    }
    #[doc = "TX launched from posedge internal clock"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Txneg0::Normal
    }
    #[doc = "TX data launched from negedge of internal clock"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == Txneg0::Negedge
    }
}
#[doc = "Field `TXNEG0` writer - Launches TX data a half clock cycle (~10ns) early. This should normally be programmed to zero (NORMAL)."]
pub type Txneg0W<'a, REG> = crate::BitWriter<'a, REG, Txneg0>;
impl<'a, REG> Txneg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX launched from posedge internal clock"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Txneg0::Normal)
    }
    #[doc = "TX data launched from negedge of internal clock"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut crate::W<REG> {
        self.variant(Txneg0::Negedge)
    }
}
#[doc = "Field `SEPIO0` reader - Separate IO configuration. This bit should be set when the target device has separate MOSI and MISO pins. Respective IN/OUT bits below should be set to map pins."]
pub type Sepio0R = crate::BitReader;
#[doc = "Field `SEPIO0` writer - Separate IO configuration. This bit should be set when the target device has separate MOSI and MISO pins. Respective IN/OUT bits below should be set to map pins."]
pub type Sepio0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITELATENCY0` reader - Number of write Latency cycles. Qualified by ENTURN bit field."]
pub type Writelatency0R = crate::FieldReader;
#[doc = "Field `WRITELATENCY0` writer - Number of write Latency cycles. Qualified by ENTURN bit field."]
pub type Writelatency0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:4 - Flash configuration for XIP and AUTO DMA operations. Controls value for SER (Slave Enable) for XIP operations and address generation for DMA/XIP modes. Also used to configure SPIFRF (frame format)."]
    #[inline(always)]
    pub fn devcfg0(&self) -> Devcfg0R {
        Devcfg0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Address Size. Address bytes to send from ADDR register"]
    #[inline(always)]
    pub fn asize0(&self) -> Asize0R {
        Asize0R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Instruction Size"]
    #[inline(always)]
    pub fn isize0(&self) -> Isize0R {
        Isize0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Number of turnaound cycles (for TX->RX transitions). Qualified by ENTURN bit field."]
    #[inline(always)]
    pub fn turnaround0(&self) -> Turnaround0R {
        Turnaround0R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Serial clock phase."]
    #[inline(always)]
    pub fn cpha0(&self) -> Cpha0R {
        Cpha0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Serial clock polarity."]
    #[inline(always)]
    pub fn cpol0(&self) -> Cpol0R {
        Cpol0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Clock Divider. Allows dividing 96 MHz base clock by integer multiples. Enumerations are provided for common frequency, but any integer divide from 96 MHz is allowed. Odd divide ratios will result in a 33/66 percent duty cycle with a long low clock pulse (to allow longer round-trip for read data)."]
    #[inline(always)]
    pub fn clkdiv0(&self) -> Clkdiv0R {
        Clkdiv0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Controls RX data capture phase. A setting of 0 (NORMAL) captures read data at the normal capture point relative to the internal clock launch point. However, to accomodate chip/pad/board delays, a setting of RXCAP of 1 is expected to be used to align the capture point with the return data window. This bit is used in conjunction with RXNEG to provide 4 unique capture points, all about 10ns apart."]
    #[inline(always)]
    pub fn rxcap0(&self) -> Rxcap0R {
        Rxcap0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Adjusts the RX capture phase to the negedge of the 48MHz internal clock (~10ns early). For normal operation, it is expected that RXNEG will be set to 0."]
    #[inline(always)]
    pub fn rxneg0(&self) -> Rxneg0R {
        Rxneg0R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Launches TX data a half clock cycle (~10ns) early. This should normally be programmed to zero (NORMAL)."]
    #[inline(always)]
    pub fn txneg0(&self) -> Txneg0R {
        Txneg0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Separate IO configuration. This bit should be set when the target device has separate MOSI and MISO pins. Respective IN/OUT bits below should be set to map pins."]
    #[inline(always)]
    pub fn sepio0(&self) -> Sepio0R {
        Sepio0R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - Number of write Latency cycles. Qualified by ENTURN bit field."]
    #[inline(always)]
    pub fn writelatency0(&self) -> Writelatency0R {
        Writelatency0R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Flash configuration for XIP and AUTO DMA operations. Controls value for SER (Slave Enable) for XIP operations and address generation for DMA/XIP modes. Also used to configure SPIFRF (frame format)."]
    #[inline(always)]
    #[must_use]
    pub fn devcfg0(&mut self) -> Devcfg0W<Dev0cfgSpec> {
        Devcfg0W::new(self, 0)
    }
    #[doc = "Bits 5:6 - Address Size. Address bytes to send from ADDR register"]
    #[inline(always)]
    #[must_use]
    pub fn asize0(&mut self) -> Asize0W<Dev0cfgSpec> {
        Asize0W::new(self, 5)
    }
    #[doc = "Bit 7 - Instruction Size"]
    #[inline(always)]
    #[must_use]
    pub fn isize0(&mut self) -> Isize0W<Dev0cfgSpec> {
        Isize0W::new(self, 7)
    }
    #[doc = "Bits 8:13 - Number of turnaound cycles (for TX->RX transitions). Qualified by ENTURN bit field."]
    #[inline(always)]
    #[must_use]
    pub fn turnaround0(&mut self) -> Turnaround0W<Dev0cfgSpec> {
        Turnaround0W::new(self, 8)
    }
    #[doc = "Bit 14 - Serial clock phase."]
    #[inline(always)]
    #[must_use]
    pub fn cpha0(&mut self) -> Cpha0W<Dev0cfgSpec> {
        Cpha0W::new(self, 14)
    }
    #[doc = "Bit 15 - Serial clock polarity."]
    #[inline(always)]
    #[must_use]
    pub fn cpol0(&mut self) -> Cpol0W<Dev0cfgSpec> {
        Cpol0W::new(self, 15)
    }
    #[doc = "Bits 16:21 - Clock Divider. Allows dividing 96 MHz base clock by integer multiples. Enumerations are provided for common frequency, but any integer divide from 96 MHz is allowed. Odd divide ratios will result in a 33/66 percent duty cycle with a long low clock pulse (to allow longer round-trip for read data)."]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv0(&mut self) -> Clkdiv0W<Dev0cfgSpec> {
        Clkdiv0W::new(self, 16)
    }
    #[doc = "Bit 22 - Controls RX data capture phase. A setting of 0 (NORMAL) captures read data at the normal capture point relative to the internal clock launch point. However, to accomodate chip/pad/board delays, a setting of RXCAP of 1 is expected to be used to align the capture point with the return data window. This bit is used in conjunction with RXNEG to provide 4 unique capture points, all about 10ns apart."]
    #[inline(always)]
    #[must_use]
    pub fn rxcap0(&mut self) -> Rxcap0W<Dev0cfgSpec> {
        Rxcap0W::new(self, 22)
    }
    #[doc = "Bit 23 - Adjusts the RX capture phase to the negedge of the 48MHz internal clock (~10ns early). For normal operation, it is expected that RXNEG will be set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn rxneg0(&mut self) -> Rxneg0W<Dev0cfgSpec> {
        Rxneg0W::new(self, 23)
    }
    #[doc = "Bit 24 - Launches TX data a half clock cycle (~10ns) early. This should normally be programmed to zero (NORMAL)."]
    #[inline(always)]
    #[must_use]
    pub fn txneg0(&mut self) -> Txneg0W<Dev0cfgSpec> {
        Txneg0W::new(self, 24)
    }
    #[doc = "Bit 25 - Separate IO configuration. This bit should be set when the target device has separate MOSI and MISO pins. Respective IN/OUT bits below should be set to map pins."]
    #[inline(always)]
    #[must_use]
    pub fn sepio0(&mut self) -> Sepio0W<Dev0cfgSpec> {
        Sepio0W::new(self, 25)
    }
    #[doc = "Bits 26:31 - Number of write Latency cycles. Qualified by ENTURN bit field."]
    #[inline(always)]
    #[must_use]
    pub fn writelatency0(&mut self) -> Writelatency0W<Dev0cfgSpec> {
        Writelatency0W::new(self, 26)
    }
}
#[doc = "Command formatting for PIO based transactions (initiated by writes to CTRL register)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev0cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev0cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dev0cfgSpec;
impl crate::RegisterSpec for Dev0cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev0cfg::R`](R) reader structure"]
impl crate::Readable for Dev0cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dev0cfg::W`](W) writer structure"]
impl crate::Writable for Dev0cfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEV0CFG to value 0x0002_0001"]
impl crate::Resettable for Dev0cfgSpec {
    const RESET_VALUE: u32 = 0x0002_0001;
}
