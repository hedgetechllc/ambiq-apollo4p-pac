#[doc = "Register `LAYER2ADDR` reader"]
pub type R = crate::R<Layer2addrSpec>;
#[doc = "Register `LAYER2ADDR` writer"]
pub type W = crate::W<Layer2addrSpec>;
#[doc = "Field `LAYER2STARTADDRFBUF` reader - Specify the start address of framebuffer for each layer 2."]
pub type Layer2startaddrfbufR = crate::FieldReader<u32>;
#[doc = "Field `LAYER2STARTADDRFBUF` writer - Specify the start address of framebuffer for each layer 2."]
pub type Layer2startaddrfbufW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specify the start address of framebuffer for each layer 2."]
    #[inline(always)]
    pub fn layer2startaddrfbuf(&self) -> Layer2startaddrfbufR {
        Layer2startaddrfbufR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specify the start address of framebuffer for each layer 2."]
    #[inline(always)]
    #[must_use]
    pub fn layer2startaddrfbuf(&mut self) -> Layer2startaddrfbufW<Layer2addrSpec> {
        Layer2startaddrfbufW::new(self, 0)
    }
}
#[doc = "The start address of the framebuffer to be accessed by layer 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer2addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer2addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Layer2addrSpec;
impl crate::RegisterSpec for Layer2addrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`layer2addr::R`](R) reader structure"]
impl crate::Readable for Layer2addrSpec {}
#[doc = "`write(|w| ..)` method takes [`layer2addr::W`](W) writer structure"]
impl crate::Writable for Layer2addrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAYER2ADDR to value 0"]
impl crate::Resettable for Layer2addrSpec {
    const RESET_VALUE: u32 = 0;
}
