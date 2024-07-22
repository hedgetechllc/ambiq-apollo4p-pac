#[doc = "Register `DINBUFFER` reader"]
pub type R = crate::R<DinbufferSpec>;
#[doc = "Register `DINBUFFER` writer"]
pub type W = crate::W<DinbufferSpec>;
#[doc = "Field `DINBUFFERDATA` reader - This register is mapped into 8 addresses in order to enable a CPU burst."]
pub type DinbufferdataR = crate::FieldReader<u32>;
#[doc = "Field `DINBUFFERDATA` writer - This register is mapped into 8 addresses in order to enable a CPU burst."]
pub type DinbufferdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register is mapped into 8 addresses in order to enable a CPU burst."]
    #[inline(always)]
    pub fn dinbufferdata(&self) -> DinbufferdataR {
        DinbufferdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is mapped into 8 addresses in order to enable a CPU burst."]
    #[inline(always)]
    #[must_use]
    pub fn dinbufferdata(&mut self) -> DinbufferdataW<DinbufferSpec> {
        DinbufferdataW::new(self, 0)
    }
}
#[doc = "This address can be used by the CPU to write data directly to the DIN buffer to be sent to engines.\n\nYou can [`read`](crate::Reg::read) this register and get [`dinbuffer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinbuffer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DinbufferSpec;
impl crate::RegisterSpec for DinbufferSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinbuffer::R`](R) reader structure"]
impl crate::Readable for DinbufferSpec {}
#[doc = "`write(|w| ..)` method takes [`dinbuffer::W`](W) writer structure"]
impl crate::Writable for DinbufferSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DINBUFFER to value 0x8770"]
impl crate::Resettable for DinbufferSpec {
    const RESET_VALUE: u32 = 0x8770;
}
