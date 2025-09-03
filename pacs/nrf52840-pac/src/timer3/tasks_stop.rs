#[doc = "Register `TASKS_STOP` writer"]
pub type W = crate::W<TasksStopSpec>;
#[doc = "Field `TASKS_STOP` writer - "]
pub type TasksStopW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tasks_stop(&mut self) -> TasksStopW<'_, TasksStopSpec> {
        TasksStopW::new(self, 0)
    }
}
#[doc = "Stop Timer\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksStopSpec;
impl crate::RegisterSpec for TasksStopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_stop::W`](W) writer structure"]
impl crate::Writable for TasksStopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_STOP to value 0"]
impl crate::Resettable for TasksStopSpec {}
