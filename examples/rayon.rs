use rayon::prelude::*;
use lodtree::*;
use lodtree::coords::OctVec;

struct Chunk {
	data: [f32; 4096]
}

impl Chunk {

	// this does a cheap init so it can safely be put inside the vec
	fn new(_position: OctVec) -> Self {
		Self {
			data: [0.0; 4096]
		}
	}

	// pretend this inits the data with some expensive procedural generation
	fn expensive_init(&mut self, _position: OctVec) {
		self.data = [1.0; 4096];
	}

	// and pretend this makes chunks visible/invisible
	fn set_visible(&mut self, _visibility: bool) {}
}

fn main() {

	// create an octree
	let mut tree = Tree::<Chunk, OctVec>::new();

	// the game loop that runs for 100 iterations
	for _ in 0..100 {

		let start_time = std::time::Instant::now();

		// get the pending updates
		if tree.prepare_update(&[OctVec::new(4096, 4096, 4096, 32)], 2, |position_in_tree| Chunk::new(position_in_tree)) {

			let duration = start_time.elapsed().as_micros();

			println!("Took {} microseconds to get the tree update ready", duration);

			// if there was an update, we need to first generate new chunks with expensive_init
			tree.get_chunks_to_add_slice_mut()
				.par_iter_mut()
				.for_each(|(position, chunk)| {

					// and run expensive init
					chunk.expensive_init(*position);
				});

			// and make all chunks visible or not
			for i in 0..tree.get_num_chunks_to_activate() {
				tree.get_chunk_to_activate_mut(i).set_visible(true);
			}

			for i in 0..tree.get_num_chunks_to_deactivate() {
				tree.get_chunk_to_deactivate_mut(i).set_visible(false);
			}

			let start_time = std::time::Instant::now();

			// and don't forget to actually run the update
			tree.do_update();

			let duration = start_time.elapsed().as_micros();

			println!("Took {} microseconds to execute the update", duration);
		}

		// and print some data about the run
		println!("Num chunks in the tree: {}", tree.get_num_chunks());
	}
}