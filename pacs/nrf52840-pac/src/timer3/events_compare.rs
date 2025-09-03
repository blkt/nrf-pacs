#[doc = "Register `EVENTS_COMPARE[%s]` reader"]
pub type R = crate::R<EventsCompareSpec>;
#[doc = "Register `EVENTS_COMPARE[%s]` writer"]
pub type W = crate::W<EventsCompareSpec>;
#[doc = "Field `EVENTS_COMPARE` reader - "]
pub type EventsCompareR = crate::BitReader;
#[doc = "Field `EVENTS_COMPARE` writer - "]
pub type EventsCompareW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_compare(&self) -> EventsCompareR {
        EventsCompareR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_compare(&mut self) -> EventsCompareW<'_, EventsCompareSpec> {
        EventsCompareW::new(self, 0)
    }
}
#[doc = "Description collection\\[n\\]: Compare event on CC\\[n\\] match\n\nYou can [`read`](crate::Reg::read) this register and get [`events_compare::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_compare::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsCompareSpec;
impl crate::RegisterSpec for EventsCompareSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_compare::R`](R) reader structure"]
impl crate::Readable for EventsCompareSpec {}
#[doc = "`write(|w| ..)` method takes [`events_compare::W`](W) writer structure"]
impl crate::Writable for EventsCompareSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_COMPARE[%s] to value 0"]
impl crate::Resettable for EventsCompareSpec {}
