#[doc = "Register `KEY2` reader"]
pub type R = crate::R<Key2Spec>;
#[doc = "Register `KEY2` writer"]
pub type W = crate::W<Key2Spec>;
#[doc = "Field `KEY2` reader - Bits \\[95:64\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
pub type Key2R = crate::FieldReader<u32>;
#[doc = "Field `KEY2` writer - Bits \\[95:64\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
pub type Key2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bits \\[95:64\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
    #[inline(always)]
    pub fn key2(&self) -> Key2R {
        Key2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits \\[95:64\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
    #[inline(always)]
    #[must_use]
    pub fn key2(&mut self) -> Key2W<Key2Spec> {
        Key2W::new(self, 0)
    }
}
#[doc = "Key2\n\nYou can [`read`](crate::Reg::read) this register and get [`key2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key2Spec;
impl crate::RegisterSpec for Key2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key2::R`](R) reader structure"]
impl crate::Readable for Key2Spec {}
#[doc = "`write(|w| ..)` method takes [`key2::W`](W) writer structure"]
impl crate::Writable for Key2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY2 to value 0"]
impl crate::Resettable for Key2Spec {
    const RESET_VALUE: u32 = 0;
}
