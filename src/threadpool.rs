use std::thread;
use std::vec;

pub struct Pool {
    pool_size: usize
}

impl Pool{
    pub fn new(size: usize) -> Self {
        Self { pool_size: size }    
    }

    pub fn reduce_mod<I,T,U>(&self,f:fn(T) -> U, iter: I,mod_n: U) -> U
    where 
        I:Iterator<Item = T>,
        T: std::marker::Send + 'static + std::marker::Sync + Clone,
        U: std::marker::Send + 'static + std::ops::Rem<Output = U> + std::ops::AddAssign<<U as std::ops::Rem>::Output> + Default + Copy
    {
        let mut counter:usize = 0;
        let mut list_of_tasks:Vec<Vec<T>> = vec![];
        for _i in 0..self.pool_size{
            list_of_tasks.push(vec![]);
        }

        for element in iter{
            list_of_tasks[counter].push(element);
            counter = (counter + 1) % self.pool_size;
        }
        let mut handles = vec![];
        for i in 0..self.pool_size {
            let share_of_elements = list_of_tasks[i].clone();
            let handle = thread::spawn(move || {
                    let mut local_counter:U = U::default() as U;
                    for element in share_of_elements{
                        local_counter += f(element) % mod_n.clone();
                    }
                    local_counter
                });
            handles.push(handle);
        }

        let mut out:U = U::default();
        for handle in handles {
            out += handle.join().unwrap() % mod_n;
        }
        out % mod_n
    }
}
