#[doc = "Register `INTSIG` reader"]
pub type R = crate::R<IntsigSpec>;
#[doc = "Register `INTSIG` writer"]
pub type W = crate::W<IntsigSpec>;
#[doc = "Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdcmpen {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Cmdcmpen> for bool {
    #[inline(always)]
    fn from(variant: Cmdcmpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDCMPEN` reader - Interrupt"]
pub type CmdcmpenR = crate::BitReader<Cmdcmpen>;
impl CmdcmpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdcmpen {
        match self.bits {
            false => Cmdcmpen::Masked,
            true => Cmdcmpen::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Cmdcmpen::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cmdcmpen::Enabled
    }
}
#[doc = "Field `CMDCMPEN` writer - Interrupt"]
pub type CmdcmpenW<'a, REG> = crate::BitWriter<'a, REG, Cmdcmpen>;
impl<'a, REG> CmdcmpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdcmpen::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdcmpen::Enabled)
    }
}
#[doc = "Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xfercmpen {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Xfercmpen> for bool {
    #[inline(always)]
    fn from(variant: Xfercmpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XFERCMPEN` reader - Interrupt"]
pub type XfercmpenR = crate::BitReader<Xfercmpen>;
impl XfercmpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xfercmpen {
        match self.bits {
            false => Xfercmpen::Masked,
            true => Xfercmpen::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Xfercmpen::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Xfercmpen::Enabled
    }
}
#[doc = "Field `XFERCMPEN` writer - Interrupt"]
pub type XfercmpenW<'a, REG> = crate::BitWriter<'a, REG, Xfercmpen>;
impl<'a, REG> XfercmpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Xfercmpen::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Xfercmpen::Enabled)
    }
}
#[doc = "Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blockgapen {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Blockgapen> for bool {
    #[inline(always)]
    fn from(variant: Blockgapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLOCKGAPEN` reader - Interrupt"]
pub type BlockgapenR = crate::BitReader<Blockgapen>;
impl BlockgapenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blockgapen {
        match self.bits {
            false => Blockgapen::Masked,
            true => Blockgapen::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Blockgapen::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Blockgapen::Enabled
    }
}
#[doc = "Field `BLOCKGAPEN` writer - Interrupt"]
pub type BlockgapenW<'a, REG> = crate::BitWriter<'a, REG, Blockgapen>;
impl<'a, REG> BlockgapenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Blockgapen::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Blockgapen::Enabled)
    }
}
#[doc = "Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmainten {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Dmainten> for bool {
    #[inline(always)]
    fn from(variant: Dmainten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAINTEN` reader - Interrupt"]
pub type DmaintenR = crate::BitReader<Dmainten>;
impl DmaintenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmainten {
        match self.bits {
            false => Dmainten::Masked,
            true => Dmainten::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Dmainten::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmainten::Enabled
    }
}
#[doc = "Field `DMAINTEN` writer - Interrupt"]
pub type DmaintenW<'a, REG> = crate::BitWriter<'a, REG, Dmainten>;
impl<'a, REG> DmaintenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Dmainten::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmainten::Enabled)
    }
}
#[doc = "Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufferwren {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Bufferwren> for bool {
    #[inline(always)]
    fn from(variant: Bufferwren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFFERWREN` reader - Interrupt"]
pub type BufferwrenR = crate::BitReader<Bufferwren>;
impl BufferwrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bufferwren {
        match self.bits {
            false => Bufferwren::Masked,
            true => Bufferwren::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Bufferwren::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Bufferwren::Enabled
    }
}
#[doc = "Field `BUFFERWREN` writer - Interrupt"]
pub type BufferwrenW<'a, REG> = crate::BitWriter<'a, REG, Bufferwren>;
impl<'a, REG> BufferwrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferwren::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferwren::Enabled)
    }
}
#[doc = "Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufferrden {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Bufferrden> for bool {
    #[inline(always)]
    fn from(variant: Bufferrden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFFERRDEN` reader - Interrupt"]
pub type BufferrdenR = crate::BitReader<Bufferrden>;
impl BufferrdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bufferrden {
        match self.bits {
            false => Bufferrden::Masked,
            true => Bufferrden::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Bufferrden::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Bufferrden::Enabled
    }
}
#[doc = "Field `BUFFERRDEN` writer - Interrupt"]
pub type BufferrdenW<'a, REG> = crate::BitWriter<'a, REG, Bufferrden>;
impl<'a, REG> BufferrdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferrden::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferrden::Enabled)
    }
}
#[doc = "Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cardinserten {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Cardinserten> for bool {
    #[inline(always)]
    fn from(variant: Cardinserten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDINSERTEN` reader - Interrupt"]
pub type CardinsertenR = crate::BitReader<Cardinserten>;
impl CardinsertenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cardinserten {
        match self.bits {
            false => Cardinserten::Masked,
            true => Cardinserten::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Cardinserten::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cardinserten::Enabled
    }
}
#[doc = "Field `CARDINSERTEN` writer - Interrupt"]
pub type CardinsertenW<'a, REG> = crate::BitWriter<'a, REG, Cardinserten>;
impl<'a, REG> CardinsertenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Cardinserten::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cardinserten::Enabled)
    }
}
#[doc = "Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cardremovalen {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Cardremovalen> for bool {
    #[inline(always)]
    fn from(variant: Cardremovalen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDREMOVALEN` reader - Interrupt"]
pub type CardremovalenR = crate::BitReader<Cardremovalen>;
impl CardremovalenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cardremovalen {
        match self.bits {
            false => Cardremovalen::Masked,
            true => Cardremovalen::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Cardremovalen::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cardremovalen::Enabled
    }
}
#[doc = "Field `CARDREMOVALEN` writer - Interrupt"]
pub type CardremovalenW<'a, REG> = crate::BitWriter<'a, REG, Cardremovalen>;
impl<'a, REG> CardremovalenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Cardremovalen::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cardremovalen::Enabled)
    }
}
#[doc = "Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cardinten {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Cardinten> for bool {
    #[inline(always)]
    fn from(variant: Cardinten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDINTEN` reader - Interrupt"]
pub type CardintenR = crate::BitReader<Cardinten>;
impl CardintenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cardinten {
        match self.bits {
            false => Cardinten::Masked,
            true => Cardinten::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Cardinten::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cardinten::Enabled
    }
}
#[doc = "Field `CARDINTEN` writer - Interrupt"]
pub type CardintenW<'a, REG> = crate::BitWriter<'a, REG, Cardinten>;
impl<'a, REG> CardintenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Cardinten::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cardinten::Enabled)
    }
}
#[doc = "Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intaen {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Intaen> for bool {
    #[inline(always)]
    fn from(variant: Intaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTAEN` reader - Interrupt"]
pub type IntaenR = crate::BitReader<Intaen>;
impl IntaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intaen {
        match self.bits {
            false => Intaen::Masked,
            true => Intaen::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Intaen::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Intaen::Enabled
    }
}
#[doc = "Field `INTAEN` writer - Interrupt"]
pub type IntaenW<'a, REG> = crate::BitWriter<'a, REG, Intaen>;
impl<'a, REG> IntaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Intaen::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Intaen::Enabled)
    }
}
#[doc = "Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intben {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Intben> for bool {
    #[inline(always)]
    fn from(variant: Intben) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTBEN` reader - Interrupt"]
pub type IntbenR = crate::BitReader<Intben>;
impl IntbenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intben {
        match self.bits {
            false => Intben::Masked,
            true => Intben::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Intben::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Intben::Enabled
    }
}
#[doc = "Field `INTBEN` writer - Interrupt"]
pub type IntbenW<'a, REG> = crate::BitWriter<'a, REG, Intben>;
impl<'a, REG> IntbenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Intben::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Intben::Enabled)
    }
}
#[doc = "Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intcen {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Intcen> for bool {
    #[inline(always)]
    fn from(variant: Intcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTCEN` reader - Interrupt"]
