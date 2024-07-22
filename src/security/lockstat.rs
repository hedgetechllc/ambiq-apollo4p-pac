#[doc = "Register `LOCKSTAT` reader"]
pub type R = crate::R<LockstatSpec>;
#[doc = "Register `LOCKSTAT` writer"]
pub type W = crate::W<LockstatSpec>;
#[doc = "Lock status. Bit is high to signify it is enabled. 0: LOCK01, 1: LOCK02, 4: LOCK11, 5: LOCK12, 30: LOCK9D, 31: LOCK9E\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Status {
    #[doc = "1: Enabled customer OTP program."]
    Lock01 = 1,
    #[doc = "2: Enabled customer OTP read."]
    Lock02 = 2,
    #[doc = "16: Enabled Ambiq OTP program."]
    Lock11 = 16,
    #[doc = "32: Enabled Ambiq OTP read."]
    Lock12 = 32,
    #[doc = "1073741824: Enabled Bootloader Recovery."]
    Lock9d = 1073741824,
    #[doc = "2147483648: Enabled Bootloader info1."]
    Lock9e = 2147483648,
    #[doc = "0: All Reset."]
    Reset = 0,
}
impl From<Status> for u32 {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Status {
    type Ux = u32;
}
impl crate::IsEnum for Status {}
#[doc = "Field `STATUS` reader - Lock status. Bit is high to signify it is enabled. 0: LOCK01, 1: LOCK02, 4: LOCK11, 5: LOCK12, 30: LOCK9D, 31: LOCK9E"]
pub type StatusR = crate::FieldReader<Status>;
impl StatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Status> {
        match self.bits {
            1 => Some(Status::Lock01),
            2 => Some(Status::Lock02),
            16 => Some(Status::Lock11),
            32 => Some(Status::Lock12),
            1073741824 => Some(Status::Lock9d),
            2147483648 => Some(Status::Lock9e),
            0 => Some(Status::Reset),
            _ => None,
        }
    }
    #[doc = "Enabled customer OTP program."]
    #[inline(always)]
    pub fn is_lock01(&self) -> bool {
        *self == Status::Lock01
    }
    #[doc = "Enabled customer OTP read."]
    #[inline(always)]
    pub fn is_lock02(&self) -> bool {
        *self == Status::Lock02
    }
    #[doc = "Enabled Ambiq OTP program."]
    #[inline(always)]
    pub fn is_lock11(&self) -> bool {
        *self == Status::Lock11
    }
    #[doc = "Enabled Ambiq OTP read."]
    #[inline(always)]
    pub fn is_lock12(&self) -> bool {
        *self == Status::Lock12
    }
    #[doc = "Enabled Bootloader Recovery."]
    #[inline(always)]
    pub fn is_lock9d(&self) -> bool {
        *self == Status::Lock9d
    }
    #[doc = "Enabled Bootloader info1."]
    #[inline(always)]
    pub fn is_lock9e(&self) -> bool {
        *self == Status::Lock9e
    }
    #[doc = "All Reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Status::Reset
    }
}
#[doc = "Field `STATUS` writer - Lock status. Bit is high to signify it is enabled. 0: LOCK01, 1: LOCK02, 4: LOCK11, 5: LOCK12, 30: LOCK9D, 31: LOCK9E"]
pub type StatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, Status>;
impl<'a, REG> StatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Enabled customer OTP program."]
    #[inline(always)]
    pub fn lock01(self) -> &'a mut crate::W<REG> {
        self.variant(Status::Lock01)
    }
    #[doc = "Enabled customer OTP read."]
    #[inline(always)]
    pub fn lock02(self) -> &'a mut crate::W<REG> {
        self.variant(Status::Lock02)
    }
    #[doc = "Enabled Ambiq OTP program."]
    #[inline(always)]
    pub fn lock11(self) -> &'a mut crate::W<REG> {
        self.variant(Status::Lock11)
    }
    #[doc = "Enabled Ambiq OTP read."]
    #[inline(always)]
    pub fn lock12(self) -> &'a mut crate::W<REG> {
        self.variant(Status::Lock12)
    }
    #[doc = "Enabled Bootloader Recovery."]
    #[inline(always)]
    pub fn lock9d(self) -> &'a mut crate::W<REG> {
        self.variant(Status::Lock9d)
    }
    #[doc = "Enabled Bootloader info1."]
    #[inline(always)]
    pub fn lock9e(self) -> &'a mut crate::W<REG> {
        self.variant(Status::Lock9e)
    }
    #[doc = "All Reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Status::Reset)
    }
}
impl R {
    #[doc = "Bits 0:31 - Lock status. Bit is high to signify it is enabled. 0: LOCK01, 1: LOCK02, 4: LOCK11, 5: LOCK12, 30: LOCK9D, 31: LOCK9E"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Lock status. Bit is high to signify it is enabled. 0: LOCK01, 1: LOCK02, 4: LOCK11, 5: LOCK12, 30: LOCK9D, 31: LOCK9E"]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> StatusW<LockstatSpec> {
        StatusW::new(self, 0)
    }
}
#[doc = "LOCK Status\n\nYou can [`read`](crate::Reg::read) this register and get [`lockstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockstatSpec;
impl crate::RegisterSpec for LockstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lockstat::R`](R) reader structure"]
impl crate::Readable for LockstatSpec {}
#[doc = "`write(|w| ..)` method takes [`lockstat::W`](W) writer structure"]
impl crate::Writable for LockstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOCKSTAT to value 0"]
impl crate::Resettable for LockstatSpec {
    const RESET_VALUE: u32 = 0;
}
