#[doc = "Register `DINCPUDATASIZE` reader"]
pub type R = crate::R<DincpudatasizeSpec>;
#[doc = "Register `DINCPUDATASIZE` writer"]
pub type W = crate::W<DincpudatasizeSpec>;
#[doc = "Field `CPUDINSIZE` reader - When using external DMA, the size of transmited data in bytes should be written to this register."]
pub type CpudinsizeR = crate::FieldReader<u16>;
#[doc = "Field `CPUDINSIZE` writer - When using external DMA, the size of transmited data in bytes should be written to this register."]
pub type CpudinsizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - When using external DMA, the size of transmited data in bytes should be written to this register."]
    #[inline(always)]
    pub fn cpudinsize(&self) -> CpudinsizeR {
        CpudinsizeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - When using external DMA, the size of transmited data in bytes should be written to this register."]
    #[inline(always)]
    #[must_use]
    pub fn cpudinsize(&mut self) -> CpudinsizeW<DincpudatasizeSpec> {
        CpudinsizeW::new(self, 0)
    }
}
#[doc = "This register hold the number of bytes to be transmited using external DMA. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`dincpudatasize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dincpudatasize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DincpudatasizeSpec;
impl crate::RegisterSpec for DincpudatasizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dincpudatasize::R`](R) reader structure"]
impl crate::Readable for DincpudatasizeSpec {}
#[doc = "`write(|w| ..)` method takes [`dincpudatasize::W`](W) writer structure"]
impl crate::Writable for DincpudatasizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DINCPUDATASIZE to value 0"]
impl crate::Resettable for DincpudatasizeSpec {
    const RESET_VALUE: u32 = 0;
}
