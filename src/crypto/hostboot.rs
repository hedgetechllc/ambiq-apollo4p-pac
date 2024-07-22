#[doc = "Register `HOSTBOOT` reader"]
pub type R = crate::R<HostbootSpec>;
#[doc = "Register `HOSTBOOT` writer"]
pub type W = crate::W<HostbootSpec>;
#[doc = "Field `SYNTHESISCONFIG` reader - POWER_GATING_EXISTS_LOCAL"]
pub type SynthesisconfigR = crate::BitReader;
#[doc = "Field `SYNTHESISCONFIG` writer - POWER_GATING_EXISTS_LOCAL"]
pub type SynthesisconfigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LARGERKEKLOCAL` reader - LARGE_RKEK_LOCAL"]
pub type LargerkeklocalR = crate::BitReader;
#[doc = "Field `LARGERKEKLOCAL` writer - LARGE_RKEK_LOCAL"]
pub type LargerkeklocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASHINFUSESLOCAL` reader - HASH_IN_FUSES_LOCAL"]
pub type HashinfuseslocalR = crate::BitReader;
#[doc = "Field `HASHINFUSESLOCAL` writer - HASH_IN_FUSES_LOCAL"]
pub type HashinfuseslocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTMEMSECUREDLOCAL` reader - EXT_MEM_SECURED_LOCAL"]
pub type ExtmemsecuredlocalR = crate::BitReader;
#[doc = "Field `EXTMEMSECUREDLOCAL` writer - EXT_MEM_SECURED_LOCAL"]
pub type ExtmemsecuredlocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RKEKECCEXISTSLOCALN` reader - RKEK_ECC_EXISTS_LOCAL_N"]
pub type RkekeccexistslocalnR = crate::BitReader;
#[doc = "Field `RKEKECCEXISTSLOCALN` writer - RKEK_ECC_EXISTS_LOCAL_N"]
pub type RkekeccexistslocalnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMSIZELOCAL` reader - SRAM_SIZE_LOCAL"]
pub type SramsizelocalR = crate::FieldReader;
#[doc = "Field `SRAMSIZELOCAL` writer - SRAM_SIZE_LOCAL"]
pub type SramsizelocalW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DSCRPTREXISTSLOCAL` reader - DSCRPTR_EXISTS_LOCAL"]
pub type DscrptrexistslocalR = crate::BitReader;
#[doc = "Field `DSCRPTREXISTSLOCAL` writer - DSCRPTR_EXISTS_LOCAL"]
pub type DscrptrexistslocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAUEXISTSLOCAL` reader - PAU_EXISTS_LOCAL"]
pub type PauexistslocalR = crate::BitReader;
#[doc = "Field `PAUEXISTSLOCAL` writer - PAU_EXISTS_LOCAL"]
pub type PauexistslocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGEXISTSLOCAL` reader - RNG_EXISTS_LOCAL"]
pub type RngexistslocalR = crate::BitReader;
#[doc = "Field `RNGEXISTSLOCAL` writer - RNG_EXISTS_LOCAL"]
pub type RngexistslocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKAEXISTSLOCAL` reader - PKA_EXISTS_LOCAL"]
pub type PkaexistslocalR = crate::BitReader;
#[doc = "Field `PKAEXISTSLOCAL` writer - PKA_EXISTS_LOCAL"]
pub type PkaexistslocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RC4EXISTSLOCAL` reader - RC4_EXISTS_LOCAL"]
pub type Rc4existslocalR = crate::BitReader;
#[doc = "Field `RC4EXISTSLOCAL` writer - RC4_EXISTS_LOCAL"]
pub type Rc4existslocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHA512PRSNTLOCAL` reader - SHA_512_PRSNT_LOCAL"]
pub type Sha512prsntlocalR = crate::BitReader;
#[doc = "Field `SHA512PRSNTLOCAL` writer - SHA_512_PRSNT_LOCAL"]
pub type Sha512prsntlocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHA256PRSNTLOCAL` reader - SHA_256_PRSNT_LOCAL"]
pub type Sha256prsntlocalR = crate::BitReader;
#[doc = "Field `SHA256PRSNTLOCAL` writer - SHA_256_PRSNT_LOCAL"]
pub type Sha256prsntlocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MD5PRSNTLOCAL` reader - MD5_PRSNT_LOCAL"]
pub type Md5prsntlocalR = crate::BitReader;
#[doc = "Field `MD5PRSNTLOCAL` writer - MD5_PRSNT_LOCAL"]
pub type Md5prsntlocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASHEXISTSLOCAL` reader - HASH_EXISTS_LOCAL"]
pub type HashexistslocalR = crate::BitReader;
#[doc = "Field `HASHEXISTSLOCAL` writer - HASH_EXISTS_LOCAL"]
pub type HashexistslocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2EXISTSLOCAL` reader - C2_EXISTS_LOCAL"]
pub type C2existslocalR = crate::BitReader;
#[doc = "Field `C2EXISTSLOCAL` writer - C2_EXISTS_LOCAL"]
pub type C2existslocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESEXISTSLOCAL` reader - DES_EXISTS_LOCAL"]
pub type DesexistslocalR = crate::BitReader;
#[doc = "Field `DESEXISTSLOCAL` writer - DES_EXISTS_LOCAL"]
pub type DesexistslocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESXCBCMACEXISTSLOCAL` reader - AES_XCBC_MAC_EXISTS_LOCAL"]
pub type AesxcbcmacexistslocalR = crate::BitReader;
#[doc = "Field `AESXCBCMACEXISTSLOCAL` writer - AES_XCBC_MAC_EXISTS_LOCAL"]
pub type AesxcbcmacexistslocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESCMACEXISTSLOCAL` reader - AES_CMAC_EXISTS_LOCAL"]
pub type AescmacexistslocalR = crate::BitReader;
#[doc = "Field `AESCMACEXISTSLOCAL` writer - AES_CMAC_EXISTS_LOCAL"]
pub type AescmacexistslocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESCCMEXISTSLOCAL` reader - AES_CCM_EXISTS_LOCAL"]
pub type AesccmexistslocalR = crate::BitReader;
#[doc = "Field `AESCCMEXISTSLOCAL` writer - AES_CCM_EXISTS_LOCAL"]
pub type AesccmexistslocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESXEXHWTCALCLOCAL` reader - AES_XEX_HW_T_CALC_LOCAL"]
pub type AesxexhwtcalclocalR = crate::BitReader;
#[doc = "Field `AESXEXHWTCALCLOCAL` writer - AES_XEX_HW_T_CALC_LOCAL"]
pub type AesxexhwtcalclocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESXEXEXISTSLOCAL` reader - AES_XEX_EXISTS_LOCAL"]
pub type AesxexexistslocalR = crate::BitReader;
#[doc = "Field `AESXEXEXISTSLOCAL` writer - AES_XEX_EXISTS_LOCAL"]
pub type AesxexexistslocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTREXISTSLOCAL` reader - CTR_EXISTS_LOCAL"]
pub type CtrexistslocalR = crate::BitReader;
#[doc = "Field `CTREXISTSLOCAL` writer - CTR_EXISTS_LOCAL"]
pub type CtrexistslocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESDINBYTERESOLUTIONLOCAL` reader - AES_DIN_BYTE_RESOLUTION_LOCAL"]
pub type AesdinbyteresolutionlocalR = crate::BitReader;
#[doc = "Field `AESDINBYTERESOLUTIONLOCAL` writer - AES_DIN_BYTE_RESOLUTION_LOCAL"]
pub type AesdinbyteresolutionlocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUNNELINGENBLOCAL` reader - TUNNELING_ENB_LOCAL"]
pub type TunnelingenblocalR = crate::BitReader;
#[doc = "Field `TUNNELINGENBLOCAL` writer - TUNNELING_ENB_LOCAL"]
pub type TunnelingenblocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUPPORT256192KEYLOCAL` reader - SUPPORT_256_192_KEY_LOCAL"]
pub type Support256192keylocalR = crate::BitReader;
#[doc = "Field `SUPPORT256192KEYLOCAL` writer - SUPPORT_256_192_KEY_LOCAL"]
pub type Support256192keylocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONLYENCRYPTLOCAL` reader - ONLY_ENCRYPT_LOCAL"]
pub type OnlyencryptlocalR = crate::BitReader;
#[doc = "Field `ONLYENCRYPTLOCAL` writer - ONLY_ENCRYPT_LOCAL"]
pub type OnlyencryptlocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESEXISTSLOCAL` reader - AES_EXISTS_LOCAL"]
pub type AesexistslocalR = crate::BitReader;
#[doc = "Field `AESEXISTSLOCAL` writer - AES_EXISTS_LOCAL"]
pub type AesexistslocalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - POWER_GATING_EXISTS_LOCAL"]
    #[inline(always)]
    pub fn synthesisconfig(&self) -> SynthesisconfigR {
        SynthesisconfigR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LARGE_RKEK_LOCAL"]
    #[inline(always)]
    pub fn largerkeklocal(&self) -> LargerkeklocalR {
        LargerkeklocalR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HASH_IN_FUSES_LOCAL"]
    #[inline(always)]
    pub fn hashinfuseslocal(&self) -> HashinfuseslocalR {
        HashinfuseslocalR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EXT_MEM_SECURED_LOCAL"]
    #[inline(always)]
    pub fn extmemsecuredlocal(&self) -> ExtmemsecuredlocalR {
        ExtmemsecuredlocalR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - RKEK_ECC_EXISTS_LOCAL_N"]
    #[inline(always)]
    pub fn rkekeccexistslocaln(&self) -> RkekeccexistslocalnR {
        RkekeccexistslocalnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - SRAM_SIZE_LOCAL"]
    #[inline(always)]
    pub fn sramsizelocal(&self) -> SramsizelocalR {
        SramsizelocalR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9 - DSCRPTR_EXISTS_LOCAL"]
    #[inline(always)]
    pub fn dscrptrexistslocal(&self) -> DscrptrexistslocalR {
        DscrptrexistslocalR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PAU_EXISTS_LOCAL"]
    #[inline(always)]
    pub fn pauexistslocal(&self) -> PauexistslocalR {
        PauexistslocalR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RNG_EXISTS_LOCAL"]
    #[inline(always)]
    pub fn rngexistslocal(&self) -> RngexistslocalR {
        RngexistslocalR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PKA_EXISTS_LOCAL"]
    #[inline(always)]
    pub fn pkaexistslocal(&self) -> PkaexistslocalR {
        PkaexistslocalR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RC4_EXISTS_LOCAL"]
    #[inline(always)]
    pub fn rc4existslocal(&self) -> Rc4existslocalR {
        Rc4existslocalR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SHA_512_PRSNT_LOCAL"]
    #[inline(always)]
    pub fn sha512prsntlocal(&self) -> Sha512prsntlocalR {
        Sha512prsntlocalR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SHA_256_PRSNT_LOCAL"]
    #[inline(always)]
    pub fn sha256prsntlocal(&self) -> Sha256prsntlocalR {
        Sha256prsntlocalR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MD5_PRSNT_LOCAL"]
    #[inline(always)]
    pub fn md5prsntlocal(&self) -> Md5prsntlocalR {
        Md5prsntlocalR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HASH_EXISTS_LOCAL"]
    #[inline(always)]
    pub fn hashexistslocal(&self) -> HashexistslocalR {
        HashexistslocalR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - C2_EXISTS_LOCAL"]
    #[inline(always)]
    pub fn c2existslocal(&self) -> C2existslocalR {
        C2existslocalR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DES_EXISTS_LOCAL"]
    #[inline(always)]
    pub fn desexistslocal(&self) -> DesexistslocalR {
        DesexistslocalR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - AES_XCBC_MAC_EXISTS_LOCAL"]
    #[inline(always)]
    pub fn aesxcbcmacexistslocal(&self) -> AesxcbcmacexistslocalR {
        AesxcbcmacexistslocalR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - AES_CMAC_EXISTS_LOCAL"]
    #[inline(always)]
    pub fn aescmacexistslocal(&self) -> AescmacexistslocalR {
        AescmacexistslocalR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - AES_CCM_EXISTS_LOCAL"]
    #[inline(always)]
    pub fn aesccmexistslocal(&self) -> AesccmexistslocalR {
        AesccmexistslocalR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - AES_XEX_HW_T_CALC_LOCAL"]
    #[inline(always)]
    pub fn aesxexhwtcalclocal(&self) -> AesxexhwtcalclocalR {
        AesxexhwtcalclocalR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - AES_XEX_EXISTS_LOCAL"]
    #[inline(always)]
    pub fn aesxexexistslocal(&self) -> AesxexexistslocalR {
        AesxexexistslocalR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CTR_EXISTS_LOCAL"]
    #[inline(always)]
    pub fn ctrexistslocal(&self) -> CtrexistslocalR {
        CtrexistslocalR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - AES_DIN_BYTE_RESOLUTION_LOCAL"]
    #[inline(always)]
    pub fn aesdinbyteresolutionlocal(&self) -> AesdinbyteresolutionlocalR {
        AesdinbyteresolutionlocalR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TUNNELING_ENB_LOCAL"]
    #[inline(always)]
    pub fn tunnelingenblocal(&self) -> TunnelingenblocalR {
        TunnelingenblocalR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - SUPPORT_256_192_KEY_LOCAL"]
    #[inline(always)]
    pub fn support256192keylocal(&self) -> Support256192keylocalR {
        Support256192keylocalR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ONLY_ENCRYPT_LOCAL"]
    #[inline(always)]
    pub fn onlyencryptlocal(&self) -> OnlyencryptlocalR {
        OnlyencryptlocalR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - AES_EXISTS_LOCAL"]
    #[inline(always)]
    pub fn aesexistslocal(&self) -> AesexistslocalR {
        AesexistslocalR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - POWER_GATING_EXISTS_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn synthesisconfig(&mut self) -> SynthesisconfigW<HostbootSpec> {
        SynthesisconfigW::new(self, 0)
    }
    #[doc = "Bit 1 - LARGE_RKEK_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn largerkeklocal(&mut self) -> LargerkeklocalW<HostbootSpec> {
        LargerkeklocalW::new(self, 1)
    }
    #[doc = "Bit 2 - HASH_IN_FUSES_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn hashinfuseslocal(&mut self) -> HashinfuseslocalW<HostbootSpec> {
        HashinfuseslocalW::new(self, 2)
    }
    #[doc = "Bit 3 - EXT_MEM_SECURED_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn extmemsecuredlocal(&mut self) -> ExtmemsecuredlocalW<HostbootSpec> {
        ExtmemsecuredlocalW::new(self, 3)
    }
    #[doc = "Bit 5 - RKEK_ECC_EXISTS_LOCAL_N"]
    #[inline(always)]
    #[must_use]
    pub fn rkekeccexistslocaln(&mut self) -> RkekeccexistslocalnW<HostbootSpec> {
        RkekeccexistslocalnW::new(self, 5)
    }
    #[doc = "Bits 6:8 - SRAM_SIZE_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn sramsizelocal(&mut self) -> SramsizelocalW<HostbootSpec> {
        SramsizelocalW::new(self, 6)
    }
    #[doc = "Bit 9 - DSCRPTR_EXISTS_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn dscrptrexistslocal(&mut self) -> DscrptrexistslocalW<HostbootSpec> {
        DscrptrexistslocalW::new(self, 9)
    }
    #[doc = "Bit 10 - PAU_EXISTS_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn pauexistslocal(&mut self) -> PauexistslocalW<HostbootSpec> {
        PauexistslocalW::new(self, 10)
    }
    #[doc = "Bit 11 - RNG_EXISTS_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn rngexistslocal(&mut self) -> RngexistslocalW<HostbootSpec> {
        RngexistslocalW::new(self, 11)
    }
    #[doc = "Bit 12 - PKA_EXISTS_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn pkaexistslocal(&mut self) -> PkaexistslocalW<HostbootSpec> {
        PkaexistslocalW::new(self, 12)
    }
    #[doc = "Bit 13 - RC4_EXISTS_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn rc4existslocal(&mut self) -> Rc4existslocalW<HostbootSpec> {
        Rc4existslocalW::new(self, 13)
    }
    #[doc = "Bit 14 - SHA_512_PRSNT_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn sha512prsntlocal(&mut self) -> Sha512prsntlocalW<HostbootSpec> {
        Sha512prsntlocalW::new(self, 14)
    }
    #[doc = "Bit 15 - SHA_256_PRSNT_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn sha256prsntlocal(&mut self) -> Sha256prsntlocalW<HostbootSpec> {
        Sha256prsntlocalW::new(self, 15)
    }
    #[doc = "Bit 16 - MD5_PRSNT_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn md5prsntlocal(&mut self) -> Md5prsntlocalW<HostbootSpec> {
        Md5prsntlocalW::new(self, 16)
    }
    #[doc = "Bit 17 - HASH_EXISTS_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn hashexistslocal(&mut self) -> HashexistslocalW<HostbootSpec> {
        HashexistslocalW::new(self, 17)
    }
    #[doc = "Bit 18 - C2_EXISTS_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn c2existslocal(&mut self) -> C2existslocalW<HostbootSpec> {
        C2existslocalW::new(self, 18)
    }
    #[doc = "Bit 19 - DES_EXISTS_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn desexistslocal(&mut self) -> DesexistslocalW<HostbootSpec> {
        DesexistslocalW::new(self, 19)
    }
    #[doc = "Bit 20 - AES_XCBC_MAC_EXISTS_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn aesxcbcmacexistslocal(&mut self) -> AesxcbcmacexistslocalW<HostbootSpec> {
        AesxcbcmacexistslocalW::new(self, 20)
    }
    #[doc = "Bit 21 - AES_CMAC_EXISTS_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn aescmacexistslocal(&mut self) -> AescmacexistslocalW<HostbootSpec> {
        AescmacexistslocalW::new(self, 21)
    }
    #[doc = "Bit 22 - AES_CCM_EXISTS_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn aesccmexistslocal(&mut self) -> AesccmexistslocalW<HostbootSpec> {
        AesccmexistslocalW::new(self, 22)
    }
    #[doc = "Bit 23 - AES_XEX_HW_T_CALC_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn aesxexhwtcalclocal(&mut self) -> AesxexhwtcalclocalW<HostbootSpec> {
        AesxexhwtcalclocalW::new(self, 23)
    }
    #[doc = "Bit 24 - AES_XEX_EXISTS_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn aesxexexistslocal(&mut self) -> AesxexexistslocalW<HostbootSpec> {
        AesxexexistslocalW::new(self, 24)
    }
    #[doc = "Bit 25 - CTR_EXISTS_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn ctrexistslocal(&mut self) -> CtrexistslocalW<HostbootSpec> {
        CtrexistslocalW::new(self, 25)
    }
    #[doc = "Bit 26 - AES_DIN_BYTE_RESOLUTION_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn aesdinbyteresolutionlocal(&mut self) -> AesdinbyteresolutionlocalW<HostbootSpec> {
        AesdinbyteresolutionlocalW::new(self, 26)
    }
    #[doc = "Bit 27 - TUNNELING_ENB_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn tunnelingenblocal(&mut self) -> TunnelingenblocalW<HostbootSpec> {
        TunnelingenblocalW::new(self, 27)
    }
    #[doc = "Bit 28 - SUPPORT_256_192_KEY_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn support256192keylocal(&mut self) -> Support256192keylocalW<HostbootSpec> {
        Support256192keylocalW::new(self, 28)
    }
    #[doc = "Bit 29 - ONLY_ENCRYPT_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn onlyencryptlocal(&mut self) -> OnlyencryptlocalW<HostbootSpec> {
        OnlyencryptlocalW::new(self, 29)
    }
    #[doc = "Bit 30 - AES_EXISTS_LOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn aesexistslocal(&mut self) -> AesexistslocalW<HostbootSpec> {
        AesexistslocalW::new(self, 30)
    }
}
#[doc = "This register holds the values of CryptoCells pre-synthesis flags\n\nYou can [`read`](crate::Reg::read) this register and get [`hostboot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostboot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostbootSpec;
impl crate::RegisterSpec for HostbootSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostboot::R`](R) reader structure"]
impl crate::Readable for HostbootSpec {}
#[doc = "`write(|w| ..)` method takes [`hostboot::W`](W) writer structure"]
impl crate::Writable for HostbootSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTBOOT to value 0x5e62_982e"]
impl crate::Resettable for HostbootSpec {
    const RESET_VALUE: u32 = 0x5e62_982e;
}
