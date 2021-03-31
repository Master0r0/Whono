use std::io;
use std::{io::{Write, Read}};
use serde::{Deserialize, Serialize};
use byteorder::{ReadBytesExt, BigEndian, WriteBytesExt};

pub trait Protocol {
    type Object;

    fn byte_length(value: &Self::Object) -> usize;
    fn encode(value: &Self::Object, dest: &mut dyn Write) -> io::Result<()>;
    fn decode(src: &mut dyn Read) -> io::Result<Self::Object>;
}

pub trait PacketWrite {
    fn packet_len(&self) -> usize;
    fn packet_encode(&self, dst: &mut dyn Write) -> io::Result<()>;

    fn write(&self, dst: &mut dyn Write) -> io::Result<()> {
        let len = self.packet_len();
        u16::encode(&(len as u16), dst)?;
        self.packet_encode(dst)
    }
}

pub trait PacketRead: Sized {
    fn packet_decode(src: &mut dyn Read) -> io::Result<Self>;

    fn read<R: Read>(src: &mut R) -> io::Result<Self> {
        let proto_len = <u16 as Protocol>::decode(src)?;
        Self::packet_decode(&mut src.take(proto_len as u64))
    }
}

macro_rules! struct_protocol {
    ($s_name:ident{$($name:ident: $val:ty),*} ) => {
        
        struct $s_name {
            $(
                $name: $val,
            )*
        }

        impl $s_name {
            #[allow(unused)]
            pub fn new($($name: $val,)*) -> Self {
                Self {
                    $(
                        $name
                    ),*
                }
            }
        }

        impl Protocol for $s_name {
            type Object = Self;
            #[allow(unused)]
            fn byte_length(value: &$s_name) -> usize {
                $(
                    <$val as Protocol>::byte_length(&value.$name) +
                )*
                0
            }
            #[allow(unused)]
            fn encode(value: &$s_name, dest: &mut dyn Write) -> io::Result<()> {
                $(
                    <$val as Protocol>::encode(&value.$name, dest)?;
                )*

                Ok(())
            }
            #[allow(unused)]
            fn decode(src: &mut dyn Read) -> io::Result<$s_name> {
                Ok(
                    $s_name {
                        $(
                            $name: <$val as Protocol>::decode(src)?
                        ),*
                    }
                )
            }
        }

    };
    // TODO: Support tuple structs
}

macro_rules! impl_protocol {
    ($name:ty, 1, $encode_name:ident, $decode_name:ident) => {
        impl Protocol for $name {
            type Object = Self;

            fn byte_length(_: &$name) -> usize { 1 }

            fn encode(value: &$name, dest: &mut dyn Write) -> io::Result<()> {
                dest.$encode_name(*value)?;
                Ok(())
            }

            fn decode(src: &mut dyn Read) -> io::Result<$name> {
                src.$decode_name().map_err(|err| io::Error::from(err))
            }
        }
    };
    // Type name, byte length, byteorder encode name, byteorder decode name
    ($name:ty, $len:expr, $encode_name:ident, $decode_name:ident) => {
        impl Protocol for $name {
            type Object = Self;

            fn byte_length(_: &$name) -> usize { $len }

            fn encode(value: &$name, dest: &mut dyn Write) -> io::Result<()> {
                dest.$encode_name::<BigEndian>(*value)?;
                Ok(())
            }

            fn decode(src: &mut dyn Read) -> io::Result<$name> {
                src.$decode_name::<BigEndian>().map_err(|err| io::Error::from(err))
            }
        }
    };
}
#[allow(unused_macros)]
macro_rules! packets {
    ($($id:expr => $s_name:ident{$($name:ident: $val:ty),*}),*) => {
        enum Packet {
            // Creates an enum with the name of the struct and has the struct as data
            $($s_name($s_name),)*
            #[allow(unused)]
            Unknown,
        }

        $(
            struct_protocol!($s_name{$($name: $val),*});

            impl PacketWrite for $s_name {
                fn packet_len(&self) -> usize {
                    let id_len = <u16 as Protocol>::byte_length(&$id);
                    id_len + <Self as Protocol>::byte_length(self)
                }
    
                fn packet_encode(&self, dst: &mut dyn Write) -> io::Result<()> {
                    <u16 as Protocol>::encode(&$id, dst)?;
                    <Self as Protocol>::encode(self, dst)
                }
            }
        )*

        impl PacketRead for Packet {
            fn packet_decode(src: &mut dyn Read) -> io::Result<Self> {
                match <u16 as Protocol>::decode(src)? {
                    $($id => <$s_name as Protocol>::decode(src).map(Packet::$s_name),)*
                    _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unknown Packet id")),
                }
            }
        }


    };
}

