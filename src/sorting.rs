use rand::Rng;
use std::{thread, time::Duration};


pub struct SortVisualizer {
    pub numbers: Vec<usize>,
    pub is_sorting: bool,
    pub rx: Option<std::sync::mpsc::Receiver<Vec<usize>>>,
    pub show_scanning: bool,
}

impl Default for SortVisualizer {
    fn default() -> Self {
        Self {
            numbers: (0..50).map(|_| rand::thread_rng().gen_range(10..400)).collect(),
            is_sorting: false,
            rx: None,
            show_scanning: false, 
        }
    }
}

impl SortVisualizer {
    pub fn bubble_sort(&mut self, ctx: &egui::Context) {
        self.is_sorting = true;
        let len = self.numbers.len();
        
        let (tx, rx) = std::sync::mpsc::channel();
        
        let mut numbers_clone = self.numbers.clone();
        let ctx_clone = ctx.clone();
        let show_scanning = self.show_scanning;
        
        thread::spawn(move || {
            for i in 0..len {
                for j in 0..len - i - 1 {
                    if show_scanning {
                        tx.send(numbers_clone.clone()).unwrap();
                        thread::sleep(Duration::from_millis(30));
                        ctx_clone.request_repaint();
                    }
                    
                    if numbers_clone[j] > numbers_clone[j + 1] {
                        numbers_clone.swap(j, j + 1);
                        
                        tx.send(numbers_clone.clone()).unwrap();
                        thread::sleep(Duration::from_millis(30));
                        ctx_clone.request_repaint();
                    }
                }
            }
        });
        
        self.rx = Some(rx);
    }

    pub fn insertion_sort(&mut self, ctx: &egui::Context) {
        self.is_sorting = true;
        let len = self.numbers.len();
        let (tx, rx) = std::sync::mpsc::channel();
        
        let mut numbers_clone = self.numbers.clone();
        let ctx_clone = ctx.clone();
        let show_scanning = self.show_scanning;

        thread::spawn(move || {
            for i in 1..len {
                let current_value = numbers_clone[i];
                let mut j = i;

                while j > 0 && numbers_clone[j-1] > current_value {
                    if show_scanning {
                        tx.send(numbers_clone.clone()).unwrap();
                        thread::sleep(Duration::from_millis(30));
                        ctx_clone.request_repaint();
                    }
                    
                    numbers_clone[j] = numbers_clone[j-1];
                    j -= 1;

                    tx.send(numbers_clone.clone()).unwrap();
                    thread::sleep(Duration::from_millis(30));
                    ctx_clone.request_repaint();
                }
                
                numbers_clone[j] = current_value;
                tx.send(numbers_clone.clone()).unwrap();
                thread::sleep(Duration::from_millis(30));
                ctx_clone.request_repaint();
            }
        });

        self.rx = Some(rx);
    }

    pub fn selection_sort(&mut self, ctx: &egui::Context) {
        self.is_sorting = true;
        let len = self.numbers.len();
        let (tx, rx) = std::sync::mpsc::channel();
        
        let mut numbers_clone = self.numbers.clone();
        let ctx_clone = ctx.clone();
        let show_scanning = self.show_scanning;
        
        thread::spawn(move || {
            for i in 0..len-1 {
                let mut min_index = i;
                
                for j in i+1..len {
                    if show_scanning {
                        tx.send(numbers_clone.clone()).unwrap();
                        thread::sleep(Duration::from_millis(30));
                        ctx_clone.request_repaint();
                    }
                    
                    if numbers_clone[j] < numbers_clone[min_index] {
                        min_index = j;
                    }
                }
                
                if min_index != i {
                    numbers_clone.swap(i, min_index);
                    
                    tx.send(numbers_clone.clone()).unwrap();
                    thread::sleep(Duration::from_millis(30));
                    ctx_clone.request_repaint();
                }
            }
        });
        
        self.rx = Some(rx);
    }




