#[doc = "Register `MSPICFG` reader"]
pub type R = crate::R<MspicfgSpec>;
#[doc = "Register `MSPICFG` writer"]
pub type W = crate::W<MspicfgSpec>;
#[doc = "Selects SPI polarity.IMPORTANT NOTICE: Due to the susceptibility of creating a clock glitch which could cause register corruption, changing SPHA and SPOL bits should be done in separate writes to this register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spol {
    #[doc = "0: The base value of the clock is 0."]
    ClkBase0 = 0,
    #[doc = "1: The base value of the clock is 1."]
    ClkBase1 = 1,
}
impl From<Spol> for bool {
    #[inline(always)]
    fn from(variant: Spol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPOL` reader - Selects SPI polarity.IMPORTANT NOTICE: Due to the susceptibility of creating a clock glitch which could cause register corruption, changing SPHA and SPOL bits should be done in separate writes to this register."]
pub type SpolR = crate::BitReader<Spol>;
impl SpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spol {
        match self.bits {
            false => Spol::ClkBase0,
            true => Spol::ClkBase1,
        }
    }
    #[doc = "The base value of the clock is 0."]
    #[inline(always)]
    pub fn is_clk_base_0(&self) -> bool {
        *self == Spol::ClkBase0
    }
    #[doc = "The base value of the clock is 1."]
    #[inline(always)]
    pub fn is_clk_base_1(&self) -> bool {
        *self == Spol::ClkBase1
    }
}
#[doc = "Field `SPOL` writer - Selects SPI polarity.IMPORTANT NOTICE: Due to the susceptibility of creating a clock glitch which could cause register corruption, changing SPHA and SPOL bits should be done in separate writes to this register."]
pub type SpolW<'a, REG> = crate::BitWriter<'a, REG, Spol>;
impl<'a, REG> SpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The base value of the clock is 0."]
    #[inline(always)]
    pub fn clk_base_0(self) -> &'a mut crate::W<REG> {
        self.variant(Spol::ClkBase0)
    }
    #[doc = "The base value of the clock is 1."]
    #[inline(always)]
    pub fn clk_base_1(self) -> &'a mut crate::W<REG> {
        self.variant(Spol::ClkBase1)
    }
}
#[doc = "Selects SPI phase.IMPORTANT NOTICE: Due to the susceptibility of creating a clock glitch which could cause register corruption, changing SPHA and SPOL bits should be done in separate writes to this register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spha {
    #[doc = "0: Sample on the leading (first) clock edge."]
    SampleLeadingEdge = 0,
    #[doc = "1: Sample on the trailing (second) clock edge."]
    SampleTrailingEdge = 1,
}
impl From<Spha> for bool {
    #[inline(always)]
    fn from(variant: Spha) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPHA` reader - Selects SPI phase.IMPORTANT NOTICE: Due to the susceptibility of creating a clock glitch which could cause register corruption, changing SPHA and SPOL bits should be done in separate writes to this register."]
pub type SphaR = crate::BitReader<Spha>;
impl SphaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spha {
        match self.bits {
            false => Spha::SampleLeadingEdge,
            true => Spha::SampleTrailingEdge,
        }
    }
    #[doc = "Sample on the leading (first) clock edge."]
    #[inline(always)]
    pub fn is_sample_leading_edge(&self) -> bool {
        *self == Spha::SampleLeadingEdge
    }
    #[doc = "Sample on the trailing (second) clock edge."]
    #[inline(always)]
    pub fn is_sample_trailing_edge(&self) -> bool {
        *self == Spha::SampleTrailingEdge
    }
}
#[doc = "Field `SPHA` writer - Selects SPI phase.IMPORTANT NOTICE: Due to the susceptibility of creating a clock glitch which could cause register corruption, changing SPHA and SPOL bits should be done in separate writes to this register."]
pub type SphaW<'a, REG> = crate::BitWriter<'a, REG, Spha>;
impl<'a, REG> SphaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sample on the leading (first) clock edge."]
    #[inline(always)]
    pub fn sample_leading_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Spha::SampleLeadingEdge)
    }
    #[doc = "Sample on the trailing (second) clock edge."]
    #[inline(always)]
    pub fn sample_trailing_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Spha::SampleTrailingEdge)
    }
}
#[doc = "Field `FULLDUP` reader - Enables full duplex mode for Master SPI write operations. Data will be captured simultaneously into the read fifo"]
pub type FulldupR = crate::BitReader;
#[doc = "Field `FULLDUP` writer - Enables full duplex mode for Master SPI write operations. Data will be captured simultaneously into the read fifo"]
pub type FulldupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "enables write mode flow control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wtfc {
    #[doc = "0: Write mode flow control disabled."]
    Dis = 0,
    #[doc = "1: Write mode flow control enabled."]
    En = 1,
}
impl From<Wtfc> for bool {
    #[inline(always)]
    fn from(variant: Wtfc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WTFC` reader - enables write mode flow control."]
pub type WtfcR = crate::BitReader<Wtfc>;
impl WtfcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wtfc {
        match self.bits {
            false => Wtfc::Dis,
            true => Wtfc::En,
        }
    }
    #[doc = "Write mode flow control disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wtfc::Dis
    }
    #[doc = "Write mode flow control enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wtfc::En
    }
}
#[doc = "Field `WTFC` writer - enables write mode flow control."]
pub type WtfcW<'a, REG> = crate::BitWriter<'a, REG, Wtfc>;
impl<'a, REG> WtfcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write mode flow control disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Wtfc::Dis)
    }
    #[doc = "Write mode flow control enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Wtfc::En)
    }
}
#[doc = "Enables read mode flow control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdfc {
    #[doc = "0: Read mode flow control disabled."]
    Dis = 0,
    #[doc = "1: Read mode flow control enabled."]
    En = 1,
}
impl From<Rdfc> for bool {
    #[inline(always)]
    fn from(variant: Rdfc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDFC` reader - Enables read mode flow control."]
pub type RdfcR = crate::BitReader<Rdfc>;
impl RdfcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdfc {
        match self.bits {
            false => Rdfc::Dis,
            true => Rdfc::En,
        }
    }
    #[doc = "Read mode flow control disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rdfc::Dis
    }
    #[doc = "Read mode flow control enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Rdfc::En
    }
}
#[doc = "Field `RDFC` writer - Enables read mode flow control."]
pub type RdfcW<'a, REG> = crate::BitWriter<'a, REG, Rdfc>;
impl<'a, REG> RdfcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read mode flow control disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rdfc::Dis)
    }
    #[doc = "Read mode flow control enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Rdfc::En)
    }
}
#[doc = "Inverts MOSI when flow control is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mosiinv {
    #[doc = "0: MOSI is set to 0 in read mode and 1 in write mode."]
    Normal = 0,
    #[doc = "1: MOSI is set to 1 in read mode and 0 in write mode."]
    Invert = 1,
}
impl From<Mosiinv> for bool {
    #[inline(always)]
    fn from(variant: Mosiinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MOSIINV` reader - Inverts MOSI when flow control is enabled."]
pub type MosiinvR = crate::BitReader<Mosiinv>;
impl MosiinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mosiinv {
        match self.bits {
            false => Mosiinv::Normal,
            true => Mosiinv::Invert,
        }
    }
    #[doc = "MOSI is set to 0 in read mode and 1 in write mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Mosiinv::Normal
    }
    #[doc = "MOSI is set to 1 in read mode and 0 in write mode."]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == Mosiinv::Invert
    }
}
#[doc = "Field `MOSIINV` writer - Inverts MOSI when flow control is enabled."]
pub type MosiinvW<'a, REG> = crate::BitWriter<'a, REG, Mosiinv>;
impl<'a, REG> MosiinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MOSI is set to 0 in read mode and 1 in write mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Mosiinv::Normal)
    }
    #[doc = "MOSI is set to 1 in read mode and 0 in write mode."]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(Mosiinv::Invert)
    }
}
#[doc = "Selects the write mode flow control signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wtfcirq {
    #[doc = "0: MISO is used as the write mode flow control signal."]
    Miso = 0,
    #[doc = "1: IRQ is used as the write mode flow control signal."]
    Irq = 1,
}
impl From<Wtfcirq> for bool {
    #[inline(always)]
    fn from(variant: Wtfcirq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WTFCIRQ` reader - Selects the write mode flow control signal."]
pub type WtfcirqR = crate::BitReader<Wtfcirq>;
impl WtfcirqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wtfcirq {
        match self.bits {
            false => Wtfcirq::Miso,
            true => Wtfcirq::Irq,
        }
    }
    #[doc = "MISO is used as the write mode flow control signal."]
    #[inline(always)]
    pub fn is_miso(&self) -> bool {
        *self == Wtfcirq::Miso
    }
    #[doc = "IRQ is used as the write mode flow control signal."]
    #[inline(always)]
    pub fn is_irq(&self) -> bool {
        *self == Wtfcirq::Irq
    }
}
#[doc = "Field `WTFCIRQ` writer - Selects the write mode flow control signal."]
pub type WtfcirqW<'a, REG> = crate::BitWriter<'a, REG, Wtfcirq>;
impl<'a, REG> WtfcirqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MISO is used as the write mode flow control signal."]
    #[inline(always)]
    pub fn miso(self) -> &'a mut crate::W<REG> {
        self.variant(Wtfcirq::Miso)
    }
    #[doc = "IRQ is used as the write mode flow control signal."]
    #[inline(always)]
    pub fn irq(self) -> &'a mut crate::W<REG> {
        self.variant(Wtfcirq::Irq)
    }
}
#[doc = "selects the write flow control signal polarity. The transfers are halted when the selected flow control signal is OPPOSITE polarity of bit. (For example: WTFCPOL = 0 will allow a IRQ=1 to pause transfers).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wtfcpol {
    #[doc = "0: Flow control signal high(1) creates flow control and byte transfers will stop until the flow control signal goes low."]
    High = 0,
    #[doc = "1: Flow control signal low(0) creates flow control and byte transfers will stop until the flow control signal goes high(1)."]
    Low = 1,
}
impl From<Wtfcpol> for bool {
    #[inline(always)]
    fn from(variant: Wtfcpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WTFCPOL` reader - selects the write flow control signal polarity. The transfers are halted when the selected flow control signal is OPPOSITE polarity of bit. (For example: WTFCPOL = 0 will allow a IRQ=1 to pause transfers)."]
pub type WtfcpolR = crate::BitReader<Wtfcpol>;
impl WtfcpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wtfcpol {
        match self.bits {
            false => Wtfcpol::High,
            true => Wtfcpol::Low,
        }
    }
    #[doc = "Flow control signal high(1) creates flow control and byte transfers will stop until the flow control signal goes low."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wtfcpol::High
    }
    #[doc = "Flow control signal low(0) creates flow control and byte transfers will stop until the flow control signal goes high(1)."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wtfcpol::Low
    }
}
#[doc = "Field `WTFCPOL` writer - selects the write flow control signal polarity. The transfers are halted when the selected flow control signal is OPPOSITE polarity of bit. (For example: WTFCPOL = 0 will allow a IRQ=1 to pause transfers)."]
pub type WtfcpolW<'a, REG> = crate::BitWriter<'a, REG, Wtfcpol>;
impl<'a, REG> WtfcpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flow control signal high(1) creates flow control and byte transfers will stop until the flow control signal goes low."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wtfcpol::High)
    }
    #[doc = "Flow control signal low(0) creates flow control and byte transfers will stop until the flow control signal goes high(1)."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wtfcpol::Low)
    }
}
#[doc = "Selects the read flow control signal polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdfcpol {
    #[doc = "0: Flow control signal high creates flow control."]
    High = 0,
    #[doc = "1: Flow control signal low creates flow control."]
    Low = 1,
}
impl From<Rdfcpol> for bool {
    #[inline(always)]
    fn from(variant: Rdfcpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDFCPOL` reader - Selects the read flow control signal polarity."]
pub type RdfcpolR = crate::BitReader<Rdfcpol>;
impl RdfcpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdfcpol {
        match self.bits {
            false => Rdfcpol::High,
            true => Rdfcpol::Low,
        }
    }
    #[doc = "Flow control signal high creates flow control."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Rdfcpol::High
    }
    #[doc = "Flow control signal low creates flow control."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Rdfcpol::Low
    }
}
#[doc = "Field `RDFCPOL` writer - Selects the read flow control signal polarity."]
pub type RdfcpolW<'a, REG> = crate::BitWriter<'a, REG, Rdfcpol>;
impl<'a, REG> RdfcpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flow control signal high creates flow control."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Rdfcpol::High)
    }
    #[doc = "Flow control signal low creates flow control."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Rdfcpol::Low)
    }
}
#[doc = "Selects data transfer as MSB first (0) or LSB first (1) for the data portion of the SPI transaction. The offset bytes are always transmitted MSB first.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spilsb {
    #[doc = "0: Send and receive MSB bit first"]
    Msb = 0,
    #[doc = "1: Send and receive LSB bit first"]
    Lsb = 1,
}
impl From<Spilsb> for bool {
    #[inline(always)]
    fn from(variant: Spilsb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPILSB` reader - Selects data transfer as MSB first (0) or LSB first (1) for the data portion of the SPI transaction. The offset bytes are always transmitted MSB first."]
pub type SpilsbR = crate::BitReader<Spilsb>;
impl SpilsbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spilsb {
        match self.bits {
            false => Spilsb::Msb,
            true => Spilsb::Lsb,
        }
    }
    #[doc = "Send and receive MSB bit first"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == Spilsb::Msb
    }
    #[doc = "Send and receive LSB bit first"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == Spilsb::Lsb
    }
}
#[doc = "Field `SPILSB` writer - Selects data transfer as MSB first (0) or LSB first (1) for the data portion of the SPI transaction. The offset bytes are always transmitted MSB first."]
pub type SpilsbW<'a, REG> = crate::BitWriter<'a, REG, Spilsb>;
impl<'a, REG> SpilsbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Send and receive MSB bit first"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(Spilsb::Msb)
    }
    #[doc = "Send and receive LSB bit first"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(Spilsb::Lsb)
    }
}
#[doc = "Field `DINDLY` reader - Delay tap to use for the input signal (MISO). This gives more hold time on the input data."]
pub type DindlyR = crate::FieldReader;
#[doc = "Field `DINDLY` writer - Delay tap to use for the input signal (MISO). This gives more hold time on the input data."]
pub type DindlyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DOUTDLY` reader - Delay tap to use for the output signal (MOSI). This give more hold time on the output data"]
pub type DoutdlyR = crate::FieldReader;
#[doc = "Field `DOUTDLY` writer - Delay tap to use for the output signal (MOSI). This give more hold time on the output data"]
pub type DoutdlyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSPIRST` reader - Not used. To reset the module, toggle the SMOD_EN for the module"]
pub type MspirstR = crate::BitReader;
#[doc = "Field `MSPIRST` writer - Not used. To reset the module, toggle the SMOD_EN for the module"]
pub type MspirstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Selects SPI polarity.IMPORTANT NOTICE: Due to the susceptibility of creating a clock glitch which could cause register corruption, changing SPHA and SPOL bits should be done in separate writes to this register."]
    #[inline(always)]
    pub fn spol(&self) -> SpolR {
        SpolR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects SPI phase.IMPORTANT NOTICE: Due to the susceptibility of creating a clock glitch which could cause register corruption, changing SPHA and SPOL bits should be done in separate writes to this register."]
    #[inline(always)]
    pub fn spha(&self) -> SphaR {
        SphaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables full duplex mode for Master SPI write operations. Data will be captured simultaneously into the read fifo"]
    #[inline(always)]
    pub fn fulldup(&self) -> FulldupR {
        FulldupR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - enables write mode flow control."]
    #[inline(always)]
    pub fn wtfc(&self) -> WtfcR {
        WtfcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enables read mode flow control."]
    #[inline(always)]
    pub fn rdfc(&self) -> RdfcR {
        RdfcR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Inverts MOSI when flow control is enabled."]
    #[inline(always)]
    pub fn mosiinv(&self) -> MosiinvR {
        MosiinvR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Selects the write mode flow control signal."]
    #[inline(always)]
    pub fn wtfcirq(&self) -> WtfcirqR {
        WtfcirqR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - selects the write flow control signal polarity. The transfers are halted when the selected flow control signal is OPPOSITE polarity of bit. (For example: WTFCPOL = 0 will allow a IRQ=1 to pause transfers)."]
    #[inline(always)]
    pub fn wtfcpol(&self) -> WtfcpolR {
        WtfcpolR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Selects the read flow control signal polarity."]
    #[inline(always)]
    pub fn rdfcpol(&self) -> RdfcpolR {
        RdfcpolR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Selects data transfer as MSB first (0) or LSB first (1) for the data portion of the SPI transaction. The offset bytes are always transmitted MSB first."]
    #[inline(always)]
    pub fn spilsb(&self) -> SpilsbR {
        SpilsbR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Delay tap to use for the input signal (MISO). This gives more hold time on the input data."]
    #[inline(always)]
    pub fn dindly(&self) -> DindlyR {
        DindlyR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Delay tap to use for the output signal (MOSI). This give more hold time on the output data"]
    #[inline(always)]
    pub fn doutdly(&self) -> DoutdlyR {
        DoutdlyR::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bit 30 - Not used. To reset the module, toggle the SMOD_EN for the module"]
    #[inline(always)]
    pub fn mspirst(&self) -> MspirstR {
        MspirstR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects SPI polarity.IMPORTANT NOTICE: Due to the susceptibility of creating a clock glitch which could cause register corruption, changing SPHA and SPOL bits should be done in separate writes to this register."]
    #[inline(always)]
    #[must_use]
    pub fn spol(&mut self) -> SpolW<MspicfgSpec> {
        SpolW::new(self, 0)
    }
    #[doc = "Bit 1 - Selects SPI phase.IMPORTANT NOTICE: Due to the susceptibility of creating a clock glitch which could cause register corruption, changing SPHA and SPOL bits should be done in separate writes to this register."]
    #[inline(always)]
    #[must_use]
    pub fn spha(&mut self) -> SphaW<MspicfgSpec> {
        SphaW::new(self, 1)
    }
    #[doc = "Bit 2 - Enables full duplex mode for Master SPI write operations. Data will be captured simultaneously into the read fifo"]
    #[inline(always)]
    #[must_use]
    pub fn fulldup(&mut self) -> FulldupW<MspicfgSpec> {
        FulldupW::new(self, 2)
    }
    #[doc = "Bit 16 - enables write mode flow control."]
    #[inline(always)]
    #[must_use]
    pub fn wtfc(&mut self) -> WtfcW<MspicfgSpec> {
        WtfcW::new(self, 16)
    }
    #[doc = "Bit 17 - Enables read mode flow control."]
    #[inline(always)]
    #[must_use]
    pub fn rdfc(&mut self) -> RdfcW<MspicfgSpec> {
        RdfcW::new(self, 17)
    }
    #[doc = "Bit 18 - Inverts MOSI when flow control is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn mosiinv(&mut self) -> MosiinvW<MspicfgSpec> {
        MosiinvW::new(self, 18)
    }
    #[doc = "Bit 20 - Selects the write mode flow control signal."]
    #[inline(always)]
    #[must_use]
    pub fn wtfcirq(&mut self) -> WtfcirqW<MspicfgSpec> {
        WtfcirqW::new(self, 20)
    }
    #[doc = "Bit 21 - selects the write flow control signal polarity. The transfers are halted when the selected flow control signal is OPPOSITE polarity of bit. (For example: WTFCPOL = 0 will allow a IRQ=1 to pause transfers)."]
    #[inline(always)]
    #[must_use]
    pub fn wtfcpol(&mut self) -> WtfcpolW<MspicfgSpec> {
        WtfcpolW::new(self, 21)
    }
    #[doc = "Bit 22 - Selects the read flow control signal polarity."]
    #[inline(always)]
    #[must_use]
    pub fn rdfcpol(&mut self) -> RdfcpolW<MspicfgSpec> {
        RdfcpolW::new(self, 22)
    }
    #[doc = "Bit 23 - Selects data transfer as MSB first (0) or LSB first (1) for the data portion of the SPI transaction. The offset bytes are always transmitted MSB first."]
    #[inline(always)]
    #[must_use]
    pub fn spilsb(&mut self) -> SpilsbW<MspicfgSpec> {
        SpilsbW::new(self, 23)
    }
    #[doc = "Bits 24:26 - Delay tap to use for the input signal (MISO). This gives more hold time on the input data."]
    #[inline(always)]
    #[must_use]
    pub fn dindly(&mut self) -> DindlyW<MspicfgSpec> {
        DindlyW::new(self, 24)
    }
    #[doc = "Bits 27:29 - Delay tap to use for the output signal (MOSI). This give more hold time on the output data"]
    #[inline(always)]
    #[must_use]
    pub fn doutdly(&mut self) -> DoutdlyW<MspicfgSpec> {
        DoutdlyW::new(self, 27)
    }
    #[doc = "Bit 30 - Not used. To reset the module, toggle the SMOD_EN for the module"]
    #[inline(always)]
    #[must_use]
    pub fn mspirst(&mut self) -> MspirstW<MspicfgSpec> {
        MspirstW::new(self, 30)
    }
}
#[doc = "Controls the configuration of the SPI master module, including POL/PHA, LSB, flow control, and delays for MISO and MOSI\n\nYou can [`read`](crate::Reg::read) this register and get [`mspicfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspicfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MspicfgSpec;
impl crate::RegisterSpec for MspicfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mspicfg::R`](R) reader structure"]
impl crate::Readable for MspicfgSpec {}
#[doc = "`write(|w| ..)` method takes [`mspicfg::W`](W) writer structure"]
impl crate::Writable for MspicfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSPICFG to value 0x0020_0000"]
impl crate::Resettable for MspicfgSpec {
    const RESET_VALUE: u32 = 0x0020_0000;
}
