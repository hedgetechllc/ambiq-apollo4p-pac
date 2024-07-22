#[doc = "Register `LAYER3ADDR` reader"]
pub type R = crate::R<Layer3addrSpec>;
#[doc = "Register `LAYER3ADDR` writer"]
pub type W = crate::W<Layer3addrSpec>;
#[doc = "Field `LAYER3STARTADDRFBUF` reader - Specify the start address of framebuffer for each layer 3."]
pub type Layer3startaddrfbufR = crate::FieldReader<u32>;
#[doc = "Field `LAYER3STARTADDRFBUF` writer - Specify the start address of framebuffer for each layer 3."]
pub type Layer3startaddrfbufW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specify the start address of framebuffer for each layer 3."]
    #[inline(always)]
    pub fn layer3startaddrfbuf(&self) -> Layer3startaddrfbufR {
        Layer3startaddrfbufR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specify the start address of framebuffer for each layer 3."]
    #[inline(always)]
    #[must_use]
    pub fn layer3startaddrfbuf(&mut self) -> Layer3startaddrfbufW<Layer3addrSpec> {
        Layer3startaddrfbufW::new(self, 0)
    }
}
#[doc = "The start address of the framebuffer to be accessed by layer 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer3addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer3addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer3addrSpec;
impl crate::RegisterSpec for Layer3addrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer3addr::R`](R) reader structure"]
impl crate::Readable for Layer3addrSpec {}
#[doc = "`write(|w| ..)` method takes [`layer3addr::W`](W) writer structure"]
impl crate::Writable for Layer3addrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER3ADDR to value 0"]
impl crate::Resettable for Layer3addrSpec {
    const RESET_VALUE: u32 = 0;
}
