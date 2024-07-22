#[doc = "Register `GHASHBUSY` reader"]
pub type R = crate::R<GhashbusySpec>;
#[doc = "Register `GHASHBUSY` writer"]
pub type W = crate::W<GhashbusySpec>;
#[doc = "Field `GHASHBUSY` reader - GHASH_BUSY Register. This register is set when the GHASH core is active"]
pub type GhashbusyR = crate::BitReader;
#[doc = "Field `GHASHBUSY` writer - GHASH_BUSY Register. This register is set when the GHASH core is active"]
pub type GhashbusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GHASH_BUSY Register. This register is set when the GHASH core is active"]
    #[inline(always)]
    pub fn ghashbusy(&self) -> GhashbusyR {
        GhashbusyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GHASH_BUSY Register. This register is set when the GHASH core is active"]
    #[inline(always)]
    #[must_use]
    pub fn ghashbusy(&mut self) -> GhashbusyW<GhashbusySpec> {
        GhashbusyW::new(self, 0)
    }
}
#[doc = "The GHASH module GHASH_BUSY Register. This register is set when the GHASH core is active.\n\nYou can [`read`](crate::Reg::read) this register and get [`ghashbusy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghashbusy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GhashbusySpec;
impl crate::RegisterSpec for GhashbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghashbusy::R`](R) reader structure"]
impl crate::Readable for GhashbusySpec {}
#[doc = "`write(|w| ..)` method takes [`ghashbusy::W`](W) writer structure"]
impl crate::Writable for GhashbusySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GHASHBUSY to value 0"]
impl crate::Resettable for GhashbusySpec {
    const RESET_VALUE: u32 = 0;
}
