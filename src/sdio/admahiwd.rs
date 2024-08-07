#[doc = "Register `ADMAHIWD` reader"]
pub type R = crate::R<AdmahiwdSpec>;
#[doc = "Register `ADMAHIWD` writer"]
pub type W = crate::W<AdmahiwdSpec>;
#[doc = "Field `HIWD` reader - This register holds byte address of executing command of the Descriptor table. 32-bit Address Descriptor uses lower 32bit of this register. At the start of ADMA, the Host Driver shall set start address of the Descriptor table. The ADMA increments this register address, which points to next line, when every fetching a Descriptor line. When the ADMA Error Interrupt is generated, this register shall hold valid Descriptor address depending on the ADMA state. The Host Driver shall program Descriptor Table on 32-bit boundary and set 32-bit boundary address to this register. ADMA2 ignores lower 2-bit of this register and assumes it to be 00b. 32-bit Address ADMA Register Value 32bit System Address"]
pub type HiwdR = crate::FieldReader<u32>;
#[doc = "Field `HIWD` writer - This register holds byte address of executing command of the Descriptor table. 32-bit Address Descriptor uses lower 32bit of this register. At the start of ADMA, the Host Driver shall set start address of the Descriptor table. The ADMA increments this register address, which points to next line, when every fetching a Descriptor line. When the ADMA Error Interrupt is generated, this register shall hold valid Descriptor address depending on the ADMA state. The Host Driver shall program Descriptor Table on 32-bit boundary and set 32-bit boundary address to this register. ADMA2 ignores lower 2-bit of this register and assumes it to be 00b. 32-bit Address ADMA Register Value 32bit System Address"]
pub type HiwdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register holds byte address of executing command of the Descriptor table. 32-bit Address Descriptor uses lower 32bit of this register. At the start of ADMA, the Host Driver shall set start address of the Descriptor table. The ADMA increments this register address, which points to next line, when every fetching a Descriptor line. When the ADMA Error Interrupt is generated, this register shall hold valid Descriptor address depending on the ADMA state. The Host Driver shall program Descriptor Table on 32-bit boundary and set 32-bit boundary address to this register. ADMA2 ignores lower 2-bit of this register and assumes it to be 00b. 32-bit Address ADMA Register Value 32bit System Address"]
    #[inline(always)]
    pub fn hiwd(&self) -> HiwdR {
        HiwdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register holds byte address of executing command of the Descriptor table. 32-bit Address Descriptor uses lower 32bit of this register. At the start of ADMA, the Host Driver shall set start address of the Descriptor table. The ADMA increments this register address, which points to next line, when every fetching a Descriptor line. When the ADMA Error Interrupt is generated, this register shall hold valid Descriptor address depending on the ADMA state. The Host Driver shall program Descriptor Table on 32-bit boundary and set 32-bit boundary address to this register. ADMA2 ignores lower 2-bit of this register and assumes it to be 00b. 32-bit Address ADMA Register Value 32bit System Address"]
    #[inline(always)]
    #[must_use]
    pub fn hiwd(&mut self) -> HiwdW<AdmahiwdSpec> {
        HiwdW::new(self, 0)
    }
}
#[doc = "ADMA system address \\[63:0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`admahiwd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`admahiwd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdmahiwdSpec;
impl crate::RegisterSpec for AdmahiwdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`admahiwd::R`](R) reader structure"]
impl crate::Readable for AdmahiwdSpec {}
#[doc = "`write(|w| ..)` method takes [`admahiwd::W`](W) writer structure"]
impl crate::Writable for AdmahiwdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADMAHIWD to value 0"]
impl crate::Resettable for AdmahiwdSpec {
    const RESET_VALUE: u32 = 0;
}
