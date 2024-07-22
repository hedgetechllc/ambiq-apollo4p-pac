#[doc = "Register `DEV0AXI` reader"]
pub type R = crate::R<Dev0axiSpec>;
#[doc = "Register `DEV0AXI` writer"]
pub type W = crate::W<Dev0axiSpec>;
#[doc = "Indicates the AXI aperture size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Size0 {
    #[doc = "0: 64KB Aperture"]
    Size64k = 0,
    #[doc = "1: 128KB Aperture"]
    Size128k = 1,
    #[doc = "2: 256KB Aperture"]
    Size256k = 2,
    #[doc = "3: 512KB Aperture"]
    Size512k = 3,
    #[doc = "4: 1MB Aperture"]
    Size1m = 4,
    #[doc = "5: 2MB Aperture"]
    Size2m = 5,
    #[doc = "6: 4MB Aperture"]
    Size4m = 6,
    #[doc = "7: 8MB Aperture"]
    Size8m = 7,
    #[doc = "8: 16MB Aperture"]
    Size16m = 8,
    #[doc = "9: 32MB Aperture"]
    Size32m = 9,
    #[doc = "10: 64MB Aperture"]
    Size64m = 10,
}
impl From<Size0> for u8 {
    #[inline(always)]
    fn from(variant: Size0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Size0 {
    type Ux = u8;
}
impl crate::IsEnum for Size0 {}
#[doc = "Field `SIZE0` reader - Indicates the AXI aperture size"]
pub type Size0R = crate::FieldReader<Size0>;
impl Size0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Size0> {
        match self.bits {
            0 => Some(Size0::Size64k),
            1 => Some(Size0::Size128k),
            2 => Some(Size0::Size256k),
            3 => Some(Size0::Size512k),
            4 => Some(Size0::Size1m),
            5 => Some(Size0::Size2m),
            6 => Some(Size0::Size4m),
            7 => Some(Size0::Size8m),
            8 => Some(Size0::Size16m),
            9 => Some(Size0::Size32m),
            10 => Some(Size0::Size64m),
            _ => None,
        }
    }
    #[doc = "64KB Aperture"]
    #[inline(always)]
    pub fn is_size64k(&self) -> bool {
        *self == Size0::Size64k
    }
    #[doc = "128KB Aperture"]
    #[inline(always)]
    pub fn is_size128k(&self) -> bool {
        *self == Size0::Size128k
    }
    #[doc = "256KB Aperture"]
    #[inline(always)]
    pub fn is_size256k(&self) -> bool {
        *self == Size0::Size256k
    }
    #[doc = "512KB Aperture"]
    #[inline(always)]
    pub fn is_size512k(&self) -> bool {
        *self == Size0::Size512k
    }
    #[doc = "1MB Aperture"]
    #[inline(always)]
    pub fn is_size1m(&self) -> bool {
        *self == Size0::Size1m
    }
    #[doc = "2MB Aperture"]
    #[inline(always)]
    pub fn is_size2m(&self) -> bool {
        *self == Size0::Size2m
    }
    #[doc = "4MB Aperture"]
    #[inline(always)]
    pub fn is_size4m(&self) -> bool {
        *self == Size0::Size4m
    }
    #[doc = "8MB Aperture"]
    #[inline(always)]
    pub fn is_size8m(&self) -> bool {
        *self == Size0::Size8m
    }
    #[doc = "16MB Aperture"]
    #[inline(always)]
    pub fn is_size16m(&self) -> bool {
        *self == Size0::Size16m
    }
    #[doc = "32MB Aperture"]
    #[inline(always)]
    pub fn is_size32m(&self) -> bool {
        *self == Size0::Size32m
    }
    #[doc = "64MB Aperture"]
    #[inline(always)]
    pub fn is_size64m(&self) -> bool {
        *self == Size0::Size64m
    }
}
#[doc = "Field `SIZE0` writer - Indicates the AXI aperture size"]
pub type Size0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Size0>;
impl<'a, REG> Size0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "64KB Aperture"]
    #[inline(always)]
    pub fn size64k(self) -> &'a mut crate::W<REG> {
        self.variant(Size0::Size64k)
    }
    #[doc = "128KB Aperture"]
    #[inline(always)]
    pub fn size128k(self) -> &'a mut crate::W<REG> {
        self.variant(Size0::Size128k)
    }
    #[doc = "256KB Aperture"]
    #[inline(always)]
    pub fn size256k(self) -> &'a mut crate::W<REG> {
        self.variant(Size0::Size256k)
    }
    #[doc = "512KB Aperture"]
    #[inline(always)]
    pub fn size512k(self) -> &'a mut crate::W<REG> {
        self.variant(Size0::Size512k)
    }
    #[doc = "1MB Aperture"]
    #[inline(always)]
    pub fn size1m(self) -> &'a mut crate::W<REG> {
        self.variant(Size0::Size1m)
    }
    #[doc = "2MB Aperture"]
    #[inline(always)]
    pub fn size2m(self) -> &'a mut crate::W<REG> {
        self.variant(Size0::Size2m)
    }
    #[doc = "4MB Aperture"]
    #[inline(always)]
    pub fn size4m(self) -> &'a mut crate::W<REG> {
        self.variant(Size0::Size4m)
    }
    #[doc = "8MB Aperture"]
    #[inline(always)]
    pub fn size8m(self) -> &'a mut crate::W<REG> {
        self.variant(Size0::Size8m)
    }
    #[doc = "16MB Aperture"]
    #[inline(always)]
    pub fn size16m(self) -> &'a mut crate::W<REG> {
        self.variant(Size0::Size16m)
    }
    #[doc = "32MB Aperture"]
    #[inline(always)]
    pub fn size32m(self) -> &'a mut crate::W<REG> {
        self.variant(Size0::Size32m)
    }
    #[doc = "64MB Aperture"]
    #[inline(always)]
    pub fn size64m(self) -> &'a mut crate::W<REG> {
        self.variant(Size0::Size64m)
    }
}
#[doc = "Indicates the AXI aperture is read-only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Readonly0 {
    #[doc = "1: Indicates AXI aperture only supports read operations"]
    Readonly = 1,
    #[doc = "0: Indicates AXI aperture supports read and write operations"]
    Readwrite = 0,
}
impl From<Readonly0> for bool {
    #[inline(always)]
    fn from(variant: Readonly0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READONLY0` reader - Indicates the AXI aperture is read-only"]
pub type Readonly0R = crate::BitReader<Readonly0>;
impl Readonly0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Readonly0 {
        match self.bits {
            true => Readonly0::Readonly,
            false => Readonly0::Readwrite,
        }
    }
    #[doc = "Indicates AXI aperture only supports read operations"]
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        *self == Readonly0::Readonly
    }
    #[doc = "Indicates AXI aperture supports read and write operations"]
    #[inline(always)]
    pub fn is_readwrite(&self) -> bool {
        *self == Readonly0::Readwrite
    }
}
#[doc = "Field `READONLY0` writer - Indicates the AXI aperture is read-only"]
pub type Readonly0W<'a, REG> = crate::BitWriter<'a, REG, Readonly0>;
impl<'a, REG> Readonly0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Indicates AXI aperture only supports read operations"]
    #[inline(always)]
    pub fn readonly(self) -> &'a mut crate::W<REG> {
        self.variant(Readonly0::Readonly)
    }
    #[doc = "Indicates AXI aperture supports read and write operations"]
    #[inline(always)]
    pub fn readwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Readonly0::Readwrite)
    }
}
#[doc = "Field `BASE0` reader - XIPEN has to be enabled to enable aperture. The BASE address needs to be SIZE aligned."]
pub type Base0R = crate::FieldReader<u16>;
#[doc = "Field `BASE0` writer - XIPEN has to be enabled to enable aperture. The BASE address needs to be SIZE aligned."]
pub type Base0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:3 - Indicates the AXI aperture size"]
    #[inline(always)]
    pub fn size0(&self) -> Size0R {
        Size0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Indicates the AXI aperture is read-only"]
    #[inline(always)]
    pub fn readonly0(&self) -> Readonly0R {
        Readonly0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:25 - XIPEN has to be enabled to enable aperture. The BASE address needs to be SIZE aligned."]
    #[inline(always)]
    pub fn base0(&self) -> Base0R {
        Base0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Indicates the AXI aperture size"]
    #[inline(always)]
    #[must_use]
    pub fn size0(&mut self) -> Size0W<Dev0axiSpec> {
        Size0W::new(self, 0)
    }
    #[doc = "Bit 4 - Indicates the AXI aperture is read-only"]
    #[inline(always)]
    #[must_use]
    pub fn readonly0(&mut self) -> Readonly0W<Dev0axiSpec> {
        Readonly0W::new(self, 4)
    }
    #[doc = "Bits 16:25 - XIPEN has to be enabled to enable aperture. The BASE address needs to be SIZE aligned."]
    #[inline(always)]
    #[must_use]
    pub fn base0(&mut self) -> Base0W<Dev0axiSpec> {
        Base0W::new(self, 16)
    }
}
#[doc = "Specifies the base address and aperture range of the device as mapped onto the AXI bus\n\nYou can [`read`](crate::Reg::read) this register and get [`dev0axi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev0axi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dev0axiSpec;
impl crate::RegisterSpec for Dev0axiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev0axi::R`](R) reader structure"]
impl crate::Readable for Dev0axiSpec {}
#[doc = "`write(|w| ..)` method takes [`dev0axi::W`](W) writer structure"]
impl crate::Writable for Dev0axiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEV0AXI to value 0"]
impl crate::Resettable for Dev0axiSpec {
    const RESET_VALUE: u32 = 0;
}
