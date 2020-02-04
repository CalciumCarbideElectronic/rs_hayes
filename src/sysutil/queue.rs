use crate::sysutil::import::{
    osMessageQueueAttr_t, osMessageQueueGet, osMessageQueueId_t, osMessageQueueNew,
    osMessageQueuePut, osStatus_t,
};
use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem::{size_of, zeroed};
use core::ptr::null;

pub struct Queue<T> {
    cmsis_queue_id: osMessageQueueId_t,
    _phantom: PhantomData<T>,
}

impl<T> Queue<T>
where
    T: Sized + Clone,
{
    pub fn new(msg_len: u32) -> Queue<T> {
        let msg_size = size_of::<T>() as u32;
        let id = unsafe {
            osMessageQueueNew(
                msg_len,
                msg_size,
                &osMessageQueueAttr_t {
                    name: null(),
                    attr_bits: 0,
                    cb_mem: null(),
                    cb_size: 0,
                    mq_mem: null(),
                    mq_size: 0,
                },
            )
        };
        Queue {
            cmsis_queue_id: id,
            _phantom: PhantomData,
        }
    }
    pub fn put(&self, data: T, tick: u32) -> Result<osStatus_t, osStatus_t> {
        unsafe {
            let ptr: *const T = &data;
            let status = osMessageQueuePut(self.cmsis_queue_id, ptr as *const c_void, 0, tick);
            if status == osStatus_t::osOK {
                return Ok(status);
            } else {
                return Err(status);
            }
        }
    }
    pub fn get(&self, tick: u32) -> Result<T, osStatus_t> {
        unsafe {
            let mut data: T = zeroed::<T>();
            let ptr: *mut T = &mut data;
            let status = osMessageQueueGet(self.cmsis_queue_id, ptr as *mut c_void, 0, tick);
            if status == osStatus_t::osOK {
                return Ok(data);
            } else {
                return Err(status);
            }
        }
    }
}

impl<T> From<osMessageQueueId_t> for Queue<T>
where
    T: Sized,
{
    fn from(id: osMessageQueueId_t) -> Self {
        return Queue {
            cmsis_queue_id: id,
            _phantom: PhantomData,
        };
    }
}
