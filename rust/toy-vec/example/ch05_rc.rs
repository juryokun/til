use std::rc::Rc;

fn main() {
    let mut rcl = Rc::new(Child(1));
    println!("(a) count: {}, rcl: {:?}", Rc::strong_count(&rcl), rcl);
    {
        let rc2 = Rc::clone(&rcl);
        println!(
            "(b) count: {}, rcl: {:?}, rc2: {:?}",
            Rc::strong_count(&rcl),
            rcl,
            rc2
        );
    }
    println!("(c) count: {}, rcl: {:?}", Rc::strong_count(&rcl), rcl);

    if let Some(child) = Rc::get_mut(&mut rcl) {
        child.0 += 1;
    }
    println!("(d) count: {}, rcl: {:?}", Rc::strong_count(&rcl), rcl);

    let weak = Rc::downgrade(&rcl);
    println!(
        "(e) count: {}, rcl: {:?}, weak: {:?}",
        Rc::strong_count(&rcl),
        rcl,
        weak
    );

    if let Some(rc3) = weak.upgrade() {
        println!(
            "(f) count: {}, rcl: {:?}, rc3: {:?}",
            Rc::strong_count(&rcl),
            rcl,
            rc3,
        );
    }

    std::mem::drop(rcl);
    println!("(g) count: 0, weak.upgrade(): {:?}", weak.upgrade());
}
