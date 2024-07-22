#[doc = "Register `AOAPBFILTERING` reader"]
pub type R = crate::R<AoapbfilteringSpec>;
#[doc = "Register `AOAPBFILTERING` writer"]
pub type W = crate::W<AoapbfilteringSpec>;
#[doc = "Field `ONLYSECACCESSALLOW` reader - when this FW controlled register is set, the APB slave accepts only secure accesses"]
pub type OnlysecaccessallowR = crate::BitReader;
#[doc = "Field `ONLYSECACCESSALLOW` writer - when this FW controlled register is set, the APB slave accepts only secure accesses"]
pub type OnlysecaccessallowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONLYSECACCESSALLOWLOCK` reader - when this FW controlled register is set, the ONLY_SEC_ACCESS_ALLOWED register cant be modified (until the next POR)."]
pub type OnlysecaccessallowlockR = crate::BitReader;
#[doc = "Field `ONLYSECACCESSALLOWLOCK` writer - when this FW controlled register is set, the ONLY_SEC_ACCESS_ALLOWED register cant be modified (until the next POR)."]
pub type OnlysecaccessallowlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONLYPRIVACCESSALLOW` reader - when this FW controlled register is set, the APB slave accepts only privileged accesses"]
pub type OnlyprivaccessallowR = crate::BitReader;
#[doc = "Field `ONLYPRIVACCESSALLOW` writer - when this FW controlled register is set, the APB slave accepts only privileged accesses"]
pub type OnlyprivaccessallowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONLYPRIVACCESSALLOWLOCK` reader - when this FW controlled register is set, the APBC_ONLY_PRIV_ACCESS_ALLOWED register cant be modified (until the next POR)"]
pub type OnlyprivaccessallowlockR = crate::BitReader;
#[doc = "Field `ONLYPRIVACCESSALLOWLOCK` writer - when this FW controlled register is set, the APBC_ONLY_PRIV_ACCESS_ALLOWED register cant be modified (until the next POR)"]
pub type OnlyprivaccessallowlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APBCONLYSECACCESSALLOW` reader - when this FW controlled register is set, the APB-C slave accepts only secure accesses"]
pub type ApbconlysecaccessallowR = crate::BitReader;
#[doc = "Field `APBCONLYSECACCESSALLOW` writer - when this FW controlled register is set, the APB-C slave accepts only secure accesses"]
pub type ApbconlysecaccessallowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APBCONLYSECACCESSALLOWLOCK` reader - when this FW controlled register is set, the APBC_ONLY_SEC_ACCESS_ALLOWED register cant be modified (until the next POR)."]
pub type ApbconlysecaccessallowlockR = crate::BitReader;
#[doc = "Field `APBCONLYSECACCESSALLOWLOCK` writer - when this FW controlled register is set, the APBC_ONLY_SEC_ACCESS_ALLOWED register cant be modified (until the next POR)."]
pub type ApbconlysecaccessallowlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APBCONLYPRIVACCESSALLOW` reader - when this FW controlled register is set, the APB-C slave accepts only privileged accesses"]
pub type ApbconlyprivaccessallowR = crate::BitReader;
#[doc = "Field `APBCONLYPRIVACCESSALLOW` writer - when this FW controlled register is set, the APB-C slave accepts only privileged accesses"]
pub type ApbconlyprivaccessallowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APBCONLYPRIVACCESSALLOWLOCK` reader - when this FW controlled register is set, the APBC_ONLY_PRIV_ACCESS_ALLOWED register cant be modified (until the next POR)"]
pub type ApbconlyprivaccessallowlockR = crate::BitReader;
#[doc = "Field `APBCONLYPRIVACCESSALLOWLOCK` writer - when this FW controlled register is set, the APBC_ONLY_PRIV_ACCESS_ALLOWED register cant be modified (until the next POR)"]
pub type ApbconlyprivaccessallowlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APBCONLYINSTACCESSALLOW` reader - when this FW controlled register is set, the APB-C slave accepts only instruction accesses"]
pub type ApbconlyinstaccessallowR = crate::BitReader;
#[doc = "Field `APBCONLYINSTACCESSALLOW` writer - when this FW controlled register is set, the APB-C slave accepts only instruction accesses"]
pub type ApbconlyinstaccessallowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APBCONLYINSTACCESSALLOWLOCK` reader - when this FW controlled register is set, the APBC_ONLY_INST_ACCESS_ALLOWED register cant be modified (until the next POR)"]
pub type ApbconlyinstaccessallowlockR = crate::BitReader;
#[doc = "Field `APBCONLYINSTACCESSALLOWLOCK` writer - when this FW controlled register is set, the APBC_ONLY_INST_ACCESS_ALLOWED register cant be modified (until the next POR)"]
pub type ApbconlyinstaccessallowlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - when this FW controlled register is set, the APB slave accepts only secure accesses"]
    #[inline(always)]
    pub fn onlysecaccessallow(&self) -> OnlysecaccessallowR {
        OnlysecaccessallowR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - when this FW controlled register is set, the ONLY_SEC_ACCESS_ALLOWED register cant be modified (until the next POR)."]
    #[inline(always)]
    pub fn onlysecaccessallowlock(&self) -> OnlysecaccessallowlockR {
        OnlysecaccessallowlockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - when this FW controlled register is set, the APB slave accepts only privileged accesses"]
    #[inline(always)]
    pub fn onlyprivaccessallow(&self) -> OnlyprivaccessallowR {
        OnlyprivaccessallowR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - when this FW controlled register is set, the APBC_ONLY_PRIV_ACCESS_ALLOWED register cant be modified (until the next POR)"]
    #[inline(always)]
    pub fn onlyprivaccessallowlock(&self) -> OnlyprivaccessallowlockR {
        OnlyprivaccessallowlockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - when this FW controlled register is set, the APB-C slave accepts only secure accesses"]
    #[inline(always)]
    pub fn apbconlysecaccessallow(&self) -> ApbconlysecaccessallowR {
        ApbconlysecaccessallowR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - when this FW controlled register is set, the APBC_ONLY_SEC_ACCESS_ALLOWED register cant be modified (until the next POR)."]
    #[inline(always)]
    pub fn apbconlysecaccessallowlock(&self) -> ApbconlysecaccessallowlockR {
        ApbconlysecaccessallowlockR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - when this FW controlled register is set, the APB-C slave accepts only privileged accesses"]
    #[inline(always)]
    pub fn apbconlyprivaccessallow(&self) -> ApbconlyprivaccessallowR {
        ApbconlyprivaccessallowR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - when this FW controlled register is set, the APBC_ONLY_PRIV_ACCESS_ALLOWED register cant be modified (until the next POR)"]
    #[inline(always)]
    pub fn apbconlyprivaccessallowlock(&self) -> ApbconlyprivaccessallowlockR {
        ApbconlyprivaccessallowlockR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - when this FW controlled register is set, the APB-C slave accepts only instruction accesses"]
    #[inline(always)]
    pub fn apbconlyinstaccessallow(&self) -> ApbconlyinstaccessallowR {
        ApbconlyinstaccessallowR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - when this FW controlled register is set, the APBC_ONLY_INST_ACCESS_ALLOWED register cant be modified (until the next POR)"]
    #[inline(always)]
    pub fn apbconlyinstaccessallowlock(&self) -> ApbconlyinstaccessallowlockR {
        ApbconlyinstaccessallowlockR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - when this FW controlled register is set, the APB slave accepts only secure accesses"]
    #[inline(always)]
    #[must_use]
    pub fn onlysecaccessallow(&mut self) -> OnlysecaccessallowW<AoapbfilteringSpec> {
        OnlysecaccessallowW::new(self, 0)
    }
    #[doc = "Bit 1 - when this FW controlled register is set, the ONLY_SEC_ACCESS_ALLOWED register cant be modified (until the next POR)."]
    #[inline(always)]
    #[must_use]
    pub fn onlysecaccessallowlock(&mut self) -> OnlysecaccessallowlockW<AoapbfilteringSpec> {
        OnlysecaccessallowlockW::new(self, 1)
    }
    #[doc = "Bit 2 - when this FW controlled register is set, the APB slave accepts only privileged accesses"]
    #[inline(always)]
    #[must_use]
    pub fn onlyprivaccessallow(&mut self) -> OnlyprivaccessallowW<AoapbfilteringSpec> {
        OnlyprivaccessallowW::new(self, 2)
    }
    #[doc = "Bit 3 - when this FW controlled register is set, the APBC_ONLY_PRIV_ACCESS_ALLOWED register cant be modified (until the next POR)"]
    #[inline(always)]
    #[must_use]
    pub fn onlyprivaccessallowlock(&mut self) -> OnlyprivaccessallowlockW<AoapbfilteringSpec> {
        OnlyprivaccessallowlockW::new(self, 3)
    }
    #[doc = "Bit 4 - when this FW controlled register is set, the APB-C slave accepts only secure accesses"]
    #[inline(always)]
    #[must_use]
    pub fn apbconlysecaccessallow(&mut self) -> ApbconlysecaccessallowW<AoapbfilteringSpec> {
        ApbconlysecaccessallowW::new(self, 4)
    }
    #[doc = "Bit 5 - when this FW controlled register is set, the APBC_ONLY_SEC_ACCESS_ALLOWED register cant be modified (until the next POR)."]
    #[inline(always)]
    #[must_use]
    pub fn apbconlysecaccessallowlock(
        &mut self,
    ) -> ApbconlysecaccessallowlockW<AoapbfilteringSpec> {
        ApbconlysecaccessallowlockW::new(self, 5)
    }
    #[doc = "Bit 6 - when this FW controlled register is set, the APB-C slave accepts only privileged accesses"]
    #[inline(always)]
    #[must_use]
    pub fn apbconlyprivaccessallow(&mut self) -> ApbconlyprivaccessallowW<AoapbfilteringSpec> {
        ApbconlyprivaccessallowW::new(self, 6)
    }
    #[doc = "Bit 7 - when this FW controlled register is set, the APBC_ONLY_PRIV_ACCESS_ALLOWED register cant be modified (until the next POR)"]
    #[inline(always)]
    #[must_use]
    pub fn apbconlyprivaccessallowlock(
        &mut self,
    ) -> ApbconlyprivaccessallowlockW<AoapbfilteringSpec> {
        ApbconlyprivaccessallowlockW::new(self, 7)
    }
    #[doc = "Bit 8 - when this FW controlled register is set, the APB-C slave accepts only instruction accesses"]
    #[inline(always)]
    #[must_use]
    pub fn apbconlyinstaccessallow(&mut self) -> ApbconlyinstaccessallowW<AoapbfilteringSpec> {
        ApbconlyinstaccessallowW::new(self, 8)
    }
    #[doc = "Bit 9 - when this FW controlled register is set, the APBC_ONLY_INST_ACCESS_ALLOWED register cant be modified (until the next POR)"]
    #[inline(always)]
    #[must_use]
    pub fn apbconlyinstaccessallowlock(
        &mut self,
    ) -> ApbconlyinstaccessallowlockW<AoapbfilteringSpec> {
        ApbconlyinstaccessallowlockW::new(self, 9)
    }
}
#[doc = "This register holds the AO_APB_FILTERING data. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`aoapbfiltering::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aoapbfiltering::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AoapbfilteringSpec;
impl crate::RegisterSpec for AoapbfilteringSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aoapbfiltering::R`](R) reader structure"]
impl crate::Readable for AoapbfilteringSpec {}
#[doc = "`write(|w| ..)` method takes [`aoapbfiltering::W`](W) writer structure"]
impl crate::Writable for AoapbfilteringSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AOAPBFILTERING to value 0x55"]
impl crate::Resettable for AoapbfilteringSpec {
    const RESET_VALUE: u32 = 0x55;
}
