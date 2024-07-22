#[doc = "Register `AESHWFLAGS` reader"]
pub type R = crate::R<AeshwflagsSpec>;
#[doc = "Register `AESHWFLAGS` writer"]
pub type W = crate::W<AeshwflagsSpec>;
#[doc = "Field `SUPPORT256192KEY` reader - the SUPPORT_256_192_KEY flag"]
pub type Support256192keyR = crate::BitReader;
#[doc = "Field `SUPPORT256192KEY` writer - the SUPPORT_256_192_KEY flag"]
pub type Support256192keyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESLARGERKEK` reader - the AES_LARGE_RKEK flag"]
pub type AeslargerkekR = crate::BitReader;
#[doc = "Field `AESLARGERKEK` writer - the AES_LARGE_RKEK flag"]
pub type AeslargerkekW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPACNTRMSREXIST` reader - the DPA_CNTRMSR_EXIST flag"]
pub type DpacntrmsrexistR = crate::BitReader;
#[doc = "Field `DPACNTRMSREXIST` writer - the DPA_CNTRMSR_EXIST flag"]
pub type DpacntrmsrexistW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTREXIST` reader - the CTR_EXIST flag"]
pub type CtrexistR = crate::BitReader;
#[doc = "Field `CTREXIST` writer - the CTR_EXIST flag"]
pub type CtrexistW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONLYENCRYPT` reader - the ONLY_ENCRYPT flag"]
pub type OnlyencryptR = crate::BitReader;
#[doc = "Field `ONLYENCRYPT` writer - the ONLY_ENCRYPT flag"]
pub type OnlyencryptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USESBOXTABLE` reader - the USE_SBOX_TABLE flag"]
pub type UsesboxtableR = crate::BitReader;
#[doc = "Field `USESBOXTABLE` writer - the USE_SBOX_TABLE flag"]
pub type UsesboxtableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USE5SBOXES` reader - the USE_5_SBOXES flag"]
pub type Use5sboxesR = crate::BitReader;
#[doc = "Field `USE5SBOXES` writer - the USE_5_SBOXES flag"]
pub type Use5sboxesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESSUPPORTPREVIV` reader - the AES_SUPPORT_PREV_IV flag"]
pub type AessupportprevivR = crate::BitReader;
#[doc = "Field `AESSUPPORTPREVIV` writer - the AES_SUPPORT_PREV_IV flag"]
pub type AessupportprevivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aestunnelexists` reader - the aes_tunnel_exists flag"]
pub type AestunnelexistsR = crate::BitReader;
#[doc = "Field `aestunnelexists` writer - the aes_tunnel_exists flag"]
pub type AestunnelexistsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECONDREGSSETEXIST` reader - the SECOND_REGS_SET_EXIST flag"]
pub type SecondregssetexistR = crate::BitReader;
#[doc = "Field `SECONDREGSSETEXIST` writer - the SECOND_REGS_SET_EXIST flag"]
pub type SecondregssetexistW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFACNTRMSREXIST` reader - the DFA_CNTRMSR_EXIST flag"]
pub type DfacntrmsrexistR = crate::BitReader;
#[doc = "Field `DFACNTRMSREXIST` writer - the DFA_CNTRMSR_EXIST flag"]
pub type DfacntrmsrexistW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - the SUPPORT_256_192_KEY flag"]
    #[inline(always)]
    pub fn support256192key(&self) -> Support256192keyR {
        Support256192keyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the AES_LARGE_RKEK flag"]
    #[inline(always)]
    pub fn aeslargerkek(&self) -> AeslargerkekR {
        AeslargerkekR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the DPA_CNTRMSR_EXIST flag"]
    #[inline(always)]
    pub fn dpacntrmsrexist(&self) -> DpacntrmsrexistR {
        DpacntrmsrexistR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the CTR_EXIST flag"]
    #[inline(always)]
    pub fn ctrexist(&self) -> CtrexistR {
        CtrexistR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - the ONLY_ENCRYPT flag"]
    #[inline(always)]
    pub fn onlyencrypt(&self) -> OnlyencryptR {
        OnlyencryptR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - the USE_SBOX_TABLE flag"]
    #[inline(always)]
    pub fn usesboxtable(&self) -> UsesboxtableR {
        UsesboxtableR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - the USE_5_SBOXES flag"]
    #[inline(always)]
    pub fn use5sboxes(&self) -> Use5sboxesR {
        Use5sboxesR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - the AES_SUPPORT_PREV_IV flag"]
    #[inline(always)]
    pub fn aessupportpreviv(&self) -> AessupportprevivR {
        AessupportprevivR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - the aes_tunnel_exists flag"]
    #[inline(always)]
    pub fn aestunnelexists(&self) -> AestunnelexistsR {
        AestunnelexistsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - the SECOND_REGS_SET_EXIST flag"]
    #[inline(always)]
    pub fn secondregssetexist(&self) -> SecondregssetexistR {
        SecondregssetexistR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - the DFA_CNTRMSR_EXIST flag"]
    #[inline(always)]
    pub fn dfacntrmsrexist(&self) -> DfacntrmsrexistR {
        DfacntrmsrexistR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - the SUPPORT_256_192_KEY flag"]
    #[inline(always)]
    #[must_use]
    pub fn support256192key(&mut self) -> Support256192keyW<AeshwflagsSpec> {
        Support256192keyW::new(self, 0)
    }
    #[doc = "Bit 1 - the AES_LARGE_RKEK flag"]
    #[inline(always)]
    #[must_use]
    pub fn aeslargerkek(&mut self) -> AeslargerkekW<AeshwflagsSpec> {
        AeslargerkekW::new(self, 1)
    }
    #[doc = "Bit 2 - the DPA_CNTRMSR_EXIST flag"]
    #[inline(always)]
    #[must_use]
    pub fn dpacntrmsrexist(&mut self) -> DpacntrmsrexistW<AeshwflagsSpec> {
        DpacntrmsrexistW::new(self, 2)
    }
    #[doc = "Bit 3 - the CTR_EXIST flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctrexist(&mut self) -> CtrexistW<AeshwflagsSpec> {
        CtrexistW::new(self, 3)
    }
    #[doc = "Bit 4 - the ONLY_ENCRYPT flag"]
    #[inline(always)]
    #[must_use]
    pub fn onlyencrypt(&mut self) -> OnlyencryptW<AeshwflagsSpec> {
        OnlyencryptW::new(self, 4)
    }
    #[doc = "Bit 5 - the USE_SBOX_TABLE flag"]
    #[inline(always)]
    #[must_use]
    pub fn usesboxtable(&mut self) -> UsesboxtableW<AeshwflagsSpec> {
        UsesboxtableW::new(self, 5)
    }
    #[doc = "Bit 8 - the USE_5_SBOXES flag"]
    #[inline(always)]
    #[must_use]
    pub fn use5sboxes(&mut self) -> Use5sboxesW<AeshwflagsSpec> {
        Use5sboxesW::new(self, 8)
    }
    #[doc = "Bit 9 - the AES_SUPPORT_PREV_IV flag"]
    #[inline(always)]
    #[must_use]
    pub fn aessupportpreviv(&mut self) -> AessupportprevivW<AeshwflagsSpec> {
        AessupportprevivW::new(self, 9)
    }
    #[doc = "Bit 10 - the aes_tunnel_exists flag"]
    #[inline(always)]
    #[must_use]
    pub fn aestunnelexists(&mut self) -> AestunnelexistsW<AeshwflagsSpec> {
        AestunnelexistsW::new(self, 10)
    }
    #[doc = "Bit 11 - the SECOND_REGS_SET_EXIST flag"]
    #[inline(always)]
    #[must_use]
    pub fn secondregssetexist(&mut self) -> SecondregssetexistW<AeshwflagsSpec> {
        SecondregssetexistW::new(self, 11)
    }
    #[doc = "Bit 12 - the DFA_CNTRMSR_EXIST flag"]
    #[inline(always)]
    #[must_use]
    pub fn dfacntrmsrexist(&mut self) -> DfacntrmsrexistW<AeshwflagsSpec> {
        DfacntrmsrexistW::new(self, 12)
    }
}
#[doc = "This register holds the pre-synthesis HW flag configuration of the AES engine\n\nYou can [`read`](crate::Reg::read) this register and get [`aeshwflags::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeshwflags::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AeshwflagsSpec;
impl crate::RegisterSpec for AeshwflagsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aeshwflags::R`](R) reader structure"]
impl crate::Readable for AeshwflagsSpec {}
#[doc = "`write(|w| ..)` method takes [`aeshwflags::W`](W) writer structure"]
impl crate::Writable for AeshwflagsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESHWFLAGS to value 0x1d0b"]
impl crate::Resettable for AeshwflagsSpec {
    const RESET_VALUE: u32 = 0x1d0b;
}
