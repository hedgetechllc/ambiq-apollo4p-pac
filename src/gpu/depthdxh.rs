#[doc = "Register `DEPTHDXH` reader"]
pub type R = crate::R<DepthdxhSpec>;
#[doc = "Register `DEPTHDXH` writer"]
pub type W = crate::W<DepthdxhSpec>;
#[doc = "Field `XAXISHI` reader - Added depth value for each step at x-axis"]
pub type XaxishiR = crate::FieldReader<u32>;
#[doc = "Field `XAXISHI` writer - Added depth value for each step at x-axis"]
pub type XaxishiW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Added depth value for each step at x-axis"]
    #[inline(always)]
    pub fn xaxishi(&self) -> XaxishiR {
        XaxishiR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Added depth value for each step at x-axis"]
    #[inline(always)]
    #[must_use]
    pub fn xaxishi(&mut self) -> XaxishiW<DepthdxhSpec> {
        XaxishiW::new(self, 0)
    }
}
#[doc = "Added depth value for each step at x-axis (32 high bits integral.)\n\nYou can [`read`](crate::Reg::read) this register and get [`depthdxh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`depthdxh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DepthdxhSpec;
impl crate::RegisterSpec for DepthdxhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`depthdxh::R`](R) reader structure"]
impl crate::Readable for DepthdxhSpec {}
#[doc = "`write(|w| ..)` method takes [`depthdxh::W`](W) writer structure"]
impl crate::Writable for DepthdxhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEPTHDXH to value 0"]
impl crate::Resettable for DepthdxhSpec {
    const RESET_VALUE: u32 = 0;
}
