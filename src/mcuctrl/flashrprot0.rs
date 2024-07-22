#[doc = "Register `FLASHRPROT0` reader"]
pub type R = crate::R<Flashrprot0Spec>;
#[doc = "Register `FLASHRPROT0` writer"]
pub type W = crate::W<Flashrprot0Spec>;
#[doc = "Field `FR0BITS` reader - Copy (read) protect flash 0x00000000 - 0x0007FFFF. Each bit provides read protection for 16KB chunks of flash. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
pub type Fr0bitsR = crate::FieldReader<u32>;
#[doc = "Field `FR0BITS` writer - Copy (read) protect flash 0x00000000 - 0x0007FFFF. Each bit provides read protection for 16KB chunks of flash. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
pub type Fr0bitsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Copy (read) protect flash 0x00000000 - 0x0007FFFF. Each bit provides read protection for 16KB chunks of flash. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
    #[inline(always)]
    pub fn fr0bits(&self) -> Fr0bitsR {
        Fr0bitsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Copy (read) protect flash 0x00000000 - 0x0007FFFF. Each bit provides read protection for 16KB chunks of flash. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
    #[inline(always)]
    #[must_use]
    pub fn fr0bits(&mut self) -> Fr0bitsW<Flashrprot0Spec> {
        Fr0bitsW::new(self, 0)
    }
}
#[doc = "These bits read-protect flash in 16KB chunks.\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrprot0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrprot0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flashrprot0Spec;
impl crate::RegisterSpec for Flashrprot0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashrprot0::R`](R) reader structure"]
impl crate::Readable for Flashrprot0Spec {}
#[doc = "`write(|w| ..)` method takes [`flashrprot0::W`](W) writer structure"]
impl crate::Writable for Flashrprot0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASHRPROT0 to value 0"]
impl crate::Resettable for Flashrprot0Spec {
    const RESET_VALUE: u32 = 0;
}
