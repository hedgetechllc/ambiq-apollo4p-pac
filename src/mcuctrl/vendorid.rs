#[doc = "Register `VENDORID` reader"]
pub type R = crate::R<VendoridSpec>;
#[doc = "Register `VENDORID` writer"]
pub type W = crate::W<VendoridSpec>;
#[doc = "Unique Vendor ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Vendorid {
    #[doc = "1095582289: Ambiq Vendor ID 'AMBQ'"]
    Ambiq = 1095582289,
    #[doc = "0: Default Vendor ID"]
    Default = 0,
}
impl From<Vendorid> for u32 {
    #[inline(always)]
    fn from(variant: Vendorid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vendorid {
    type Ux = u32;
}
impl crate::IsEnum for Vendorid {}
#[doc = "Field `VENDORID` reader - Unique Vendor ID"]
pub type VendoridR = crate::FieldReader<Vendorid>;
impl VendoridR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vendorid> {
        match self.bits {
            1095582289 => Some(Vendorid::Ambiq),
            0 => Some(Vendorid::Default),
            _ => None,
        }
    }
    #[doc = "Ambiq Vendor ID 'AMBQ'"]
    #[inline(always)]
    pub fn is_ambiq(&self) -> bool {
        *self == Vendorid::Ambiq
    }
    #[doc = "Default Vendor ID"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Vendorid::Default
    }
}
#[doc = "Field `VENDORID` writer - Unique Vendor ID"]
pub type VendoridW<'a, REG> = crate::FieldWriter<'a, REG, 32, Vendorid>;
impl<'a, REG> VendoridW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Ambiq Vendor ID 'AMBQ'"]
    #[inline(always)]
    pub fn ambiq(self) -> &'a mut crate::W<REG> {
        self.variant(Vendorid::Ambiq)
    }
    #[doc = "Default Vendor ID"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Vendorid::Default)
    }
}
impl R {
    #[doc = "Bits 0:31 - Unique Vendor ID"]
    #[inline(always)]
    pub fn vendorid(&self) -> VendoridR {
        VendoridR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Unique Vendor ID"]
    #[inline(always)]
    #[must_use]
    pub fn vendorid(&mut self) -> VendoridW<VendoridSpec> {
        VendoridW::new(self, 0)
    }
}
#[doc = "Unique Vendor ID\n\nYou can [`read`](crate::Reg::read) this register and get [`vendorid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vendorid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VendoridSpec;
impl crate::RegisterSpec for VendoridSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vendorid::R`](R) reader structure"]
impl crate::Readable for VendoridSpec {}
#[doc = "`write(|w| ..)` method takes [`vendorid::W`](W) writer structure"]
impl crate::Writable for VendoridSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VENDORID to value 0"]
impl crate::Resettable for VendoridSpec {
    const RESET_VALUE: u32 = 0;
}
