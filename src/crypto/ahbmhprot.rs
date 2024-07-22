#[doc = "Register `AHBMHPROT` reader"]
pub type R = crate::R<AhbmhprotSpec>;
#[doc = "Register `AHBMHPROT` writer"]
pub type W = crate::W<AhbmhprotSpec>;
#[doc = "Field `AHBPROT` reader - The ahb prot value"]
pub type AhbprotR = crate::FieldReader;
#[doc = "Field `AHBPROT` writer - The ahb prot value"]
pub type AhbprotW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - The ahb prot value"]
    #[inline(always)]
    pub fn ahbprot(&self) -> AhbprotR {
        AhbprotR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - The ahb prot value"]
    #[inline(always)]
    #[must_use]
    pub fn ahbprot(&mut self) -> AhbprotW<AhbmhprotSpec> {
        AhbprotW::new(self, 0)
    }
}
#[doc = "This register holds the ahb prot value\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbmhprot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbmhprot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbmhprotSpec;
impl crate::RegisterSpec for AhbmhprotSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbmhprot::R`](R) reader structure"]
impl crate::Readable for AhbmhprotSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbmhprot::W`](W) writer structure"]
impl crate::Writable for AhbmhprotSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBMHPROT to value 0"]
impl crate::Resettable for AhbmhprotSpec {
    const RESET_VALUE: u32 = 0;
}
