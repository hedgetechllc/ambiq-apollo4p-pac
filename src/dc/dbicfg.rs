#[doc = "Register `DBICFG` reader"]
pub type R = crate::R<DbicfgSpec>;
#[doc = "Register `DBICFG` writer"]
pub type W = crate::W<DbicfgSpec>;
#[doc = "Set the color format for DBI interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dbicolorfmt {
    #[doc = "1: RGB111 (3 bits/pixel)"]
    DbiFmtRgb111 = 1,
    #[doc = "2: RGB332 (8 bits/pixel)"]
    DbiFmtRgb332 = 2,
    #[doc = "3: RGB444 (12 bits/pixel)"]
    DbiFmtRgb444 = 3,
    #[doc = "5: RGB565 (16 bits/pixel)"]
    DbiFmtRgb565 = 5,
    #[doc = "6: RGB666 (18 bits/pixel)"]
    DbiFmtRgb666 = 6,
    #[doc = "7: RGB888 (24 bits/pixel)"]
    DbiFmtRgb888 = 7,
}
impl From<Dbicolorfmt> for u8 {
    #[inline(always)]
    fn from(variant: Dbicolorfmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dbicolorfmt {
    type Ux = u8;
}
impl crate::IsEnum for Dbicolorfmt {}
#[doc = "Field `DBICOLORFMT` reader - Set the color format for DBI interface"]
pub type DbicolorfmtR = crate::FieldReader<Dbicolorfmt>;
impl DbicolorfmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dbicolorfmt> {
        match self.bits {
            1 => Some(Dbicolorfmt::DbiFmtRgb111),
            2 => Some(Dbicolorfmt::DbiFmtRgb332),
            3 => Some(Dbicolorfmt::DbiFmtRgb444),
            5 => Some(Dbicolorfmt::DbiFmtRgb565),
            6 => Some(Dbicolorfmt::DbiFmtRgb666),
            7 => Some(Dbicolorfmt::DbiFmtRgb888),
            _ => None,
        }
    }
    #[doc = "RGB111 (3 bits/pixel)"]
    #[inline(always)]
    pub fn is_dbi_fmt_rgb111(&self) -> bool {
        *self == Dbicolorfmt::DbiFmtRgb111
    }
    #[doc = "RGB332 (8 bits/pixel)"]
    #[inline(always)]
    pub fn is_dbi_fmt_rgb332(&self) -> bool {
        *self == Dbicolorfmt::DbiFmtRgb332
    }
    #[doc = "RGB444 (12 bits/pixel)"]
    #[inline(always)]
    pub fn is_dbi_fmt_rgb444(&self) -> bool {
        *self == Dbicolorfmt::DbiFmtRgb444
    }
    #[doc = "RGB565 (16 bits/pixel)"]
    #[inline(always)]
    pub fn is_dbi_fmt_rgb565(&self) -> bool {
        *self == Dbicolorfmt::DbiFmtRgb565
    }
    #[doc = "RGB666 (18 bits/pixel)"]
    #[inline(always)]
    pub fn is_dbi_fmt_rgb666(&self) -> bool {
        *self == Dbicolorfmt::DbiFmtRgb666
    }
    #[doc = "RGB888 (24 bits/pixel)"]
    #[inline(always)]
    pub fn is_dbi_fmt_rgb888(&self) -> bool {
        *self == Dbicolorfmt::DbiFmtRgb888
    }
}
#[doc = "Field `DBICOLORFMT` writer - Set the color format for DBI interface"]
pub type DbicolorfmtW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dbicolorfmt>;
impl<'a, REG> DbicolorfmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RGB111 (3 bits/pixel)"]
    #[inline(always)]
    pub fn dbi_fmt_rgb111(self) -> &'a mut crate::W<REG> {
        self.variant(Dbicolorfmt::DbiFmtRgb111)
    }
    #[doc = "RGB332 (8 bits/pixel)"]
    #[inline(always)]
    pub fn dbi_fmt_rgb332(self) -> &'a mut crate::W<REG> {
        self.variant(Dbicolorfmt::DbiFmtRgb332)
    }
    #[doc = "RGB444 (12 bits/pixel)"]
    #[inline(always)]
    pub fn dbi_fmt_rgb444(self) -> &'a mut crate::W<REG> {
        self.variant(Dbicolorfmt::DbiFmtRgb444)
    }
    #[doc = "RGB565 (16 bits/pixel)"]
    #[inline(always)]
    pub fn dbi_fmt_rgb565(self) -> &'a mut crate::W<REG> {
        self.variant(Dbicolorfmt::DbiFmtRgb565)
    }
    #[doc = "RGB666 (18 bits/pixel)"]
    #[inline(always)]
    pub fn dbi_fmt_rgb666(self) -> &'a mut crate::W<REG> {
        self.variant(Dbicolorfmt::DbiFmtRgb666)
    }
    #[doc = "RGB888 (24 bits/pixel)"]
    #[inline(always)]
    pub fn dbi_fmt_rgb888(self) -> &'a mut crate::W<REG> {
        self.variant(Dbicolorfmt::DbiFmtRgb888)
    }
}
#[doc = "Set the data order of the 8-bit data word:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datawdorder {
    #[doc = "0: option 0"]
    WdOrderOpt0 = 0,
    #[doc = "1: option 1"]
    WdOrderOpt1 = 1,
    #[doc = "2: option 2"]
    WdOrderOpt2 = 2,
    #[doc = "3: option 3"]
    WdOrderOpt3 = 3,
}
impl From<Datawdorder> for u8 {
    #[inline(always)]
    fn from(variant: Datawdorder) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Datawdorder {
    type Ux = u8;
}
impl crate::IsEnum for Datawdorder {}
#[doc = "Field `DATAWDORDER` reader - Set the data order of the 8-bit data word:"]
pub type DatawdorderR = crate::FieldReader<Datawdorder>;
impl DatawdorderR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Datawdorder> {
        match self.bits {
            0 => Some(Datawdorder::WdOrderOpt0),
            1 => Some(Datawdorder::WdOrderOpt1),
            2 => Some(Datawdorder::WdOrderOpt2),
            3 => Some(Datawdorder::WdOrderOpt3),
            _ => None,
        }
    }
    #[doc = "option 0"]
    #[inline(always)]
    pub fn is_wd_order_opt0(&self) -> bool {
        *self == Datawdorder::WdOrderOpt0
    }
    #[doc = "option 1"]
    #[inline(always)]
    pub fn is_wd_order_opt1(&self) -> bool {
        *self == Datawdorder::WdOrderOpt1
    }
    #[doc = "option 2"]
    #[inline(always)]
    pub fn is_wd_order_opt2(&self) -> bool {
        *self == Datawdorder::WdOrderOpt2
    }
    #[doc = "option 3"]
    #[inline(always)]
    pub fn is_wd_order_opt3(&self) -> bool {
        *self == Datawdorder::WdOrderOpt3
    }
}
#[doc = "Field `DATAWDORDER` writer - Set the data order of the 8-bit data word:"]
pub type DatawdorderW<'a, REG> = crate::FieldWriter<'a, REG, 3, Datawdorder>;
impl<'a, REG> DatawdorderW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "option 0"]
    #[inline(always)]
    pub fn wd_order_opt0(self) -> &'a mut crate::W<REG> {
        self.variant(Datawdorder::WdOrderOpt0)
    }
    #[doc = "option 1"]
    #[inline(always)]
    pub fn wd_order_opt1(self) -> &'a mut crate::W<REG> {
        self.variant(Datawdorder::WdOrderOpt1)
    }
    #[doc = "option 2"]
    #[inline(always)]
    pub fn wd_order_opt2(self) -> &'a mut crate::W<REG> {
        self.variant(Datawdorder::WdOrderOpt2)
    }
    #[doc = "option 3"]
    #[inline(always)]
    pub fn wd_order_opt3(self) -> &'a mut crate::W<REG> {
        self.variant(Datawdorder::WdOrderOpt3)
    }
}
#[doc = "Set DBI Type-B interface width (8, 9 or 16 bits) and the serial interface:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Typebwidth {
    #[doc = "0: 16-bit interface"]
    Int16 = 0,
    #[doc = "1: 9-bit interface"]
    Int9 = 1,
    #[doc = "2: 8-bit interface"]
    Int8 = 2,
    #[doc = "3: Serial interface"]
    IntSerial = 3,
}
impl From<Typebwidth> for u8 {
    #[inline(always)]
    fn from(variant: Typebwidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Typebwidth {
    type Ux = u8;
}
impl crate::IsEnum for Typebwidth {}
#[doc = "Field `TYPEBWIDTH` reader - Set DBI Type-B interface width (8, 9 or 16 bits) and the serial interface:"]
pub type TypebwidthR = crate::FieldReader<Typebwidth>;
impl TypebwidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Typebwidth {
        match self.bits {
            0 => Typebwidth::Int16,
            1 => Typebwidth::Int9,
            2 => Typebwidth::Int8,
            3 => Typebwidth::IntSerial,
            _ => unreachable!(),
        }
    }
    #[doc = "16-bit interface"]
    #[inline(always)]
    pub fn is_int_16(&self) -> bool {
        *self == Typebwidth::Int16
    }
    #[doc = "9-bit interface"]
    #[inline(always)]
    pub fn is_int_9(&self) -> bool {
        *self == Typebwidth::Int9
    }
    #[doc = "8-bit interface"]
    #[inline(always)]
    pub fn is_int_8(&self) -> bool {
        *self == Typebwidth::Int8
    }
    #[doc = "Serial interface"]
    #[inline(always)]
    pub fn is_int_serial(&self) -> bool {
        *self == Typebwidth::IntSerial
    }
}
#[doc = "Field `TYPEBWIDTH` writer - Set DBI Type-B interface width (8, 9 or 16 bits) and the serial interface:"]
pub type TypebwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, Typebwidth, crate::Safe>;
impl<'a, REG> TypebwidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16-bit interface"]
    #[inline(always)]
    pub fn int_16(self) -> &'a mut crate::W<REG> {
        self.variant(Typebwidth::Int16)
    }
    #[doc = "9-bit interface"]
    #[inline(always)]
    pub fn int_9(self) -> &'a mut crate::W<REG> {
        self.variant(Typebwidth::Int9)
    }
    #[doc = "8-bit interface"]
    #[inline(always)]
    pub fn int_8(self) -> &'a mut crate::W<REG> {
        self.variant(Typebwidth::Int8)
    }
    #[doc = "Serial interface"]
    #[inline(always)]
    pub fn int_serial(self) -> &'a mut crate::W<REG> {
        self.variant(Typebwidth::IntSerial)
    }
}
#[doc = "Field `RSVD0` reader - This field is reserved."]
pub type Rsvd0R = crate::FieldReader;
#[doc = "Field `RSVD0` writer - This field is reserved."]
pub type Rsvd0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BACKPRESSUREEN` reader - When set to 1, back pressure support is enabled (not currently supported)"]
pub type BackpressureenR = crate::BitReader;
#[doc = "Field `BACKPRESSUREEN` writer - When set to 1, back pressure support is enabled (not currently supported)"]
pub type BackpressureenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD1` reader - This field is reserved."]
pub type Rsvd1R = crate::FieldReader;
#[doc = "Field `RSVD1` writer - This field is reserved."]
pub type Rsvd1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INVHRZLINE` reader - When set to 1, inverts the bit-order of the horizontal line address (used along with DBI_CFG\\[17\\]
register bit)"]
pub type InvhrzlineR = crate::BitReader;
#[doc = "Field `INVHRZLINE` writer - When set to 1, inverts the bit-order of the horizontal line address (used along with DBI_CFG\\[17\\]
register bit)"]
pub type InvhrzlineW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BINDCMDS` reader - When set to 1, binds the store commands with the RGB data and two-byte address is sent with each horizontal line"]
pub type BindcmdsR = crate::BitReader;
#[doc = "Field `BINDCMDS` writer - When set to 1, binds the store commands with the RGB data and two-byte address is sent with each horizontal line"]
pub type BindcmdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD2` reader - This field is reserved."]
pub type Rsvd2R = crate::FieldReader;
#[doc = "Field `RSVD2` writer - This field is reserved."]
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SPI4` reader - When set to 1, SPI 4-wire interface is enabled"]
pub type Spi4R = crate::BitReader;
#[doc = "Field `SPI4` writer - When set to 1, SPI 4-wire interface is enabled"]
pub type Spi4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3` reader - When set to 1, SPI 3-wire interface is enabled"]
pub type Spi3R = crate::BitReader;
#[doc = "Field `SPI3` writer - When set to 1, SPI 3-wire interface is enabled"]
pub type Spi3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD3` reader - This field is reserved."]
pub type Rsvd3R = crate::BitReader;
#[doc = "Field `RSVD3` writer - This field is reserved."]
pub type Rsvd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESXLOW` reader - When set to 1, drives RESX signal low to reset DBI Type-B interface"]
pub type ResxlowR = crate::BitReader;
#[doc = "Field `RESXLOW` writer - When set to 1, drives RESX signal low to reset DBI Type-B interface"]
pub type ResxlowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD4` reader - This field is reserved."]
pub type Rsvd4R = crate::FieldReader;
#[doc = "Field `RSVD4` writer - This field is reserved."]
pub type Rsvd4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DBIBTEDIS` reader - When set to 1, the DBIB_TE signal is disabled"]
pub type DbibtedisR = crate::BitReader;
#[doc = "Field `DBIBTEDIS` writer - When set to 1, the DBIB_TE signal is disabled"]
pub type DbibtedisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Sets the value of DBIB_CSX signal:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csxset {
    #[doc = "1: is set to one if DBI_CFG\\[29\\]
has the value of one"]
    Csx1 = 1,
    #[doc = "0: is set to zero if DBI_CFG\\[29\\]
has the value of zero"]
    Csx0 = 0,
}
impl From<Csxset> for bool {
    #[inline(always)]
    fn from(variant: Csxset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSXSET` reader - Sets the value of DBIB_CSX signal:"]
