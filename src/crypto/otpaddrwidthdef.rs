#[doc = "Register `OTPADDRWIDTHDEF` reader"]
pub type R = crate::R<OtpaddrwidthdefSpec>;
#[doc = "Register `OTPADDRWIDTHDEF` writer"]
pub type W = crate::W<OtpaddrwidthdefSpec>;
#[doc = "Field `OTPADDRWIDTHDEF` reader - Holds the OTP_ADDR_WIDTH_DEF value."]
pub type OtpaddrwidthdefR = crate::FieldReader;
#[doc = "Field `OTPADDRWIDTHDEF` writer - Holds the OTP_ADDR_WIDTH_DEF value."]
pub type OtpaddrwidthdefW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Holds the OTP_ADDR_WIDTH_DEF value."]
    #[inline(always)]
    pub fn otpaddrwidthdef(&self) -> OtpaddrwidthdefR {
        OtpaddrwidthdefR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Holds the OTP_ADDR_WIDTH_DEF value."]
    #[inline(always)]
    #[must_use]
    pub fn otpaddrwidthdef(&mut self) -> OtpaddrwidthdefW<OtpaddrwidthdefSpec> {
        OtpaddrwidthdefW::new(self, 0)
    }
}
#[doc = "OTP_ADDR_WIDTH parameter, that will define the integrated OTP address width (address in words). The supported sizes are 6 (for 2 Kbits),7,8,9,11 (for 64 Kbits). The default value in the provided RTL will be 6.Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`otpaddrwidthdef::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpaddrwidthdef::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpaddrwidthdefSpec;
impl crate::RegisterSpec for OtpaddrwidthdefSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otpaddrwidthdef::R`](R) reader structure"]
impl crate::Readable for OtpaddrwidthdefSpec {}
#[doc = "`write(|w| ..)` method takes [`otpaddrwidthdef::W`](W) writer structure"]
impl crate::Writable for OtpaddrwidthdefSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTPADDRWIDTHDEF to value 0x0b"]
impl crate::Resettable for OtpaddrwidthdefSpec {
    const RESET_VALUE: u32 = 0x0b;
}
