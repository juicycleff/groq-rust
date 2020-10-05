#[derive(Debug)]
enum GroqValueName {
    Null,
    Boolean,
    Number,
    Array,
    Object,
    Range,
    Pair,
    Path,
}

fn get_type() -> GroqValueName {

}

#[derive(Debug)]
struct StaticValue<P> {
    data: P
}

impl StaticValue<P> {

    fn new(&self, new_data: P) {
        self.data = new_data
    }

    fn get_data(&self) -> P {
        self.data
    }

    fn get_boolean() -> bool {
        // TODO run type check on data
        true
    }
}

#[derive(Debug)]
struct StreamValue {
    data: Vec<i8>
}
