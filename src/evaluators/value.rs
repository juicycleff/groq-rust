use std::any::{TypeId, Any};
use std::fmt;

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

impl fmt::Display for GroqValueName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

fn get_type() -> GroqValueName {

}

#[derive(Debug)]
struct StaticValue<P: Any> {
    data: P
}

impl StaticValue<P> {

    pub fn new(new_data: P) -> StaticValue<P> {
        StaticValue {
            data: new_data
        }
    }

    pub fn get(&self) -> P {
        self.data
    }

    pub fn get_boolean(&self) -> bool {
        // TODO: Run type check
        false
    }
}

// Implement iterator for static value
impl Iterator for StaticValue<P> {
    type Item = P;

    fn next(&mut self) -> Option<Self::Item> {

    }
}

/// StreamValue implementation
#[derive(Debug)]
struct StreamValue {
    data: Vec<dyn Any>,
    is_done: bool
}

impl StreamValue {
    pub fn new() -> StreamValue {
        StreamValue {
            data: Vec::new(),
            is_done: false,
        }
    }

    fn get_type() -> &'static str {
        "array"
    }

    fn get() -> Vec<dyn Any> {
        let result = Vec::new();

        /* for await (let value of this) {
            result.push(await value.get())
        } */

        for value in result {
            // TODO: Add item to the result
        }

        result
    }
}
