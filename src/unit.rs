#[derive(PartialEq, Eq)]
enum Power {
    Zero,

}

#[derive(PartialEq, Eq)]
struct UnitDimensions {
    time: Power,
    length: Power,
    mass: Power,
    current: Power,
    temperature: Power,
    quantity: Power,
    luminosity: Power,
}


struct Unit<T, const UNITS: UnitDimensions>(T);
