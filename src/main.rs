#![no_std]
#![no_main]

use uefi::Status;
use uefi_raw::protocol::console::InputKey;

#[panic_handler]
fn panic<'a>(_info: &core::panic::PanicInfo<'a>) -> ! {
    loop {}
}

#[unsafe(export_name = "efi_main")]
extern "efiapi" fn uefi_main(
    image_handle: ::uefi::Handle,
    system_table_raw: *const ::core::ffi::c_void,
) -> uefi::prelude::Status {
    unsafe {
        ::uefi::boot::set_image_handle(image_handle);
        ::uefi::table::set_system_table(system_table_raw.cast());
    }

    let table: *mut uefi_raw::table::system::SystemTable = system_table_raw
        .cast::<uefi_raw::table::system::SystemTable>()
        .cast_mut();
    let stdout = unsafe { (*table).stdout };
    let stdin = unsafe { (*table).stdin };
    let mut num = 0;
    let mut correct_key = InputKey {
        scan_code: 16,
        unicode_char: b'Q' as u16,
    };
    let mut guys = 0;
    let mut guycost = 10;
    let mut n = 0;
    loop {
        if unsafe {
            ((*stdin).read_key_stroke)(stdin, (&mut correct_key) as *mut InputKey)
                == Status::SUCCESS
        } && num >= guycost
        {
            num -= guycost;
            guys += 1;
            guycost = guycost + (guycost / 10)
        }
        if n == 0 {
            let _ = unsafe { ((*stdout).clear_screen)(stdout) };
            let _ = unsafe { ((*stdout).output_string)(stdout, get_num_string(num).as_ptr()) };
            let _ = unsafe { ((*stdout).output_string)(stdout, get_num_string(guys).as_ptr()) };
            let _ = unsafe { ((*stdout).output_string)(stdout, get_num_string(guycost).as_ptr()) };
        }
        for _ in 0..10_000 {
            let _ = 2 + 2;
        }
        n = (n + 1) % 100;
        num += 1 + guys;
    }
}

fn get_num_string(num: u64) -> [u16; 64] {
    let mut ret = [0; 64];
    if num == 0 {
        ret[0] = b'0' as u16;
        ret[1] = b'\n' as u16;
        return ret;
    }
    let mut running = num;
    let mut ind = max_ten_pow(num) as usize;
    ret[ind] = b'\n' as u16;
    ind -= 1;
    loop {
        ret[ind] = ((running % 10) as u16) + (b'0' as u16);
        running /= 10;
        if ind == 0 {
            break;
        }
        ind -= 1;
    }
    ret
}
//max power of ten strictly less
//number of digits in num
fn max_ten_pow(num: u64) -> u16 {
    let mut test: u16 = 0;
    if num == 0 {
        return 1;
    }
    loop {
        if num < 10_u64.pow(test.into()) {
            break test;
        }
        test += 1;
    }
}
