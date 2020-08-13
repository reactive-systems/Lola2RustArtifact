//extern crate prusti_contracts;
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

fn emit(
    trigger_many_incoming_connections: bool,
    trigger_Workload_too_high: bool,
    trigger_Closed_more_connection_than_were_open: bool,
) {
    if trigger_many_incoming_connections {
        //println!("trigger_many_incoming_connections");
    }
    if trigger_Workload_too_high {
        //println!("trigger_Workload_too_high");
    }
    if trigger_Closed_more_connection_than_were_open {
        //println!("trigger_Closed_more_connection_than_were_open");
    }
}

pub struct Memory {
    pub count_0: i64,
    pub receiver_0: i64,
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
    //#[ensures="self.receiver_0 == old(self.receiver_0)"]
    //#[ensures="self.workload_0 == old(self.workload_0)"]
    //#[ensures="self.opened_0 == old(self.opened_0)"]
    //#[ensures="self.closed_0 == old(self.closed_0)"]
    pub fn store_count(&mut self, v: i64) {
        self.count_0 = v;
    }

    //#[pure]
    //#[requires="idx >= 0 && idx < 1"]
    //#[ensures="idx == 0 ==> result == self.receiver_0"]
    pub fn get_receiver(&self, idx: usize) -> i64 {
        if idx == 0 {
            self.receiver_0
        } else {
            unreachable!()
        }
    }
    //#[ensures="self.get_receiver(0) == v"]
    //#[ensures="self.count_0 == old(self.count_0)"]
    //#[ensures="self.workload_0 == old(self.workload_0)"]
    //#[ensures="self.opened_0 == old(self.opened_0)"]
    //#[ensures="self.closed_0 == old(self.closed_0)"]
    pub fn store_receiver(&mut self, v: i64) {
        self.receiver_0 = v;
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
    //#[ensures="self.receiver_0 == old(self.receiver_0)"]
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
    //#[ensures="self.receiver_0 == old(self.receiver_0)"]
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
    //#[ensures="self.receiver_0 == old(self.receiver_0)"]
    //#[ensures="self.workload_0 == old(self.workload_0)"]
    //#[ensures="self.opened_0 == old(self.opened_0)"]
    pub fn store_closed(&mut self, v: i64) {
        self.closed_0 = v;
    }
}

struct GhostMemory_bool {
    mem: Vec<bool>,
}

impl GhostMemory_bool {
    //#[trusted]
    //#[ensures="result.len() == 0"]
    pub fn new() -> Self {
        GhostMemory_bool { mem: Vec::new() }
    }

    //#[trusted]
    //#[ensures="self.len() == old(self.len()) + 1"]
    //#[ensures="forall i: usize :: (0 <= i && i < old(self.len())) ==> (old(self.get(i)) == self.get(i))"]
    //#[ensures="self.get(self.len() -1) == v"]
    pub fn store(&mut self, v: bool) {
        self.mem.push(v);
    }

    //#[pure]
    //#[trusted]
    //#[requires="idx >= 0 && idx < self.len()"]
    pub fn get(&self, idx: usize) -> bool {
        self.mem[idx]
    }

    //#[trusted]
    //#[pure]
    //#[ensures="result >= 0"]
    pub fn len(&self) -> usize {
        self.mem.len()
    }
}

struct GhostMemory_i64 {
    mem: Vec<i64>,
}

impl GhostMemory_i64 {
    //#[trusted]
    //#[ensures="result.len() == 0"]
    pub fn new() -> Self {
        GhostMemory_i64 { mem: Vec::new() }
    }

    //#[trusted]
    //#[ensures="self.len() == old(self.len()) + 1"]
    //#[ensures="forall i: usize :: (0 <= i && i < old(self.len())) ==> (old(self.get(i)) == self.get(i))"]
    //#[ensures="self.get(self.len() -1) == v"]
    pub fn store(&mut self, v: i64) {
        self.mem.push(v);
    }

    //#[pure]
    //#[trusted]
    //#[requires="idx >= 0 && idx < self.len()"]
    pub fn get(&self, idx: usize) -> i64 {
        self.mem[idx]
    }

