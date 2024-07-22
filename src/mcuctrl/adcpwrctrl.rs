#[doc = "Register `ADCPWRCTRL` reader"]
pub type R = crate::R<AdcpwrctrlSpec>;
#[doc = "Register `ADCPWRCTRL` writer"]
pub type W = crate::W<AdcpwrctrlSpec>;
#[doc = "ADC Power Control Software Override Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcpwrctrlswe {
    #[doc = "0: ADC temperature sensor and bandgap Software Override Disable."]
    OverrideDis = 0,
    #[doc = "1: ADC temperature sensor and bandgap Software Override Enable."]
    OverrideEn = 1,
}
impl From<Adcpwrctrlswe> for bool {
    #[inline(always)]
    fn from(variant: Adcpwrctrlswe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCPWRCTRLSWE` reader - ADC Power Control Software Override Enable"]
pub type AdcpwrctrlsweR = crate::BitReader<Adcpwrctrlswe>;
impl AdcpwrctrlsweR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcpwrctrlswe {
        match self.bits {
            false => Adcpwrctrlswe::OverrideDis,
            true => Adcpwrctrlswe::OverrideEn,
        }
    }
    #[doc = "ADC temperature sensor and bandgap Software Override Disable."]
    #[inline(always)]
    pub fn is_override_dis(&self) -> bool {
        *self == Adcpwrctrlswe::OverrideDis
    }
    #[doc = "ADC temperature sensor and bandgap Software Override Enable."]
    #[inline(always)]
    pub fn is_override_en(&self) -> bool {
        *self == Adcpwrctrlswe::OverrideEn
    }
}
#[doc = "Field `ADCPWRCTRLSWE` writer - ADC Power Control Software Override Enable"]
pub type AdcpwrctrlsweW<'a, REG> = crate::BitWriter<'a, REG, Adcpwrctrlswe>;
impl<'a, REG> AdcpwrctrlsweW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC temperature sensor and bandgap Software Override Disable."]
    #[inline(always)]
    pub fn override_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpwrctrlswe::OverrideDis)
    }
    #[doc = "ADC temperature sensor and bandgap Software Override Enable."]
    #[inline(always)]
    pub fn override_en(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpwrctrlswe::OverrideEn)
    }
}
#[doc = "Enable the Global ADC Power Switch on when set to 1 if the ADCPWRCTRLSWE bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcapsen {
    #[doc = "0: ADC power switch software power disable."]
    Dis = 0,
    #[doc = "1: ADC power switch software power enable."]
    En = 1,
}
impl From<Adcapsen> for bool {
    #[inline(always)]
    fn from(variant: Adcapsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCAPSEN` reader - Enable the Global ADC Power Switch on when set to 1 if the ADCPWRCTRLSWE bit is set."]
pub type AdcapsenR = crate::BitReader<Adcapsen>;
impl AdcapsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcapsen {
        match self.bits {
            false => Adcapsen::Dis,
            true => Adcapsen::En,
        }
    }
    #[doc = "ADC power switch software power disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Adcapsen::Dis
    }
    #[doc = "ADC power switch software power enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Adcapsen::En
    }
}
#[doc = "Field `ADCAPSEN` writer - Enable the Global ADC Power Switch on when set to 1 if the ADCPWRCTRLSWE bit is set."]
pub type AdcapsenW<'a, REG> = crate::BitWriter<'a, REG, Adcapsen>;
impl<'a, REG> AdcapsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC power switch software power disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Adcapsen::Dis)
    }
    #[doc = "ADC power switch software power enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Adcapsen::En)
    }
}
#[doc = "Enable the Analog, IO and SAR Digital logic Power Switch on when set to 1 if the ADCPWRCTRLSWE bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcbpsen {
    #[doc = "0: ADC power switch software power disable."]
    Dis = 0,
    #[doc = "1: ADC power switch software power enable."]
    En = 1,
}
impl From<Adcbpsen> for bool {
    #[inline(always)]
    fn from(variant: Adcbpsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCBPSEN` reader - Enable the Analog, IO and SAR Digital logic Power Switch on when set to 1 if the ADCPWRCTRLSWE bit is set."]
pub type AdcbpsenR = crate::BitReader<Adcbpsen>;
impl AdcbpsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcbpsen {
        match self.bits {
            false => Adcbpsen::Dis,
            true => Adcbpsen::En,
        }
    }
    #[doc = "ADC power switch software power disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Adcbpsen::Dis
    }
    #[doc = "ADC power switch software power enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Adcbpsen::En
    }
}
#[doc = "Field `ADCBPSEN` writer - Enable the Analog, IO and SAR Digital logic Power Switch on when set to 1 if the ADCPWRCTRLSWE bit is set."]
pub type AdcbpsenW<'a, REG> = crate::BitWriter<'a, REG, Adcbpsen>;
impl<'a, REG> AdcbpsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC power switch software power disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Adcbpsen::Dis)
    }
    #[doc = "ADC power switch software power enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Adcbpsen::En)
    }
}
#[doc = "Bandgap and Temperature Sensor Power Switch Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bgtpen {
    #[doc = "0: Bandgap and temperature sensor disable."]
    Dis = 0,
    #[doc = "1: Bandgap and temperature sensor enable."]
    En = 1,
}
impl From<Bgtpen> for bool {
    #[inline(always)]
    fn from(variant: Bgtpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BGTPEN` reader - Bandgap and Temperature Sensor Power Switch Enable"]
pub type BgtpenR = crate::BitReader<Bgtpen>;
impl BgtpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bgtpen {
        match self.bits {
            false => Bgtpen::Dis,
            true => Bgtpen::En,
        }
    }
    #[doc = "Bandgap and temperature sensor disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Bgtpen::Dis
    }
    #[doc = "Bandgap and temperature sensor enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Bgtpen::En
    }
}
#[doc = "Field `BGTPEN` writer - Bandgap and Temperature Sensor Power Switch Enable"]
pub type BgtpenW<'a, REG> = crate::BitWriter<'a, REG, Bgtpen>;
impl<'a, REG> BgtpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bandgap and temperature sensor disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Bgtpen::Dis)
    }
    #[doc = "Bandgap and temperature sensor enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Bgtpen::En)
    }
}
#[doc = "Bandgap and Temperature Sensor Power Switch Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bgtlppen {
    #[doc = "0: Bandgap and temperature sensor disable."]
    Dis = 0,
    #[doc = "1: Bandgap and temperature sensor enable."]
    En = 1,
}
impl From<Bgtlppen> for bool {
    #[inline(always)]
    fn from(variant: Bgtlppen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BGTLPPEN` reader - Bandgap and Temperature Sensor Power Switch Enable"]
pub type BgtlppenR = crate::BitReader<Bgtlppen>;
impl BgtlppenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bgtlppen {
        match self.bits {
            false => Bgtlppen::Dis,
            true => Bgtlppen::En,
        }
    }
    #[doc = "Bandgap and temperature sensor disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Bgtlppen::Dis
    }
    #[doc = "Bandgap and temperature sensor enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Bgtlppen::En
    }
}
#[doc = "Field `BGTLPPEN` writer - Bandgap and Temperature Sensor Power Switch Enable"]
pub type BgtlppenW<'a, REG> = crate::BitWriter<'a, REG, Bgtlppen>;
impl<'a, REG> BgtlppenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bandgap and temperature sensor disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Bgtlppen::Dis)
    }
    #[doc = "Bandgap and temperature sensor enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Bgtlppen::En)
    }
}
#[doc = "Reference Buffer Power Switch Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refbufpen {
    #[doc = "0: Reference Buffer Power Switch disable."]
    Dis = 0,
    #[doc = "1: Reference Buffer Power Switch enable."]
    En = 1,
}
impl From<Refbufpen> for bool {
    #[inline(always)]
    fn from(variant: Refbufpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFBUFPEN` reader - Reference Buffer Power Switch Enable"]
pub type RefbufpenR = crate::BitReader<Refbufpen>;
impl RefbufpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refbufpen {
        match self.bits {
            false => Refbufpen::Dis,
            true => Refbufpen::En,
        }
    }
    #[doc = "Reference Buffer Power Switch disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Refbufpen::Dis
    }
    #[doc = "Reference Buffer Power Switch enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Refbufpen::En
    }
}
#[doc = "Field `REFBUFPEN` writer - Reference Buffer Power Switch Enable"]
pub type RefbufpenW<'a, REG> = crate::BitWriter<'a, REG, Refbufpen>;
impl<'a, REG> RefbufpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reference Buffer Power Switch disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Refbufpen::Dis)
    }
    #[doc = "Reference Buffer Power Switch enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Refbufpen::En)
    }
}
#[doc = "Reference Buffer Keeper Power Switch Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refkeeppen {
    #[doc = "0: Reference Buffer Keeper Power Switch disable."]
    Dis = 0,
    #[doc = "1: Reference Buffer Keeper Power Switch enable."]
    En = 1,
}
impl From<Refkeeppen> for bool {
    #[inline(always)]
    fn from(variant: Refkeeppen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFKEEPPEN` reader - Reference Buffer Keeper Power Switch Enable"]
pub type RefkeeppenR = crate::BitReader<Refkeeppen>;
impl RefkeeppenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refkeeppen {
        match self.bits {
            false => Refkeeppen::Dis,
            true => Refkeeppen::En,
        }
    }
    #[doc = "Reference Buffer Keeper Power Switch disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Refkeeppen::Dis
    }
    #[doc = "Reference Buffer Keeper Power Switch enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Refkeeppen::En
    }
}
#[doc = "Field `REFKEEPPEN` writer - Reference Buffer Keeper Power Switch Enable"]
pub type RefkeeppenW<'a, REG> = crate::BitWriter<'a, REG, Refkeeppen>;
impl<'a, REG> RefkeeppenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reference Buffer Keeper Power Switch disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Refkeeppen::Dis)
    }
    #[doc = "Reference Buffer Keeper Power Switch enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Refkeeppen::En)
    }
}
#[doc = "ISOLATE signal for Power Switched SAR ( when ADCBPSEN is switched off )\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vddadcsarisolate {
    #[doc = "0: No Isolation"]
    Dis = 0,
    #[doc = "1: Isolate"]
    En = 1,
}
impl From<Vddadcsarisolate> for bool {
    #[inline(always)]
    fn from(variant: Vddadcsarisolate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDADCSARISOLATE` reader - ISOLATE signal for Power Switched SAR ( when ADCBPSEN is switched off )"]
pub type VddadcsarisolateR = crate::BitReader<Vddadcsarisolate>;
impl VddadcsarisolateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vddadcsarisolate {
        match self.bits {
            false => Vddadcsarisolate::Dis,
            true => Vddadcsarisolate::En,
        }
    }
    #[doc = "No Isolation"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Vddadcsarisolate::Dis
    }
    #[doc = "Isolate"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Vddadcsarisolate::En
    }
}
#[doc = "Field `VDDADCSARISOLATE` writer - ISOLATE signal for Power Switched SAR ( when ADCBPSEN is switched off )"]
pub type VddadcsarisolateW<'a, REG> = crate::BitWriter<'a, REG, Vddadcsarisolate>;
impl<'a, REG> VddadcsarisolateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Isolation"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Vddadcsarisolate::Dis)
    }
    #[doc = "Isolate"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Vddadcsarisolate::En)
    }
}
#[doc = "ISOLATE signal for ADC Digital Contoller ( when ADCAPSEN is switched off and if the ADCPWRCTRLSWE bit is set)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vddadcdigisolate {
    #[doc = "0: No Isolation"]
    Dis = 0,
    #[doc = "1: Isolate"]
    En = 1,
}
impl From<Vddadcdigisolate> for bool {
    #[inline(always)]
    fn from(variant: Vddadcdigisolate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDADCDIGISOLATE` reader - ISOLATE signal for ADC Digital Contoller ( when ADCAPSEN is switched off and if the ADCPWRCTRLSWE bit is set)"]
pub type VddadcdigisolateR = crate::BitReader<Vddadcdigisolate>;
impl VddadcdigisolateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vddadcdigisolate {
        match self.bits {
            false => Vddadcdigisolate::Dis,
            true => Vddadcdigisolate::En,
        }
    }
    #[doc = "No Isolation"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Vddadcdigisolate::Dis
    }
    #[doc = "Isolate"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Vddadcdigisolate::En
    }
}
#[doc = "Field `VDDADCDIGISOLATE` writer - ISOLATE signal for ADC Digital Contoller ( when ADCAPSEN is switched off and if the ADCPWRCTRLSWE bit is set)"]
pub type VddadcdigisolateW<'a, REG> = crate::BitWriter<'a, REG, Vddadcdigisolate>;
impl<'a, REG> VddadcdigisolateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Isolation"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Vddadcdigisolate::Dis)
    }
    #[doc = "Isolate"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Vddadcdigisolate::En)
    }
}
#[doc = "RESETN signal for Power Switched SAR and Digital Controller (when global power switch is off and if the ADCPWRCTRLSWE bit is set)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vddadcresetn {
    #[doc = "0: Resetn is asserted"]
    Assert = 0,
    #[doc = "1: Resetn is de-asserted"]
    Deassert = 1,
}
impl From<Vddadcresetn> for bool {
    #[inline(always)]
    fn from(variant: Vddadcresetn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDADCRESETN` reader - RESETN signal for Power Switched SAR and Digital Controller (when global power switch is off and if the ADCPWRCTRLSWE bit is set)"]
pub type VddadcresetnR = crate::BitReader<Vddadcresetn>;
impl VddadcresetnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vddadcresetn {
        match self.bits {
            false => Vddadcresetn::Assert,
            true => Vddadcresetn::Deassert,
        }
    }
    #[doc = "Resetn is asserted"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == Vddadcresetn::Assert
    }
    #[doc = "Resetn is de-asserted"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == Vddadcresetn::Deassert
    }
}
#[doc = "Field `VDDADCRESETN` writer - RESETN signal for Power Switched SAR and Digital Controller (when global power switch is off and if the ADCPWRCTRLSWE bit is set)"]
pub type VddadcresetnW<'a, REG> = crate::BitWriter<'a, REG, Vddadcresetn>;
impl<'a, REG> VddadcresetnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resetn is asserted"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut crate::W<REG> {
        self.variant(Vddadcresetn::Assert)
    }
    #[doc = "Resetn is de-asserted"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut crate::W<REG> {
        self.variant(Vddadcresetn::Deassert)
    }
}
#[doc = "Field `ADCVBATDIVEN` reader - ADC VBAT DIV Power Enable ( if the ADCPWRCTRLSWE bit is set )"]
pub type AdcvbatdivenR = crate::BitReader;
#[doc = "Field `ADCVBATDIVEN` writer - ADC VBAT DIV Power Enable ( if the ADCPWRCTRLSWE bit is set )"]
pub type AdcvbatdivenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCINBUFSEL` reader - ADC input buffer mux select"]
pub type AdcinbufselR = crate::FieldReader;
#[doc = "Field `ADCINBUFSEL` writer - ADC input buffer mux select"]
pub type AdcinbufselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADCINBUFEN` reader - ADC Input Buffer Power Enable ( if the ADCPWRCTRLSWE bit is set )"]
pub type AdcinbufenR = crate::BitReader;
#[doc = "Field `ADCINBUFEN` writer - ADC Input Buffer Power Enable ( if the ADCPWRCTRLSWE bit is set )"]
pub type AdcinbufenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCRFBUFSLWEN` reader - ADC reference buffer slew enable"]
pub type AdcrfbufslwenR = crate::BitReader;
#[doc = "Field `ADCRFBUFSLWEN` writer - ADC reference buffer slew enable"]
pub type AdcrfbufslwenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCKEEPOUTEN` reader - ADC reference keeper out en"]
pub type AdckeepoutenR = crate::BitReader;
#[doc = "Field `ADCKEEPOUTEN` writer - ADC reference keeper out en"]
pub type AdckeepoutenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC Power Control Software Override Enable"]
    #[inline(always)]
    pub fn adcpwrctrlswe(&self) -> AdcpwrctrlsweR {
        AdcpwrctrlsweR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the Global ADC Power Switch on when set to 1 if the ADCPWRCTRLSWE bit is set."]
    #[inline(always)]
    pub fn adcapsen(&self) -> AdcapsenR {
        AdcapsenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable the Analog, IO and SAR Digital logic Power Switch on when set to 1 if the ADCPWRCTRLSWE bit is set."]
    #[inline(always)]
    pub fn adcbpsen(&self) -> AdcbpsenR {
        AdcbpsenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bandgap and Temperature Sensor Power Switch Enable"]
    #[inline(always)]
    pub fn bgtpen(&self) -> BgtpenR {
        BgtpenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bandgap and Temperature Sensor Power Switch Enable"]
    #[inline(always)]
    pub fn bgtlppen(&self) -> BgtlppenR {
        BgtlppenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reference Buffer Power Switch Enable"]
    #[inline(always)]
    pub fn refbufpen(&self) -> RefbufpenR {
        RefbufpenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reference Buffer Keeper Power Switch Enable"]
    #[inline(always)]
    pub fn refkeeppen(&self) -> RefkeeppenR {
        RefkeeppenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ISOLATE signal for Power Switched SAR ( when ADCBPSEN is switched off )"]
    #[inline(always)]
    pub fn vddadcsarisolate(&self) -> VddadcsarisolateR {
        VddadcsarisolateR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ISOLATE signal for ADC Digital Contoller ( when ADCAPSEN is switched off and if the ADCPWRCTRLSWE bit is set)"]
    #[inline(always)]
    pub fn vddadcdigisolate(&self) -> VddadcdigisolateR {
        VddadcdigisolateR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RESETN signal for Power Switched SAR and Digital Controller (when global power switch is off and if the ADCPWRCTRLSWE bit is set)"]
    #[inline(always)]
    pub fn vddadcresetn(&self) -> VddadcresetnR {
        VddadcresetnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - ADC VBAT DIV Power Enable ( if the ADCPWRCTRLSWE bit is set )"]
    #[inline(always)]
    pub fn adcvbatdiven(&self) -> AdcvbatdivenR {
        AdcvbatdivenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - ADC input buffer mux select"]
    #[inline(always)]
    pub fn adcinbufsel(&self) -> AdcinbufselR {
        AdcinbufselR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - ADC Input Buffer Power Enable ( if the ADCPWRCTRLSWE bit is set )"]
    #[inline(always)]
    pub fn adcinbufen(&self) -> AdcinbufenR {
        AdcinbufenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC reference buffer slew enable"]
    #[inline(always)]
    pub fn adcrfbufslwen(&self) -> AdcrfbufslwenR {
        AdcrfbufslwenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC reference keeper out en"]
    #[inline(always)]
    pub fn adckeepouten(&self) -> AdckeepoutenR {
        AdckeepoutenR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Power Control Software Override Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcpwrctrlswe(&mut self) -> AdcpwrctrlsweW<AdcpwrctrlSpec> {
        AdcpwrctrlsweW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable the Global ADC Power Switch on when set to 1 if the ADCPWRCTRLSWE bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn adcapsen(&mut self) -> AdcapsenW<AdcpwrctrlSpec> {
        AdcapsenW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable the Analog, IO and SAR Digital logic Power Switch on when set to 1 if the ADCPWRCTRLSWE bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn adcbpsen(&mut self) -> AdcbpsenW<AdcpwrctrlSpec> {
        AdcbpsenW::new(self, 2)
    }
    #[doc = "Bit 3 - Bandgap and Temperature Sensor Power Switch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bgtpen(&mut self) -> BgtpenW<AdcpwrctrlSpec> {
        BgtpenW::new(self, 3)
    }
    #[doc = "Bit 4 - Bandgap and Temperature Sensor Power Switch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bgtlppen(&mut self) -> BgtlppenW<AdcpwrctrlSpec> {
        BgtlppenW::new(self, 4)
    }
    #[doc = "Bit 5 - Reference Buffer Power Switch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn refbufpen(&mut self) -> RefbufpenW<AdcpwrctrlSpec> {
        RefbufpenW::new(self, 5)
    }
    #[doc = "Bit 6 - Reference Buffer Keeper Power Switch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn refkeeppen(&mut self) -> RefkeeppenW<AdcpwrctrlSpec> {
        RefkeeppenW::new(self, 6)
    }
    #[doc = "Bit 7 - ISOLATE signal for Power Switched SAR ( when ADCBPSEN is switched off )"]
    #[inline(always)]
    #[must_use]
    pub fn vddadcsarisolate(&mut self) -> VddadcsarisolateW<AdcpwrctrlSpec> {
        VddadcsarisolateW::new(self, 7)
    }
    #[doc = "Bit 8 - ISOLATE signal for ADC Digital Contoller ( when ADCAPSEN is switched off and if the ADCPWRCTRLSWE bit is set)"]
    #[inline(always)]
    #[must_use]
    pub fn vddadcdigisolate(&mut self) -> VddadcdigisolateW<AdcpwrctrlSpec> {
        VddadcdigisolateW::new(self, 8)
    }
    #[doc = "Bit 9 - RESETN signal for Power Switched SAR and Digital Controller (when global power switch is off and if the ADCPWRCTRLSWE bit is set)"]
    #[inline(always)]
    #[must_use]
    pub fn vddadcresetn(&mut self) -> VddadcresetnW<AdcpwrctrlSpec> {
        VddadcresetnW::new(self, 9)
    }
    #[doc = "Bit 11 - ADC VBAT DIV Power Enable ( if the ADCPWRCTRLSWE bit is set )"]
    #[inline(always)]
    #[must_use]
    pub fn adcvbatdiven(&mut self) -> AdcvbatdivenW<AdcpwrctrlSpec> {
        AdcvbatdivenW::new(self, 11)
    }
    #[doc = "Bits 12:13 - ADC input buffer mux select"]
    #[inline(always)]
    #[must_use]
    pub fn adcinbufsel(&mut self) -> AdcinbufselW<AdcpwrctrlSpec> {
        AdcinbufselW::new(self, 12)
    }
    #[doc = "Bit 14 - ADC Input Buffer Power Enable ( if the ADCPWRCTRLSWE bit is set )"]
    #[inline(always)]
    #[must_use]
    pub fn adcinbufen(&mut self) -> AdcinbufenW<AdcpwrctrlSpec> {
        AdcinbufenW::new(self, 14)
    }
    #[doc = "Bit 15 - ADC reference buffer slew enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcrfbufslwen(&mut self) -> AdcrfbufslwenW<AdcpwrctrlSpec> {
        AdcrfbufslwenW::new(self, 15)
    }
    #[doc = "Bit 16 - ADC reference keeper out en"]
    #[inline(always)]
    #[must_use]
    pub fn adckeepouten(&mut self) -> AdckeepoutenW<AdcpwrctrlSpec> {
        AdckeepoutenW::new(self, 16)
    }
}
#[doc = "ADC Power Control\n\nYou can [`read`](crate::Reg::read) this register and get [`adcpwrctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcpwrctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcpwrctrlSpec;
impl crate::RegisterSpec for AdcpwrctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcpwrctrl::R`](R) reader structure"]
impl crate::Readable for AdcpwrctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`adcpwrctrl::W`](W) writer structure"]
impl crate::Writable for AdcpwrctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCPWRCTRL to value 0x10"]
impl crate::Resettable for AdcpwrctrlSpec {
    const RESET_VALUE: u32 = 0x10;
}
