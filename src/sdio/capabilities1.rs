#[doc = "Register `CAPABILITIES1` reader"]
pub type R = crate::R<Capabilities1Spec>;
#[doc = "Register `CAPABILITIES1` writer"]
pub type W = crate::W<Capabilities1Spec>;
#[doc = "1- SDR50 is Supported\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdr50 {
    #[doc = "1: SDR50 is Not Supported"]
    Supported = 1,
    #[doc = "0: SDR50 is Not Supported"]
    Notsupported = 0,
}
impl From<Sdr50> for bool {
    #[inline(always)]
    fn from(variant: Sdr50) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDR50` reader - 1- SDR50 is Supported"]
pub type Sdr50R = crate::BitReader<Sdr50>;
impl Sdr50R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdr50 {
        match self.bits {
            true => Sdr50::Supported,
            false => Sdr50::Notsupported,
        }
    }
    #[doc = "SDR50 is Not Supported"]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Sdr50::Supported
    }
    #[doc = "SDR50 is Not Supported"]
    #[inline(always)]
    pub fn is_notsupported(&self) -> bool {
        *self == Sdr50::Notsupported
    }
}
#[doc = "Field `SDR50` writer - 1- SDR50 is Supported"]
pub type Sdr50W<'a, REG> = crate::BitWriter<'a, REG, Sdr50>;
impl<'a, REG> Sdr50W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDR50 is Not Supported"]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr50::Supported)
    }
    #[doc = "SDR50 is Not Supported"]
    #[inline(always)]
    pub fn notsupported(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr50::Notsupported)
    }
}
#[doc = "1- SDR104 is Supported\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdr104 {
    #[doc = "1: SDR104 is Not Supported"]
    Supported = 1,
    #[doc = "0: SDR104 is Not Supported"]
    Notsupported = 0,
}
impl From<Sdr104> for bool {
    #[inline(always)]
    fn from(variant: Sdr104) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDR104` reader - 1- SDR104 is Supported"]
pub type Sdr104R = crate::BitReader<Sdr104>;
impl Sdr104R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdr104 {
        match self.bits {
            true => Sdr104::Supported,
            false => Sdr104::Notsupported,
        }
    }
    #[doc = "SDR104 is Not Supported"]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Sdr104::Supported
    }
    #[doc = "SDR104 is Not Supported"]
    #[inline(always)]
    pub fn is_notsupported(&self) -> bool {
        *self == Sdr104::Notsupported
    }
}
#[doc = "Field `SDR104` writer - 1- SDR104 is Supported"]
pub type Sdr104W<'a, REG> = crate::BitWriter<'a, REG, Sdr104>;
impl<'a, REG> Sdr104W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDR104 is Not Supported"]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr104::Supported)
    }
    #[doc = "SDR104 is Not Supported"]
    #[inline(always)]
    pub fn notsupported(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr104::Notsupported)
    }
}
#[doc = "DDR50 field description needed here.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddr50 {
    #[doc = "1: DDR50 is Supported"]
    Supported = 1,
    #[doc = "0: DDR50 is Not Supported"]
    Notsupported = 0,
}
impl From<Ddr50> for bool {
    #[inline(always)]
    fn from(variant: Ddr50) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDR50` reader - DDR50 field description needed here."]
pub type Ddr50R = crate::BitReader<Ddr50>;
impl Ddr50R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddr50 {
        match self.bits {
            true => Ddr50::Supported,
            false => Ddr50::Notsupported,
        }
    }
    #[doc = "DDR50 is Supported"]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Ddr50::Supported
    }
    #[doc = "DDR50 is Not Supported"]
    #[inline(always)]
    pub fn is_notsupported(&self) -> bool {
        *self == Ddr50::Notsupported
    }
}
#[doc = "Field `DDR50` writer - DDR50 field description needed here."]
pub type Ddr50W<'a, REG> = crate::BitWriter<'a, REG, Ddr50>;
impl<'a, REG> Ddr50W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DDR50 is Supported"]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Ddr50::Supported)
    }
    #[doc = "DDR50 is Not Supported"]
    #[inline(always)]
    pub fn notsupported(self) -> &'a mut crate::W<REG> {
        self.variant(Ddr50::Notsupported)
    }
}
#[doc = "This bit indicates support of Driver Type A for 1.8 Signaling.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Typea {
    #[doc = "1: Driver Type A is Supported"]
    Supported = 1,
    #[doc = "0: Driver Type A is Not Supported"]
    Notsupported = 0,
}
impl From<Typea> for bool {
    #[inline(always)]
    fn from(variant: Typea) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPEA` reader - This bit indicates support of Driver Type A for 1.8 Signaling."]
pub type TypeaR = crate::BitReader<Typea>;
impl TypeaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Typea {
        match self.bits {
            true => Typea::Supported,
            false => Typea::Notsupported,
        }
    }
    #[doc = "Driver Type A is Supported"]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Typea::Supported
    }
    #[doc = "Driver Type A is Not Supported"]
    #[inline(always)]
    pub fn is_notsupported(&self) -> bool {
        *self == Typea::Notsupported
    }
}
#[doc = "Field `TYPEA` writer - This bit indicates support of Driver Type A for 1.8 Signaling."]
pub type TypeaW<'a, REG> = crate::BitWriter<'a, REG, Typea>;
impl<'a, REG> TypeaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Driver Type A is Supported"]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Typea::Supported)
    }
    #[doc = "Driver Type A is Not Supported"]
    #[inline(always)]
    pub fn notsupported(self) -> &'a mut crate::W<REG> {
        self.variant(Typea::Notsupported)
    }
}
#[doc = "This bit indicates support of Driver Type C for 1.8 Signaling.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Typec {
    #[doc = "1: Driver Type C is Supported"]
    Supported = 1,
    #[doc = "0: Driver Type C is Not Supported"]
    Notsupported = 0,
}
impl From<Typec> for bool {
    #[inline(always)]
    fn from(variant: Typec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPEC` reader - This bit indicates support of Driver Type C for 1.8 Signaling."]
pub type TypecR = crate::BitReader<Typec>;
impl TypecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Typec {
        match self.bits {
            true => Typec::Supported,
            false => Typec::Notsupported,
        }
    }
    #[doc = "Driver Type C is Supported"]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Typec::Supported
    }
    #[doc = "Driver Type C is Not Supported"]
    #[inline(always)]
    pub fn is_notsupported(&self) -> bool {
        *self == Typec::Notsupported
    }
}
#[doc = "Field `TYPEC` writer - This bit indicates support of Driver Type C for 1.8 Signaling."]
pub type TypecW<'a, REG> = crate::BitWriter<'a, REG, Typec>;
impl<'a, REG> TypecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Driver Type C is Supported"]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Typec::Supported)
    }
    #[doc = "Driver Type C is Not Supported"]
    #[inline(always)]
    pub fn notsupported(self) -> &'a mut crate::W<REG> {
        self.variant(Typec::Notsupported)
    }
}
#[doc = "Reserved This bit indicates support of Driver Type D for 1.8 Signaling.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Typed {
    #[doc = "1: Driver Type D is Supported"]
    Supported = 1,
    #[doc = "0: Driver Type D is Not Supported"]
    Notsupported = 0,
}
impl From<Typed> for bool {
    #[inline(always)]
    fn from(variant: Typed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPED` reader - Reserved This bit indicates support of Driver Type D for 1.8 Signaling."]
pub type TypedR = crate::BitReader<Typed>;
impl TypedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Typed {
        match self.bits {
            true => Typed::Supported,
            false => Typed::Notsupported,
        }
    }
    #[doc = "Driver Type D is Supported"]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Typed::Supported
    }
    #[doc = "Driver Type D is Not Supported"]
    #[inline(always)]
    pub fn is_notsupported(&self) -> bool {
        *self == Typed::Notsupported
    }
}
#[doc = "Field `TYPED` writer - Reserved This bit indicates support of Driver Type D for 1.8 Signaling."]
pub type TypedW<'a, REG> = crate::BitWriter<'a, REG, Typed>;
impl<'a, REG> TypedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Driver Type D is Supported"]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Typed::Supported)
    }
    #[doc = "Driver Type D is Not Supported"]
    #[inline(always)]
    pub fn notsupported(self) -> &'a mut crate::W<REG> {
        self.variant(Typed::Notsupported)
    }
}
#[doc = "This field indicates an initial value of the Re-Tuning Timer for Re-Tuning Mode 1 to 3. 0h - Get information via other source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Retuningtmrcnt {
    #[doc = "0: 0h Get information via other source."]
    Other = 0,
    #[doc = "1: 1 seconds"]
    _1sec = 1,
    #[doc = "2: 2 seconds"]
    _2sec = 2,
    #[doc = "3: 4 seconds"]
    _4sec = 3,
    #[doc = "4: 8 seconds"]
    _8s = 4,
    #[doc = "5: 16 seconds"]
    _16s = 5,
    #[doc = "6: 32 seconds"]
    _32s = 6,
    #[doc = "7: 64 seconds"]
    _64s = 7,
    #[doc = "8: 128 seconds"]
    _128s = 8,
    #[doc = "9: 256 seconds"]
    _256s = 9,
    #[doc = "10: 512 seconds"]
    _512s = 10,
    #[doc = "11: 1024 seconds"]
    _1024s = 11,
}
impl From<Retuningtmrcnt> for u8 {
    #[inline(always)]
    fn from(variant: Retuningtmrcnt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Retuningtmrcnt {
    type Ux = u8;
}
impl crate::IsEnum for Retuningtmrcnt {}
#[doc = "Field `RETUNINGTMRCNT` reader - This field indicates an initial value of the Re-Tuning Timer for Re-Tuning Mode 1 to 3. 0h - Get information via other source."]
pub type RetuningtmrcntR = crate::FieldReader<Retuningtmrcnt>;
impl RetuningtmrcntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Retuningtmrcnt> {
        match self.bits {
            0 => Some(Retuningtmrcnt::Other),
            1 => Some(Retuningtmrcnt::_1sec),
            2 => Some(Retuningtmrcnt::_2sec),
            3 => Some(Retuningtmrcnt::_4sec),
            4 => Some(Retuningtmrcnt::_8s),
            5 => Some(Retuningtmrcnt::_16s),
            6 => Some(Retuningtmrcnt::_32s),
            7 => Some(Retuningtmrcnt::_64s),
            8 => Some(Retuningtmrcnt::_128s),
            9 => Some(Retuningtmrcnt::_256s),
            10 => Some(Retuningtmrcnt::_512s),
            11 => Some(Retuningtmrcnt::_1024s),
            _ => None,
        }
    }
    #[doc = "0h Get information via other source."]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == Retuningtmrcnt::Other
    }
    #[doc = "1 seconds"]
    #[inline(always)]
    pub fn is_1sec(&self) -> bool {
        *self == Retuningtmrcnt::_1sec
    }
    #[doc = "2 seconds"]
    #[inline(always)]
    pub fn is_2sec(&self) -> bool {
        *self == Retuningtmrcnt::_2sec
    }
    #[doc = "4 seconds"]
    #[inline(always)]
    pub fn is_4sec(&self) -> bool {
        *self == Retuningtmrcnt::_4sec
    }
    #[doc = "8 seconds"]
    #[inline(always)]
    pub fn is_8s(&self) -> bool {
        *self == Retuningtmrcnt::_8s
    }
    #[doc = "16 seconds"]
    #[inline(always)]
    pub fn is_16s(&self) -> bool {
        *self == Retuningtmrcnt::_16s
    }
    #[doc = "32 seconds"]
    #[inline(always)]
    pub fn is_32s(&self) -> bool {
        *self == Retuningtmrcnt::_32s
    }
    #[doc = "64 seconds"]
    #[inline(always)]
    pub fn is_64s(&self) -> bool {
        *self == Retuningtmrcnt::_64s
    }
    #[doc = "128 seconds"]
    #[inline(always)]
    pub fn is_128s(&self) -> bool {
        *self == Retuningtmrcnt::_128s
    }
    #[doc = "256 seconds"]
    #[inline(always)]
    pub fn is_256s(&self) -> bool {
        *self == Retuningtmrcnt::_256s
    }
    #[doc = "512 seconds"]
    #[inline(always)]
    pub fn is_512s(&self) -> bool {
        *self == Retuningtmrcnt::_512s
    }
    #[doc = "1024 seconds"]
    #[inline(always)]
    pub fn is_1024s(&self) -> bool {
        *self == Retuningtmrcnt::_1024s
    }
}
#[doc = "Field `RETUNINGTMRCNT` writer - This field indicates an initial value of the Re-Tuning Timer for Re-Tuning Mode 1 to 3. 0h - Get information via other source."]
pub type RetuningtmrcntW<'a, REG> = crate::FieldWriter<'a, REG, 4, Retuningtmrcnt>;
impl<'a, REG> RetuningtmrcntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0h Get information via other source."]
    #[inline(always)]
    pub fn other(self) -> &'a mut crate::W<REG> {
        self.variant(Retuningtmrcnt::Other)
    }
    #[doc = "1 seconds"]
    #[inline(always)]
    pub fn _1sec(self) -> &'a mut crate::W<REG> {
        self.variant(Retuningtmrcnt::_1sec)
    }
    #[doc = "2 seconds"]
    #[inline(always)]
    pub fn _2sec(self) -> &'a mut crate::W<REG> {
        self.variant(Retuningtmrcnt::_2sec)
    }
    #[doc = "4 seconds"]
    #[inline(always)]
    pub fn _4sec(self) -> &'a mut crate::W<REG> {
        self.variant(Retuningtmrcnt::_4sec)
    }
    #[doc = "8 seconds"]
    #[inline(always)]
    pub fn _8s(self) -> &'a mut crate::W<REG> {
        self.variant(Retuningtmrcnt::_8s)
    }
    #[doc = "16 seconds"]
    #[inline(always)]
    pub fn _16s(self) -> &'a mut crate::W<REG> {
        self.variant(Retuningtmrcnt::_16s)
    }
    #[doc = "32 seconds"]
    #[inline(always)]
    pub fn _32s(self) -> &'a mut crate::W<REG> {
        self.variant(Retuningtmrcnt::_32s)
    }
    #[doc = "64 seconds"]
    #[inline(always)]
    pub fn _64s(self) -> &'a mut crate::W<REG> {
        self.variant(Retuningtmrcnt::_64s)
    }
    #[doc = "128 seconds"]
    #[inline(always)]
    pub fn _128s(self) -> &'a mut crate::W<REG> {
        self.variant(Retuningtmrcnt::_128s)
    }
    #[doc = "256 seconds"]
    #[inline(always)]
    pub fn _256s(self) -> &'a mut crate::W<REG> {
        self.variant(Retuningtmrcnt::_256s)
    }
    #[doc = "512 seconds"]
    #[inline(always)]
    pub fn _512s(self) -> &'a mut crate::W<REG> {
        self.variant(Retuningtmrcnt::_512s)
    }
    #[doc = "1024 seconds"]
    #[inline(always)]
    pub fn _1024s(self) -> &'a mut crate::W<REG> {
        self.variant(Retuningtmrcnt::_1024s)
    }
}
#[doc = "If this bit is set to 1, this Host Controller requires tuning to operate SDR50. (Tuning is always required to operate SDR104.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tuningsdr50 {
    #[doc = "1: SDR50 requires tuning"]
    Tuningreqd = 1,
    #[doc = "0: SDR50 does not require tuning"]
    Notuningreqd = 0,
}
impl From<Tuningsdr50> for bool {
    #[inline(always)]
    fn from(variant: Tuningsdr50) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TUNINGSDR50` reader - If this bit is set to 1, this Host Controller requires tuning to operate SDR50. (Tuning is always required to operate SDR104.)"]
pub type Tuningsdr50R = crate::BitReader<Tuningsdr50>;
impl Tuningsdr50R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tuningsdr50 {
        match self.bits {
            true => Tuningsdr50::Tuningreqd,
            false => Tuningsdr50::Notuningreqd,
        }
    }
    #[doc = "SDR50 requires tuning"]
    #[inline(always)]
    pub fn is_tuningreqd(&self) -> bool {
        *self == Tuningsdr50::Tuningreqd
    }
    #[doc = "SDR50 does not require tuning"]
    #[inline(always)]
    pub fn is_notuningreqd(&self) -> bool {
        *self == Tuningsdr50::Notuningreqd
    }
}
#[doc = "Field `TUNINGSDR50` writer - If this bit is set to 1, this Host Controller requires tuning to operate SDR50. (Tuning is always required to operate SDR104.)"]
pub type Tuningsdr50W<'a, REG> = crate::BitWriter<'a, REG, Tuningsdr50>;
impl<'a, REG> Tuningsdr50W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDR50 requires tuning"]
    #[inline(always)]
    pub fn tuningreqd(self) -> &'a mut crate::W<REG> {
        self.variant(Tuningsdr50::Tuningreqd)
    }
    #[doc = "SDR50 does not require tuning"]
    #[inline(always)]
    pub fn notuningreqd(self) -> &'a mut crate::W<REG> {
        self.variant(Tuningsdr50::Notuningreqd)
    }
}
#[doc = "This field defines the re-tuning capability of a Host Controller and how to manage the data transfer length and a Re-Tuning Timer by the Host Driver There are two re-tuning timings: Re-Tuning Request and expiration of a Re-Tuning Timer. By receiving either timing, the Host Driver executes the re-tuning procedure just before a next command issue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Retuningmodes {
    #[doc = "0: Mode1"]
    Mode1 = 0,
    #[doc = "1: Mode2"]
    Mode2 = 1,
    #[doc = "2: Mode3"]
    Mode3 = 2,
    #[doc = "3: Clock Multiplier is not supported."]
    Notsupported = 3,
}
impl From<Retuningmodes> for u8 {
    #[inline(always)]
    fn from(variant: Retuningmodes) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Retuningmodes {
    type Ux = u8;
}
impl crate::IsEnum for Retuningmodes {}
#[doc = "Field `RETUNINGMODES` reader - This field defines the re-tuning capability of a Host Controller and how to manage the data transfer length and a Re-Tuning Timer by the Host Driver There are two re-tuning timings: Re-Tuning Request and expiration of a Re-Tuning Timer. By receiving either timing, the Host Driver executes the re-tuning procedure just before a next command issue"]
pub type RetuningmodesR = crate::FieldReader<Retuningmodes>;
impl RetuningmodesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Retuningmodes {
        match self.bits {
            0 => Retuningmodes::Mode1,
            1 => Retuningmodes::Mode2,
            2 => Retuningmodes::Mode3,
            3 => Retuningmodes::Notsupported,
            _ => unreachable!(),
        }
    }
    #[doc = "Mode1"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == Retuningmodes::Mode1
    }
    #[doc = "Mode2"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == Retuningmodes::Mode2
    }
    #[doc = "Mode3"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == Retuningmodes::Mode3
    }
    #[doc = "Clock Multiplier is not supported."]
    #[inline(always)]
    pub fn is_notsupported(&self) -> bool {
        *self == Retuningmodes::Notsupported
    }
}
#[doc = "Field `RETUNINGMODES` writer - This field defines the re-tuning capability of a Host Controller and how to manage the data transfer length and a Re-Tuning Timer by the Host Driver There are two re-tuning timings: Re-Tuning Request and expiration of a Re-Tuning Timer. By receiving either timing, the Host Driver executes the re-tuning procedure just before a next command issue"]
pub type RetuningmodesW<'a, REG> = crate::FieldWriter<'a, REG, 2, Retuningmodes, crate::Safe>;
impl<'a, REG> RetuningmodesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Mode1"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(Retuningmodes::Mode1)
    }
    #[doc = "Mode2"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(Retuningmodes::Mode2)
    }
    #[doc = "Mode3"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(Retuningmodes::Mode3)
    }
    #[doc = "Clock Multiplier is not supported."]
    #[inline(always)]
    pub fn notsupported(self) -> &'a mut crate::W<REG> {
        self.variant(Retuningmodes::Notsupported)
    }
}
#[doc = "This field indicates clock multiplier value of programmable clock generator. Refer to Clock Control register. Setting 00h means that Host Controller does not support programmable clock generator. The multiplier is (CLKMULT+1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkmult {
    #[doc = "255: Clock Multiplier M = 256"]
    Multx256 = 255,
    #[doc = "2: Clock Multiplier M = 3"]
    Multx3 = 2,
    #[doc = "1: Clock Multiplier M = 2"]
    Multx2 = 1,
    #[doc = "0: Clock Multiplier is Not Supported"]
    Notsupported = 0,
}
impl From<Clkmult> for u8 {
    #[inline(always)]
    fn from(variant: Clkmult) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkmult {
    type Ux = u8;
}
impl crate::IsEnum for Clkmult {}
#[doc = "Field `CLKMULT` reader - This field indicates clock multiplier value of programmable clock generator. Refer to Clock Control register. Setting 00h means that Host Controller does not support programmable clock generator. The multiplier is (CLKMULT+1)."]
pub type ClkmultR = crate::FieldReader<Clkmult>;
impl ClkmultR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkmult> {
        match self.bits {
            255 => Some(Clkmult::Multx256),
            2 => Some(Clkmult::Multx3),
            1 => Some(Clkmult::Multx2),
            0 => Some(Clkmult::Notsupported),
            _ => None,
        }
    }
    #[doc = "Clock Multiplier M = 256"]
    #[inline(always)]
    pub fn is_multx256(&self) -> bool {
        *self == Clkmult::Multx256
    }
    #[doc = "Clock Multiplier M = 3"]
    #[inline(always)]
    pub fn is_multx3(&self) -> bool {
        *self == Clkmult::Multx3
    }
    #[doc = "Clock Multiplier M = 2"]
    #[inline(always)]
    pub fn is_multx2(&self) -> bool {
        *self == Clkmult::Multx2
    }
    #[doc = "Clock Multiplier is Not Supported"]
    #[inline(always)]
    pub fn is_notsupported(&self) -> bool {
        *self == Clkmult::Notsupported
    }
}
#[doc = "Field `CLKMULT` writer - This field indicates clock multiplier value of programmable clock generator. Refer to Clock Control register. Setting 00h means that Host Controller does not support programmable clock generator. The multiplier is (CLKMULT+1)."]
pub type ClkmultW<'a, REG> = crate::FieldWriter<'a, REG, 8, Clkmult>;
impl<'a, REG> ClkmultW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock Multiplier M = 256"]
    #[inline(always)]
    pub fn multx256(self) -> &'a mut crate::W<REG> {
        self.variant(Clkmult::Multx256)
    }
    #[doc = "Clock Multiplier M = 3"]
    #[inline(always)]
    pub fn multx3(self) -> &'a mut crate::W<REG> {
        self.variant(Clkmult::Multx3)
    }
    #[doc = "Clock Multiplier M = 2"]
    #[inline(always)]
    pub fn multx2(self) -> &'a mut crate::W<REG> {
        self.variant(Clkmult::Multx2)
    }
    #[doc = "Clock Multiplier is Not Supported"]
    #[inline(always)]
    pub fn notsupported(self) -> &'a mut crate::W<REG> {
        self.variant(Clkmult::Notsupported)
    }
}
#[doc = "Spi mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spimode {
    #[doc = "0: Not Supported"]
    Notsupported = 0,
    #[doc = "1: Supported"]
    Supported = 1,
}
impl From<Spimode> for bool {
    #[inline(always)]
    fn from(variant: Spimode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIMODE` reader - Spi mode"]
pub type SpimodeR = crate::BitReader<Spimode>;
impl SpimodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spimode {
        match self.bits {
            false => Spimode::Notsupported,
            true => Spimode::Supported,
        }
    }
    #[doc = "Not Supported"]
    #[inline(always)]
    pub fn is_notsupported(&self) -> bool {
        *self == Spimode::Notsupported
    }
    #[doc = "Supported"]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Spimode::Supported
    }
}
#[doc = "Field `SPIMODE` writer - Spi mode"]
pub type SpimodeW<'a, REG> = crate::BitWriter<'a, REG, Spimode>;
impl<'a, REG> SpimodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Supported"]
    #[inline(always)]
    pub fn notsupported(self) -> &'a mut crate::W<REG> {
        self.variant(Spimode::Notsupported)
    }
    #[doc = "Supported"]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Spimode::Supported)
    }
}
#[doc = "Spi block mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spiblockmode {
    #[doc = "0: Not Supported"]
    Notsupported = 0,
    #[doc = "1: Supported"]
    Supported = 1,
}
impl From<Spiblockmode> for bool {
    #[inline(always)]
    fn from(variant: Spiblockmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIBLOCKMODE` reader - Spi block mode"]
pub type SpiblockmodeR = crate::BitReader<Spiblockmode>;
impl SpiblockmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spiblockmode {
        match self.bits {
            false => Spiblockmode::Notsupported,
            true => Spiblockmode::Supported,
        }
    }
    #[doc = "Not Supported"]
    #[inline(always)]
    pub fn is_notsupported(&self) -> bool {
        *self == Spiblockmode::Notsupported
    }
    #[doc = "Supported"]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Spiblockmode::Supported
    }
}
#[doc = "Field `SPIBLOCKMODE` writer - Spi block mode"]
pub type SpiblockmodeW<'a, REG> = crate::BitWriter<'a, REG, Spiblockmode>;
impl<'a, REG> SpiblockmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Supported"]
    #[inline(always)]
    pub fn notsupported(self) -> &'a mut crate::W<REG> {
        self.variant(Spiblockmode::Notsupported)
    }
    #[doc = "Supported"]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Spiblockmode::Supported)
    }
}
impl R {
    #[doc = "Bit 0 - 1- SDR50 is Supported"]
    #[inline(always)]
    pub fn sdr50(&self) -> Sdr50R {
        Sdr50R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1- SDR104 is Supported"]
    #[inline(always)]
    pub fn sdr104(&self) -> Sdr104R {
        Sdr104R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DDR50 field description needed here."]
    #[inline(always)]
    pub fn ddr50(&self) -> Ddr50R {
        Ddr50R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit indicates support of Driver Type A for 1.8 Signaling."]
    #[inline(always)]
    pub fn typea(&self) -> TypeaR {
        TypeaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit indicates support of Driver Type C for 1.8 Signaling."]
    #[inline(always)]
    pub fn typec(&self) -> TypecR {
        TypecR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved This bit indicates support of Driver Type D for 1.8 Signaling."]
    #[inline(always)]
    pub fn typed(&self) -> TypedR {
        TypedR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - This field indicates an initial value of the Re-Tuning Timer for Re-Tuning Mode 1 to 3. 0h - Get information via other source."]
    #[inline(always)]
    pub fn retuningtmrcnt(&self) -> RetuningtmrcntR {
        RetuningtmrcntR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - If this bit is set to 1, this Host Controller requires tuning to operate SDR50. (Tuning is always required to operate SDR104.)"]
    #[inline(always)]
    pub fn tuningsdr50(&self) -> Tuningsdr50R {
        Tuningsdr50R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - This field defines the re-tuning capability of a Host Controller and how to manage the data transfer length and a Re-Tuning Timer by the Host Driver There are two re-tuning timings: Re-Tuning Request and expiration of a Re-Tuning Timer. By receiving either timing, the Host Driver executes the re-tuning procedure just before a next command issue"]
    #[inline(always)]
    pub fn retuningmodes(&self) -> RetuningmodesR {
        RetuningmodesR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - This field indicates clock multiplier value of programmable clock generator. Refer to Clock Control register. Setting 00h means that Host Controller does not support programmable clock generator. The multiplier is (CLKMULT+1)."]
    #[inline(always)]
    pub fn clkmult(&self) -> ClkmultR {
        ClkmultR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Spi mode"]
    #[inline(always)]
    pub fn spimode(&self) -> SpimodeR {
        SpimodeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Spi block mode"]
    #[inline(always)]
    pub fn spiblockmode(&self) -> SpiblockmodeR {
        SpiblockmodeR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1- SDR50 is Supported"]
    #[inline(always)]
    #[must_use]
    pub fn sdr50(&mut self) -> Sdr50W<Capabilities1Spec> {
        Sdr50W::new(self, 0)
    }
    #[doc = "Bit 1 - 1- SDR104 is Supported"]
    #[inline(always)]
    #[must_use]
    pub fn sdr104(&mut self) -> Sdr104W<Capabilities1Spec> {
        Sdr104W::new(self, 1)
    }
    #[doc = "Bit 2 - DDR50 field description needed here."]
    #[inline(always)]
    #[must_use]
    pub fn ddr50(&mut self) -> Ddr50W<Capabilities1Spec> {
        Ddr50W::new(self, 2)
    }
    #[doc = "Bit 4 - This bit indicates support of Driver Type A for 1.8 Signaling."]
    #[inline(always)]
    #[must_use]
    pub fn typea(&mut self) -> TypeaW<Capabilities1Spec> {
        TypeaW::new(self, 4)
    }
    #[doc = "Bit 5 - This bit indicates support of Driver Type C for 1.8 Signaling."]
    #[inline(always)]
    #[must_use]
    pub fn typec(&mut self) -> TypecW<Capabilities1Spec> {
        TypecW::new(self, 5)
    }
    #[doc = "Bit 6 - Reserved This bit indicates support of Driver Type D for 1.8 Signaling."]
    #[inline(always)]
    #[must_use]
    pub fn typed(&mut self) -> TypedW<Capabilities1Spec> {
        TypedW::new(self, 6)
    }
    #[doc = "Bits 8:11 - This field indicates an initial value of the Re-Tuning Timer for Re-Tuning Mode 1 to 3. 0h - Get information via other source."]
    #[inline(always)]
    #[must_use]
    pub fn retuningtmrcnt(&mut self) -> RetuningtmrcntW<Capabilities1Spec> {
        RetuningtmrcntW::new(self, 8)
    }
    #[doc = "Bit 13 - If this bit is set to 1, this Host Controller requires tuning to operate SDR50. (Tuning is always required to operate SDR104.)"]
    #[inline(always)]
    #[must_use]
    pub fn tuningsdr50(&mut self) -> Tuningsdr50W<Capabilities1Spec> {
        Tuningsdr50W::new(self, 13)
    }
    #[doc = "Bits 14:15 - This field defines the re-tuning capability of a Host Controller and how to manage the data transfer length and a Re-Tuning Timer by the Host Driver There are two re-tuning timings: Re-Tuning Request and expiration of a Re-Tuning Timer. By receiving either timing, the Host Driver executes the re-tuning procedure just before a next command issue"]
    #[inline(always)]
    #[must_use]
    pub fn retuningmodes(&mut self) -> RetuningmodesW<Capabilities1Spec> {
        RetuningmodesW::new(self, 14)
    }
    #[doc = "Bits 16:23 - This field indicates clock multiplier value of programmable clock generator. Refer to Clock Control register. Setting 00h means that Host Controller does not support programmable clock generator. The multiplier is (CLKMULT+1)."]
    #[inline(always)]
    #[must_use]
    pub fn clkmult(&mut self) -> ClkmultW<Capabilities1Spec> {
        ClkmultW::new(self, 16)
    }
    #[doc = "Bit 24 - Spi mode"]
    #[inline(always)]
    #[must_use]
    pub fn spimode(&mut self) -> SpimodeW<Capabilities1Spec> {
        SpimodeW::new(self, 24)
    }
    #[doc = "Bit 25 - Spi block mode"]
    #[inline(always)]
    #[must_use]
    pub fn spiblockmode(&mut self) -> SpiblockmodeW<Capabilities1Spec> {
        SpiblockmodeW::new(self, 25)
    }
}
#[doc = "Capabilities\n\nYou can [`read`](crate::Reg::read) this register and get [`capabilities1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capabilities1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Capabilities1Spec;
impl crate::RegisterSpec for Capabilities1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capabilities1::R`](R) reader structure"]
impl crate::Readable for Capabilities1Spec {}
#[doc = "`write(|w| ..)` method takes [`capabilities1::W`](W) writer structure"]
impl crate::Writable for Capabilities1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAPABILITIES1 to value 0"]
impl crate::Resettable for Capabilities1Spec {
    const RESET_VALUE: u32 = 0;
}
