#[doc = "Register `INTCLR` reader"]
pub type R = crate::R<IntclrSpec>;
#[doc = "Register `INTCLR` writer"]
pub type W = crate::W<IntclrSpec>;
#[doc = "ADC conversion complete interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cnvcmp {
    #[doc = "1: ADC conversion complete interrupt."]
    Cnvcmpint = 1,
    #[doc = "0: No ADC conversion complete interrupt."]
    Cnvcmpnoint = 0,
}
impl From<Cnvcmp> for bool {
    #[inline(always)]
    fn from(variant: Cnvcmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNVCMP` reader - ADC conversion complete interrupt."]
pub type CnvcmpR = crate::BitReader<Cnvcmp>;
impl CnvcmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cnvcmp {
        match self.bits {
            true => Cnvcmp::Cnvcmpint,
            false => Cnvcmp::Cnvcmpnoint,
        }
    }
    #[doc = "ADC conversion complete interrupt."]
    #[inline(always)]
    pub fn is_cnvcmpint(&self) -> bool {
        *self == Cnvcmp::Cnvcmpint
    }
    #[doc = "No ADC conversion complete interrupt."]
    #[inline(always)]
    pub fn is_cnvcmpnoint(&self) -> bool {
        *self == Cnvcmp::Cnvcmpnoint
    }
}
#[doc = "Field `CNVCMP` writer - ADC conversion complete interrupt."]
pub type CnvcmpW<'a, REG> = crate::BitWriter<'a, REG, Cnvcmp>;
impl<'a, REG> CnvcmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC conversion complete interrupt."]
    #[inline(always)]
    pub fn cnvcmpint(self) -> &'a mut crate::W<REG> {
        self.variant(Cnvcmp::Cnvcmpint)
    }
    #[doc = "No ADC conversion complete interrupt."]
    #[inline(always)]
    pub fn cnvcmpnoint(self) -> &'a mut crate::W<REG> {
        self.variant(Cnvcmp::Cnvcmpnoint)
    }
}
#[doc = "ADC scan complete interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scncmp {
    #[doc = "1: ADC scan complete interrupt."]
    Scncmpint = 1,
    #[doc = "0: No ADC scan complete interrupt."]
    Scncmpnoint = 0,
}
impl From<Scncmp> for bool {
    #[inline(always)]
    fn from(variant: Scncmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCNCMP` reader - ADC scan complete interrupt."]
pub type ScncmpR = crate::BitReader<Scncmp>;
impl ScncmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scncmp {
        match self.bits {
            true => Scncmp::Scncmpint,
            false => Scncmp::Scncmpnoint,
        }
    }
    #[doc = "ADC scan complete interrupt."]
    #[inline(always)]
    pub fn is_scncmpint(&self) -> bool {
        *self == Scncmp::Scncmpint
    }
    #[doc = "No ADC scan complete interrupt."]
    #[inline(always)]
    pub fn is_scncmpnoint(&self) -> bool {
        *self == Scncmp::Scncmpnoint
    }
}
#[doc = "Field `SCNCMP` writer - ADC scan complete interrupt."]
pub type ScncmpW<'a, REG> = crate::BitWriter<'a, REG, Scncmp>;
impl<'a, REG> ScncmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC scan complete interrupt."]
    #[inline(always)]
    pub fn scncmpint(self) -> &'a mut crate::W<REG> {
        self.variant(Scncmp::Scncmpint)
    }
    #[doc = "No ADC scan complete interrupt."]
    #[inline(always)]
    pub fn scncmpnoint(self) -> &'a mut crate::W<REG> {
        self.variant(Scncmp::Scncmpnoint)
    }
}
#[doc = "FIFO 75 percent full interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fifoovr1 {
    #[doc = "1: FIFO 75 percent full interrupt."]
    Fifo75int = 1,
    #[doc = "0: Not FIFO 75 percent full interrupt."]
    Fifo75noint = 0,
}
impl From<Fifoovr1> for bool {
    #[inline(always)]
    fn from(variant: Fifoovr1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOOVR1` reader - FIFO 75 percent full interrupt."]
pub type Fifoovr1R = crate::BitReader<Fifoovr1>;
impl Fifoovr1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fifoovr1 {
        match self.bits {
            true => Fifoovr1::Fifo75int,
            false => Fifoovr1::Fifo75noint,
        }
    }
    #[doc = "FIFO 75 percent full interrupt."]
    #[inline(always)]
    pub fn is_fifo75int(&self) -> bool {
        *self == Fifoovr1::Fifo75int
    }
    #[doc = "Not FIFO 75 percent full interrupt."]
    #[inline(always)]
    pub fn is_fifo75noint(&self) -> bool {
        *self == Fifoovr1::Fifo75noint
    }
}
#[doc = "Field `FIFOOVR1` writer - FIFO 75 percent full interrupt."]
pub type Fifoovr1W<'a, REG> = crate::BitWriter<'a, REG, Fifoovr1>;
impl<'a, REG> Fifoovr1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FIFO 75 percent full interrupt."]
    #[inline(always)]
    pub fn fifo75int(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoovr1::Fifo75int)
    }
    #[doc = "Not FIFO 75 percent full interrupt."]
    #[inline(always)]
    pub fn fifo75noint(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoovr1::Fifo75noint)
    }
}
#[doc = "FIFO 100 percent full interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fifoovr2 {
    #[doc = "1: FIFO 100 percent full interrupt."]
    Fifofullint = 1,
    #[doc = "0: Not a FIFO 100 percent full interrupt."]
    Fifofullnoint = 0,
}
impl From<Fifoovr2> for bool {
    #[inline(always)]
    fn from(variant: Fifoovr2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOOVR2` reader - FIFO 100 percent full interrupt."]
pub type Fifoovr2R = crate::BitReader<Fifoovr2>;
impl Fifoovr2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fifoovr2 {
        match self.bits {
            true => Fifoovr2::Fifofullint,
            false => Fifoovr2::Fifofullnoint,
        }
    }
    #[doc = "FIFO 100 percent full interrupt."]
    #[inline(always)]
    pub fn is_fifofullint(&self) -> bool {
        *self == Fifoovr2::Fifofullint
    }
    #[doc = "Not a FIFO 100 percent full interrupt."]
    #[inline(always)]
    pub fn is_fifofullnoint(&self) -> bool {
        *self == Fifoovr2::Fifofullnoint
    }
}
#[doc = "Field `FIFOOVR2` writer - FIFO 100 percent full interrupt."]
pub type Fifoovr2W<'a, REG> = crate::BitWriter<'a, REG, Fifoovr2>;
impl<'a, REG> Fifoovr2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FIFO 100 percent full interrupt."]
    #[inline(always)]
    pub fn fifofullint(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoovr2::Fifofullint)
    }
    #[doc = "Not a FIFO 100 percent full interrupt."]
    #[inline(always)]
    pub fn fifofullnoint(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoovr2::Fifofullnoint)
    }
}
#[doc = "Window comparator voltage excursion interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wcexc {
    #[doc = "1: Window comparator voltage excursion interrupt."]
    Wcexcint = 1,
    #[doc = "0: Not a Window comparator voltage excursion interrupt."]
    Wcexcnoint = 0,
}
impl From<Wcexc> for bool {
    #[inline(always)]
    fn from(variant: Wcexc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WCEXC` reader - Window comparator voltage excursion interrupt."]
pub type WcexcR = crate::BitReader<Wcexc>;
impl WcexcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wcexc {
        match self.bits {
            true => Wcexc::Wcexcint,
            false => Wcexc::Wcexcnoint,
        }
    }
    #[doc = "Window comparator voltage excursion interrupt."]
    #[inline(always)]
    pub fn is_wcexcint(&self) -> bool {
        *self == Wcexc::Wcexcint
    }
    #[doc = "Not a Window comparator voltage excursion interrupt."]
    #[inline(always)]
    pub fn is_wcexcnoint(&self) -> bool {
        *self == Wcexc::Wcexcnoint
    }
}
#[doc = "Field `WCEXC` writer - Window comparator voltage excursion interrupt."]
pub type WcexcW<'a, REG> = crate::BitWriter<'a, REG, Wcexc>;
impl<'a, REG> WcexcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Window comparator voltage excursion interrupt."]
    #[inline(always)]
    pub fn wcexcint(self) -> &'a mut crate::W<REG> {
        self.variant(Wcexc::Wcexcint)
    }
    #[doc = "Not a Window comparator voltage excursion interrupt."]
    #[inline(always)]
    pub fn wcexcnoint(self) -> &'a mut crate::W<REG> {
        self.variant(Wcexc::Wcexcnoint)
    }
}
#[doc = "Window comparator voltage incursion interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wcinc {
    #[doc = "1: Window comparator voltage incursion interrupt."]
    Wcincint = 1,
    #[doc = "0: Not a Window comparator voltage incursion interrupt."]
    Wcincnoint = 0,
}
impl From<Wcinc> for bool {
    #[inline(always)]
    fn from(variant: Wcinc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WCINC` reader - Window comparator voltage incursion interrupt."]
pub type WcincR = crate::BitReader<Wcinc>;
impl WcincR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wcinc {
        match self.bits {
            true => Wcinc::Wcincint,
            false => Wcinc::Wcincnoint,
        }
    }
    #[doc = "Window comparator voltage incursion interrupt."]
    #[inline(always)]
    pub fn is_wcincint(&self) -> bool {
        *self == Wcinc::Wcincint
    }
    #[doc = "Not a Window comparator voltage incursion interrupt."]
    #[inline(always)]
    pub fn is_wcincnoint(&self) -> bool {
        *self == Wcinc::Wcincnoint
    }
}
#[doc = "Field `WCINC` writer - Window comparator voltage incursion interrupt."]
pub type WcincW<'a, REG> = crate::BitWriter<'a, REG, Wcinc>;
impl<'a, REG> WcincW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Window comparator voltage incursion interrupt."]
    #[inline(always)]
    pub fn wcincint(self) -> &'a mut crate::W<REG> {
        self.variant(Wcinc::Wcincint)
    }
    #[doc = "Not a Window comparator voltage incursion interrupt."]
    #[inline(always)]
    pub fn wcincnoint(self) -> &'a mut crate::W<REG> {
        self.variant(Wcinc::Wcincnoint)
    }
}
#[doc = "DMA Transfer Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcmp {
    #[doc = "1: DMA Completed a transfer"]
    Dmacomplete = 1,
    #[doc = "0: DMA completion is pending or not triggered."]
    Dmaon = 0,
}
impl From<Dcmp> for bool {
    #[inline(always)]
    fn from(variant: Dcmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMP` reader - DMA Transfer Complete"]
pub type DcmpR = crate::BitReader<Dcmp>;
impl DcmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcmp {
        match self.bits {
            true => Dcmp::Dmacomplete,
            false => Dcmp::Dmaon,
        }
    }
    #[doc = "DMA Completed a transfer"]
    #[inline(always)]
    pub fn is_dmacomplete(&self) -> bool {
        *self == Dcmp::Dmacomplete
    }
    #[doc = "DMA completion is pending or not triggered."]
    #[inline(always)]
    pub fn is_dmaon(&self) -> bool {
        *self == Dcmp::Dmaon
    }
}
#[doc = "Field `DCMP` writer - DMA Transfer Complete"]
pub type DcmpW<'a, REG> = crate::BitWriter<'a, REG, Dcmp>;
impl<'a, REG> DcmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA Completed a transfer"]
    #[inline(always)]
    pub fn dmacomplete(self) -> &'a mut crate::W<REG> {
        self.variant(Dcmp::Dmacomplete)
    }
    #[doc = "DMA completion is pending or not triggered."]
    #[inline(always)]
    pub fn dmaon(self) -> &'a mut crate::W<REG> {
        self.variant(Dcmp::Dmaon)
    }
}
#[doc = "DMA Error Condition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Derr {
    #[doc = "1: DMA Error Condition Occurred"]
    Dmaerror = 1,
    #[doc = "0: DMA Error Condition did not Occurred"]
    Nodmaerror = 0,
}
impl From<Derr> for bool {
    #[inline(always)]
    fn from(variant: Derr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DERR` reader - DMA Error Condition"]
pub type DerrR = crate::BitReader<Derr>;
impl DerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Derr {
        match self.bits {
            true => Derr::Dmaerror,
            false => Derr::Nodmaerror,
        }
    }
    #[doc = "DMA Error Condition Occurred"]
    #[inline(always)]
    pub fn is_dmaerror(&self) -> bool {
        *self == Derr::Dmaerror
    }
    #[doc = "DMA Error Condition did not Occurred"]
    #[inline(always)]
    pub fn is_nodmaerror(&self) -> bool {
        *self == Derr::Nodmaerror
    }
}
#[doc = "Field `DERR` writer - DMA Error Condition"]
pub type DerrW<'a, REG> = crate::BitWriter<'a, REG, Derr>;
impl<'a, REG> DerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA Error Condition Occurred"]
    #[inline(always)]
    pub fn dmaerror(self) -> &'a mut crate::W<REG> {
        self.variant(Derr::Dmaerror)
    }
    #[doc = "DMA Error Condition did not Occurred"]
    #[inline(always)]
    pub fn nodmaerror(self) -> &'a mut crate::W<REG> {
        self.variant(Derr::Nodmaerror)
    }
}
#[doc = "Zero Crossing - Channel A (Slots 0 or 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Zxca {
    #[doc = "1: Zero Crossing, as specified by ZX configuration registers, occurred on either slot 0 or 1 (channel A)"]
    Zxcaint = 1,
    #[doc = "0: Non Zero Crossing"]
    Nonzxcaint = 0,
}
impl From<Zxca> for bool {
    #[inline(always)]
    fn from(variant: Zxca) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ZXCA` reader - Zero Crossing - Channel A (Slots 0 or 1)"]
pub type ZxcaR = crate::BitReader<Zxca>;
impl ZxcaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Zxca {
        match self.bits {
            true => Zxca::Zxcaint,
            false => Zxca::Nonzxcaint,
        }
    }
    #[doc = "Zero Crossing, as specified by ZX configuration registers, occurred on either slot 0 or 1 (channel A)"]
    #[inline(always)]
    pub fn is_zxcaint(&self) -> bool {
        *self == Zxca::Zxcaint
    }
    #[doc = "Non Zero Crossing"]
    #[inline(always)]
    pub fn is_nonzxcaint(&self) -> bool {
        *self == Zxca::Nonzxcaint
    }
}
#[doc = "Field `ZXCA` writer - Zero Crossing - Channel A (Slots 0 or 1)"]
pub type ZxcaW<'a, REG> = crate::BitWriter<'a, REG, Zxca>;
impl<'a, REG> ZxcaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Zero Crossing, as specified by ZX configuration registers, occurred on either slot 0 or 1 (channel A)"]
    #[inline(always)]
    pub fn zxcaint(self) -> &'a mut crate::W<REG> {
        self.variant(Zxca::Zxcaint)
    }
    #[doc = "Non Zero Crossing"]
    #[inline(always)]
    pub fn nonzxcaint(self) -> &'a mut crate::W<REG> {
        self.variant(Zxca::Nonzxcaint)
    }
}
#[doc = "Zero Crossing - Channel B (Slots 2 or 3)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Zxcb {
    #[doc = "1: Zero Crossing, as specified by ZX configuration registers, occurred on either slot 2 or 3 (channel B)"]
    Zxcbint = 1,
    #[doc = "0: Non Zero Crossing"]
    Nonzxcbint = 0,
}
impl From<Zxcb> for bool {
    #[inline(always)]
    fn from(variant: Zxcb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ZXCB` reader - Zero Crossing - Channel B (Slots 2 or 3)"]
pub type ZxcbR = crate::BitReader<Zxcb>;
impl ZxcbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Zxcb {
        match self.bits {
            true => Zxcb::Zxcbint,
            false => Zxcb::Nonzxcbint,
        }
    }
    #[doc = "Zero Crossing, as specified by ZX configuration registers, occurred on either slot 2 or 3 (channel B)"]
    #[inline(always)]
    pub fn is_zxcbint(&self) -> bool {
        *self == Zxcb::Zxcbint
    }
    #[doc = "Non Zero Crossing"]
    #[inline(always)]
    pub fn is_nonzxcbint(&self) -> bool {
        *self == Zxcb::Nonzxcbint
    }
}
#[doc = "Field `ZXCB` writer - Zero Crossing - Channel B (Slots 2 or 3)"]
pub type ZxcbW<'a, REG> = crate::BitWriter<'a, REG, Zxcb>;
impl<'a, REG> ZxcbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Zero Crossing, as specified by ZX configuration registers, occurred on either slot 2 or 3 (channel B)"]
    #[inline(always)]
    pub fn zxcbint(self) -> &'a mut crate::W<REG> {
        self.variant(Zxcb::Zxcbint)
    }
    #[doc = "Non Zero Crossing"]
    #[inline(always)]
    pub fn nonzxcbint(self) -> &'a mut crate::W<REG> {
        self.variant(Zxcb::Nonzxcbint)
    }
}
#[doc = "Saturation - Channel A (Slots 0 or 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Satca {
    #[doc = "1: Saturation, as specified by SAT configuration registers, occurred on either slot 0 or 1 (channel A)"]
    Satcaint = 1,
    #[doc = "0: No Saturation"]
    Nonsatcaint = 0,
}
impl From<Satca> for bool {
    #[inline(always)]
    fn from(variant: Satca) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SATCA` reader - Saturation - Channel A (Slots 0 or 1)"]
pub type SatcaR = crate::BitReader<Satca>;
impl SatcaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Satca {
        match self.bits {
            true => Satca::Satcaint,
            false => Satca::Nonsatcaint,
        }
    }
    #[doc = "Saturation, as specified by SAT configuration registers, occurred on either slot 0 or 1 (channel A)"]
    #[inline(always)]
    pub fn is_satcaint(&self) -> bool {
        *self == Satca::Satcaint
    }
    #[doc = "No Saturation"]
    #[inline(always)]
    pub fn is_nonsatcaint(&self) -> bool {
        *self == Satca::Nonsatcaint
    }
}
#[doc = "Field `SATCA` writer - Saturation - Channel A (Slots 0 or 1)"]
pub type SatcaW<'a, REG> = crate::BitWriter<'a, REG, Satca>;
impl<'a, REG> SatcaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Saturation, as specified by SAT configuration registers, occurred on either slot 0 or 1 (channel A)"]
    #[inline(always)]
    pub fn satcaint(self) -> &'a mut crate::W<REG> {
        self.variant(Satca::Satcaint)
    }
    #[doc = "No Saturation"]
    #[inline(always)]
    pub fn nonsatcaint(self) -> &'a mut crate::W<REG> {
        self.variant(Satca::Nonsatcaint)
    }
}
#[doc = "Saturation - Channel B (Slots 2 or 3)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Satcb {
    #[doc = "1: Saturation, as specified by SAT configuration registers, occurred on either slot 2 or 3 (channel B)"]
    Satcbint = 1,
    #[doc = "0: No-Saturation"]
    Nonsatcbint = 0,
}
impl From<Satcb> for bool {
    #[inline(always)]
    fn from(variant: Satcb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SATCB` reader - Saturation - Channel B (Slots 2 or 3)"]
pub type SatcbR = crate::BitReader<Satcb>;
impl SatcbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Satcb {
        match self.bits {
            true => Satcb::Satcbint,
            false => Satcb::Nonsatcbint,
        }
    }
    #[doc = "Saturation, as specified by SAT configuration registers, occurred on either slot 2 or 3 (channel B)"]
    #[inline(always)]
    pub fn is_satcbint(&self) -> bool {
        *self == Satcb::Satcbint
    }
    #[doc = "No-Saturation"]
    #[inline(always)]
    pub fn is_nonsatcbint(&self) -> bool {
        *self == Satcb::Nonsatcbint
    }
}
#[doc = "Field `SATCB` writer - Saturation - Channel B (Slots 2 or 3)"]
pub type SatcbW<'a, REG> = crate::BitWriter<'a, REG, Satcb>;
impl<'a, REG> SatcbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Saturation, as specified by SAT configuration registers, occurred on either slot 2 or 3 (channel B)"]
    #[inline(always)]
    pub fn satcbint(self) -> &'a mut crate::W<REG> {
        self.variant(Satcb::Satcbint)
    }
    #[doc = "No-Saturation"]
    #[inline(always)]
    pub fn nonsatcbint(self) -> &'a mut crate::W<REG> {
        self.variant(Satcb::Nonsatcbint)
    }
}
impl R {
    #[doc = "Bit 0 - ADC conversion complete interrupt."]
    #[inline(always)]
    pub fn cnvcmp(&self) -> CnvcmpR {
        CnvcmpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC scan complete interrupt."]
    #[inline(always)]
    pub fn scncmp(&self) -> ScncmpR {
        ScncmpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO 75 percent full interrupt."]
    #[inline(always)]
    pub fn fifoovr1(&self) -> Fifoovr1R {
        Fifoovr1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO 100 percent full interrupt."]
    #[inline(always)]
    pub fn fifoovr2(&self) -> Fifoovr2R {
        Fifoovr2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Window comparator voltage excursion interrupt."]
    #[inline(always)]
    pub fn wcexc(&self) -> WcexcR {
        WcexcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Window comparator voltage incursion interrupt."]
    #[inline(always)]
    pub fn wcinc(&self) -> WcincR {
        WcincR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Transfer Complete"]
    #[inline(always)]
    pub fn dcmp(&self) -> DcmpR {
        DcmpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Error Condition"]
    #[inline(always)]
    pub fn derr(&self) -> DerrR {
        DerrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Zero Crossing - Channel A (Slots 0 or 1)"]
    #[inline(always)]
    pub fn zxca(&self) -> ZxcaR {
        ZxcaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Zero Crossing - Channel B (Slots 2 or 3)"]
    #[inline(always)]
    pub fn zxcb(&self) -> ZxcbR {
        ZxcbR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Saturation - Channel A (Slots 0 or 1)"]
    #[inline(always)]
    pub fn satca(&self) -> SatcaR {
        SatcaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Saturation - Channel B (Slots 2 or 3)"]
    #[inline(always)]
    pub fn satcb(&self) -> SatcbR {
        SatcbR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC conversion complete interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cnvcmp(&mut self) -> CnvcmpW<IntclrSpec> {
        CnvcmpW::new(self, 0)
    }
    #[doc = "Bit 1 - ADC scan complete interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn scncmp(&mut self) -> ScncmpW<IntclrSpec> {
        ScncmpW::new(self, 1)
    }
    #[doc = "Bit 2 - FIFO 75 percent full interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn fifoovr1(&mut self) -> Fifoovr1W<IntclrSpec> {
        Fifoovr1W::new(self, 2)
    }
    #[doc = "Bit 3 - FIFO 100 percent full interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn fifoovr2(&mut self) -> Fifoovr2W<IntclrSpec> {
        Fifoovr2W::new(self, 3)
    }
    #[doc = "Bit 4 - Window comparator voltage excursion interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn wcexc(&mut self) -> WcexcW<IntclrSpec> {
        WcexcW::new(self, 4)
    }
    #[doc = "Bit 5 - Window comparator voltage incursion interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn wcinc(&mut self) -> WcincW<IntclrSpec> {
        WcincW::new(self, 5)
    }
    #[doc = "Bit 6 - DMA Transfer Complete"]
    #[inline(always)]
    #[must_use]
    pub fn dcmp(&mut self) -> DcmpW<IntclrSpec> {
        DcmpW::new(self, 6)
    }
    #[doc = "Bit 7 - DMA Error Condition"]
    #[inline(always)]
    #[must_use]
    pub fn derr(&mut self) -> DerrW<IntclrSpec> {
        DerrW::new(self, 7)
    }
    #[doc = "Bit 8 - Zero Crossing - Channel A (Slots 0 or 1)"]
    #[inline(always)]
    #[must_use]
    pub fn zxca(&mut self) -> ZxcaW<IntclrSpec> {
        ZxcaW::new(self, 8)
    }
    #[doc = "Bit 9 - Zero Crossing - Channel B (Slots 2 or 3)"]
    #[inline(always)]
    #[must_use]
    pub fn zxcb(&mut self) -> ZxcbW<IntclrSpec> {
        ZxcbW::new(self, 9)
    }
    #[doc = "Bit 10 - Saturation - Channel A (Slots 0 or 1)"]
    #[inline(always)]
    #[must_use]
    pub fn satca(&mut self) -> SatcaW<IntclrSpec> {
        SatcaW::new(self, 10)
    }
    #[doc = "Bit 11 - Saturation - Channel B (Slots 2 or 3)"]
    #[inline(always)]
    #[must_use]
    pub fn satcb(&mut self) -> SatcbW<IntclrSpec> {
        SatcbW::new(self, 11)
    }
}
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`intclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntclrSpec;
impl crate::RegisterSpec for IntclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intclr::R`](R) reader structure"]
impl crate::Readable for IntclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intclr::W`](W) writer structure"]
impl crate::Writable for IntclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTCLR to value 0"]
impl crate::Resettable for IntclrSpec {
    const RESET_VALUE: u32 = 0;
}
