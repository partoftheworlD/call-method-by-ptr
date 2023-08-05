use std::mem;

macro_rules! reinterpret_cast {
    ($from:ty, $to:ty, $ptr:expr) => {
        unsafe { mem::transmute::<$from, $to>($ptr) }
    };
}

fn function_a(u: u32) -> u32 {
    println!("i'm external function {u:?}");
    0
}

struct Droppable {
    ptr: *const u64,
}

impl Droppable {
    fn new(function_a: *const u64) -> Droppable {
        println!("> Create {function_a:?}");
        Droppable { ptr: function_a }
    }
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {:?}", self.ptr);
    }
}

fn main() {
    // Function prototype definition
    type Proto = extern "C" fn(u32) -> u32;
    type Ptr = *const u64;
    // Getting a pointer to a function
    let ptr_function_a = Droppable::new(function_a as *const u64);
    // Cast pointer to a function prototype
    let function = reinterpret_cast!(Ptr, Proto, ptr_function_a.ptr);
    // Calling a function by pointer
    function(1337);
}
