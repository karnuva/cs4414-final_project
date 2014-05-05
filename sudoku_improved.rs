// ignore-pretty

// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(managed_boxes)]


extern crate time;
use std::io;
use std::io::stdio::StdReader;
use std::io::BufferedReader;
use std::os;
use std::num::Bitwise;
use std::num::{from_int,ToStrRadix};
use std::num;
use std::cast::transmute_mut;
use time::{Timespec, get_time, precise_time_ns};

static zero:u16 = ((1u16 << 10) -1);
static one:u16 = 1u16;
static two:u16 = (1u16 << 2);//0000000000000010 as u16;
static three:u16 = (1u16 << 3);//0000000000000100 as u16;
static four:u16 = (1u16 << 4);//0000000000001000 as u16;
static five:u16 = (1u16 << 5);
static six:u16 = (1u16 << 6);
static seven:u16 = (1u16 << 7);
static eight:u16 = (1u16 << 8);
static nine:u16 = (1u16 << 9);

//static numbers:[u16,..9] = [one,two,three,four,five,six,seven,eight,nine];
static numbers:[u16,..9] = [nine,eight,seven,six,five,four,three,two,one];


// Computes a single solution to a given 9x9 sudoku
//
// Call with "-" to read input sudoku from stdin
//
// The expected line-based format is:
//
// 9,9
// <row>,<column>,<color>
// ...
//
// Row and column are 0-based (i.e. <= 8) and color is 1-based (>=1,<=9).
// A color of 0 indicates an empty field.
//
// If called without arguments, sudoku solves a built-in example sudoku
//

// internal type of sudoku grids
type grid = Vec<Vec<u16> > ;

struct Sudoku {
    grid: grid
}

impl Sudoku {
    pub fn new(g: grid) -> Sudoku {
        return Sudoku { grid: g }
    }

    pub fn from_vec(vec: &[[u16, ..9], ..9]) -> Sudoku {
        let g = Vec::from_fn(9u, |i| {
            Vec::from_fn(9u, |j| { vec[i][j] })
        });
        return Sudoku::new(g)
    }

