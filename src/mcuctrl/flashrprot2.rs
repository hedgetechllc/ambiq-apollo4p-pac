#[doc = "Register `FLASHRPROT2` reader"]
pub type R = crate::R<Flashrprot2Spec>;
#[doc = "Register `FLASHRPROT2` writer"]
pub type W = crate::W<Flashrprot2Spec>;
#[doc = "Field `FR2BITS` reader - Copy (read) protect flash 0x00100000 - 0x0017FFFF. Each bit provides read protection for 16KB chunks of flash. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
pub type Fr2bitsR = crate::FieldReader<u32>;
#[doc = "Field `FR2BITS` writer - Copy (read) protect flash 0x00100000 - 0x0017FFFF. Each bit provides read protection for 16KB chunks of flash. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
pub type Fr2bitsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Copy (read) protect flash 0x00100000 - 0x0017FFFF. Each bit provides read protection for 16KB chunks of flash. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
    #[inline(always)]
    pub fn fr2bits(&self) -> Fr2bitsR {
        Fr2bitsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Copy (read) protect flash 0x00100000 - 0x0017FFFF. Each bit provides read protection for 16KB chunks of flash. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
    #[inline(always)]
    #[must_use]
    pub fn fr2bits(&mut self) -> Fr2bitsW<Flashrprot2Spec> {
        Fr2bitsW::new(self, 0)
    }
}
#[doc = "These bits read-protect flash in 16KB chunks.\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrprot2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrprot2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flashrprot2Spec;
impl crate::RegisterSpec for Flashrprot2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashrprot2::R`](R) reader structure"]
impl crate::Readable for Flashrprot2Spec {}
#[doc = "`write(|w| ..)` method takes [`flashrprot2::W`](W) writer structure"]
impl crate::Writable for Flashrprot2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASHRPROT2 to value 0"]
impl crate::Resettable for Flashrprot2Spec {
    const RESET_VALUE: u32 = 0;
}
