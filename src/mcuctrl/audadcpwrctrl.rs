#[doc = "Register `AUDADCPWRCTRL` reader"]
pub type R = crate::R<AudadcpwrctrlSpec>;
#[doc = "Register `AUDADCPWRCTRL` writer"]
pub type W = crate::W<AudadcpwrctrlSpec>;
#[doc = "Audio ADC Power Control Software Override Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Audadcpwrctrlswe {
    #[doc = "0: Audio ADC temperature sensor and bandgap Software Override Disable."]
    OverrideDis = 0,
    #[doc = "1: Audio ADC temperature sensor and bandgap Software Override Enable."]
    OverrideEn = 1,
}
impl From<Audadcpwrctrlswe> for bool {
    #[inline(always)]
    fn from(variant: Audadcpwrctrlswe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUDADCPWRCTRLSWE` reader - Audio ADC Power Control Software Override Enable"]
pub type AudadcpwrctrlsweR = crate::BitReader<Audadcpwrctrlswe>;
impl AudadcpwrctrlsweR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Audadcpwrctrlswe {
        match self.bits {
            false => Audadcpwrctrlswe::OverrideDis,
            true => Audadcpwrctrlswe::OverrideEn,
        }
    }
    #[doc = "Audio ADC temperature sensor and bandgap Software Override Disable."]
    #[inline(always)]
    pub fn is_override_dis(&self) -> bool {
        *self == Audadcpwrctrlswe::OverrideDis
    }
    #[doc = "Audio ADC temperature sensor and bandgap Software Override Enable."]
    #[inline(always)]
    pub fn is_override_en(&self) -> bool {
        *self == Audadcpwrctrlswe::OverrideEn
    }
}
#[doc = "Field `AUDADCPWRCTRLSWE` writer - Audio ADC Power Control Software Override Enable"]
pub type AudadcpwrctrlsweW<'a, REG> = crate::BitWriter<'a, REG, Audadcpwrctrlswe>;
impl<'a, REG> AudadcpwrctrlsweW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Audio ADC temperature sensor and bandgap Software Override Disable."]
    #[inline(always)]
    pub fn override_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Audadcpwrctrlswe::OverrideDis)
    }
    #[doc = "Audio ADC temperature sensor and bandgap Software Override Enable."]
    #[inline(always)]
    pub fn override_en(self) -> &'a mut crate::W<REG> {
        self.variant(Audadcpwrctrlswe::OverrideEn)
    }
}
#[doc = "Enable the Global audio ADC Power Switch on when set to 1 if the AUDADCPWRCTRLSWE bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Audadcapsen {
    #[doc = "0: AUDADC power switch software power disable."]
    Dis = 0,
    #[doc = "1: AUDADC power switch software power enable."]
    En = 1,
}
impl From<Audadcapsen> for bool {
    #[inline(always)]
    fn from(variant: Audadcapsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUDADCAPSEN` reader - Enable the Global audio ADC Power Switch on when set to 1 if the AUDADCPWRCTRLSWE bit is set."]
pub type AudadcapsenR = crate::BitReader<Audadcapsen>;
impl AudadcapsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Audadcapsen {
        match self.bits {
            false => Audadcapsen::Dis,
            true => Audadcapsen::En,
        }
    }
    #[doc = "AUDADC power switch software power disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Audadcapsen::Dis
    }
    #[doc = "AUDADC power switch software power enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Audadcapsen::En
    }
}
#[doc = "Field `AUDADCAPSEN` writer - Enable the Global audio ADC Power Switch on when set to 1 if the AUDADCPWRCTRLSWE bit is set."]
pub type AudadcapsenW<'a, REG> = crate::BitWriter<'a, REG, Audadcapsen>;
impl<'a, REG> AudadcapsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AUDADC power switch software power disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Audadcapsen::Dis)
    }
    #[doc = "AUDADC power switch software power enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Audadcapsen::En)
    }
}
#[doc = "Enable the Analog, IO and SAR Digital logic Power Switch on when set to 1 if the AUDADCPWRCTRLSWE bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Audadcbpsen {
    #[doc = "0: AUDADC power switch software power disable."]
    Dis = 0,
    #[doc = "1: AUDADC power switch software power enable."]
    En = 1,
}
impl From<Audadcbpsen> for bool {
    #[inline(always)]
    fn from(variant: Audadcbpsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUDADCBPSEN` reader - Enable the Analog, IO and SAR Digital logic Power Switch on when set to 1 if the AUDADCPWRCTRLSWE bit is set."]
pub type AudadcbpsenR = crate::BitReader<Audadcbpsen>;
impl AudadcbpsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Audadcbpsen {
        match self.bits {
            false => Audadcbpsen::Dis,
            true => Audadcbpsen::En,
        }
    }
    #[doc = "AUDADC power switch software power disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Audadcbpsen::Dis
    }
    #[doc = "AUDADC power switch software power enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Audadcbpsen::En
    }
}
#[doc = "Field `AUDADCBPSEN` writer - Enable the Analog, IO and SAR Digital logic Power Switch on when set to 1 if the AUDADCPWRCTRLSWE bit is set."]
pub type AudadcbpsenW<'a, REG> = crate::BitWriter<'a, REG, Audadcbpsen>;
impl<'a, REG> AudadcbpsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AUDADC power switch software power disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Audadcbpsen::Dis)
    }
    #[doc = "AUDADC power switch software power enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Audadcbpsen::En)
    }
}
#[doc = "Bandgap and Temperature Sensor Power Switch Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Audbgtpen {
    #[doc = "0: Bandgap and temperature sensor disable."]
    Dis = 0,
    #[doc = "1: Bandgap and temperature sensor enable."]
    En = 1,
}
impl From<Audbgtpen> for bool {
    #[inline(always)]
    fn from(variant: Audbgtpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUDBGTPEN` reader - Bandgap and Temperature Sensor Power Switch Enable"]
pub type AudbgtpenR = crate::BitReader<Audbgtpen>;
impl AudbgtpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Audbgtpen {
        match self.bits {
            false => Audbgtpen::Dis,
            true => Audbgtpen::En,
        }
    }
    #[doc = "Bandgap and temperature sensor disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Audbgtpen::Dis
    }
    #[doc = "Bandgap and temperature sensor enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Audbgtpen::En
    }
}
#[doc = "Field `AUDBGTPEN` writer - Bandgap and Temperature Sensor Power Switch Enable"]
pub type AudbgtpenW<'a, REG> = crate::BitWriter<'a, REG, Audbgtpen>;
impl<'a, REG> AudbgtpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bandgap and temperature sensor disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Audbgtpen::Dis)
    }
    #[doc = "Bandgap and temperature sensor enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Audbgtpen::En)
    }
}
#[doc = "Reference Buffer Power Switch Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Audrefbufpen {
    #[doc = "0: Reference Buffer Power Switch disable."]
    Dis = 0,
    #[doc = "1: Reference Buffer Power Switch enable."]
    En = 1,
}
impl From<Audrefbufpen> for bool {
    #[inline(always)]
    fn from(variant: Audrefbufpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUDREFBUFPEN` reader - Reference Buffer Power Switch Enable"]
pub type AudrefbufpenR = crate::BitReader<Audrefbufpen>;
impl AudrefbufpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Audrefbufpen {
        match self.bits {
            false => Audrefbufpen::Dis,
            true => Audrefbufpen::En,
        }
    }
    #[doc = "Reference Buffer Power Switch disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Audrefbufpen::Dis
    }
    #[doc = "Reference Buffer Power Switch enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Audrefbufpen::En
    }
}
#[doc = "Field `AUDREFBUFPEN` writer - Reference Buffer Power Switch Enable"]
pub type AudrefbufpenW<'a, REG> = crate::BitWriter<'a, REG, Audrefbufpen>;
impl<'a, REG> AudrefbufpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reference Buffer Power Switch disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Audrefbufpen::Dis)
    }
    #[doc = "Reference Buffer Power Switch enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Audrefbufpen::En)
    }
}
#[doc = "Reference Buffer Keeper Power Switch Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Audrefkeeppen {
    #[doc = "0: Reference Buffer Keeper Power Switch disable."]
    Dis = 0,
    #[doc = "1: Reference Buffer Keeper Power Switch enable."]
    En = 1,
}
impl From<Audrefkeeppen> for bool {
    #[inline(always)]
    fn from(variant: Audrefkeeppen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUDREFKEEPPEN` reader - Reference Buffer Keeper Power Switch Enable"]
pub type AudrefkeeppenR = crate::BitReader<Audrefkeeppen>;
impl AudrefkeeppenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Audrefkeeppen {
        match self.bits {
            false => Audrefkeeppen::Dis,
            true => Audrefkeeppen::En,
        }
    }
    #[doc = "Reference Buffer Keeper Power Switch disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Audrefkeeppen::Dis
    }
    #[doc = "Reference Buffer Keeper Power Switch enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Audrefkeeppen::En
    }
}
#[doc = "Field `AUDREFKEEPPEN` writer - Reference Buffer Keeper Power Switch Enable"]
pub type AudrefkeeppenW<'a, REG> = crate::BitWriter<'a, REG, Audrefkeeppen>;
impl<'a, REG> AudrefkeeppenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reference Buffer Keeper Power Switch disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Audrefkeeppen::Dis)
    }
    #[doc = "Reference Buffer Keeper Power Switch enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Audrefkeeppen::En)
    }
}
#[doc = "ISOLATE signal for Power Switched SAR ( when AUDADCBPSEN is switched off )\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vddaudadcsarisolate {
    #[doc = "0: No Isolation"]
    Dis = 0,
    #[doc = "1: Isolate"]
    En = 1,
}
impl From<Vddaudadcsarisolate> for bool {
    #[inline(always)]
    fn from(variant: Vddaudadcsarisolate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDAUDADCSARISOLATE` reader - ISOLATE signal for Power Switched SAR ( when AUDADCBPSEN is switched off )"]
pub type VddaudadcsarisolateR = crate::BitReader<Vddaudadcsarisolate>;
impl VddaudadcsarisolateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vddaudadcsarisolate {
        match self.bits {
            false => Vddaudadcsarisolate::Dis,
            true => Vddaudadcsarisolate::En,
        }
    }
    #[doc = "No Isolation"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Vddaudadcsarisolate::Dis
    }
    #[doc = "Isolate"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Vddaudadcsarisolate::En
    }
}
#[doc = "Field `VDDAUDADCSARISOLATE` writer - ISOLATE signal for Power Switched SAR ( when AUDADCBPSEN is switched off )"]
pub type VddaudadcsarisolateW<'a, REG> = crate::BitWriter<'a, REG, Vddaudadcsarisolate>;
impl<'a, REG> VddaudadcsarisolateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Isolation"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Vddaudadcsarisolate::Dis)
    }
    #[doc = "Isolate"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Vddaudadcsarisolate::En)
    }
}
#[doc = "ISOLATE signal for audio ADC Digital Contoller ( when AUDADCAPSEN is switched off and if the AUDADCPWRCTRLSWE bit is set)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vddaudadcdigisolate {
    #[doc = "0: No Isolation"]
    Dis = 0,
    #[doc = "1: Isolate"]
    En = 1,
}
impl From<Vddaudadcdigisolate> for bool {
    #[inline(always)]
    fn from(variant: Vddaudadcdigisolate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDAUDADCDIGISOLATE` reader - ISOLATE signal for audio ADC Digital Contoller ( when AUDADCAPSEN is switched off and if the AUDADCPWRCTRLSWE bit is set)"]
pub type VddaudadcdigisolateR = crate::BitReader<Vddaudadcdigisolate>;
impl VddaudadcdigisolateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vddaudadcdigisolate {
        match self.bits {
            false => Vddaudadcdigisolate::Dis,
            true => Vddaudadcdigisolate::En,
        }
    }
    #[doc = "No Isolation"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Vddaudadcdigisolate::Dis
    }
    #[doc = "Isolate"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Vddaudadcdigisolate::En
    }
}
#[doc = "Field `VDDAUDADCDIGISOLATE` writer - ISOLATE signal for audio ADC Digital Contoller ( when AUDADCAPSEN is switched off and if the AUDADCPWRCTRLSWE bit is set)"]
pub type VddaudadcdigisolateW<'a, REG> = crate::BitWriter<'a, REG, Vddaudadcdigisolate>;
impl<'a, REG> VddaudadcdigisolateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Isolation"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Vddaudadcdigisolate::Dis)
    }
    #[doc = "Isolate"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Vddaudadcdigisolate::En)
    }
}
#[doc = "RESETN signal for Power Switched SAR and Digital Controller (when global power switch is off and if the AUDADCPWRCTRLSWE bit is set)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vddaudadcresetn {
    #[doc = "0: Resetn is asserted"]
    Assert = 0,
    #[doc = "1: Resetn is de-asserted"]
    Deassert = 1,
}
impl From<Vddaudadcresetn> for bool {
    #[inline(always)]
    fn from(variant: Vddaudadcresetn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDAUDADCRESETN` reader - RESETN signal for Power Switched SAR and Digital Controller (when global power switch is off and if the AUDADCPWRCTRLSWE bit is set)"]
pub type VddaudadcresetnR = crate::BitReader<Vddaudadcresetn>;
impl VddaudadcresetnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vddaudadcresetn {
        match self.bits {
            false => Vddaudadcresetn::Assert,
            true => Vddaudadcresetn::Deassert,
        }
    }
    #[doc = "Resetn is asserted"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == Vddaudadcresetn::Assert
    }
    #[doc = "Resetn is de-asserted"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == Vddaudadcresetn::Deassert
    }
}
#[doc = "Field `VDDAUDADCRESETN` writer - RESETN signal for Power Switched SAR and Digital Controller (when global power switch is off and if the AUDADCPWRCTRLSWE bit is set)"]
pub type VddaudadcresetnW<'a, REG> = crate::BitWriter<'a, REG, Vddaudadcresetn>;
impl<'a, REG> VddaudadcresetnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resetn is asserted"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut crate::W<REG> {
        self.variant(Vddaudadcresetn::Assert)
    }
    #[doc = "Resetn is de-asserted"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut crate::W<REG> {
        self.variant(Vddaudadcresetn::Deassert)
    }
}
#[doc = "Field `AUDADCVBATDIVEN` reader - Audio ADC VBAT DIV Power Enable ( if the AUDADCPWRCTRLSWE bit is set )"]
pub type AudadcvbatdivenR = crate::BitReader;
#[doc = "Field `AUDADCVBATDIVEN` writer - Audio ADC VBAT DIV Power Enable ( if the AUDADCPWRCTRLSWE bit is set )"]
pub type AudadcvbatdivenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUDADCINBUFSEL` reader - Audio ADC input buffer mux select"]
pub type AudadcinbufselR = crate::FieldReader;
#[doc = "Field `AUDADCINBUFSEL` writer - Audio ADC input buffer mux select"]
pub type AudadcinbufselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AUDADCINBUFEN` reader - Audio ADC Input Buffer Power Enable ( if the AUDADCPWRCTRLSWE bit is set )"]
pub type AudadcinbufenR = crate::BitReader;
#[doc = "Field `AUDADCINBUFEN` writer - Audio ADC Input Buffer Power Enable ( if the AUDADCPWRCTRLSWE bit is set )"]
pub type AudadcinbufenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUDADCRFBUFSLWEN` reader - Audio ADC reference buffer slew enable"]
pub type AudadcrfbufslwenR = crate::BitReader;
#[doc = "Field `AUDADCRFBUFSLWEN` writer - Audio ADC reference buffer slew enable"]
pub type AudadcrfbufslwenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUDADCKEEPOUTEN` reader - Audio ADC reference keeper out en"]
pub type AudadckeepoutenR = crate::BitReader;
#[doc = "Field `AUDADCKEEPOUTEN` writer - Audio ADC reference keeper out en"]
pub type AudadckeepoutenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Audio ADC Power Control Software Override Enable"]
    #[inline(always)]
    pub fn audadcpwrctrlswe(&self) -> AudadcpwrctrlsweR {
        AudadcpwrctrlsweR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the Global audio ADC Power Switch on when set to 1 if the AUDADCPWRCTRLSWE bit is set."]
    #[inline(always)]
    pub fn audadcapsen(&self) -> AudadcapsenR {
        AudadcapsenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable the Analog, IO and SAR Digital logic Power Switch on when set to 1 if the AUDADCPWRCTRLSWE bit is set."]
    #[inline(always)]
    pub fn audadcbpsen(&self) -> AudadcbpsenR {
        AudadcbpsenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bandgap and Temperature Sensor Power Switch Enable"]
    #[inline(always)]
    pub fn audbgtpen(&self) -> AudbgtpenR {
        AudbgtpenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reference Buffer Power Switch Enable"]
    #[inline(always)]
    pub fn audrefbufpen(&self) -> AudrefbufpenR {
        AudrefbufpenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reference Buffer Keeper Power Switch Enable"]
    #[inline(always)]
    pub fn audrefkeeppen(&self) -> AudrefkeeppenR {
        AudrefkeeppenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - ISOLATE signal for Power Switched SAR ( when AUDADCBPSEN is switched off )"]
    #[inline(always)]
    pub fn vddaudadcsarisolate(&self) -> VddaudadcsarisolateR {
        VddaudadcsarisolateR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ISOLATE signal for audio ADC Digital Contoller ( when AUDADCAPSEN is switched off and if the AUDADCPWRCTRLSWE bit is set)"]
    #[inline(always)]
    pub fn vddaudadcdigisolate(&self) -> VddaudadcdigisolateR {
        VddaudadcdigisolateR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RESETN signal for Power Switched SAR and Digital Controller (when global power switch is off and if the AUDADCPWRCTRLSWE bit is set)"]
    #[inline(always)]
    pub fn vddaudadcresetn(&self) -> VddaudadcresetnR {
        VddaudadcresetnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Audio ADC VBAT DIV Power Enable ( if the AUDADCPWRCTRLSWE bit is set )"]
    #[inline(always)]
    pub fn audadcvbatdiven(&self) -> AudadcvbatdivenR {
        AudadcvbatdivenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Audio ADC input buffer mux select"]
    #[inline(always)]
    pub fn audadcinbufsel(&self) -> AudadcinbufselR {
        AudadcinbufselR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Audio ADC Input Buffer Power Enable ( if the AUDADCPWRCTRLSWE bit is set )"]
    #[inline(always)]
    pub fn audadcinbufen(&self) -> AudadcinbufenR {
        AudadcinbufenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Audio ADC reference buffer slew enable"]
    #[inline(always)]
    pub fn audadcrfbufslwen(&self) -> AudadcrfbufslwenR {
        AudadcrfbufslwenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Audio ADC reference keeper out en"]
    #[inline(always)]
    pub fn audadckeepouten(&self) -> AudadckeepoutenR {
        AudadckeepoutenR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Audio ADC Power Control Software Override Enable"]
    #[inline(always)]
    #[must_use]
    pub fn audadcpwrctrlswe(&mut self) -> AudadcpwrctrlsweW<AudadcpwrctrlSpec> {
        AudadcpwrctrlsweW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable the Global audio ADC Power Switch on when set to 1 if the AUDADCPWRCTRLSWE bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn audadcapsen(&mut self) -> AudadcapsenW<AudadcpwrctrlSpec> {
        AudadcapsenW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable the Analog, IO and SAR Digital logic Power Switch on when set to 1 if the AUDADCPWRCTRLSWE bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn audadcbpsen(&mut self) -> AudadcbpsenW<AudadcpwrctrlSpec> {
        AudadcbpsenW::new(self, 2)
    }
    #[doc = "Bit 3 - Bandgap and Temperature Sensor Power Switch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn audbgtpen(&mut self) -> AudbgtpenW<AudadcpwrctrlSpec> {
        AudbgtpenW::new(self, 3)
    }
    #[doc = "Bit 4 - Reference Buffer Power Switch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn audrefbufpen(&mut self) -> AudrefbufpenW<AudadcpwrctrlSpec> {
        AudrefbufpenW::new(self, 4)
    }
    #[doc = "Bit 5 - Reference Buffer Keeper Power Switch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn audrefkeeppen(&mut self) -> AudrefkeeppenW<AudadcpwrctrlSpec> {
        AudrefkeeppenW::new(self, 5)
    }
    #[doc = "Bit 8 - ISOLATE signal for Power Switched SAR ( when AUDADCBPSEN is switched off )"]
    #[inline(always)]
    #[must_use]
    pub fn vddaudadcsarisolate(&mut self) -> VddaudadcsarisolateW<AudadcpwrctrlSpec> {
        VddaudadcsarisolateW::new(self, 8)
    }
    #[doc = "Bit 9 - ISOLATE signal for audio ADC Digital Contoller ( when AUDADCAPSEN is switched off and if the AUDADCPWRCTRLSWE bit is set)"]
    #[inline(always)]
    #[must_use]
    pub fn vddaudadcdigisolate(&mut self) -> VddaudadcdigisolateW<AudadcpwrctrlSpec> {
        VddaudadcdigisolateW::new(self, 9)
    }
    #[doc = "Bit 10 - RESETN signal for Power Switched SAR and Digital Controller (when global power switch is off and if the AUDADCPWRCTRLSWE bit is set)"]
    #[inline(always)]
    #[must_use]
    pub fn vddaudadcresetn(&mut self) -> VddaudadcresetnW<AudadcpwrctrlSpec> {
        VddaudadcresetnW::new(self, 10)
    }
    #[doc = "Bit 12 - Audio ADC VBAT DIV Power Enable ( if the AUDADCPWRCTRLSWE bit is set )"]
    #[inline(always)]
    #[must_use]
    pub fn audadcvbatdiven(&mut self) -> AudadcvbatdivenW<AudadcpwrctrlSpec> {
        AudadcvbatdivenW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Audio ADC input buffer mux select"]
    #[inline(always)]
    #[must_use]
    pub fn audadcinbufsel(&mut self) -> AudadcinbufselW<AudadcpwrctrlSpec> {
        AudadcinbufselW::new(self, 14)
    }
    #[doc = "Bit 16 - Audio ADC Input Buffer Power Enable ( if the AUDADCPWRCTRLSWE bit is set )"]
    #[inline(always)]
    #[must_use]
    pub fn audadcinbufen(&mut self) -> AudadcinbufenW<AudadcpwrctrlSpec> {
        AudadcinbufenW::new(self, 16)
    }
    #[doc = "Bit 17 - Audio ADC reference buffer slew enable"]
    #[inline(always)]
    #[must_use]
    pub fn audadcrfbufslwen(&mut self) -> AudadcrfbufslwenW<AudadcpwrctrlSpec> {
        AudadcrfbufslwenW::new(self, 17)
    }
    #[doc = "Bit 18 - Audio ADC reference keeper out en"]
    #[inline(always)]
    #[must_use]
    pub fn audadckeepouten(&mut self) -> AudadckeepoutenW<AudadcpwrctrlSpec> {
        AudadckeepoutenW::new(self, 18)
    }
}
#[doc = "Audio ADC Power Control\n\nYou can [`read`](crate::Reg::read) this register and get [`audadcpwrctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audadcpwrctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudadcpwrctrlSpec;
impl crate::RegisterSpec for AudadcpwrctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audadcpwrctrl::R`](R) reader structure"]
impl crate::Readable for AudadcpwrctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`audadcpwrctrl::W`](W) writer structure"]
impl crate::Writable for AudadcpwrctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUDADCPWRCTRL to value 0"]
impl crate::Resettable for AudadcpwrctrlSpec {
    const RESET_VALUE: u32 = 0;
}
