#[doc = "Register `SWT` reader"]
pub type R = crate::R<SwtSpec>;
#[doc = "Register `SWT` writer"]
pub type W = crate::W<SwtSpec>;
#[doc = "Writing 0x37 to this register generates a software trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Swt {
    #[doc = "55: Writing this value generates a software trigger."]
    GenSwTrigger = 55,
    #[doc = "0: Default value."]
    NoSwTrigger = 0,
}
impl From<Swt> for u8 {
    #[inline(always)]
    fn from(variant: Swt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Swt {
    type Ux = u8;
}
impl crate::IsEnum for Swt {}
#[doc = "Field `SWT` reader - Writing 0x37 to this register generates a software trigger."]
pub type SwtR = crate::FieldReader<Swt>;
impl SwtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Swt> {
        match self.bits {
            55 => Some(Swt::GenSwTrigger),
            0 => Some(Swt::NoSwTrigger),
            _ => None,
        }
    }
    #[doc = "Writing this value generates a software trigger."]
    #[inline(always)]
    pub fn is_gen_sw_trigger(&self) -> bool {
        *self == Swt::GenSwTrigger
    }
    #[doc = "Default value."]
    #[inline(always)]
    pub fn is_no_sw_trigger(&self) -> bool {
        *self == Swt::NoSwTrigger
    }
}
#[doc = "Field `SWT` writer - Writing 0x37 to this register generates a software trigger."]
pub type SwtW<'a, REG> = crate::FieldWriter<'a, REG, 8, Swt>;
impl<'a, REG> SwtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing this value generates a software trigger."]
    #[inline(always)]
    pub fn gen_sw_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Swt::GenSwTrigger)
    }
    #[doc = "Default value."]
    #[inline(always)]
    pub fn no_sw_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Swt::NoSwTrigger)
    }
}
impl R {
    #[doc = "Bits 0:7 - Writing 0x37 to this register generates a software trigger."]
    #[inline(always)]
    pub fn swt(&self) -> SwtR {
        SwtR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Writing 0x37 to this register generates a software trigger."]
    #[inline(always)]
    #[must_use]
    pub fn swt(&mut self) -> SwtW<SwtSpec> {
        SwtW::new(self, 0)
    }
}
#[doc = "This register enables initiating an ADC scan through software.\n\nYou can [`read`](crate::Reg::read) this register and get [`swt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwtSpec;
impl crate::RegisterSpec for SwtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swt::R`](R) reader structure"]
impl crate::Readable for SwtSpec {}
#[doc = "`write(|w| ..)` method takes [`swt::W`](W) writer structure"]
impl crate::Writable for SwtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWT to value 0"]
impl crate::Resettable for SwtSpec {
    const RESET_VALUE: u32 = 0;
}
