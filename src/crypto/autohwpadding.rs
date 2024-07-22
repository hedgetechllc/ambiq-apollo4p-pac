#[doc = "Register `AUTOHWPADDING` reader"]
pub type R = crate::R<AutohwpaddingSpec>;
#[doc = "Register `AUTOHWPADDING` writer"]
pub type W = crate::W<AutohwpaddingSpec>;
#[doc = "Field `EN` reader - 0x1 - Enable Automatic HW padding (No need for SW intervention by writing PAD_CFG). Note: Not supported for 0 bytes ! Note: Disable this register when HASH op is done"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0x1 - Enable Automatic HW padding (No need for SW intervention by writing PAD_CFG). Note: Not supported for 0 bytes ! Note: Disable this register when HASH op is done"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0x1 - Enable Automatic HW padding (No need for SW intervention by writing PAD_CFG). Note: Not supported for 0 bytes ! Note: Disable this register when HASH op is done"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0x1 - Enable Automatic HW padding (No need for SW intervention by writing PAD_CFG). Note: Not supported for 0 bytes ! Note: Disable this register when HASH op is done"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<AutohwpaddingSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "HW padding automatically activated by engine. For the special case of ZERO bytes data vector this register should not be used! instead use HASH_PAD_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`autohwpadding::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autohwpadding::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AutohwpaddingSpec;
impl crate::RegisterSpec for AutohwpaddingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autohwpadding::R`](R) reader structure"]
impl crate::Readable for AutohwpaddingSpec {}
#[doc = "`write(|w| ..)` method takes [`autohwpadding::W`](W) writer structure"]
impl crate::Writable for AutohwpaddingSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUTOHWPADDING to value 0"]
impl crate::Resettable for AutohwpaddingSpec {
    const RESET_VALUE: u32 = 0;
}
