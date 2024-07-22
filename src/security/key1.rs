#[doc = "Register `KEY1` reader"]
pub type R = crate::R<Key1Spec>;
#[doc = "Register `KEY1` writer"]
pub type W = crate::W<Key1Spec>;
#[doc = "Field `KEY1` reader - Bits \\[63:32\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
pub type Key1R = crate::FieldReader<u32>;
#[doc = "Field `KEY1` writer - Bits \\[63:32\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
pub type Key1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bits \\[63:32\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
    #[inline(always)]
    pub fn key1(&self) -> Key1R {
        Key1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits \\[63:32\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
    #[inline(always)]
    #[must_use]
    pub fn key1(&mut self) -> Key1W<Key1Spec> {
        Key1W::new(self, 0)
    }
}
#[doc = "Key1\n\nYou can [`read`](crate::Reg::read) this register and get [`key1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key1Spec;
impl crate::RegisterSpec for Key1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key1::R`](R) reader structure"]
impl crate::Readable for Key1Spec {}
#[doc = "`write(|w| ..)` method takes [`key1::W`](W) writer structure"]
impl crate::Writable for Key1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY1 to value 0"]
impl crate::Resettable for Key1Spec {
    const RESET_VALUE: u32 = 0;
}
