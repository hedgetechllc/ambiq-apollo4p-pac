#[doc = "Register `SLOTSTAT` reader"]
pub type R = crate::R<SlotstatSpec>;
#[doc = "Register `SLOTSTAT` writer"]
pub type W = crate::W<SlotstatSpec>;
#[doc = "Field `INTSLOT0` reader - This status bit indicates the OR of Interrupt signal and Wakeup signal for slot"]
pub type Intslot0R = crate::BitReader;
#[doc = "Field `INTSLOT0` writer - This status bit indicates the OR of Interrupt signal and Wakeup signal for slot"]
pub type Intslot0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPECVER` reader - The Host Controller Version Number is set to 0x02 (SD Host Specification Version 3.00)."]
pub type SpecverR = crate::FieldReader;
#[doc = "Field `SPECVER` writer - The Host Controller Version Number is set to 0x02 (SD Host Specification Version 3.00)."]
pub type SpecverW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VENDORVER` reader - The Vendor Version Number is set to 0x10 (1.0)"]
pub type VendorverR = crate::FieldReader;
#[doc = "Field `VENDORVER` writer - The Vendor Version Number is set to 0x10 (1.0)"]
pub type VendorverW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - This status bit indicates the OR of Interrupt signal and Wakeup signal for slot"]
    #[inline(always)]
    pub fn intslot0(&self) -> Intslot0R {
        Intslot0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:23 - The Host Controller Version Number is set to 0x02 (SD Host Specification Version 3.00)."]
    #[inline(always)]
    pub fn specver(&self) -> SpecverR {
        SpecverR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - The Vendor Version Number is set to 0x10 (1.0)"]
    #[inline(always)]
    pub fn vendorver(&self) -> VendorverR {
        VendorverR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This status bit indicates the OR of Interrupt signal and Wakeup signal for slot"]
    #[inline(always)]
    #[must_use]
    pub fn intslot0(&mut self) -> Intslot0W<SlotstatSpec> {
        Intslot0W::new(self, 0)
    }
    #[doc = "Bits 16:23 - The Host Controller Version Number is set to 0x02 (SD Host Specification Version 3.00)."]
    #[inline(always)]
    #[must_use]
    pub fn specver(&mut self) -> SpecverW<SlotstatSpec> {
        SpecverW::new(self, 16)
    }
    #[doc = "Bits 24:31 - The Vendor Version Number is set to 0x10 (1.0)"]
    #[inline(always)]
    #[must_use]
    pub fn vendorver(&mut self) -> VendorverW<SlotstatSpec> {
        VendorverW::new(self, 24)
    }
}
#[doc = "Slot interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`slotstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slotstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlotstatSpec;
impl crate::RegisterSpec for SlotstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slotstat::R`](R) reader structure"]
impl crate::Readable for SlotstatSpec {}
#[doc = "`write(|w| ..)` method takes [`slotstat::W`](W) writer structure"]
impl crate::Writable for SlotstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLOTSTAT to value 0x0a02_0000"]
impl crate::Resettable for SlotstatSpec {
    const RESET_VALUE: u32 = 0x0a02_0000;
}
