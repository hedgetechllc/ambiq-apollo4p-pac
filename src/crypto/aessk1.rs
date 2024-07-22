#[doc = "Register `AESSK1` reader"]
pub type R = crate::R<Aessk1Spec>;
#[doc = "Register `AESSK1` writer"]
pub type W = crate::W<Aessk1Spec>;
#[doc = "Field `AESSK1` reader - writing to this address causes sampling of the HW key to the AES_KEY1 register"]
pub type Aessk1R = crate::BitReader;
#[doc = "Field `AESSK1` writer - writing to this address causes sampling of the HW key to the AES_KEY1 register"]
pub type Aessk1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - writing to this address causes sampling of the HW key to the AES_KEY1 register"]
    #[inline(always)]
    pub fn aessk1(&self) -> Aessk1R {
        Aessk1R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - writing to this address causes sampling of the HW key to the AES_KEY1 register"]
    #[inline(always)]
    #[must_use]
    pub fn aessk1(&mut self) -> Aessk1W<Aessk1Spec> {
        Aessk1W::new(self, 0)
    }
}
#[doc = "writing to this address causes sampling of the HW key to the AES_KEY1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`aessk1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aessk1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aessk1Spec;
impl crate::RegisterSpec for Aessk1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aessk1::R`](R) reader structure"]
impl crate::Readable for Aessk1Spec {}
#[doc = "`write(|w| ..)` method takes [`aessk1::W`](W) writer structure"]
impl crate::Writable for Aessk1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESSK1 to value 0"]
impl crate::Resettable for Aessk1Spec {
    const RESET_VALUE: u32 = 0;
}
