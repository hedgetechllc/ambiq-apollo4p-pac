#[doc = "Register `HOSTRGFENDIAN` reader"]
pub type R = crate::R<HostrgfendianSpec>;
#[doc = "Register `HOSTRGFENDIAN` writer"]
pub type W = crate::W<HostrgfendianSpec>;
#[doc = "DOUT write endianness:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutwrbg {
    #[doc = "0: little endian"]
    Le = 0,
    #[doc = "1: big endian"]
    Be = 1,
}
impl From<Doutwrbg> for bool {
    #[inline(always)]
    fn from(variant: Doutwrbg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTWRBG` reader - DOUT write endianness:"]
pub type DoutwrbgR = crate::BitReader<Doutwrbg>;
impl DoutwrbgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doutwrbg {
        match self.bits {
            false => Doutwrbg::Le,
            true => Doutwrbg::Be,
        }
    }
    #[doc = "little endian"]
    #[inline(always)]
    pub fn is_le(&self) -> bool {
        *self == Doutwrbg::Le
    }
    #[doc = "big endian"]
    #[inline(always)]
    pub fn is_be(&self) -> bool {
        *self == Doutwrbg::Be
    }
}
#[doc = "Field `DOUTWRBG` writer - DOUT write endianness:"]
pub type DoutwrbgW<'a, REG> = crate::BitWriter<'a, REG, Doutwrbg>;
impl<'a, REG> DoutwrbgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "little endian"]
    #[inline(always)]
    pub fn le(self) -> &'a mut crate::W<REG> {
        self.variant(Doutwrbg::Le)
    }
    #[doc = "big endian"]
    #[inline(always)]
    pub fn be(self) -> &'a mut crate::W<REG> {
        self.variant(Doutwrbg::Be)
    }
}
#[doc = "DIN write endianness:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dinrdbg {
    #[doc = "0: little endian"]
    Le = 0,
    #[doc = "1: big endian"]
    Be = 1,
}
impl From<Dinrdbg> for bool {
    #[inline(always)]
    fn from(variant: Dinrdbg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINRDBG` reader - DIN write endianness:"]
pub type DinrdbgR = crate::BitReader<Dinrdbg>;
impl DinrdbgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dinrdbg {
        match self.bits {
            false => Dinrdbg::Le,
            true => Dinrdbg::Be,
        }
    }
    #[doc = "little endian"]
    #[inline(always)]
    pub fn is_le(&self) -> bool {
        *self == Dinrdbg::Le
    }
    #[doc = "big endian"]
    #[inline(always)]
    pub fn is_be(&self) -> bool {
        *self == Dinrdbg::Be
    }
}
#[doc = "Field `DINRDBG` writer - DIN write endianness:"]
pub type DinrdbgW<'a, REG> = crate::BitWriter<'a, REG, Dinrdbg>;
impl<'a, REG> DinrdbgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "little endian"]
    #[inline(always)]
    pub fn le(self) -> &'a mut crate::W<REG> {
        self.variant(Dinrdbg::Le)
    }
    #[doc = "big endian"]
    #[inline(always)]
    pub fn be(self) -> &'a mut crate::W<REG> {
        self.variant(Dinrdbg::Be)
    }
}
#[doc = "DOUT write word endianness:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutwrwbg {
    #[doc = "0: little endian"]
    Le = 0,
    #[doc = "1: big endian"]
    Be = 1,
}
impl From<Doutwrwbg> for bool {
    #[inline(always)]
    fn from(variant: Doutwrwbg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTWRWBG` reader - DOUT write word endianness:"]
pub type DoutwrwbgR = crate::BitReader<Doutwrwbg>;
impl DoutwrwbgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doutwrwbg {
        match self.bits {
            false => Doutwrwbg::Le,
            true => Doutwrwbg::Be,
        }
    }
    #[doc = "little endian"]
    #[inline(always)]
    pub fn is_le(&self) -> bool {
        *self == Doutwrwbg::Le
    }
    #[doc = "big endian"]
    #[inline(always)]
    pub fn is_be(&self) -> bool {
        *self == Doutwrwbg::Be
    }
}
#[doc = "Field `DOUTWRWBG` writer - DOUT write word endianness:"]
pub type DoutwrwbgW<'a, REG> = crate::BitWriter<'a, REG, Doutwrwbg>;
impl<'a, REG> DoutwrwbgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "little endian"]
    #[inline(always)]
    pub fn le(self) -> &'a mut crate::W<REG> {
        self.variant(Doutwrwbg::Le)
    }
    #[doc = "big endian"]
    #[inline(always)]
    pub fn be(self) -> &'a mut crate::W<REG> {
        self.variant(Doutwrwbg::Be)
    }
}
#[doc = "DIN write word endianness:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dinrdwbg {
    #[doc = "0: little endian"]
    Le = 0,
    #[doc = "1: big endian"]
    Be = 1,
}
impl From<Dinrdwbg> for bool {
    #[inline(always)]
    fn from(variant: Dinrdwbg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINRDWBG` reader - DIN write word endianness:"]
pub type DinrdwbgR = crate::BitReader<Dinrdwbg>;
impl DinrdwbgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dinrdwbg {
        match self.bits {
            false => Dinrdwbg::Le,
            true => Dinrdwbg::Be,
        }
    }
    #[doc = "little endian"]
    #[inline(always)]
    pub fn is_le(&self) -> bool {
        *self == Dinrdwbg::Le
    }
    #[doc = "big endian"]
    #[inline(always)]
    pub fn is_be(&self) -> bool {
        *self == Dinrdwbg::Be
    }
}
#[doc = "Field `DINRDWBG` writer - DIN write word endianness:"]
pub type DinrdwbgW<'a, REG> = crate::BitWriter<'a, REG, Dinrdwbg>;
impl<'a, REG> DinrdwbgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "little endian"]
    #[inline(always)]
    pub fn le(self) -> &'a mut crate::W<REG> {
        self.variant(Dinrdwbg::Le)
    }
    #[doc = "big endian"]
    #[inline(always)]
    pub fn be(self) -> &'a mut crate::W<REG> {
        self.variant(Dinrdwbg::Be)
    }
}
impl R {
    #[doc = "Bit 3 - DOUT write endianness:"]
    #[inline(always)]
    pub fn doutwrbg(&self) -> DoutwrbgR {
        DoutwrbgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - DIN write endianness:"]
    #[inline(always)]
    pub fn dinrdbg(&self) -> DinrdbgR {
        DinrdbgR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - DOUT write word endianness:"]
    #[inline(always)]
    pub fn doutwrwbg(&self) -> DoutwrwbgR {
        DoutwrwbgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - DIN write word endianness:"]
    #[inline(always)]
    pub fn dinrdwbg(&self) -> DinrdwbgR {
        DinrdwbgR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - DOUT write endianness:"]
    #[inline(always)]
    #[must_use]
    pub fn doutwrbg(&mut self) -> DoutwrbgW<HostrgfendianSpec> {
        DoutwrbgW::new(self, 3)
    }
    #[doc = "Bit 7 - DIN write endianness:"]
    #[inline(always)]
    #[must_use]
    pub fn dinrdbg(&mut self) -> DinrdbgW<HostrgfendianSpec> {
        DinrdbgW::new(self, 7)
    }
    #[doc = "Bit 11 - DOUT write word endianness:"]
    #[inline(always)]
    #[must_use]
    pub fn doutwrwbg(&mut self) -> DoutwrwbgW<HostrgfendianSpec> {
        DoutwrwbgW::new(self, 11)
    }
    #[doc = "Bit 15 - DIN write word endianness:"]
    #[inline(always)]
    #[must_use]
    pub fn dinrdwbg(&mut self) -> DinrdwbgW<HostrgfendianSpec> {
        DinrdwbgW::new(self, 15)
    }
}
#[doc = "This register defines the endianness of the Host-accessible registers. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostrgfendian::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostrgfendian::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostrgfendianSpec;
impl crate::RegisterSpec for HostrgfendianSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostrgfendian::R`](R) reader structure"]
impl crate::Readable for HostrgfendianSpec {}
#[doc = "`write(|w| ..)` method takes [`hostrgfendian::W`](W) writer structure"]
impl crate::Writable for HostrgfendianSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTRGFENDIAN to value 0"]
impl crate::Resettable for HostrgfendianSpec {
    const RESET_VALUE: u32 = 0;
}
