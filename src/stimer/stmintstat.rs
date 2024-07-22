#[doc = "Register `STMINTSTAT` reader"]
pub type R = crate::R<StmintstatSpec>;
#[doc = "Register `STMINTSTAT` writer"]
pub type W = crate::W<StmintstatSpec>;
#[doc = "COUNTER is greater than or equal to COMPARE register A.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparea {
    #[doc = "1: COUNTER greater than or equal to COMPARE register."]
    Compared = 1,
    #[doc = "0: COUNTER less than COMPARE register."]
    Default = 0,
}
impl From<Comparea> for bool {
    #[inline(always)]
    fn from(variant: Comparea) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREA` reader - COUNTER is greater than or equal to COMPARE register A."]
pub type CompareaR = crate::BitReader<Comparea>;
impl CompareaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Comparea {
        match self.bits {
            true => Comparea::Compared,
            false => Comparea::Default,
        }
    }
    #[doc = "COUNTER greater than or equal to COMPARE register."]
    #[inline(always)]
    pub fn is_compared(&self) -> bool {
        *self == Comparea::Compared
    }
    #[doc = "COUNTER less than COMPARE register."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Comparea::Default
    }
}
#[doc = "Field `COMPAREA` writer - COUNTER is greater than or equal to COMPARE register A."]
pub type CompareaW<'a, REG> = crate::BitWriter<'a, REG, Comparea>;
impl<'a, REG> CompareaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COUNTER greater than or equal to COMPARE register."]
    #[inline(always)]
    pub fn compared(self) -> &'a mut crate::W<REG> {
        self.variant(Comparea::Compared)
    }
    #[doc = "COUNTER less than COMPARE register."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Comparea::Default)
    }
}
#[doc = "COUNTER is greater than or equal to COMPARE register B.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compareb {
    #[doc = "1: COUNTER greater than or equal to COMPARE register."]
    Compared = 1,
    #[doc = "0: COUNTER less than COMPARE register."]
    Default = 0,
}
impl From<Compareb> for bool {
    #[inline(always)]
    fn from(variant: Compareb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREB` reader - COUNTER is greater than or equal to COMPARE register B."]
pub type ComparebR = crate::BitReader<Compareb>;
impl ComparebR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compareb {
        match self.bits {
            true => Compareb::Compared,
            false => Compareb::Default,
        }
    }
    #[doc = "COUNTER greater than or equal to COMPARE register."]
    #[inline(always)]
    pub fn is_compared(&self) -> bool {
        *self == Compareb::Compared
    }
    #[doc = "COUNTER less than COMPARE register."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Compareb::Default
    }
}
#[doc = "Field `COMPAREB` writer - COUNTER is greater than or equal to COMPARE register B."]
pub type ComparebW<'a, REG> = crate::BitWriter<'a, REG, Compareb>;
impl<'a, REG> ComparebW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COUNTER greater than or equal to COMPARE register."]
    #[inline(always)]
    pub fn compared(self) -> &'a mut crate::W<REG> {
        self.variant(Compareb::Compared)
    }
    #[doc = "COUNTER less than COMPARE register."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Compareb::Default)
    }
}
#[doc = "COUNTER is greater than or equal to COMPARE register C.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparec {
    #[doc = "1: COUNTER greater than or equal to COMPARE register."]
    Compared = 1,
    #[doc = "0: COUNTER less than COMPARE register."]
    Default = 0,
}
impl From<Comparec> for bool {
    #[inline(always)]
    fn from(variant: Comparec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREC` reader - COUNTER is greater than or equal to COMPARE register C."]
pub type ComparecR = crate::BitReader<Comparec>;
impl ComparecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Comparec {
        match self.bits {
            true => Comparec::Compared,
            false => Comparec::Default,
        }
    }
    #[doc = "COUNTER greater than or equal to COMPARE register."]
    #[inline(always)]
    pub fn is_compared(&self) -> bool {
        *self == Comparec::Compared
    }
    #[doc = "COUNTER less than COMPARE register."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Comparec::Default
    }
}
#[doc = "Field `COMPAREC` writer - COUNTER is greater than or equal to COMPARE register C."]
pub type ComparecW<'a, REG> = crate::BitWriter<'a, REG, Comparec>;
impl<'a, REG> ComparecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COUNTER greater than or equal to COMPARE register."]
    #[inline(always)]
    pub fn compared(self) -> &'a mut crate::W<REG> {
        self.variant(Comparec::Compared)
    }
    #[doc = "COUNTER less than COMPARE register."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Comparec::Default)
    }
}
#[doc = "COUNTER is greater than or equal to COMPARE register D.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compared {
    #[doc = "1: COUNTER greater than or equal to COMPARE register."]
    Compared = 1,
    #[doc = "0: COUNTER less than COMPARE register."]
    Default = 0,
}
impl From<Compared> for bool {
    #[inline(always)]
    fn from(variant: Compared) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARED` reader - COUNTER is greater than or equal to COMPARE register D."]
pub type ComparedR = crate::BitReader<Compared>;
impl ComparedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compared {
        match self.bits {
            true => Compared::Compared,
            false => Compared::Default,
        }
    }
    #[doc = "COUNTER greater than or equal to COMPARE register."]
    #[inline(always)]
    pub fn is_compared(&self) -> bool {
        *self == Compared::Compared
    }
    #[doc = "COUNTER less than COMPARE register."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Compared::Default
    }
}
#[doc = "Field `COMPARED` writer - COUNTER is greater than or equal to COMPARE register D."]
pub type ComparedW<'a, REG> = crate::BitWriter<'a, REG, Compared>;
impl<'a, REG> ComparedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COUNTER greater than or equal to COMPARE register."]
    #[inline(always)]
    pub fn compared(self) -> &'a mut crate::W<REG> {
        self.variant(Compared::Compared)
    }
    #[doc = "COUNTER less than COMPARE register."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Compared::Default)
    }
}
#[doc = "COUNTER is greater than or equal to COMPARE register E.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparee {
    #[doc = "1: COUNTER greater than or equal to COMPARE register."]
    Compared = 1,
    #[doc = "0: COUNTER less than COMPARE register."]
    Default = 0,
}
impl From<Comparee> for bool {
    #[inline(always)]
    fn from(variant: Comparee) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREE` reader - COUNTER is greater than or equal to COMPARE register E."]
pub type CompareeR = crate::BitReader<Comparee>;
impl CompareeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Comparee {
        match self.bits {
            true => Comparee::Compared,
            false => Comparee::Default,
        }
    }
    #[doc = "COUNTER greater than or equal to COMPARE register."]
    #[inline(always)]
    pub fn is_compared(&self) -> bool {
        *self == Comparee::Compared
    }
    #[doc = "COUNTER less than COMPARE register."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Comparee::Default
    }
}
#[doc = "Field `COMPAREE` writer - COUNTER is greater than or equal to COMPARE register E."]
pub type CompareeW<'a, REG> = crate::BitWriter<'a, REG, Comparee>;
impl<'a, REG> CompareeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COUNTER greater than or equal to COMPARE register."]
    #[inline(always)]
    pub fn compared(self) -> &'a mut crate::W<REG> {
        self.variant(Comparee::Compared)
    }
    #[doc = "COUNTER less than COMPARE register."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Comparee::Default)
    }
}
#[doc = "COUNTER is greater than or equal to COMPARE register F.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparef {
    #[doc = "1: COUNTER greater than or equal to COMPARE register."]
    Compared = 1,
    #[doc = "0: COUNTER less than COMPARE register."]
    Default = 0,
}
impl From<Comparef> for bool {
    #[inline(always)]
    fn from(variant: Comparef) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREF` reader - COUNTER is greater than or equal to COMPARE register F."]
pub type ComparefR = crate::BitReader<Comparef>;
impl ComparefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Comparef {
        match self.bits {
            true => Comparef::Compared,
            false => Comparef::Default,
        }
    }
    #[doc = "COUNTER greater than or equal to COMPARE register."]
    #[inline(always)]
    pub fn is_compared(&self) -> bool {
        *self == Comparef::Compared
    }
    #[doc = "COUNTER less than COMPARE register."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Comparef::Default
    }
}
#[doc = "Field `COMPAREF` writer - COUNTER is greater than or equal to COMPARE register F."]
pub type ComparefW<'a, REG> = crate::BitWriter<'a, REG, Comparef>;
impl<'a, REG> ComparefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COUNTER greater than or equal to COMPARE register."]
    #[inline(always)]
    pub fn compared(self) -> &'a mut crate::W<REG> {
        self.variant(Comparef::Compared)
    }
    #[doc = "COUNTER less than COMPARE register."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Comparef::Default)
    }
}
#[doc = "COUNTER is greater than or equal to COMPARE register G.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compareg {
    #[doc = "1: COUNTER greater than or equal to COMPARE register."]
    Compared = 1,
    #[doc = "0: COUNTER less than COMPARE register."]
    Default = 0,
}
impl From<Compareg> for bool {
    #[inline(always)]
    fn from(variant: Compareg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREG` reader - COUNTER is greater than or equal to COMPARE register G."]
pub type ComparegR = crate::BitReader<Compareg>;
impl ComparegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compareg {
        match self.bits {
            true => Compareg::Compared,
            false => Compareg::Default,
        }
    }
    #[doc = "COUNTER greater than or equal to COMPARE register."]
    #[inline(always)]
    pub fn is_compared(&self) -> bool {
        *self == Compareg::Compared
    }
    #[doc = "COUNTER less than COMPARE register."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Compareg::Default
    }
}
#[doc = "Field `COMPAREG` writer - COUNTER is greater than or equal to COMPARE register G."]
pub type ComparegW<'a, REG> = crate::BitWriter<'a, REG, Compareg>;
impl<'a, REG> ComparegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COUNTER greater than or equal to COMPARE register."]
    #[inline(always)]
    pub fn compared(self) -> &'a mut crate::W<REG> {
        self.variant(Compareg::Compared)
    }
    #[doc = "COUNTER less than COMPARE register."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Compareg::Default)
    }
}
#[doc = "COUNTER is greater than or equal to COMPARE register H.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compareh {
    #[doc = "1: COUNTER greater than or equal to COMPARE register."]
    Compared = 1,
    #[doc = "0: COUNTER less than COMPARE register."]
    Default = 0,
}
impl From<Compareh> for bool {
    #[inline(always)]
    fn from(variant: Compareh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREH` reader - COUNTER is greater than or equal to COMPARE register H."]
pub type ComparehR = crate::BitReader<Compareh>;
impl ComparehR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compareh {
        match self.bits {
            true => Compareh::Compared,
            false => Compareh::Default,
        }
    }
    #[doc = "COUNTER greater than or equal to COMPARE register."]
    #[inline(always)]
    pub fn is_compared(&self) -> bool {
        *self == Compareh::Compared
    }
    #[doc = "COUNTER less than COMPARE register."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Compareh::Default
    }
}
#[doc = "Field `COMPAREH` writer - COUNTER is greater than or equal to COMPARE register H."]
pub type ComparehW<'a, REG> = crate::BitWriter<'a, REG, Compareh>;
impl<'a, REG> ComparehW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COUNTER greater than or equal to COMPARE register."]
    #[inline(always)]
    pub fn compared(self) -> &'a mut crate::W<REG> {
        self.variant(Compareh::Compared)
    }
    #[doc = "COUNTER less than COMPARE register."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Compareh::Default)
    }
}
#[doc = "COUNTER over flowed from 0xFFFFFFFF back to 0x00000000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Overflow {
    #[doc = "1: Overflow interrupt status bit was set."]
    OflowInt = 1,
    #[doc = "0: Overflow interrupt status bit default/not set."]
    OflowDefault = 0,
}
impl From<Overflow> for bool {
    #[inline(always)]
    fn from(variant: Overflow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERFLOW` reader - COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
pub type OverflowR = crate::BitReader<Overflow>;
impl OverflowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Overflow {
        match self.bits {
            true => Overflow::OflowInt,
            false => Overflow::OflowDefault,
        }
    }
    #[doc = "Overflow interrupt status bit was set."]
    #[inline(always)]
    pub fn is_oflow_int(&self) -> bool {
        *self == Overflow::OflowInt
    }
    #[doc = "Overflow interrupt status bit default/not set."]
    #[inline(always)]
    pub fn is_oflow_default(&self) -> bool {
        *self == Overflow::OflowDefault
    }
}
#[doc = "Field `OVERFLOW` writer - COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
pub type OverflowW<'a, REG> = crate::BitWriter<'a, REG, Overflow>;
impl<'a, REG> OverflowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overflow interrupt status bit was set."]
    #[inline(always)]
    pub fn oflow_int(self) -> &'a mut crate::W<REG> {
        self.variant(Overflow::OflowInt)
    }
    #[doc = "Overflow interrupt status bit default/not set."]
    #[inline(always)]
    pub fn oflow_default(self) -> &'a mut crate::W<REG> {
        self.variant(Overflow::OflowDefault)
    }
}
#[doc = "CAPTURE register A has grabbed the value in the counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capturea {
    #[doc = "1: CAPTURE A interrupt status bit was set."]
    CapaInt = 1,
    #[doc = "0: CAPTURE A interrupt status default/not set."]
    CapaDefault = 0,
}
impl From<Capturea> for bool {
    #[inline(always)]
    fn from(variant: Capturea) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTUREA` reader - CAPTURE register A has grabbed the value in the counter"]
pub type CaptureaR = crate::BitReader<Capturea>;
impl CaptureaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Capturea {
        match self.bits {
            true => Capturea::CapaInt,
            false => Capturea::CapaDefault,
        }
    }
    #[doc = "CAPTURE A interrupt status bit was set."]
    #[inline(always)]
    pub fn is_capa_int(&self) -> bool {
        *self == Capturea::CapaInt
    }
    #[doc = "CAPTURE A interrupt status default/not set."]
    #[inline(always)]
    pub fn is_capa_default(&self) -> bool {
        *self == Capturea::CapaDefault
    }
}
#[doc = "Field `CAPTUREA` writer - CAPTURE register A has grabbed the value in the counter"]
pub type CaptureaW<'a, REG> = crate::BitWriter<'a, REG, Capturea>;
impl<'a, REG> CaptureaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CAPTURE A interrupt status bit was set."]
    #[inline(always)]
    pub fn capa_int(self) -> &'a mut crate::W<REG> {
        self.variant(Capturea::CapaInt)
    }
    #[doc = "CAPTURE A interrupt status default/not set."]
    #[inline(always)]
    pub fn capa_default(self) -> &'a mut crate::W<REG> {
        self.variant(Capturea::CapaDefault)
    }
}
#[doc = "CAPTURE register B has grabbed the value in the counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Captureb {
    #[doc = "1: CAPTURE B interrupt status bit was set."]
    CapbInt = 1,
    #[doc = "0: CAPTURE B interrupt status default/not set."]
    CapbDefault = 0,
}
impl From<Captureb> for bool {
    #[inline(always)]
    fn from(variant: Captureb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTUREB` reader - CAPTURE register B has grabbed the value in the counter"]
pub type CapturebR = crate::BitReader<Captureb>;
impl CapturebR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Captureb {
        match self.bits {
            true => Captureb::CapbInt,
            false => Captureb::CapbDefault,
        }
    }
    #[doc = "CAPTURE B interrupt status bit was set."]
    #[inline(always)]
    pub fn is_capb_int(&self) -> bool {
        *self == Captureb::CapbInt
    }
    #[doc = "CAPTURE B interrupt status default/not set."]
    #[inline(always)]
    pub fn is_capb_default(&self) -> bool {
        *self == Captureb::CapbDefault
    }
}
#[doc = "Field `CAPTUREB` writer - CAPTURE register B has grabbed the value in the counter"]
pub type CapturebW<'a, REG> = crate::BitWriter<'a, REG, Captureb>;
impl<'a, REG> CapturebW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CAPTURE B interrupt status bit was set."]
    #[inline(always)]
    pub fn capb_int(self) -> &'a mut crate::W<REG> {
        self.variant(Captureb::CapbInt)
    }
    #[doc = "CAPTURE B interrupt status default/not set."]
    #[inline(always)]
    pub fn capb_default(self) -> &'a mut crate::W<REG> {
        self.variant(Captureb::CapbDefault)
    }
}
#[doc = "CAPTURE register C has grabbed the value in the counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capturec {
    #[doc = "1: CAPTURE C interrupt status bit was set."]
    CapcInt = 1,
    #[doc = "0: CAPTURE C interrupt status default/not set."]
    CapcDefault = 0,
}
impl From<Capturec> for bool {
    #[inline(always)]
    fn from(variant: Capturec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTUREC` reader - CAPTURE register C has grabbed the value in the counter"]
pub type CapturecR = crate::BitReader<Capturec>;
impl CapturecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Capturec {
        match self.bits {
            true => Capturec::CapcInt,
            false => Capturec::CapcDefault,
        }
    }
    #[doc = "CAPTURE C interrupt status bit was set."]
    #[inline(always)]
    pub fn is_capc_int(&self) -> bool {
        *self == Capturec::CapcInt
    }
    #[doc = "CAPTURE C interrupt status default/not set."]
    #[inline(always)]
    pub fn is_capc_default(&self) -> bool {
        *self == Capturec::CapcDefault
    }
}
#[doc = "Field `CAPTUREC` writer - CAPTURE register C has grabbed the value in the counter"]
pub type CapturecW<'a, REG> = crate::BitWriter<'a, REG, Capturec>;
impl<'a, REG> CapturecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CAPTURE C interrupt status bit was set."]
    #[inline(always)]
    pub fn capc_int(self) -> &'a mut crate::W<REG> {
        self.variant(Capturec::CapcInt)
    }
    #[doc = "CAPTURE C interrupt status default/not set."]
    #[inline(always)]
    pub fn capc_default(self) -> &'a mut crate::W<REG> {
        self.variant(Capturec::CapcDefault)
    }
}
#[doc = "CAPTURE register D has grabbed the value in the counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Captured {
    #[doc = "1: Capture D interrupt status bit was set."]
    CapdInt = 1,
    #[doc = "0: Capture D interrupt status default/not set."]
    CapdDefault = 0,
}
impl From<Captured> for bool {
    #[inline(always)]
    fn from(variant: Captured) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTURED` reader - CAPTURE register D has grabbed the value in the counter"]
pub type CapturedR = crate::BitReader<Captured>;
impl CapturedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Captured {
        match self.bits {
            true => Captured::CapdInt,
            false => Captured::CapdDefault,
        }
    }
    #[doc = "Capture D interrupt status bit was set."]
    #[inline(always)]
    pub fn is_capd_int(&self) -> bool {
        *self == Captured::CapdInt
    }
    #[doc = "Capture D interrupt status default/not set."]
    #[inline(always)]
    pub fn is_capd_default(&self) -> bool {
        *self == Captured::CapdDefault
    }
}
#[doc = "Field `CAPTURED` writer - CAPTURE register D has grabbed the value in the counter"]
pub type CapturedW<'a, REG> = crate::BitWriter<'a, REG, Captured>;
impl<'a, REG> CapturedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture D interrupt status bit was set."]
    #[inline(always)]
    pub fn capd_int(self) -> &'a mut crate::W<REG> {
        self.variant(Captured::CapdInt)
    }
    #[doc = "Capture D interrupt status default/not set."]
    #[inline(always)]
    pub fn capd_default(self) -> &'a mut crate::W<REG> {
        self.variant(Captured::CapdDefault)
    }
}
impl R {
    #[doc = "Bit 0 - COUNTER is greater than or equal to COMPARE register A."]
    #[inline(always)]
    pub fn comparea(&self) -> CompareaR {
        CompareaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - COUNTER is greater than or equal to COMPARE register B."]
    #[inline(always)]
    pub fn compareb(&self) -> ComparebR {
        ComparebR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - COUNTER is greater than or equal to COMPARE register C."]
    #[inline(always)]
    pub fn comparec(&self) -> ComparecR {
        ComparecR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - COUNTER is greater than or equal to COMPARE register D."]
    #[inline(always)]
    pub fn compared(&self) -> ComparedR {
        ComparedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - COUNTER is greater than or equal to COMPARE register E."]
    #[inline(always)]
    pub fn comparee(&self) -> CompareeR {
        CompareeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COUNTER is greater than or equal to COMPARE register F."]
    #[inline(always)]
    pub fn comparef(&self) -> ComparefR {
        ComparefR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - COUNTER is greater than or equal to COMPARE register G."]
    #[inline(always)]
    pub fn compareg(&self) -> ComparegR {
        ComparegR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - COUNTER is greater than or equal to COMPARE register H."]
    #[inline(always)]
    pub fn compareh(&self) -> ComparehR {
        ComparehR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
    #[inline(always)]
    pub fn overflow(&self) -> OverflowR {
        OverflowR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CAPTURE register A has grabbed the value in the counter"]
    #[inline(always)]
    pub fn capturea(&self) -> CaptureaR {
        CaptureaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CAPTURE register B has grabbed the value in the counter"]
    #[inline(always)]
    pub fn captureb(&self) -> CapturebR {
        CapturebR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CAPTURE register C has grabbed the value in the counter"]
    #[inline(always)]
    pub fn capturec(&self) -> CapturecR {
        CapturecR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CAPTURE register D has grabbed the value in the counter"]
    #[inline(always)]
    pub fn captured(&self) -> CapturedR {
        CapturedR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - COUNTER is greater than or equal to COMPARE register A."]
    #[inline(always)]
    #[must_use]
    pub fn comparea(&mut self) -> CompareaW<StmintstatSpec> {
        CompareaW::new(self, 0)
    }
    #[doc = "Bit 1 - COUNTER is greater than or equal to COMPARE register B."]
    #[inline(always)]
    #[must_use]
    pub fn compareb(&mut self) -> ComparebW<StmintstatSpec> {
        ComparebW::new(self, 1)
    }
    #[doc = "Bit 2 - COUNTER is greater than or equal to COMPARE register C."]
    #[inline(always)]
    #[must_use]
    pub fn comparec(&mut self) -> ComparecW<StmintstatSpec> {
        ComparecW::new(self, 2)
    }
    #[doc = "Bit 3 - COUNTER is greater than or equal to COMPARE register D."]
    #[inline(always)]
    #[must_use]
    pub fn compared(&mut self) -> ComparedW<StmintstatSpec> {
        ComparedW::new(self, 3)
    }
    #[doc = "Bit 4 - COUNTER is greater than or equal to COMPARE register E."]
    #[inline(always)]
    #[must_use]
    pub fn comparee(&mut self) -> CompareeW<StmintstatSpec> {
        CompareeW::new(self, 4)
    }
    #[doc = "Bit 5 - COUNTER is greater than or equal to COMPARE register F."]
    #[inline(always)]
    #[must_use]
    pub fn comparef(&mut self) -> ComparefW<StmintstatSpec> {
        ComparefW::new(self, 5)
    }
    #[doc = "Bit 6 - COUNTER is greater than or equal to COMPARE register G."]
    #[inline(always)]
    #[must_use]
    pub fn compareg(&mut self) -> ComparegW<StmintstatSpec> {
        ComparegW::new(self, 6)
    }
    #[doc = "Bit 7 - COUNTER is greater than or equal to COMPARE register H."]
    #[inline(always)]
    #[must_use]
    pub fn compareh(&mut self) -> ComparehW<StmintstatSpec> {
        ComparehW::new(self, 7)
    }
    #[doc = "Bit 8 - COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
    #[inline(always)]
    #[must_use]
    pub fn overflow(&mut self) -> OverflowW<StmintstatSpec> {
        OverflowW::new(self, 8)
    }
    #[doc = "Bit 9 - CAPTURE register A has grabbed the value in the counter"]
    #[inline(always)]
    #[must_use]
    pub fn capturea(&mut self) -> CaptureaW<StmintstatSpec> {
        CaptureaW::new(self, 9)
    }
    #[doc = "Bit 10 - CAPTURE register B has grabbed the value in the counter"]
    #[inline(always)]
    #[must_use]
    pub fn captureb(&mut self) -> CapturebW<StmintstatSpec> {
        CapturebW::new(self, 10)
    }
    #[doc = "Bit 11 - CAPTURE register C has grabbed the value in the counter"]
    #[inline(always)]
    #[must_use]
    pub fn capturec(&mut self) -> CapturecW<StmintstatSpec> {
        CapturecW::new(self, 11)
    }
    #[doc = "Bit 12 - CAPTURE register D has grabbed the value in the counter"]
    #[inline(always)]
    #[must_use]
    pub fn captured(&mut self) -> CapturedW<StmintstatSpec> {
        CapturedW::new(self, 12)
    }
}
#[doc = "Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`stmintstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stmintstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StmintstatSpec;
impl crate::RegisterSpec for StmintstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stmintstat::R`](R) reader structure"]
impl crate::Readable for StmintstatSpec {}
#[doc = "`write(|w| ..)` method takes [`stmintstat::W`](W) writer structure"]
impl crate::Writable for StmintstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STMINTSTAT to value 0"]
impl crate::Resettable for StmintstatSpec {
    const RESET_VALUE: u32 = 0;
}
