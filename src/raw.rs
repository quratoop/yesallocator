use core::alloc::Layout;

#[inline]
pub unsafe fn alloc(layout: Layout) -> *mut u8 {
    unsafe {
        // SAFETY: der user muss garantieren dass groesse und ausrichtung legal ist.
        sys::yesmalloc(layout.size() as libc::size_t, layout.align() as libc::size_t) as *mut u8
    }
}

#[inline]
pub unsafe fn alloc_zeroed(layout: Layout) -> *mut u8 {
    unsafe {
        // SAFETY: der user muss garantieren dass groesse und ausrichtung legal ist.
        sys::yescalloc(layout.size() as libc::size_t, layout.align() as libc::size_t) as *mut u8
    }
}

#[inline]
pub unsafe fn realloc(ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
    unsafe {
        // SAFETY: der user muss garantieren dass der ptr von einem dieser allocatoren in dieser carte alloziert, der layout fuer das ptr korrekt und new_size gueltig ist.
        sys::yesremalloc(ptr as *mut libc::c_void, layout.size() as libc::size_t, layout.align() as libc::size_t, new_size as libc::size_t) as *mut u8
    }
}

#[inline]
pub unsafe fn realloc_zeroed(ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
    unsafe {
        // SAFETY: der user muss garantieren dass der ptr von einem dieser allocatoren in dieser carte alloziert, der layout fuer das ptr korrekt und new_size gueltig ist.
        sys::yesrecalloc(ptr as *mut libc::c_void, layout.size() as libc::size_t, layout.align() as libc::size_t, new_size as libc::size_t) as *mut u8
    }
}

#[inline]
pub unsafe fn dealloc(ptr: *mut u8, layout: Layout) {
    unsafe {
         // SAFETY: der user muss garantieren dass der ptr von einem dieser allocatoren in dieser carte alloziert und das der layout fuer das ptr korrekt ist.
         sys::yesfree(ptr as *mut libc::c_void, layout.size() as libc::size_t, layout.align() as libc::size_t);
    }
}

#[cfg(test)]
mod tests {
    use std::{alloc::Layout, ptr};
    use super::*;

    #[test]
    fn can_i_alloc() -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            // fangen wir mit was einfachem an, einem char.
            let layout_for_char = Layout::new::<char>();

            // mal die groesse checken es sollte 4 sein.
            assert_eq!(layout_for_char.size(), 4);
            // mal die ausrichtung checken es sollte 4 sein.
            assert_eq!(layout_for_char.align(), 4);

            // jetzt allozieren wir einen neuen ptr.
            let ptr = alloc(layout_for_char) as *mut char;

            // damit wir sicherstellen das wir keine falschen zugriffe auf ungueltige ptr mache wird auf null geprueft.
            assert!(!ptr.is_null());

            // so nun schreiben wir einen char in den pointer.
            ptr::write(ptr, '\u{1F60A}');  // '\u{1F60A}' ist ein smiley. 

            // wir lesen die daten aus dem pointer.
            let data = ptr::read(ptr as *const char);

            // da wir den pointer ab diesen moment nicht mehr brauchen geben wir ihn frei.
            dealloc(ptr as *mut u8, layout_for_char);

            // und wir ueberpruefen den wert den der pointer gespeichert hat.
            assert_eq!(data, '\u{1F60A}');
        }
        Ok(())
    }
}