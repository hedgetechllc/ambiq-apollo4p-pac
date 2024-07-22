#[doc = "Register `I2SDATACFG` reader"]
pub type R = crate::R<I2sdatacfgSpec>;
#[doc = "Register `I2SDATACFG` writer"]
pub type W = crate::W<I2sdatacfgSpec>;
#[doc = "Receive audio sample length for phase 1. 0: 8b, 2: 16b, 4: 24b, 5: 32b, 1,3,6,7: Reserved\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ssz1 {
    #[doc = "0: Receive audio sample length is 8 bits for phase 1."]
    _8b = 0,
    #[doc = "2: Receive audio sample length is 16 bits for phase 1."]
    _16b = 2,
    #[doc = "4: Receive audio sample length is 24 bits for phase 1."]
    _24b = 4,
    #[doc = "5: Receive audio sample length is 32 bits for phase 1."]
    _32b = 5,
}
impl From<Ssz1> for u8 {
    #[inline(always)]
    fn from(variant: Ssz1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ssz1 {
    type Ux = u8;
}
impl crate::IsEnum for Ssz1 {}
#[doc = "Field `SSZ1` reader - Receive audio sample length for phase 1. 0: 8b, 2: 16b, 4: 24b, 5: 32b, 1,3,6,7: Reserved"]
pub type Ssz1R = crate::FieldReader<Ssz1>;
impl Ssz1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ssz1> {
        match self.bits {
            0 => Some(Ssz1::_8b),
            2 => Some(Ssz1::_16b),
            4 => Some(Ssz1::_24b),
            5 => Some(Ssz1::_32b),
            _ => None,
        }
    }
    #[doc = "Receive audio sample length is 8 bits for phase 1."]
    #[inline(always)]
    pub fn is_8b(&self) -> bool {
        *self == Ssz1::_8b
    }
    #[doc = "Receive audio sample length is 16 bits for phase 1."]
    #[inline(always)]
    pub fn is_16b(&self) -> bool {
        *self == Ssz1::_16b
    }
    #[doc = "Receive audio sample length is 24 bits for phase 1."]
    #[inline(always)]
    pub fn is_24b(&self) -> bool {
        *self == Ssz1::_24b
    }
    #[doc = "Receive audio sample length is 32 bits for phase 1."]
    #[inline(always)]
    pub fn is_32b(&self) -> bool {
        *self == Ssz1::_32b
    }
}
#[doc = "Field `SSZ1` writer - Receive audio sample length for phase 1. 0: 8b, 2: 16b, 4: 24b, 5: 32b, 1,3,6,7: Reserved"]
pub type Ssz1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Ssz1>;
impl<'a, REG> Ssz1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Receive audio sample length is 8 bits for phase 1."]
    #[inline(always)]
    pub fn _8b(self) -> &'a mut crate::W<REG> {
        self.variant(Ssz1::_8b)
    }
    #[doc = "Receive audio sample length is 16 bits for phase 1."]
    #[inline(always)]
    pub fn _16b(self) -> &'a mut crate::W<REG> {
        self.variant(Ssz1::_16b)
    }
    #[doc = "Receive audio sample length is 24 bits for phase 1."]
    #[inline(always)]
    pub fn _24b(self) -> &'a mut crate::W<REG> {
        self.variant(Ssz1::_24b)
    }
    #[doc = "Receive audio sample length is 32 bits for phase 1."]
    #[inline(always)]
    pub fn _32b(self) -> &'a mut crate::W<REG> {
        self.variant(Ssz1::_32b)
    }
}
#[doc = "Field `JUST` reader - Audio sample justification. 0: Left-justified, 1: Right-justified"]
pub type JustR = crate::BitReader;
#[doc = "Field `JUST` writer - Audio sample justification. 0: Left-justified, 1: Right-justified"]
pub type JustW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Receive channel length in bits for phase 1. 0: 8b, 2: 16b, 4: 24b, 5: 32b, 1,3,6,7: Reserved\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wdlen1 {
    #[doc = "0: Receive channel length is 8 bits for phase 1."]
    _8b = 0,
    #[doc = "2: Receive channel length is 16 bits for phase 1."]
    _16b = 2,
    #[doc = "4: Receive channel length is 24 bits for phase 1."]
    _24b = 4,
    #[doc = "5: Receive channel length is 32 bits for phase 1."]
    _32b = 5,
}
impl From<Wdlen1> for u8 {
    #[inline(always)]
    fn from(variant: Wdlen1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wdlen1 {
    type Ux = u8;
}
impl crate::IsEnum for Wdlen1 {}
#[doc = "Field `WDLEN1` reader - Receive channel length in bits for phase 1. 0: 8b, 2: 16b, 4: 24b, 5: 32b, 1,3,6,7: Reserved"]
pub type Wdlen1R = crate::FieldReader<Wdlen1>;
impl Wdlen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wdlen1> {
        match self.bits {
            0 => Some(Wdlen1::_8b),
            2 => Some(Wdlen1::_16b),
            4 => Some(Wdlen1::_24b),
            5 => Some(Wdlen1::_32b),
            _ => None,
        }
    }
    #[doc = "Receive channel length is 8 bits for phase 1."]
    #[inline(always)]
    pub fn is_8b(&self) -> bool {
        *self == Wdlen1::_8b
    }
    #[doc = "Receive channel length is 16 bits for phase 1."]
    #[inline(always)]
    pub fn is_16b(&self) -> bool {
        *self == Wdlen1::_16b
    }
    #[doc = "Receive channel length is 24 bits for phase 1."]
    #[inline(always)]
    pub fn is_24b(&self) -> bool {
        *self == Wdlen1::_24b
    }
    #[doc = "Receive channel length is 32 bits for phase 1."]
    #[inline(always)]
    pub fn is_32b(&self) -> bool {
        *self == Wdlen1::_32b
    }
}
#[doc = "Field `WDLEN1` writer - Receive channel length in bits for phase 1. 0: 8b, 2: 16b, 4: 24b, 5: 32b, 1,3,6,7: Reserved"]
pub type Wdlen1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Wdlen1>;
impl<'a, REG> Wdlen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Receive channel length is 8 bits for phase 1."]
    #[inline(always)]
    pub fn _8b(self) -> &'a mut crate::W<REG> {
        self.variant(Wdlen1::_8b)
    }
    #[doc = "Receive channel length is 16 bits for phase 1."]
    #[inline(always)]
    pub fn _16b(self) -> &'a mut crate::W<REG> {
        self.variant(Wdlen1::_16b)
    }
    #[doc = "Receive channel length is 24 bits for phase 1."]
    #[inline(always)]
    pub fn _24b(self) -> &'a mut crate::W<REG> {
        self.variant(Wdlen1::_24b)
    }
    #[doc = "Receive channel length is 32 bits for phase 1."]
    #[inline(always)]
    pub fn _32b(self) -> &'a mut crate::W<REG> {
        self.variant(Wdlen1::_32b)
    }
}
#[doc = "Number of channels in phase 1; 0: 1 Channel in phase 2, .. 0x7: 8 channels in phase 1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frlen1 {
    #[doc = "0: One channel in phase 1."]
    _1chls = 0,
    #[doc = "1: Two channels in phase 1."]
    _2chls = 1,
    #[doc = "2: Three channels in phase 1."]
    _3chls = 2,
    #[doc = "3: Four channels in phase 1."]
    _4chls = 3,
    #[doc = "4: Five channels in phase 1."]
    _5chls = 4,
    #[doc = "5: Six channels in phase 1."]
    _6chls = 5,
    #[doc = "6: Seven channels in phase 1."]
    _7chls = 6,
    #[doc = "7: Eight channels in phase 1."]
    _8chls = 7,
}
impl From<Frlen1> for u8 {
    #[inline(always)]
    fn from(variant: Frlen1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frlen1 {
    type Ux = u8;
}
impl crate::IsEnum for Frlen1 {}
#[doc = "Field `FRLEN1` reader - Number of channels in phase 1; 0: 1 Channel in phase 2, .. 0x7: 8 channels in phase 1"]
pub type Frlen1R = crate::FieldReader<Frlen1>;
impl Frlen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Frlen1> {
        match self.bits {
            0 => Some(Frlen1::_1chls),
            1 => Some(Frlen1::_2chls),
            2 => Some(Frlen1::_3chls),
            3 => Some(Frlen1::_4chls),
            4 => Some(Frlen1::_5chls),
            5 => Some(Frlen1::_6chls),
            6 => Some(Frlen1::_7chls),
            7 => Some(Frlen1::_8chls),
            _ => None,
        }
    }
    #[doc = "One channel in phase 1."]
    #[inline(always)]
    pub fn is_1chls(&self) -> bool {
        *self == Frlen1::_1chls
    }
    #[doc = "Two channels in phase 1."]
    #[inline(always)]
    pub fn is_2chls(&self) -> bool {
        *self == Frlen1::_2chls
    }
    #[doc = "Three channels in phase 1."]
    #[inline(always)]
    pub fn is_3chls(&self) -> bool {
        *self == Frlen1::_3chls
    }
    #[doc = "Four channels in phase 1."]
    #[inline(always)]
    pub fn is_4chls(&self) -> bool {
        *self == Frlen1::_4chls
    }
    #[doc = "Five channels in phase 1."]
    #[inline(always)]
    pub fn is_5chls(&self) -> bool {
        *self == Frlen1::_5chls
    }
    #[doc = "Six channels in phase 1."]
    #[inline(always)]
    pub fn is_6chls(&self) -> bool {
        *self == Frlen1::_6chls
    }
    #[doc = "Seven channels in phase 1."]
    #[inline(always)]
    pub fn is_7chls(&self) -> bool {
        *self == Frlen1::_7chls
    }
    #[doc = "Eight channels in phase 1."]
    #[inline(always)]
    pub fn is_8chls(&self) -> bool {
        *self == Frlen1::_8chls
    }
}
#[doc = "Field `FRLEN1` writer - Number of channels in phase 1; 0: 1 Channel in phase 2, .. 0x7: 8 channels in phase 1"]
pub type Frlen1W<'a, REG> = crate::FieldWriter<'a, REG, 7, Frlen1>;
impl<'a, REG> Frlen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One channel in phase 1."]
    #[inline(always)]
    pub fn _1chls(self) -> &'a mut crate::W<REG> {
        self.variant(Frlen1::_1chls)
    }
    #[doc = "Two channels in phase 1."]
    #[inline(always)]
    pub fn _2chls(self) -> &'a mut crate::W<REG> {
        self.variant(Frlen1::_2chls)
    }
    #[doc = "Three channels in phase 1."]
    #[inline(always)]
    pub fn _3chls(self) -> &'a mut crate::W<REG> {
        self.variant(Frlen1::_3chls)
    }
    #[doc = "Four channels in phase 1."]
    #[inline(always)]
    pub fn _4chls(self) -> &'a mut crate::W<REG> {
        self.variant(Frlen1::_4chls)
    }
    #[doc = "Five channels in phase 1."]
    #[inline(always)]
    pub fn _5chls(self) -> &'a mut crate::W<REG> {
        self.variant(Frlen1::_5chls)
    }
    #[doc = "Six channels in phase 1."]
    #[inline(always)]
    pub fn _6chls(self) -> &'a mut crate::W<REG> {
        self.variant(Frlen1::_6chls)
    }
    #[doc = "Seven channels in phase 1."]
    #[inline(always)]
    pub fn _7chls(self) -> &'a mut crate::W<REG> {
        self.variant(Frlen1::_7chls)
    }
    #[doc = "Eight channels in phase 1."]
    #[inline(always)]
    pub fn _8chls(self) -> &'a mut crate::W<REG> {
        self.variant(Frlen1::_8chls)
    }
}
#[doc = "Receive audio sample length for phase 2. 0: 8b, 2: 16b, 4: 24b, 5: 32b, 1,3,6,7: Reserved\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ssz2 {
    #[doc = "0: Receive audio sample length is 8 bits for phase 2."]
    _8b = 0,
    #[doc = "2: Receive audio sample length is 16 bits for phase 2."]
    _16b = 2,
    #[doc = "4: Receive audio sample length is 24 bits for phase 2."]
    _24b = 4,
    #[doc = "5: Receive audio sample length is 32 bits for phase 2."]
    _32b = 5,
}
impl From<Ssz2> for u8 {
    #[inline(always)]
    fn from(variant: Ssz2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ssz2 {
    type Ux = u8;
}
impl crate::IsEnum for Ssz2 {}
#[doc = "Field `SSZ2` reader - Receive audio sample length for phase 2. 0: 8b, 2: 16b, 4: 24b, 5: 32b, 1,3,6,7: Reserved"]
pub type Ssz2R = crate::FieldReader<Ssz2>;
impl Ssz2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ssz2> {
        match self.bits {
            0 => Some(Ssz2::_8b),
            2 => Some(Ssz2::_16b),
            4 => Some(Ssz2::_24b),
            5 => Some(Ssz2::_32b),
            _ => None,
        }
    }
    #[doc = "Receive audio sample length is 8 bits for phase 2."]
    #[inline(always)]
    pub fn is_8b(&self) -> bool {
        *self == Ssz2::_8b
    }
    #[doc = "Receive audio sample length is 16 bits for phase 2."]
    #[inline(always)]
    pub fn is_16b(&self) -> bool {
        *self == Ssz2::_16b
    }
    #[doc = "Receive audio sample length is 24 bits for phase 2."]
    #[inline(always)]
    pub fn is_24b(&self) -> bool {
        *self == Ssz2::_24b
    }
    #[doc = "Receive audio sample length is 32 bits for phase 2."]
    #[inline(always)]
    pub fn is_32b(&self) -> bool {
        *self == Ssz2::_32b
    }
}
#[doc = "Field `SSZ2` writer - Receive audio sample length for phase 2. 0: 8b, 2: 16b, 4: 24b, 5: 32b, 1,3,6,7: Reserved"]
pub type Ssz2W<'a, REG> = crate::FieldWriter<'a, REG, 3, Ssz2>;
impl<'a, REG> Ssz2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Receive audio sample length is 8 bits for phase 2."]
    #[inline(always)]
    pub fn _8b(self) -> &'a mut crate::W<REG> {
        self.variant(Ssz2::_8b)
    }
    #[doc = "Receive audio sample length is 16 bits for phase 2."]
    #[inline(always)]
    pub fn _16b(self) -> &'a mut crate::W<REG> {
        self.variant(Ssz2::_16b)
    }
    #[doc = "Receive audio sample length is 24 bits for phase 2."]
    #[inline(always)]
    pub fn _24b(self) -> &'a mut crate::W<REG> {
        self.variant(Ssz2::_24b)
    }
    #[doc = "Receive audio sample length is 32 bits for phase 2."]
    #[inline(always)]
    pub fn _32b(self) -> &'a mut crate::W<REG> {
        self.variant(Ssz2::_32b)
    }
}
#[doc = "Field `DATADLY` reader - Receive data delay bit count. Valid values are 0-2, 3 is reserved."]
pub type DatadlyR = crate::FieldReader;
#[doc = "Field `DATADLY` writer - Receive data delay bit count. Valid values are 0-2, 3 is reserved."]
pub type DatadlyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Receive channel length in bits for phase 2. 0: 8b, 2: 16b, 4: 24b, 5: 32b, 1,3,6,7: Reserved\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wdlen2 {
    #[doc = "0: Receive channel length is 8 bits for phase 2."]
    _8b = 0,
    #[doc = "2: Receive channel length is 16 bits for phase 2."]
    _16b = 2,
    #[doc = "4: Receive channel length is 24 bits for phase 2."]
    _24b = 4,
    #[doc = "5: Receive channel length is 32 bits for phase 2."]
    _32b = 5,
}
impl From<Wdlen2> for u8 {
    #[inline(always)]
    fn from(variant: Wdlen2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wdlen2 {
    type Ux = u8;
}
impl crate::IsEnum for Wdlen2 {}
#[doc = "Field `WDLEN2` reader - Receive channel length in bits for phase 2. 0: 8b, 2: 16b, 4: 24b, 5: 32b, 1,3,6,7: Reserved"]
pub type Wdlen2R = crate::FieldReader<Wdlen2>;
impl Wdlen2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wdlen2> {
        match self.bits {
            0 => Some(Wdlen2::_8b),
            2 => Some(Wdlen2::_16b),
            4 => Some(Wdlen2::_24b),
            5 => Some(Wdlen2::_32b),
            _ => None,
        }
    }
    #[doc = "Receive channel length is 8 bits for phase 2."]
    #[inline(always)]
    pub fn is_8b(&self) -> bool {
        *self == Wdlen2::_8b
    }
    #[doc = "Receive channel length is 16 bits for phase 2."]
    #[inline(always)]
    pub fn is_16b(&self) -> bool {
        *self == Wdlen2::_16b
    }
    #[doc = "Receive channel length is 24 bits for phase 2."]
    #[inline(always)]
    pub fn is_24b(&self) -> bool {
        *self == Wdlen2::_24b
    }
    #[doc = "Receive channel length is 32 bits for phase 2."]
    #[inline(always)]
    pub fn is_32b(&self) -> bool {
        *self == Wdlen2::_32b
    }
}
#[doc = "Field `WDLEN2` writer - Receive channel length in bits for phase 2. 0: 8b, 2: 16b, 4: 24b, 5: 32b, 1,3,6,7: Reserved"]
pub type Wdlen2W<'a, REG> = crate::FieldWriter<'a, REG, 3, Wdlen2>;
impl<'a, REG> Wdlen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Receive channel length is 8 bits for phase 2."]
    #[inline(always)]
    pub fn _8b(self) -> &'a mut crate::W<REG> {
        self.variant(Wdlen2::_8b)
    }
    #[doc = "Receive channel length is 16 bits for phase 2."]
    #[inline(always)]
    pub fn _16b(self) -> &'a mut crate::W<REG> {
        self.variant(Wdlen2::_16b)
    }
    #[doc = "Receive channel length is 24 bits for phase 2."]
    #[inline(always)]
    pub fn _24b(self) -> &'a mut crate::W<REG> {
        self.variant(Wdlen2::_24b)
    }
    #[doc = "Receive channel length is 32 bits for phase 2."]
    #[inline(always)]
    pub fn _32b(self) -> &'a mut crate::W<REG> {
        self.variant(Wdlen2::_32b)
    }
}
#[doc = "Number of channels in phase 2; 0: 1 Channel in phase 2, .. 0x7: 8 channels in phase 2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frlen2 {
    #[doc = "0: One channel in phase 2."]
    _1chls = 0,
    #[doc = "1: Two channels in phase 2."]
    _2chls = 1,
    #[doc = "2: Three channels in phase 2."]
    _3chls = 2,
    #[doc = "3: Four channels in phase 2."]
    _4chls = 3,
    #[doc = "4: Five channels in phase 2."]
    _5chls = 4,
    #[doc = "5: Six channels in phase 2."]
    _6chls = 5,
    #[doc = "6: Seven channels in phase 2."]
    _7chls = 6,
    #[doc = "7: Eight channels in phase 2."]
    _8chls = 7,
}
impl From<Frlen2> for u8 {
    #[inline(always)]
    fn from(variant: Frlen2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frlen2 {
    type Ux = u8;
}
impl crate::IsEnum for Frlen2 {}
#[doc = "Field `FRLEN2` reader - Number of channels in phase 2; 0: 1 Channel in phase 2, .. 0x7: 8 channels in phase 2"]
pub type Frlen2R = crate::FieldReader<Frlen2>;
impl Frlen2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Frlen2> {
        match self.bits {
            0 => Some(Frlen2::_1chls),
            1 => Some(Frlen2::_2chls),
            2 => Some(Frlen2::_3chls),
            3 => Some(Frlen2::_4chls),
            4 => Some(Frlen2::_5chls),
            5 => Some(Frlen2::_6chls),
            6 => Some(Frlen2::_7chls),
            7 => Some(Frlen2::_8chls),
            _ => None,
        }
    }
    #[doc = "One channel in phase 2."]
    #[inline(always)]
    pub fn is_1chls(&self) -> bool {
        *self == Frlen2::_1chls
    }
    #[doc = "Two channels in phase 2."]
    #[inline(always)]
    pub fn is_2chls(&self) -> bool {
        *self == Frlen2::_2chls
    }
    #[doc = "Three channels in phase 2."]
    #[inline(always)]
    pub fn is_3chls(&self) -> bool {
        *self == Frlen2::_3chls
    }
    #[doc = "Four channels in phase 2."]
    #[inline(always)]
    pub fn is_4chls(&self) -> bool {
        *self == Frlen2::_4chls
    }
    #[doc = "Five channels in phase 2."]
    #[inline(always)]
    pub fn is_5chls(&self) -> bool {
        *self == Frlen2::_5chls
    }
    #[doc = "Six channels in phase 2."]
    #[inline(always)]
    pub fn is_6chls(&self) -> bool {
        *self == Frlen2::_6chls
    }
    #[doc = "Seven channels in phase 2."]
    #[inline(always)]
    pub fn is_7chls(&self) -> bool {
        *self == Frlen2::_7chls
    }
    #[doc = "Eight channels in phase 2."]
    #[inline(always)]
    pub fn is_8chls(&self) -> bool {
        *self == Frlen2::_8chls
    }
}
#[doc = "Field `FRLEN2` writer - Number of channels in phase 2; 0: 1 Channel in phase 2, .. 0x7: 8 channels in phase 2"]
pub type Frlen2W<'a, REG> = crate::FieldWriter<'a, REG, 7, Frlen2>;
impl<'a, REG> Frlen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One channel in phase 2."]
    #[inline(always)]
    pub fn _1chls(self) -> &'a mut crate::W<REG> {
        self.variant(Frlen2::_1chls)
    }
    #[doc = "Two channels in phase 2."]
    #[inline(always)]
    pub fn _2chls(self) -> &'a mut crate::W<REG> {
        self.variant(Frlen2::_2chls)
    }
    #[doc = "Three channels in phase 2."]
    #[inline(always)]
    pub fn _3chls(self) -> &'a mut crate::W<REG> {
        self.variant(Frlen2::_3chls)
    }
    #[doc = "Four channels in phase 2."]
    #[inline(always)]
    pub fn _4chls(self) -> &'a mut crate::W<REG> {
        self.variant(Frlen2::_4chls)
    }
    #[doc = "Five channels in phase 2."]
    #[inline(always)]
    pub fn _5chls(self) -> &'a mut crate::W<REG> {
        self.variant(Frlen2::_5chls)
    }
    #[doc = "Six channels in phase 2."]
    #[inline(always)]
    pub fn _6chls(self) -> &'a mut crate::W<REG> {
        self.variant(Frlen2::_6chls)
    }
    #[doc = "Seven channels in phase 2."]
    #[inline(always)]
    pub fn _7chls(self) -> &'a mut crate::W<REG> {
        self.variant(Frlen2::_7chls)
    }
    #[doc = "Eight channels in phase 2."]
    #[inline(always)]
    pub fn _8chls(self) -> &'a mut crate::W<REG> {
        self.variant(Frlen2::_8chls)
    }
}
#[doc = "Field `PH` reader - Read Phase Bit. 0: Single Phase frame; 1: Dual-Phase frame."]
pub type PhR = crate::BitReader;
#[doc = "Field `PH` writer - Read Phase Bit. 0: Single Phase frame; 1: Dual-Phase frame."]
pub type PhW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Receive audio sample length for phase 1. 0: 8b, 2: 16b, 4: 24b, 5: 32b, 1,3,6,7: Reserved"]
    #[inline(always)]
    pub fn ssz1(&self) -> Ssz1R {
        Ssz1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Audio sample justification. 0: Left-justified, 1: Right-justified"]
    #[inline(always)]
    pub fn just(&self) -> JustR {
        JustR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Receive channel length in bits for phase 1. 0: 8b, 2: 16b, 4: 24b, 5: 32b, 1,3,6,7: Reserved"]
    #[inline(always)]
    pub fn wdlen1(&self) -> Wdlen1R {
        Wdlen1R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:14 - Number of channels in phase 1; 0: 1 Channel in phase 2, .. 0x7: 8 channels in phase 1"]
    #[inline(always)]
    pub fn frlen1(&self) -> Frlen1R {
        Frlen1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:18 - Receive audio sample length for phase 2. 0: 8b, 2: 16b, 4: 24b, 5: 32b, 1,3,6,7: Reserved"]
    #[inline(always)]
    pub fn ssz2(&self) -> Ssz2R {
        Ssz2R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:20 - Receive data delay bit count. Valid values are 0-2, 3 is reserved."]
    #[inline(always)]
    pub fn datadly(&self) -> DatadlyR {
        DatadlyR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:23 - Receive channel length in bits for phase 2. 0: 8b, 2: 16b, 4: 24b, 5: 32b, 1,3,6,7: Reserved"]
    #[inline(always)]
    pub fn wdlen2(&self) -> Wdlen2R {
        Wdlen2R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:30 - Number of channels in phase 2; 0: 1 Channel in phase 2, .. 0x7: 8 channels in phase 2"]
    #[inline(always)]
    pub fn frlen2(&self) -> Frlen2R {
        Frlen2R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Read Phase Bit. 0: Single Phase frame; 1: Dual-Phase frame."]
    #[inline(always)]
    pub fn ph(&self) -> PhR {
        PhR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive audio sample length for phase 1. 0: 8b, 2: 16b, 4: 24b, 5: 32b, 1,3,6,7: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ssz1(&mut self) -> Ssz1W<I2sdatacfgSpec> {
        Ssz1W::new(self, 0)
    }
    #[doc = "Bit 3 - Audio sample justification. 0: Left-justified, 1: Right-justified"]
    #[inline(always)]
    #[must_use]
    pub fn just(&mut self) -> JustW<I2sdatacfgSpec> {
        JustW::new(self, 3)
    }
    #[doc = "Bits 5:7 - Receive channel length in bits for phase 1. 0: 8b, 2: 16b, 4: 24b, 5: 32b, 1,3,6,7: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn wdlen1(&mut self) -> Wdlen1W<I2sdatacfgSpec> {
        Wdlen1W::new(self, 5)
    }
    #[doc = "Bits 8:14 - Number of channels in phase 1; 0: 1 Channel in phase 2, .. 0x7: 8 channels in phase 1"]
    #[inline(always)]
    #[must_use]
    pub fn frlen1(&mut self) -> Frlen1W<I2sdatacfgSpec> {
        Frlen1W::new(self, 8)
    }
    #[doc = "Bits 16:18 - Receive audio sample length for phase 2. 0: 8b, 2: 16b, 4: 24b, 5: 32b, 1,3,6,7: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ssz2(&mut self) -> Ssz2W<I2sdatacfgSpec> {
        Ssz2W::new(self, 16)
    }
    #[doc = "Bits 19:20 - Receive data delay bit count. Valid values are 0-2, 3 is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn datadly(&mut self) -> DatadlyW<I2sdatacfgSpec> {
        DatadlyW::new(self, 19)
    }
    #[doc = "Bits 21:23 - Receive channel length in bits for phase 2. 0: 8b, 2: 16b, 4: 24b, 5: 32b, 1,3,6,7: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn wdlen2(&mut self) -> Wdlen2W<I2sdatacfgSpec> {
        Wdlen2W::new(self, 21)
    }
    #[doc = "Bits 24:30 - Number of channels in phase 2; 0: 1 Channel in phase 2, .. 0x7: 8 channels in phase 2"]
    #[inline(always)]
    #[must_use]
    pub fn frlen2(&mut self) -> Frlen2W<I2sdatacfgSpec> {
        Frlen2W::new(self, 24)
    }
    #[doc = "Bit 31 - Read Phase Bit. 0: Single Phase frame; 1: Dual-Phase frame."]
    #[inline(always)]
    #[must_use]
    pub fn ph(&mut self) -> PhW<I2sdatacfgSpec> {
        PhW::new(self, 31)
    }
}
#[doc = "Specifies the data format of I2S sub frames\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sdatacfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sdatacfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sdatacfgSpec;
impl crate::RegisterSpec for I2sdatacfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2sdatacfg::R`](R) reader structure"]
impl crate::Readable for I2sdatacfgSpec {}
#[doc = "`write(|w| ..)` method takes [`i2sdatacfg::W`](W) writer structure"]
impl crate::Writable for I2sdatacfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2SDATACFG to value 0x01ac_01a4"]
impl crate::Resettable for I2sdatacfgSpec {
    const RESET_VALUE: u32 = 0x01ac_01a4;
}
