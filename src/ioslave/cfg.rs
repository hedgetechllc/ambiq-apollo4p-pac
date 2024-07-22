#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "This bit selects the I/O interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ifcsel {
    #[doc = "0: Selects I2C interface for the IO Slave."]
    I2c = 0,
    #[doc = "1: Selects SPI interface for the IO Slave."]
    Spi = 1,
}
impl From<Ifcsel> for bool {
    #[inline(always)]
    fn from(variant: Ifcsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IFCSEL` reader - This bit selects the I/O interface."]
pub type IfcselR = crate::BitReader<Ifcsel>;
impl IfcselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ifcsel {
        match self.bits {
            false => Ifcsel::I2c,
            true => Ifcsel::Spi,
        }
    }
    #[doc = "Selects I2C interface for the IO Slave."]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == Ifcsel::I2c
    }
    #[doc = "Selects SPI interface for the IO Slave."]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == Ifcsel::Spi
    }
}
#[doc = "Field `IFCSEL` writer - This bit selects the I/O interface."]
pub type IfcselW<'a, REG> = crate::BitWriter<'a, REG, Ifcsel>;
impl<'a, REG> IfcselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selects I2C interface for the IO Slave."]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut crate::W<REG> {
        self.variant(Ifcsel::I2c)
    }
    #[doc = "Selects SPI interface for the IO Slave."]
    #[inline(always)]
    pub fn spi(self) -> &'a mut crate::W<REG> {
        self.variant(Ifcsel::Spi)
    }
}
#[doc = "This bit selects SPI polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spol {
    #[doc = "0: Polarity 0, handles SPI modes 0 and 3."]
    SpiModes0_3 = 0,
    #[doc = "1: Polarity 1, handles SPI modes 1 and 2."]
    SpiModes1_2 = 1,
}
impl From<Spol> for bool {
    #[inline(always)]
    fn from(variant: Spol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPOL` reader - This bit selects SPI polarity."]
pub type SpolR = crate::BitReader<Spol>;
impl SpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spol {
        match self.bits {
            false => Spol::SpiModes0_3,
            true => Spol::SpiModes1_2,
        }
    }
    #[doc = "Polarity 0, handles SPI modes 0 and 3."]
    #[inline(always)]
    pub fn is_spi_modes_0_3(&self) -> bool {
        *self == Spol::SpiModes0_3
    }
    #[doc = "Polarity 1, handles SPI modes 1 and 2."]
    #[inline(always)]
    pub fn is_spi_modes_1_2(&self) -> bool {
        *self == Spol::SpiModes1_2
    }
}
#[doc = "Field `SPOL` writer - This bit selects SPI polarity."]
pub type SpolW<'a, REG> = crate::BitWriter<'a, REG, Spol>;
impl<'a, REG> SpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Polarity 0, handles SPI modes 0 and 3."]
    #[inline(always)]
    pub fn spi_modes_0_3(self) -> &'a mut crate::W<REG> {
        self.variant(Spol::SpiModes0_3)
    }
    #[doc = "Polarity 1, handles SPI modes 1 and 2."]
    #[inline(always)]
    pub fn spi_modes_1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Spol::SpiModes1_2)
    }
}
#[doc = "This bit selects the transfer bit ordering.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsb {
    #[doc = "0: Data is assumed to be sent and received with MSB first."]
    MsbFirst = 0,
    #[doc = "1: Data is assumed to be sent and received with LSB first."]
    LsbFirst = 1,
}
impl From<Lsb> for bool {
    #[inline(always)]
    fn from(variant: Lsb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSB` reader - This bit selects the transfer bit ordering."]
pub type LsbR = crate::BitReader<Lsb>;
impl LsbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsb {
        match self.bits {
            false => Lsb::MsbFirst,
            true => Lsb::LsbFirst,
        }
    }
    #[doc = "Data is assumed to be sent and received with MSB first."]
    #[inline(always)]
    pub fn is_msb_first(&self) -> bool {
        *self == Lsb::MsbFirst
    }
    #[doc = "Data is assumed to be sent and received with LSB first."]
    #[inline(always)]
    pub fn is_lsb_first(&self) -> bool {
        *self == Lsb::LsbFirst
    }
}
#[doc = "Field `LSB` writer - This bit selects the transfer bit ordering."]
pub type LsbW<'a, REG> = crate::BitWriter<'a, REG, Lsb>;
impl<'a, REG> LsbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is assumed to be sent and received with MSB first."]
    #[inline(always)]
    pub fn msb_first(self) -> &'a mut crate::W<REG> {
        self.variant(Lsb::MsbFirst)
    }
    #[doc = "Data is assumed to be sent and received with LSB first."]
    #[inline(always)]
    pub fn lsb_first(self) -> &'a mut crate::W<REG> {
        self.variant(Lsb::LsbFirst)
    }
}
#[doc = "This bit holds the cycle to initiate an I/O RAM read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Startrd {
    #[doc = "0: Initiate I/O RAM read late in each transferred byte."]
    Late = 0,
    #[doc = "1: Initiate I/O RAM read early in each transferred byte."]
    Early = 1,
}
impl From<Startrd> for bool {
    #[inline(always)]
    fn from(variant: Startrd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARTRD` reader - This bit holds the cycle to initiate an I/O RAM read."]
pub type StartrdR = crate::BitReader<Startrd>;
impl StartrdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Startrd {
        match self.bits {
            false => Startrd::Late,
            true => Startrd::Early,
        }
    }
    #[doc = "Initiate I/O RAM read late in each transferred byte."]
    #[inline(always)]
    pub fn is_late(&self) -> bool {
        *self == Startrd::Late
    }
    #[doc = "Initiate I/O RAM read early in each transferred byte."]
    #[inline(always)]
    pub fn is_early(&self) -> bool {
        *self == Startrd::Early
    }
}
#[doc = "Field `STARTRD` writer - This bit holds the cycle to initiate an I/O RAM read."]
pub type StartrdW<'a, REG> = crate::BitWriter<'a, REG, Startrd>;
impl<'a, REG> StartrdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Initiate I/O RAM read late in each transferred byte."]
    #[inline(always)]
    pub fn late(self) -> &'a mut crate::W<REG> {
        self.variant(Startrd::Late)
    }
    #[doc = "Initiate I/O RAM read early in each transferred byte."]
    #[inline(always)]
    pub fn early(self) -> &'a mut crate::W<REG> {
        self.variant(Startrd::Early)
    }
}
#[doc = "Field `I2CADDR` reader - 7-bit or 10-bit I2C device address."]
pub type I2caddrR = crate::FieldReader<u16>;
#[doc = "Field `I2CADDR` writer - 7-bit or 10-bit I2C device address."]
pub type I2caddrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Address pointer wrap mode enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrapptr {
    #[doc = "0: Address pointer does not wrap around to FIFOBASE*8 after it reaches FIFOMAX*8-1. Additionally, the address pointer does not automatically skip Direct Area locations 0x78 to 0x7F, so care must be taken that the host does not inadvertently write to the Host Registers during a data transfer."]
    Nowrap = 0,
    #[doc = "1: Address pointer wraps around to FIFOBASE*8 after it reaches FIFOMAX*8-1 to accommodate any length transfers. In addition, the address pointer automatically skips Direct Area locations 0x78 to 0x7F (if the FIFO Area encompasses these locations) to avoid writing to the Host Registers."]
    Wrap = 1,
}
impl From<Wrapptr> for bool {
    #[inline(always)]
    fn from(variant: Wrapptr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRAPPTR` reader - Address pointer wrap mode enable."]
pub type WrapptrR = crate::BitReader<Wrapptr>;
impl WrapptrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wrapptr {
        match self.bits {
            false => Wrapptr::Nowrap,
            true => Wrapptr::Wrap,
        }
    }
    #[doc = "Address pointer does not wrap around to FIFOBASE*8 after it reaches FIFOMAX*8-1. Additionally, the address pointer does not automatically skip Direct Area locations 0x78 to 0x7F, so care must be taken that the host does not inadvertently write to the Host Registers during a data transfer."]
    #[inline(always)]
    pub fn is_nowrap(&self) -> bool {
        *self == Wrapptr::Nowrap
    }
    #[doc = "Address pointer wraps around to FIFOBASE*8 after it reaches FIFOMAX*8-1 to accommodate any length transfers. In addition, the address pointer automatically skips Direct Area locations 0x78 to 0x7F (if the FIFO Area encompasses these locations) to avoid writing to the Host Registers."]
    #[inline(always)]
    pub fn is_wrap(&self) -> bool {
        *self == Wrapptr::Wrap
    }
}
#[doc = "Field `WRAPPTR` writer - Address pointer wrap mode enable."]
pub type WrapptrW<'a, REG> = crate::BitWriter<'a, REG, Wrapptr>;
impl<'a, REG> WrapptrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Address pointer does not wrap around to FIFOBASE*8 after it reaches FIFOMAX*8-1. Additionally, the address pointer does not automatically skip Direct Area locations 0x78 to 0x7F, so care must be taken that the host does not inadvertently write to the Host Registers during a data transfer."]
    #[inline(always)]
    pub fn nowrap(self) -> &'a mut crate::W<REG> {
        self.variant(Wrapptr::Nowrap)
    }
    #[doc = "Address pointer wraps around to FIFOBASE*8 after it reaches FIFOMAX*8-1 to accommodate any length transfers. In addition, the address pointer automatically skips Direct Area locations 0x78 to 0x7F (if the FIFO Area encompasses these locations) to avoid writing to the Host Registers."]
    #[inline(always)]
    pub fn wrap(self) -> &'a mut crate::W<REG> {
        self.variant(Wrapptr::Wrap)
    }
}
#[doc = "IOSLAVE interface enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ifcen {
    #[doc = "0: Disable the IOSLAVE"]
    Dis = 0,
    #[doc = "1: Enable the IOSLAVE"]
    En = 1,
}
impl From<Ifcen> for bool {
    #[inline(always)]
    fn from(variant: Ifcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IFCEN` reader - IOSLAVE interface enable."]
pub type IfcenR = crate::BitReader<Ifcen>;
impl IfcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ifcen {
        match self.bits {
            false => Ifcen::Dis,
            true => Ifcen::En,
        }
    }
    #[doc = "Disable the IOSLAVE"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ifcen::Dis
    }
    #[doc = "Enable the IOSLAVE"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ifcen::En
    }
}
#[doc = "Field `IFCEN` writer - IOSLAVE interface enable."]
pub type IfcenW<'a, REG> = crate::BitWriter<'a, REG, Ifcen>;
impl<'a, REG> IfcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the IOSLAVE"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ifcen::Dis)
    }
    #[doc = "Enable the IOSLAVE"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ifcen::En)
    }
}
impl R {
    #[doc = "Bit 0 - This bit selects the I/O interface."]
    #[inline(always)]
    pub fn ifcsel(&self) -> IfcselR {
        IfcselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit selects SPI polarity."]
    #[inline(always)]
    pub fn spol(&self) -> SpolR {
        SpolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit selects the transfer bit ordering."]
    #[inline(always)]
    pub fn lsb(&self) -> LsbR {
        LsbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit holds the cycle to initiate an I/O RAM read."]
    #[inline(always)]
    pub fn startrd(&self) -> StartrdR {
        StartrdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:19 - 7-bit or 10-bit I2C device address."]
    #[inline(always)]
    pub fn i2caddr(&self) -> I2caddrR {
        I2caddrR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bit 20 - Address pointer wrap mode enable."]
    #[inline(always)]
    pub fn wrapptr(&self) -> WrapptrR {
        WrapptrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 31 - IOSLAVE interface enable."]
    #[inline(always)]
    pub fn ifcen(&self) -> IfcenR {
        IfcenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit selects the I/O interface."]
    #[inline(always)]
    #[must_use]
    pub fn ifcsel(&mut self) -> IfcselW<CfgSpec> {
        IfcselW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit selects SPI polarity."]
    #[inline(always)]
    #[must_use]
    pub fn spol(&mut self) -> SpolW<CfgSpec> {
        SpolW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit selects the transfer bit ordering."]
    #[inline(always)]
    #[must_use]
    pub fn lsb(&mut self) -> LsbW<CfgSpec> {
        LsbW::new(self, 2)
    }
    #[doc = "Bit 4 - This bit holds the cycle to initiate an I/O RAM read."]
    #[inline(always)]
    #[must_use]
    pub fn startrd(&mut self) -> StartrdW<CfgSpec> {
        StartrdW::new(self, 4)
    }
    #[doc = "Bits 8:19 - 7-bit or 10-bit I2C device address."]
    #[inline(always)]
    #[must_use]
    pub fn i2caddr(&mut self) -> I2caddrW<CfgSpec> {
        I2caddrW::new(self, 8)
    }
    #[doc = "Bit 20 - Address pointer wrap mode enable."]
    #[inline(always)]
    #[must_use]
    pub fn wrapptr(&mut self) -> WrapptrW<CfgSpec> {
        WrapptrW::new(self, 20)
    }
    #[doc = "Bit 31 - IOSLAVE interface enable."]
    #[inline(always)]
    #[must_use]
    pub fn ifcen(&mut self) -> IfcenW<CfgSpec> {
        IfcenW::new(self, 31)
    }
}
#[doc = "I/O Slave Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0;
}
