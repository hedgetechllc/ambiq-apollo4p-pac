#[doc = "Register `USBRSTCTRL` reader"]
pub type R = crate::R<UsbrstctrlSpec>;
#[doc = "Register `USBRSTCTRL` writer"]
pub type W = crate::W<UsbrstctrlSpec>;
#[doc = "Field `USBRSTENABLE` reader - This bit enables this register control. If set to '1', the reset release bits will be active. If set to '0', this register is not controlling the USB override bits."]
pub type UsbrstenableR = crate::BitReader;
#[doc = "Field `USBRSTENABLE` writer - This bit enables this register control. If set to '1', the reset release bits will be active. If set to '0', this register is not controlling the USB override bits."]
pub type UsbrstenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBPORRSTRELEASE` reader - Set this bit to '1' after USB power domain is up. This will release the reset override condition"]
pub type UsbporrstreleaseR = crate::BitReader;
#[doc = "Field `USBPORRSTRELEASE` writer - Set this bit to '1' after USB power domain is up. This will release the reset override condition"]
pub type UsbporrstreleaseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBUTMIRSTRELEASE` reader - Set this bit to '1' after USB power domain is up. This will release the reset override condition"]
pub type UsbutmirstreleaseR = crate::BitReader;
#[doc = "Field `USBUTMIRSTRELEASE` writer - Set this bit to '1' after USB power domain is up. This will release the reset override condition"]
pub type UsbutmirstreleaseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit enables this register control. If set to '1', the reset release bits will be active. If set to '0', this register is not controlling the USB override bits."]
    #[inline(always)]
    pub fn usbrstenable(&self) -> UsbrstenableR {
        UsbrstenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to '1' after USB power domain is up. This will release the reset override condition"]
    #[inline(always)]
    pub fn usbporrstrelease(&self) -> UsbporrstreleaseR {
        UsbporrstreleaseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to '1' after USB power domain is up. This will release the reset override condition"]
    #[inline(always)]
    pub fn usbutmirstrelease(&self) -> UsbutmirstreleaseR {
        UsbutmirstreleaseR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit enables this register control. If set to '1', the reset release bits will be active. If set to '0', this register is not controlling the USB override bits."]
    #[inline(always)]
    #[must_use]
    pub fn usbrstenable(&mut self) -> UsbrstenableW<UsbrstctrlSpec> {
        UsbrstenableW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to '1' after USB power domain is up. This will release the reset override condition"]
    #[inline(always)]
    #[must_use]
    pub fn usbporrstrelease(&mut self) -> UsbporrstreleaseW<UsbrstctrlSpec> {
        UsbporrstreleaseW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to '1' after USB power domain is up. This will release the reset override condition"]
    #[inline(always)]
    #[must_use]
    pub fn usbutmirstrelease(&mut self) -> UsbutmirstreleaseW<UsbrstctrlSpec> {
        UsbutmirstreleaseW::new(self, 2)
    }
}
#[doc = "USB Reset Startup Control\n\nYou can [`read`](crate::Reg::read) this register and get [`usbrstctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbrstctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbrstctrlSpec;
impl crate::RegisterSpec for UsbrstctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbrstctrl::R`](R) reader structure"]
impl crate::Readable for UsbrstctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`usbrstctrl::W`](W) writer structure"]
impl crate::Writable for UsbrstctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBRSTCTRL to value 0"]
impl crate::Resettable for UsbrstctrlSpec {
    const RESET_VALUE: u32 = 0;
}