    pub fn equal(&self, other: &Sudoku) -> bool {
        for row in range(0u8, 9u8) {
            for col in range(0u8, 9u8) {
                if *self.grid.get(row as uint).get(col as uint) !=
                        *other.grid.get(row as uint).get(col as uint) {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn read(mut reader: BufferedReader<StdReader>) -> Sudoku {
        //print!("this is from read");
        //assert!(reader.read_line().unwrap() == ~"9,9"); /* assert first line is exactly "9,9" */
        let mut g = Vec::from_fn(10u, { |_i| vec!(0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16) });

        //for line in reader.lines() {
            //let line = line.unwrap();
            let line = reader.read_line().ok().unwrap_or(~"nothing");
            let comps: Vec<char> = line.trim().chars().collect();
            if comps.len() == 81u{
                    //print!("char: {}\n",comps.get(0));
                    //let mut g = Vec::from_fn(10u, { |_i| vec!(0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8) });
                    let mut i = 0;
                    for row in range(0u8, 9u8) {
                        for col in range(0u8, 9u8) {
                            
                            //print!("i : {}\n",i);
                            // if *comps.get(i)== '.'{
                            //     *g.get_mut(row as uint).get_mut(col as uint) = from_str::<uint>(std::str::from_char('0')).unwrap() as u8;
                            //     //from_str::<uint>("0").unwrap() as u8;
                            // }
                            // else{
                            
                             //*g.get_mut(row as uint).get_mut(col as uint) = from_str::<uint>(std::str::from_char(*comps.get(i))).unwrap() as u16;
                             let temp_num = from_str::<uint>(std::str::from_char(*comps.get(i))).unwrap();
                             match temp_num{
                             1 => {*g.get_mut(row as uint).get_mut(col as uint) = (1u16)}
                             2 => {*g.get_mut(row as uint).get_mut(col as uint) = (1u16 << 2)}
                             3 => {*g.get_mut(row as uint).get_mut(col as uint) = (1u16 << 3)}
                             4 => {*g.get_mut(row as uint).get_mut(col as uint) = (1u16 << 4)}
                             5 => {*g.get_mut(row as uint).get_mut(col as uint) = (1u16 << 5)}
                             6 => {*g.get_mut(row as uint).get_mut(col as uint) = (1u16 << 6)}
                             7 => {*g.get_mut(row as uint).get_mut(col as uint) = (1u16 << 7)}
                             8 => {*g.get_mut(row as uint).get_mut(col as uint) = (1u16 << 8)}
                             9 => {*g.get_mut(row as uint).get_mut(col as uint) = (1u16 << 9)}
                             _ => {*g.get_mut(row as uint).get_mut(col as uint) = (1u16 << 10)-1}
                             }
                            //print!("char: {}",*comps.get(i) as u8);
                                //str::from_char((*comps.get(i)).unwrap()) as u8;
                            //}
                            i += 1;
                        }
                    }
            }    
            else {
                fail!("Invalid sudoku file");
            }
      //  }
        return Sudoku::new(g)
    }

    pub fn write(&self, writer: &mut io::Writer) {
        for row in range(0u8, 9u8) {
            if(row == 3u8 || row == 6u8){
                    write!(writer, "------+-------+------\n");
            }
            if(*self.grid.get(row as uint).get(0) == 0){
                write!(writer, "_");
            }
            else{
            write!(writer, "{}", biToNum(*self.grid.get(row as uint).get(0)) );
            }
            for col in range(1u8, 9u8) {
                if(col == 3u8 || col == 6u8){
                    write!(writer, " |");
                }
                if(*self.grid.get(row as uint).get(col as uint) == 0){
                    write!(writer, " _");
                }
                else 
                {   write!(writer, " {}", biToNum(*self.grid
                                           .get(row as uint)
                                           .get(col as uint) as u16));
                }
            }
            write!(writer, "\n");
         }
    }
    
    // solve sudoku grid
    pub fn solve(&mut self) {
    let mut work: Vec<(u8, u8)> = Vec::new(); /* queue of uncolored fields */
	//loop through 
	   for row in range(0u8, 9u8) {
	    for col in range(0u8, 9u8) {
    	    	//get the data (row,col)
    		let color = *self.grid.get(row as uint).get(col as uint);
    		//if no input for (row,col)
    		if (color & zero) == zero {
    			//store the point that need to be worked on
    		    work.push((row, col));
        }    
      }
    }
   
    for i in range (0,work.len()){
     let (row, col) = *work.get(i);
     let mut n = *self.grid.get_mut(row as uint).get_mut(col as uint);
    	self.drop_number(&mut n,row,col);
    	*self.grid.get_mut(row as uint).get_mut(col as uint) = n;
      if(biToNum(n).len()==1){//if the possible value is reduced to one value, eliminate them from peers too.
          self.eliminate(&mut n,row,col);
        }
      //println!("{} at ({},{})",n.count_ones()-1,row+1,col+1);
    	//printBiToNum(*self.grid.get(row as uint).get(col as uint));
    }
    //
      // //at this point we got the possible outcome from the sudoku grid
        let mut stack: Vec<(grid,uint,uint)> = Vec::new(); 
        let (mut min,mut max,mut r, mut c) = self.findmin();//find the lowest possible cell that is more than 1
      // //push the first grid into the vector;
      // //let mut x = biToNum(*self.grid.get(r as uint).get(c as uint));
        let mut max = 20;
        let mut m = 0;
        stack.push((self.grid.clone(),2,0));
        while(max > 1){//keep going until we find the solution
          self.write(&mut io::stdout());
        let (mut min,mut p,mut r, mut c) = self.findmin();
        
        max = p;
        if(max == 1){
          break;
        }
        //println!("min = {} max = {}",min,max);
        let x = biToNum(*self.grid.get(r as uint).get(c as uint));
        if(m >= x.len()){
          loop{
          //println!("Backup");
          let (mut tempgrid, mut templen,mut tempindex) = stack.pop().unwrap();
         // println!("({},{})",tempindex,templen);
          if(tempindex<(templen-1)){
            self.grid = tempgrid;
            self.write(&mut io::stdout());
            m = tempindex+1;
            break;
            }
          }
        }
        let number = std::str::from_char(x[m] as char);
        println!("Try {} from {} at ({},{})",number,x,r+1,c+1);
        println!("current index = {}",m);
        if(!number.eq(&~"1")){
          let numu16 = from_int::<u16>(num::pow(2,from_str::<uint>(number).unwrap())).unwrap();
          stack.push((self.grid.clone(),x.len(),m));
          if(self.eliminate(&mut numu16.clone(),r as u8,c as u8)){
              m=0;
          }else{
              let (tempgrid,templen,tempindex) = stack.pop().unwrap();
              self.grid = tempgrid;
              m = tempindex+1; 
            println!("Found conflict try another number");
          }
        }else{
          stack.push((self.grid.clone(),x.len(),m));
          if(self.eliminate(&mut one.clone(),r as u8,c as u8)){
              m=0;
          }
          else{
              let (tempgrid,templen,tempindex) = stack.pop().unwrap();
              self.grid = tempgrid;
              m= tempindex+1;
              println!("Found conflict try another");
          }
        }
        //max = max-1;
        println!("=====================");
      }
      
    }
    fn findmin(&mut self)->(uint,uint,uint,uint){
      let mut max = 1;
      let mut min = 10;
      let (mut r,mut c) = (0,0);
      for row in range (0,9){
        for col in range(0,9){
          let x = biToNum(*self.grid.get(row as uint).get(col as uint));
          let p = x.len();
          if(p > max){
            max = p;
          }
          if(p ==2){
            min = p;
            r = row;
            c = col;
            break;
          }
          if(p > 1 && p < min){
            min = p;
            r = row;
            c = col;
          }
        }
    }
    return (min,max,r as uint,c as uint);
  } 
    //elminate that number from neighbors
    fn eliminate(&mut self, number: &mut u16, r:u8, c:u8)->bool{
      let mut work: Vec<(u8,u8)> = Vec::new();
      //eliminate itself 
      let mut temp = number.clone();
      *self.grid.get_mut(r as uint).get_mut(c as uint) = temp;
      //start to eliminate peers
      for change_val in range(0u8,9u8){
        if((change_val as int) != (c as int)){
          let x = biToNum(*self.grid.get(r as uint).get(change_val  as uint));
          if(*self.grid.get(r as uint).get(change_val as uint) & *number == *number){
            if(x.len()==1){ //found conflict return false
              return false;
            }else{//conflict not found keep going
            let mut temp = number.clone();
            temp = *self.grid.get(r as uint).get(change_val as uint) & !(temp);
            *self.grid.get_mut(r as uint).get_mut(change_val as uint) = temp;
              if(biToNum(temp).len()==1){
                work.push((r,change_val));
              }
            }
          }
        }
         if((change_val as int)!= (r as int)){
          let x = biToNum(*self.grid.get(change_val as uint).get(c as uint));
            if(*self.grid.get(change_val as uint).get(c as uint) & *number == *number){
              if(x.len()==1){ //found conflict return false
                return false;
              }else{//conflict not found keep going
                let mut temp2 = number.clone();
                temp2 = *self.grid.get(change_val as uint).get(c as uint) &! (temp2);
                *self.grid.get_mut(change_val as uint).get_mut(c as uint) = temp2;
                if(biToNum(temp2).len()==1){
                  work.push((change_val,c));
              }
            }
          }
        }
      }
      let row0 = (r / 3u8) * 3u8;
      let col0 = (c / 3u8) * 3u8;
      for alt_row in range(row0, row0 + 3u8) {
            for alt_col in range(col0, col0 + 3u8) {
              if(!(alt_row == r && alt_col == c)){
                let x = biToNum(*self.grid.get(alt_row as uint).get(alt_col as uint));
                if(*self.grid.get(alt_row as uint).get(alt_col as uint) & *number == *number){
                if(x.len()==1){ //found conflict return false
                  return false;
                }else{//conflict not found keep going
                  let mut temp2 = number.clone();
                  temp2 = *self.grid.get(alt_row as uint).get(alt_col as uint) &! (temp2);
                  *self.grid.get_mut(alt_row as uint).get_mut(alt_col as uint) = temp2;
                    if(biToNum(temp2).len()==1){
                      work.push((alt_row,alt_col));
                    }
                  }
                }
              }
            }
          }
      for i in range(0,work.len()){
        let (mut r1,mut r2) = *work.get(i);
        let mut temp_n = *self.grid.get_mut(r1 as uint).get_mut(r2 as uint);
        if !(self.eliminate(&mut temp_n,r1,r2)){
          return false;       
        }
      }
      //Check
let mut tempvec: Vec<~str> = Vec::new(); 
      //check row first
      for row1 in range(0u8,9u8){
        let x = biToNum(*self.grid.get(row1 as uint).get(c as uint));
        if(x.len() == 1){
          if(tempvec.contains(&x)){
            return false;
          }else{
            tempvec.push(x);
          }
        }
      }
      tempvec.clear();
      for column1 in range (0u8,9u8){
        let x = biToNum(*self.grid.get(r as uint).get(column1 as uint));
        if(x.len() == 1){
          if(tempvec.contains(&x)){
            return false;
          }else{
            tempvec.push(x);
          }
        }
      }
      tempvec.clear();
      let row0 = (r / 3u8) * 3u8;
      let col0 = (c / 3u8) * 3u8;
      for alt_row in range(row0, row0 + 3u8) {
            for alt_col in range(col0, col0 + 3u8) {
                let x = biToNum(*self.grid.get(alt_row as uint).get(alt_col as uint));
                if(x.len()==1){ //found conflict return false
                  if(tempvec.contains(&x)){
                    return false;
                    }else{
                      tempvec.push(x);
                    }
                  }
                }
              }
      return true;
    }

    fn check(&mut self,r: u8, c: u8)->bool{
    //check if there is a conflict to that row, column and unit
      let mut tempvec: Vec<~str> = Vec::new(); 
      //check row first
      for row1 in range(0u8,9u8){
        let x = biToNum(*self.grid.get(row1 as uint).get(c as uint));
        if(x.len() == 1){
          if(tempvec.contains(&x)){
            return false;
          }else{
            tempvec.push(x);
          }
        }
      }
      tempvec.clear();
      for column1 in range (0u8,9u8){
        let x = biToNum(*self.grid.get(r as uint).get(column1 as uint));
        if(x.len() == 1){
          if(tempvec.contains(&x)){
            return false;
          }else{
            tempvec.push(x);
          }
        }
      }
      tempvec.clear();
      let row0 = (r / 3u8) * 3u8;
      let col0 = (c / 3u8) * 3u8;
      for alt_row in range(row0, row0 + 3u8) {
            for alt_col in range(col0, col0 + 3u8) {
                let x = biToNum(*self.grid.get(alt_row as uint).get(alt_col as uint));
                if(x.len()==1){ //found conflict return false
                  if(tempvec.contains(&x)){
                    return false;
                    }else{
                      tempvec.push(x);
                    }
                  }
                }
              }
      return true;
    }

    // find colors available in neighbourhood of (row, col)
    fn drop_number(&mut self, avail: &mut u16, row: u8, col: u8){
      	//loop through the block
        for idx in range(0u8, 9u8) {
          	//vertical
          	let mut cur_neighbor = *self.grid.get(idx as uint).get(col as uint);
          	
          	//if neighbourhood only has 1 number
            if (checkNeighbor(cur_neighbor) != 0){
             *avail = (*avail & ! (cur_neighbor));		
           }

  		//horizontal		
            cur_neighbor = *self.grid.get(row as uint).get(idx as uint);
          	//if neighbourhood only has 1 number
            if (checkNeighbor(cur_neighbor) != 0){
             *avail = (*avail & !cur_neighbor);
           }
         }

          // check same block fields
          let row0 = (row / 3u8) * 3u8;
          let col0 = (col / 3u8) * 3u8;
          for alt_row in range(row0, row0 + 3u8) {
            for alt_col in range(col0, col0 + 3u8) {
              let cur_neighbor = *self.grid.get(alt_row as uint).get(alt_col as uint);
          	//if neighbourhood only has 1 number
            if (checkNeighbor(cur_neighbor) != 0){
             *avail = (*avail & !cur_neighbor);
           }              
         }
       }
     }

}

pub fn checkNeighbor(n: u16) -> uint{

match n{
     one => {return 1;}
     two => {return 2;}
     three => {return 3;}
     four => {return 4;}
     five => {return 5;}
     six => {return 6;}
     seven => {return 7;}
     eight => {return 8;}
     nine => {return 9;}
     _ => {return 0;}
     }
}

pub fn biToNum(n: u16) -> ~str{

let mut n=n;
let mut k = 0;
let mut res = 0;
let mut counter = 0 as uint;

for i in range(0u,9u){
	if(n & *(numbers.get(i).unwrap())) == *(numbers.get(i).unwrap()){
		n -= *(numbers.get(i).unwrap());
		//print!("{}",i+1);	
		//println!("to be add {}", ((9-i)*(num::pow(10,counter)) ) );	
		res += (9-i)*((num::pow(10,counter)) as uint);
		counter +=1;
	}
	
}
return res.to_str();
}    

// pub fn printBiToNum(n: u16){
// let mut n=n;
// let mut k = 0;
// let mut res = 0;
// let mut counter = 0 as uint;

// for i in range(0u,9u){
// 	if(n & *(numbers.get(i).unwrap())) == *(numbers.get(i).unwrap()){
// 		n -= *(numbers.get(i).unwrap());	
// 		res += (9-i)*((num::pow(10,counter)) as uint);
// 		counter +=1;
// 	}
	
// }

// //println!("{}",res);
// }

static DEFAULT_SUDOKU: [[u16, ..9], ..9] = [
         /* 0    1    2    3    4    5    6    7    8    */
  /* 0 */  [0u16, 4u16, 0u16, 6u16, 0u16, 0u16, 0u16, 3u16, 2u16],
  /* 1 */  [0u16, 0u16, 8u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16],
  /* 2 */  [7u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16],
  /* 3 */  [0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16],
  /* 4 */  [0u16, 5u16, 0u16, 0u16, 0u16, 3u16, 6u16, 0u16, 0u16],
  /* 5 */  [6u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16],
  /* 6 */  [0u16, 9u16, 5u16, 0u16, 0u16, 6u16, 0u16, 7u16, 0u16],
  /* 7 */  [0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 6u16, 0u16],
  /* 8 */  [4u16, 0u16, 0u16, 0u16, 0u16, 7u16, 2u16, 0u16, 3u16]
];


fn main() {
	//printBiToNum(one|two|three|six|nine);
  println!("Please input a sudoku puzzle with 0 indicating an empty space :");

    let args = os::args();
    let use_default = args.len() == 1u;
    let mut sudoku = if use_default {
        Sudoku::from_vec(&DEFAULT_SUDOKU)
    } else {
        Sudoku::read(io::stdin())
    };
        println!("-------------------------------------------------");
    print!("Original Sudoku : \n");
    sudoku.write(&mut io::stdout());
    //println!("{}",printBiToNum(two));
    let tv1 = get_time();

    for i in range(0,20){
      sudoku.solve();
    }
    let tv2 = get_time();

    println!("-------------------------------------------------");
    let diff_nsec = tv2.nsec - tv1.nsec;
    print!("Result : \n");
    sudoku.write(&mut io::stdout());
    
    println!("Total time : {} nanosecs",num::abs(diff_nsec/20));
}
