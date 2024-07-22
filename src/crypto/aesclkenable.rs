#[doc = "Register `AESCLKENABLE` reader"]
pub type R = crate::R<AesclkenableSpec>;
#[doc = "Register `AESCLKENABLE` writer"]
pub type W = crate::W<AesclkenableSpec>;
#[doc = "Enable the AES clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "1: the AES clock is enabled."]
    ClkE = 1,
    #[doc = "0: the AES clock is disabled."]
    ClkD = 0,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Enable the AES clock."]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            true => En::ClkE,
            false => En::ClkD,
        }
    }
    #[doc = "the AES clock is enabled."]
    #[inline(always)]
    pub fn is_clk_e(&self) -> bool {
        *self == En::ClkE
    }
    #[doc = "the AES clock is disabled."]
    #[inline(always)]
    pub fn is_clk_d(&self) -> bool {
        *self == En::ClkD
    }
}
#[doc = "Field `EN` writer - Enable the AES clock."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the AES clock is enabled."]
    #[inline(always)]
    pub fn clk_e(self) -> &'a mut crate::W<REG> {
        self.variant(En::ClkE)
    }
    #[doc = "the AES clock is disabled."]
    #[inline(always)]
    pub fn clk_d(self) -> &'a mut crate::W<REG> {
        self.variant(En::ClkD)
    }
}
impl R {
    #[doc = "Bit 0 - Enable the AES clock."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the AES clock."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<AesclkenableSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`aesclkenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesclkenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesclkenableSpec;
impl crate::RegisterSpec for AesclkenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesclkenable::R`](R) reader structure"]
impl crate::Readable for AesclkenableSpec {}
#[doc = "`write(|w| ..)` method takes [`aesclkenable::W`](W) writer structure"]
impl crate::Writable for AesclkenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESCLKENABLE to value 0"]
impl crate::Resettable for AesclkenableSpec {
    const RESET_VALUE: u32 = 0;
}
