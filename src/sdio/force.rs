#[doc = "Register `FORCE` reader"]
pub type R = crate::R<ForceSpec>;
#[doc = "Register `FORCE` writer"]
pub type W = crate::W<ForceSpec>;
#[doc = "Description\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forceacmd12not {
    #[doc = "1: Interrupt is generated"]
    Int = 1,
    #[doc = "0: no interrupt"]
    Noint = 0,
}
impl From<Forceacmd12not> for bool {
    #[inline(always)]
    fn from(variant: Forceacmd12not) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCEACMD12NOT` reader - Description"]
pub type Forceacmd12notR = crate::BitReader<Forceacmd12not>;
impl Forceacmd12notR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forceacmd12not {
        match self.bits {
            true => Forceacmd12not::Int,
            false => Forceacmd12not::Noint,
        }
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == Forceacmd12not::Int
    }
    #[doc = "no interrupt"]
    #[inline(always)]
    pub fn is_noint(&self) -> bool {
        *self == Forceacmd12not::Noint
    }
}
#[doc = "Field `FORCEACMD12NOT` writer - Description"]
pub type Forceacmd12notW<'a, REG> = crate::BitWriter<'a, REG, Forceacmd12not>;
impl<'a, REG> Forceacmd12notW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(Forceacmd12not::Int)
    }
    #[doc = "no interrupt"]
    #[inline(always)]
    pub fn noint(self) -> &'a mut crate::W<REG> {
        self.variant(Forceacmd12not::Noint)
    }
}
#[doc = "Description\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forceacmdtoerr {
    #[doc = "1: Interrupt is generated"]
    Int = 1,
    #[doc = "0: no interrupt"]
    Noint = 0,
}
impl From<Forceacmdtoerr> for bool {
    #[inline(always)]
    fn from(variant: Forceacmdtoerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCEACMDTOERR` reader - Description"]
pub type ForceacmdtoerrR = crate::BitReader<Forceacmdtoerr>;
impl ForceacmdtoerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forceacmdtoerr {
        match self.bits {
            true => Forceacmdtoerr::Int,
            false => Forceacmdtoerr::Noint,
        }
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == Forceacmdtoerr::Int
    }
    #[doc = "no interrupt"]
    #[inline(always)]
    pub fn is_noint(&self) -> bool {
        *self == Forceacmdtoerr::Noint
    }
}
#[doc = "Field `FORCEACMDTOERR` writer - Description"]
pub type ForceacmdtoerrW<'a, REG> = crate::BitWriter<'a, REG, Forceacmdtoerr>;
impl<'a, REG> ForceacmdtoerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(Forceacmdtoerr::Int)
    }
    #[doc = "no interrupt"]
    #[inline(always)]
    pub fn noint(self) -> &'a mut crate::W<REG> {
        self.variant(Forceacmdtoerr::Noint)
    }
}
#[doc = "Description\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forceacmdcrcerr {
    #[doc = "1: Interrupt is generated"]
    Int = 1,
    #[doc = "0: no interrupt"]
    Noint = 0,
}
impl From<Forceacmdcrcerr> for bool {
    #[inline(always)]
    fn from(variant: Forceacmdcrcerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCEACMDCRCERR` reader - Description"]
pub type ForceacmdcrcerrR = crate::BitReader<Forceacmdcrcerr>;
impl ForceacmdcrcerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forceacmdcrcerr {
        match self.bits {
            true => Forceacmdcrcerr::Int,
            false => Forceacmdcrcerr::Noint,
        }
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == Forceacmdcrcerr::Int
    }
    #[doc = "no interrupt"]
    #[inline(always)]
    pub fn is_noint(&self) -> bool {
        *self == Forceacmdcrcerr::Noint
    }
}
#[doc = "Field `FORCEACMDCRCERR` writer - Description"]
pub type ForceacmdcrcerrW<'a, REG> = crate::BitWriter<'a, REG, Forceacmdcrcerr>;
impl<'a, REG> ForceacmdcrcerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(Forceacmdcrcerr::Int)
    }
    #[doc = "no interrupt"]
    #[inline(always)]
    pub fn noint(self) -> &'a mut crate::W<REG> {
        self.variant(Forceacmdcrcerr::Noint)
    }
}
#[doc = "Description\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forceacmdenderr {
    #[doc = "1: Interrupt is generated"]
    Int = 1,
    #[doc = "0: no interrupt"]
    Noint = 0,
}
impl From<Forceacmdenderr> for bool {
    #[inline(always)]
    fn from(variant: Forceacmdenderr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCEACMDENDERR` reader - Description"]
pub type ForceacmdenderrR = crate::BitReader<Forceacmdenderr>;
impl ForceacmdenderrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forceacmdenderr {
        match self.bits {
            true => Forceacmdenderr::Int,
            false => Forceacmdenderr::Noint,
        }
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == Forceacmdenderr::Int
    }
    #[doc = "no interrupt"]
    #[inline(always)]
    pub fn is_noint(&self) -> bool {
        *self == Forceacmdenderr::Noint
    }
}
#[doc = "Field `FORCEACMDENDERR` writer - Description"]
pub type ForceacmdenderrW<'a, REG> = crate::BitWriter<'a, REG, Forceacmdenderr>;
impl<'a, REG> ForceacmdenderrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(Forceacmdenderr::Int)
    }
    #[doc = "no interrupt"]
    #[inline(always)]
    pub fn noint(self) -> &'a mut crate::W<REG> {
        self.variant(Forceacmdenderr::Noint)
    }
}
#[doc = "Desc\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forceacmdidxerr {
    #[doc = "1: Interrupt is generated"]
    Int = 1,
    #[doc = "0: no interrupt"]
    Noint = 0,
}
impl From<Forceacmdidxerr> for bool {
    #[inline(always)]
    fn from(variant: Forceacmdidxerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCEACMDIDXERR` reader - Desc"]
pub type ForceacmdidxerrR = crate::BitReader<Forceacmdidxerr>;
impl ForceacmdidxerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forceacmdidxerr {
        match self.bits {
            true => Forceacmdidxerr::Int,
            false => Forceacmdidxerr::Noint,
        }
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == Forceacmdidxerr::Int
    }
    #[doc = "no interrupt"]
    #[inline(always)]
    pub fn is_noint(&self) -> bool {
        *self == Forceacmdidxerr::Noint
    }
}
#[doc = "Field `FORCEACMDIDXERR` writer - Desc"]
pub type ForceacmdidxerrW<'a, REG> = crate::BitWriter<'a, REG, Forceacmdidxerr>;
impl<'a, REG> ForceacmdidxerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(Forceacmdidxerr::Int)
    }
    #[doc = "no interrupt"]
    #[inline(always)]
    pub fn noint(self) -> &'a mut crate::W<REG> {
        self.variant(Forceacmdidxerr::Noint)
    }
}
#[doc = "1 - Interrupt is generated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forceacmdissuederr {
    #[doc = "1: Interrupt is generated"]
    Int = 1,
    #[doc = "0: no interrupt"]
    Noint = 0,
}
impl From<Forceacmdissuederr> for bool {
    #[inline(always)]
    fn from(variant: Forceacmdissuederr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCEACMDISSUEDERR` reader - 1 - Interrupt is generated"]
pub type ForceacmdissuederrR = crate::BitReader<Forceacmdissuederr>;
impl ForceacmdissuederrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forceacmdissuederr {
        match self.bits {
            true => Forceacmdissuederr::Int,
            false => Forceacmdissuederr::Noint,
        }
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == Forceacmdissuederr::Int
    }
    #[doc = "no interrupt"]
    #[inline(always)]
    pub fn is_noint(&self) -> bool {
        *self == Forceacmdissuederr::Noint
    }
}
#[doc = "Field `FORCEACMDISSUEDERR` writer - 1 - Interrupt is generated"]
pub type ForceacmdissuederrW<'a, REG> = crate::BitWriter<'a, REG, Forceacmdissuederr>;
impl<'a, REG> ForceacmdissuederrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(Forceacmdissuederr::Int)
    }
    #[doc = "no interrupt"]
    #[inline(always)]
    pub fn noint(self) -> &'a mut crate::W<REG> {
        self.variant(Forceacmdissuederr::Noint)
    }
}
#[doc = "Force Event for Command Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forcecmdtoerr {
    #[doc = "1: Interrupt is generated"]
    Int = 1,
    #[doc = "0: No interrupt"]
    Noint = 0,
}
impl From<Forcecmdtoerr> for bool {
    #[inline(always)]
    fn from(variant: Forcecmdtoerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCECMDTOERR` reader - Force Event for Command Timeout Error"]
pub type ForcecmdtoerrR = crate::BitReader<Forcecmdtoerr>;
impl ForcecmdtoerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forcecmdtoerr {
        match self.bits {
            true => Forcecmdtoerr::Int,
            false => Forcecmdtoerr::Noint,
        }
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == Forcecmdtoerr::Int
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn is_noint(&self) -> bool {
        *self == Forcecmdtoerr::Noint
    }
}
#[doc = "Field `FORCECMDTOERR` writer - Force Event for Command Timeout Error"]
pub type ForcecmdtoerrW<'a, REG> = crate::BitWriter<'a, REG, Forcecmdtoerr>;
impl<'a, REG> ForcecmdtoerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(Forcecmdtoerr::Int)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn noint(self) -> &'a mut crate::W<REG> {
        self.variant(Forcecmdtoerr::Noint)
    }
}
#[doc = "Force Event for Command CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forcecmdcrcerr {
    #[doc = "1: Interrupt is generated"]
    Int = 1,
    #[doc = "0: No interrupt"]
    Noint = 0,
}
impl From<Forcecmdcrcerr> for bool {
    #[inline(always)]
    fn from(variant: Forcecmdcrcerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCECMDCRCERR` reader - Force Event for Command CRC Error"]
pub type ForcecmdcrcerrR = crate::BitReader<Forcecmdcrcerr>;
impl ForcecmdcrcerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forcecmdcrcerr {
        match self.bits {
            true => Forcecmdcrcerr::Int,
            false => Forcecmdcrcerr::Noint,
        }
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == Forcecmdcrcerr::Int
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn is_noint(&self) -> bool {
        *self == Forcecmdcrcerr::Noint
    }
}
#[doc = "Field `FORCECMDCRCERR` writer - Force Event for Command CRC Error"]
pub type ForcecmdcrcerrW<'a, REG> = crate::BitWriter<'a, REG, Forcecmdcrcerr>;
impl<'a, REG> ForcecmdcrcerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(Forcecmdcrcerr::Int)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn noint(self) -> &'a mut crate::W<REG> {
        self.variant(Forcecmdcrcerr::Noint)
    }
}
#[doc = "Force Event for Command End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forcecmdenderr {
    #[doc = "1: Interrupt is generated"]
    Int = 1,
    #[doc = "0: No interrupt"]
    Noint = 0,
}
impl From<Forcecmdenderr> for bool {
    #[inline(always)]
    fn from(variant: Forcecmdenderr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCECMDENDERR` reader - Force Event for Command End Bit Error"]
pub type ForcecmdenderrR = crate::BitReader<Forcecmdenderr>;
impl ForcecmdenderrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forcecmdenderr {
        match self.bits {
            true => Forcecmdenderr::Int,
            false => Forcecmdenderr::Noint,
        }
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == Forcecmdenderr::Int
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn is_noint(&self) -> bool {
        *self == Forcecmdenderr::Noint
    }
}
#[doc = "Field `FORCECMDENDERR` writer - Force Event for Command End Bit Error"]
pub type ForcecmdenderrW<'a, REG> = crate::BitWriter<'a, REG, Forcecmdenderr>;
impl<'a, REG> ForcecmdenderrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(Forcecmdenderr::Int)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn noint(self) -> &'a mut crate::W<REG> {
        self.variant(Forcecmdenderr::Noint)
    }
}
#[doc = "Force Event for Command Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forcecmdidxerr {
    #[doc = "1: Interrupt is generated"]
    Int = 1,
    #[doc = "0: No interrupt"]
    Noint = 0,
}
impl From<Forcecmdidxerr> for bool {
    #[inline(always)]
    fn from(variant: Forcecmdidxerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCECMDIDXERR` reader - Force Event for Command Index Error"]
pub type ForcecmdidxerrR = crate::BitReader<Forcecmdidxerr>;
impl ForcecmdidxerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forcecmdidxerr {
        match self.bits {
            true => Forcecmdidxerr::Int,
            false => Forcecmdidxerr::Noint,
        }
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == Forcecmdidxerr::Int
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn is_noint(&self) -> bool {
        *self == Forcecmdidxerr::Noint
    }
}
#[doc = "Field `FORCECMDIDXERR` writer - Force Event for Command Index Error"]
pub type ForcecmdidxerrW<'a, REG> = crate::BitWriter<'a, REG, Forcecmdidxerr>;
impl<'a, REG> ForcecmdidxerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(Forcecmdidxerr::Int)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn noint(self) -> &'a mut crate::W<REG> {
        self.variant(Forcecmdidxerr::Noint)
    }
}
#[doc = "Force Event for Data Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forcedatatoerr {
    #[doc = "1: Interrupt is generated"]
    Int = 1,
    #[doc = "0: No interrupt"]
    Noint = 0,
}
impl From<Forcedatatoerr> for bool {
    #[inline(always)]
    fn from(variant: Forcedatatoerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCEDATATOERR` reader - Force Event for Data Timeout Error"]
pub type ForcedatatoerrR = crate::BitReader<Forcedatatoerr>;
impl ForcedatatoerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forcedatatoerr {
        match self.bits {
            true => Forcedatatoerr::Int,
            false => Forcedatatoerr::Noint,
        }
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == Forcedatatoerr::Int
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn is_noint(&self) -> bool {
        *self == Forcedatatoerr::Noint
    }
}
#[doc = "Field `FORCEDATATOERR` writer - Force Event for Data Timeout Error"]
pub type ForcedatatoerrW<'a, REG> = crate::BitWriter<'a, REG, Forcedatatoerr>;
impl<'a, REG> ForcedatatoerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(Forcedatatoerr::Int)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn noint(self) -> &'a mut crate::W<REG> {
        self.variant(Forcedatatoerr::Noint)
    }
}
#[doc = "Force Event for Data CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forcedatacrcerr {
    #[doc = "1: Interrupt is generated"]
    Int = 1,
    #[doc = "0: No interrupt"]
    Noint = 0,
}
impl From<Forcedatacrcerr> for bool {
    #[inline(always)]
    fn from(variant: Forcedatacrcerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCEDATACRCERR` reader - Force Event for Data CRC Error"]
pub type ForcedatacrcerrR = crate::BitReader<Forcedatacrcerr>;
impl ForcedatacrcerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forcedatacrcerr {
        match self.bits {
            true => Forcedatacrcerr::Int,
            false => Forcedatacrcerr::Noint,
        }
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == Forcedatacrcerr::Int
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn is_noint(&self) -> bool {
        *self == Forcedatacrcerr::Noint
    }
}
#[doc = "Field `FORCEDATACRCERR` writer - Force Event for Data CRC Error"]
pub type ForcedatacrcerrW<'a, REG> = crate::BitWriter<'a, REG, Forcedatacrcerr>;
impl<'a, REG> ForcedatacrcerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(Forcedatacrcerr::Int)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn noint(self) -> &'a mut crate::W<REG> {
        self.variant(Forcedatacrcerr::Noint)
    }
}
#[doc = "Force Event for Data End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forcedataenderr {
    #[doc = "1: Interrupt is generated"]
    Int = 1,
    #[doc = "0: No interrupt"]
    Noint = 0,
}
impl From<Forcedataenderr> for bool {
    #[inline(always)]
    fn from(variant: Forcedataenderr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCEDATAENDERR` reader - Force Event for Data End Bit Error"]
pub type ForcedataenderrR = crate::BitReader<Forcedataenderr>;
impl ForcedataenderrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forcedataenderr {
        match self.bits {
            true => Forcedataenderr::Int,
            false => Forcedataenderr::Noint,
        }
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == Forcedataenderr::Int
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn is_noint(&self) -> bool {
        *self == Forcedataenderr::Noint
    }
}
#[doc = "Field `FORCEDATAENDERR` writer - Force Event for Data End Bit Error"]
pub type ForcedataenderrW<'a, REG> = crate::BitWriter<'a, REG, Forcedataenderr>;
impl<'a, REG> ForcedataenderrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(Forcedataenderr::Int)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn noint(self) -> &'a mut crate::W<REG> {
        self.variant(Forcedataenderr::Noint)
    }
}
#[doc = "Force Event for Current Limit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forcecurrlimiterr {
    #[doc = "1: Interrupt is generated"]
    Int = 1,
    #[doc = "0: No interrupt"]
    Noint = 0,
}
impl From<Forcecurrlimiterr> for bool {
    #[inline(always)]
    fn from(variant: Forcecurrlimiterr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCECURRLIMITERR` reader - Force Event for Current Limit Error"]
pub type ForcecurrlimiterrR = crate::BitReader<Forcecurrlimiterr>;
impl ForcecurrlimiterrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forcecurrlimiterr {
        match self.bits {
            true => Forcecurrlimiterr::Int,
            false => Forcecurrlimiterr::Noint,
        }
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == Forcecurrlimiterr::Int
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn is_noint(&self) -> bool {
        *self == Forcecurrlimiterr::Noint
    }
}
#[doc = "Field `FORCECURRLIMITERR` writer - Force Event for Current Limit Error"]
pub type ForcecurrlimiterrW<'a, REG> = crate::BitWriter<'a, REG, Forcecurrlimiterr>;
impl<'a, REG> ForcecurrlimiterrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(Forcecurrlimiterr::Int)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn noint(self) -> &'a mut crate::W<REG> {
        self.variant(Forcecurrlimiterr::Noint)
    }
}
#[doc = "Force Event for Auto CMD Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forceacmderr {
    #[doc = "1: Interrupt is generated"]
    Int = 1,
    #[doc = "0: No interrupt"]
    Noint = 0,
}
impl From<Forceacmderr> for bool {
    #[inline(always)]
    fn from(variant: Forceacmderr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCEACMDERR` reader - Force Event for Auto CMD Error"]
pub type ForceacmderrR = crate::BitReader<Forceacmderr>;
impl ForceacmderrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forceacmderr {
        match self.bits {
            true => Forceacmderr::Int,
            false => Forceacmderr::Noint,
        }
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == Forceacmderr::Int
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn is_noint(&self) -> bool {
        *self == Forceacmderr::Noint
    }
}
#[doc = "Field `FORCEACMDERR` writer - Force Event for Auto CMD Error"]
pub type ForceacmderrW<'a, REG> = crate::BitWriter<'a, REG, Forceacmderr>;
impl<'a, REG> ForceacmderrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(Forceacmderr::Int)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn noint(self) -> &'a mut crate::W<REG> {
        self.variant(Forceacmderr::Noint)
    }
}
#[doc = "Force event for ADMA error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forceadmaerr {
    #[doc = "1: Interrupt is generated"]
    Int = 1,
    #[doc = "0: No interrupt"]
    Noint = 0,
}
impl From<Forceadmaerr> for bool {
    #[inline(always)]
    fn from(variant: Forceadmaerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCEADMAERR` reader - Force event for ADMA error"]
pub type ForceadmaerrR = crate::BitReader<Forceadmaerr>;
impl ForceadmaerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forceadmaerr {
        match self.bits {
            true => Forceadmaerr::Int,
            false => Forceadmaerr::Noint,
        }
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == Forceadmaerr::Int
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn is_noint(&self) -> bool {
        *self == Forceadmaerr::Noint
    }
}
#[doc = "Field `FORCEADMAERR` writer - Force event for ADMA error"]
pub type ForceadmaerrW<'a, REG> = crate::BitWriter<'a, REG, Forceadmaerr>;
impl<'a, REG> ForceadmaerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(Forceadmaerr::Int)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn noint(self) -> &'a mut crate::W<REG> {
        self.variant(Forceadmaerr::Noint)
    }
}
impl R {
    #[doc = "Bit 0 - Description"]
    #[inline(always)]
    pub fn forceacmd12not(&self) -> Forceacmd12notR {
        Forceacmd12notR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Description"]
    #[inline(always)]
    pub fn forceacmdtoerr(&self) -> ForceacmdtoerrR {
        ForceacmdtoerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Description"]
    #[inline(always)]
    pub fn forceacmdcrcerr(&self) -> ForceacmdcrcerrR {
        ForceacmdcrcerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Description"]
    #[inline(always)]
    pub fn forceacmdenderr(&self) -> ForceacmdenderrR {
        ForceacmdenderrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Desc"]
    #[inline(always)]
    pub fn forceacmdidxerr(&self) -> ForceacmdidxerrR {
        ForceacmdidxerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - 1 - Interrupt is generated"]
    #[inline(always)]
    pub fn forceacmdissuederr(&self) -> ForceacmdissuederrR {
        ForceacmdissuederrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Force Event for Command Timeout Error"]
    #[inline(always)]
    pub fn forcecmdtoerr(&self) -> ForcecmdtoerrR {
        ForcecmdtoerrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Force Event for Command CRC Error"]
    #[inline(always)]
    pub fn forcecmdcrcerr(&self) -> ForcecmdcrcerrR {
        ForcecmdcrcerrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Force Event for Command End Bit Error"]
    #[inline(always)]
    pub fn forcecmdenderr(&self) -> ForcecmdenderrR {
        ForcecmdenderrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Force Event for Command Index Error"]
    #[inline(always)]
    pub fn forcecmdidxerr(&self) -> ForcecmdidxerrR {
        ForcecmdidxerrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Force Event for Data Timeout Error"]
    #[inline(always)]
    pub fn forcedatatoerr(&self) -> ForcedatatoerrR {
        ForcedatatoerrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Force Event for Data CRC Error"]
    #[inline(always)]
    pub fn forcedatacrcerr(&self) -> ForcedatacrcerrR {
        ForcedatacrcerrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Force Event for Data End Bit Error"]
    #[inline(always)]
    pub fn forcedataenderr(&self) -> ForcedataenderrR {
        ForcedataenderrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Force Event for Current Limit Error"]
    #[inline(always)]
    pub fn forcecurrlimiterr(&self) -> ForcecurrlimiterrR {
        ForcecurrlimiterrR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Force Event for Auto CMD Error"]
    #[inline(always)]
    pub fn forceacmderr(&self) -> ForceacmderrR {
        ForceacmderrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Force event for ADMA error"]
    #[inline(always)]
    pub fn forceadmaerr(&self) -> ForceadmaerrR {
        ForceadmaerrR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Description"]
    #[inline(always)]
    #[must_use]
    pub fn forceacmd12not(&mut self) -> Forceacmd12notW<ForceSpec> {
        Forceacmd12notW::new(self, 0)
    }
    #[doc = "Bit 1 - Description"]
    #[inline(always)]
    #[must_use]
    pub fn forceacmdtoerr(&mut self) -> ForceacmdtoerrW<ForceSpec> {
        ForceacmdtoerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Description"]
    #[inline(always)]
    #[must_use]
    pub fn forceacmdcrcerr(&mut self) -> ForceacmdcrcerrW<ForceSpec> {
        ForceacmdcrcerrW::new(self, 2)
    }
    #[doc = "Bit 3 - Description"]
    #[inline(always)]
    #[must_use]
    pub fn forceacmdenderr(&mut self) -> ForceacmdenderrW<ForceSpec> {
        ForceacmdenderrW::new(self, 3)
    }
    #[doc = "Bit 4 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn forceacmdidxerr(&mut self) -> ForceacmdidxerrW<ForceSpec> {
        ForceacmdidxerrW::new(self, 4)
    }
    #[doc = "Bit 7 - 1 - Interrupt is generated"]
    #[inline(always)]
    #[must_use]
    pub fn forceacmdissuederr(&mut self) -> ForceacmdissuederrW<ForceSpec> {
        ForceacmdissuederrW::new(self, 7)
    }
    #[doc = "Bit 16 - Force Event for Command Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn forcecmdtoerr(&mut self) -> ForcecmdtoerrW<ForceSpec> {
        ForcecmdtoerrW::new(self, 16)
    }
    #[doc = "Bit 17 - Force Event for Command CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn forcecmdcrcerr(&mut self) -> ForcecmdcrcerrW<ForceSpec> {
        ForcecmdcrcerrW::new(self, 17)
    }
    #[doc = "Bit 18 - Force Event for Command End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn forcecmdenderr(&mut self) -> ForcecmdenderrW<ForceSpec> {
        ForcecmdenderrW::new(self, 18)
    }
    #[doc = "Bit 19 - Force Event for Command Index Error"]
    #[inline(always)]
    #[must_use]
    pub fn forcecmdidxerr(&mut self) -> ForcecmdidxerrW<ForceSpec> {
        ForcecmdidxerrW::new(self, 19)
    }
    #[doc = "Bit 20 - Force Event for Data Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn forcedatatoerr(&mut self) -> ForcedatatoerrW<ForceSpec> {
        ForcedatatoerrW::new(self, 20)
    }
    #[doc = "Bit 21 - Force Event for Data CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn forcedatacrcerr(&mut self) -> ForcedatacrcerrW<ForceSpec> {
        ForcedatacrcerrW::new(self, 21)
    }
    #[doc = "Bit 22 - Force Event for Data End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn forcedataenderr(&mut self) -> ForcedataenderrW<ForceSpec> {
        ForcedataenderrW::new(self, 22)
    }
    #[doc = "Bit 23 - Force Event for Current Limit Error"]
    #[inline(always)]
    #[must_use]
    pub fn forcecurrlimiterr(&mut self) -> ForcecurrlimiterrW<ForceSpec> {
        ForcecurrlimiterrW::new(self, 23)
    }
    #[doc = "Bit 24 - Force Event for Auto CMD Error"]
    #[inline(always)]
    #[must_use]
    pub fn forceacmderr(&mut self) -> ForceacmderrW<ForceSpec> {
        ForceacmderrW::new(self, 24)
    }
    #[doc = "Bit 25 - Force event for ADMA error"]
    #[inline(always)]
    #[must_use]
    pub fn forceadmaerr(&mut self) -> ForceadmaerrW<ForceSpec> {
        ForceadmaerrW::new(self, 25)
    }
}
#[doc = "Force event register for error interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`force::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ForceSpec;
impl crate::RegisterSpec for ForceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`force::R`](R) reader structure"]
impl crate::Readable for ForceSpec {}
#[doc = "`write(|w| ..)` method takes [`force::W`](W) writer structure"]
impl crate::Writable for ForceSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FORCE to value 0"]
impl crate::Resettable for ForceSpec {
    const RESET_VALUE: u32 = 0;
}
