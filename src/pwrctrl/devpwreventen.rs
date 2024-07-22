#[doc = "Register `DEVPWREVENTEN` reader"]
pub type R = crate::R<DevpwreventenSpec>;
#[doc = "Register `DEVPWREVENTEN` writer"]
pub type W = crate::W<DevpwreventenSpec>;
#[doc = "Control MCUL power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mculeven {
    #[doc = "1: Enable MCUL power-on status event"]
    En = 1,
    #[doc = "0: Disable MCUL power-on status event"]
    Dis = 0,
}
impl From<Mculeven> for bool {
    #[inline(always)]
    fn from(variant: Mculeven) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCULEVEN` reader - Control MCUL power-on status event"]
pub type MculevenR = crate::BitReader<Mculeven>;
impl MculevenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mculeven {
        match self.bits {
            true => Mculeven::En,
            false => Mculeven::Dis,
        }
    }
    #[doc = "Enable MCUL power-on status event"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Mculeven::En
    }
    #[doc = "Disable MCUL power-on status event"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Mculeven::Dis
    }
}
#[doc = "Field `MCULEVEN` writer - Control MCUL power-on status event"]
pub type MculevenW<'a, REG> = crate::BitWriter<'a, REG, Mculeven>;
impl<'a, REG> MculevenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable MCUL power-on status event"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Mculeven::En)
    }
    #[doc = "Disable MCUL power-on status event"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Mculeven::Dis)
    }
}
#[doc = "Control MCUH power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mcuheven {
    #[doc = "1: Enable MCHU power-on status event"]
    En = 1,
    #[doc = "0: Disable MCUH power-on status event"]
    Dis = 0,
}
impl From<Mcuheven> for bool {
    #[inline(always)]
    fn from(variant: Mcuheven) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCUHEVEN` reader - Control MCUH power-on status event"]
pub type McuhevenR = crate::BitReader<Mcuheven>;
impl McuhevenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mcuheven {
        match self.bits {
            true => Mcuheven::En,
            false => Mcuheven::Dis,
        }
    }
    #[doc = "Enable MCHU power-on status event"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Mcuheven::En
    }
    #[doc = "Disable MCUH power-on status event"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Mcuheven::Dis
    }
}
#[doc = "Field `MCUHEVEN` writer - Control MCUH power-on status event"]
pub type McuhevenW<'a, REG> = crate::BitWriter<'a, REG, Mcuheven>;
impl<'a, REG> McuhevenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable MCHU power-on status event"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Mcuheven::En)
    }
    #[doc = "Disable MCUH power-on status event"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Mcuheven::Dis)
    }
}
#[doc = "Control HCPA power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hcpaeven {
    #[doc = "1: Enable HCPA power-on status event"]
    En = 1,
    #[doc = "0: Disable HCPA power-on status event"]
    Dis = 0,
}
impl From<Hcpaeven> for bool {
    #[inline(always)]
    fn from(variant: Hcpaeven) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HCPAEVEN` reader - Control HCPA power-on status event"]
pub type HcpaevenR = crate::BitReader<Hcpaeven>;
impl HcpaevenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hcpaeven {
        match self.bits {
            true => Hcpaeven::En,
            false => Hcpaeven::Dis,
        }
    }
    #[doc = "Enable HCPA power-on status event"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Hcpaeven::En
    }
    #[doc = "Disable HCPA power-on status event"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Hcpaeven::Dis
    }
}
#[doc = "Field `HCPAEVEN` writer - Control HCPA power-on status event"]
pub type HcpaevenW<'a, REG> = crate::BitWriter<'a, REG, Hcpaeven>;
impl<'a, REG> HcpaevenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable HCPA power-on status event"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Hcpaeven::En)
    }
    #[doc = "Disable HCPA power-on status event"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Hcpaeven::Dis)
    }
}
#[doc = "Control HCPB power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hcpbeven {
    #[doc = "1: Enable HCPB power-on status event"]
    En = 1,
    #[doc = "0: Disable HCPB power-on status event"]
    Dis = 0,
}
impl From<Hcpbeven> for bool {
    #[inline(always)]
    fn from(variant: Hcpbeven) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HCPBEVEN` reader - Control HCPB power-on status event"]
pub type HcpbevenR = crate::BitReader<Hcpbeven>;
impl HcpbevenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hcpbeven {
        match self.bits {
            true => Hcpbeven::En,
            false => Hcpbeven::Dis,
        }
    }
    #[doc = "Enable HCPB power-on status event"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Hcpbeven::En
    }
    #[doc = "Disable HCPB power-on status event"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Hcpbeven::Dis
    }
}
#[doc = "Field `HCPBEVEN` writer - Control HCPB power-on status event"]
pub type HcpbevenW<'a, REG> = crate::BitWriter<'a, REG, Hcpbeven>;
impl<'a, REG> HcpbevenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable HCPB power-on status event"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Hcpbeven::En)
    }
    #[doc = "Disable HCPB power-on status event"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Hcpbeven::Dis)
    }
}
#[doc = "Control HCPC power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hcpceven {
    #[doc = "1: Enable HCPC power-on status event"]
    En = 1,
    #[doc = "0: Disable HCPC power-on status event"]
    Dis = 0,
}
impl From<Hcpceven> for bool {
    #[inline(always)]
    fn from(variant: Hcpceven) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HCPCEVEN` reader - Control HCPC power-on status event"]
pub type HcpcevenR = crate::BitReader<Hcpceven>;
impl HcpcevenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hcpceven {
        match self.bits {
            true => Hcpceven::En,
            false => Hcpceven::Dis,
        }
    }
    #[doc = "Enable HCPC power-on status event"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Hcpceven::En
    }
    #[doc = "Disable HCPC power-on status event"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Hcpceven::Dis
    }
}
#[doc = "Field `HCPCEVEN` writer - Control HCPC power-on status event"]
pub type HcpcevenW<'a, REG> = crate::BitWriter<'a, REG, Hcpceven>;
impl<'a, REG> HcpcevenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable HCPC power-on status event"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Hcpceven::En)
    }
    #[doc = "Disable HCPC power-on status event"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Hcpceven::Dis)
    }
}
#[doc = "Control ADC power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adceven {
    #[doc = "1: Enable ADC power-on status event"]
    En = 1,
    #[doc = "0: Disable ADC power-on status event"]
    Dis = 0,
}
impl From<Adceven> for bool {
    #[inline(always)]
    fn from(variant: Adceven) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCEVEN` reader - Control ADC power-on status event"]
pub type AdcevenR = crate::BitReader<Adceven>;
impl AdcevenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adceven {
        match self.bits {
            true => Adceven::En,
            false => Adceven::Dis,
        }
    }
    #[doc = "Enable ADC power-on status event"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Adceven::En
    }
    #[doc = "Disable ADC power-on status event"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Adceven::Dis
    }
}
#[doc = "Field `ADCEVEN` writer - Control ADC power-on status event"]
pub type AdcevenW<'a, REG> = crate::BitWriter<'a, REG, Adceven>;
impl<'a, REG> AdcevenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable ADC power-on status event"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Adceven::En)
    }
    #[doc = "Disable ADC power-on status event"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Adceven::Dis)
    }
}
#[doc = "Control MSPI power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mspieven {
    #[doc = "1: Enable MSPI power-on status event"]
    En = 1,
    #[doc = "0: Disable MSPI power-on status event"]
    Dis = 0,
}
impl From<Mspieven> for bool {
    #[inline(always)]
    fn from(variant: Mspieven) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSPIEVEN` reader - Control MSPI power-on status event"]
pub type MspievenR = crate::BitReader<Mspieven>;
impl MspievenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mspieven {
        match self.bits {
            true => Mspieven::En,
            false => Mspieven::Dis,
        }
    }
    #[doc = "Enable MSPI power-on status event"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Mspieven::En
    }
    #[doc = "Disable MSPI power-on status event"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Mspieven::Dis
    }
}
#[doc = "Field `MSPIEVEN` writer - Control MSPI power-on status event"]
pub type MspievenW<'a, REG> = crate::BitWriter<'a, REG, Mspieven>;
impl<'a, REG> MspievenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable MSPI power-on status event"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Mspieven::En)
    }
    #[doc = "Disable MSPI power-on status event"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Mspieven::Dis)
    }
}
#[doc = "Control AUD power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Audeven {
    #[doc = "1: Enable AUD power-on status event"]
    En = 1,
    #[doc = "0: Disable AUD power-on status event"]
    Dis = 0,
}
impl From<Audeven> for bool {
    #[inline(always)]
    fn from(variant: Audeven) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUDEVEN` reader - Control AUD power-on status event"]
pub type AudevenR = crate::BitReader<Audeven>;
impl AudevenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Audeven {
        match self.bits {
            true => Audeven::En,
            false => Audeven::Dis,
        }
    }
    #[doc = "Enable AUD power-on status event"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Audeven::En
    }
    #[doc = "Disable AUD power-on status event"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Audeven::Dis
    }
}
#[doc = "Field `AUDEVEN` writer - Control AUD power-on status event"]
pub type AudevenW<'a, REG> = crate::BitWriter<'a, REG, Audeven>;
impl<'a, REG> AudevenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable AUD power-on status event"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Audeven::En)
    }
    #[doc = "Disable AUD power-on status event"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Audeven::Dis)
    }
}
impl R {
    #[doc = "Bit 0 - Control MCUL power-on status event"]
    #[inline(always)]
    pub fn mculeven(&self) -> MculevenR {
        MculevenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control MCUH power-on status event"]
    #[inline(always)]
    pub fn mcuheven(&self) -> McuhevenR {
        McuhevenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Control HCPA power-on status event"]
    #[inline(always)]
    pub fn hcpaeven(&self) -> HcpaevenR {
        HcpaevenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Control HCPB power-on status event"]
    #[inline(always)]
    pub fn hcpbeven(&self) -> HcpbevenR {
        HcpbevenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Control HCPC power-on status event"]
    #[inline(always)]
    pub fn hcpceven(&self) -> HcpcevenR {
        HcpcevenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Control ADC power-on status event"]
    #[inline(always)]
    pub fn adceven(&self) -> AdcevenR {
        AdcevenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Control MSPI power-on status event"]
    #[inline(always)]
    pub fn mspieven(&self) -> MspievenR {
        MspievenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Control AUD power-on status event"]
    #[inline(always)]
    pub fn audeven(&self) -> AudevenR {
        AudevenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control MCUL power-on status event"]
    #[inline(always)]
    #[must_use]
    pub fn mculeven(&mut self) -> MculevenW<DevpwreventenSpec> {
        MculevenW::new(self, 0)
    }
    #[doc = "Bit 1 - Control MCUH power-on status event"]
    #[inline(always)]
    #[must_use]
    pub fn mcuheven(&mut self) -> McuhevenW<DevpwreventenSpec> {
        McuhevenW::new(self, 1)
    }
    #[doc = "Bit 2 - Control HCPA power-on status event"]
    #[inline(always)]
    #[must_use]
    pub fn hcpaeven(&mut self) -> HcpaevenW<DevpwreventenSpec> {
        HcpaevenW::new(self, 2)
    }
    #[doc = "Bit 3 - Control HCPB power-on status event"]
    #[inline(always)]
    #[must_use]
    pub fn hcpbeven(&mut self) -> HcpbevenW<DevpwreventenSpec> {
        HcpbevenW::new(self, 3)
    }
    #[doc = "Bit 4 - Control HCPC power-on status event"]
    #[inline(always)]
    #[must_use]
    pub fn hcpceven(&mut self) -> HcpcevenW<DevpwreventenSpec> {
        HcpcevenW::new(self, 4)
    }
    #[doc = "Bit 5 - Control ADC power-on status event"]
    #[inline(always)]
    #[must_use]
    pub fn adceven(&mut self) -> AdcevenW<DevpwreventenSpec> {
        AdcevenW::new(self, 5)
    }
    #[doc = "Bit 6 - Control MSPI power-on status event"]
    #[inline(always)]
    #[must_use]
    pub fn mspieven(&mut self) -> MspievenW<DevpwreventenSpec> {
        MspievenW::new(self, 6)
    }
    #[doc = "Bit 7 - Control AUD power-on status event"]
    #[inline(always)]
    #[must_use]
    pub fn audeven(&mut self) -> AudevenW<DevpwreventenSpec> {
        AudevenW::new(self, 7)
    }
}
#[doc = "This register controls which feature trigger will result in an event to the CPU. It includes all the power on status for the core domains. If any bits are set, then if the domain is turned on, it will result in an event to the ARM core.\n\nYou can [`read`](crate::Reg::read) this register and get [`devpwreventen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devpwreventen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevpwreventenSpec;
impl crate::RegisterSpec for DevpwreventenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devpwreventen::R`](R) reader structure"]
impl crate::Readable for DevpwreventenSpec {}
#[doc = "`write(|w| ..)` method takes [`devpwreventen::W`](W) writer structure"]
impl crate::Writable for DevpwreventenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVPWREVENTEN to value 0"]
impl crate::Resettable for DevpwreventenSpec {
    const RESET_VALUE: u32 = 0;
}
