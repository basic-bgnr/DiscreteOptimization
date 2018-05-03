use std::fs::File;
use std::io::Read;

use std::env;
use std::cmp;


fn main(){
     //let file_location = env::args().nth(1).expect("no input");
    
     //let mut string_buffer = String::new();
     //
     //let mut file = File::open(file_location).expect("file error");
     //file.read_to_string(&mut string_buffer);

     //let problem = knapsack_problem(&string_buffer);
     
     //println!("{:?}", problem);
     //let solution = knapsack_solution(problem);

     //println!("{:?}", solution);
     println!("{:?}", dfs_stack(env::args().nth(1).expect("no input").parse::<i32>().expect("parse error")));
}

#[derive(Debug)]
struct KnapsackProblem {
    capacity: i32,
    items: Vec<i32>,
    value: Vec<i32>,
}

#[derive(Debug)]
struct KnapsackSolution {
    is_optimum: bool,
    value : i32,
}
#[derive(Debug)]
struct Vertex{
    id: i32,
    depth: i32,
    path: Vec<i32>,
}

    
fn knapsack_problem(string_buffer: &str)-> KnapsackProblem { 
    let mut my_knapsack_problem = KnapsackProblem{
                                                  capacity: 0, 
                                                  items: Vec::new(),
                                                  value: Vec::new(),
                                                 };
    
    let mut line_iterator = string_buffer.split("\n"); 
    
    if let Some(first_line) = line_iterator.next(){
        let mut first_line_buffer = first_line.split(" ");
        if let Some(no) = first_line_buffer.next() {
            if let Some(knapsack_capacity) = first_line_buffer.next(){
                my_knapsack_problem.capacity = knapsack_capacity.parse::<i32>().unwrap();
                
                for rem_line in line_iterator {
                    let mut temp_line = rem_line.split(" ");
                    if let Some(x) = temp_line.next(){
                        if let Some(v) = temp_line.next(){
                            my_knapsack_problem.items.push(x.parse::<i32>().unwrap());
                            my_knapsack_problem.value.push(v.parse::<i32>().unwrap());
                        };
                    }
                }
            }
        }
    }    
    my_knapsack_problem
}

fn knapsack_solution(my_knapsack_problem: KnapsackProblem) -> KnapsackSolution {
    
    let mut search_space = Vec::new();
    DFS(my_knapsack_problem.items.len() as i32, 0, &mut Vec::new(), &mut search_space);
    //println!("{:?}", search_space)
    let mut max_value = 0_i32; 
    for sol in search_space {
        let mut cum_weight = 0_i32;
        let mut cum_value  = 0_i32;
        for ((is_included, value), weight) in sol.iter().zip(my_knapsack_problem.value.iter())
                                                        .zip(my_knapsack_problem.items.iter()) {
            if *is_included == 1 {
                if cum_weight + weight > my_knapsack_problem.capacity {
                    break;
                }
                cum_weight += weight;
                cum_value  += value;
            }
        }
        max_value = cmp::max(max_value, cum_value); 

    }
    KnapsackSolution{ is_optimum : true, value: max_value}
}

fn DFS(depth: i32, current_level: i32, path: &mut Vec<i32>, search_space: &mut Vec<Vec<i32>>) {
    if depth == current_level{
        //println!("{:?}", path);
        search_space.push(path.clone());
        path.pop();
        return;
    }
    for i in 0..2 {
        path.push(i);
        DFS(depth, current_level+1, path, search_space);
    }
    path.pop();
}

fn dfs_stack(max_depth: i32) -> Vec<Vertex> {
   let mut closed_stack = Vec::new();
   let mut open_stack   = Vec::new();

   open_stack.push( Vertex{ id: 0, depth:0, path: [0].to_vec() } );
   open_stack.push( Vertex{ id: 1, depth:0, path: [1].to_vec() } );

   loop {
       if open_stack.len() == 0 {
           break;
       }
       let mut last_item = open_stack.pop().unwrap();

       for &i in [0, 1].iter() {
           let depth = last_item.depth + 1;
           if depth < max_depth {
               let mut pth = last_item.path.clone();
               pth.push(i);
               open_stack.push(Vertex{id:i, depth: depth, path: pth});
           }
       }
       closed_stack.push(last_item);
   }
   closed_stack
}




