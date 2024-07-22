#[doc = "Register `LAYER1ADDR` reader"]
pub type R = crate::R<Layer1addrSpec>;
#[doc = "Register `LAYER1ADDR` writer"]
pub type W = crate::W<Layer1addrSpec>;
#[doc = "Field `LAYER1STARTADDRFBUF` reader - Specify the start address of framebuffer for each layer 1."]
pub type Layer1startaddrfbufR = crate::FieldReader<u32>;
#[doc = "Field `LAYER1STARTADDRFBUF` writer - Specify the start address of framebuffer for each layer 1."]
pub type Layer1startaddrfbufW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specify the start address of framebuffer for each layer 1."]
    #[inline(always)]
    pub fn layer1startaddrfbuf(&self) -> Layer1startaddrfbufR {
        Layer1startaddrfbufR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specify the start address of framebuffer for each layer 1."]
    #[inline(always)]
    #[must_use]
    pub fn layer1startaddrfbuf(&mut self) -> Layer1startaddrfbufW<Layer1addrSpec> {
        Layer1startaddrfbufW::new(self, 0)
    }
}
#[doc = "The start address of the framebuffer to be accessed by layer 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer1addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer1addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer1addrSpec;
impl crate::RegisterSpec for Layer1addrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer1addr::R`](R) reader structure"]
impl crate::Readable for Layer1addrSpec {}
#[doc = "`write(|w| ..)` method takes [`layer1addr::W`](W) writer structure"]
impl crate::Writable for Layer1addrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER1ADDR to value 0"]
impl crate::Resettable for Layer1addrSpec {
    const RESET_VALUE: u32 = 0;
}
