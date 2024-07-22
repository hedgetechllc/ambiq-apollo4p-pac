#[doc = "Register `HASHBUSY` reader"]
pub type R = crate::R<HashbusySpec>;
#[doc = "Register `HASHBUSY` writer"]
pub type W = crate::W<HashbusySpec>;
#[doc = "Hash busy status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hashbusy {
    #[doc = "0: Ready"]
    Ready = 0,
    #[doc = "1: Busy"]
    Busy = 1,
}
impl From<Hashbusy> for bool {
    #[inline(always)]
    fn from(variant: Hashbusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASHBUSY` reader - Hash busy status."]
pub type HashbusyR = crate::BitReader<Hashbusy>;
impl HashbusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hashbusy {
        match self.bits {
            false => Hashbusy::Ready,
            true => Hashbusy::Busy,
        }
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Hashbusy::Ready
    }
    #[doc = "Busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Hashbusy::Busy
    }
}
#[doc = "Field `HASHBUSY` writer - Hash busy status."]
pub type HashbusyW<'a, REG> = crate::BitWriter<'a, REG, Hashbusy>;
impl<'a, REG> HashbusyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(Hashbusy::Ready)
    }
    #[doc = "Busy"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(Hashbusy::Busy)
    }
}
impl R {
    #[doc = "Bit 0 - Hash busy status."]
    #[inline(always)]
    pub fn hashbusy(&self) -> HashbusyR {
        HashbusyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hash busy status."]
    #[inline(always)]
    #[must_use]
    pub fn hashbusy(&mut self) -> HashbusyW<HashbusySpec> {
        HashbusyW::new(self, 0)
    }
}
#[doc = "This register is set when the Hash engine is busy.\n\nYou can [`read`](crate::Reg::read) this register and get [`hashbusy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashbusy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashbusySpec;
impl crate::RegisterSpec for HashbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashbusy::R`](R) reader structure"]
impl crate::Readable for HashbusySpec {}
#[doc = "`write(|w| ..)` method takes [`hashbusy::W`](W) writer structure"]
impl crate::Writable for HashbusySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHBUSY to value 0"]
impl crate::Resettable for HashbusySpec {
    const RESET_VALUE: u32 = 0;
}
