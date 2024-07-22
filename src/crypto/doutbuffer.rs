#[doc = "Register `DOUTBUFFER` reader"]
pub type R = crate::R<DoutbufferSpec>;
#[doc = "Register `DOUTBUFFER` writer"]
pub type W = crate::W<DoutbufferSpec>;
#[doc = "Field `DATA` reader - DOUT This address can be used by the CPU to read data directly from the DOUT buffer."]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - DOUT This address can be used by the CPU to read data directly from the DOUT buffer."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DOUT This address can be used by the CPU to read data directly from the DOUT buffer."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DOUT This address can be used by the CPU to read data directly from the DOUT buffer."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<DoutbufferSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Cryptographic result - CPU can directly access it. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`doutbuffer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutbuffer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoutbufferSpec;
impl crate::RegisterSpec for DoutbufferSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutbuffer::R`](R) reader structure"]
impl crate::Readable for DoutbufferSpec {}
#[doc = "`write(|w| ..)` method takes [`doutbuffer::W`](W) writer structure"]
impl crate::Writable for DoutbufferSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTBUFFER to value 0"]
impl crate::Resettable for DoutbufferSpec {
    const RESET_VALUE: u32 = 0;
}