pub type IntcenR = crate::BitReader<Intcen>;
impl IntcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intcen {
        match self.bits {
            false => Intcen::Masked,
            true => Intcen::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Intcen::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Intcen::Enabled
    }
}
#[doc = "Field `INTCEN` writer - Interrupt"]
pub type IntcenW<'a, REG> = crate::BitWriter<'a, REG, Intcen>;
impl<'a, REG> IntcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Intcen::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Intcen::Enabled)
    }
}
#[doc = "Interrupt signal enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Retuneeventen {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Retuneeventen> for bool {
    #[inline(always)]
    fn from(variant: Retuneeventen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RETUNEEVENTEN` reader - Interrupt signal enable"]
pub type RetuneeventenR = crate::BitReader<Retuneeventen>;
impl RetuneeventenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Retuneeventen {
        match self.bits {
            false => Retuneeventen::Masked,
            true => Retuneeventen::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Retuneeventen::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Retuneeventen::Enabled
    }
}
#[doc = "Field `RETUNEEVENTEN` writer - Interrupt signal enable"]
pub type RetuneeventenW<'a, REG> = crate::BitWriter<'a, REG, Retuneeventen>;
impl<'a, REG> RetuneeventenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Retuneeventen::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Retuneeventen::Enabled)
    }
}
#[doc = "Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bootacken {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Bootacken> for bool {
    #[inline(always)]
    fn from(variant: Bootacken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOTACKEN` reader - Interrupt"]
pub type BootackenR = crate::BitReader<Bootacken>;
impl BootackenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bootacken {
        match self.bits {
            false => Bootacken::Masked,
            true => Bootacken::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Bootacken::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Bootacken::Enabled
    }
}
#[doc = "Field `BOOTACKEN` writer - Interrupt"]
pub type BootackenW<'a, REG> = crate::BitWriter<'a, REG, Bootacken>;
impl<'a, REG> BootackenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Bootacken::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Bootacken::Enabled)
    }
}
#[doc = "Boot terminate interrupt signal enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bootterm {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Bootterm> for bool {
    #[inline(always)]
    fn from(variant: Bootterm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOTTERM` reader - Boot terminate interrupt signal enable"]
pub type BoottermR = crate::BitReader<Bootterm>;
impl BoottermR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bootterm {
        match self.bits {
            false => Bootterm::Masked,
            true => Bootterm::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Bootterm::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Bootterm::Enabled
    }
}
#[doc = "Field `BOOTTERM` writer - Boot terminate interrupt signal enable"]
pub type BoottermW<'a, REG> = crate::BitWriter<'a, REG, Bootterm>;
impl<'a, REG> BoottermW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Bootterm::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Bootterm::Enabled)
    }
}
#[doc = "Fixed to 0. The HD shall control error Interrupts using the Error Interrupt Signal Enable register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fixed0 {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Fixed0> for bool {
    #[inline(always)]
    fn from(variant: Fixed0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIXED0` reader - Fixed to 0. The HD shall control error Interrupts using the Error Interrupt Signal Enable register."]
pub type Fixed0R = crate::BitReader<Fixed0>;
impl Fixed0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fixed0 {
        match self.bits {
            false => Fixed0::Masked,
            true => Fixed0::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Fixed0::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fixed0::Enabled
    }
}
#[doc = "Field `FIXED0` writer - Fixed to 0. The HD shall control error Interrupts using the Error Interrupt Signal Enable register."]
pub type Fixed0W<'a, REG> = crate::BitWriter<'a, REG, Fixed0>;
impl<'a, REG> Fixed0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Fixed0::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fixed0::Enabled)
    }
}
#[doc = "Desc\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdtoerren {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Cmdtoerren> for bool {
    #[inline(always)]
    fn from(variant: Cmdtoerren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDTOERREN` reader - Desc"]
pub type CmdtoerrenR = crate::BitReader<Cmdtoerren>;
impl CmdtoerrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdtoerren {
        match self.bits {
            false => Cmdtoerren::Masked,
            true => Cmdtoerren::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Cmdtoerren::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cmdtoerren::Enabled
    }
}
#[doc = "Field `CMDTOERREN` writer - Desc"]
pub type CmdtoerrenW<'a, REG> = crate::BitWriter<'a, REG, Cmdtoerren>;
impl<'a, REG> CmdtoerrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtoerren::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtoerren::Enabled)
    }
}
#[doc = "Desc\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdcrcerren {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Cmdcrcerren> for bool {
    #[inline(always)]
    fn from(variant: Cmdcrcerren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDCRCERREN` reader - Desc"]
pub type CmdcrcerrenR = crate::BitReader<Cmdcrcerren>;
impl CmdcrcerrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdcrcerren {
        match self.bits {
            false => Cmdcrcerren::Masked,
            true => Cmdcrcerren::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Cmdcrcerren::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cmdcrcerren::Enabled
    }
}
#[doc = "Field `CMDCRCERREN` writer - Desc"]
pub type CmdcrcerrenW<'a, REG> = crate::BitWriter<'a, REG, Cmdcrcerren>;
impl<'a, REG> CmdcrcerrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdcrcerren::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdcrcerren::Enabled)
    }
}
#[doc = "Desc\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdendbiterren {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Cmdendbiterren> for bool {
    #[inline(always)]
    fn from(variant: Cmdendbiterren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDENDBITERREN` reader - Desc"]
pub type CmdendbiterrenR = crate::BitReader<Cmdendbiterren>;
impl CmdendbiterrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdendbiterren {
        match self.bits {
            false => Cmdendbiterren::Masked,
            true => Cmdendbiterren::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Cmdendbiterren::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cmdendbiterren::Enabled
    }
}
#[doc = "Field `CMDENDBITERREN` writer - Desc"]
pub type CmdendbiterrenW<'a, REG> = crate::BitWriter<'a, REG, Cmdendbiterren>;
impl<'a, REG> CmdendbiterrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdendbiterren::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdendbiterren::Enabled)
    }
}
#[doc = "Desc\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdidxerren {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Cmdidxerren> for bool {
    #[inline(always)]
    fn from(variant: Cmdidxerren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDIDXERREN` reader - Desc"]
pub type CmdidxerrenR = crate::BitReader<Cmdidxerren>;
impl CmdidxerrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdidxerren {
        match self.bits {
            false => Cmdidxerren::Masked,
            true => Cmdidxerren::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Cmdidxerren::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cmdidxerren::Enabled
    }
}
#[doc = "Field `CMDIDXERREN` writer - Desc"]
pub type CmdidxerrenW<'a, REG> = crate::BitWriter<'a, REG, Cmdidxerren>;
impl<'a, REG> CmdidxerrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdidxerren::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdidxerren::Enabled)
    }
}
#[doc = "Desc\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datatoerroren {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Datatoerroren> for bool {
    #[inline(always)]
    fn from(variant: Datatoerroren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATATOERROREN` reader - Desc"]
pub type DatatoerrorenR = crate::BitReader<Datatoerroren>;
impl DatatoerrorenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datatoerroren {
        match self.bits {
            false => Datatoerroren::Masked,
            true => Datatoerroren::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Datatoerroren::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Datatoerroren::Enabled
    }
}
#[doc = "Field `DATATOERROREN` writer - Desc"]
pub type DatatoerrorenW<'a, REG> = crate::BitWriter<'a, REG, Datatoerroren>;
impl<'a, REG> DatatoerrorenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Datatoerroren::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Datatoerroren::Enabled)
    }
}
#[doc = "Desc\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datacrcerren {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Datacrcerren> for bool {
    #[inline(always)]
    fn from(variant: Datacrcerren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATACRCERREN` reader - Desc"]
pub type DatacrcerrenR = crate::BitReader<Datacrcerren>;
impl DatacrcerrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datacrcerren {
        match self.bits {
            false => Datacrcerren::Masked,
            true => Datacrcerren::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Datacrcerren::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Datacrcerren::Enabled
    }
}
#[doc = "Field `DATACRCERREN` writer - Desc"]
pub type DatacrcerrenW<'a, REG> = crate::BitWriter<'a, REG, Datacrcerren>;
impl<'a, REG> DatacrcerrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Datacrcerren::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Datacrcerren::Enabled)
    }
}
#[doc = "Desc\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dataenderren {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Dataenderren> for bool {
    #[inline(always)]
    fn from(variant: Dataenderren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAENDERREN` reader - Desc"]
pub type DataenderrenR = crate::BitReader<Dataenderren>;
impl DataenderrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dataenderren {
        match self.bits {
            false => Dataenderren::Masked,
            true => Dataenderren::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Dataenderren::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dataenderren::Enabled
    }
}
#[doc = "Field `DATAENDERREN` writer - Desc"]
pub type DataenderrenW<'a, REG> = crate::BitWriter<'a, REG, Dataenderren>;
impl<'a, REG> DataenderrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Dataenderren::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dataenderren::Enabled)
    }
}
#[doc = "Desc\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Currlmterren {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Currlmterren> for bool {
    #[inline(always)]
    fn from(variant: Currlmterren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CURRLMTERREN` reader - Desc"]
pub type CurrlmterrenR = crate::BitReader<Currlmterren>;
impl CurrlmterrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Currlmterren {
        match self.bits {
            false => Currlmterren::Masked,
            true => Currlmterren::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Currlmterren::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Currlmterren::Enabled
    }
}
#[doc = "Field `CURRLMTERREN` writer - Desc"]
pub type CurrlmterrenW<'a, REG> = crate::BitWriter<'a, REG, Currlmterren>;
impl<'a, REG> CurrlmterrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Currlmterren::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Currlmterren::Enabled)
    }
}
#[doc = "Desc\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autocmd12erren {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Autocmd12erren> for bool {
    #[inline(always)]
    fn from(variant: Autocmd12erren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOCMD12ERREN` reader - Desc"]
pub type Autocmd12errenR = crate::BitReader<Autocmd12erren>;
impl Autocmd12errenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autocmd12erren {
        match self.bits {
            false => Autocmd12erren::Masked,
            true => Autocmd12erren::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Autocmd12erren::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Autocmd12erren::Enabled
    }
}
#[doc = "Field `AUTOCMD12ERREN` writer - Desc"]
pub type Autocmd12errenW<'a, REG> = crate::BitWriter<'a, REG, Autocmd12erren>;
impl<'a, REG> Autocmd12errenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Autocmd12erren::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Autocmd12erren::Enabled)
    }
}
#[doc = "Desc\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Admaerren {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Admaerren> for bool {
    #[inline(always)]
    fn from(variant: Admaerren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADMAERREN` reader - Desc"]
pub type AdmaerrenR = crate::BitReader<Admaerren>;
impl AdmaerrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Admaerren {
        match self.bits {
            false => Admaerren::Masked,
            true => Admaerren::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Admaerren::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Admaerren::Enabled
    }
}
#[doc = "Field `ADMAERREN` writer - Desc"]
pub type AdmaerrenW<'a, REG> = crate::BitWriter<'a, REG, Admaerren>;
impl<'a, REG> AdmaerrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Admaerren::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Admaerren::Enabled)
    }
}
#[doc = "Desc\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tuningerren {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Tuningerren> for bool {
    #[inline(always)]
    fn from(variant: Tuningerren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TUNINGERREN` reader - Desc"]
pub type TuningerrenR = crate::BitReader<Tuningerren>;
impl TuningerrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tuningerren {
        match self.bits {
            false => Tuningerren::Masked,
            true => Tuningerren::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Tuningerren::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tuningerren::Enabled
    }
}
#[doc = "Field `TUNINGERREN` writer - Desc"]
pub type TuningerrenW<'a, REG> = crate::BitWriter<'a, REG, Tuningerren>;
impl<'a, REG> TuningerrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Tuningerren::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tuningerren::Enabled)
    }
}
#[doc = "Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tgtrespen {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Tgtrespen> for bool {
    #[inline(always)]
    fn from(variant: Tgtrespen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TGTRESPEN` reader - Interrupt"]
pub type TgtrespenR = crate::BitReader<Tgtrespen>;
impl TgtrespenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tgtrespen {
        match self.bits {
            false => Tgtrespen::Masked,
            true => Tgtrespen::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Tgtrespen::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tgtrespen::Enabled
    }
}
#[doc = "Field `TGTRESPEN` writer - Interrupt"]
pub type TgtrespenW<'a, REG> = crate::BitWriter<'a, REG, Tgtrespen>;
impl<'a, REG> TgtrespenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Tgtrespen::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tgtrespen::Enabled)
    }
}
#[doc = "Field `VNDERREN` reader - VNDERREN field description needed here."]
pub type VnderrenR = crate::FieldReader;
#[doc = "Field `VNDERREN` writer - VNDERREN field description needed here."]
pub type VnderrenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Interrupt"]
    #[inline(always)]
    pub fn cmdcmpen(&self) -> CmdcmpenR {
        CmdcmpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt"]
    #[inline(always)]
    pub fn xfercmpen(&self) -> XfercmpenR {
        XfercmpenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt"]
    #[inline(always)]
    pub fn blockgapen(&self) -> BlockgapenR {
        BlockgapenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt"]
    #[inline(always)]
    pub fn dmainten(&self) -> DmaintenR {
        DmaintenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt"]
    #[inline(always)]
    pub fn bufferwren(&self) -> BufferwrenR {
        BufferwrenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt"]
    #[inline(always)]
    pub fn bufferrden(&self) -> BufferrdenR {
        BufferrdenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt"]
    #[inline(always)]
    pub fn cardinserten(&self) -> CardinsertenR {
        CardinsertenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt"]
    #[inline(always)]
    pub fn cardremovalen(&self) -> CardremovalenR {
        CardremovalenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt"]
    #[inline(always)]
    pub fn cardinten(&self) -> CardintenR {
        CardintenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt"]
    #[inline(always)]
    pub fn intaen(&self) -> IntaenR {
        IntaenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt"]
    #[inline(always)]
    pub fn intben(&self) -> IntbenR {
        IntbenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt"]
    #[inline(always)]
    pub fn intcen(&self) -> IntcenR {
        IntcenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt signal enable"]
    #[inline(always)]
    pub fn retuneeventen(&self) -> RetuneeventenR {
        RetuneeventenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt"]
    #[inline(always)]
    pub fn bootacken(&self) -> BootackenR {
        BootackenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Boot terminate interrupt signal enable"]
    #[inline(always)]
    pub fn bootterm(&self) -> BoottermR {
        BoottermR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Fixed to 0. The HD shall control error Interrupts using the Error Interrupt Signal Enable register."]
    #[inline(always)]
    pub fn fixed0(&self) -> Fixed0R {
        Fixed0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Desc"]
    #[inline(always)]
    pub fn cmdtoerren(&self) -> CmdtoerrenR {
        CmdtoerrenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Desc"]
    #[inline(always)]
    pub fn cmdcrcerren(&self) -> CmdcrcerrenR {
        CmdcrcerrenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Desc"]
    #[inline(always)]
    pub fn cmdendbiterren(&self) -> CmdendbiterrenR {
        CmdendbiterrenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Desc"]
    #[inline(always)]
    pub fn cmdidxerren(&self) -> CmdidxerrenR {
        CmdidxerrenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Desc"]
    #[inline(always)]
    pub fn datatoerroren(&self) -> DatatoerrorenR {
        DatatoerrorenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Desc"]
    #[inline(always)]
    pub fn datacrcerren(&self) -> DatacrcerrenR {
        DatacrcerrenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Desc"]
    #[inline(always)]
    pub fn dataenderren(&self) -> DataenderrenR {
        DataenderrenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Desc"]
    #[inline(always)]
    pub fn currlmterren(&self) -> CurrlmterrenR {
        CurrlmterrenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Desc"]
    #[inline(always)]
    pub fn autocmd12erren(&self) -> Autocmd12errenR {
        Autocmd12errenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Desc"]
    #[inline(always)]
    pub fn admaerren(&self) -> AdmaerrenR {
        AdmaerrenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Desc"]
    #[inline(always)]
    pub fn tuningerren(&self) -> TuningerrenR {
        TuningerrenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt"]
    #[inline(always)]
    pub fn tgtrespen(&self) -> TgtrespenR {
        TgtrespenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - VNDERREN field description needed here."]
    #[inline(always)]
    pub fn vnderren(&self) -> VnderrenR {
        VnderrenR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cmdcmpen(&mut self) -> CmdcmpenW<IntsigSpec> {
        CmdcmpenW::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn xfercmpen(&mut self) -> XfercmpenW<IntsigSpec> {
        XfercmpenW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn blockgapen(&mut self) -> BlockgapenW<IntsigSpec> {
        BlockgapenW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dmainten(&mut self) -> DmaintenW<IntsigSpec> {
        DmaintenW::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bufferwren(&mut self) -> BufferwrenW<IntsigSpec> {
        BufferwrenW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bufferrden(&mut self) -> BufferrdenW<IntsigSpec> {
        BufferrdenW::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cardinserten(&mut self) -> CardinsertenW<IntsigSpec> {
        CardinsertenW::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cardremovalen(&mut self) -> CardremovalenW<IntsigSpec> {
        CardremovalenW::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cardinten(&mut self) -> CardintenW<IntsigSpec> {
        CardintenW::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn intaen(&mut self) -> IntaenW<IntsigSpec> {
        IntaenW::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn intben(&mut self) -> IntbenW<IntsigSpec> {
        IntbenW::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn intcen(&mut self) -> IntcenW<IntsigSpec> {
        IntcenW::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt signal enable"]
    #[inline(always)]
    #[must_use]
    pub fn retuneeventen(&mut self) -> RetuneeventenW<IntsigSpec> {
        RetuneeventenW::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bootacken(&mut self) -> BootackenW<IntsigSpec> {
        BootackenW::new(self, 13)
    }
    #[doc = "Bit 14 - Boot terminate interrupt signal enable"]
    #[inline(always)]
    #[must_use]
    pub fn bootterm(&mut self) -> BoottermW<IntsigSpec> {
        BoottermW::new(self, 14)
    }
    #[doc = "Bit 15 - Fixed to 0. The HD shall control error Interrupts using the Error Interrupt Signal Enable register."]
    #[inline(always)]
    #[must_use]
    pub fn fixed0(&mut self) -> Fixed0W<IntsigSpec> {
        Fixed0W::new(self, 15)
    }
    #[doc = "Bit 16 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtoerren(&mut self) -> CmdtoerrenW<IntsigSpec> {
        CmdtoerrenW::new(self, 16)
    }
    #[doc = "Bit 17 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn cmdcrcerren(&mut self) -> CmdcrcerrenW<IntsigSpec> {
        CmdcrcerrenW::new(self, 17)
    }
    #[doc = "Bit 18 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn cmdendbiterren(&mut self) -> CmdendbiterrenW<IntsigSpec> {
        CmdendbiterrenW::new(self, 18)
    }
    #[doc = "Bit 19 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn cmdidxerren(&mut self) -> CmdidxerrenW<IntsigSpec> {
        CmdidxerrenW::new(self, 19)
    }
    #[doc = "Bit 20 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn datatoerroren(&mut self) -> DatatoerrorenW<IntsigSpec> {
        DatatoerrorenW::new(self, 20)
    }
    #[doc = "Bit 21 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn datacrcerren(&mut self) -> DatacrcerrenW<IntsigSpec> {
        DatacrcerrenW::new(self, 21)
    }
    #[doc = "Bit 22 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn dataenderren(&mut self) -> DataenderrenW<IntsigSpec> {
        DataenderrenW::new(self, 22)
    }
    #[doc = "Bit 23 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn currlmterren(&mut self) -> CurrlmterrenW<IntsigSpec> {
        CurrlmterrenW::new(self, 23)
    }
    #[doc = "Bit 24 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn autocmd12erren(&mut self) -> Autocmd12errenW<IntsigSpec> {
        Autocmd12errenW::new(self, 24)
    }
    #[doc = "Bit 25 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn admaerren(&mut self) -> AdmaerrenW<IntsigSpec> {
        AdmaerrenW::new(self, 25)
    }
    #[doc = "Bit 26 - Desc"]
    #[inline(always)]
    #[must_use]
    pub fn tuningerren(&mut self) -> TuningerrenW<IntsigSpec> {
        TuningerrenW::new(self, 26)
    }
    #[doc = "Bit 28 - Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tgtrespen(&mut self) -> TgtrespenW<IntsigSpec> {
        TgtrespenW::new(self, 28)
    }
    #[doc = "Bits 29:31 - VNDERREN field description needed here."]
    #[inline(always)]
    #[must_use]
    pub fn vnderren(&mut self) -> VnderrenW<IntsigSpec> {
        VnderrenW::new(self, 29)
    }
}
#[doc = "Normal interrupt signal enable\n\nYou can [`read`](crate::Reg::read) this register and get [`intsig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntsigSpec;
impl crate::RegisterSpec for IntsigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intsig::R`](R) reader structure"]
impl crate::Readable for IntsigSpec {}
#[doc = "`write(|w| ..)` method takes [`intsig::W`](W) writer structure"]
impl crate::Writable for IntsigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTSIG to value 0"]
impl crate::Resettable for IntsigSpec {
    const RESET_VALUE: u32 = 0;
}
