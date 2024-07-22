#[doc = "Register `KEY3` reader"]
pub type R = crate::R<Key3Spec>;
#[doc = "Register `KEY3` writer"]
pub type W = crate::W<Key3Spec>;
#[doc = "Field `KEY3` reader - Bits \\[127:96\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
pub type Key3R = crate::FieldReader<u32>;
#[doc = "Field `KEY3` writer - Bits \\[127:96\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
pub type Key3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bits \\[127:96\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
    #[inline(always)]
    pub fn key3(&self) -> Key3R {
        Key3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits \\[127:96\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
    #[inline(always)]
    #[must_use]
    pub fn key3(&mut self) -> Key3W<Key3Spec> {
        Key3W::new(self, 0)
    }
}
#[doc = "Key3\n\nYou can [`read`](crate::Reg::read) this register and get [`key3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key3Spec;
impl crate::RegisterSpec for Key3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key3::R`](R) reader structure"]
impl crate::Readable for Key3Spec {}
#[doc = "`write(|w| ..)` method takes [`key3::W`](W) writer structure"]
impl crate::Writable for Key3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY3 to value 0"]
impl crate::Resettable for Key3Spec {
    const RESET_VALUE: u32 = 0;
}
