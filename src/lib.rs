use std::mem;
use std::slice;
use std::os::raw::c_void;


#[no_mangle]
pub extern "C" fn malloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, size: usize) {
    unsafe {
        let _buf = Vec::from_raw_parts(ptr, 0, size);
    }
}

#[no_mangle]
pub extern "C" fn mandelbrot(
    num_runs: usize,
    ptr: *mut u8,
    width: usize,
    height: usize,
    max_iterations: usize,
    real_min: f64,
    real_max: f64,
    imag_min: f64,
    imag_max: f64,
    set_col: u8
) {
    if width == 0 || height == 0 {
        return;
    }

    let pixel_size = width * height;
    let byte_size  = pixel_size * 4;
    let sl = unsafe { slice::from_raw_parts_mut(ptr, byte_size) };

    let real_ratio = (real_max - real_min) / (width as f64);
    let imag_ratio = (imag_max - imag_min) / (height as f64);

    for _ in 0..num_runs {
        for y in 0..height {
            let buf_off_y = y * width * 4;
            let c_imag: f64 = (y as f64) * imag_ratio + imag_min;
            for x in 0..width {
                let buf_off_x = buf_off_y + (x * 4);
                let c_real: f64 = (x as f64) * real_ratio + real_min;

                let mut iters: usize = 1;
                let mut a: f64 = c_real;
                let mut b: f64 = c_imag;
                while iters < max_iterations {
                    let a_temp = (a * a - b * b) + c_real;
                    b = (2f64 * a * b) + c_imag;
                    a = a_temp;
                    if (a * a + b * b) > 4f64 {
                        break;
                    }
                    iters += 1;
                }

                let mut col: u8;
                if iters == max_iterations {
                    col = set_col;
                } else {
                    col = ((iters as f64) * (255f64 / (max_iterations as f64))) as u8;
                }

                sl[buf_off_x + 0] = col;
                sl[buf_off_x + 1] = col;
                sl[buf_off_x + 2] = col;
                sl[buf_off_x + 3] = 255;
            }
        }
    }
}


#[cfg(test)]
mod tests {
}
