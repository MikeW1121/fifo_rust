mod fifo; 
use fifo::fifo_t;
use fifo::fifo_event_t;
impl fifo_t{
    fn print_fifo_info(&self){
        println!("Length: {}\nEmpty: {}\nFull :{}", self.size, self.fifo_empty(), self.fifo_full());
    }
}

fn print(fifo:fifo_event_t){
    match fifo.event{
        None => println!("Pulled from empty fifo!"),
        Some(value) =>println!("Event Type: {}\nData Address: {:p}", value, fifo.data)
    }
}


fn test_basics(){
    let mut fifo = fifo_t::fifo_init(); 
    let mut out:fifo_event_t;

    println!("> Pull from empty");
    fifo.print_fifo_info();
    out = fifo.fifo_pull(); 
    print(out); 
    fifo.print_fifo_info(); 

    println!("> Peek from empty");
    out = fifo.fifo_peek(); 
    print(out); 
    fifo.print_fifo_info(); 

    println!("> Push event 1"); 
    fifo.fifo_push(1); 
    fifo.print_fifo_info();

    println!("> Peek at event 1");
    out = fifo.fifo_peek(); 
    print(out); 
    fifo.print_fifo_info(); 

    println!("> Push event 2");
    fifo.fifo_push(2); 
    fifo.print_fifo_info(); 

    println!("> Push event 3"); 
    fifo.fifo_push(3); 
    fifo.print_fifo_info(); 

    println!("> Pull event 1"); 
    out = fifo.fifo_pull(); 
    print(out); 
    fifo.print_fifo_info(); 

    println!("> Push event 4"); 
    fifo.fifo_push(4); 
    fifo.print_fifo_info(); 

    println!("> Pull event 2"); 
    out = fifo.fifo_pull(); 
    print(out); 
    fifo.print_fifo_info(); 

    println!("> Pull event 3"); 
    out = fifo.fifo_pull();
    print(out); 
    fifo.print_fifo_info(); 

    println!("> Pull event 4"); 
    out = fifo.fifo_pull(); 
    print(out); 
    fifo.print_fifo_info(); 

    println!("> Pull from empty"); 
    out = fifo.fifo_pull(); 
    print(out); 
    fifo.print_fifo_info(); 


}
fn main(){
   test_basics();
}
