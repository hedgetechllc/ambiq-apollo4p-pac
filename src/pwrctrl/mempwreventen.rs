#[doc = "Register `MEMPWREVENTEN` reader"]
pub type R = crate::R<MempwreventenSpec>;
#[doc = "Register `MEMPWREVENTEN` writer"]
pub type W = crate::W<MempwreventenSpec>;
#[doc = "Enable DTCM power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dtcmen {
    #[doc = "0: Do not enable DTCM power-on status event"]
    None = 0,
    #[doc = "1: Enable GROUP0_DTCM0 power on status event"]
    Group0dtcm0en = 1,
    #[doc = "2: Enable GROUP0_DTCM1 power on status event"]
    Group0dtcm1en = 2,
    #[doc = "3: Enable DTCMs in group0 power on status event"]
    Group0en = 3,
    #[doc = "4: Enable DTCMs in group1 power on status event"]
    Group1en = 4,
    #[doc = "7: Enable all DTCM power on status event"]
    All = 7,
}
impl From<Dtcmen> for u8 {
    #[inline(always)]
    fn from(variant: Dtcmen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dtcmen {
    type Ux = u8;
}
impl crate::IsEnum for Dtcmen {}
#[doc = "Field `DTCMEN` reader - Enable DTCM power-on status event"]
pub type DtcmenR = crate::FieldReader<Dtcmen>;
impl DtcmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dtcmen> {
        match self.bits {
            0 => Some(Dtcmen::None),
            1 => Some(Dtcmen::Group0dtcm0en),
            2 => Some(Dtcmen::Group0dtcm1en),
            3 => Some(Dtcmen::Group0en),
            4 => Some(Dtcmen::Group1en),
            7 => Some(Dtcmen::All),
            _ => None,
        }
    }
    #[doc = "Do not enable DTCM power-on status event"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Dtcmen::None
    }
    #[doc = "Enable GROUP0_DTCM0 power on status event"]
    #[inline(always)]
    pub fn is_group0dtcm0en(&self) -> bool {
        *self == Dtcmen::Group0dtcm0en
    }
    #[doc = "Enable GROUP0_DTCM1 power on status event"]
    #[inline(always)]
    pub fn is_group0dtcm1en(&self) -> bool {
        *self == Dtcmen::Group0dtcm1en
    }
    #[doc = "Enable DTCMs in group0 power on status event"]
    #[inline(always)]
    pub fn is_group0en(&self) -> bool {
        *self == Dtcmen::Group0en
    }
    #[doc = "Enable DTCMs in group1 power on status event"]
    #[inline(always)]
    pub fn is_group1en(&self) -> bool {
        *self == Dtcmen::Group1en
    }
    #[doc = "Enable all DTCM power on status event"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == Dtcmen::All
    }
}
#[doc = "Field `DTCMEN` writer - Enable DTCM power-on status event"]
pub type DtcmenW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dtcmen>;
impl<'a, REG> DtcmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do not enable DTCM power-on status event"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcmen::None)
    }
    #[doc = "Enable GROUP0_DTCM0 power on status event"]
    #[inline(always)]
    pub fn group0dtcm0en(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcmen::Group0dtcm0en)
    }
    #[doc = "Enable GROUP0_DTCM1 power on status event"]
    #[inline(always)]
    pub fn group0dtcm1en(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcmen::Group0dtcm1en)
    }
    #[doc = "Enable DTCMs in group0 power on status event"]
    #[inline(always)]
    pub fn group0en(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcmen::Group0en)
    }
    #[doc = "Enable DTCMs in group1 power on status event"]
    #[inline(always)]
    pub fn group1en(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcmen::Group1en)
    }
    #[doc = "Enable all DTCM power on status event"]
    #[inline(always)]
    pub fn all(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcmen::All)
    }
}
#[doc = "Control NVM power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nvm0en {
    #[doc = "1: Enable NVM status event"]
    En = 1,
    #[doc = "0: Disables NVM status event"]
    Dis = 0,
}
impl From<Nvm0en> for bool {
    #[inline(always)]
    fn from(variant: Nvm0en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NVM0EN` reader - Control NVM power-on status event"]
pub type Nvm0enR = crate::BitReader<Nvm0en>;
impl Nvm0enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nvm0en {
        match self.bits {
            true => Nvm0en::En,
            false => Nvm0en::Dis,
        }
    }
    #[doc = "Enable NVM status event"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Nvm0en::En
    }
    #[doc = "Disables NVM status event"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Nvm0en::Dis
    }
}
#[doc = "Field `NVM0EN` writer - Control NVM power-on status event"]
pub type Nvm0enW<'a, REG> = crate::BitWriter<'a, REG, Nvm0en>;
impl<'a, REG> Nvm0enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable NVM status event"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Nvm0en::En)
    }
    #[doc = "Disables NVM status event"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Nvm0en::Dis)
    }
}
#[doc = "Control CACHE BANK 0 power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cacheb0en {
    #[doc = "1: Enable CACHE BANK 0 status event"]
    En = 1,
    #[doc = "0: Disable CACHE BANK 0 status event"]
    Dis = 0,
}
impl From<Cacheb0en> for bool {
    #[inline(always)]
    fn from(variant: Cacheb0en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CACHEB0EN` reader - Control CACHE BANK 0 power-on status event"]
pub type Cacheb0enR = crate::BitReader<Cacheb0en>;
impl Cacheb0enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cacheb0en {
        match self.bits {
            true => Cacheb0en::En,
            false => Cacheb0en::Dis,
        }
    }
    #[doc = "Enable CACHE BANK 0 status event"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Cacheb0en::En
    }
    #[doc = "Disable CACHE BANK 0 status event"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Cacheb0en::Dis
    }
}
#[doc = "Field `CACHEB0EN` writer - Control CACHE BANK 0 power-on status event"]
pub type Cacheb0enW<'a, REG> = crate::BitWriter<'a, REG, Cacheb0en>;
impl<'a, REG> Cacheb0enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable CACHE BANK 0 status event"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Cacheb0en::En)
    }
    #[doc = "Disable CACHE BANK 0 status event"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Cacheb0en::Dis)
    }
}
#[doc = "Control CACHEB2 power-on status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cacheb2en {
    #[doc = "1: Enable CACHE BANK 2 status event"]
    En = 1,
    #[doc = "0: Disable CACHE BANK 2 status event"]
    Dis = 0,
}
impl From<Cacheb2en> for bool {
    #[inline(always)]
    fn from(variant: Cacheb2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CACHEB2EN` reader - Control CACHEB2 power-on status event"]
pub type Cacheb2enR = crate::BitReader<Cacheb2en>;
impl Cacheb2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cacheb2en {
        match self.bits {
            true => Cacheb2en::En,
            false => Cacheb2en::Dis,
        }
    }
    #[doc = "Enable CACHE BANK 2 status event"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Cacheb2en::En
    }
    #[doc = "Disable CACHE BANK 2 status event"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Cacheb2en::Dis
    }
}
#[doc = "Field `CACHEB2EN` writer - Control CACHEB2 power-on status event"]
pub type Cacheb2enW<'a, REG> = crate::BitWriter<'a, REG, Cacheb2en>;
impl<'a, REG> Cacheb2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable CACHE BANK 2 status event"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Cacheb2en::En)
    }
    #[doc = "Disable CACHE BANK 2 status event"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Cacheb2en::Dis)
    }
}
impl R {
    #[doc = "Bits 0:2 - Enable DTCM power-on status event"]
    #[inline(always)]
    pub fn dtcmen(&self) -> DtcmenR {
        DtcmenR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Control NVM power-on status event"]
    #[inline(always)]
    pub fn nvm0en(&self) -> Nvm0enR {
        Nvm0enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Control CACHE BANK 0 power-on status event"]
    #[inline(always)]
    pub fn cacheb0en(&self) -> Cacheb0enR {
        Cacheb0enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Control CACHEB2 power-on status event"]
    #[inline(always)]
    pub fn cacheb2en(&self) -> Cacheb2enR {
        Cacheb2enR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Enable DTCM power-on status event"]
    #[inline(always)]
    #[must_use]
    pub fn dtcmen(&mut self) -> DtcmenW<MempwreventenSpec> {
        DtcmenW::new(self, 0)
    }
    #[doc = "Bit 3 - Control NVM power-on status event"]
    #[inline(always)]
    #[must_use]
    pub fn nvm0en(&mut self) -> Nvm0enW<MempwreventenSpec> {
        Nvm0enW::new(self, 3)
    }
    #[doc = "Bit 4 - Control CACHE BANK 0 power-on status event"]
    #[inline(always)]
    #[must_use]
    pub fn cacheb0en(&mut self) -> Cacheb0enW<MempwreventenSpec> {
        Cacheb0enW::new(self, 4)
    }
    #[doc = "Bit 5 - Control CACHEB2 power-on status event"]
    #[inline(always)]
    #[must_use]
    pub fn cacheb2en(&mut self) -> Cacheb2enW<MempwreventenSpec> {
        Cacheb2enW::new(self, 5)
    }
}
#[doc = "This register controls which power enable for the memories will result in an event to the CPU. It includes all the power on status for the memory domains. If any bits are set, then if the domain is turned on, it will result in an event to the ARM core.\n\nYou can [`read`](crate::Reg::read) this register and get [`mempwreventen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mempwreventen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MempwreventenSpec;
impl crate::RegisterSpec for MempwreventenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mempwreventen::R`](R) reader structure"]
impl crate::Readable for MempwreventenSpec {}
#[doc = "`write(|w| ..)` method takes [`mempwreventen::W`](W) writer structure"]
impl crate::Writable for MempwreventenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMPWREVENTEN to value 0"]
impl crate::Resettable for MempwreventenSpec {
    const RESET_VALUE: u32 = 0;
}
