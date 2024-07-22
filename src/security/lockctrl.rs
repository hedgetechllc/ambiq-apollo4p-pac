#[doc = "Register `LOCKCTRL` reader"]
pub type R = crate::R<LockctrlSpec>;
#[doc = "Register `LOCKCTRL` writer"]
pub type W = crate::W<LockctrlSpec>;
#[doc = "LOCK Function Select register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Select {
    #[doc = "1: Enable customer OTP program."]
    Lock01 = 1,
    #[doc = "2: Enable customer OTP read."]
    Lock02 = 2,
    #[doc = "17: Enable Ambiq OTP program."]
    Lock11 = 17,
    #[doc = "18: Enable Ambiq OTP read."]
    Lock12 = 18,
    #[doc = "157: Enable Bootloader Recovery."]
    Lock9d = 157,
    #[doc = "158: Enable Bootloader info1."]
    Lock9e = 158,
    #[doc = "0: Reset all."]
    Reset = 0,
}
impl From<Select> for u8 {
    #[inline(always)]
    fn from(variant: Select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Select {
    type Ux = u8;
}
impl crate::IsEnum for Select {}
#[doc = "Field `SELECT` reader - LOCK Function Select register."]
pub type SelectR = crate::FieldReader<Select>;
impl SelectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Select> {
        match self.bits {
            1 => Some(Select::Lock01),
            2 => Some(Select::Lock02),
            17 => Some(Select::Lock11),
            18 => Some(Select::Lock12),
            157 => Some(Select::Lock9d),
            158 => Some(Select::Lock9e),
            0 => Some(Select::Reset),
            _ => None,
        }
    }
    #[doc = "Enable customer OTP program."]
    #[inline(always)]
    pub fn is_lock01(&self) -> bool {
        *self == Select::Lock01
    }
    #[doc = "Enable customer OTP read."]
    #[inline(always)]
    pub fn is_lock02(&self) -> bool {
        *self == Select::Lock02
    }
    #[doc = "Enable Ambiq OTP program."]
    #[inline(always)]
    pub fn is_lock11(&self) -> bool {
        *self == Select::Lock11
    }
    #[doc = "Enable Ambiq OTP read."]
    #[inline(always)]
    pub fn is_lock12(&self) -> bool {
        *self == Select::Lock12
    }
    #[doc = "Enable Bootloader Recovery."]
    #[inline(always)]
    pub fn is_lock9d(&self) -> bool {
        *self == Select::Lock9d
    }
    #[doc = "Enable Bootloader info1."]
    #[inline(always)]
    pub fn is_lock9e(&self) -> bool {
        *self == Select::Lock9e
    }
    #[doc = "Reset all."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Select::Reset
    }
}
#[doc = "Field `SELECT` writer - LOCK Function Select register."]
pub type SelectW<'a, REG> = crate::FieldWriter<'a, REG, 8, Select>;
impl<'a, REG> SelectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Enable customer OTP program."]
    #[inline(always)]
    pub fn lock01(self) -> &'a mut crate::W<REG> {
        self.variant(Select::Lock01)
    }
    #[doc = "Enable customer OTP read."]
    #[inline(always)]
    pub fn lock02(self) -> &'a mut crate::W<REG> {
        self.variant(Select::Lock02)
    }
    #[doc = "Enable Ambiq OTP program."]
    #[inline(always)]
    pub fn lock11(self) -> &'a mut crate::W<REG> {
        self.variant(Select::Lock11)
    }
    #[doc = "Enable Ambiq OTP read."]
    #[inline(always)]
    pub fn lock12(self) -> &'a mut crate::W<REG> {
        self.variant(Select::Lock12)
    }
    #[doc = "Enable Bootloader Recovery."]
    #[inline(always)]
    pub fn lock9d(self) -> &'a mut crate::W<REG> {
        self.variant(Select::Lock9d)
    }
    #[doc = "Enable Bootloader info1."]
    #[inline(always)]
    pub fn lock9e(self) -> &'a mut crate::W<REG> {
        self.variant(Select::Lock9e)
    }
    #[doc = "Reset all."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Select::Reset)
    }
}
impl R {
    #[doc = "Bits 0:7 - LOCK Function Select register."]
    #[inline(always)]
    pub fn select(&self) -> SelectR {
        SelectR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LOCK Function Select register."]
    #[inline(always)]
    #[must_use]
    pub fn select(&mut self) -> SelectW<LockctrlSpec> {
        SelectW::new(self, 0)
    }
}
#[doc = "LOCK Control\n\nYou can [`read`](crate::Reg::read) this register and get [`lockctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockctrlSpec;
impl crate::RegisterSpec for LockctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lockctrl::R`](R) reader structure"]
impl crate::Readable for LockctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`lockctrl::W`](W) writer structure"]
impl crate::Writable for LockctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOCKCTRL to value 0"]
impl crate::Resettable for LockctrlSpec {
    const RESET_VALUE: u32 = 0;
}
