use lodtree::coords::OctVec;
use lodtree::Tree;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::cmp::Ordering;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, SamplingMode};

const N_LOOKUPS:usize = 40;
fn generate_area_bounds(rng: &mut SmallRng) -> (OctVec, OctVec) {
    const D: u8 = 4;
    let cmax = 1 << D;

    let min = OctVec::new(
        rng.gen_range(0..cmax-1),
        rng.gen_range(0..cmax-1),
        rng.gen_range(0..cmax-1),
        D,
    );
    let max = OctVec::new(
            rng.gen_range(min.x+1..cmax),
            rng.gen_range(min.y+1..cmax),
            rng.gen_range(min.z+1..cmax),
            D,
        );
    return (min, max);
}

struct ChuChunk {
    tx_index: u8,
    rx_index: u8,
    material_index: u16,
    tx_visible: u64,
    rx_visible: u64,
}

impl Default for ChuChunk {
    fn default() -> ChuChunk {
        ChuChunk {
            tx_index: 1,
            rx_index: 2,
            material_index: 3,
            tx_visible: 4,
            rx_visible: 5,
        }
    }
}

fn create_and_fill_octree<C: Default>(num_chunks: u32, depth: u8) -> Tree<C, OctVec> {
    let mut rng = SmallRng::seed_from_u64(42);
    let mut tree: Tree<C, OctVec> = Tree::with_capacity( 0, 0);

    let cmax = 1 << depth;

    for _c in 0..num_chunks {
        let qv = OctVec::new(
            rng.gen_range(0..cmax),
            rng.gen_range(0..cmax),
            rng.gen_range(0..cmax),
            depth,
        );

        while tree.prepare_update(&[qv], 0, |_p| C::default()) {
            // do the update
            tree.do_update();
            // and clean
            tree.complete_update();
        }
    }
    tree
}

fn bench_lookups_in_octree(tree: & Tree<ChuChunk, OctVec>) {
    let mut rng = SmallRng::seed_from_u64(42);
    for _ in 0..N_LOOKUPS {
        let (min, max) = generate_area_bounds(&mut rng);
        for i in tree.iter_all_chunks_in_bounds_and_tree(min, max, 4) {
            black_box(i);
        }
    }
}

fn bench_mut_lookups_in_octree(tree: &mut Tree<ChuChunk, OctVec>) {
    let mut rng = SmallRng::seed_from_u64(42);
    for _ in 0..N_LOOKUPS {
        let (min, max) = generate_area_bounds(&mut rng);
        for i in tree.iter_all_chunks_in_bounds_and_tree_mut(min, max, 4) {
            i.1.material_index += 1;
            i.1.rx_index += 1;
            i.1.tx_visible += 1;
            i.1.tx_index += 1;
        }
    }
}

pub fn bench_iteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("mutable iteration");
    let mut samples_num = 100;

    for depth in [4u8, 6, 8, 10].iter() {

        if *depth as i8 == 4{
            samples_num = 100;
        }
        if *depth as i8   ==6{
            samples_num = 40;
        }
        if *depth as i8 ==8{
            samples_num = 10;
        }
        group.significance_level(0.1).sample_size(samples_num);

        let num_chunks: u32 = 2u32.pow(*depth as u32).pow(3) / 10;
        group.bench_with_input(BenchmarkId::from_parameter(depth), depth, |b, depth| {
            let mut tree = create_and_fill_octree::<ChuChunk>(num_chunks, *depth);
            b.iter(|| {
                bench_mut_lookups_in_octree(&mut tree);
            });
            black_box(tree);
        });
    }
    group.finish();

     let mut group = c.benchmark_group("immutable iteration");
     let mut samples_num = 10;


    for depth in [4u8, 6, 8, 10].iter() {

        if *depth as i8   == 4{
            samples_num = 100;
        }
        if *depth as i8   ==6{
            samples_num = 40;
        }
        if *depth as i8   ==8{
            samples_num = 10;
        }
        group.significance_level(0.1).sample_size(samples_num);
        let num_chunks: u32 = 2u32.pow(*depth as u32).pow(3) / 10;
        group.bench_with_input(BenchmarkId::from_parameter(depth), depth, |b, depth| {
            let  tree = create_and_fill_octree::<ChuChunk>(num_chunks, *depth);
            b.iter(|| {
                bench_lookups_in_octree(&tree);
            });
        });
    }
    group.finish();
}

pub fn bench_creation (c: &mut Criterion) {
    let mut group = c.benchmark_group("tree creation");

    let mut samples_num = 10;

    for depth in [4u8, 6, 8].iter() {

        if *depth as i8  == 4{
            samples_num = 100;
        }
        if *depth  as i8  ==6{
            samples_num = 40;
        }
        if *depth as i8  ==8{
            samples_num = 10;
        }
        group.significance_level(0.1).sample_size(samples_num);
        group.bench_with_input(BenchmarkId::from_parameter(depth), depth, |b, &depth| {
            let volume = 2u32.pow(depth as u32).pow(3);
            let num_chunks: u32 = volume / 10;
            println!("Creating {num_chunks} voxels out of {volume} possible");
            b.iter(|| {

                let t = create_and_fill_octree::<ChuChunk>(num_chunks, depth);
                black_box(t);
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_creation, bench_iteration);
criterion_main!(benches);