    pub fn quick_sort(&mut self, ctx: &egui::Context) {
        self.is_sorting = true;
        let len = self.numbers.len();
        
        if len <= 1 {
            return;
        }
        
        let (tx, rx) = std::sync::mpsc::channel();
        
        let mut numbers_clone = self.numbers.clone();
        let ctx_clone = ctx.clone();
        let show_scanning = self.show_scanning;
        
        thread::spawn(move || {
            fn quick_sort_recursive(
                numbers: &mut Vec<usize>, 
                low: usize, 
                high: usize, 
                tx: &std::sync::mpsc::Sender<Vec<usize>>, 
                ctx: &egui::Context,
                show_scanning: bool
            ) {
                if low >= high {
                    return;
                }
                
                let pivot_index = partition(numbers, low, high, tx, ctx, show_scanning);
                
                if pivot_index > 0 {
                    quick_sort_recursive(numbers, low, pivot_index - 1, tx, ctx, show_scanning);
                }
                quick_sort_recursive(numbers, pivot_index + 1, high, tx, ctx, show_scanning);
            }
            
            fn partition(
                numbers: &mut Vec<usize>, 
                low: usize, 
                high: usize, 
                tx: &std::sync::mpsc::Sender<Vec<usize>>, 
                ctx: &egui::Context,
                show_scanning: bool
            ) -> usize {
                let pivot = numbers[high];
                let mut i = low;
                
                for j in low..high {
                    if show_scanning {
                        tx.send(numbers.clone()).unwrap();
                        thread::sleep(Duration::from_millis(30));
                        ctx.request_repaint();
                    }
                    
                    if numbers[j] <= pivot {
                        numbers.swap(i, j);
                        
                        tx.send(numbers.clone()).unwrap();
                        thread::sleep(Duration::from_millis(30));
                        ctx.request_repaint();
                        
                        i += 1;
                    }
                }
                
                numbers.swap(i, high);
                tx.send(numbers.clone()).unwrap();
                thread::sleep(Duration::from_millis(30));
                ctx.request_repaint();
                
                i
            }
            
            if len > 0 {
                quick_sort_recursive(&mut numbers_clone, 0, len - 1, &tx, &ctx_clone, show_scanning);
            }
            
            tx.send(numbers_clone).unwrap();
            ctx_clone.request_repaint();
        });
        
        self.rx = Some(rx);
    }


    pub fn merge_sort(&mut self, ctx: &egui::Context) {
        self.is_sorting = true;
        let len = self.numbers.len();
        
        if len <= 1 {
            return;
        }
        
        let (tx, rx) = std::sync::mpsc::channel();
        
        let mut numbers_clone = self.numbers.clone();
        let ctx_clone = ctx.clone();
        let show_scanning = self.show_scanning;
    
        thread::spawn(move || {
            fn merge_sort_recursive(
                numbers: &mut Vec<usize>,
                low: usize,
                high: usize,
                tx: &std::sync::mpsc::Sender<Vec<usize>>,
                ctx: &egui::Context,
                show_scanning: bool
            ) {
                if low >= high {
                    return;
                }
                
                let mid = low + (high - low) / 2;
                
                merge_sort_recursive(numbers, low, mid, tx, ctx, show_scanning);
                merge_sort_recursive(numbers, mid + 1, high, tx, ctx, show_scanning);
                
                merge(numbers, low, mid, high, tx, ctx, show_scanning);
            }
            
            fn merge(
                numbers: &mut Vec<usize>,
                low: usize,
                mid: usize,
                high: usize,
                tx: &std::sync::mpsc::Sender<Vec<usize>>,
                ctx: &egui::Context,
                show_scanning: bool
            ) {
                let left_size = mid - low + 1;
                let right_size = high - mid;
                
                
                let mut left = Vec::with_capacity(left_size);
                let mut right = Vec::with_capacity(right_size);
                
                
                for i in 0..left_size {
                    left.push(numbers[low + i]);
                }
                
                for i in 0..right_size {
                    right.push(numbers[mid + 1 + i]);
                }
                
                let mut i = 0; 
                let mut j = 0; 
                let mut k = low; 
                
                while i < left_size && j < right_size {
                    if show_scanning {
                        tx.send(numbers.clone()).unwrap();
                        thread::sleep(Duration::from_millis(30));
                        ctx.request_repaint();
                    }
                    
                    if left[i] <= right[j] {
                        numbers[k] = left[i];
                        i += 1;
                    } else {
                        numbers[k] = right[j];
                        j += 1;
                    }
                    
                    tx.send(numbers.clone()).unwrap();
                    thread::sleep(Duration::from_millis(30));
                    ctx.request_repaint();
                    
                    k += 1;
                }
                
                while i < left_size {
                    numbers[k] = left[i];
                    
                    tx.send(numbers.clone()).unwrap();
                    thread::sleep(Duration::from_millis(30));
                    ctx.request_repaint();
                    
                    i += 1;
                    k += 1;
                }
                
                while j < right_size {
                    numbers[k] = right[j];
                    
                    tx.send(numbers.clone()).unwrap();
                    thread::sleep(Duration::from_millis(30));
                    ctx.request_repaint();
                    
                    j += 1;
                    k += 1;
                }
            }
            
            if len > 0 {
                merge_sort_recursive(&mut numbers_clone, 0, len - 1, &tx, &ctx_clone, show_scanning);
            }
            
            
            tx.send(numbers_clone).unwrap();
            ctx_clone.request_repaint();
        });
        
        self.rx = Some(rx);
    }

    






    
    pub fn reset_array(&mut self) {
        self.numbers = (0..50).map(|_| rand::thread_rng().gen_range(10..400)).collect();
        self.is_sorting = false;
    }
}