#[doc = "Register `MI2CCFG` reader"]
pub type R = crate::R<Mi2ccfgSpec>;
#[doc = "Register `MI2CCFG` writer"]
pub type W = crate::W<Mi2ccfgSpec>;
#[doc = "Sets the I2C master device address size to either 7b (0) or 10b (1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addrsz {
    #[doc = "0: Use 7b addressing for I2C master transactions"]
    Addrsz7 = 0,
    #[doc = "1: Use 10b addressing for I2C master transactions"]
    Addrsz10 = 1,
}
impl From<Addrsz> for bool {
    #[inline(always)]
    fn from(variant: Addrsz) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRSZ` reader - Sets the I2C master device address size to either 7b (0) or 10b (1)."]
pub type AddrszR = crate::BitReader<Addrsz>;
impl AddrszR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addrsz {
        match self.bits {
            false => Addrsz::Addrsz7,
            true => Addrsz::Addrsz10,
        }
    }
    #[doc = "Use 7b addressing for I2C master transactions"]
    #[inline(always)]
    pub fn is_addrsz7(&self) -> bool {
        *self == Addrsz::Addrsz7
    }
    #[doc = "Use 10b addressing for I2C master transactions"]
    #[inline(always)]
    pub fn is_addrsz10(&self) -> bool {
        *self == Addrsz::Addrsz10
    }
}
#[doc = "Field `ADDRSZ` writer - Sets the I2C master device address size to either 7b (0) or 10b (1)."]
pub type AddrszW<'a, REG> = crate::BitWriter<'a, REG, Addrsz>;
impl<'a, REG> AddrszW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use 7b addressing for I2C master transactions"]
    #[inline(always)]
    pub fn addrsz7(self) -> &'a mut crate::W<REG> {
        self.variant(Addrsz::Addrsz7)
    }
    #[doc = "Use 10b addressing for I2C master transactions"]
    #[inline(always)]
    pub fn addrsz10(self) -> &'a mut crate::W<REG> {
        self.variant(Addrsz::Addrsz10)
    }
}
#[doc = "Direction of data transmit and receive, MSB(0) or LSB(1) first. Default per I2C specification is MSB first. This applies to both read and write data, and read data will be bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2clsb {
    #[doc = "0: Byte data is transmitted MSB first onto the bus/read from the bus"]
    Msbfirst = 0,
    #[doc = "1: Byte data is transmitted LSB first onto the bus/read from the bus"]
    Lsbfirst = 1,
}
impl From<I2clsb> for bool {
    #[inline(always)]
    fn from(variant: I2clsb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2CLSB` reader - Direction of data transmit and receive, MSB(0) or LSB(1) first. Default per I2C specification is MSB first. This applies to both read and write data, and read data will be bit"]
pub type I2clsbR = crate::BitReader<I2clsb>;
impl I2clsbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2clsb {
        match self.bits {
            false => I2clsb::Msbfirst,
            true => I2clsb::Lsbfirst,
        }
    }
    #[doc = "Byte data is transmitted MSB first onto the bus/read from the bus"]
    #[inline(always)]
    pub fn is_msbfirst(&self) -> bool {
        *self == I2clsb::Msbfirst
    }
    #[doc = "Byte data is transmitted LSB first onto the bus/read from the bus"]
    #[inline(always)]
    pub fn is_lsbfirst(&self) -> bool {
        *self == I2clsb::Lsbfirst
    }
}
#[doc = "Field `I2CLSB` writer - Direction of data transmit and receive, MSB(0) or LSB(1) first. Default per I2C specification is MSB first. This applies to both read and write data, and read data will be bit"]
pub type I2clsbW<'a, REG> = crate::BitWriter<'a, REG, I2clsb>;
impl<'a, REG> I2clsbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Byte data is transmitted MSB first onto the bus/read from the bus"]
    #[inline(always)]
    pub fn msbfirst(self) -> &'a mut crate::W<REG> {
        self.variant(I2clsb::Msbfirst)
    }
    #[doc = "Byte data is transmitted LSB first onto the bus/read from the bus"]
    #[inline(always)]
    pub fn lsbfirst(self) -> &'a mut crate::W<REG> {
        self.variant(I2clsb::Lsbfirst)
    }
}
#[doc = "Enables multi-master arbitration for the I2C master. If the bus is known to have only a single master, this function can be disabled to save clock cycles on I2C transactions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arben {
    #[doc = "1: Enable multi-master bus arbitration support for this i2c master"]
    Arbenable = 1,
    #[doc = "0: Disable multi-master bus arbitration support for this i2c master"]
    Arbdisable = 0,
}
impl From<Arben> for bool {
    #[inline(always)]
    fn from(variant: Arben) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARBEN` reader - Enables multi-master arbitration for the I2C master. If the bus is known to have only a single master, this function can be disabled to save clock cycles on I2C transactions"]
pub type ArbenR = crate::BitReader<Arben>;
impl ArbenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arben {
        match self.bits {
            true => Arben::Arbenable,
            false => Arben::Arbdisable,
        }
    }
    #[doc = "Enable multi-master bus arbitration support for this i2c master"]
    #[inline(always)]
    pub fn is_arbenable(&self) -> bool {
        *self == Arben::Arbenable
    }
    #[doc = "Disable multi-master bus arbitration support for this i2c master"]
    #[inline(always)]
    pub fn is_arbdisable(&self) -> bool {
        *self == Arben::Arbdisable
    }
}
#[doc = "Field `ARBEN` writer - Enables multi-master arbitration for the I2C master. If the bus is known to have only a single master, this function can be disabled to save clock cycles on I2C transactions"]
pub type ArbenW<'a, REG> = crate::BitWriter<'a, REG, Arben>;
impl<'a, REG> ArbenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable multi-master bus arbitration support for this i2c master"]
    #[inline(always)]
    pub fn arbenable(self) -> &'a mut crate::W<REG> {
        self.variant(Arben::Arbenable)
    }
    #[doc = "Disable multi-master bus arbitration support for this i2c master"]
    #[inline(always)]
    pub fn arbdisable(self) -> &'a mut crate::W<REG> {
        self.variant(Arben::Arbdisable)
    }
}
#[doc = "Field `SDADLY` reader - Delay to enable on the SDA output. Values are 0x0-0x3."]
pub type SdadlyR = crate::FieldReader;
#[doc = "Field `SDADLY` writer - Delay to enable on the SDA output. Values are 0x0-0x3."]
pub type SdadlyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MI2CRST` reader - Not used. To reset the module, toggle the SMOD_EN for the module"]
pub type Mi2crstR = crate::BitReader;
#[doc = "Field `MI2CRST` writer - Not used. To reset the module, toggle the SMOD_EN for the module"]
pub type Mi2crstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLENDLY` reader - Number of IOCLK cycles to delay the rising edge of the SCL output en (clock will go low on this edge). Used to allow clock shaping."]
pub type SclendlyR = crate::FieldReader;
#[doc = "Field `SCLENDLY` writer - Number of IOCLK cycles to delay the rising edge of the SCL output en (clock will go low on this edge). Used to allow clock shaping."]
pub type SclendlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SDAENDLY` reader - Number of IOCLK cycles to delay the SDA output en (all transitions affected). Used to delay data relative to clock"]
pub type SdaendlyR = crate::FieldReader;
#[doc = "Field `SDAENDLY` writer - Number of IOCLK cycles to delay the SDA output en (all transitions affected). Used to delay data relative to clock"]
pub type SdaendlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SMPCNT` reader - Number of Base clk cycles to wait before sampling the SCL clock to determine if a clock stretch event has occured"]
pub type SmpcntR = crate::FieldReader;
#[doc = "Field `SMPCNT` writer - Number of Base clk cycles to wait before sampling the SCL clock to determine if a clock stretch event has occured"]
pub type SmpcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `STRDIS` reader - Disable detection of clock stretch events smaller than 1 cycle"]
pub type StrdisR = crate::BitReader;
#[doc = "Field `STRDIS` writer - Disable detection of clock stretch events smaller than 1 cycle"]
pub type StrdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sets the I2C master device address size to either 7b (0) or 10b (1)."]
    #[inline(always)]
    pub fn addrsz(&self) -> AddrszR {
        AddrszR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direction of data transmit and receive, MSB(0) or LSB(1) first. Default per I2C specification is MSB first. This applies to both read and write data, and read data will be bit"]
    #[inline(always)]
    pub fn i2clsb(&self) -> I2clsbR {
        I2clsbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables multi-master arbitration for the I2C master. If the bus is known to have only a single master, this function can be disabled to save clock cycles on I2C transactions"]
    #[inline(always)]
    pub fn arben(&self) -> ArbenR {
        ArbenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Delay to enable on the SDA output. Values are 0x0-0x3."]
    #[inline(always)]
    pub fn sdadly(&self) -> SdadlyR {
        SdadlyR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Not used. To reset the module, toggle the SMOD_EN for the module"]
    #[inline(always)]
    pub fn mi2crst(&self) -> Mi2crstR {
        Mi2crstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Number of IOCLK cycles to delay the rising edge of the SCL output en (clock will go low on this edge). Used to allow clock shaping."]
    #[inline(always)]
    pub fn sclendly(&self) -> SclendlyR {
        SclendlyR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Number of IOCLK cycles to delay the SDA output en (all transitions affected). Used to delay data relative to clock"]
    #[inline(always)]
    pub fn sdaendly(&self) -> SdaendlyR {
        SdaendlyR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Number of Base clk cycles to wait before sampling the SCL clock to determine if a clock stretch event has occured"]
    #[inline(always)]
    pub fn smpcnt(&self) -> SmpcntR {
        SmpcntR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Disable detection of clock stretch events smaller than 1 cycle"]
    #[inline(always)]
    pub fn strdis(&self) -> StrdisR {
        StrdisR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sets the I2C master device address size to either 7b (0) or 10b (1)."]
    #[inline(always)]
    #[must_use]
    pub fn addrsz(&mut self) -> AddrszW<Mi2ccfgSpec> {
        AddrszW::new(self, 0)
    }
    #[doc = "Bit 1 - Direction of data transmit and receive, MSB(0) or LSB(1) first. Default per I2C specification is MSB first. This applies to both read and write data, and read data will be bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2clsb(&mut self) -> I2clsbW<Mi2ccfgSpec> {
        I2clsbW::new(self, 1)
    }
    #[doc = "Bit 2 - Enables multi-master arbitration for the I2C master. If the bus is known to have only a single master, this function can be disabled to save clock cycles on I2C transactions"]
    #[inline(always)]
    #[must_use]
    pub fn arben(&mut self) -> ArbenW<Mi2ccfgSpec> {
        ArbenW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Delay to enable on the SDA output. Values are 0x0-0x3."]
    #[inline(always)]
    #[must_use]
    pub fn sdadly(&mut self) -> SdadlyW<Mi2ccfgSpec> {
        SdadlyW::new(self, 4)
    }
    #[doc = "Bit 6 - Not used. To reset the module, toggle the SMOD_EN for the module"]
    #[inline(always)]
    #[must_use]
    pub fn mi2crst(&mut self) -> Mi2crstW<Mi2ccfgSpec> {
        Mi2crstW::new(self, 6)
    }
    #[doc = "Bits 8:11 - Number of IOCLK cycles to delay the rising edge of the SCL output en (clock will go low on this edge). Used to allow clock shaping."]
    #[inline(always)]
    #[must_use]
    pub fn sclendly(&mut self) -> SclendlyW<Mi2ccfgSpec> {
        SclendlyW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Number of IOCLK cycles to delay the SDA output en (all transitions affected). Used to delay data relative to clock"]
    #[inline(always)]
    #[must_use]
    pub fn sdaendly(&mut self) -> SdaendlyW<Mi2ccfgSpec> {
        SdaendlyW::new(self, 12)
    }
    #[doc = "Bits 16:23 - Number of Base clk cycles to wait before sampling the SCL clock to determine if a clock stretch event has occured"]
    #[inline(always)]
    #[must_use]
    pub fn smpcnt(&mut self) -> SmpcntW<Mi2ccfgSpec> {
        SmpcntW::new(self, 16)
    }
    #[doc = "Bit 24 - Disable detection of clock stretch events smaller than 1 cycle"]
    #[inline(always)]
    #[must_use]
    pub fn strdis(&mut self) -> StrdisW<Mi2ccfgSpec> {
        StrdisW::new(self, 24)
    }
}
#[doc = "Controls the configuration of the I2C bus master.\n\nYou can [`read`](crate::Reg::read) this register and get [`mi2ccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mi2ccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mi2ccfgSpec;
impl crate::RegisterSpec for Mi2ccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi2ccfg::R`](R) reader structure"]
impl crate::Readable for Mi2ccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`mi2ccfg::W`](W) writer structure"]
impl crate::Writable for Mi2ccfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI2CCFG to value 0"]
impl crate::Resettable for Mi2ccfgSpec {
    const RESET_VALUE: u32 = 0;
}
