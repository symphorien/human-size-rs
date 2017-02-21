use super::*;

#[test]
fn size_try_into_u32() {
    let tests = vec![
		(Size::new(1, Multiple::Byte), Ok(1)),

        (Size::new(1, Multiple::Kilobyte), Ok(1_000)),
        (Size::new(23, Multiple::Kilobyte), Ok(23_000)),
        (Size::new(65, Multiple::Megabyte), Ok(65_000_000)),

        (Size::new(10, Multiple::Mebibyte), Ok(10_485_760)),
    ];

    for test in tests {
        let got: Result<u32, ConversionError> = test.0.clone().try_into();
        let want = test.1;
        assert_eq!(got, want, "input: {:?}", test.0);
    }
}

#[test]
fn size_try_into_u64() {
    let tests = vec![
		(Size::new(1, Multiple::Byte), Ok(1)),

        (Size::new(1, Multiple::Kilobyte), Ok(1_000)),
        (Size::new(23, Multiple::Kilobyte), Ok(23_000)),
        (Size::new(65, Multiple::Megabyte), Ok(65_000_000)),
        (Size::new(123, Multiple::Gigabyte), Ok(123_000_000_000)),
        (Size::new(2, Multiple::Petabyte), Ok(2_000_000_000_000_000)),

        (Size::new(10, Multiple::Mebibyte), Ok(10_485_760)),
        (Size::new(1000, Multiple::Gigibyte), Ok(1_073_741_824_000)),
        (Size::new(1, Multiple::Pebibyte), Ok(1_125_899_906_842_624)),
        (Size::new(2, Multiple::Pebibyte), Ok(2_251_799_813_685_248)),
    ];

    for test in tests {
        let got: Result<u64, ConversionError> = test.0.clone().try_into();
        let want = test.1;
        assert_eq!(got, want, "input: {:?}", test.0);
    }
}

#[test]
fn size_try_into_u128() {
    let tests = vec![
		(Size::new(1, Multiple::Byte), Ok(1)),

        (Size::new(1, Multiple::Kilobyte), Ok(1_000)),
        (Size::new(23, Multiple::Kilobyte), Ok(23_000)),
        (Size::new(65, Multiple::Megabyte), Ok(65_000_000)),
        (Size::new(123, Multiple::Gigabyte), Ok(123_000_000_000)),
        (Size::new(2, Multiple::Petabyte), Ok(2_000_000_000_000_000)),
        (Size::new(25, Multiple::Exabyte), Ok(25_000_000_000_000_000_000)),
        (Size::new(200, Multiple::Zettabyte), Ok(200_000_000_000_000_000_000_000)),
        (Size::new(2, Multiple::Yottabyte), Ok(2_000_000_000_000_000_000_000_000)),

        (Size::new(10, Multiple::Mebibyte), Ok(10_485_760)),
        (Size::new(1000, Multiple::Gigibyte), Ok(1_073_741_824_000)),
        (Size::new(1, Multiple::Pebibyte), Ok(1_125_899_906_842_624)),
        (Size::new(2, Multiple::Pebibyte), Ok(2_251_799_813_685_248)),

        (Size::new(3, Multiple::Exbibyte), Ok(3_458_764_513_820_540_928)),
        (Size::new(2, Multiple::Exbibyte), Ok(2_305_843_009_213_693_952)),
        (Size::new(1, Multiple::Yobibyte), Ok(1_208_925_819_614_629_174_706_176)),
    ];

    for test in tests {
        let got: Result<u128, ConversionError> = test.0.clone().try_into();
        let want = test.1;
        assert_eq!(got, want, "input: {:?}", test.0);
    }
}

#[test]
fn multiple_try_into_u32() {
    let tests = vec![
		(Multiple::Byte, Ok(1)),

		(Multiple::Kilobyte, Ok(1_000)),
		(Multiple::Megabyte, Ok(1_000_000)),
		(Multiple::Gigabyte, Ok(1_000_000_000)),

		(Multiple::Kibibyte, Ok(1024)),
		(Multiple::Mebibyte, Ok(1_048_576)),
		(Multiple::Gigibyte, Ok(1_073_741_824)),
    ];

    for test in tests {
        let got: Result<u32, ConversionError> = test.0.try_into();
        let want = test.1;
        assert_eq!(got, want, "input: {:?}", test.0);
    }
}

