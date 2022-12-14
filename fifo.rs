use libc::*; 
const FIFO_QUEUE_SIZE:usize = 128;
#[derive(Clone, Copy)]
pub struct fifo_event_t{
    pub event : Option<i32>,
    pub data : * mut i32, 
}
#[derive(Clone)]
pub struct fifo_t{
    pub queue: [fifo_event_t; FIFO_QUEUE_SIZE], 
    pub head : size_t,
    pub tail : size_t, 
    pub size : size_t, 
}
impl fifo_event_t{
        pub fn new() -> Self{
            Self{
                event: None,
                data : & mut 0,
            }
        }
}

impl fifo_t{

    pub fn fifo_init() -> Self{
        let temp = fifo_event_t::new();
        Self{
            queue: [temp; FIFO_QUEUE_SIZE],
            head : 0,
            tail : 0, 
            size : 0,
        }
    }

    pub fn fifo_full(&self)-> bool{
        let result = self.size >= FIFO_QUEUE_SIZE; 
        result
    }

    pub fn fifo_empty(&self) -> bool{
        let result = self.size ==0; 
        result
    }

    pub fn fifo_pull(& mut self)-> fifo_event_t{
        if self.size == 0{
            return fifo_event_t{event : None, data: & mut 0};
        }
        let head = self.head;
        let event = self.queue[head]; 
        if head == FIFO_QUEUE_SIZE -1{
            self.head = 0; 
        }else{
            self.head += 1; 
        }
        self.size -=1;
        return event
    }

    pub fn fifo_peek(&self) -> fifo_event_t{
        if self.size == 0{
            return fifo_event_t::new();
        }
        let event = self.queue[self.head];
        event
    }

    pub fn fifo_push(&mut self, mut value: i32) -> bool{
        if self.size >= FIFO_QUEUE_SIZE{
            return false
        }
        let tail = self.tail;
        self.queue[tail] = fifo_event_t{event:Some(value), data: & mut value};
        if tail == FIFO_QUEUE_SIZE-1{
            self.tail = 0;
        }else{
            self.tail += 1;
        }
        self.size += 1;
        true
    }
}