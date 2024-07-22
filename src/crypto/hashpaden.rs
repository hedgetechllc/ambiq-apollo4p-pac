#[doc = "Register `HASHPADEN` reader"]
pub type R = crate::R<HashpadenSpec>;
#[doc = "Register `HASHPADEN` writer"]
pub type W = crate::W<HashpadenSpec>;
#[doc = "Field `EN` reader - 0x1 : Enable generation of padding by HW Pad block. 0x0 : Disable generation of padding by HW Pad block."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0x1 : Enable generation of padding by HW Pad block. 0x0 : Disable generation of padding by HW Pad block."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0x1 : Enable generation of padding by HW Pad block. 0x0 : Disable generation of padding by HW Pad block."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0x1 : Enable generation of padding by HW Pad block. 0x0 : Disable generation of padding by HW Pad block."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<HashpadenSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "Enables the hash hw padding.\n\nYou can [`read`](crate::Reg::read) this register and get [`hashpaden::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashpaden::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashpadenSpec;
impl crate::RegisterSpec for HashpadenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashpaden::R`](R) reader structure"]
impl crate::Readable for HashpadenSpec {}
#[doc = "`write(|w| ..)` method takes [`hashpaden::W`](W) writer structure"]
impl crate::Writable for HashpadenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHPADEN to value 0x01"]
impl crate::Resettable for HashpadenSpec {
    const RESET_VALUE: u32 = 0x01;
}
