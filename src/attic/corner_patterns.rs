use lazy_static::lazy_static;
use std::collections::HashMap;

type AdjacencyKernel = [bool;8];

lazy_static! {
    static ref CORNER_COUNT: HashMap<AdjacencyKernel, i32> = {
        let mut m: HashMap<AdjacencyKernel, i32> = HashMap::new();
        m.insert([true , true , true ,
                  true ,        true ,
                  true , true , true ], 0);
        m.insert([true , true , true ,
                  true ,        true ,
                  true , true , false], 0);
        m.insert([true , true , true ,
                  true ,        true ,
                  true , false, true ], 0);
        m.insert([true , true , true ,
                  true ,        true ,
                  true , false, false], 0);
        m.insert([true , true , true ,
                  true ,        true ,
                  false, true , true ], 0);
        m.insert([true , true , true ,
                  true ,        true ,
                  false, true , false], 0);
        m.insert([true , true , true ,
                  true ,        true ,
                  false, false, true ], 0);
        m.insert([true , true , true ,
                  true ,        true ,
                  false, false, false], 0);
        m
    };
}
