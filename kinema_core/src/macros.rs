#[macro_export]
macro_rules! make_linkage {
    ($transform: expr, $points: expr, $geometry: expr $(, $childs: expr )?) => {
        {
            let mut l = Linkage::new(na::convert($transform), $points, $geometry, 0);
            let l = Arc::new(Mutex::new(l));
            $(
                for (child, index, relation) in $childs {
                    child.lock()
                        .unwrap()
                        .add_child(&l, index, relation);
                }
            )?
            l
        }
    };
}
