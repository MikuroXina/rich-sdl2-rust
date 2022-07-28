/// A converter between the audio format type and `f64`.
#[macro_export(local_inner_macros)]
macro_rules! converter {
    ($target:ty, $chunk:ident, $gained:ident, $inner:expr) => {{
        for (g, bytes) in $gained
            .iter_mut()
            .zip($chunk.chunks_exact(std::mem::size_of::<$target>()))
        {
            use std::convert::TryInto;
            *g = <$target>::from_ne_bytes(bytes.try_into().unwrap()) as f64;
        }
        $inner;
        for (ch, byte) in $chunk.iter_mut().zip(
            $gained
                .iter()
                .flat_map(|&n| <$target>::to_ne_bytes(n as $target)),
        ) {
            *ch = byte;
        }
    }};
    (Lsb, $target:ty, $chunk:ident, $gained:ident, $inner:expr) => {{
        for (g, bytes) in $gained
            .iter_mut()
            .zip($chunk.chunks_exact(std::mem::size_of::<$target>()))
        {
            use std::convert::TryInto;
            *g = <$target>::from_le_bytes(bytes.try_into().unwrap()) as f64;
        }
        $inner;
        for (ch, byte) in $chunk.iter_mut().zip(
            $gained
                .iter()
                .flat_map(|&n| <$target>::to_le_bytes(n as $target)),
        ) {
            *ch = byte;
        }
    }};
    (Msb, $target:ty, $chunk:ident, $gained:ident, $inner:expr) => {{
        for (g, bytes) in $gained
            .iter_mut()
            .zip($chunk.chunks_exact(std::mem::size_of::<$target>()))
        {
            use std::convert::TryInto;
            *g = <$target>::from_be_bytes(bytes.try_into().unwrap()) as f64;
        }
        $inner;
        for (ch, byte) in $chunk.iter_mut().zip(
            $gained
                .iter()
                .flat_map(|&n| <$target>::to_be_bytes(n as $target)),
        ) {
            *ch = byte;
        }
    }};
}
