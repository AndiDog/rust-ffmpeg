use std::ptr;
use std::marker::PhantomData;

use ffi::*;
use ::Error;
use ::format::Context;
use ::device;
use libc::c_int;

impl Context {
	pub fn devices(&self) -> Result<DeviceIter, Error> {
		DeviceIter::new(self.ptr)
	}
}

pub struct DeviceIter<'a> {
	ptr: *mut AVDeviceInfoList,
	cur: c_int,

	_marker: PhantomData<&'a ()>,
}

impl<'a> DeviceIter<'a> {
	pub fn new(ctx: *mut AVFormatContext) -> Result<Self, Error> {
		unsafe {
			let mut ptr: *mut AVDeviceInfoList = ptr::null_mut();

			match avdevice_list_devices(ctx, &mut ptr) {
				n if n < 0 =>
					Err(Error::new(n)),

				_ =>
					Ok(DeviceIter { ptr: ptr, cur: 0, _marker: PhantomData })
			}
		}
	}

	pub fn default(&self) -> usize {
		unsafe {
			(*self.ptr).default_device as usize
		}
	}
}

impl<'a> Drop for DeviceIter<'a> {
	fn drop(&mut self) {
		unsafe {
			avdevice_free_list_devices(&mut self.ptr);
		}
	}
}

impl<'a> Iterator for DeviceIter<'a> {
	type Item = device::Info<'a>;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		unsafe {
			if self.cur >= (*self.ptr).nb_devices {
				None
			}
			else {
				self.cur += 1;
				Some(device::Info::wrap(*(*self.ptr).devices.offset((self.cur - 1) as isize)))
			}
		}
	}
}