#[doc = "Register `LAYER0ADDR` reader"]
pub type R = crate::R<Layer0addrSpec>;
#[doc = "Register `LAYER0ADDR` writer"]
pub type W = crate::W<Layer0addrSpec>;
#[doc = "Field `LAYER0STARTADDRFBUF` reader - Specify the start address of framebuffer for each layer 0."]
pub type Layer0startaddrfbufR = crate::FieldReader<u32>;
#[doc = "Field `LAYER0STARTADDRFBUF` writer - Specify the start address of framebuffer for each layer 0."]
pub type Layer0startaddrfbufW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specify the start address of framebuffer for each layer 0."]
    #[inline(always)]
    pub fn layer0startaddrfbuf(&self) -> Layer0startaddrfbufR {
        Layer0startaddrfbufR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specify the start address of framebuffer for each layer 0."]
    #[inline(always)]
    #[must_use]
    pub fn layer0startaddrfbuf(&mut self) -> Layer0startaddrfbufW<Layer0addrSpec> {
        Layer0startaddrfbufW::new(self, 0)
    }
}
#[doc = "The start address of the framebuffer to be accessed by layer 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer0addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer0addrSpec;
impl crate::RegisterSpec for Layer0addrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer0addr::R`](R) reader structure"]
impl crate::Readable for Layer0addrSpec {}
#[doc = "`write(|w| ..)` method takes [`layer0addr::W`](W) writer structure"]
impl crate::Writable for Layer0addrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER0ADDR to value 0"]
impl crate::Resettable for Layer0addrSpec {
    const RESET_VALUE: u32 = 0;
}
