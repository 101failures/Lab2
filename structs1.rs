struct ColorRegularStruct {
    red: u8,
    green: u8,
    blue: u8,
}

struct ColorTupleStruct(u8, u8, u8);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // Testing Regular Struct
    let green_struct = ColorRegularStruct { red: 0, green: 255, blue: 0 };
    println!("Regular Struct: red={}, green={}, blue={}", green_struct.red, green_struct.green, green_struct.blue);

    // Testing Tuple Struct
    let green_tuple = ColorTupleStruct(0, 255, 0);
    println!("Tuple Struct: ({}, {}, {})", green_tuple.0, green_tuple.1, green_tuple.2);

    // Testing Unit Struct
    let unit = UnitStruct;
    println!("{:?}s are fun!", unit);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        let green = ColorRegularStruct { red: 0, green: 255, blue: 0 };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        let unit_struct = UnitStruct;
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