impl_protocol!(i8,  1, write_i8,  read_i8);
impl_protocol!(u8,  1, write_u8,  read_u8);
impl_protocol!(i16, 2, write_i16, read_i16);
impl_protocol!(u16, 2, write_u16, read_u16);
impl_protocol!(i32, 4, write_i32, read_i32);
impl_protocol!(u32, 4, write_u32, read_u32);
impl_protocol!(i64, 8, write_i64, read_i64);
impl_protocol!(u64, 8, write_u64, read_u64);
impl_protocol!(f32, 4, write_f32, read_f32);
impl_protocol!(f64, 8, write_f64, read_f64);

impl Protocol for String {
    type Object = String;

    fn byte_length(value: &Self::Object) -> usize {
        value.as_bytes().len()
    }

    fn encode(value: &Self::Object, dest: &mut dyn Write) -> io::Result<()> {
        dest.write_all(value.as_bytes())?;
        Ok(())
    }

    fn decode(src: &mut dyn Read) -> io::Result<Self::Object> {
        let mut value: String = String::new();
        loop {
            let byte = src.read_u8()?;
            if byte == b'\n' {break;}
            value.push(char::from(byte));
        }

        Ok(value)
    }
}

impl Protocol for bool {
    type Object = bool;

    fn byte_length(_value: &Self::Object) -> usize { 1 }

    fn encode(value: &Self::Object, dest: &mut dyn Write) -> io::Result<()> {
        dest.write_u8(if *value {1} else {0})?;
        Ok(())
    }

    fn decode(src: &mut dyn Read) -> io::Result<Self::Object> {
        let value = src.read_u8()?;
        if value > 1 {
            Err(io::Error::new(io::ErrorKind::InvalidInput, &format!("Invalid bool value, expected 0 or 1, got {:#}", value)[..]))
        } else {
            Ok(value == 1)
        }
    }
}

impl<T: Protocol> Protocol for Option<T> {
    type Object = Option<T::Object>;

    fn byte_length(value: &Self::Object) -> usize {
        match *value {
            Some(ref inner) => 1 + T::byte_length(inner),
            None => 1,
        }
    }

    fn encode(value: &Self::Object, dest: &mut dyn Write) -> io::Result<()> {
        // If value has a data (Some) encode a true and the data into the writer
        match *value {
            Some(ref inner) => {
                bool::encode(&true, dest)?;
                T::encode(inner, dest)?;
            }
            None => {
                bool::encode(&false, dest)?;
            }
        }
        Ok(())
    }

    fn decode(src: &mut dyn Read) -> io::Result<Self::Object> {
        // Decode a boolean from the src, if it's true then decode and return it's contents
        if bool::decode(src)? {
            Ok(Some(T::decode(src)?))
        } else {
            Ok(None)
        }
    }
}

mod web_socket {
    mod client_bound {
        use super::super::{Protocol, PacketRead, PacketWrite};
        use std::io;
        use std::io::{Read, Write};

        packets! {
            0x00 => Ping{}
        }
    }

    mod server_bound {
        use super::super::{Protocol, PacketRead, PacketWrite};
        use std::io;
        use std::io::{Read, Write};

        packets! {
            0x00 => Pong{},
            0x01 => Create{creator_name: String, lobby_name: String}
        }
    }
}