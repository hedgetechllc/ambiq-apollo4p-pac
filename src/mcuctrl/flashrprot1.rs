#[doc = "Register `FLASHRPROT1` reader"]
pub type R = crate::R<Flashrprot1Spec>;
#[doc = "Register `FLASHRPROT1` writer"]
pub type W = crate::W<Flashrprot1Spec>;
#[doc = "Field `FR1BITS` reader - Copy (read) protect flash 0x00080000 - 0x000FFFFF. Each bit provides read protection for 16KB chunks of flash. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
pub type Fr1bitsR = crate::FieldReader<u32>;
#[doc = "Field `FR1BITS` writer - Copy (read) protect flash 0x00080000 - 0x000FFFFF. Each bit provides read protection for 16KB chunks of flash. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
pub type Fr1bitsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Copy (read) protect flash 0x00080000 - 0x000FFFFF. Each bit provides read protection for 16KB chunks of flash. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
    #[inline(always)]
    pub fn fr1bits(&self) -> Fr1bitsR {
        Fr1bitsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Copy (read) protect flash 0x00080000 - 0x000FFFFF. Each bit provides read protection for 16KB chunks of flash. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
    #[inline(always)]
    #[must_use]
    pub fn fr1bits(&mut self) -> Fr1bitsW<Flashrprot1Spec> {
        Fr1bitsW::new(self, 0)
    }
}
#[doc = "These bits read-protect flash in 16KB chunks.\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrprot1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrprot1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flashrprot1Spec;
impl crate::RegisterSpec for Flashrprot1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashrprot1::R`](R) reader structure"]
impl crate::Readable for Flashrprot1Spec {}
#[doc = "`write(|w| ..)` method takes [`flashrprot1::W`](W) writer structure"]
impl crate::Writable for Flashrprot1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASHRPROT1 to value 0"]
impl crate::Resettable for Flashrprot1Spec {
    const RESET_VALUE: u32 = 0;
}
