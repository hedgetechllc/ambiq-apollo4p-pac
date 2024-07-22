#[doc = "Register `KEXTCLKSEL` reader"]
pub type R = crate::R<KextclkselSpec>;
#[doc = "Register `KEXTCLKSEL` writer"]
pub type W = crate::W<KextclkselSpec>;
#[doc = "Key register value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Kextclksel {
    #[doc = "83: Key value to unlock the register."]
    Key = 83,
}
impl From<Kextclksel> for u32 {
    #[inline(always)]
    fn from(variant: Kextclksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Kextclksel {
    type Ux = u32;
}
impl crate::IsEnum for Kextclksel {}
#[doc = "Field `KEXTCLKSEL` reader - Key register value."]
pub type KextclkselR = crate::FieldReader<Kextclksel>;
impl KextclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Kextclksel> {
        match self.bits {
            83 => Some(Kextclksel::Key),
            _ => None,
        }
    }
    #[doc = "Key value to unlock the register."]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == Kextclksel::Key
    }
}
#[doc = "Field `KEXTCLKSEL` writer - Key register value."]
pub type KextclkselW<'a, REG> = crate::FieldWriter<'a, REG, 32, Kextclksel>;
impl<'a, REG> KextclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Key value to unlock the register."]
    #[inline(always)]
    pub fn key(self) -> &'a mut crate::W<REG> {
        self.variant(Kextclksel::Key)
    }
}
impl R {
    #[doc = "Bits 0:31 - Key register value."]
    #[inline(always)]
    pub fn kextclksel(&self) -> KextclkselR {
        KextclkselR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key register value."]
    #[inline(always)]
    #[must_use]
    pub fn kextclksel(&mut self) -> KextclkselW<KextclkselSpec> {
        KextclkselW::new(self, 0)
    }
}
#[doc = "Locks the state of the EXTCLKSEL register from writes. This is done to prevent errant writes to the register, as this could cause the chip to halt. Write a value of 0x53 to unlock write access to the EXTCLKSEL register. Once unlocked, the register will read back a 1 to undicate this is unlocked. Writing the register with any other value other than 0x53 will enable the lock.\n\nYou can [`read`](crate::Reg::read) this register and get [`kextclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kextclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KextclkselSpec;
impl crate::RegisterSpec for KextclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`kextclksel::R`](R) reader structure"]
impl crate::Readable for KextclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`kextclksel::W`](W) writer structure"]
impl crate::Writable for KextclkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEXTCLKSEL to value 0"]
impl crate::Resettable for KextclkselSpec {
    const RESET_VALUE: u32 = 0;
}
