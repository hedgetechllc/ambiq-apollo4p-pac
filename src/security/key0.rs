#[doc = "Register `KEY0` reader"]
pub type R = crate::R<Key0Spec>;
#[doc = "Register `KEY0` writer"]
pub type W = crate::W<Key0Spec>;
#[doc = "Field `KEY0` reader - Bits \\[31:0\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
pub type Key0R = crate::FieldReader<u32>;
#[doc = "Field `KEY0` writer - Bits \\[31:0\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
pub type Key0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bits \\[31:0\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
    #[inline(always)]
    pub fn key0(&self) -> Key0R {
        Key0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits \\[31:0\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
    #[inline(always)]
    #[must_use]
    pub fn key0(&mut self) -> Key0W<Key0Spec> {
        Key0W::new(self, 0)
    }
}
#[doc = "Key0\n\nYou can [`read`](crate::Reg::read) this register and get [`key0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key0Spec;
impl crate::RegisterSpec for Key0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key0::R`](R) reader structure"]
impl crate::Readable for Key0Spec {}
#[doc = "`write(|w| ..)` method takes [`key0::W`](W) writer structure"]
impl crate::Writable for Key0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY0 to value 0"]
impl crate::Resettable for Key0Spec {
    const RESET_VALUE: u32 = 0;
}
