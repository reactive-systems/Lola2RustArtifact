#![allow(unused_parens)]
//extern crate prusti_contracts;

use crossbeam_channel as channel;
use std::collections::HashMap;

static mut ITERATION: usize = 0;

//#[trusted]
fn read_src() -> i64 {
    unsafe { super::VAL1[ITERATION] }
}

//#[trusted]
fn read_dst() -> i64 {
    unsafe { super::VAL2[ITERATION] }
}

//#[trusted]
fn read_fin() -> bool {
    unsafe { super::VAL3[ITERATION] }
}

//#[trusted]
fn read_push() -> bool {
    unsafe { super::VAL4[ITERATION] }
}

//#[trusted]
fn read_syn() -> bool {
    unsafe { super::VAL5[ITERATION] }
}

//#[trusted]
fn read_length() -> i64 {
    unsafe {
        let r = super::VAL6[ITERATION];
        ITERATION += 1;
        r
    }
}

//#[trusted]
fn exists_input() -> bool {
    unsafe { ITERATION < super::LENGTH }
}

#[derive(Copy, Clone)]
pub struct Memory {
    pub count_0: i64,
    pub rec_var_0: i64,
    pub workload_0: i64,
    pub opened_0: i64,
    pub closed_0: i64,
}

impl Memory {
    //#[pure]
    //#[requires="idx >= 0 && idx < 1"]
    //#[ensures="idx == 0 ==> result == self.count_0"]
    pub fn get_count(&self, idx: usize) -> i64 {
        if idx == 0 {
            self.count_0
        } else {
            unreachable!()
        }
    }
    //#[ensures="self.get_count(0) == v"]
    //#[ensures="self.rec_var_0 == old(self.rec_var_0)"]
    //#[ensures="self.workload_0 == old(self.workload_0)"]
    //#[ensures="self.opened_0 == old(self.opened_0)"]
    //#[ensures="self.closed_0 == old(self.closed_0)"]
    pub fn store_count(&mut self, v: i64) {
        self.count_0 = v;
    }

    //#[pure]
    //#[requires="idx >= 0 && idx < 1"]
    //#[ensures="idx == 0 ==> result == self.rec_var_0"]
    pub fn get_rec_var(&self, idx: usize) -> i64 {
        if idx == 0 {
            self.rec_var_0
        } else {
            unreachable!()
        }
    }
    //#[ensures="self.get_rec_var(0) == v"]
    //#[ensures="self.count_0 == old(self.count_0)"]
    //#[ensures="self.workload_0 == old(self.workload_0)"]
    //#[ensures="self.opened_0 == old(self.opened_0)"]
    //#[ensures="self.closed_0 == old(self.closed_0)"]
    pub fn store_rec_var(&mut self, v: i64) {
        self.rec_var_0 = v;
    }

    //#[pure]
    //#[requires="idx >= 0 && idx < 1"]
    //#[ensures="idx == 0 ==> result == self.workload_0"]
    pub fn get_workload(&self, idx: usize) -> i64 {
        if idx == 0 {
            self.workload_0
        } else {
            unreachable!()
        }
    }
    //#[ensures="self.get_workload(0) == v"]
    //#[ensures="self.count_0 == old(self.count_0)"]
    //#[ensures="self.rec_var_0 == old(self.rec_var_0)"]
    //#[ensures="self.opened_0 == old(self.opened_0)"]
    //#[ensures="self.closed_0 == old(self.closed_0)"]
    pub fn store_workload(&mut self, v: i64) {
        self.workload_0 = v;
    }

    //#[pure]
    //#[requires="idx >= 0 && idx < 1"]
    //#[ensures="idx == 0 ==> result == self.opened_0"]
    pub fn get_opened(&self, idx: usize) -> i64 {
        if idx == 0 {
            self.opened_0
        } else {
            unreachable!()
        }
    }
    //#[ensures="self.get_opened(0) == v"]
    //#[ensures="self.count_0 == old(self.count_0)"]
    //#[ensures="self.rec_var_0 == old(self.rec_var_0)"]
    //#[ensures="self.workload_0 == old(self.workload_0)"]
    //#[ensures="self.closed_0 == old(self.closed_0)"]
    pub fn store_opened(&mut self, v: i64) {
        self.opened_0 = v;
    }

