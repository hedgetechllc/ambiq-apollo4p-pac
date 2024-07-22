#[doc = "Register `HOSTCRYPTOKEYSEL` reader"]
pub type R = crate::R<HostcryptokeyselSpec>;
#[doc = "Register `HOSTCRYPTOKEYSEL` writer"]
pub type W = crate::W<HostcryptokeyselSpec>;
#[doc = "Select the source of the HW key that is used by the AES engine:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selcryptokey {
    #[doc = "0: rkek"]
    Rkek = 0,
    #[doc = "1: the Krtl."]
    Krtl = 1,
    #[doc = "2: the provision key KCP."]
    Kcp = 2,
    #[doc = "3: the code encryption key KCE."]
    Kce = 3,
    #[doc = "4: the KPICV, The ICV provisioning key ."]
    Kpicv = 4,
    #[doc = "5: the code encryption key KCEICV Note: When 'kprtl_lock' is set - kprtl will be masked (trying to load it will load zeros to the AES key register. When 'kcertl_lock' is set - kcertl will be masked (trying to load it will load zeros to the AES key register. When scan_mode is asserted all the RTL keys (Krtll) will be masked."]
    Kceicv = 5,
}
impl From<Selcryptokey> for u8 {
    #[inline(always)]
    fn from(variant: Selcryptokey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selcryptokey {
    type Ux = u8;
}
impl crate::IsEnum for Selcryptokey {}
#[doc = "Field `SELCRYPTOKEY` reader - Select the source of the HW key that is used by the AES engine:"]
pub type SelcryptokeyR = crate::FieldReader<Selcryptokey>;
impl SelcryptokeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Selcryptokey> {
        match self.bits {
            0 => Some(Selcryptokey::Rkek),
            1 => Some(Selcryptokey::Krtl),
            2 => Some(Selcryptokey::Kcp),
            3 => Some(Selcryptokey::Kce),
            4 => Some(Selcryptokey::Kpicv),
            5 => Some(Selcryptokey::Kceicv),
            _ => None,
        }
    }
    #[doc = "rkek"]
    #[inline(always)]
    pub fn is_rkek(&self) -> bool {
        *self == Selcryptokey::Rkek
    }
    #[doc = "the Krtl."]
    #[inline(always)]
    pub fn is_krtl(&self) -> bool {
        *self == Selcryptokey::Krtl
    }
    #[doc = "the provision key KCP."]
    #[inline(always)]
    pub fn is_kcp(&self) -> bool {
        *self == Selcryptokey::Kcp
    }
    #[doc = "the code encryption key KCE."]
    #[inline(always)]
    pub fn is_kce(&self) -> bool {
        *self == Selcryptokey::Kce
    }
    #[doc = "the KPICV, The ICV provisioning key ."]
    #[inline(always)]
    pub fn is_kpicv(&self) -> bool {
        *self == Selcryptokey::Kpicv
    }
    #[doc = "the code encryption key KCEICV Note: When 'kprtl_lock' is set - kprtl will be masked (trying to load it will load zeros to the AES key register. When 'kcertl_lock' is set - kcertl will be masked (trying to load it will load zeros to the AES key register. When scan_mode is asserted all the RTL keys (Krtll) will be masked."]
    #[inline(always)]
    pub fn is_kceicv(&self) -> bool {
        *self == Selcryptokey::Kceicv
    }
}
#[doc = "Field `SELCRYPTOKEY` writer - Select the source of the HW key that is used by the AES engine:"]
pub type SelcryptokeyW<'a, REG> = crate::FieldWriter<'a, REG, 3, Selcryptokey>;
impl<'a, REG> SelcryptokeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rkek"]
    #[inline(always)]
    pub fn rkek(self) -> &'a mut crate::W<REG> {
        self.variant(Selcryptokey::Rkek)
    }
    #[doc = "the Krtl."]
    #[inline(always)]
    pub fn krtl(self) -> &'a mut crate::W<REG> {
        self.variant(Selcryptokey::Krtl)
    }
    #[doc = "the provision key KCP."]
    #[inline(always)]
    pub fn kcp(self) -> &'a mut crate::W<REG> {
        self.variant(Selcryptokey::Kcp)
    }
    #[doc = "the code encryption key KCE."]
    #[inline(always)]
    pub fn kce(self) -> &'a mut crate::W<REG> {
        self.variant(Selcryptokey::Kce)
    }
    #[doc = "the KPICV, The ICV provisioning key ."]
    #[inline(always)]
    pub fn kpicv(self) -> &'a mut crate::W<REG> {
        self.variant(Selcryptokey::Kpicv)
    }
    #[doc = "the code encryption key KCEICV Note: When 'kprtl_lock' is set - kprtl will be masked (trying to load it will load zeros to the AES key register. When 'kcertl_lock' is set - kcertl will be masked (trying to load it will load zeros to the AES key register. When scan_mode is asserted all the RTL keys (Krtll) will be masked."]
    #[inline(always)]
    pub fn kceicv(self) -> &'a mut crate::W<REG> {
        self.variant(Selcryptokey::Kceicv)
    }
}
impl R {
    #[doc = "Bits 0:2 - Select the source of the HW key that is used by the AES engine:"]
    #[inline(always)]
    pub fn selcryptokey(&self) -> SelcryptokeyR {
        SelcryptokeyR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select the source of the HW key that is used by the AES engine:"]
    #[inline(always)]
    #[must_use]
    pub fn selcryptokey(&mut self) -> SelcryptokeyW<HostcryptokeyselSpec> {
        SelcryptokeyW::new(self, 0)
    }
}
#[doc = "AES hardware key select. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostcryptokeysel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostcryptokeysel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostcryptokeyselSpec;
impl crate::RegisterSpec for HostcryptokeyselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostcryptokeysel::R`](R) reader structure"]
impl crate::Readable for HostcryptokeyselSpec {}
#[doc = "`write(|w| ..)` method takes [`hostcryptokeysel::W`](W) writer structure"]
impl crate::Writable for HostcryptokeyselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTCRYPTOKEYSEL to value 0"]
impl crate::Resettable for HostcryptokeyselSpec {
    const RESET_VALUE: u32 = 0;
}