pub type CsxsetR = crate::BitReader<Csxset>;
impl CsxsetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csxset {
        match self.bits {
            true => Csxset::Csx1,
            false => Csxset::Csx0,
        }
    }
    #[doc = "is set to one if DBI_CFG\\[29\\]
has the value of one"]
    #[inline(always)]
    pub fn is_csx1(&self) -> bool {
        *self == Csxset::Csx1
    }
    #[doc = "is set to zero if DBI_CFG\\[29\\]
has the value of zero"]
    #[inline(always)]
    pub fn is_csx0(&self) -> bool {
        *self == Csxset::Csx0
    }
}
#[doc = "Field `CSXSET` writer - Sets the value of DBIB_CSX signal:"]
pub type CsxsetW<'a, REG> = crate::BitWriter<'a, REG, Csxset>;
impl<'a, REG> CsxsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "is set to one if DBI_CFG\\[29\\]
has the value of one"]
    #[inline(always)]
    pub fn csx1(self) -> &'a mut crate::W<REG> {
        self.variant(Csxset::Csx1)
    }
    #[doc = "is set to zero if DBI_CFG\\[29\\]
has the value of zero"]
    #[inline(always)]
    pub fn csx0(self) -> &'a mut crate::W<REG> {
        self.variant(Csxset::Csx0)
    }
}
#[doc = "Field `CSXCFG` reader - When set to 1, the value of the CSX signal of the DBI interface can be configured from the DBI_CFG\\[29\\]
register bit"]
pub type CsxcfgR = crate::BitReader;
#[doc = "Field `CSXCFG` writer - When set to 1, the value of the CSX signal of the DBI interface can be configured from the DBI_CFG\\[29\\]
register bit"]
pub type CsxcfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBIINTACT` reader - When set to 1, the DBI interface is activated"]
pub type DbiintactR = crate::BitReader;
#[doc = "Field `DBIINTACT` writer - When set to 1, the DBI interface is activated"]
pub type DbiintactW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Set the color format for DBI interface"]
    #[inline(always)]
    pub fn dbicolorfmt(&self) -> DbicolorfmtR {
        DbicolorfmtR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Set the data order of the 8-bit data word:"]
    #[inline(always)]
    pub fn datawdorder(&self) -> DatawdorderR {
        DatawdorderR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Set DBI Type-B interface width (8, 9 or 16 bits) and the serial interface:"]
    #[inline(always)]
    pub fn typebwidth(&self) -> TypebwidthR {
        TypebwidthR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd0(&self) -> Rsvd0R {
        Rsvd0R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - When set to 1, back pressure support is enabled (not currently supported)"]
    #[inline(always)]
    pub fn backpressureen(&self) -> BackpressureenR {
        BackpressureenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd1(&self) -> Rsvd1R {
        Rsvd1R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - When set to 1, inverts the bit-order of the horizontal line address (used along with DBI_CFG\\[17\\]
register bit)"]
    #[inline(always)]
    pub fn invhrzline(&self) -> InvhrzlineR {
        InvhrzlineR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When set to 1, binds the store commands with the RGB data and two-byte address is sent with each horizontal line"]
    #[inline(always)]
    pub fn bindcmds(&self) -> BindcmdsR {
        BindcmdsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - When set to 1, SPI 4-wire interface is enabled"]
    #[inline(always)]
    pub fn spi4(&self) -> Spi4R {
        Spi4R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - When set to 1, SPI 3-wire interface is enabled"]
    #[inline(always)]
    pub fn spi3(&self) -> Spi3R {
        Spi3R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - When set to 1, drives RESX signal low to reset DBI Type-B interface"]
    #[inline(always)]
    pub fn resxlow(&self) -> ResxlowR {
        ResxlowR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd4(&self) -> Rsvd4R {
        Rsvd4R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - When set to 1, the DBIB_TE signal is disabled"]
    #[inline(always)]
    pub fn dbibtedis(&self) -> DbibtedisR {
        DbibtedisR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Sets the value of DBIB_CSX signal:"]
    #[inline(always)]
    pub fn csxset(&self) -> CsxsetR {
        CsxsetR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When set to 1, the value of the CSX signal of the DBI interface can be configured from the DBI_CFG\\[29\\]
register bit"]
    #[inline(always)]
    pub fn csxcfg(&self) -> CsxcfgR {
        CsxcfgR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When set to 1, the DBI interface is activated"]
    #[inline(always)]
    pub fn dbiintact(&self) -> DbiintactR {
        DbiintactR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Set the color format for DBI interface"]
    #[inline(always)]
    #[must_use]
    pub fn dbicolorfmt(&mut self) -> DbicolorfmtW<DbicfgSpec> {
        DbicolorfmtW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Set the data order of the 8-bit data word:"]
    #[inline(always)]
    #[must_use]
    pub fn datawdorder(&mut self) -> DatawdorderW<DbicfgSpec> {
        DatawdorderW::new(self, 3)
    }
    #[doc = "Bits 6:7 - Set DBI Type-B interface width (8, 9 or 16 bits) and the serial interface:"]
    #[inline(always)]
    #[must_use]
    pub fn typebwidth(&mut self) -> TypebwidthW<DbicfgSpec> {
        TypebwidthW::new(self, 6)
    }
    #[doc = "Bits 8:10 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd0(&mut self) -> Rsvd0W<DbicfgSpec> {
        Rsvd0W::new(self, 8)
    }
    #[doc = "Bit 11 - When set to 1, back pressure support is enabled (not currently supported)"]
    #[inline(always)]
    #[must_use]
    pub fn backpressureen(&mut self) -> BackpressureenW<DbicfgSpec> {
        BackpressureenW::new(self, 11)
    }
    #[doc = "Bits 12:15 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd1(&mut self) -> Rsvd1W<DbicfgSpec> {
        Rsvd1W::new(self, 12)
    }
    #[doc = "Bit 16 - When set to 1, inverts the bit-order of the horizontal line address (used along with DBI_CFG\\[17\\]
register bit)"]
    #[inline(always)]
    #[must_use]
    pub fn invhrzline(&mut self) -> InvhrzlineW<DbicfgSpec> {
        InvhrzlineW::new(self, 16)
    }
    #[doc = "Bit 17 - When set to 1, binds the store commands with the RGB data and two-byte address is sent with each horizontal line"]
    #[inline(always)]
    #[must_use]
    pub fn bindcmds(&mut self) -> BindcmdsW<DbicfgSpec> {
        BindcmdsW::new(self, 17)
    }
    #[doc = "Bits 18:21 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd2(&mut self) -> Rsvd2W<DbicfgSpec> {
        Rsvd2W::new(self, 18)
    }
    #[doc = "Bit 22 - When set to 1, SPI 4-wire interface is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn spi4(&mut self) -> Spi4W<DbicfgSpec> {
        Spi4W::new(self, 22)
    }
    #[doc = "Bit 23 - When set to 1, SPI 3-wire interface is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn spi3(&mut self) -> Spi3W<DbicfgSpec> {
        Spi3W::new(self, 23)
    }
    #[doc = "Bit 24 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd3(&mut self) -> Rsvd3W<DbicfgSpec> {
        Rsvd3W::new(self, 24)
    }
    #[doc = "Bit 25 - When set to 1, drives RESX signal low to reset DBI Type-B interface"]
    #[inline(always)]
    #[must_use]
    pub fn resxlow(&mut self) -> ResxlowW<DbicfgSpec> {
        ResxlowW::new(self, 25)
    }
    #[doc = "Bits 26:27 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd4(&mut self) -> Rsvd4W<DbicfgSpec> {
        Rsvd4W::new(self, 26)
    }
    #[doc = "Bit 28 - When set to 1, the DBIB_TE signal is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn dbibtedis(&mut self) -> DbibtedisW<DbicfgSpec> {
        DbibtedisW::new(self, 28)
    }
    #[doc = "Bit 29 - Sets the value of DBIB_CSX signal:"]
    #[inline(always)]
    #[must_use]
    pub fn csxset(&mut self) -> CsxsetW<DbicfgSpec> {
        CsxsetW::new(self, 29)
    }
    #[doc = "Bit 30 - When set to 1, the value of the CSX signal of the DBI interface can be configured from the DBI_CFG\\[29\\]
register bit"]
    #[inline(always)]
    #[must_use]
    pub fn csxcfg(&mut self) -> CsxcfgW<DbicfgSpec> {
        CsxcfgW::new(self, 30)
    }
    #[doc = "Bit 31 - When set to 1, the DBI interface is activated"]
    #[inline(always)]
    #[must_use]
    pub fn dbiintact(&mut self) -> DbiintactW<DbicfgSpec> {
        DbiintactW::new(self, 31)
    }
}
#[doc = "Register for the configuration DBI Type-B interface and the activation of SPI 3-/4-wire interfaces.\n\nYou can [`read`](crate::Reg::read) this register and get [`dbicfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbicfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbicfgSpec;
impl crate::RegisterSpec for DbicfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbicfg::R`](R) reader structure"]
impl crate::Readable for DbicfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dbicfg::W`](W) writer structure"]
impl crate::Writable for DbicfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBICFG to value 0"]
impl crate::Resettable for DbicfgSpec {
    const RESET_VALUE: u32 = 0;
}
