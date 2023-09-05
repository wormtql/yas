use crate::common::*;

use super::*;

pub trait ConvertToScanInfo<T> {
    fn to_scan_info(&self, size: Size<f64>) -> T;
}

pub trait ScaleToScanInfo<T> {
    fn scale_to_scan(&self, radio: (f64, f64)) -> T;
}

impl ScaleToScanInfo<usize> for usize {
    #[inline(always)]
    fn scale_to_scan(&self, _: (f64, f64)) -> usize {
        *self
    }
}

impl ScaleToScanInfo<RectBound<ScanInfoType>> for RectBound<WindowInfoType> {
    #[inline(always)]
    fn scale_to_scan(&self, (rw, rh): (f64, f64)) -> RectBound<ScanInfoType> {
        let scaled = self.rect_scale(rw, rh);

        RectBound {
            left: scaled.left as ScanInfoType,
            top: scaled.top as ScanInfoType,
            right: scaled.right as ScanInfoType,
            bottom: scaled.bottom as ScanInfoType,
        }
    }
}

impl ScaleToScanInfo<Pos<ScanInfoType>> for Pos<WindowInfoType> {
    #[inline(always)]
    fn scale_to_scan(&self, (rw, rh): (f64, f64)) -> Pos<ScanInfoType> {
        Pos {
            x: (self.x * rw) as ScanInfoType,
            y: (self.y * rh) as ScanInfoType,
        }
    }
}

impl ScaleToScanInfo<Size<ScanInfoType>> for Size<WindowInfoType> {
    #[inline(always)]
    fn scale_to_scan(&self, (rw, rh): (f64, f64)) -> Size<ScanInfoType> {
        Size {
            width: (self.width * rw) as ScanInfoType,
            height: (self.height * rh) as ScanInfoType,
        }
    }
}

impl ScaleToScanInfo<Rect<ScanInfoType, ScanInfoType>> for Rect<WindowInfoType, WindowInfoType> {
    #[inline(always)]
    fn scale_to_scan(&self, radio: (f64, f64)) -> Rect<ScanInfoType, ScanInfoType> {
        Rect {
            origin: self.origin.scale_to_scan(radio),
            size: self.size.scale_to_scan(radio),
        }
    }
}

/// use to define a struct that can be converted to scan info
#[macro_export]
macro_rules! scan_info_convert {
    (
        pub type ScanInfoType = $stype:ty;
        pub type WindowInfoType = $wtype:ty;

        $(#[$outer:meta])*
        pub struct $name:ident<T = ScanInfoType> {
            $(#[$inner:meta])*
            $(
                pub $field:ident: $type:ty,
            )*
        }
    ) => {
        pub type ScanInfoType = $stype;
        pub type WindowInfoType = $wtype;

        $(#[$outer])*
        pub struct $name<T = ScanInfoType> {
            $(#[$inner])*
            $(
                $field: $type,
            )*
        }

        impl ConvertToScanInfo<$name<ScanInfoType>> for $name<WindowInfoType> {
            fn to_scan_info(&self, size: Size<f64>) -> $name<ScanInfoType> {
                let radio = (size.width / self.size.width, size.height / self.size.height);

                $name::<ScanInfoType> {
                    $(
                        $field: self.$field.scale_to_scan(radio),
                    )*
                }
            }
        }
    };
}
