#[doc = "Register `NVMDEBUGSTATUS` reader"]
pub type R = crate::R<NvmdebugstatusSpec>;
#[doc = "Register `NVMDEBUGSTATUS` writer"]
pub type W = crate::W<NvmdebugstatusSpec>;
#[doc = "Main nvm fsm\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nvmsm {
    #[doc = "0: IDLE NVMSM"]
    Idle = 0,
    #[doc = "1: READ_DUMMY NVMSM"]
    ReadDummy = 1,
    #[doc = "2: READ_MAN_FLAG NVMSM"]
    ReadManFlag = 2,
    #[doc = "3: READ_OEM_FLAG NVMSM"]
    ReadOemFlag = 3,
    #[doc = "4: READ_GPPC NVMSM"]
    ReadGppc = 4,
    #[doc = "5: DECODE NVMSM"]
    Decode = 5,
    #[doc = "6: OTP_LCS_VALID NVMSM"]
    OtpLcsValid = 6,
    #[doc = "7: LCS_IS_VALID NVMSM"]
    LcsIsValid = 7,
}
impl From<Nvmsm> for u8 {
    #[inline(always)]
    fn from(variant: Nvmsm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nvmsm {
    type Ux = u8;
}
impl crate::IsEnum for Nvmsm {}
#[doc = "Field `NVMSM` reader - Main nvm fsm"]
pub type NvmsmR = crate::FieldReader<Nvmsm>;
impl NvmsmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nvmsm {
        match self.bits {
            0 => Nvmsm::Idle,
            1 => Nvmsm::ReadDummy,
            2 => Nvmsm::ReadManFlag,
            3 => Nvmsm::ReadOemFlag,
            4 => Nvmsm::ReadGppc,
            5 => Nvmsm::Decode,
            6 => Nvmsm::OtpLcsValid,
            7 => Nvmsm::LcsIsValid,
            _ => unreachable!(),
        }
    }
    #[doc = "IDLE NVMSM"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Nvmsm::Idle
    }
    #[doc = "READ_DUMMY NVMSM"]
    #[inline(always)]
    pub fn is_read_dummy(&self) -> bool {
        *self == Nvmsm::ReadDummy
    }
    #[doc = "READ_MAN_FLAG NVMSM"]
    #[inline(always)]
    pub fn is_read_man_flag(&self) -> bool {
        *self == Nvmsm::ReadManFlag
    }
    #[doc = "READ_OEM_FLAG NVMSM"]
    #[inline(always)]
    pub fn is_read_oem_flag(&self) -> bool {
        *self == Nvmsm::ReadOemFlag
    }
    #[doc = "READ_GPPC NVMSM"]
    #[inline(always)]
    pub fn is_read_gppc(&self) -> bool {
        *self == Nvmsm::ReadGppc
    }
    #[doc = "DECODE NVMSM"]
    #[inline(always)]
    pub fn is_decode(&self) -> bool {
        *self == Nvmsm::Decode
    }
    #[doc = "OTP_LCS_VALID NVMSM"]
    #[inline(always)]
    pub fn is_otp_lcs_valid(&self) -> bool {
        *self == Nvmsm::OtpLcsValid
    }
    #[doc = "LCS_IS_VALID NVMSM"]
    #[inline(always)]
    pub fn is_lcs_is_valid(&self) -> bool {
        *self == Nvmsm::LcsIsValid
    }
}
#[doc = "Field `NVMSM` writer - Main nvm fsm"]
pub type NvmsmW<'a, REG> = crate::FieldWriter<'a, REG, 3, Nvmsm, crate::Safe>;
impl<'a, REG> NvmsmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IDLE NVMSM"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(Nvmsm::Idle)
    }
    #[doc = "READ_DUMMY NVMSM"]
    #[inline(always)]
    pub fn read_dummy(self) -> &'a mut crate::W<REG> {
        self.variant(Nvmsm::ReadDummy)
    }
    #[doc = "READ_MAN_FLAG NVMSM"]
    #[inline(always)]
    pub fn read_man_flag(self) -> &'a mut crate::W<REG> {
        self.variant(Nvmsm::ReadManFlag)
    }
    #[doc = "READ_OEM_FLAG NVMSM"]
    #[inline(always)]
    pub fn read_oem_flag(self) -> &'a mut crate::W<REG> {
        self.variant(Nvmsm::ReadOemFlag)
    }
    #[doc = "READ_GPPC NVMSM"]
    #[inline(always)]
    pub fn read_gppc(self) -> &'a mut crate::W<REG> {
        self.variant(Nvmsm::ReadGppc)
    }
    #[doc = "DECODE NVMSM"]
    #[inline(always)]
    pub fn decode(self) -> &'a mut crate::W<REG> {
        self.variant(Nvmsm::Decode)
    }
    #[doc = "OTP_LCS_VALID NVMSM"]
    #[inline(always)]
    pub fn otp_lcs_valid(self) -> &'a mut crate::W<REG> {
        self.variant(Nvmsm::OtpLcsValid)
    }
    #[doc = "LCS_IS_VALID NVMSM"]
    #[inline(always)]
    pub fn lcs_is_valid(self) -> &'a mut crate::W<REG> {
        self.variant(Nvmsm::LcsIsValid)
    }
}
impl R {
    #[doc = "Bits 1:3 - Main nvm fsm"]
    #[inline(always)]
    pub fn nvmsm(&self) -> NvmsmR {
        NvmsmR::new(((self.bits >> 1) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 1:3 - Main nvm fsm"]
    #[inline(always)]
    #[must_use]
    pub fn nvmsm(&mut self) -> NvmsmW<NvmdebugstatusSpec> {
        NvmsmW::new(self, 1)
    }
}
#[doc = "AIB debug status register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`nvmdebugstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvmdebugstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvmdebugstatusSpec;
impl crate::RegisterSpec for NvmdebugstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvmdebugstatus::R`](R) reader structure"]
impl crate::Readable for NvmdebugstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`nvmdebugstatus::W`](W) writer structure"]
impl crate::Writable for NvmdebugstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVMDEBUGSTATUS to value 0x0e"]
impl crate::Resettable for NvmdebugstatusSpec {
    const RESET_VALUE: u32 = 0x0e;
}
