#[doc = "Register `CHIPPN` reader"]
pub type R = crate::R<ChippnSpec>;
#[doc = "Register `CHIPPN` writer"]
pub type W = crate::W<ChippnSpec>;
#[doc = "Temperature.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Temp {
    #[doc = "0: Commercial temperature"]
    Com = 0,
    #[doc = "1: Military temperature"]
    Mil = 1,
    #[doc = "2: Automotive temperature"]
    Auto = 2,
    #[doc = "3: Industrial temperature"]
    Ind = 3,
}
impl From<Temp> for u8 {
    #[inline(always)]
    fn from(variant: Temp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Temp {
    type Ux = u8;
}
impl crate::IsEnum for Temp {}
#[doc = "Field `TEMP` reader - Temperature."]
pub type TempR = crate::FieldReader<Temp>;
impl TempR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Temp {
        match self.bits {
            0 => Temp::Com,
            1 => Temp::Mil,
            2 => Temp::Auto,
            3 => Temp::Ind,
            _ => unreachable!(),
        }
    }
    #[doc = "Commercial temperature"]
    #[inline(always)]
    pub fn is_com(&self) -> bool {
        *self == Temp::Com
    }
    #[doc = "Military temperature"]
    #[inline(always)]
    pub fn is_mil(&self) -> bool {
        *self == Temp::Mil
    }
    #[doc = "Automotive temperature"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == Temp::Auto
    }
    #[doc = "Industrial temperature"]
    #[inline(always)]
    pub fn is_ind(&self) -> bool {
        *self == Temp::Ind
    }
}
#[doc = "Field `TEMP` writer - Temperature."]
pub type TempW<'a, REG> = crate::FieldWriter<'a, REG, 2, Temp, crate::Safe>;
impl<'a, REG> TempW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Commercial temperature"]
    #[inline(always)]
    pub fn com(self) -> &'a mut crate::W<REG> {
        self.variant(Temp::Com)
    }
    #[doc = "Military temperature"]
    #[inline(always)]
    pub fn mil(self) -> &'a mut crate::W<REG> {
        self.variant(Temp::Mil)
    }
    #[doc = "Automotive temperature"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(Temp::Auto)
    }
    #[doc = "Industrial temperature"]
    #[inline(always)]
    pub fn ind(self) -> &'a mut crate::W<REG> {
        self.variant(Temp::Ind)
    }
}
#[doc = "Number of pins for this package.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pins {
    #[doc = "0: 25 pins"]
    _25 = 0,
    #[doc = "1: 49 pins"]
    _49 = 1,
    #[doc = "2: 64 pins"]
    _64 = 2,
    #[doc = "3: 81 pins"]
    _81 = 3,
}
impl From<Pins> for u8 {
    #[inline(always)]
    fn from(variant: Pins) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pins {
    type Ux = u8;
}
impl crate::IsEnum for Pins {}
#[doc = "Field `PINS` reader - Number of pins for this package."]
pub type PinsR = crate::FieldReader<Pins>;
impl PinsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pins> {
        match self.bits {
            0 => Some(Pins::_25),
            1 => Some(Pins::_49),
            2 => Some(Pins::_64),
            3 => Some(Pins::_81),
            _ => None,
        }
    }
    #[doc = "25 pins"]
    #[inline(always)]
    pub fn is_25(&self) -> bool {
        *self == Pins::_25
    }
    #[doc = "49 pins"]
    #[inline(always)]
    pub fn is_49(&self) -> bool {
        *self == Pins::_49
    }
    #[doc = "64 pins"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Pins::_64
    }
    #[doc = "81 pins"]
    #[inline(always)]
    pub fn is_81(&self) -> bool {
        *self == Pins::_81
    }
}
#[doc = "Field `PINS` writer - Number of pins for this package."]
pub type PinsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pins>;
impl<'a, REG> PinsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "25 pins"]
    #[inline(always)]
    pub fn _25(self) -> &'a mut crate::W<REG> {
        self.variant(Pins::_25)
    }
    #[doc = "49 pins"]
    #[inline(always)]
    pub fn _49(self) -> &'a mut crate::W<REG> {
        self.variant(Pins::_49)
    }
    #[doc = "64 pins"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Pins::_64)
    }
    #[doc = "81 pins"]
    #[inline(always)]
    pub fn _81(self) -> &'a mut crate::W<REG> {
        self.variant(Pins::_81)
    }
}
#[doc = "Package type.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pkg {
    #[doc = "0: SIP package."]
    Sip = 0,
    #[doc = "1: SIP2 package."]
    Sip2 = 1,
    #[doc = "2: BGA package."]
    Bga = 2,
    #[doc = "3: CSP package."]
    Csp = 3,
}
impl From<Pkg> for u8 {
    #[inline(always)]
    fn from(variant: Pkg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pkg {
    type Ux = u8;
}
impl crate::IsEnum for Pkg {}
#[doc = "Field `PKG` reader - Package type."]
pub type PkgR = crate::FieldReader<Pkg>;
impl PkgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pkg {
        match self.bits {
            0 => Pkg::Sip,
            1 => Pkg::Sip2,
            2 => Pkg::Bga,
            3 => Pkg::Csp,
            _ => unreachable!(),
        }
    }
    #[doc = "SIP package."]
    #[inline(always)]
    pub fn is_sip(&self) -> bool {
        *self == Pkg::Sip
    }
    #[doc = "SIP2 package."]
    #[inline(always)]
    pub fn is_sip2(&self) -> bool {
        *self == Pkg::Sip2
    }
    #[doc = "BGA package."]
    #[inline(always)]
    pub fn is_bga(&self) -> bool {
        *self == Pkg::Bga
    }
    #[doc = "CSP package."]
    #[inline(always)]
    pub fn is_csp(&self) -> bool {
        *self == Pkg::Csp
    }
}
#[doc = "Field `PKG` writer - Package type."]
pub type PkgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pkg, crate::Safe>;
impl<'a, REG> PkgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SIP package."]
    #[inline(always)]
    pub fn sip(self) -> &'a mut crate::W<REG> {
        self.variant(Pkg::Sip)
    }
    #[doc = "SIP2 package."]
    #[inline(always)]
    pub fn sip2(self) -> &'a mut crate::W<REG> {
        self.variant(Pkg::Sip2)
    }
    #[doc = "BGA package."]
    #[inline(always)]
    pub fn bga(self) -> &'a mut crate::W<REG> {
        self.variant(Pkg::Bga)
    }
    #[doc = "CSP package."]
    #[inline(always)]
    pub fn csp(self) -> &'a mut crate::W<REG> {
        self.variant(Pkg::Csp)
    }
}
#[doc = "Minor revision.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Revmin {
    #[doc = "0: Minor Rev 0"]
    _0 = 0,
    #[doc = "1: Minor Rev 1"]
    _1 = 1,
}
impl From<Revmin> for u8 {
    #[inline(always)]
    fn from(variant: Revmin) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Revmin {
    type Ux = u8;
}
impl crate::IsEnum for Revmin {}
#[doc = "Field `REVMIN` reader - Minor revision."]
pub type RevminR = crate::FieldReader<Revmin>;
impl RevminR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Revmin> {
        match self.bits {
            0 => Some(Revmin::_0),
            1 => Some(Revmin::_1),
            _ => None,
        }
    }
    #[doc = "Minor Rev 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Revmin::_0
    }
    #[doc = "Minor Rev 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Revmin::_1
    }
}
#[doc = "Field `REVMIN` writer - Minor revision."]
pub type RevminW<'a, REG> = crate::FieldWriter<'a, REG, 4, Revmin>;
impl<'a, REG> RevminW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Minor Rev 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Revmin::_0)
    }
    #[doc = "Minor Rev 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Revmin::_1)
    }
}
#[doc = "Major revision.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Revmaj {
    #[doc = "0: Major Rev A"]
    A = 0,
    #[doc = "1: Major Rev B"]
    B = 1,
    #[doc = "2: Major Rev C"]
    C = 2,
}
impl From<Revmaj> for u8 {
    #[inline(always)]
    fn from(variant: Revmaj) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Revmaj {
    type Ux = u8;
}
impl crate::IsEnum for Revmaj {}
#[doc = "Field `REVMAJ` reader - Major revision."]
pub type RevmajR = crate::FieldReader<Revmaj>;
impl RevmajR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Revmaj> {
        match self.bits {
            0 => Some(Revmaj::A),
            1 => Some(Revmaj::B),
            2 => Some(Revmaj::C),
            _ => None,
        }
    }
    #[doc = "Major Rev A"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == Revmaj::A
    }
    #[doc = "Major Rev B"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == Revmaj::B
    }
    #[doc = "Major Rev C"]
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        *self == Revmaj::C
    }
}
#[doc = "Field `REVMAJ` writer - Major revision."]
pub type RevmajW<'a, REG> = crate::FieldWriter<'a, REG, 4, Revmaj>;
impl<'a, REG> RevmajW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Major Rev A"]
    #[inline(always)]
    pub fn a(self) -> &'a mut crate::W<REG> {
        self.variant(Revmaj::A)
    }
    #[doc = "Major Rev B"]
    #[inline(always)]
    pub fn b(self) -> &'a mut crate::W<REG> {
        self.variant(Revmaj::B)
    }
    #[doc = "Major Rev C"]
    #[inline(always)]
    pub fn c(self) -> &'a mut crate::W<REG> {
        self.variant(Revmaj::C)
    }
}
#[doc = "SRAM size.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sramsize {
    #[doc = "0: SRAM size is 384KB+512KB"]
    _384_512 = 0,
    #[doc = "1: SRAM size is 384KB+1MB"]
    _384_1024 = 1,
    #[doc = "2: SRAM size is 384KB+1MB+384KB+96KB"]
    _384_1024_384_96 = 2,
}
impl From<Sramsize> for u8 {
    #[inline(always)]
    fn from(variant: Sramsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sramsize {
    type Ux = u8;
}
impl crate::IsEnum for Sramsize {}
#[doc = "Field `SRAMSIZE` reader - SRAM size."]
pub type SramsizeR = crate::FieldReader<Sramsize>;
impl SramsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sramsize> {
        match self.bits {
            0 => Some(Sramsize::_384_512),
            1 => Some(Sramsize::_384_1024),
            2 => Some(Sramsize::_384_1024_384_96),
            _ => None,
        }
    }
    #[doc = "SRAM size is 384KB+512KB"]
    #[inline(always)]
    pub fn is_384_512(&self) -> bool {
        *self == Sramsize::_384_512
    }
    #[doc = "SRAM size is 384KB+1MB"]
    #[inline(always)]
    pub fn is_384_1024(&self) -> bool {
        *self == Sramsize::_384_1024
    }
    #[doc = "SRAM size is 384KB+1MB+384KB+96KB"]
    #[inline(always)]
    pub fn is_384_1024_384_96(&self) -> bool {
        *self == Sramsize::_384_1024_384_96
    }
}
#[doc = "Field `SRAMSIZE` writer - SRAM size."]
pub type SramsizeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Sramsize>;
impl<'a, REG> SramsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SRAM size is 384KB+512KB"]
    #[inline(always)]
    pub fn _384_512(self) -> &'a mut crate::W<REG> {
        self.variant(Sramsize::_384_512)
    }
    #[doc = "SRAM size is 384KB+1MB"]
    #[inline(always)]
    pub fn _384_1024(self) -> &'a mut crate::W<REG> {
        self.variant(Sramsize::_384_1024)
    }
    #[doc = "SRAM size is 384KB+1MB+384KB+96KB"]
    #[inline(always)]
    pub fn _384_1024_384_96(self) -> &'a mut crate::W<REG> {
        self.variant(Sramsize::_384_1024_384_96)
    }
}
#[doc = "MRAM size.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mramsize {
    #[doc = "0: MRAM size is 0.5MB"]
    _0p5mb = 0,
    #[doc = "1: MRAM size is 1.0MB"]
    _1p0mb = 1,
    #[doc = "2: MRAM size is 1.5MB"]
    _1p5mb = 2,
    #[doc = "3: MRAM size is 2.0MB"]
    _2p0mb = 3,
}
impl From<Mramsize> for u8 {
    #[inline(always)]
    fn from(variant: Mramsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mramsize {
    type Ux = u8;
}
impl crate::IsEnum for Mramsize {}
#[doc = "Field `MRAMSIZE` reader - MRAM size."]
pub type MramsizeR = crate::FieldReader<Mramsize>;
impl MramsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mramsize> {
        match self.bits {
            0 => Some(Mramsize::_0p5mb),
            1 => Some(Mramsize::_1p0mb),
            2 => Some(Mramsize::_1p5mb),
            3 => Some(Mramsize::_2p0mb),
            _ => None,
        }
    }
    #[doc = "MRAM size is 0.5MB"]
    #[inline(always)]
    pub fn is_0p5mb(&self) -> bool {
        *self == Mramsize::_0p5mb
    }
    #[doc = "MRAM size is 1.0MB"]
    #[inline(always)]
    pub fn is_1p0mb(&self) -> bool {
        *self == Mramsize::_1p0mb
    }
    #[doc = "MRAM size is 1.5MB"]
    #[inline(always)]
    pub fn is_1p5mb(&self) -> bool {
        *self == Mramsize::_1p5mb
    }
    #[doc = "MRAM size is 2.0MB"]
    #[inline(always)]
    pub fn is_2p0mb(&self) -> bool {
        *self == Mramsize::_2p0mb
    }
}
#[doc = "Field `MRAMSIZE` writer - MRAM size."]
pub type MramsizeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Mramsize>;
impl<'a, REG> MramsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MRAM size is 0.5MB"]
    #[inline(always)]
    pub fn _0p5mb(self) -> &'a mut crate::W<REG> {
        self.variant(Mramsize::_0p5mb)
    }
    #[doc = "MRAM size is 1.0MB"]
    #[inline(always)]
    pub fn _1p0mb(self) -> &'a mut crate::W<REG> {
        self.variant(Mramsize::_1p0mb)
    }
    #[doc = "MRAM size is 1.5MB"]
    #[inline(always)]
    pub fn _1p5mb(self) -> &'a mut crate::W<REG> {
        self.variant(Mramsize::_1p5mb)
    }
    #[doc = "MRAM size is 2.0MB"]
    #[inline(always)]
    pub fn _2p0mb(self) -> &'a mut crate::W<REG> {
        self.variant(Mramsize::_2p0mb)
    }
}
#[doc = "Apollo family device type.\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pn {
    #[doc = "9: Apollo4L part number is 0x09xxxxxx."]
    Apollo4l = 9,
    #[doc = "8: Apollo4 part number is 0x08xxxxxx."]
    Apollo4 = 8,
    #[doc = "7: Apollo3P part number is 0x07xxxxxx."]
    Apollo3p = 7,
    #[doc = "6: Apollo3 part number is 0x06xxxxxx."]
    Apollo3 = 6,
    #[doc = "3: Apollo2 part number is 0x03xxxxxx."]
    Apollo2 = 3,
    #[doc = "1: Apollo part number is 0x01xxxxxx."]
    Apollo = 1,
}
impl From<Pn> for u8 {
    #[inline(always)]
    fn from(variant: Pn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pn {
    type Ux = u8;
}
impl crate::IsEnum for Pn {}
#[doc = "Field `PN` reader - Apollo family device type."]
pub type PnR = crate::FieldReader<Pn>;
impl PnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pn> {
        match self.bits {
            9 => Some(Pn::Apollo4l),
            8 => Some(Pn::Apollo4),
            7 => Some(Pn::Apollo3p),
            6 => Some(Pn::Apollo3),
            3 => Some(Pn::Apollo2),
            1 => Some(Pn::Apollo),
            _ => None,
        }
    }
    #[doc = "Apollo4L part number is 0x09xxxxxx."]
    #[inline(always)]
    pub fn is_apollo4l(&self) -> bool {
        *self == Pn::Apollo4l
    }
    #[doc = "Apollo4 part number is 0x08xxxxxx."]
    #[inline(always)]
    pub fn is_apollo4(&self) -> bool {
        *self == Pn::Apollo4
    }
    #[doc = "Apollo3P part number is 0x07xxxxxx."]
    #[inline(always)]
    pub fn is_apollo3p(&self) -> bool {
        *self == Pn::Apollo3p
    }
    #[doc = "Apollo3 part number is 0x06xxxxxx."]
    #[inline(always)]
    pub fn is_apollo3(&self) -> bool {
        *self == Pn::Apollo3
    }
    #[doc = "Apollo2 part number is 0x03xxxxxx."]
    #[inline(always)]
    pub fn is_apollo2(&self) -> bool {
        *self == Pn::Apollo2
    }
    #[doc = "Apollo part number is 0x01xxxxxx."]
    #[inline(always)]
    pub fn is_apollo(&self) -> bool {
        *self == Pn::Apollo
    }
}
#[doc = "Field `PN` writer - Apollo family device type."]
pub type PnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Pn>;
impl<'a, REG> PnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Apollo4L part number is 0x09xxxxxx."]
    #[inline(always)]
    pub fn apollo4l(self) -> &'a mut crate::W<REG> {
        self.variant(Pn::Apollo4l)
    }
    #[doc = "Apollo4 part number is 0x08xxxxxx."]
    #[inline(always)]
    pub fn apollo4(self) -> &'a mut crate::W<REG> {
        self.variant(Pn::Apollo4)
    }
    #[doc = "Apollo3P part number is 0x07xxxxxx."]
    #[inline(always)]
    pub fn apollo3p(self) -> &'a mut crate::W<REG> {
        self.variant(Pn::Apollo3p)
    }
    #[doc = "Apollo3 part number is 0x06xxxxxx."]
    #[inline(always)]
    pub fn apollo3(self) -> &'a mut crate::W<REG> {
        self.variant(Pn::Apollo3)
    }
    #[doc = "Apollo2 part number is 0x03xxxxxx."]
    #[inline(always)]
    pub fn apollo2(self) -> &'a mut crate::W<REG> {
        self.variant(Pn::Apollo2)
    }
    #[doc = "Apollo part number is 0x01xxxxxx."]
    #[inline(always)]
    pub fn apollo(self) -> &'a mut crate::W<REG> {
        self.variant(Pn::Apollo)
    }
}
impl R {
    #[doc = "Bits 1:2 - Temperature."]
    #[inline(always)]
    pub fn temp(&self) -> TempR {
        TempR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:5 - Number of pins for this package."]
    #[inline(always)]
    pub fn pins(&self) -> PinsR {
        PinsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Package type."]
    #[inline(always)]
    pub fn pkg(&self) -> PkgR {
        PkgR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Minor revision."]
    #[inline(always)]
    pub fn revmin(&self) -> RevminR {
        RevminR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Major revision."]
    #[inline(always)]
    pub fn revmaj(&self) -> RevmajR {
        RevmajR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - SRAM size."]
    #[inline(always)]
    pub fn sramsize(&self) -> SramsizeR {
        SramsizeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - MRAM size."]
    #[inline(always)]
    pub fn mramsize(&self) -> MramsizeR {
        MramsizeR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Apollo family device type."]
    #[inline(always)]
    pub fn pn(&self) -> PnR {
        PnR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 1:2 - Temperature."]
    #[inline(always)]
    #[must_use]
    pub fn temp(&mut self) -> TempW<ChippnSpec> {
        TempW::new(self, 1)
    }
    #[doc = "Bits 3:5 - Number of pins for this package."]
    #[inline(always)]
    #[must_use]
    pub fn pins(&mut self) -> PinsW<ChippnSpec> {
        PinsW::new(self, 3)
    }
    #[doc = "Bits 6:7 - Package type."]
    #[inline(always)]
    #[must_use]
    pub fn pkg(&mut self) -> PkgW<ChippnSpec> {
        PkgW::new(self, 6)
    }
    #[doc = "Bits 8:11 - Minor revision."]
    #[inline(always)]
    #[must_use]
    pub fn revmin(&mut self) -> RevminW<ChippnSpec> {
        RevminW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Major revision."]
    #[inline(always)]
    #[must_use]
    pub fn revmaj(&mut self) -> RevmajW<ChippnSpec> {
        RevmajW::new(self, 12)
    }
    #[doc = "Bits 16:19 - SRAM size."]
    #[inline(always)]
    #[must_use]
    pub fn sramsize(&mut self) -> SramsizeW<ChippnSpec> {
        SramsizeW::new(self, 16)
    }
    #[doc = "Bits 20:23 - MRAM size."]
    #[inline(always)]
    #[must_use]
    pub fn mramsize(&mut self) -> MramsizeW<ChippnSpec> {
        MramsizeW::new(self, 20)
    }
    #[doc = "Bits 24:31 - Apollo family device type."]
    #[inline(always)]
    #[must_use]
    pub fn pn(&mut self) -> PnW<ChippnSpec> {
        PnW::new(self, 24)
    }
}
#[doc = "Chip Information\n\nYou can [`read`](crate::Reg::read) this register and get [`chippn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chippn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChippnSpec;
impl crate::RegisterSpec for ChippnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chippn::R`](R) reader structure"]
impl crate::Readable for ChippnSpec {}
#[doc = "`write(|w| ..)` method takes [`chippn::W`](W) writer structure"]
impl crate::Writable for ChippnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHIPPN to value 0x0832_0088"]
impl crate::Resettable for ChippnSpec {
    const RESET_VALUE: u32 = 0x0832_0088;
}
