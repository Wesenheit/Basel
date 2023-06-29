
#[macro_export]
macro_rules! time_it{
    ($order:stmt)=> {
        let timer = std::time::Instant::now();
        $order
        println!("elapsed time: {:?}",timer.elapsed());
    };
}