#[doc = "Register `FLASHWPROT3` reader"]
pub type R = crate::R<Flashwprot3Spec>;
#[doc = "Register `FLASHWPROT3` writer"]
pub type W = crate::W<Flashwprot3Spec>;
#[doc = "Field `FW3BITS` reader - Write protect flash 0x00180000 - 0x001FFFFF. Each bit provides write protection for 16KB chunks of flash data space. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
pub type Fw3bitsR = crate::FieldReader<u32>;
#[doc = "Field `FW3BITS` writer - Write protect flash 0x00180000 - 0x001FFFFF. Each bit provides write protection for 16KB chunks of flash data space. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
pub type Fw3bitsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write protect flash 0x00180000 - 0x001FFFFF. Each bit provides write protection for 16KB chunks of flash data space. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
    #[inline(always)]
    pub fn fw3bits(&self) -> Fw3bitsR {
        Fw3bitsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write protect flash 0x00180000 - 0x001FFFFF. Each bit provides write protection for 16KB chunks of flash data space. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
    #[inline(always)]
    #[must_use]
    pub fn fw3bits(&mut self) -> Fw3bitsW<Flashwprot3Spec> {
        Fw3bitsW::new(self, 0)
    }
}
#[doc = "These bits write-protect flash in 16KB chunks.\n\nYou can [`read`](crate::Reg::read) this register and get [`flashwprot3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashwprot3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flashwprot3Spec;
impl crate::RegisterSpec for Flashwprot3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashwprot3::R`](R) reader structure"]
impl crate::Readable for Flashwprot3Spec {}
#[doc = "`write(|w| ..)` method takes [`flashwprot3::W`](W) writer structure"]
impl crate::Writable for Flashwprot3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASHWPROT3 to value 0"]
impl crate::Resettable for Flashwprot3Spec {
    const RESET_VALUE: u32 = 0;
}
