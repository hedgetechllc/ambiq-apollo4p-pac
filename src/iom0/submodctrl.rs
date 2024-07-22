#[doc = "Register `SUBMODCTRL` reader"]
pub type R = crate::R<SubmodctrlSpec>;
#[doc = "Register `SUBMODCTRL` writer"]
pub type W = crate::W<SubmodctrlSpec>;
#[doc = "Field `SMOD0EN` reader - Submodule 0 enable (1) or disable (0)"]
pub type Smod0enR = crate::BitReader;
#[doc = "Field `SMOD0EN` writer - Submodule 0 enable (1) or disable (0)"]
pub type Smod0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Submodule 0 module type. This is the SPI Master interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Smod0type {
    #[doc = "0: MSPI submodule"]
    Mspi = 0,
    #[doc = "1: I2C Master submodule"]
    Mi2c = 1,
    #[doc = "2: I2S Master/Slave Module"]
    Msi2s = 2,
    #[doc = "7: NOT INSTALLED"]
    Na = 7,
}
impl From<Smod0type> for u8 {
    #[inline(always)]
    fn from(variant: Smod0type) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Smod0type {
    type Ux = u8;
}
impl crate::IsEnum for Smod0type {}
#[doc = "Field `SMOD0TYPE` reader - Submodule 0 module type. This is the SPI Master interface."]
pub type Smod0typeR = crate::FieldReader<Smod0type>;
impl Smod0typeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Smod0type> {
        match self.bits {
            0 => Some(Smod0type::Mspi),
            1 => Some(Smod0type::Mi2c),
            2 => Some(Smod0type::Msi2s),
            7 => Some(Smod0type::Na),
            _ => None,
        }
    }
    #[doc = "MSPI submodule"]
    #[inline(always)]
    pub fn is_mspi(&self) -> bool {
        *self == Smod0type::Mspi
    }
    #[doc = "I2C Master submodule"]
    #[inline(always)]
    pub fn is_mi2c(&self) -> bool {
        *self == Smod0type::Mi2c
    }
    #[doc = "I2S Master/Slave Module"]
    #[inline(always)]
    pub fn is_msi2s(&self) -> bool {
        *self == Smod0type::Msi2s
    }
    #[doc = "NOT INSTALLED"]
    #[inline(always)]
    pub fn is_na(&self) -> bool {
        *self == Smod0type::Na
    }
}
#[doc = "Field `SMOD0TYPE` writer - Submodule 0 module type. This is the SPI Master interface."]
pub type Smod0typeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Smod0type>;
impl<'a, REG> Smod0typeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MSPI submodule"]
    #[inline(always)]
    pub fn mspi(self) -> &'a mut crate::W<REG> {
        self.variant(Smod0type::Mspi)
    }
    #[doc = "I2C Master submodule"]
    #[inline(always)]
    pub fn mi2c(self) -> &'a mut crate::W<REG> {
        self.variant(Smod0type::Mi2c)
    }
    #[doc = "I2S Master/Slave Module"]
    #[inline(always)]
    pub fn msi2s(self) -> &'a mut crate::W<REG> {
        self.variant(Smod0type::Msi2s)
    }
    #[doc = "NOT INSTALLED"]
    #[inline(always)]
    pub fn na(self) -> &'a mut crate::W<REG> {
        self.variant(Smod0type::Na)
    }
}
#[doc = "Field `SMOD1EN` reader - Submodule 1 enable (1) or disable (0)"]
pub type Smod1enR = crate::BitReader;
#[doc = "Field `SMOD1EN` writer - Submodule 1 enable (1) or disable (0)"]
pub type Smod1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Submodule 1 module type. This is the I2C Master interface\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Smod1type {
    #[doc = "0: SPI Master submodule"]
    Mspi = 0,
    #[doc = "1: MI2C submodule"]
    Mi2c = 1,
    #[doc = "2: SPI Slave submodule"]
    Sspi = 2,
    #[doc = "3: I2C Slave submodule"]
    Si2c = 3,
    #[doc = "4: Master/Slave submodule"]
    Msi2s = 4,
    #[doc = "7: NOT INSTALLED"]
    Na = 7,
}
impl From<Smod1type> for u8 {
    #[inline(always)]
    fn from(variant: Smod1type) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Smod1type {
    type Ux = u8;
}
impl crate::IsEnum for Smod1type {}
#[doc = "Field `SMOD1TYPE` reader - Submodule 1 module type. This is the I2C Master interface"]
pub type Smod1typeR = crate::FieldReader<Smod1type>;
impl Smod1typeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Smod1type> {
        match self.bits {
            0 => Some(Smod1type::Mspi),
            1 => Some(Smod1type::Mi2c),
            2 => Some(Smod1type::Sspi),
            3 => Some(Smod1type::Si2c),
            4 => Some(Smod1type::Msi2s),
            7 => Some(Smod1type::Na),
            _ => None,
        }
    }
    #[doc = "SPI Master submodule"]
    #[inline(always)]
    pub fn is_mspi(&self) -> bool {
        *self == Smod1type::Mspi
    }
    #[doc = "MI2C submodule"]
    #[inline(always)]
    pub fn is_mi2c(&self) -> bool {
        *self == Smod1type::Mi2c
    }
    #[doc = "SPI Slave submodule"]
    #[inline(always)]
    pub fn is_sspi(&self) -> bool {
        *self == Smod1type::Sspi
    }
    #[doc = "I2C Slave submodule"]
    #[inline(always)]
    pub fn is_si2c(&self) -> bool {
        *self == Smod1type::Si2c
    }
    #[doc = "Master/Slave submodule"]
    #[inline(always)]
    pub fn is_msi2s(&self) -> bool {
        *self == Smod1type::Msi2s
    }
    #[doc = "NOT INSTALLED"]
    #[inline(always)]
    pub fn is_na(&self) -> bool {
        *self == Smod1type::Na
    }
}
#[doc = "Field `SMOD1TYPE` writer - Submodule 1 module type. This is the I2C Master interface"]
pub type Smod1typeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Smod1type>;
impl<'a, REG> Smod1typeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPI Master submodule"]
    #[inline(always)]
    pub fn mspi(self) -> &'a mut crate::W<REG> {
        self.variant(Smod1type::Mspi)
    }
    #[doc = "MI2C submodule"]
    #[inline(always)]
    pub fn mi2c(self) -> &'a mut crate::W<REG> {
        self.variant(Smod1type::Mi2c)
    }
    #[doc = "SPI Slave submodule"]
    #[inline(always)]
    pub fn sspi(self) -> &'a mut crate::W<REG> {
        self.variant(Smod1type::Sspi)
    }
    #[doc = "I2C Slave submodule"]
    #[inline(always)]
    pub fn si2c(self) -> &'a mut crate::W<REG> {
        self.variant(Smod1type::Si2c)
    }
    #[doc = "Master/Slave submodule"]
    #[inline(always)]
    pub fn msi2s(self) -> &'a mut crate::W<REG> {
        self.variant(Smod1type::Msi2s)
    }
    #[doc = "NOT INSTALLED"]
    #[inline(always)]
    pub fn na(self) -> &'a mut crate::W<REG> {
        self.variant(Smod1type::Na)
    }
}
#[doc = "Field `SMOD2EN` reader - Submodule 2 enable (1) or disable (0)"]
pub type Smod2enR = crate::BitReader;
#[doc = "Field `SMOD2EN` writer - Submodule 2 enable (1) or disable (0)"]
pub type Smod2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Submodule 2 module type. This is the I2S Master/Slave interface\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Smod2type {
    #[doc = "0: SPI Master submodule"]
    Mspi = 0,
    #[doc = "1: MI2C submodule"]
    Mi2c = 1,
    #[doc = "2: SPI Slave submodule"]
    Sspi = 2,
    #[doc = "3: I2C Slave submodule"]
    Si2c = 3,
    #[doc = "4: Master/Slave submodule"]
    Msi2s = 4,
    #[doc = "7: NOT INSTALLED"]
    Na = 7,
}
impl From<Smod2type> for u8 {
    #[inline(always)]
    fn from(variant: Smod2type) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Smod2type {
    type Ux = u8;
}
impl crate::IsEnum for Smod2type {}
#[doc = "Field `SMOD2TYPE` reader - Submodule 2 module type. This is the I2S Master/Slave interface"]
pub type Smod2typeR = crate::FieldReader<Smod2type>;
impl Smod2typeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Smod2type> {
        match self.bits {
            0 => Some(Smod2type::Mspi),
            1 => Some(Smod2type::Mi2c),
            2 => Some(Smod2type::Sspi),
            3 => Some(Smod2type::Si2c),
            4 => Some(Smod2type::Msi2s),
            7 => Some(Smod2type::Na),
            _ => None,
        }
    }
    #[doc = "SPI Master submodule"]
    #[inline(always)]
    pub fn is_mspi(&self) -> bool {
        *self == Smod2type::Mspi
    }
    #[doc = "MI2C submodule"]
    #[inline(always)]
    pub fn is_mi2c(&self) -> bool {
        *self == Smod2type::Mi2c
    }
    #[doc = "SPI Slave submodule"]
    #[inline(always)]
    pub fn is_sspi(&self) -> bool {
        *self == Smod2type::Sspi
    }
    #[doc = "I2C Slave submodule"]
    #[inline(always)]
    pub fn is_si2c(&self) -> bool {
        *self == Smod2type::Si2c
    }
    #[doc = "Master/Slave submodule"]
    #[inline(always)]
    pub fn is_msi2s(&self) -> bool {
        *self == Smod2type::Msi2s
    }
    #[doc = "NOT INSTALLED"]
    #[inline(always)]
    pub fn is_na(&self) -> bool {
        *self == Smod2type::Na
    }
}
#[doc = "Field `SMOD2TYPE` writer - Submodule 2 module type. This is the I2S Master/Slave interface"]
pub type Smod2typeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Smod2type>;
impl<'a, REG> Smod2typeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPI Master submodule"]
    #[inline(always)]
    pub fn mspi(self) -> &'a mut crate::W<REG> {
        self.variant(Smod2type::Mspi)
    }
    #[doc = "MI2C submodule"]
    #[inline(always)]
    pub fn mi2c(self) -> &'a mut crate::W<REG> {
        self.variant(Smod2type::Mi2c)
    }
    #[doc = "SPI Slave submodule"]
    #[inline(always)]
    pub fn sspi(self) -> &'a mut crate::W<REG> {
        self.variant(Smod2type::Sspi)
    }
    #[doc = "I2C Slave submodule"]
    #[inline(always)]
    pub fn si2c(self) -> &'a mut crate::W<REG> {
        self.variant(Smod2type::Si2c)
    }
    #[doc = "Master/Slave submodule"]
    #[inline(always)]
    pub fn msi2s(self) -> &'a mut crate::W<REG> {
        self.variant(Smod2type::Msi2s)
    }
    #[doc = "NOT INSTALLED"]
    #[inline(always)]
    pub fn na(self) -> &'a mut crate::W<REG> {
        self.variant(Smod2type::Na)
    }
}
impl R {
    #[doc = "Bit 0 - Submodule 0 enable (1) or disable (0)"]
    #[inline(always)]
    pub fn smod0en(&self) -> Smod0enR {
        Smod0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Submodule 0 module type. This is the SPI Master interface."]
    #[inline(always)]
    pub fn smod0type(&self) -> Smod0typeR {
        Smod0typeR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Submodule 1 enable (1) or disable (0)"]
    #[inline(always)]
    pub fn smod1en(&self) -> Smod1enR {
        Smod1enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Submodule 1 module type. This is the I2C Master interface"]
    #[inline(always)]
    pub fn smod1type(&self) -> Smod1typeR {
        Smod1typeR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Submodule 2 enable (1) or disable (0)"]
    #[inline(always)]
    pub fn smod2en(&self) -> Smod2enR {
        Smod2enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Submodule 2 module type. This is the I2S Master/Slave interface"]
    #[inline(always)]
    pub fn smod2type(&self) -> Smod2typeR {
        Smod2typeR::new(((self.bits >> 9) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Submodule 0 enable (1) or disable (0)"]
    #[inline(always)]
    #[must_use]
    pub fn smod0en(&mut self) -> Smod0enW<SubmodctrlSpec> {
        Smod0enW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Submodule 0 module type. This is the SPI Master interface."]
    #[inline(always)]
    #[must_use]
    pub fn smod0type(&mut self) -> Smod0typeW<SubmodctrlSpec> {
        Smod0typeW::new(self, 1)
    }
    #[doc = "Bit 4 - Submodule 1 enable (1) or disable (0)"]
    #[inline(always)]
    #[must_use]
    pub fn smod1en(&mut self) -> Smod1enW<SubmodctrlSpec> {
        Smod1enW::new(self, 4)
    }
    #[doc = "Bits 5:7 - Submodule 1 module type. This is the I2C Master interface"]
    #[inline(always)]
    #[must_use]
    pub fn smod1type(&mut self) -> Smod1typeW<SubmodctrlSpec> {
        Smod1typeW::new(self, 5)
    }
    #[doc = "Bit 8 - Submodule 2 enable (1) or disable (0)"]
    #[inline(always)]
    #[must_use]
    pub fn smod2en(&mut self) -> Smod2enW<SubmodctrlSpec> {
        Smod2enW::new(self, 8)
    }
    #[doc = "Bits 9:11 - Submodule 2 module type. This is the I2S Master/Slave interface"]
    #[inline(always)]
    #[must_use]
    pub fn smod2type(&mut self) -> Smod2typeW<SubmodctrlSpec> {
        Smod2typeW::new(self, 9)
    }
}
#[doc = "Provides enable for each submodule. Only a sigle submodule can be enabled at one time.\n\nYou can [`read`](crate::Reg::read) this register and get [`submodctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`submodctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SubmodctrlSpec;
impl crate::RegisterSpec for SubmodctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`submodctrl::R`](R) reader structure"]
impl crate::Readable for SubmodctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`submodctrl::W`](W) writer structure"]
impl crate::Writable for SubmodctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUBMODCTRL to value 0x0820"]
impl crate::Resettable for SubmodctrlSpec {
    const RESET_VALUE: u32 = 0x0820;
}
