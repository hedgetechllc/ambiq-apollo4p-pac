#[doc = "Register `HOSTAOLOCKBITS` reader"]
pub type R = crate::R<HostaolockbitsSpec>;
#[doc = "Register `HOSTAOLOCKBITS` writer"]
pub type W = crate::W<HostaolockbitsSpec>;
#[doc = "Field `HOSTFATALERR` reader - When the 'FATAL_ERROR' register is asserted - HW keys will not be copied from OTP"]
pub type HostfatalerrR = crate::BitReader;
#[doc = "Field `HOSTFATALERR` writer - When the 'FATAL_ERROR' register is asserted - HW keys will not be copied from OTP"]
pub type HostfatalerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOSTKPICVLOCK` reader - When this FW controlled register is set, the Kpicv HW key is masked (to zero)."]
pub type HostkpicvlockR = crate::BitReader;
#[doc = "Field `HOSTKPICVLOCK` writer - When this FW controlled register is set, the Kpicv HW key is masked (to zero)."]
pub type HostkpicvlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOSTKCEICVLOCK` reader - When this FW controlled register is set, the Kceicv HW key is masked (to zero)."]
pub type HostkceicvlockR = crate::BitReader;
#[doc = "Field `HOSTKCEICVLOCK` writer - When this FW controlled register is set, the Kceicv HW key is masked (to zero)."]
pub type HostkceicvlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOSTKCPLOCK` reader - When this FW controlled register is set, the Kcp HW key is masked (to zero)."]
pub type HostkcplockR = crate::BitReader;
#[doc = "Field `HOSTKCPLOCK` writer - When this FW controlled register is set, the Kcp HW key is masked (to zero)."]
pub type HostkcplockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOSTKCELOCK` reader - When this FW controlled register is set, the Kce HW key is masked (to zero)."]
pub type HostkcelockR = crate::BitReader;
#[doc = "Field `HOSTKCELOCK` writer - When this FW controlled register is set, the Kce HW key is masked (to zero)."]
pub type HostkcelockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOSTICVRMALOCK` reader - The ICV_RMA_LOCK register is set-once (per POR)."]
pub type HosticvrmalockR = crate::BitReader;
#[doc = "Field `HOSTICVRMALOCK` writer - The ICV_RMA_LOCK register is set-once (per POR)."]
pub type HosticvrmalockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETUPONDEBUGDISABLE` reader - The RESET_UPON_DEBUG_DISABLE register is set-once (per POR)."]
pub type ResetupondebugdisableR = crate::BitReader;
#[doc = "Field `RESETUPONDEBUGDISABLE` writer - The RESET_UPON_DEBUG_DISABLE register is set-once (per POR)."]
pub type ResetupondebugdisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOSTFORCEDFAENABLE` reader - When this FW controlled register is set, the AES DFA countermeasures are enabled_disabled (regardless of the AES_DFA_IS_ON register value)."]
pub type HostforcedfaenableR = crate::BitReader;
#[doc = "Field `HOSTFORCEDFAENABLE` writer - When this FW controlled register is set, the AES DFA countermeasures are enabled_disabled (regardless of the AES_DFA_IS_ON register value)."]
pub type HostforcedfaenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOSTDFAENABLELOCK` reader - When this FW control is set, the DFA_ENABLE register cant be written until the next POR. The DFA_ENABLE_LOCK register is set-once (per POR)."]
pub type HostdfaenablelockR = crate::BitReader;
#[doc = "Field `HOSTDFAENABLELOCK` writer - When this FW control is set, the DFA_ENABLE register cant be written until the next POR. The DFA_ENABLE_LOCK register is set-once (per POR)."]
pub type HostdfaenablelockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When the 'FATAL_ERROR' register is asserted - HW keys will not be copied from OTP"]
    #[inline(always)]
    pub fn hostfatalerr(&self) -> HostfatalerrR {
        HostfatalerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When this FW controlled register is set, the Kpicv HW key is masked (to zero)."]
    #[inline(always)]
    pub fn hostkpicvlock(&self) -> HostkpicvlockR {
        HostkpicvlockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When this FW controlled register is set, the Kceicv HW key is masked (to zero)."]
    #[inline(always)]
    pub fn hostkceicvlock(&self) -> HostkceicvlockR {
        HostkceicvlockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When this FW controlled register is set, the Kcp HW key is masked (to zero)."]
    #[inline(always)]
    pub fn hostkcplock(&self) -> HostkcplockR {
        HostkcplockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When this FW controlled register is set, the Kce HW key is masked (to zero)."]
    #[inline(always)]
    pub fn hostkcelock(&self) -> HostkcelockR {
        HostkcelockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The ICV_RMA_LOCK register is set-once (per POR)."]
    #[inline(always)]
    pub fn hosticvrmalock(&self) -> HosticvrmalockR {
        HosticvrmalockR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The RESET_UPON_DEBUG_DISABLE register is set-once (per POR)."]
    #[inline(always)]
    pub fn resetupondebugdisable(&self) -> ResetupondebugdisableR {
        ResetupondebugdisableR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When this FW controlled register is set, the AES DFA countermeasures are enabled_disabled (regardless of the AES_DFA_IS_ON register value)."]
    #[inline(always)]
    pub fn hostforcedfaenable(&self) -> HostforcedfaenableR {
        HostforcedfaenableR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - When this FW control is set, the DFA_ENABLE register cant be written until the next POR. The DFA_ENABLE_LOCK register is set-once (per POR)."]
    #[inline(always)]
    pub fn hostdfaenablelock(&self) -> HostdfaenablelockR {
        HostdfaenablelockR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When the 'FATAL_ERROR' register is asserted - HW keys will not be copied from OTP"]
    #[inline(always)]
    #[must_use]
    pub fn hostfatalerr(&mut self) -> HostfatalerrW<HostaolockbitsSpec> {
        HostfatalerrW::new(self, 0)
    }
    #[doc = "Bit 1 - When this FW controlled register is set, the Kpicv HW key is masked (to zero)."]
    #[inline(always)]
    #[must_use]
    pub fn hostkpicvlock(&mut self) -> HostkpicvlockW<HostaolockbitsSpec> {
        HostkpicvlockW::new(self, 1)
    }
    #[doc = "Bit 2 - When this FW controlled register is set, the Kceicv HW key is masked (to zero)."]
    #[inline(always)]
    #[must_use]
    pub fn hostkceicvlock(&mut self) -> HostkceicvlockW<HostaolockbitsSpec> {
        HostkceicvlockW::new(self, 2)
    }
    #[doc = "Bit 3 - When this FW controlled register is set, the Kcp HW key is masked (to zero)."]
    #[inline(always)]
    #[must_use]
    pub fn hostkcplock(&mut self) -> HostkcplockW<HostaolockbitsSpec> {
        HostkcplockW::new(self, 3)
    }
    #[doc = "Bit 4 - When this FW controlled register is set, the Kce HW key is masked (to zero)."]
    #[inline(always)]
    #[must_use]
    pub fn hostkcelock(&mut self) -> HostkcelockW<HostaolockbitsSpec> {
        HostkcelockW::new(self, 4)
    }
    #[doc = "Bit 5 - The ICV_RMA_LOCK register is set-once (per POR)."]
    #[inline(always)]
    #[must_use]
    pub fn hosticvrmalock(&mut self) -> HosticvrmalockW<HostaolockbitsSpec> {
        HosticvrmalockW::new(self, 5)
    }
    #[doc = "Bit 6 - The RESET_UPON_DEBUG_DISABLE register is set-once (per POR)."]
    #[inline(always)]
    #[must_use]
    pub fn resetupondebugdisable(&mut self) -> ResetupondebugdisableW<HostaolockbitsSpec> {
        ResetupondebugdisableW::new(self, 6)
    }
    #[doc = "Bit 7 - When this FW controlled register is set, the AES DFA countermeasures are enabled_disabled (regardless of the AES_DFA_IS_ON register value)."]
    #[inline(always)]
    #[must_use]
    pub fn hostforcedfaenable(&mut self) -> HostforcedfaenableW<HostaolockbitsSpec> {
        HostforcedfaenableW::new(self, 7)
    }
    #[doc = "Bit 8 - When this FW control is set, the DFA_ENABLE register cant be written until the next POR. The DFA_ENABLE_LOCK register is set-once (per POR)."]
    #[inline(always)]
    #[must_use]
    pub fn hostdfaenablelock(&mut self) -> HostdfaenablelockW<HostaolockbitsSpec> {
        HostdfaenablelockW::new(self, 8)
    }
}
#[doc = "These masks will define, per LCS, which DCU bits will be tied to zero, even if the Host tries to set them. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostaolockbits::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostaolockbits::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostaolockbitsSpec;
impl crate::RegisterSpec for HostaolockbitsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostaolockbits::R`](R) reader structure"]
impl crate::Readable for HostaolockbitsSpec {}
#[doc = "`write(|w| ..)` method takes [`hostaolockbits::W`](W) writer structure"]
impl crate::Writable for HostaolockbitsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTAOLOCKBITS to value 0x80"]
impl crate::Resettable for HostaolockbitsSpec {
    const RESET_VALUE: u32 = 0x80;
}