#[test]
fn multiple_try_into_u64() {
    let tests = vec![
		(Multiple::Byte, Ok(1)),

		(Multiple::Kilobyte, Ok(1_000)),
		(Multiple::Megabyte, Ok(1_000_000)),
		(Multiple::Gigabyte, Ok(1_000_000_000)),
		(Multiple::Terabyte, Ok(1_000_000_000_000)),
		(Multiple::Petabyte, Ok(1_000_000_000_000_000)),

		(Multiple::Kibibyte, Ok(1024)),
		(Multiple::Mebibyte, Ok(1_048_576)),
		(Multiple::Gigibyte, Ok(1_073_741_824)),
		(Multiple::Tebibyte, Ok(1_099_511_627_776)),
		(Multiple::Pebibyte, Ok(1_125_899_906_842_624)),
    ];

    for test in tests {
        let got: Result<u64, ConversionError> = test.0.try_into();
        let want = test.1;
        assert_eq!(got, want, "input: {:?}", test.0);
    }
}

#[test]
fn multiple_try_into_u128() {
    let tests = vec![
		(Multiple::Byte, Ok(1)),

		(Multiple::Kilobyte, Ok(1_000)),
		(Multiple::Megabyte, Ok(1_000_000)),
		(Multiple::Gigabyte, Ok(1_000_000_000)),
		(Multiple::Terabyte, Ok(1_000_000_000_000)),
		(Multiple::Petabyte, Ok(1_000_000_000_000_000)),
        (Multiple::Exabyte, Ok(1_000_000_000_000_000_000)),
        (Multiple::Zettabyte, Ok(1_000_000_000_000_000_000_000)),
        (Multiple::Yottabyte, Ok(1_000_000_000_000_000_000_000_000)),

		(Multiple::Kibibyte, Ok(1024)),
		(Multiple::Mebibyte, Ok(1_048_576)),
		(Multiple::Gigibyte, Ok(1_073_741_824)),
		(Multiple::Tebibyte, Ok(1_099_511_627_776)),
		(Multiple::Pebibyte, Ok(1_125_899_906_842_624)),
        (Multiple::Exbibyte, Ok(1_152_921_504_606_846_976)),
        (Multiple::Zebibyte, Ok(1_180_591_620_717_411_303_424)),
        (Multiple::Yobibyte, Ok(1_208_925_819_614_629_174_706_176)),
    ];

    for test in tests {
        let got: Result<u128, ConversionError> = test.0.try_into();
        let want = test.1;
        assert_eq!(got, want, "input: {:?}", test.0);
    }
}

#[test]
fn multiple_from_str() {
    let tests = vec![
		("B", Ok(Multiple::Byte)),

		("kB", Ok(Multiple::Kilobyte)),
		("MB", Ok(Multiple::Megabyte)),
		("GB", Ok(Multiple::Gigabyte)),
		("TB", Ok(Multiple::Terabyte)),
		("PB", Ok(Multiple::Petabyte)),
		("EB", Ok(Multiple::Exabyte)),
		("ZB", Ok(Multiple::Zettabyte)),
		("YB", Ok(Multiple::Yottabyte)),

		("KB", Ok(Multiple::Kibibyte)),
		("KiB", Ok(Multiple::Kibibyte)),
		("MiB", Ok(Multiple::Mebibyte)),
		("GiB", Ok(Multiple::Gigibyte)),
		("TiB", Ok(Multiple::Tebibyte)),
		("PiB", Ok(Multiple::Pebibyte)),
		("EiB", Ok(Multiple::Exbibyte)),
		("ZiB", Ok(Multiple::Zebibyte)),
		("YiB", Ok(Multiple::Yobibyte)),

        ("", Err(ParsingError::UnknownMultiple)),
        ("abc", Err(ParsingError::UnknownMultiple)),
    ];

    for test in tests {
        let got = Multiple::from_str(test.0);
        let want = test.1;
        assert_eq!(got, want, "input: {:?}", test.0);
    }
}

#[test]
fn multiple_to_string() {
    let tests = vec![
		(Multiple::Byte, "B"),

		(Multiple::Kilobyte, "kB"),
		(Multiple::Megabyte, "MB"),
		(Multiple::Gigabyte, "GB"),
		(Multiple::Terabyte, "TB"),
		(Multiple::Petabyte, "PB"),
		(Multiple::Exabyte, "EB"),
		(Multiple::Zettabyte, "ZB"),
		(Multiple::Yottabyte, "YB"),

		(Multiple::Kibibyte, "KiB"),
		(Multiple::Mebibyte, "MiB"),
		(Multiple::Gigibyte, "GiB"),
		(Multiple::Tebibyte, "TiB"),
		(Multiple::Pebibyte, "PiB"),
		(Multiple::Exbibyte, "EiB"),
		(Multiple::Zebibyte, "ZiB"),
		(Multiple::Yobibyte, "YiB"),
    ];

    for test in tests {
        let got = test.0.to_string();
        let want = test.1;
        assert_eq!(got, want, "input: {:?}", test.0);
    }
}
