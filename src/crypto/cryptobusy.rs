#[doc = "Register `CRYPTOBUSY` reader"]
pub type R = crate::R<CryptobusySpec>;
#[doc = "Register `CRYPTOBUSY` writer"]
pub type W = crate::W<CryptobusySpec>;
#[doc = "Crypto busy status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cryptobusy {
    #[doc = "0: Ready"]
    Ready = 0,
    #[doc = "1: Busy"]
    Busy = 1,
}
impl From<Cryptobusy> for bool {
    #[inline(always)]
    fn from(variant: Cryptobusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRYPTOBUSY` reader - Crypto busy status."]
pub type CryptobusyR = crate::BitReader<Cryptobusy>;
impl CryptobusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cryptobusy {
        match self.bits {
            false => Cryptobusy::Ready,
            true => Cryptobusy::Busy,
        }
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Cryptobusy::Ready
    }
    #[doc = "Busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Cryptobusy::Busy
    }
}
#[doc = "Field `CRYPTOBUSY` writer - Crypto busy status."]
pub type CryptobusyW<'a, REG> = crate::BitWriter<'a, REG, Cryptobusy>;
impl<'a, REG> CryptobusyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(Cryptobusy::Ready)
    }
    #[doc = "Busy"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(Cryptobusy::Busy)
    }
}
impl R {
    #[doc = "Bit 0 - Crypto busy status."]
    #[inline(always)]
    pub fn cryptobusy(&self) -> CryptobusyR {
        CryptobusyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Crypto busy status."]
    #[inline(always)]
    #[must_use]
    pub fn cryptobusy(&mut self) -> CryptobusyW<CryptobusySpec> {
        CryptobusyW::new(self, 0)
    }
}
#[doc = "This register is set when the cryptographic core is busy.\n\nYou can [`read`](crate::Reg::read) this register and get [`cryptobusy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cryptobusy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptobusySpec;
impl crate::RegisterSpec for CryptobusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cryptobusy::R`](R) reader structure"]
impl crate::Readable for CryptobusySpec {}
#[doc = "`write(|w| ..)` method takes [`cryptobusy::W`](W) writer structure"]
impl crate::Writable for CryptobusySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRYPTOBUSY to value 0"]
impl crate::Resettable for CryptobusySpec {
    const RESET_VALUE: u32 = 0;
}