    //#[pure]
    //#[requires="idx >= 0 && idx < 1"]
    //#[ensures="idx == 0 ==> result == self.closed_0"]
    pub fn get_closed(&self, idx: usize) -> i64 {
        if idx == 0 {
            self.closed_0
        } else {
            unreachable!()
        }
    }
    //#[ensures="self.get_closed(0) == v"]
    //#[ensures="self.count_0 == old(self.count_0)"]
    //#[ensures="self.rec_var_0 == old(self.rec_var_0)"]
    //#[ensures="self.workload_0 == old(self.workload_0)"]
    //#[ensures="self.opened_0 == old(self.opened_0)"]
    pub fn store_closed(&mut self, v: i64) {
        self.closed_0 = v;
    }
}

pub fn monitor(mem: &mut Memory) {
    let mut src: i64;
    let mut dst: i64;
    let mut fin: bool;
    let mut push: bool;
    let mut syn: bool;
    let mut length: i64;

    let mut count: i64;
    let mut rec_var: i64;
    let mut received: i64;
    let mut workload: i64;
    let mut opened: i64;
    let mut closed: i64;
    let mut trigger_0: bool;
    let mut trigger_1: bool;
    let mut trigger_2: bool;

    //Iteration 0
    src = read_src();
    dst = read_dst();
    fin = read_fin();
    push = read_push();
    syn = read_syn();
    length = read_length();

    count = (if (0 > 201) { 0 } else { (0 + 1) });
    rec_var = (if (dst == 213451) {
        (0 + 1)
    } else {
        (if (count > 200) { 0 } else { 0 })
    });
    received = (if ((dst == 213451) && push) { 0 } else { length });
    workload = (if (count > 200) { (0 + 1) } else { 0 });
    opened = (0 + (if ((dst == 213451) && syn) { 1 } else { 0 }));
    closed = (0 + (if ((dst == 213451) && fin) { 1 } else { 0 }));
    trigger_0 = (rec_var > 50);
    trigger_1 = (workload > 25);
    trigger_2 = ((opened - closed) < 0);
    mem.store_count(count);
    mem.store_rec_var(rec_var);
    mem.store_workload(workload);
    mem.store_opened(opened);
    mem.store_closed(closed);
    //Iteration 1
    src = read_src();
    dst = read_dst();
    fin = read_fin();
    push = read_push();
    syn = read_syn();
    length = read_length();

    count = (if (mem.get_count(0) > 201) {
        0
    } else {
        (mem.get_count(0) + 1)
    });
    rec_var = (if (dst == 213451) {
        (mem.get_rec_var(0) + 1)
    } else {
        (if (count > 200) { 0 } else { mem.get_rec_var(0) })
    });
    received = (if ((dst == 213451) && push) { 0 } else { length });
    workload = (if (count > 200) {
        (mem.get_workload(0) + 1)
    } else {
        0
    });
    opened = (mem.get_opened(0) + (if ((dst == 213451) && syn) { 1 } else { 0 }));
    closed = (mem.get_closed(0) + (if ((dst == 213451) && fin) { 1 } else { 0 }));
    trigger_0 = (rec_var > 50);
    trigger_1 = (workload > 25);
    trigger_2 = ((opened - closed) < 0);
    mem.store_count(count);
    mem.store_rec_var(rec_var);
    mem.store_workload(workload);
    mem.store_opened(opened);
    mem.store_closed(closed);
    let mut con = exists_input();
    let mut iteration = 2;

    //#[invariant="iteration >= 2"]
    //#[invariant="gm_dst.len() == iteration"]
    //#[invariant="gm_fin.len() == iteration"]
    //#[invariant="gm_syn.len() == iteration"]
    //#[invariant="gm_count.len() == iteration"]
    //#[invariant="mem.get_count(0) == gm_count.get(iteration - 1)"]
    //#[invariant="gm_rec_var.len() == iteration"]
    //#[invariant="mem.get_rec_var(0) == gm_rec_var.get(iteration - 1)"]
    //#[invariant="gm_workload.len() == iteration"]
    //#[invariant="mem.get_workload(0) == gm_workload.get(iteration - 1)"]
    //#[invariant="gm_opened.len() == iteration"]
    //#[invariant="mem.get_opened(0) == gm_opened.get(iteration - 1)"]
    //#[invariant="gm_closed.len() == iteration"]
    //#[invariant="mem.get_closed(0) == gm_closed.get(iteration - 1)"]
    //#[invariant="trigger_0 == ((if (gm_dst.get(iteration - 1) == 213451) { (gm_rec_var.get(iteration - 2) + 1) } else { (if ((if (gm_count.get(iteration - 2) > 201) { 0 } else { (gm_count.get(iteration - 2) + 1) }) > 200) { 0 } else { gm_rec_var.get(iteration - 2) }) }) > 50)"]
    //#[invariant="trigger_1 == ((if ((if (gm_count.get(iteration - 2) > 201) { 0 } else { (gm_count.get(iteration - 2) + 1) }) > 200) { (gm_workload.get(iteration - 2) + 1) } else { 0 }) > 25)"]
    //#[invariant="trigger_2 == (((gm_opened.get(iteration - 2) + (if ((gm_dst.get(iteration - 1) == 213451) && gm_syn.get(iteration - 1)) { 1 } else { 0 })) - (gm_closed.get(iteration - 2) + (if ((gm_dst.get(iteration - 1) == 213451) && gm_fin.get(iteration - 1)) { 1 } else { 0 }))) < 0)"]
	
	let (og_order_tx, og_order_rx) = channel::bounded(5);
	let (og_result_tx, og_result_rx) = channel::bounded(5);
	let order_rx = og_order_rx.clone();
	let result_tx = og_result_tx.clone();
	let _thread_0 = std::thread::spawn(move || {
		let mut f_map: HashMap<usize, Box<dyn Fn(i64,i64, i64, i64) -> i64>> = HashMap::new();
		//count 
		let fn_6 = |_,_, _, mem_v0| if mem_v0 > 201 {0} else {mem_v0 + 1};
		// receiver
		let fn_7 = |dst, count,_,  mem_v0|(if dst == 213451 { mem_v0 + 1 } else {	if count > 200 {0} else {mem_v0}});

		//received
		let fn_8 = |dst: i64, push, length, mem_v0|(if (dst == 213451) && push > 1 { 0 } else { length });
		//workload
		let fn_9 = |count, _, _, mem_v0| (if (count > 200) {(mem_v0 + 1)} else {0});

		//opened
		let fn_10 = |dst, syn, _, mem_v0| mem_v0 + (if (dst == 213451) && syn == 1 { 1 } else { 0 });
		//closed
		let fn_11 = |dst, fin, _, mem_v0| mem_v0 + (if (dst == 213451) && fin == 1 { 1 } else { 0 });
		//trigger0
		let fn_12 = |r,_,_,_|  if r > 50 {1} else {0};
		//trigger1
		let fn_13 = |w,_,_,_|  if w > 25 {1} else {0};
		//trigger2
		let fn_14 = |o,c,_,_|  if o - c < 0 {1} else {0};

		f_map.insert(6, Box::new(fn_6));
		f_map.insert(7, Box::new(fn_7));
		f_map.insert(8, Box::new(fn_8));
		f_map.insert(9, Box::new(fn_9));
		f_map.insert(10, Box::new(fn_10));
		f_map.insert(11, Box::new(fn_11));
		f_map.insert(12, Box::new(fn_12));
		f_map.insert(13, Box::new(fn_13));
		f_map.insert(14, Box::new(fn_14));
		let f_map = f_map;
		order_rx.iter().for_each(|(v,(a,b,c,m))| result_tx.send((v,f_map[&v](a, b, c, m))).expect(""));
	});
	
	let order_rx = og_order_rx.clone();
	let result_tx = og_result_tx.clone();
    let _thread_1 = std::thread::spawn(move || {
		let mut f_map: HashMap<usize, Box<dyn Fn(i64,i64, i64, i64) -> i64>> = HashMap::new();
		//count 
		let fn_6 = |_,_, _, mem_v0| if mem_v0 > 201 {0} else {mem_v0 + 1};
		// receiver
		let fn_7 = |dst, count,_,  mem_v0|(if dst == 213451 { mem_v0 + 1 } else {	if count > 200 {0} else {mem_v0}});
		//received
		let fn_8 = |dst: i64, push, length, mem_v0|(if (dst == 213451) && push > 1 { 0 } else { length });
		//workload
		let fn_9 = |count, _, _, mem_v0| (if (count > 200) {(mem_v0 + 1)} else {0});
		//opened
		let fn_10 = |dst, syn, _, mem_v0| mem_v0 + (if (dst == 213451) && syn == 1 { 1 } else { 0 });
		//closed
		let fn_11 = |dst, fin, _, mem_v0| mem_v0 + (if (dst == 213451) && fin == 1 { 1 } else { 0 });
		//trigger0
		let fn_12 = |r,_,_,_|  if r > 50 {1} else {0};
		//trigger1
		let fn_13 = |w,_,_,_|  if w > 25 {1} else {0};
		//trigger2
		let fn_14 = |o,c,_,_|  if o - c < 0 {1} else {0};

		f_map.insert(6, Box::new(fn_6));
		f_map.insert(7, Box::new(fn_7));
		f_map.insert(8, Box::new(fn_8));
		f_map.insert(9, Box::new(fn_9));
		f_map.insert(10, Box::new(fn_10));
		f_map.insert(11, Box::new(fn_11));
		f_map.insert(12, Box::new(fn_12));
		f_map.insert(13, Box::new(fn_13));
		f_map.insert(14, Box::new(fn_14));
		let f_map = f_map;
		order_rx.iter().for_each(|(v,(a,b,c,m))| result_tx.send((v,f_map[&v](a, b, c, m))).expect(""));
	});
	let order_rx = og_order_rx.clone();
	let result_tx = og_result_tx.clone();
	let _thread_2 = std::thread::spawn(move || {
		let mut f_map: HashMap<usize, Box<dyn Fn(i64,i64, i64, i64) -> i64>> = HashMap::new();
		//count 
		let fn_6 = |_,_, _, mem_v0| if mem_v0 > 201 {0} else {mem_v0 + 1};
		// receiver
		let fn_7 = |dst, count,_,  mem_v0|(if dst == 213451 { mem_v0 + 1 } else {	if count > 200 {0} else {mem_v0}});
        //received
		let fn_8 = |dst: i64, push, length, mem_v0|(if (dst == 213451) && push > 1 { 0 } else { length });
		//workload
		let fn_9 = |count, _, _, mem_v0| (if (count > 200) {(mem_v0 + 1)} else {0});
		//opened
		let fn_10 = |dst, syn, _, mem_v0| mem_v0 + (if (dst == 213451) && syn == 1 { 1 } else { 0 });
		//closed
		let fn_11 = |dst, fin, _, mem_v0| mem_v0 + (if (dst == 213451) && fin == 1 { 1 } else { 0 });
		//trigger0
		let fn_12 = |r,_,_,_|  if r > 50 {1} else {0};
		//trigger1
		let fn_13 = |w,_,_,_|  if w > 25 {1} else {0};
		//trigger2
		let fn_14 = |o,c,_,_|  if o - c < 0 {1} else {0};

		f_map.insert(6, Box::new(fn_6));
		f_map.insert(7, Box::new(fn_7));
		f_map.insert(8, Box::new(fn_8));
		f_map.insert(9, Box::new(fn_9));
		f_map.insert(10, Box::new(fn_10));
		f_map.insert(11, Box::new(fn_11));
		f_map.insert(12, Box::new(fn_12));
		f_map.insert(13, Box::new(fn_13));
		f_map.insert(14, Box::new(fn_14));
		let f_map = f_map;
		order_rx.iter().for_each(|(v,(a,b,c,m))| result_tx.send((v,f_map[&v](a, b, c, m))).expect(""));
	});
    let order_rx = og_order_rx.clone();
	let result_tx = og_result_tx.clone();
	let _thread_4 = std::thread::spawn(move || {
		let mut f_map: HashMap<usize, Box<dyn Fn(i64,i64, i64, i64) -> i64>> = HashMap::new();
		//count 
		let fn_6 = |_,_, _, mem_v0| if mem_v0 > 201 {0} else {mem_v0 + 1};
		// receiver
		let fn_7 = |dst, count,_,  mem_v0|(if dst == 213451 { mem_v0 + 1 } else {	if count > 200 {0} else {mem_v0}});
		//received
		let fn_8 = |dst: i64, push, length, mem_v0|(if (dst == 213451) && push > 1 { 0 } else { length });
		//workload
		let fn_9 = |count, _, _, mem_v0| (if (count > 200) {(mem_v0 + 1)} else {0});
		//opened
		let fn_10 = |dst, syn, _, mem_v0| mem_v0 + (if (dst == 213451) && syn == 1 { 1 } else { 0 });
		//closed
		let fn_11 = |dst, fin, _, mem_v0| mem_v0 + (if (dst == 213451) && fin == 1 { 1 } else { 0 });
		//trigger0
		let fn_12 = |r,_,_,_|  if r > 50 {1} else {0};
		//trigger1
		let fn_13 = |w,_,_,_|  if w > 25 {1} else {0};
		//trigger2
		let fn_14 = |o,c,_,_|  if o - c < 0 {1} else {0};

		f_map.insert(6, Box::new(fn_6));
		f_map.insert(7, Box::new(fn_7));
		f_map.insert(8, Box::new(fn_8));
		f_map.insert(9, Box::new(fn_9));
		f_map.insert(10, Box::new(fn_10));
		f_map.insert(11, Box::new(fn_11));
		f_map.insert(12, Box::new(fn_12));
		f_map.insert(13, Box::new(fn_13));
		f_map.insert(14, Box::new(fn_14));
		let f_map = f_map;
		order_rx.iter().for_each(|(v,(a,b,c,m))| result_tx.send((v,f_map[&v](a, b, c, m))).expect(""));
	});
    let order_rx = og_order_rx.clone();
	let result_tx = og_result_tx.clone();
	let _thread_5 = std::thread::spawn(move || {
		let mut f_map: HashMap<usize, Box<dyn Fn(i64,i64, i64, i64) -> i64>> = HashMap::new();
		//count 
		let fn_6 = |_,_, _, mem_v0| if mem_v0 > 201 {0} else {mem_v0 + 1};
		// receiver
		let fn_7 = |dst, count,_,  mem_v0|(if dst == 213451 { mem_v0 + 1 } else {	if count > 200 {0} else {mem_v0}});

		//received
		let fn_8 = |dst: i64, push, length, mem_v0|(if (dst == 213451) && push > 1 { 0 } else { length });
		//workload
		let fn_9 = |count, _, _, mem_v0| (if (count > 200) {(mem_v0 + 1)} else {0});

		//opened
		let fn_10 = |dst, syn, _, mem_v0| mem_v0 + (if (dst == 213451) && syn == 1 { 1 } else { 0 });
		//closed
		let fn_11 = |dst, fin, _, mem_v0| mem_v0 + (if (dst == 213451) && fin == 1 { 1 } else { 0 });
		//trigger0
		let fn_12 = |r,_,_,_|  if r > 50 {1} else {0};
		//trigger1
		let fn_13 = |w,_,_,_|  if w > 25 {1} else {0};
		//trigger2
		let fn_14 = |o,c,_,_|  if o - c < 0 {1} else {0};

		f_map.insert(6, Box::new(fn_6));
		f_map.insert(7, Box::new(fn_7));
		f_map.insert(8, Box::new(fn_8));
		f_map.insert(9, Box::new(fn_9));
		f_map.insert(10, Box::new(fn_10));
		f_map.insert(11, Box::new(fn_11));
		f_map.insert(12, Box::new(fn_12));
		f_map.insert(13, Box::new(fn_13));
		f_map.insert(14, Box::new(fn_14));
		let f_map = f_map;
		order_rx.iter().for_each(|(v,(a,b,c,m))| result_tx.send((v,f_map[&v](a, b, c, m))).expect(""));
	});

    while con {
        //Layer 0
        src = read_src();
        dst = read_dst();
        fin = read_fin();
        push = read_push();
        syn = read_syn();
        length = read_length();
        //Layer 1
        
		og_order_tx.send((6,(0,0,0, mem.get_count(0)))).expect("");
		og_order_tx.send((7,(dst,count,0, mem.get_rec_var(0)))).expect("");
		og_order_tx.send((8,(dst,if push {1} else {0},length, 0))).expect("");
		og_order_tx.send((9,(count,0,0, mem.get_workload(0)))).expect("");
        og_order_tx.send((10,(dst,if syn {1} else {0},0, mem.get_opened(0)))).expect("");
		og_order_tx.send((11,(dst,if fin {1} else {0},0, mem.get_opened(0)))).expect("");
		og_result_rx.iter().take(6).for_each(|(ix, v): (usize,i64)| match ix {
            6 => {
                count = v;
            }
            7 => {
                rec_var = v;
            }
            8 => {
                received = v;
            }
            9 => {
                workload = v;
            }
            10 => {
                opened = v;
            }
			11 => {
                closed = v;
            }
			_ => {}
        });

		//Layer 2 

		og_order_tx.send((12,(0,0,0, mem.get_count(0)))).expect("");
		og_order_tx.send((13,(dst,count,0, mem.get_rec_var(0)))).expect("");
		og_order_tx.send((14,(dst,if push {1} else {0},length, 0))).expect("");
		og_result_rx.iter().take(3).for_each(|(ix, v): (usize,i64)| match ix {
			12 => {
                trigger_0 = v == 1;
            }
			13 => {
                trigger_1 = v == 1;
            }
			14 => {
                trigger_2 = v == 1;
            }
			_ => {}
        });
	
		
        mem.store_count(count);
        mem.store_rec_var(rec_var);
        mem.store_workload(workload);
        mem.store_opened(opened);
        mem.store_closed(closed);

        iteration += 1;
        con = exists_input();
    }

}
pub fn main() {}
