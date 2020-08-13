//extern crate prusti_contracts;
static mut ITERATION: usize = 0;

//#[trusted]
fn read_altitude() -> i64 {
    unsafe {
        let r = super::VAL[ITERATION];
        ITERATION += 1;
        return r;
    }
}

//#[trusted]
fn exists_input() -> bool {
    unsafe {
        return ITERATION < super::LENGTH;
    }
}

//#[trusted]
fn emit(trigger_Flying_above_maximum_altitude: bool, trigger_Flying_below_minimum_altitude: bool) {
    if trigger_Flying_above_maximum_altitude {
        //println!("Too High");
    }
    if trigger_Flying_below_minimum_altitude {
        //println!("Too Low");
    }
}

pub struct Memory {
    pub altitude_0: i64,
    pub altitude_1: i64,
}

impl Memory {
    //#[pure]
    //#[requires="idx >= 0 && idx < 2"]
    //#[ensures="idx == 0 ==> result == self.altitude_0"]
    //#[ensures="idx == 1 ==> result == self.altitude_1"]
    pub fn get_altitude(&self, idx: usize) -> i64 {
        if idx == 0 {
            self.altitude_0
        } else if idx == 1 {
            self.altitude_1
        } else {
            unreachable!()
        }
    }
    //#[ensures="self.get_altitude(1) == v"]
    //#[ensures="self.altitude_0 == old(self.altitude_1)"]
    pub fn store_altitude(&mut self, v: i64) {
        self.altitude_0 = self.altitude_1;
        self.altitude_1 = v;
    }
}

struct GhostMemory_i64 {
    mem: Vec<i64>,
}

impl GhostMemory_i64 {
    //    #[trusted]
    //    #[ensures="result.len() == 0"]
    pub fn new() -> Self {
        GhostMemory_i64 { mem: Vec::new() }
    }

    //    #[trusted]
    //    #[ensures="self.len() == old(self.len()) + 1"]
    //    #[ensures="forall i: usize :: (0 <= i && i < old(self.len())) ==> (old(self.get(i)) == self.get(i))"]
    //    #[ensures="self.get(self.len() -1) == v"]
    pub fn store(&mut self, v: i64) {
        self.mem.push(v);
    }

    //    #[pure]
    //    #[trusted]
    //    #[requires="idx >= 0 && idx < self.len()"]
    pub fn get(&self, idx: usize) -> i64 {
        self.mem[idx]
    }

    //    #[trusted]
    //    #[pure]
    //    #[ensures="result >= 0"]
    pub fn len(&self) -> usize {
        self.mem.len()
    }
}

pub fn monitor(mem: &mut Memory) {
    let mut altitude: i64;

    let mut tooLow: bool;
    let mut tooHigh: bool;
    let mut trigger_Flying_below_minimum_altitude: bool;
    let mut trigger_Flying_above_maximum_altitude: bool;

    let mut gm_altitude = GhostMemory_i64::new();

    //Iteration 0
    altitude = read_altitude();

    gm_altitude.store(altitude);
    mem.store_altitude(altitude);
    //Iteration 1
    altitude = read_altitude();

    tooLow = ((0 < 200) && ((mem.get_altitude(1) < 200) && (altitude < 200)));
    tooHigh = ((0 > 600) && ((mem.get_altitude(1) > 600) && (altitude > 600)));
    trigger_Flying_below_minimum_altitude = tooLow;
    trigger_Flying_above_maximum_altitude = tooHigh;
    emit(
        trigger_Flying_above_maximum_altitude,
        trigger_Flying_below_minimum_altitude,
    );
    gm_altitude.store(altitude);
    mem.store_altitude(altitude);
    assert!(
        trigger_Flying_below_minimum_altitude
            == ((0 < 200) && ((gm_altitude.get(0) < 200) && (gm_altitude.get(1) < 200)))
    );
    assert!(
        trigger_Flying_above_maximum_altitude
            == ((0 > 600) && ((gm_altitude.get(0) > 600) && (gm_altitude.get(1) > 600)))
    );
    //Iteration 2
    altitude = read_altitude();

    tooLow = ((mem.get_altitude(0) < 200) && ((mem.get_altitude(1) < 200) && (altitude < 200)));
    tooHigh = ((mem.get_altitude(0) > 600) && ((mem.get_altitude(1) > 600) && (altitude > 600)));
    trigger_Flying_below_minimum_altitude = tooLow;
    trigger_Flying_above_maximum_altitude = tooHigh;
    emit(
        trigger_Flying_above_maximum_altitude,
        trigger_Flying_below_minimum_altitude,
    );
    gm_altitude.store(altitude);
    mem.store_altitude(altitude);
    assert!(
        trigger_Flying_below_minimum_altitude
            == ((gm_altitude.get(0) < 200)
                && ((gm_altitude.get(1) < 200) && (gm_altitude.get(2) < 200)))
    );
    assert!(
        trigger_Flying_above_maximum_altitude
            == ((gm_altitude.get(0) > 600)
                && ((gm_altitude.get(1) > 600) && (gm_altitude.get(2) > 600)))
    );
    let mut con = exists_input();
    let mut iteration = 3;

    //#[invariant="iteration >= 3"]
    //#[invariant="gm_altitude.len() == iteration"]
    //#[invariant="mem.get_altitude(0) == gm_altitude.get(iteration - 2)"]
    //#[invariant="mem.get_altitude(1) == gm_altitude.get(iteration - 1)"]
    //#[invariant="trigger_Flying_below_minimum_altitude == ((gm_altitude.get(iteration - 3) < 200) && ((gm_altitude.get(iteration - 2) < 200) && (gm_altitude.get(iteration - 1) < 200)))"]
    //#[invariant="trigger_Flying_above_maximum_altitude == ((gm_altitude.get(iteration - 3) > 600) && ((gm_altitude.get(iteration - 2) > 600) && (gm_altitude.get(iteration - 1) > 600)))"]
    while (con) {
        altitude = read_altitude();
        tooLow = ((mem.get_altitude(0) < 200) && ((mem.get_altitude(1) < 200) && (altitude < 200)));
        tooHigh =
            ((mem.get_altitude(0) > 600) && ((mem.get_altitude(1) > 600) && (altitude > 600)));
        trigger_Flying_below_minimum_altitude = tooLow;
        trigger_Flying_above_maximum_altitude = tooHigh;
        emit(
            trigger_Flying_above_maximum_altitude,
            trigger_Flying_below_minimum_altitude,
        );

        mem.store_altitude(altitude);
        gm_altitude.store(altitude);

        iteration += 1;
        //dbg!(iteration);
        con = exists_input();
    }

    //Iteration N + 1
    tooLow = ((mem.get_altitude(0) < 200) && ((mem.get_altitude(1) < 200) && (0 < 200)));
    tooHigh = ((mem.get_altitude(0) > 600) && ((mem.get_altitude(1) > 600) && (0 > 600)));
    trigger_Flying_below_minimum_altitude = tooLow;
    trigger_Flying_above_maximum_altitude = tooHigh;
    emit(
        trigger_Flying_above_maximum_altitude,
        trigger_Flying_below_minimum_altitude,
    );
    assert!(
        trigger_Flying_below_minimum_altitude
            == ((gm_altitude.get(iteration - 2) < 200)
                && ((gm_altitude.get(iteration - 1) < 200) && (0 < 200)))
    );
    assert!(
        trigger_Flying_above_maximum_altitude
            == ((gm_altitude.get(iteration - 2) > 600)
                && ((gm_altitude.get(iteration - 1) > 600) && (0 > 600)))
    );
}
