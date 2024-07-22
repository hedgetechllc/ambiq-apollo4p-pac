#[doc = "Register `AESCONTROL` reader"]
pub type R = crate::R<AescontrolSpec>;
#[doc = "Register `AESCONTROL` writer"]
pub type W = crate::W<AescontrolSpec>;
#[doc = "This field determines whether the AES performs Decrypt_Encrypt operations, in non-tunneling operations:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Deckey0 {
    #[doc = "0: Encrypt"]
    Encrypt = 0,
    #[doc = "1: Decrypt"]
    Decrypt = 1,
}
impl From<Deckey0> for bool {
    #[inline(always)]
    fn from(variant: Deckey0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DECKEY0` reader - This field determines whether the AES performs Decrypt_Encrypt operations, in non-tunneling operations:"]
pub type Deckey0R = crate::BitReader<Deckey0>;
impl Deckey0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Deckey0 {
        match self.bits {
            false => Deckey0::Encrypt,
            true => Deckey0::Decrypt,
        }
    }
    #[doc = "Encrypt"]
    #[inline(always)]
    pub fn is_encrypt(&self) -> bool {
        *self == Deckey0::Encrypt
    }
    #[doc = "Decrypt"]
    #[inline(always)]
    pub fn is_decrypt(&self) -> bool {
        *self == Deckey0::Decrypt
    }
}
#[doc = "Field `DECKEY0` writer - This field determines whether the AES performs Decrypt_Encrypt operations, in non-tunneling operations:"]
pub type Deckey0W<'a, REG> = crate::BitWriter<'a, REG, Deckey0>;
impl<'a, REG> Deckey0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Encrypt"]
    #[inline(always)]
    pub fn encrypt(self) -> &'a mut crate::W<REG> {
        self.variant(Deckey0::Encrypt)
    }
    #[doc = "Decrypt"]
    #[inline(always)]
    pub fn decrypt(self) -> &'a mut crate::W<REG> {
        self.variant(Deckey0::Decrypt)
    }
}
#[doc = "Field `MODE0ISCBCCTS` reader - If MODE_KEY0 is set to 3b001 (CBC), and this field is set - the mode is CBC-CTS. In addition, If MODE_KEY0 is set to 3b010 (CTR), and this field is set - the mode is GCTR."]
pub type Mode0iscbcctsR = crate::BitReader;
#[doc = "Field `MODE0ISCBCCTS` writer - If MODE_KEY0 is set to 3b001 (CBC), and this field is set - the mode is CBC-CTS. In addition, If MODE_KEY0 is set to 3b010 (CTR), and this field is set - the mode is GCTR."]
pub type Mode0iscbcctsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "This field determines the AES mode in non tunneling operations, and the AES mode of the first stage in tunneling operations:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Modekey0 {
    #[doc = "0: ECB modekey0"]
    Ecb = 0,
    #[doc = "1: CBC modekey0"]
    Cbc = 1,
    #[doc = "2: CTR modekey0"]
    Ctr = 2,
    #[doc = "3: CBCMAC modekey0"]
    Cbcmac = 3,
    #[doc = "4: XEX XTS modekey0"]
    XexXts = 4,
    #[doc = "5: XCBC MAC modekey0"]
    XcbcMac = 5,
    #[doc = "6: OFB modekey0"]
    Ofb = 6,
    #[doc = "7: CMAC modekey0"]
    Cmac = 7,
}
impl From<Modekey0> for u8 {
    #[inline(always)]
    fn from(variant: Modekey0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modekey0 {
    type Ux = u8;
}
impl crate::IsEnum for Modekey0 {}
#[doc = "Field `MODEKEY0` reader - This field determines the AES mode in non tunneling operations, and the AES mode of the first stage in tunneling operations:"]
pub type Modekey0R = crate::FieldReader<Modekey0>;
impl Modekey0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Modekey0 {
        match self.bits {
            0 => Modekey0::Ecb,
            1 => Modekey0::Cbc,
            2 => Modekey0::Ctr,
            3 => Modekey0::Cbcmac,
            4 => Modekey0::XexXts,
            5 => Modekey0::XcbcMac,
            6 => Modekey0::Ofb,
            7 => Modekey0::Cmac,
            _ => unreachable!(),
        }
    }
    #[doc = "ECB modekey0"]
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == Modekey0::Ecb
    }
    #[doc = "CBC modekey0"]
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == Modekey0::Cbc
    }
    #[doc = "CTR modekey0"]
    #[inline(always)]
    pub fn is_ctr(&self) -> bool {
        *self == Modekey0::Ctr
    }
    #[doc = "CBCMAC modekey0"]
    #[inline(always)]
    pub fn is_cbcmac(&self) -> bool {
        *self == Modekey0::Cbcmac
    }
    #[doc = "XEX XTS modekey0"]
    #[inline(always)]
    pub fn is_xex_xts(&self) -> bool {
        *self == Modekey0::XexXts
    }
    #[doc = "XCBC MAC modekey0"]
    #[inline(always)]
    pub fn is_xcbc_mac(&self) -> bool {
        *self == Modekey0::XcbcMac
    }
    #[doc = "OFB modekey0"]
    #[inline(always)]
    pub fn is_ofb(&self) -> bool {
        *self == Modekey0::Ofb
    }
    #[doc = "CMAC modekey0"]
    #[inline(always)]
    pub fn is_cmac(&self) -> bool {
        *self == Modekey0::Cmac
    }
}
#[doc = "Field `MODEKEY0` writer - This field determines the AES mode in non tunneling operations, and the AES mode of the first stage in tunneling operations:"]
pub type Modekey0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Modekey0, crate::Safe>;
impl<'a, REG> Modekey0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ECB modekey0"]
    #[inline(always)]
    pub fn ecb(self) -> &'a mut crate::W<REG> {
        self.variant(Modekey0::Ecb)
    }
    #[doc = "CBC modekey0"]
    #[inline(always)]
    pub fn cbc(self) -> &'a mut crate::W<REG> {
        self.variant(Modekey0::Cbc)
    }
    #[doc = "CTR modekey0"]
    #[inline(always)]
    pub fn ctr(self) -> &'a mut crate::W<REG> {
        self.variant(Modekey0::Ctr)
    }
    #[doc = "CBCMAC modekey0"]
    #[inline(always)]
    pub fn cbcmac(self) -> &'a mut crate::W<REG> {
        self.variant(Modekey0::Cbcmac)
    }
    #[doc = "XEX XTS modekey0"]
    #[inline(always)]
    pub fn xex_xts(self) -> &'a mut crate::W<REG> {
        self.variant(Modekey0::XexXts)
    }
    #[doc = "XCBC MAC modekey0"]
    #[inline(always)]
    pub fn xcbc_mac(self) -> &'a mut crate::W<REG> {
        self.variant(Modekey0::XcbcMac)
    }
    #[doc = "OFB modekey0"]
    #[inline(always)]
    pub fn ofb(self) -> &'a mut crate::W<REG> {
        self.variant(Modekey0::Ofb)
    }
    #[doc = "CMAC modekey0"]
    #[inline(always)]
    pub fn cmac(self) -> &'a mut crate::W<REG> {
        self.variant(Modekey0::Cmac)
    }
}
#[doc = "This field determines the AES mode of the second stage operation in tunneling operations:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Modekey1 {
    #[doc = "0: ECB modekey1"]
    Ecb = 0,
    #[doc = "1: CBC modekey1"]
    Cbc = 1,
    #[doc = "2: CTR modekey1"]
    Ctr = 2,
    #[doc = "3: CBC MAC modekey1"]
    CbcMac = 3,
    #[doc = "4: XEX_XTS modekey1"]
    XexXts = 4,
    #[doc = "5: XCBC MAC modekey1"]
    XcbcMac = 5,
    #[doc = "6: OFB modekey1"]
    Ofb = 6,
    #[doc = "7: CMAC modekey1"]
    Cmac = 7,
}
impl From<Modekey1> for u8 {
    #[inline(always)]
    fn from(variant: Modekey1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modekey1 {
    type Ux = u8;
}
impl crate::IsEnum for Modekey1 {}
#[doc = "Field `MODEKEY1` reader - This field determines the AES mode of the second stage operation in tunneling operations:"]
pub type Modekey1R = crate::FieldReader<Modekey1>;
impl Modekey1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Modekey1 {
        match self.bits {
            0 => Modekey1::Ecb,
            1 => Modekey1::Cbc,
            2 => Modekey1::Ctr,
            3 => Modekey1::CbcMac,
            4 => Modekey1::XexXts,
            5 => Modekey1::XcbcMac,
            6 => Modekey1::Ofb,
            7 => Modekey1::Cmac,
            _ => unreachable!(),
        }
    }
    #[doc = "ECB modekey1"]
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == Modekey1::Ecb
    }
    #[doc = "CBC modekey1"]
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == Modekey1::Cbc
    }
    #[doc = "CTR modekey1"]
    #[inline(always)]
    pub fn is_ctr(&self) -> bool {
        *self == Modekey1::Ctr
    }
    #[doc = "CBC MAC modekey1"]
    #[inline(always)]
    pub fn is_cbc_mac(&self) -> bool {
        *self == Modekey1::CbcMac
    }
    #[doc = "XEX_XTS modekey1"]
    #[inline(always)]
    pub fn is_xex_xts(&self) -> bool {
        *self == Modekey1::XexXts
    }
    #[doc = "XCBC MAC modekey1"]
    #[inline(always)]
    pub fn is_xcbc_mac(&self) -> bool {
        *self == Modekey1::XcbcMac
    }
    #[doc = "OFB modekey1"]
    #[inline(always)]
    pub fn is_ofb(&self) -> bool {
        *self == Modekey1::Ofb
    }
    #[doc = "CMAC modekey1"]
    #[inline(always)]
    pub fn is_cmac(&self) -> bool {
        *self == Modekey1::Cmac
    }
}
#[doc = "Field `MODEKEY1` writer - This field determines the AES mode of the second stage operation in tunneling operations:"]
pub type Modekey1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Modekey1, crate::Safe>;
impl<'a, REG> Modekey1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ECB modekey1"]
    #[inline(always)]
    pub fn ecb(self) -> &'a mut crate::W<REG> {
        self.variant(Modekey1::Ecb)
    }
    #[doc = "CBC modekey1"]
    #[inline(always)]
    pub fn cbc(self) -> &'a mut crate::W<REG> {
        self.variant(Modekey1::Cbc)
    }
    #[doc = "CTR modekey1"]
    #[inline(always)]
    pub fn ctr(self) -> &'a mut crate::W<REG> {
        self.variant(Modekey1::Ctr)
    }
    #[doc = "CBC MAC modekey1"]
    #[inline(always)]
    pub fn cbc_mac(self) -> &'a mut crate::W<REG> {
        self.variant(Modekey1::CbcMac)
    }
    #[doc = "XEX_XTS modekey1"]
    #[inline(always)]
    pub fn xex_xts(self) -> &'a mut crate::W<REG> {
        self.variant(Modekey1::XexXts)
    }
    #[doc = "XCBC MAC modekey1"]
    #[inline(always)]
    pub fn xcbc_mac(self) -> &'a mut crate::W<REG> {
        self.variant(Modekey1::XcbcMac)
    }
    #[doc = "OFB modekey1"]
    #[inline(always)]
    pub fn ofb(self) -> &'a mut crate::W<REG> {
        self.variant(Modekey1::Ofb)
    }
    #[doc = "CMAC modekey1"]
    #[inline(always)]
    pub fn cmac(self) -> &'a mut crate::W<REG> {
        self.variant(Modekey1::Cmac)
    }
}
#[doc = "Field `CBCISESSIV` reader - If MODE_KEY0 is set to 3b001 (CBC), and this field is set - the mode is CBC with ESSIV."]
pub type CbcisessivR = crate::BitReader;
#[doc = "Field `CBCISESSIV` writer - If MODE_KEY0 is set to 3b001 (CBC), and this field is set - the mode is CBC with ESSIV."]
pub type CbcisessivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "This field determines whether the AES performs dual-tunnel operations or standard non-tunneling operations:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aestunnelison {
    #[doc = "0: standard non-tunneling operations"]
    StdNontunnel = 0,
    #[doc = "1: tunneling operations."]
    Tunnel = 1,
}
impl From<Aestunnelison> for bool {
    #[inline(always)]
    fn from(variant: Aestunnelison) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESTUNNELISON` reader - This field determines whether the AES performs dual-tunnel operations or standard non-tunneling operations:"]
pub type AestunnelisonR = crate::BitReader<Aestunnelison>;
impl AestunnelisonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aestunnelison {
        match self.bits {
            false => Aestunnelison::StdNontunnel,
            true => Aestunnelison::Tunnel,
        }
    }
    #[doc = "standard non-tunneling operations"]
    #[inline(always)]
    pub fn is_std_nontunnel(&self) -> bool {
        *self == Aestunnelison::StdNontunnel
    }
    #[doc = "tunneling operations."]
    #[inline(always)]
    pub fn is_tunnel(&self) -> bool {
        *self == Aestunnelison::Tunnel
    }
}
#[doc = "Field `AESTUNNELISON` writer - This field determines whether the AES performs dual-tunnel operations or standard non-tunneling operations:"]
pub type AestunnelisonW<'a, REG> = crate::BitWriter<'a, REG, Aestunnelison>;
impl<'a, REG> AestunnelisonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "standard non-tunneling operations"]
    #[inline(always)]
    pub fn std_nontunnel(self) -> &'a mut crate::W<REG> {
        self.variant(Aestunnelison::StdNontunnel)
    }
    #[doc = "tunneling operations."]
    #[inline(always)]
    pub fn tunnel(self) -> &'a mut crate::W<REG> {
        self.variant(Aestunnelison::Tunnel)
    }
}
#[doc = "Field `CBCISBITLOCKER` reader - If MODE_KEY0 is set to 3b001 (CBC), and this field is set - the mode isBITLOCKER."]
pub type CbcisbitlockerR = crate::BitReader;
#[doc = "Field `CBCISBITLOCKER` writer - If MODE_KEY0 is set to 3b001 (CBC), and this field is set - the mode isBITLOCKER."]
pub type CbcisbitlockerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "This field determines the AES Key length in non tunneling operations, and the AES key length of the first stage in tunneling operations:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nkkey0 {
    #[doc = "0: 128 bits key"]
    _128BitsKey = 0,
    #[doc = "1: 192 bits key"]
    _192BitsKey = 1,
    #[doc = "2: 256 bits key"]
    _256BitsKey = 2,
    #[doc = "3: Not applicable"]
    NA = 3,
}
impl From<Nkkey0> for u8 {
    #[inline(always)]
    fn from(variant: Nkkey0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nkkey0 {
    type Ux = u8;
}
impl crate::IsEnum for Nkkey0 {}
#[doc = "Field `NKKEY0` reader - This field determines the AES Key length in non tunneling operations, and the AES key length of the first stage in tunneling operations:"]
pub type Nkkey0R = crate::FieldReader<Nkkey0>;
impl Nkkey0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nkkey0 {
        match self.bits {
            0 => Nkkey0::_128BitsKey,
            1 => Nkkey0::_192BitsKey,
            2 => Nkkey0::_256BitsKey,
            3 => Nkkey0::NA,
            _ => unreachable!(),
        }
    }
    #[doc = "128 bits key"]
    #[inline(always)]
    pub fn is_128_bits_key(&self) -> bool {
        *self == Nkkey0::_128BitsKey
    }
    #[doc = "192 bits key"]
    #[inline(always)]
    pub fn is_192_bits_key(&self) -> bool {
        *self == Nkkey0::_192BitsKey
    }
    #[doc = "256 bits key"]
    #[inline(always)]
    pub fn is_256_bits_key(&self) -> bool {
        *self == Nkkey0::_256BitsKey
    }
    #[doc = "Not applicable"]
    #[inline(always)]
    pub fn is_n_a(&self) -> bool {
        *self == Nkkey0::NA
    }
}
#[doc = "Field `NKKEY0` writer - This field determines the AES Key length in non tunneling operations, and the AES key length of the first stage in tunneling operations:"]
pub type Nkkey0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Nkkey0, crate::Safe>;
impl<'a, REG> Nkkey0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128 bits key"]
    #[inline(always)]
    pub fn _128_bits_key(self) -> &'a mut crate::W<REG> {
        self.variant(Nkkey0::_128BitsKey)
    }
    #[doc = "192 bits key"]
    #[inline(always)]
    pub fn _192_bits_key(self) -> &'a mut crate::W<REG> {
        self.variant(Nkkey0::_192BitsKey)
    }
    #[doc = "256 bits key"]
    #[inline(always)]
    pub fn _256_bits_key(self) -> &'a mut crate::W<REG> {
        self.variant(Nkkey0::_256BitsKey)
    }
    #[doc = "Not applicable"]
    #[inline(always)]
    pub fn n_a(self) -> &'a mut crate::W<REG> {
        self.variant(Nkkey0::NA)
    }
}
#[doc = "This field determines the AES key length of the second stage operation in tunneling operations:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nkkey1 {
    #[doc = "0: 128 bits key"]
    _128BitsKey = 0,
    #[doc = "1: 192 bits key"]
    _192BitsKey = 1,
    #[doc = "2: 256 bits key"]
    _256BitsKey = 2,
    #[doc = "3: Not applicable"]
    NA = 3,
}
impl From<Nkkey1> for u8 {
    #[inline(always)]
    fn from(variant: Nkkey1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nkkey1 {
    type Ux = u8;
}
impl crate::IsEnum for Nkkey1 {}
#[doc = "Field `NKKEY1` reader - This field determines the AES key length of the second stage operation in tunneling operations:"]
pub type Nkkey1R = crate::FieldReader<Nkkey1>;
impl Nkkey1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nkkey1 {
        match self.bits {
            0 => Nkkey1::_128BitsKey,
            1 => Nkkey1::_192BitsKey,
            2 => Nkkey1::_256BitsKey,
            3 => Nkkey1::NA,
            _ => unreachable!(),
        }
    }
    #[doc = "128 bits key"]
    #[inline(always)]
    pub fn is_128_bits_key(&self) -> bool {
        *self == Nkkey1::_128BitsKey
    }
    #[doc = "192 bits key"]
    #[inline(always)]
    pub fn is_192_bits_key(&self) -> bool {
        *self == Nkkey1::_192BitsKey
    }
    #[doc = "256 bits key"]
    #[inline(always)]
    pub fn is_256_bits_key(&self) -> bool {
        *self == Nkkey1::_256BitsKey
    }
    #[doc = "Not applicable"]
    #[inline(always)]
    pub fn is_n_a(&self) -> bool {
        *self == Nkkey1::NA
    }
}
#[doc = "Field `NKKEY1` writer - This field determines the AES key length of the second stage operation in tunneling operations:"]
pub type Nkkey1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Nkkey1, crate::Safe>;
impl<'a, REG> Nkkey1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128 bits key"]
    #[inline(always)]
    pub fn _128_bits_key(self) -> &'a mut crate::W<REG> {
        self.variant(Nkkey1::_128BitsKey)
    }
    #[doc = "192 bits key"]
    #[inline(always)]
    pub fn _192_bits_key(self) -> &'a mut crate::W<REG> {
        self.variant(Nkkey1::_192BitsKey)
    }
    #[doc = "256 bits key"]
    #[inline(always)]
    pub fn _256_bits_key(self) -> &'a mut crate::W<REG> {
        self.variant(Nkkey1::_256BitsKey)
    }
    #[doc = "Not applicable"]
    #[inline(always)]
    pub fn n_a(self) -> &'a mut crate::W<REG> {
        self.variant(Nkkey1::NA)
    }
}
#[doc = "This field determines whether the second tunnel stage performs encrypt or decrypt operation :\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aestunnel1decrypt {
    #[doc = "0: the second tunnel stage performs encrypt operations."]
    Tunnel2E = 0,
    #[doc = "1: the second tunnel stage performs decrypt operations."]
    Tunnel2D = 1,
}
impl From<Aestunnel1decrypt> for bool {
    #[inline(always)]
    fn from(variant: Aestunnel1decrypt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESTUNNEL1DECRYPT` reader - This field determines whether the second tunnel stage performs encrypt or decrypt operation :"]
pub type Aestunnel1decryptR = crate::BitReader<Aestunnel1decrypt>;
impl Aestunnel1decryptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aestunnel1decrypt {
        match self.bits {
            false => Aestunnel1decrypt::Tunnel2E,
            true => Aestunnel1decrypt::Tunnel2D,
        }
    }
    #[doc = "the second tunnel stage performs encrypt operations."]
    #[inline(always)]
    pub fn is_tunnel_2_e(&self) -> bool {
        *self == Aestunnel1decrypt::Tunnel2E
    }
    #[doc = "the second tunnel stage performs decrypt operations."]
    #[inline(always)]
    pub fn is_tunnel_2_d(&self) -> bool {
        *self == Aestunnel1decrypt::Tunnel2D
    }
}
#[doc = "Field `AESTUNNEL1DECRYPT` writer - This field determines whether the second tunnel stage performs encrypt or decrypt operation :"]
pub type Aestunnel1decryptW<'a, REG> = crate::BitWriter<'a, REG, Aestunnel1decrypt>;
impl<'a, REG> Aestunnel1decryptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the second tunnel stage performs encrypt operations."]
    #[inline(always)]
    pub fn tunnel_2_e(self) -> &'a mut crate::W<REG> {
        self.variant(Aestunnel1decrypt::Tunnel2E)
    }
    #[doc = "the second tunnel stage performs decrypt operations."]
    #[inline(always)]
    pub fn tunnel_2_d(self) -> &'a mut crate::W<REG> {
        self.variant(Aestunnel1decrypt::Tunnel2D)
    }
}
#[doc = "This field determines, for tunneling operations, the data that is fed to the second tunneling stage:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aestunb1usespaddeddatain {
    #[doc = "0: the output of the first block (standard tunneling operation)."]
    Block1 = 0,
    #[doc = "1: data_in after padding rather than the output of the first block."]
    AfterPad = 1,
}
impl From<Aestunb1usespaddeddatain> for bool {
    #[inline(always)]
    fn from(variant: Aestunb1usespaddeddatain) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESTUNB1USESPADDEDDATAIN` reader - This field determines, for tunneling operations, the data that is fed to the second tunneling stage:"]
pub type Aestunb1usespaddeddatainR = crate::BitReader<Aestunb1usespaddeddatain>;
impl Aestunb1usespaddeddatainR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aestunb1usespaddeddatain {
        match self.bits {
            false => Aestunb1usespaddeddatain::Block1,
            true => Aestunb1usespaddeddatain::AfterPad,
        }
    }
    #[doc = "the output of the first block (standard tunneling operation)."]
    #[inline(always)]
    pub fn is_block_1(&self) -> bool {
        *self == Aestunb1usespaddeddatain::Block1
    }
    #[doc = "data_in after padding rather than the output of the first block."]
    #[inline(always)]
    pub fn is_after_pad(&self) -> bool {
        *self == Aestunb1usespaddeddatain::AfterPad
    }
}
#[doc = "Field `AESTUNB1USESPADDEDDATAIN` writer - This field determines, for tunneling operations, the data that is fed to the second tunneling stage:"]
pub type Aestunb1usespaddeddatainW<'a, REG> = crate::BitWriter<'a, REG, Aestunb1usespaddeddatain>;
impl<'a, REG> Aestunb1usespaddeddatainW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the output of the first block (standard tunneling operation)."]
    #[inline(always)]
    pub fn block_1(self) -> &'a mut crate::W<REG> {
        self.variant(Aestunb1usespaddeddatain::Block1)
    }
    #[doc = "data_in after padding rather than the output of the first block."]
    #[inline(always)]
    pub fn after_pad(self) -> &'a mut crate::W<REG> {
        self.variant(Aestunb1usespaddeddatain::AfterPad)
    }
}
#[doc = "This field determines whether the first tunnel stage performs encrypt or decrypt operation :\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aestunnel0encrypt {
    #[doc = "0: the first tunnel stage performs decrypt operations."]
    Tunnel1D = 0,
    #[doc = "1: the first tunnel stage performs encrypt operations."]
    Tunnel1E = 1,
}
impl From<Aestunnel0encrypt> for bool {
    #[inline(always)]
    fn from(variant: Aestunnel0encrypt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESTUNNEL0ENCRYPT` reader - This field determines whether the first tunnel stage performs encrypt or decrypt operation :"]
pub type Aestunnel0encryptR = crate::BitReader<Aestunnel0encrypt>;
impl Aestunnel0encryptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aestunnel0encrypt {
        match self.bits {
            false => Aestunnel0encrypt::Tunnel1D,
            true => Aestunnel0encrypt::Tunnel1E,
        }
    }
    #[doc = "the first tunnel stage performs decrypt operations."]
    #[inline(always)]
    pub fn is_tunnel_1_d(&self) -> bool {
        *self == Aestunnel0encrypt::Tunnel1D
    }
    #[doc = "the first tunnel stage performs encrypt operations."]
    #[inline(always)]
    pub fn is_tunnel_1_e(&self) -> bool {
        *self == Aestunnel0encrypt::Tunnel1E
    }
}
#[doc = "Field `AESTUNNEL0ENCRYPT` writer - This field determines whether the first tunnel stage performs encrypt or decrypt operation :"]
pub type Aestunnel0encryptW<'a, REG> = crate::BitWriter<'a, REG, Aestunnel0encrypt>;
impl<'a, REG> Aestunnel0encryptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the first tunnel stage performs decrypt operations."]
    #[inline(always)]
    pub fn tunnel_1_d(self) -> &'a mut crate::W<REG> {
        self.variant(Aestunnel0encrypt::Tunnel1D)
    }
    #[doc = "the first tunnel stage performs encrypt operations."]
    #[inline(always)]
    pub fn tunnel_1_e(self) -> &'a mut crate::W<REG> {
        self.variant(Aestunnel0encrypt::Tunnel1E)
    }
}
#[doc = "This fields determines whether the AES output is the result of the first or second tunneling stage:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aesoutputmidtunneldata {
    #[doc = "0: The AES engine outputs the result of the second tunnel stage (standard tunneling)."]
    EngineRslt2 = 0,
    #[doc = "1: The AES engine outputs the result of the first tunnel stage."]
    EngineRslt1 = 1,
}
impl From<Aesoutputmidtunneldata> for bool {
    #[inline(always)]
    fn from(variant: Aesoutputmidtunneldata) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESOUTPUTMIDTUNNELDATA` reader - This fields determines whether the AES output is the result of the first or second tunneling stage:"]
pub type AesoutputmidtunneldataR = crate::BitReader<Aesoutputmidtunneldata>;
impl AesoutputmidtunneldataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesoutputmidtunneldata {
        match self.bits {
            false => Aesoutputmidtunneldata::EngineRslt2,
            true => Aesoutputmidtunneldata::EngineRslt1,
        }
    }
    #[doc = "The AES engine outputs the result of the second tunnel stage (standard tunneling)."]
    #[inline(always)]
    pub fn is_engine_rslt_2(&self) -> bool {
        *self == Aesoutputmidtunneldata::EngineRslt2
    }
    #[doc = "The AES engine outputs the result of the first tunnel stage."]
    #[inline(always)]
    pub fn is_engine_rslt_1(&self) -> bool {
        *self == Aesoutputmidtunneldata::EngineRslt1
    }
}
#[doc = "Field `AESOUTPUTMIDTUNNELDATA` writer - This fields determines whether the AES output is the result of the first or second tunneling stage:"]
pub type AesoutputmidtunneldataW<'a, REG> = crate::BitWriter<'a, REG, Aesoutputmidtunneldata>;
impl<'a, REG> AesoutputmidtunneldataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The AES engine outputs the result of the second tunnel stage (standard tunneling)."]
    #[inline(always)]
    pub fn engine_rslt_2(self) -> &'a mut crate::W<REG> {
        self.variant(Aesoutputmidtunneldata::EngineRslt2)
    }
    #[doc = "The AES engine outputs the result of the first tunnel stage."]
    #[inline(always)]
    pub fn engine_rslt_1(self) -> &'a mut crate::W<REG> {
        self.variant(Aesoutputmidtunneldata::EngineRslt1)
    }
}
#[doc = "This field determines whether the input data to the second tunnel stage is padded with zeroes (according to the remaining_bytes value) or not:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aestunnelb1paden {
    #[doc = "0: The data input to the second tunnel block is not padded with zeros."]
    DataNoPad = 0,
    #[doc = "1: The data input to the second tunnel block is padded with zeros."]
    DataIsPad = 1,
}
impl From<Aestunnelb1paden> for bool {
    #[inline(always)]
    fn from(variant: Aestunnelb1paden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESTUNNELB1PADEN` reader - This field determines whether the input data to the second tunnel stage is padded with zeroes (according to the remaining_bytes value) or not:"]
pub type Aestunnelb1padenR = crate::BitReader<Aestunnelb1paden>;
impl Aestunnelb1padenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aestunnelb1paden {
        match self.bits {
            false => Aestunnelb1paden::DataNoPad,
            true => Aestunnelb1paden::DataIsPad,
        }
    }
    #[doc = "The data input to the second tunnel block is not padded with zeros."]
    #[inline(always)]
    pub fn is_data_no_pad(&self) -> bool {
        *self == Aestunnelb1paden::DataNoPad
    }
    #[doc = "The data input to the second tunnel block is padded with zeros."]
    #[inline(always)]
    pub fn is_data_is_pad(&self) -> bool {
        *self == Aestunnelb1paden::DataIsPad
    }
}
#[doc = "Field `AESTUNNELB1PADEN` writer - This field determines whether the input data to the second tunnel stage is padded with zeroes (according to the remaining_bytes value) or not:"]
pub type Aestunnelb1padenW<'a, REG> = crate::BitWriter<'a, REG, Aestunnelb1paden>;
impl<'a, REG> Aestunnelb1padenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The data input to the second tunnel block is not padded with zeros."]
    #[inline(always)]
    pub fn data_no_pad(self) -> &'a mut crate::W<REG> {
        self.variant(Aestunnelb1paden::DataNoPad)
    }
    #[doc = "The data input to the second tunnel block is padded with zeros."]
    #[inline(always)]
    pub fn data_is_pad(self) -> &'a mut crate::W<REG> {
        self.variant(Aestunnelb1paden::DataIsPad)
    }
}
#[doc = "This field determines for AES-TO-HASH-AND-DOUT tunneling operations, whether the AES outputs to the HASH the result of the first or the second tunneling stage:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aesoutmidtuntohash {
    #[doc = "0: The AES engine writes to the hash the result of the second tunnel stage."]
    Hash2 = 0,
    #[doc = "1: The AES engine writes to the hash the result of the first tunnel stage."]
    Hash1 = 1,
}
impl From<Aesoutmidtuntohash> for bool {
    #[inline(always)]
    fn from(variant: Aesoutmidtuntohash) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESOUTMIDTUNTOHASH` reader - This field determines for AES-TO-HASH-AND-DOUT tunneling operations, whether the AES outputs to the HASH the result of the first or the second tunneling stage:"]
pub type AesoutmidtuntohashR = crate::BitReader<Aesoutmidtuntohash>;
impl AesoutmidtuntohashR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesoutmidtuntohash {
        match self.bits {
            false => Aesoutmidtuntohash::Hash2,
            true => Aesoutmidtuntohash::Hash1,
        }
    }
    #[doc = "The AES engine writes to the hash the result of the second tunnel stage."]
    #[inline(always)]
    pub fn is_hash_2(&self) -> bool {
        *self == Aesoutmidtuntohash::Hash2
    }
    #[doc = "The AES engine writes to the hash the result of the first tunnel stage."]
    #[inline(always)]
    pub fn is_hash_1(&self) -> bool {
        *self == Aesoutmidtuntohash::Hash1
    }
}
#[doc = "Field `AESOUTMIDTUNTOHASH` writer - This field determines for AES-TO-HASH-AND-DOUT tunneling operations, whether the AES outputs to the HASH the result of the first or the second tunneling stage:"]
pub type AesoutmidtuntohashW<'a, REG> = crate::BitWriter<'a, REG, Aesoutmidtuntohash>;
impl<'a, REG> AesoutmidtuntohashW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The AES engine writes to the hash the result of the second tunnel stage."]
    #[inline(always)]
    pub fn hash_2(self) -> &'a mut crate::W<REG> {
        self.variant(Aesoutmidtuntohash::Hash2)
    }
    #[doc = "The AES engine writes to the hash the result of the first tunnel stage."]
    #[inline(always)]
    pub fn hash_1(self) -> &'a mut crate::W<REG> {
        self.variant(Aesoutmidtuntohash::Hash1)
    }
}
#[doc = "This field determines the value that is written to AES_KEY0, when AES_SK is kicked:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aesxorcryptokey {
    #[doc = "0: The value that is written to AES_KEY0 is the value of the HW cryptokey, as is."]
    Cryptokey = 0,
    #[doc = "1: The value that is written to AES_KEY0 is the value of the HW cryptokey xored with the current value of AES_KEY0."]
    CryptokeyXor = 1,
}
impl From<Aesxorcryptokey> for bool {
    #[inline(always)]
    fn from(variant: Aesxorcryptokey) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESXORCRYPTOKEY` reader - This field determines the value that is written to AES_KEY0, when AES_SK is kicked:"]
pub type AesxorcryptokeyR = crate::BitReader<Aesxorcryptokey>;
impl AesxorcryptokeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesxorcryptokey {
        match self.bits {
            false => Aesxorcryptokey::Cryptokey,
            true => Aesxorcryptokey::CryptokeyXor,
        }
    }
    #[doc = "The value that is written to AES_KEY0 is the value of the HW cryptokey, as is."]
    #[inline(always)]
    pub fn is_cryptokey(&self) -> bool {
        *self == Aesxorcryptokey::Cryptokey
    }
    #[doc = "The value that is written to AES_KEY0 is the value of the HW cryptokey xored with the current value of AES_KEY0."]
    #[inline(always)]
    pub fn is_cryptokey_xor(&self) -> bool {
        *self == Aesxorcryptokey::CryptokeyXor
    }
}
#[doc = "Field `AESXORCRYPTOKEY` writer - This field determines the value that is written to AES_KEY0, when AES_SK is kicked:"]
pub type AesxorcryptokeyW<'a, REG> = crate::BitWriter<'a, REG, Aesxorcryptokey>;
impl<'a, REG> AesxorcryptokeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The value that is written to AES_KEY0 is the value of the HW cryptokey, as is."]
    #[inline(always)]
    pub fn cryptokey(self) -> &'a mut crate::W<REG> {
        self.variant(Aesxorcryptokey::Cryptokey)
    }
    #[doc = "The value that is written to AES_KEY0 is the value of the HW cryptokey xored with the current value of AES_KEY0."]
    #[inline(always)]
    pub fn cryptokey_xor(self) -> &'a mut crate::W<REG> {
        self.variant(Aesxorcryptokey::CryptokeyXor)
    }
}
#[doc = "Field `DIRECTACCESS` reader - Using direct access and not the din-dout interface"]
pub type DirectaccessR = crate::BitReader;
#[doc = "Field `DIRECTACCESS` writer - Using direct access and not the din-dout interface"]
pub type DirectaccessW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This field determines whether the AES performs Decrypt_Encrypt operations, in non-tunneling operations:"]
    #[inline(always)]
    pub fn deckey0(&self) -> Deckey0R {
        Deckey0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If MODE_KEY0 is set to 3b001 (CBC), and this field is set - the mode is CBC-CTS. In addition, If MODE_KEY0 is set to 3b010 (CTR), and this field is set - the mode is GCTR."]
    #[inline(always)]
    pub fn mode0iscbccts(&self) -> Mode0iscbcctsR {
        Mode0iscbcctsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - This field determines the AES mode in non tunneling operations, and the AES mode of the first stage in tunneling operations:"]
    #[inline(always)]
    pub fn modekey0(&self) -> Modekey0R {
        Modekey0R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - This field determines the AES mode of the second stage operation in tunneling operations:"]
    #[inline(always)]
    pub fn modekey1(&self) -> Modekey1R {
        Modekey1R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - If MODE_KEY0 is set to 3b001 (CBC), and this field is set - the mode is CBC with ESSIV."]
    #[inline(always)]
    pub fn cbcisessiv(&self) -> CbcisessivR {
        CbcisessivR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - This field determines whether the AES performs dual-tunnel operations or standard non-tunneling operations:"]
    #[inline(always)]
    pub fn aestunnelison(&self) -> AestunnelisonR {
        AestunnelisonR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - If MODE_KEY0 is set to 3b001 (CBC), and this field is set - the mode isBITLOCKER."]
    #[inline(always)]
    pub fn cbcisbitlocker(&self) -> CbcisbitlockerR {
        CbcisbitlockerR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - This field determines the AES Key length in non tunneling operations, and the AES key length of the first stage in tunneling operations:"]
    #[inline(always)]
    pub fn nkkey0(&self) -> Nkkey0R {
        Nkkey0R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - This field determines the AES key length of the second stage operation in tunneling operations:"]
    #[inline(always)]
    pub fn nkkey1(&self) -> Nkkey1R {
        Nkkey1R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 22 - This field determines whether the second tunnel stage performs encrypt or decrypt operation :"]
    #[inline(always)]
    pub fn aestunnel1decrypt(&self) -> Aestunnel1decryptR {
        Aestunnel1decryptR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - This field determines, for tunneling operations, the data that is fed to the second tunneling stage:"]
    #[inline(always)]
    pub fn aestunb1usespaddeddatain(&self) -> Aestunb1usespaddeddatainR {
        Aestunb1usespaddeddatainR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - This field determines whether the first tunnel stage performs encrypt or decrypt operation :"]
    #[inline(always)]
    pub fn aestunnel0encrypt(&self) -> Aestunnel0encryptR {
        Aestunnel0encryptR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - This fields determines whether the AES output is the result of the first or second tunneling stage:"]
    #[inline(always)]
    pub fn aesoutputmidtunneldata(&self) -> AesoutputmidtunneldataR {
        AesoutputmidtunneldataR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - This field determines whether the input data to the second tunnel stage is padded with zeroes (according to the remaining_bytes value) or not:"]
    #[inline(always)]
    pub fn aestunnelb1paden(&self) -> Aestunnelb1padenR {
        Aestunnelb1padenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - This field determines for AES-TO-HASH-AND-DOUT tunneling operations, whether the AES outputs to the HASH the result of the first or the second tunneling stage:"]
    #[inline(always)]
    pub fn aesoutmidtuntohash(&self) -> AesoutmidtuntohashR {
        AesoutmidtuntohashR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This field determines the value that is written to AES_KEY0, when AES_SK is kicked:"]
    #[inline(always)]
    pub fn aesxorcryptokey(&self) -> AesxorcryptokeyR {
        AesxorcryptokeyR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Using direct access and not the din-dout interface"]
    #[inline(always)]
    pub fn directaccess(&self) -> DirectaccessR {
        DirectaccessR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This field determines whether the AES performs Decrypt_Encrypt operations, in non-tunneling operations:"]
    #[inline(always)]
    #[must_use]
    pub fn deckey0(&mut self) -> Deckey0W<AescontrolSpec> {
        Deckey0W::new(self, 0)
    }
    #[doc = "Bit 1 - If MODE_KEY0 is set to 3b001 (CBC), and this field is set - the mode is CBC-CTS. In addition, If MODE_KEY0 is set to 3b010 (CTR), and this field is set - the mode is GCTR."]
    #[inline(always)]
    #[must_use]
    pub fn mode0iscbccts(&mut self) -> Mode0iscbcctsW<AescontrolSpec> {
        Mode0iscbcctsW::new(self, 1)
    }
    #[doc = "Bits 2:4 - This field determines the AES mode in non tunneling operations, and the AES mode of the first stage in tunneling operations:"]
    #[inline(always)]
    #[must_use]
    pub fn modekey0(&mut self) -> Modekey0W<AescontrolSpec> {
        Modekey0W::new(self, 2)
    }
    #[doc = "Bits 5:7 - This field determines the AES mode of the second stage operation in tunneling operations:"]
    #[inline(always)]
    #[must_use]
    pub fn modekey1(&mut self) -> Modekey1W<AescontrolSpec> {
        Modekey1W::new(self, 5)
    }
    #[doc = "Bit 8 - If MODE_KEY0 is set to 3b001 (CBC), and this field is set - the mode is CBC with ESSIV."]
    #[inline(always)]
    #[must_use]
    pub fn cbcisessiv(&mut self) -> CbcisessivW<AescontrolSpec> {
        CbcisessivW::new(self, 8)
    }
    #[doc = "Bit 10 - This field determines whether the AES performs dual-tunnel operations or standard non-tunneling operations:"]
    #[inline(always)]
    #[must_use]
    pub fn aestunnelison(&mut self) -> AestunnelisonW<AescontrolSpec> {
        AestunnelisonW::new(self, 10)
    }
    #[doc = "Bit 11 - If MODE_KEY0 is set to 3b001 (CBC), and this field is set - the mode isBITLOCKER."]
    #[inline(always)]
    #[must_use]
    pub fn cbcisbitlocker(&mut self) -> CbcisbitlockerW<AescontrolSpec> {
        CbcisbitlockerW::new(self, 11)
    }
    #[doc = "Bits 12:13 - This field determines the AES Key length in non tunneling operations, and the AES key length of the first stage in tunneling operations:"]
    #[inline(always)]
    #[must_use]
    pub fn nkkey0(&mut self) -> Nkkey0W<AescontrolSpec> {
        Nkkey0W::new(self, 12)
    }
    #[doc = "Bits 14:15 - This field determines the AES key length of the second stage operation in tunneling operations:"]
    #[inline(always)]
    #[must_use]
    pub fn nkkey1(&mut self) -> Nkkey1W<AescontrolSpec> {
        Nkkey1W::new(self, 14)
    }
    #[doc = "Bit 22 - This field determines whether the second tunnel stage performs encrypt or decrypt operation :"]
    #[inline(always)]
    #[must_use]
    pub fn aestunnel1decrypt(&mut self) -> Aestunnel1decryptW<AescontrolSpec> {
        Aestunnel1decryptW::new(self, 22)
    }
    #[doc = "Bit 23 - This field determines, for tunneling operations, the data that is fed to the second tunneling stage:"]
    #[inline(always)]
    #[must_use]
    pub fn aestunb1usespaddeddatain(&mut self) -> Aestunb1usespaddeddatainW<AescontrolSpec> {
        Aestunb1usespaddeddatainW::new(self, 23)
    }
    #[doc = "Bit 24 - This field determines whether the first tunnel stage performs encrypt or decrypt operation :"]
    #[inline(always)]
    #[must_use]
    pub fn aestunnel0encrypt(&mut self) -> Aestunnel0encryptW<AescontrolSpec> {
        Aestunnel0encryptW::new(self, 24)
    }
    #[doc = "Bit 25 - This fields determines whether the AES output is the result of the first or second tunneling stage:"]
    #[inline(always)]
    #[must_use]
    pub fn aesoutputmidtunneldata(&mut self) -> AesoutputmidtunneldataW<AescontrolSpec> {
        AesoutputmidtunneldataW::new(self, 25)
    }
    #[doc = "Bit 26 - This field determines whether the input data to the second tunnel stage is padded with zeroes (according to the remaining_bytes value) or not:"]
    #[inline(always)]
    #[must_use]
    pub fn aestunnelb1paden(&mut self) -> Aestunnelb1padenW<AescontrolSpec> {
        Aestunnelb1padenW::new(self, 26)
    }
    #[doc = "Bit 28 - This field determines for AES-TO-HASH-AND-DOUT tunneling operations, whether the AES outputs to the HASH the result of the first or the second tunneling stage:"]
    #[inline(always)]
    #[must_use]
    pub fn aesoutmidtuntohash(&mut self) -> AesoutmidtuntohashW<AescontrolSpec> {
        AesoutmidtuntohashW::new(self, 28)
    }
    #[doc = "Bit 29 - This field determines the value that is written to AES_KEY0, when AES_SK is kicked:"]
    #[inline(always)]
    #[must_use]
    pub fn aesxorcryptokey(&mut self) -> AesxorcryptokeyW<AescontrolSpec> {
        AesxorcryptokeyW::new(self, 29)
    }
    #[doc = "Bit 31 - Using direct access and not the din-dout interface"]
    #[inline(always)]
    #[must_use]
    pub fn directaccess(&mut self) -> DirectaccessW<AescontrolSpec> {
        DirectaccessW::new(self, 31)
    }
}
#[doc = "This register holds the configuration of the AES engine. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`aescontrol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aescontrol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AescontrolSpec;
impl crate::RegisterSpec for AescontrolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aescontrol::R`](R) reader structure"]
impl crate::Readable for AescontrolSpec {}
#[doc = "`write(|w| ..)` method takes [`aescontrol::W`](W) writer structure"]
impl crate::Writable for AescontrolSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESCONTROL to value 0"]
impl crate::Resettable for AescontrolSpec {
    const RESET_VALUE: u32 = 0;
}
