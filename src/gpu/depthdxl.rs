#[doc = "Register `DEPTHDXL` reader"]
pub type R = crate::R<DepthdxlSpec>;
#[doc = "Register `DEPTHDXL` writer"]
pub type W = crate::W<DepthdxlSpec>;
#[doc = "Field `XAXISLO` reader - Added depth value for each step at x-axis"]
pub type XaxisloR = crate::FieldReader<u32>;
#[doc = "Field `XAXISLO` writer - Added depth value for each step at x-axis"]
pub type XaxisloW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Added depth value for each step at x-axis"]
    #[inline(always)]
    pub fn xaxislo(&self) -> XaxisloR {
        XaxisloR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Added depth value for each step at x-axis"]
    #[inline(always)]
    #[must_use]
    pub fn xaxislo(&mut self) -> XaxisloW<DepthdxlSpec> {
        XaxisloW::new(self, 0)
    }
}
#[doc = "Added depth value for each step at x-axis (32 low bits fractional.)\n\nYou can [`read`](crate::Reg::read) this register and get [`depthdxl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`depthdxl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DepthdxlSpec;
impl crate::RegisterSpec for DepthdxlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`depthdxl::R`](R) reader structure"]
impl crate::Readable for DepthdxlSpec {}
#[doc = "`write(|w| ..)` method takes [`depthdxl::W`](W) writer structure"]
impl crate::Writable for DepthdxlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEPTHDXL to value 0"]
impl crate::Resettable for DepthdxlSpec {
    const RESET_VALUE: u32 = 0;
}
