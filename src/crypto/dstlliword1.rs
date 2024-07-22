#[doc = "Register `DSTLLIWORD1` reader"]
pub type R = crate::R<Dstlliword1Spec>;
#[doc = "Register `DSTLLIWORD1` writer"]
pub type W = crate::W<Dstlliword1Spec>;
#[doc = "Field `BYTESNUM` reader - Total byte number to be written by DMA in this entry"]
pub type BytesnumR = crate::FieldReader<u32>;
#[doc = "Field `BYTESNUM` writer - Total byte number to be written by DMA in this entry"]
pub type BytesnumW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
#[doc = "Field `FIRST` reader - 0x1 - Indicates the first LLI entry"]
pub type FirstR = crate::BitReader;
#[doc = "Field `FIRST` writer - 0x1 - Indicates the first LLI entry"]
pub type FirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAST` reader - 0x1 - Indicates the last LLI entry"]
pub type LastR = crate::BitReader;
#[doc = "Field `LAST` writer - 0x1 - Indicates the last LLI entry"]
pub type LastW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:29 - Total byte number to be written by DMA in this entry"]
    #[inline(always)]
    pub fn bytesnum(&self) -> BytesnumR {
        BytesnumR::new(self.bits & 0x3fff_ffff)
    }
    #[doc = "Bit 30 - 0x1 - Indicates the first LLI entry"]
    #[inline(always)]
    pub fn first(&self) -> FirstR {
        FirstR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 0x1 - Indicates the last LLI entry"]
    #[inline(always)]
    pub fn last(&self) -> LastR {
        LastR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:29 - Total byte number to be written by DMA in this entry"]
    #[inline(always)]
    #[must_use]
    pub fn bytesnum(&mut self) -> BytesnumW<Dstlliword1Spec> {
        BytesnumW::new(self, 0)
    }
    #[doc = "Bit 30 - 0x1 - Indicates the first LLI entry"]
    #[inline(always)]
    #[must_use]
    pub fn first(&mut self) -> FirstW<Dstlliword1Spec> {
        FirstW::new(self, 30)
    }
    #[doc = "Bit 31 - 0x1 - Indicates the last LLI entry"]
    #[inline(always)]
    #[must_use]
    pub fn last(&mut self) -> LastW<Dstlliword1Spec> {
        LastW::new(self, 31)
    }
}
#[doc = "This register is used in direct LLI mode - holds the number of bytes to be written to the memory (AXI). Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`dstlliword1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstlliword1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dstlliword1Spec;
impl crate::RegisterSpec for Dstlliword1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dstlliword1::R`](R) reader structure"]
impl crate::Readable for Dstlliword1Spec {}
#[doc = "`write(|w| ..)` method takes [`dstlliword1::W`](W) writer structure"]
impl crate::Writable for Dstlliword1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSTLLIWORD1 to value 0"]
impl crate::Resettable for Dstlliword1Spec {
    const RESET_VALUE: u32 = 0;
}