    //#[trusted]
    //#[pure]
    //#[ensures="result >= 0"]
    pub fn len(&self) -> usize {
        self.mem.len()
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
    let mut receiver: i64;
    let mut received: i64;
    let mut workload: i64;
    let mut opened: i64;
    let mut closed: i64;
    let mut trigger_many_incoming_connections: bool;
    let mut trigger_Workload_too_high: bool;
    let mut trigger_Closed_more_connection_than_were_open: bool;

    let mut gm_dst = GhostMemory_i64::new();
    let mut gm_fin = GhostMemory_bool::new();
    let mut gm_syn = GhostMemory_bool::new();
    let mut gm_count = GhostMemory_i64::new();
    let mut gm_receiver = GhostMemory_i64::new();
    let mut gm_workload = GhostMemory_i64::new();
    let mut gm_opened = GhostMemory_i64::new();
    let mut gm_closed = GhostMemory_i64::new();

    //Iteration 0
    src = read_src();
    dst = read_dst();
    fin = read_fin();
    push = read_push();
    syn = read_syn();
    length = read_length();

    count = (if (0 > 201) { 0 } else { (0 + 1) });
    receiver = (if (dst == 213451) {
        (0 + 1)
    } else {
        (if (count > 200) { 0 } else { 0 })
    });
    received = (if ((dst == 213451) && push) { 0 } else { length });
    workload = (if (count > 200) { (0 + 1) } else { 0 });
    opened = (0 + (if ((dst == 213451) && syn) { 1 } else { 0 }));
    closed = (0 + (if ((dst == 213451) && fin) { 1 } else { 0 }));
    trigger_many_incoming_connections = (receiver > 50);
    trigger_Workload_too_high = (workload > 25);
    trigger_Closed_more_connection_than_were_open = ((opened - closed) < 0);
    emit(
        trigger_many_incoming_connections,
        trigger_Workload_too_high,
        trigger_Closed_more_connection_than_were_open,
    );
    gm_dst.store(dst);
    gm_fin.store(fin);
    gm_syn.store(syn);
    gm_count.store(count);
    gm_receiver.store(receiver);
    gm_workload.store(workload);
    gm_opened.store(opened);
    gm_closed.store(closed);
    mem.store_count(count);
    mem.store_receiver(receiver);
    mem.store_workload(workload);
    mem.store_opened(opened);
    mem.store_closed(closed);
    assert!(
        trigger_many_incoming_connections
            == ((if (gm_dst.get(0) == 213451) {
                (0 + 1)
            } else {
                (if ((if (0 > 201) { 0 } else { (0 + 1) }) > 200) {
                    0
                } else {
                    0
                })
            }) > 50)
    );
    assert!(
        trigger_Workload_too_high
            == ((if ((if (0 > 201) { 0 } else { (0 + 1) }) > 200) {
                (0 + 1)
            } else {
                0
            }) > 25)
    );
    assert!(
        trigger_Closed_more_connection_than_were_open
            == (((0
                + (if ((gm_dst.get(0) == 213451) && gm_syn.get(0)) {
                    1
                } else {
                    0
                }))
                - (0 + (if ((gm_dst.get(0) == 213451) && gm_fin.get(0)) {
                    1
                } else {
                    0
                })))
                < 0)
    );
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
    receiver = (if (dst == 213451) {
        (mem.get_receiver(0) + 1)
    } else {
        (if (count > 200) {
            0
        } else {
            mem.get_receiver(0)
        })
    });
    received = (if ((dst == 213451) && push) { 0 } else { length });
    workload = (if (count > 200) {
        (mem.get_workload(0) + 1)
    } else {
        0
    });
    opened = (mem.get_opened(0) + (if ((dst == 213451) && syn) { 1 } else { 0 }));
    closed = (mem.get_closed(0) + (if ((dst == 213451) && fin) { 1 } else { 0 }));
    trigger_many_incoming_connections = (receiver > 50);
    trigger_Workload_too_high = (workload > 25);
    trigger_Closed_more_connection_than_were_open = ((opened - closed) < 0);
    emit(
        trigger_many_incoming_connections,
        trigger_Workload_too_high,
        trigger_Closed_more_connection_than_were_open,
    );
    gm_dst.store(dst);
    gm_fin.store(fin);
    gm_syn.store(syn);
    gm_count.store(count);
    gm_receiver.store(receiver);
    gm_workload.store(workload);
    gm_opened.store(opened);
    gm_closed.store(closed);
    mem.store_count(count);
    mem.store_receiver(receiver);
    mem.store_workload(workload);
    mem.store_opened(opened);
    mem.store_closed(closed);
    assert!(
        trigger_many_incoming_connections
            == ((if (gm_dst.get(1) == 213451) {
                (gm_receiver.get(0) + 1)
            } else {
                (if ((if (gm_count.get(0) > 201) {
                    0
                } else {
                    (gm_count.get(0) + 1)
                }) > 200)
                {
                    0
                } else {
                    gm_receiver.get(0)
                })
            }) > 50)
    );
    assert!(
        trigger_Workload_too_high
            == ((if ((if (gm_count.get(0) > 201) {
                0
            } else {
                (gm_count.get(0) + 1)
            }) > 200)
            {
                (gm_workload.get(0) + 1)
            } else {
                0
            }) > 25)
    );
    assert!(
        trigger_Closed_more_connection_than_were_open
            == (((gm_opened.get(0)
                + (if ((gm_dst.get(1) == 213451) && gm_syn.get(1)) {
                    1
                } else {
                    0
                }))
                - (gm_closed.get(0)
                    + (if ((gm_dst.get(1) == 213451) && gm_fin.get(1)) {
                        1
                    } else {
                        0
                    })))
                < 0)
    );
    let mut con = exists_input();
    let mut iteration = 2;

    //#[invariant="iteration >= 2"]
    //#[invariant="gm_dst.len() == iteration"]
    //#[invariant="gm_fin.len() == iteration"]
    //#[invariant="gm_syn.len() == iteration"]
    //#[invariant="gm_count.len() == iteration"]
    //#[invariant="mem.get_count(0) == gm_count.get(iteration - 1)"]
    //#[invariant="gm_receiver.len() == iteration"]
    //#[invariant="mem.get_receiver(0) == gm_receiver.get(iteration - 1)"]
    //#[invariant="gm_workload.len() == iteration"]
    //#[invariant="mem.get_workload(0) == gm_workload.get(iteration - 1)"]
    //#[invariant="gm_opened.len() == iteration"]
    //#[invariant="mem.get_opened(0) == gm_opened.get(iteration - 1)"]
    //#[invariant="gm_closed.len() == iteration"]
    //#[invariant="mem.get_closed(0) == gm_closed.get(iteration - 1)"]
    //#[invariant="trigger_many_incoming_connections == ((if (gm_dst.get(iteration - 1) == 213451) { (gm_receiver.get(iteration - 2) + 1) } else { (if ((if (gm_count.get(iteration - 2) > 201) { 0 } else { (gm_count.get(iteration - 2) + 1) }) > 200) { 0 } else { gm_receiver.get(iteration - 2) }) }) > 50)"]
    //#[invariant="trigger_Workload_too_high == ((if ((if (gm_count.get(iteration - 2) > 201) { 0 } else { (gm_count.get(iteration - 2) + 1) }) > 200) { (gm_workload.get(iteration - 2) + 1) } else { 0 }) > 25)"]
    //#[invariant="trigger_Closed_more_connection_than_were_open == (((gm_opened.get(iteration - 2) + (if ((gm_dst.get(iteration - 1) == 213451) && gm_syn.get(iteration - 1)) { 1 } else { 0 })) - (gm_closed.get(iteration - 2) + (if ((gm_dst.get(iteration - 1) == 213451) && gm_fin.get(iteration - 1)) { 1 } else { 0 }))) < 0)"]
    while (con) {
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
        receiver = (if (dst == 213451) {
            (mem.get_receiver(0) + 1)
        } else {
            (if (count > 200) {
                0
            } else {
                mem.get_receiver(0)
            })
        });
        received = (if ((dst == 213451) && push) { 0 } else { length });
        workload = (if (count > 200) {
            (mem.get_workload(0) + 1)
        } else {
            0
        });
        opened = (mem.get_opened(0) + (if ((dst == 213451) && syn) { 1 } else { 0 }));
        closed = (mem.get_closed(0) + (if ((dst == 213451) && fin) { 1 } else { 0 }));
        trigger_many_incoming_connections = (receiver > 50);
        trigger_Workload_too_high = (workload > 25);
        trigger_Closed_more_connection_than_were_open = ((opened - closed) < 0);
        emit(
            trigger_many_incoming_connections,
            trigger_Workload_too_high,
            trigger_Closed_more_connection_than_were_open,
        );

        mem.store_count(count);
        mem.store_receiver(receiver);
        mem.store_workload(workload);
        mem.store_opened(opened);
        mem.store_closed(closed);
        gm_dst.store(dst);
        gm_fin.store(fin);
        gm_syn.store(syn);
        gm_count.store(count);
        gm_receiver.store(receiver);
        gm_workload.store(workload);
        gm_opened.store(opened);
        gm_closed.store(closed);

        iteration += 1;
        con = exists_input();
    }
}
pub fn main() {}
