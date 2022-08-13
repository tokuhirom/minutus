use crate::{mruby::*, types::FromMrb};

// TODO: mrb_gc_register / mrb_gc_unregister を使ってちゃんとGCと向き合う
// new 関数を作って register, drop するときに unregister すれば大丈夫な気がする
pub struct DataPtr<T: Sized> {
    rusty_value_ptr: *mut T,
    minu_value: minu_value,
    mrb: *mut minu_state,
}

impl<T: Sized> DataPtr<T> {
    pub fn minu_value(&self) -> minu_value {
        self.minu_value
    }

    pub fn mrb(&self) -> *mut minu_state {
        self.mrb
    }
}

impl<T> std::ops::Deref for DataPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.rusty_value_ptr) }
    }
}

impl<T: MrbData> FromMrb<DataPtr<T>> for DataPtr<T> {
    fn from_mrb(mrb: *mut minu_state, value: &minu_value) -> DataPtr<T> {
        T::from_mrb_data(mrb, value)
    }
}

pub trait MrbData: Sized {
    fn from_mrb_data<'a>(mrb: *mut minu_state, value: &minu_value) -> DataPtr<Self> {
        unsafe {
            DataPtr {
                rusty_value_ptr: minu_data_get_ptr(mrb, *value, Self::minu_data_type())
                    as *mut Self,
                minu_value: (*value).clone(),
                mrb,
            }
        }
    }
    fn into_mrb_data(self, mrb: *mut minu_state) -> minu_value {
        let size = std::mem::size_of::<Self>();
        unsafe {
            let mem = minu_malloc(mrb, size as u64) as *mut Self;
            core::ptr::write(mem, self);
            let rdata = minu_data_object_alloc(
                mrb,
                Self::minu_class(mrb),
                mem as *mut _,
                Self::minu_data_type(),
            );
            minu_obj_value(rdata as _)
        }
    }
    fn minu_class(mrb: *mut minu_state) -> *mut RClass;
    fn minu_data_type() -> *const minu_data_type;
}
