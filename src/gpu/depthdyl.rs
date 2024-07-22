#[doc = "Register `DEPTHDYL` reader"]
pub type R = crate::R<DepthdylSpec>;
#[doc = "Register `DEPTHDYL` writer"]
pub type W = crate::W<DepthdylSpec>;
#[doc = "Field `YAXISLO` reader - Added depth value for each step at y-axis"]
pub type YaxisloR = crate::FieldReader<u32>;
#[doc = "Field `YAXISLO` writer - Added depth value for each step at y-axis"]
pub type YaxisloW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Added depth value for each step at y-axis"]
    #[inline(always)]
    pub fn yaxislo(&self) -> YaxisloR {
        YaxisloR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Added depth value for each step at y-axis"]
    #[inline(always)]
    #[must_use]
    pub fn yaxislo(&mut self) -> YaxisloW<DepthdylSpec> {
        YaxisloW::new(self, 0)
    }
}
#[doc = "Added depth value for each step at y-axis (32 low bits fractional.)\n\nYou can [`read`](crate::Reg::read) this register and get [`depthdyl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`depthdyl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DepthdylSpec;
impl crate::RegisterSpec for DepthdylSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`depthdyl::R`](R) reader structure"]
impl crate::Readable for DepthdylSpec {}
#[doc = "`write(|w| ..)` method takes [`depthdyl::W`](W) writer structure"]
impl crate::Writable for DepthdylSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEPTHDYL to value 0"]
impl crate::Resettable for DepthdylSpec {
    const RESET_VALUE: u32 = 0;
}
