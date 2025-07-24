#[macro_export]
macro_rules! enum_u8 {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl $name {
            pub const unsafe fn from_int(v: u8) -> Self {
                unsafe { std::mem::transmute(v) }
            }

            pub const fn as_int(self) -> u8 {
                self as u8
            }
        }
    }
}

#[macro_export]
macro_rules! enum_i8 {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl $name {
            pub const unsafe fn from_int(v: i8) -> Self {
                unsafe { std::mem::transmute(v) }
            }

            pub const fn as_int(self) -> i8 {
                self as i8
            }
        }
    }
}

pub const fn maxi8(a: i8, b: i8) -> i8 {
    if a > b { a } else { b }
}

pub const fn mini8(a: i8, b: i8) -> i8 {
    if a < b { a } else { b }
}
