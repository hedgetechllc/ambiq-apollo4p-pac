#[doc = "Register `BUFFER` reader"]
pub type R = crate::R<BufferSpec>;
#[doc = "Register `BUFFER` writer"]
pub type W = crate::W<BufferSpec>;
#[doc = "Field `BUFFERDATA` reader - The Host Controller Buffer can be accessed through this 32-bit Data Port Register."]
pub type BufferdataR = crate::FieldReader<u32>;
#[doc = "Field `BUFFERDATA` writer - The Host Controller Buffer can be accessed through this 32-bit Data Port Register."]
pub type BufferdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The Host Controller Buffer can be accessed through this 32-bit Data Port Register."]
    #[inline(always)]
    pub fn bufferdata(&self) -> BufferdataR {
        BufferdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The Host Controller Buffer can be accessed through this 32-bit Data Port Register."]
    #[inline(always)]
    #[must_use]
    pub fn bufferdata(&mut self) -> BufferdataW<BufferSpec> {
        BufferdataW::new(self, 0)
    }
}
#[doc = "Buffer data port\n\nYou can [`read`](crate::Reg::read) this register and get [`buffer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buffer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufferSpec;
impl crate::RegisterSpec for BufferSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buffer::R`](R) reader structure"]
impl crate::Readable for BufferSpec {}
#[doc = "`write(|w| ..)` method takes [`buffer::W`](W) writer structure"]
impl crate::Writable for BufferSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUFFER to value 0"]
impl crate::Resettable for BufferSpec {
    const RESET_VALUE: u32 = 0;
}
