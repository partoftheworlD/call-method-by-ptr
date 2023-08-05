use std::mem;

// Macro for cpp fans
macro_rules! reinterpret_cast {
    ($from:ty, $to:ty, $ptr:expr) => {
        unsafe { mem::transmute::<$from, $to>($ptr) }
    };
}

// "External" function
fn function_a(u: u32) -> u32 {
    println!("i'm external function {u:?}");
    0
}

// Structure that will record address of created function_a
struct Droppable {
    ptr: *const u64,
}

// Implement Droppable::new to get address of function_a and write it to Droppable.ptr
impl Droppable {
    fn new(function_a: *const u64) -> Droppable {
        println!("> Create {function_a:?}");
        Droppable { ptr: function_a }
    }
}

// Overload Drop::drop by our own variant of the destructor for Droppable trait
// to output a message about freeing a specific address
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {:?}", self.ptr);
    }
}

fn main() {
    // Function prototype definition
    type Proto = extern "C" fn(u32) -> u32;
    type Ptr = *const u64;

    // Getting function_a pointer while destructor is running
    let ptr_function_a: Droppable = Droppable::new(function_a as *const u64);

    // Cast pointer to a function prototype
    let function: Proto = reinterpret_cast!(Ptr, Proto, ptr_function_a.ptr);

    // Calling a function by pointer
    function(1337);
}
