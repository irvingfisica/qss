use std::collections::BTreeMap;

fn main() {

    let world_pop = Vec::from([2525779, 
                               3026003, 
                               3691173, 
                               4449049, 
                               5320817, 
                               6127700, 
                               6916183]);

    println!("world_pop: {:?}",&world_pop);

    let pop_first = Vec::from([2525779, 3026003, 3691173]);
    let mut pop_second = Vec::from([4449049, 5320817, 6127700, 6916183]);

    let mut pop_all = pop_first;
    pop_all.append(&mut pop_second);

    println!("pop_first no se puede accesar porque fue 'prestado' a pop_all");
    println!("pop_all: {:?}",pop_all);
    println!("pop_second (está vacío pues se usó para construir pop_all): {:?}",pop_second);

    println!("{:?}",&world_pop[1]);

    let pop_indexed = [1,3].iter().map(|idx| world_pop[*idx]).collect::<Vec<i32>>();
    let pop_indexed_rev = [3,1].iter().map(|idx| world_pop[*idx]).collect::<Vec<i32>>();

    println!("pop_indexed: {:?}",pop_indexed);
    println!("world_pop: {:?}",world_pop);
    println!("pop_indexed_rev: {:?}",pop_indexed_rev);

    let pop_million: Vec<f64> = world_pop.iter().map(|ele| *ele as f64/ 1000.0).collect();

    println!("pop_million: {:?}",pop_million);

    let actual = &mut world_pop[1..].iter();
    let anterior = &mut world_pop.iter();

    let mut pop_increase: Vec<f64> = actual.zip(anterior).map(|(i,im1)| (i - im1) as f64 / *im1 as f64 * 100.0).collect();

    println!("pop_increase: {:?}",pop_increase);
    println!("world_pop: {:?}",world_pop);

    [0,1].iter().zip([20.0,22.0].iter()).for_each(|(ind,val)| {pop_increase[*ind] = *val});

    println!("pop_increase: {:?}",pop_increase);

    println!("world_pop length: {}",world_pop.len());

    let max = world_pop.iter().max();
    let min = world_pop.iter().min();

    println!("world_pop max: {:?}", max);
    println!("world_pop min: {:?}", min);

    let mean = world_pop.iter().fold(0, |acc, x | acc + x) as f64 / world_pop.len() as f64;

    println!("world_pop mean: {:?}", mean);

    let years: Vec<i32> = (0..7).map(|i| 1950 + i*10).collect();
    println!("years: {:?}", years);
    let mut world_pop_map: BTreeMap<i32,i32> = BTreeMap::new();

    years.iter().zip(world_pop.iter()).for_each(|(year,val)|{
        world_pop_map.insert(*year,*val);
    });
    println!("mapa: {:?}", world_pop_map);

    let population_summary = Summary::get_summary(&world_pop_map);

    println!("summary: {:?}", population_summary);

} 

#[derive(Debug)]
pub struct Summary {
    sum: i32,
    length: usize,
    mean: f64,
}

impl Summary {
    fn get_summary(mapa: &BTreeMap<i32,i32>) -> Summary {
        let sum = mapa.iter().fold(0, |acc, (_, val)| acc + val);
        let length = mapa.len();
        let mean = sum as f64 / length as f64;

        Summary {sum,mean,length}
    }
}
