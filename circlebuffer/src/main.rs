fn p_logo() {
    println!("(^,^)\n/)_)/\n **");
}

struct CBuffer {
    buffer: [u8; 5],
    head: usize,
    tail: usize,
}

impl CBuffer {
    // buffer init
    fn new() -> Self {
        Self {
            buffer: [0; 5],
            head: 0,
            tail: 0,
        }
    }

    pub fn push(&mut self, data: u8) -> i32 {
        let mut next: usize = self.head + 1;
        if next >= self.buffer.len() {
            next = 0;
        }
        if next == self.tail {
            return -1;
        }
        self.buffer[self.head] = data;
        println!("=== Pushed {} ===", data);
        self.head = next;
        return 0;
    }

    pub fn pop(&mut self, data: &mut u8) -> i32 {
        let mut next: usize = self.tail + 1;
        if self.head == self.tail {
            return -1; // Buffer is empty
        }
        if next >= self.buffer.len() {
            next = 0;
        }

        *data = self.buffer[self.tail];
        println!("Popped data: {}", data);
        self.tail = next;
        return 0;
    }
}

fn main() {
    p_logo();
    println!("A circular buffer in rust by alba (cam)");

    let in_data: u8 = 1;
    let mut out_data: u8 = 1;

    let mut buffer = CBuffer::new();
    buffer.push(in_data);
    buffer.push(in_data);
    buffer.push(in_data);
    buffer.push(in_data);
    buffer.pop(&mut out_data);

    println!("FINAL BUFFER!");
    for element in buffer.buffer.iter() {
        println!("Element: {}", element);
    }
}
